use vector::{Vector};
use light::{Colour, Ray, Hit, HitResult};
use world::{Material};
use random::{thread_rng};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct NormalSurface {
}

impl NormalSurface {
    pub fn new() -> NormalSurface {
        NormalSurface {}
    }
}


impl Material for NormalSurface {

    fn hit(&self, _r: &Ray, hit: &Hit) -> HitResult {
        let c = hit.surface_normal.normalise();
        return HitResult::Emitted(hit.time, Colour::new(0.5 * (c.x + 1.0),
                                                        0.5 * (c.y + 1.0),
                                                        0.5 * (c.z + 1.0)));
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Diffuse {
    pub albedo: Colour,
}

impl Diffuse {

    pub fn new(albedo: Colour) -> Diffuse {
        Diffuse { albedo: albedo }
    }
}

impl Material for Diffuse {

    fn hit(&self, r: &Ray, hit: &Hit) -> HitResult {
        let target = hit.location + hit.surface_normal + random_in_unit_sphere();
        let ray = Ray::new(hit.location, target - hit.location, r.colour * self.albedo);
        return HitResult::Reflected(hit.time, ray);
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Metal {
    pub colour: Colour,
    pub fuzz: f32,
}

impl Metal {

    pub fn new(colour: Colour, fuzz: f32) -> Metal {
        Metal { colour: colour, fuzz: fuzz }
    }
}


impl Material for Metal {

    fn hit(&self, r: &Ray, hit: &Hit) -> HitResult {
        let reflected = reflect(r.direction.normalise(), hit.surface_normal);
        let scattered = Ray::new(hit.location,
                                 reflected + self.fuzz * random_in_unit_sphere(),
                                 r.colour * self.colour);
        if Vector::dot(scattered.direction, hit.surface_normal) > 0.0 {
            return HitResult::Reflected(hit.time, scattered);
        } else {
            return HitResult::None;
        }
    }

}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct DiffuseLight {
    pub colour: Colour,
}

impl DiffuseLight {

    pub fn new(colour: Colour) -> DiffuseLight {
        DiffuseLight { colour: colour }
    }
}


impl Material for DiffuseLight {

    fn hit(&self, _r: &Ray, hit: &Hit) -> HitResult {
        HitResult::Emitted(hit.time, self.colour)
    }

}

fn random_in_unit_sphere() -> Vector {
    let mut p = Vector::new(1.0, 1.0, 1.0);
    let mut rng = thread_rng();
    while p.squared_length() >= 1.0 {
        p = 2.0 * Vector::new(rng.gen(),
                            rng.gen(),
                            rng.gen()) - Vector::new(1.0, 1.0, 1.0);
    }
    return p;
}

fn reflect(v: Vector, n: Vector) -> Vector {
    return v - (Vector::dot(v,n) * n);
}
