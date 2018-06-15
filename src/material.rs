use vector::{Vec3, dot};
use light::{Colour, Ray, Hit};
use random::{thread_rng};


pub trait Material {
    fn scatter(&self, r: &Ray, hit: &Hit) -> Option<Ray>;
    fn emit(&self) -> Option<Colour>;
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

    fn scatter(&self, r: &Ray, hit: &Hit) -> Option<Ray> {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        let ray = Ray::new(hit.p, target - hit.p, r.colour * self.albedo);
        return Some(ray);
    }

    fn emit(&self) -> Option<Colour> {
        None
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

    fn scatter(&self, r: &Ray, hit: &Hit) -> Option<Ray> {
        let reflected = reflect(r.direction.normalise(), hit.normal);
        let scattered = Ray::new(hit.p,
                                 reflected + self.fuzz * random_in_unit_sphere(),
                                 r.colour * self.colour);
        if dot(scattered.direction, hit.normal) > 0.0 {
            return Some(scattered);
        } else {
            return None;
        }
    }

    fn emit(&self) -> Option<Colour> {
        None
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

    fn scatter(&self, _r: &Ray, _hit: &Hit) -> Option<Ray> {
        None
    }

    fn emit(&self) -> Option<Colour> {
        Some(self.colour)
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
/*
fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.normalise();
    let dt = dot(uv, n);
    let discrim = 1.0 - ni_over_nt*ni_over_nt*(1.0 - dt*dt);
    if discrim > 0.0 {
        Some(ni_over_nt*(uv - n*dt) - n*discrim.sqrt())
    } else {
        None
    }
}
*/
