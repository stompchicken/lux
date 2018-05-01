use std::ops as ops;
use std::f32;

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
        return (self.x * self.x) + (self.y * self.y) + (self.z * self.z);
    }

    pub fn length(self) -> f32 {
        return (self.squared_length()).sqrt();
    }

    pub fn normalise(self) -> Vec3 {
        return self / self.length();
    }

}

pub fn dot(v1: Vec3, v2: Vec3) -> f32 {
    return (v1.x * v2.x) + (v1.y * v2.y) + (v1.z * v2.z);
}

pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3 { x: (v1.y * v2.z) + (v1.z * v2.y),
           y: (v1.x * v2.z) + (v1.z * v2.x),
           z: (v1.x * v2.y) + (v1.y * v2.x) }
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

    pub fn point_at(self, t: f32) -> Vec3 {
        return self.origin + (t * self.direction);
    }
}


pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    vertical: Vec3,
    horizontal: Vec3,
}

impl Camera {

    pub fn new(vfov: f32, aspect: f32) -> Camera {
        let theta = vfov * (f32::consts::PI / 180.0);
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        Camera { origin: Vec3::origin(),
                 lower_left_corner: Vec3::new(-half_width, -half_height, -1.0),
                 horizontal: Vec3::new(2.0*half_width, 0.0, 0.0),
                 vertical: Vec3::new(0.0, 2.0*half_height, 0.0) }
    }

    pub fn get_ray(&self, u: f32, v:f32) -> Ray {
        Ray::new(self.origin,
                 self.lower_left_corner +
                 (u*self.horizontal) +
                 (v*self.vertical))

    }

}
