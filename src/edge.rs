use std::cmp::Ordering;
use std::fmt::Debug;

use crate::vertex::Vertex;

#[derive(Debug, Copy, Clone)]
pub struct Edge<T> {
    pub a: Vertex<T>,
    pub b: Vertex<T>,
}

// How to prevent the direct construction of the struct?

impl<T: PartialEq> PartialEq for Edge<T> {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}

impl<T> Edge<T> {
    pub fn new(a: Vertex<T>, b: Vertex<T>) -> Self
    where
        T: PartialOrd ,
    {

        // FIXME: Not sure about this unwrap.
        match a.partial_cmp(&b).unwrap() {
            Ordering::Less => Edge { a, b },
            Ordering::Greater => Edge { a: b, b: a },
            Ordering::Equal => panic!("Cannot construct Edge with a == b."),
        }
    }
}
