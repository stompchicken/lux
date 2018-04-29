use std::ops as ops;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn origin() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn squared_length(self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn length(self) -> f32 {
        (self.squared_length()).sqrt()
    }

    pub fn normalise(self) -> Vec3 {
        self / self.length()
    }

    pub fn dot(self, other: Vec3) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 { x: (self.y * other.z) + (self.z * other.y),
               y: (self.x * other.z) + (self.z * other.x),
               z: (self.x * other.y) + (self.y * other.x) }
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x + other.x,
               y: self.y + other.y,
               z: self.z + other.z}
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x - other.x,
               y: self.y - other.y,
               z: self.z - other.z}
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, s: f32) -> Vec3 {
        Vec3 { x: self.x * s,
               y: self.y * s,
               z: self.z * s}
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 { x: v.x * self,
               y: v.y * self,
               z: v.z * self}
    }
}


impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, s: f32) -> Vec3 {
        Vec3 { x: self.x / s,
               y: self.y / s,
               z: self.z / s}
    }
}

impl ops::Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, v: Vec3) -> Vec3 {
        Vec3 { x: v.x / self,
               y: v.y / self,
               z: v.z / self}
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {

    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin: origin,
              direction: direction }
    }

    pub fn point_at(r: Ray, t: f32) -> Vec3 {
        r.origin + (t * r.direction)
    }
}
