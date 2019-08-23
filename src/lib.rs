extern crate mut_static;

#[macro_use]
extern crate lazy_static;

use mut_static::MutStatic;
mod edge;
use edge::Edge;
mod triangle;
mod vertex;
use vertex::Vertex;
mod delauney;
use delauney::triangulation;

lazy_static! {
    pub static ref VERTICES: MutStatic<Vec<Vertex<f64>>> = { MutStatic::from(Vec::new()) };
    pub static ref EDGES: MutStatic<Vec<Edge<f64>>> = { MutStatic::from(Vec::new()) };
}

#[no_mangle]
pub extern "C" fn add_vertex(x: f64, y: f64) {
    let mut vertices = VERTICES.write().unwrap();
    vertices.push(Vertex { x, y });
}

#[no_mangle]
pub extern "C" fn triangulate() -> i32 {
    let vertices = VERTICES.write().unwrap();
    let mut edges = EDGES.write().unwrap();
    *edges = triangulation(&vertices);
    edges.len() as i32
}

#[no_mangle]
pub extern "C" fn get_x1_at(index: i32) -> f64 {
    let edges = EDGES.write().unwrap();
    edges[index as usize].a.x
}

#[no_mangle]
pub extern "C" fn get_x2_at(index: i32) -> f64 {
    let edges = EDGES.write().unwrap();
    edges[index as usize].b.x
}

#[no_mangle]
pub extern "C" fn get_y1_at(index: i32) -> f64 {
    let edges = EDGES.write().unwrap();
    edges[index as usize].a.y

}

#[no_mangle]
pub extern "C" fn get_y2_at(index: i32) -> f64 {
    let edges = EDGES.write().unwrap();
    edges[index as usize].b.y
}