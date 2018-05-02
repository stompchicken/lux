use std::f32;
use vector::{Vec3};

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

#[derive(Debug)]
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

        Camera { origin: Vec3::new(0.0, 0.0, 1.0),
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
