use crate::vertex::Vertex;
use core::cmp::{Ordering, PartialEq, PartialOrd};

#[derive(Debug, Copy, Clone)]
pub struct Edge {
    pub a: Vertex,
    pub b: Vertex,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}

impl Edge {
    pub fn new(a: Vertex, b: Vertex) -> Self {
        match a.partial_cmp(&b).unwrap() {
            Ordering::Less => Edge { a, b },
            Ordering::Greater => Edge { a: b, b: a },
            Ordering::Equal => {
                panic!("Cannot construct Edge with a == b.");
            }
        }
    }
    pub fn as_array(&self) -> [f32; 4] {
        [self.a.x, self.a.y, self.b.x, self.b.y]
    }
}
