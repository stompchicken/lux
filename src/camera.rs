use std::f32;
use vector::{Vec3, cross};
use light::{Ray, Colour};

#[derive(Debug)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    vertical: Vec3,
    horizontal: Vec3,
}

impl Camera {

    pub fn new(look_from: Vec3,
               look_at: Vec3,
               v_up: Vec3,
               vfov: f32,
               aspect: f32) -> Camera {
        let theta = vfov * (f32::consts::PI / 180.0);
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (look_from - look_at).normalise();
        let u = cross(v_up, w).normalise();
        let v = cross(w, u);

        Camera { origin: look_from,
                 lower_left_corner: look_from - (half_width*u) - (half_height*v) - w,
                 horizontal: 2.0 * half_width * u,
                 vertical: 2.0 * half_height * v
        }
    }

    pub fn get_ray(&self, u: f32, v:f32) -> Ray {
        Ray::new(self.origin,
                 self.lower_left_corner +
                 (u*self.horizontal) +
                 (v*self.vertical),
                 Colour::white())

    }

}
