#![no_std]
#![feature(alloc_error_handler, core_intrinsics, lang_items)]
#![feature(slice_partition_dedup)]

#[macro_use]
extern crate alloc;
extern crate wee_alloc;

use alloc::vec::Vec;
use core::{mem, slice};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    core::intrinsics::abort();
}

#[alloc_error_handler]
fn oom(_: core::alloc::Layout) -> ! {
    core::intrinsics::abort();
}

#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum c_void {
    #[doc(hidden)]
    __variant1,

    #[doc(hidden)]
    __variant2,
}

mod edge;
use edge::Edge;
mod triangle;
mod vertex;
use vertex::Vertex;
mod delauney;
use delauney::triangulation;

#[no_mangle]
pub extern "C" fn alloc(capacity: usize) -> *mut c_void {
    let mut buffer = Vec::with_capacity(capacity);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);

    pointer as *mut c_void
}

#[no_mangle]
pub extern "C" fn dealloc(pointer: *mut c_void, capacity: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(pointer, 0, capacity);
    }
}

#[no_mangle]
pub extern "C" fn triangulate(pointer: *mut u8, length: u32) -> *mut u8 {
    let input = unsafe { slice::from_raw_parts(pointer, length as usize) };

    let mut vertices = read_vertices(input, length as usize / 2);

    let edges = triangulation(&mut vertices);

    let number_edges = edges.len();

    let mut output = Vec::with_capacity(4 + 4 * 4 * number_edges);

    let number_edges_u8s = u32_to_u8s(number_edges as u32);

    output.push(number_edges_u8s.0);
    output.push(number_edges_u8s.1);
    output.push(number_edges_u8s.2);
    output.push(number_edges_u8s.3);
    for edge in edges {
        write_edge(&mut output, &edge)
    }

    let pointer = output.as_mut_ptr();
    mem::forget(output);
    pointer
}

fn read_vertices(input: &[u8], number_vertices: usize) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    for n in 0..number_vertices {
        let i = 8 * n;
        unsafe {
            let x = u8s_to_u32(input.get_unchecked(i..i + 4)) as f32;
            let y = u8s_to_u32(input.get_unchecked(i + 4..i + 8)) as f32;
            vertices.push(Vertex { x, y });
        }
    }
    vertices
}

fn write_edge(output: &mut Vec<u8>, edge: &Edge) {
    let array = edge.as_array();
    for element in &array {
        let element_u8s = u32_to_u8s(*element as u32);
        output.push(element_u8s.0);
        output.push(element_u8s.1);
        output.push(element_u8s.2);
        output.push(element_u8s.3);
    }
}

fn u8s_to_u32(bytes: &[u8]) -> u32 {
    // FIXME: Should not have conversion to u8.
    u32::from_be_bytes([
        bytes[0] as u8,
        bytes[1] as u8,
        bytes[2] as u8,
        bytes[3] as u8,
    ])
}

fn u32_to_u8s(x: u32) -> (u8, u8, u8, u8) {
    (
        ((x >> 24) & 0xff) as u8,
        ((x >> 16) & 0xff) as u8,
        ((x >> 8) & 0xff) as u8,
        (x & 0xff) as u8,
    )
}
