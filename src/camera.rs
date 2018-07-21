use std::f32;
use vector::{Vector};
use light::{Ray, Colour};

#[derive(Debug)]
pub struct Camera {
    origin: Vector,
    lower_left_corner: Vector,
    direction: Vector,
    vertical: Vector,
    horizontal: Vector,
}

impl Camera {

    pub fn new(look_from: Vector,
               look_at: Vector,
               v_up: Vector,
               fov: f32,
               aspect: f32) -> Camera {
        let theta = fov.to_radians();
        let half_width = (theta / 2.0).tan();
        let half_height = half_width / aspect;

        let z = (look_at - look_from).normalise();
        let x = Vector::cross(v_up, z).normalise();
        let y = Vector::cross(x, z).normalise();

        let lower_left_corner = look_from - (half_width*x) - (half_height*y);
        let horizontal = half_width * x;
        let vertical = half_height * y;

        Camera {
            origin: look_from,
            direction: z,
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical
        }
    }

    pub fn get_ray(&self, u: f32, v:f32) -> Ray {
        Ray::new(self.origin,
                 self.direction +
                 (2.0 * u - 1.0) * self.horizontal +
                 (2.0 * v - 1.0) * self.vertical,
                 Colour::white())

    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera() {
        let camera = Camera::new(Vector::new(0.0, 0.0, 0.0),
                                 Vector::new(0.0, 0.0, -1.0),
                                 Vector::new(0.0, 1.0, 0.0),
                                 90.0,
                                 1.0);

        assert!(camera.get_ray(0.0, 0.0).direction == Vector::new(1.0, -1.0, -1.0),
                "{:?}", camera.get_ray(-1.0, -1.0).direction);

        assert!(camera.get_ray(1.0, 1.0).direction == Vector::new(-1.0, 1.0, -1.0),
                "{:?}", camera.get_ray(-1.0, -1.0).direction);

    }
}
