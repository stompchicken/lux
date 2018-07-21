use matrix::{Matrix};
use light::{Time, Ray, Hit, HitResult};


pub trait Geometry {
    fn test_collision(&self, r: &Ray, t_min: Time, t_max: Time) -> Option<Hit>;
}

pub trait Material {
    fn hit(&self, r: &Ray, hit: &Hit) -> HitResult;
}

pub trait Object {
    fn propagate(&self, r: &Ray, t_min: Time, t_max: Time) -> HitResult;
}

pub struct SimpleObject {
    geometry: Box<Geometry>,
    material: Box<Material>,
    transform: Matrix,
}

impl SimpleObject {
    pub fn new(geometry: Box<Geometry>, material: Box<Material>) -> SimpleObject {
        SimpleObject {
            geometry: geometry,
            material: material,
            transform: Matrix::identity()
        }
    }

    pub fn transform(&mut self, t: Matrix) {
        self.transform = self.transform * t;
    }
}

impl Object for SimpleObject {
    fn propagate(&self, r: &Ray, t_min: Time, t_max: Time) -> HitResult {
        //let theta = 75.0 as f32;

//        let rot = r.rotate_y(theta.to_radians());
        match self.geometry.test_collision(&r, t_min, t_max) {
            Some(hit) => {
                let rot_hit = Hit {
                    time: hit.time,
                    location: hit.location,
                    surface_normal: hit.surface_normal
                };
                return self.material.hit(&r, &rot_hit);
            },
            None => HitResult::None,
        }
    }
}

pub struct World {
    pub objects: Vec<Box<Object>>
}

impl World {

    pub fn new() -> World {
        World {
            objects: Vec::new(),
        }
    }

    pub fn push_object(&mut self, obj: Box<Object>) {
        self.objects.push(obj);
    }

    pub fn propagate_ray(&self, r: &Ray, t_min: Time, t_max: Time) -> HitResult {
        let mut closest = t_max;
        let mut h = HitResult::None;
        for obj in self.objects.iter() {
            let hit = obj.propagate(r, t_min, closest);
            match hit {
                HitResult::None => {},
                HitResult::Reflected(t, _) => { closest = t; h = hit},
                HitResult::Emitted(t, _) => { closest = t; h = hit},
            }
        }

        return h;
    }
}
