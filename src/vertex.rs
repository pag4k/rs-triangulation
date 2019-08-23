use std::cmp::Ordering;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Debug, Copy, Clone, Eq)]
pub struct Vertex<T> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T>> Add for Vertex<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vertex<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Mul<Output = T>> Mul for Vertex<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T: Div<f64, Output = T>> Div<f64> for Vertex<T> {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl<T: PartialEq> PartialEq for Vertex<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: PartialOrd> PartialOrd for Vertex<T> {
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

impl<T> Vertex<T>
where
    T: Copy,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
{
    pub fn magnitude_squared(&self) -> T {
        self.x * self.x + self.y * self.y
    }

    pub fn distance_squared(vertex1: Vertex<T>, vertex2: Vertex<T>) -> T {
        let diff = vertex2 - vertex1;
        diff.x * diff.x + diff.y * diff.y
    }

    pub fn distance(vertex1: Vertex<T>, vertex2: Vertex<T>) -> f64
    where
        f64: From<T>,
    {
        f64::sqrt(f64::from(Vertex::distance_squared(vertex1, vertex2)))
    }
}

// public float Length()
// {
// 	float ls = X * X + Y * Y;
// 	return (float)System.Math.Sqrt((double)ls);
// }

// public float LengthSquared()
// {
// 	return X * X + Y * Y;
// }

// public static float Distance(Vector2 value1, Vector2 value2)
// {
// 	float dx = value1.X - value2.X;
// 	float dy = value1.Y - value2.Y;

// 	float ls = dx * dx + dy * dy;

// 	return (float)System.Math.Sqrt((double)ls);
// }

// public override int GetHashCode()
// {
// 	int hash = this.X.GetHashCode();
// 	hash = (((hash << 5) + hash) ^ this.Y.GetHashCode ());
// 	return hash;
// }

// public bool Equals(Vector2 other)
// {
// 	return this.X == other.X && this.Y == other.Y;
// }

// public override bool Equals(object obj)
// {
// 	if (!(obj is Vector2))
// 		return false;
// 	return Equals((Vector2)obj);
// }
