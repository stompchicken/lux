use std::ops as ops;
use vector::{Vector};

pub type Time = f32;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Colour {
    pub fn new(r: f32, g:f32, b:f32) -> Colour {
        Colour { r: r, g: g, b: b }
    }

    pub fn black() -> Colour {
        Colour { r: 0.0, g: 0.0, b: 0.0 }
    }

    pub fn white() -> Colour {
        Colour { r: 1.0, g: 1.0, b: 1.0 }
    }
}

impl ops::Mul<Colour> for Colour {
    type Output = Colour;

    fn mul(self, c: Colour) -> Colour {
        Colour { r: self.r * c.r,
                 g: self.g * c.g,
                 b: self.b * c.b }
    }
}

impl ops::Add<Colour> for Colour {
    type Output = Colour;

    fn add(self, c: Colour) -> Colour {
        Colour { r: self.r + c.r,
                 g: self.g + c.g,
                 b: self.b + c.b }
    }
}

pub fn lerp(t: f32, c1: Colour, c2: Colour) -> Colour {
    Colour { r: (t * c1.r) + (1.0 - t) * c2.r,
             g: (t * c1.g) + (1.0 - t) * c2.g,
             b: (t * c1.b) + (1.0 - t) * c2.b }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
    pub colour: Colour,
}

impl Ray {

    pub fn new(origin: Vector, direction: Vector, colour: Colour) -> Ray {
        Ray { origin: origin,
              direction: direction,
              colour: colour
        }
    }

    pub fn point_at(self, t: f32) -> Vector {
        return self.origin + (t * self.direction);
    }

    pub fn rotate_x(self, theta: f32) -> Ray {
        Ray { origin: self.origin.rotate_x(theta),
              direction: self.direction.rotate_x(theta),
              colour: self.colour
        }
    }

    pub fn rotate_y(self, theta: f32) -> Ray {
        Ray { origin: self.origin.rotate_y(theta),
              direction: self.direction.rotate_y(theta),
              colour: self.colour
        }
    }

    pub fn rotate_z(self, theta: f32) -> Ray {
        Ray { origin: self.origin.rotate_z(theta),
              direction: self.direction.rotate_z(theta),
              colour: self.colour
        }
    }

}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Hit {
    pub time: Time,
    pub location: Vector,
    pub surface_normal: Vector,
}

#[derive(Debug)]
pub enum HitResult {
    None,
    Reflected(Time, Ray),
    Emitted(Time, Colour),
}
