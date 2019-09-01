
use core::intrinsics::sqrtf32;
use core::ops::{Add, Sub, Mul, Div};
use core::cmp::{Ordering, PartialEq, PartialOrd};

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
}

impl Add for Vertex {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vertex {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul for Vertex {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Div<f32> for Vertex {
    type Output = Self;

    fn div(self, other: f32) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (
            self.x < other.x,
            self.x == other.x,
            self.y < other.y,
            self.y == other.y,
        ) {
            (false, true, false, true) => Some(Ordering::Equal),
            (true, _, _, _) => Some(Ordering::Less),
            (false, true, true, _) => Some(Ordering::Less),
            _ => Some(Ordering::Greater),
        }
    }
}

impl Vertex
{
    pub fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn distance_squared(vertex1: Vertex, vertex2: Vertex) -> f32 {
        let diff = vertex2 - vertex1;
        diff.x * diff.x + diff.y * diff.y
    }

    pub fn distance(vertex1: Vertex, vertex2: Vertex) -> f32
    {
        unsafe { sqrtf32(f32::from(Vertex::distance_squared(vertex1, vertex2))) }
    }
}
