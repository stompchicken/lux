use vector::{Vector};
use light::{Time, Ray, Hit};
use world::{Geometry};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vector,
    pub radius: f32,
}

impl Sphere {

    pub fn new(center: Vector, radius: f32) -> Sphere {
        Sphere { center: center,
                 radius: radius,
        }
    }

}

impl Geometry for Sphere {

    fn test_collision(&self, r: &Ray, t_min: Time, t_max: Time) -> Option<Hit> {
        let oc = r.origin - self.center;
        let a = Vector::dot(r.direction, r.direction);
        let b = Vector::dot(oc, r.direction);
        let c = Vector::dot(oc, oc) - self.radius * self.radius;
        let d = b*b - a*c;

        if d > 0.0 {
            let temp = (-b - (b*b - a*c).sqrt()) / a;
            if (temp < t_max) && (temp > t_min) {
                let hit = Hit {
                    time: temp,
                    location: r.point_at(temp),
                    surface_normal: (r.point_at(temp) - self.center) / self.radius,
                };
                return Some(hit);
            } else {
                let temp2 = (-b + (b*b - a*c).sqrt()) / a;
                if (temp < t_max) && (temp > t_min) {
                    let hit = Hit {
                        time: temp2,
                        location: r.point_at(temp2),
                        surface_normal: (r.point_at(temp2) - self.center) / self.radius,
                    };
                    return Some(hit);
                } else {
                    return None;
                }
            }
        } else {
            return None;
        }
    }
}
