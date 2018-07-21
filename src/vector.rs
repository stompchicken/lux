use std::ops as ops;
use std::f32;
use std::fmt;

#[derive(PartialEq, Clone, Copy)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Vector {
    pub fn zero() -> Vector {
        Vector { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }
    }

    pub fn identity() -> Vector {
        Vector { x: 1.0, y: 1.0, z: 1.0, w: 1.0 }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x: x, y: y, z: z, w: 1.0 }
    }

    pub fn add(lhs: Vector, rhs: Vector) -> Vector {
        Vector { x: lhs.x + rhs.x, y: lhs.y + rhs.y, z: lhs.z + rhs.z, w: 1.0}
    }

    pub fn sub(lhs: Vector, rhs: Vector) -> Vector {
        Vector { x: lhs.x - rhs.x, y: lhs.y - rhs.y, z: lhs.z - rhs.z, w: 1.0}
    }

    pub fn mul(lhs: f32, rhs: Vector) -> Vector {
        Vector { x: lhs * rhs.x, y: lhs * rhs.y, z: lhs * rhs.z, w: 1.0}
    }

    pub fn div(lhs: f32, rhs: Vector) -> Vector {
        Vector { x: rhs.x / lhs, y: rhs.y / lhs, z: rhs.z / lhs, w: 1.0}
    }

    pub fn dot(v1: Vector, v2: Vector) -> f32 {
        return (v1.x * v2.x) + (v1.y * v2.y) + (v1.z * v2.z);
    }

    pub fn cross(v1: Vector, v2: Vector) -> Vector {
        Vector { x: (v1.y * v2.z) + (v1.z * v2.y),
                 y: (v1.x * v2.z) + (v1.z * v2.x),
                 z: (v1.x * v2.y) + (v1.y * v2.x),
                 w: 1.0 }
    }

    pub fn squared_length(&self) -> f32 {
        return (self.x * self.x) + (self.y * self.y) + (self.z * self.z);
    }

    pub fn length(&self) -> f32 {
        return (self.squared_length()).sqrt();
    }

    pub fn approx_eq(lhs: Vector, rhs: Vector) -> bool {
        let eps = 1.0e-4;
        return (lhs.x - rhs.x).abs() < eps
            && (lhs.y - rhs.y).abs() < eps
            && (lhs.z - rhs.z).abs() < eps
            && (lhs.w - rhs.w).abs() < eps;
    }

    pub fn normalise(&self) -> Vector {
        let length = self.length();
        return Vector {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
            w: 1.0
        }
    }

    pub fn rotate_x(&self, theta: f32) -> Vector {
        return Vector {
            x: self.x,
            y: theta.cos() * self.y - theta.sin() * self.z,
            z: theta.sin() * self.y + theta.cos() * self.z,
            w: 1.0
        }
    }

    pub fn rotate_y(&self, theta: f32) -> Vector {
        return Vector {
            x: theta.cos() * self.x + theta.sin() * self.z,
            y: self.y,
            z: theta.cos() * self.z - theta.sin() * self.x,
            w: 1.0
        }
    }

    pub fn rotate_z(&self, theta: f32) -> Vector {
        return Vector {
            x: theta.cos() * self.x - theta.sin() * self.y,
            y: theta.sin() * self.x + theta.cos() * self.y,
            z: self.z,
            w: 1.0
        }
    }
}

impl ops::Index<usize> for Vector {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        if i == 0 {
            &self.x
        } else if i == 1 {
            &self.y
        } else if i == 2 {
            &self.z
        } else if i == 3 {
            &self.w
        } else {
            panic!("Indexed Vector out of bounds!");
        }
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector { Vector::add(self, other) }
}

impl ops::Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector { Vector::sub(self, other) }
}

impl ops::Mul<f32> for Vector {
    type Output = Vector;
    fn mul(self, s: f32) -> Vector { Vector::mul(s, self) }
}

impl ops::Mul<Vector> for f32 {
    type Output = Vector;
    fn mul(self, v: Vector) -> Vector { Vector::mul(self, v) }
}

impl ops::Div<f32> for Vector {
    type Output = Vector;
    fn div(self, s: f32) -> Vector { Vector::div(s, self) }
}

impl ops::Div<Vector> for f32 {
    type Output = Vector;
    fn div(self, v: Vector) -> Vector { Vector::div(self, v) }
}

impl fmt::Debug for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ {:.3} {:.3} {:.3} {:.3} ]", self.x, self.y, self.z, self.w)
    }
}
