use vector::{Vec3, dot};
use camera::{Ray};
use image::{Colour};

use random::{thread_rng};
//use rand::{Rng, thread_rng};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Hit {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

pub trait Object {
    fn test_hit(self, r: Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}


#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {

    pub fn new(center: Vec3, radius: f32, material: Material) -> Sphere {
        Sphere { center: center,
                 radius: radius,
                 material: material }
    }

}

impl Object for Sphere {

    fn test_hit(self, r: Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = r.origin - self.center;
        let a = dot(r.direction, r.direction);
        let b = dot(oc, r.direction);
        let c = dot(oc, oc) - self.radius * self.radius;
        let d = b*b - a*c;
        if d > 0.0 {
            let temp = (-b - (b*b - a*c).sqrt()) / a;
            if (temp < t_max) && (temp > t_min) {
                let hit = Hit { t: temp,
                                p: r.point_at(temp),
                                normal: (r.point_at(temp) - self.center) / self.radius,
                                material: self.material,
                };
                return Some(hit);
            } else {
                let temp2 = (-b + (b*b - a*c).sqrt()) / a;
                if (temp < t_max) && (temp > t_min) {
                    let hit = Hit { t: temp2,
                                    p: r.point_at(temp2),
                                    normal: (r.point_at(temp2) - self.center) / self.radius,
                                    material: self.material,
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

pub struct World {
    pub objects: Vec<Sphere>
}


impl World {

    pub fn new() -> World {
        World { objects: Vec::new() }
    }

    pub fn test_hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut closest = t_max;
        let mut h: Option<Hit> = None;
        for obj in self.objects.iter() {
            match obj.test_hit(r, t_min, closest) {
                Some(hit) => {
                    closest = hit.t;
                    h = Some(hit);

                },
                None => {},
            }
        }

        return h;
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Material {
    pub albedo: Colour,
    pub metal: bool,
    pub fuzz: f32,
}

impl Material {

    pub fn new(albedo: Colour, metal: bool, fuzz: f32) -> Material {
        Material { albedo: albedo, metal: metal, fuzz: fuzz }
    }

    pub fn scatter(self, r: Ray, hit: Hit) -> Option<(Ray, Colour)> {
        if self.metal {
            let reflected = reflect(r.direction.normalise(), hit.normal);
            let scattered = Ray::new(hit.p, reflected + self.fuzz * random_in_unit_sphere());
            if dot(scattered.direction, hit.normal) > 0.0 {
                return Some((scattered, self.albedo));
            } else {
                return None;
            }
        } else {
            let target = hit.p + hit.normal + random_in_unit_sphere();
            let ray = Ray::new(hit.p, target - hit.p);
            return Some((ray, self.albedo));
        }
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3::new(1.0, 1.0, 1.0);
    let mut rng = thread_rng();
    while p.squared_length() >= 1.0 {
        p = 2.0 * Vec3::new(rng.gen(),
                            rng.gen(),
                            rng.gen()) - Vec3::new(1.0, 1.0, 1.0);
    }
    return p;
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    return v - (dot(v,n) * n);
}
