use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

use crate::edge::Edge;
use crate::vertex::Vertex;

#[derive(Debug, Copy, Clone)]
pub struct Triangle<T> {
    pub a: Vertex<T>,
    pub b: Vertex<T>,
    pub c: Vertex<T>,
}

impl<T> Triangle<T> {
    pub fn get_squared_circumradius(&self) -> f64
    where
        T: Copy,
        T: Add<Output = T>,
        T: Sub<Output = T>,
        T: Mul<Output = T>,
        f64: From<T>,
    {
        let A = Vertex::distance(self.a, self.b);
        let B = Vertex::distance(self.b, self.c);
        let C = Vertex::distance(self.c, self.a);
        (A * B * C) * (A * B * C) / ((A + B + C) * (B + C - A) * (A + C - B) * (A + B - C))
    }
    pub fn get_circumcenter(&self) -> Vertex<T>
    where
        T: Copy,
        T: Add<Output = T>,
        T: Sub<Output = T>,
        T: Mul<Output = T>,
        T: Mul<f64, Output = T>,
        T: Div<Output = T>,
        f64: From<T>,
    {
        let aa = self.a.magnitude_squared();
        let bb = self.b.magnitude_squared();
        let cc = self.c.magnitude_squared();

        let Axy = determinant_3x3(self.a, self.b, self.c);
        let Bx = determinant_3x3(
            Vertex { x: aa, y: self.a.y },
            Vertex { x: bb, y: self.b.y },
            Vertex { x: cc, y: self.c.y },
        );
        let By = determinant_3x3(
            Vertex { x: aa, y: self.a.x },
            Vertex { x: bb, y: self.b.x },
            Vertex { x: cc, y: self.c.x },
        );

        Vertex {
            x: Bx / (Axy * 2.0),
            y: By / (Axy * -2.0),
        }
    }

    pub fn is_in_circumcircle(&self, vertex: &Vertex<T>) -> bool
    where
        T: Copy,
        T: Add<Output = T>,
        T: Sub<Output = T>,
        T: Mul<Output = T>,
        T: Mul<f64, Output = T>,
        T: Div<Output = T>,
        f64: From<T>,
    {
        self.get_squared_circumradius()
            > f64::from((self.get_circumcenter() - *vertex).magnitude_squared())
    }

    fn contains_vertex(&self, vertex: &Vertex<T>) -> bool
    where
        Vertex<T>: PartialEq,
    {
        self.a == *vertex || self.b == *vertex || self.c == *vertex
    }

    pub fn has_shared_vertex(A: &Triangle<T>, B: &Triangle<T>) -> bool
    where
        Vertex<T>: PartialEq,
    {
        A.contains_vertex(&B.a) || A.contains_vertex(&B.b) || A.contains_vertex(&B.c)
    }

    pub fn get_edges(&self) -> Vec<Edge<T>>
    where
        T: PartialOrd,
        Vertex<T>: Clone,
    {
        vec![
            Edge::new(self.a.clone(), self.b.clone()),
            Edge::new(self.b.clone(), self.c.clone()),
            Edge::new(self.c.clone(), self.a.clone()),
        ]
    }
}

fn determinant_3x3<T>(A: Vertex<T>, B: Vertex<T>, C: Vertex<T>) -> T
where
    T: Copy,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
    f64: From<T>,
{
    A.x * B.y + A.y * C.x + B.x * C.y - B.y * C.x - A.y * B.x - A.x * C.y
}
