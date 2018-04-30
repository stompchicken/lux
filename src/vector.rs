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

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Hit {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3
}

pub trait Object {
    fn test_hit(self, r: Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}


#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32
}

impl Sphere {

    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center: center, radius: radius }
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
                                normal: (r.point_at(temp) - self.center) / self.radius
                };
                return Some(hit);
            } else {
                let temp2 = (-b + (b*b - a*c).sqrt()) / a;
                if (temp < t_max) && (temp > t_min) {
                    let hit = Hit { t: temp2,
                                    p: r.point_at(temp2),
                                    normal: (r.point_at(temp2) - self.center) / self.radius
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

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    vertical: Vec3,
    horizontal: Vec3,
}

impl Camera {

    pub fn new(origin: Vec3, lower_left_corner: Vec3, vertical: Vec3, horizontal: Vec3) -> Camera {
        Camera { origin: origin,
                 lower_left_corner: lower_left_corner,
                 vertical: vertical,
                 horizontal: horizontal }
    }

    pub fn get_ray(&self, u: f32, v:f32) -> Ray {
        Ray::new(self.origin,
                 self.lower_left_corner +
                 (u*self.horizontal) +
                 (v*self.vertical))

    }

}
