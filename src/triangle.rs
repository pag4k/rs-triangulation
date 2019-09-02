use alloc::vec::Vec;

use crate::edge::Edge;
use crate::vertex::Vertex;

#[derive(Debug, Clone)]
pub struct Triangle {
    pub a: Vertex,
    pub b: Vertex,
    pub c: Vertex,
}

impl Triangle {
    pub fn get_squared_circumradius(&self) -> f32 {
        let a = Vertex::distance(self.a, self.b);
        let b = Vertex::distance(self.b, self.c);
        let c = Vertex::distance(self.c, self.a);
        (a * b * c) * (a * b * c) / ((a + b + c) * (b + c - a) * (a + c - b) * (a + b - c))
    }
    pub fn get_circumcenter(&self) -> Vertex {
        let aa = self.a.magnitude_squared();
        let bb = self.b.magnitude_squared();
        let cc = self.c.magnitude_squared();

        let axy = determinant_3x3(self.a, self.b, self.c);
        let bx = determinant_3x3(
            Vertex { x: aa, y: self.a.y },
            Vertex { x: bb, y: self.b.y },
            Vertex { x: cc, y: self.c.y },
        );
        let by = determinant_3x3(
            Vertex { x: aa, y: self.a.x },
            Vertex { x: bb, y: self.b.x },
            Vertex { x: cc, y: self.c.x },
        );

        Vertex {
            x: bx / (axy * 2.0),
            y: by / (axy * -2.0),
        }
    }

    pub fn is_in_circumcircle(&self, vertex: &Vertex) -> bool {
        self.get_squared_circumradius()
            > f32::from((self.get_circumcenter() - *vertex).magnitude_squared())
    }

    fn contains_vertex(&self, vertex: &Vertex) -> bool {
        self.a == *vertex || self.b == *vertex || self.c == *vertex
    }

    pub fn has_shared_vertex(a: &Triangle, b: &Triangle) -> bool {
        a.contains_vertex(&b.a) || a.contains_vertex(&b.b) || a.contains_vertex(&b.c)
    }

    pub fn get_edges(&self) -> Vec<Edge> {
        vec![
            Edge::new(self.a.clone(), self.b.clone()),
            Edge::new(self.b.clone(), self.c.clone()),
            Edge::new(self.c.clone(), self.a.clone()),
        ]
    }
}

fn determinant_3x3(a: Vertex, b: Vertex, c: Vertex) -> f32 {
    a.x * b.y + a.y * c.x + b.x * c.y - b.y * c.x - a.y * b.x - a.x * c.y
}
