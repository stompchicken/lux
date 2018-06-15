use vector::{Vec3, dot};
use light::{Ray, Hit, Colour};
use material::{Material};


pub trait Object {
    fn test_hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}


#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {

    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center: center,
                 radius: radius,
        }
    }

}

impl Object for Sphere {

    fn test_hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
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
                };
                return Some(hit);
            } else {
                let temp2 = (-b + (b*b - a*c).sqrt()) / a;
                if (temp < t_max) && (temp > t_min) {
                    let hit = Hit { t: temp2,
                                    p: r.point_at(temp2),
                                    normal: (r.point_at(temp2) - self.center) / self.radius,
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

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct RectXZ {
    pub x0: f32,
    pub x1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub flip_normal: bool
}

impl RectXZ {

    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, flip_normal: bool) -> RectXZ {
        RectXZ { x0: x0, x1: x1, z0: z0, z1: z1, k: k, flip_normal: flip_normal }
    }

}

impl Object for RectXZ {

    fn test_hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let t = (self.k - r.origin.y) / r.direction.y;
        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin.x + t*r.direction.x;
        let z = r.origin.z + t*r.direction.z;
        if x < self.x0 || x > self.x1 ||
           z < self.z0 || z > self.z1 {
            return None;
        }

        let hit = Hit {
            t: t,
            p: r.point_at(t),
            normal: if self.flip_normal {
                Vec3::new(0.0, -1.0, 0.0)
            } else {
                Vec3::new(0.0, 1.0, 0.0)
            }
        };
        return Some(hit);
    }

}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct RectYZ {
    pub y0: f32,
    pub y1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub flip_normal: bool,
}

impl RectYZ {

    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, flip_normal: bool) -> RectYZ {
        RectYZ { y0: y0, y1: y1, z0: z0, z1: z1, k: k, flip_normal: flip_normal }
    }

}

impl Object for RectYZ {

    fn test_hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let t = (self.k - r.origin.x) / r.direction.x;
        if t < t_min || t > t_max {
            return None;
        }

        let y = r.origin.y + t*r.direction.y;
        let z = r.origin.z + t*r.direction.z;
        if y < self.y0 || y > self.y1 ||
           z < self.z0 || z > self.z1 {
            return None;
        }

        let hit = Hit {
            t: t,
            p: r.point_at(t),
            normal: if self.flip_normal {
                Vec3::new(-1.0, 0.0, 0.0)
            } else {
                Vec3::new(1.0, 0.0, 0.0)
            }
        };
        return Some(hit);
    }

}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct RectXY {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub k: f32,
    pub flip_normal: bool,
}

impl RectXY {

    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, flip_normal: bool) -> RectXY {
        RectXY { x0: x0, x1: x1, y0: y0, y1: y1, k: k, flip_normal: flip_normal }
    }

}

impl Object for RectXY {

    fn test_hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let t = (self.k - r.origin.z) / r.direction.z;
        if t < t_min || t > t_max {
            return None;
        }

        let x = r.origin.x + t*r.direction.x;
        let y = r.origin.y + t*r.direction.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let hit = Hit {
            t: t,
            p: r.point_at(t),
            normal: if self.flip_normal {
                Vec3::new(0.0, 0.0, -1.0)
            } else {
                Vec3::new(0.0, 0.0, 1.0)
            }
        };
        return Some(hit);
    }

}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Cube {
    pub top: RectXZ,
    pub bottom: RectXZ,
    pub front: RectXY,
    pub back: RectXY,
    pub left: RectYZ,
    pub right: RectYZ,
}

impl Cube {
    pub fn new(k: f32) -> Cube {
        Cube {
            top: RectXZ::new(-k, k, -k, k, k, false),
            bottom: RectXZ::new(-k, k, -k, k, -k, true),
            front: RectXY::new(-k, k, -k, k, -k, true),
            back: RectXY::new(-k, k, -k, k, k, false),
            left: RectYZ::new(-k, k, -k, k, k, false),
            right: RectYZ::new(-k, k, -k, k, -k, true),
        }
    }
}

impl Object for Cube {

    fn test_hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        self.front.test_hit(r, t_min, t_max)
            .or(self.back.test_hit(r, t_min, t_max))
            .or(self.left.test_hit(r, t_min, t_max))
            .or(self.right.test_hit(r, t_min, t_max))
            .or(self.top.test_hit(r, t_min, t_max))
            .or(self.bottom.test_hit(r, t_min, t_max))
         
    }

}

pub struct World {
    pub objects: Vec<Box<Object>>,
    pub materials: Vec<Box<Material>>,
}

pub struct HitResult {
    pub reflected: Option<Ray>,
    pub emitted: Option<Colour>,
}

impl HitResult {
    pub fn new() -> HitResult {
        HitResult { reflected: None, emitted: None }
    }
}

impl World {

    pub fn new() -> World {
        World { objects: Vec::new(),
                materials: Vec::new()
        }
    }

    pub fn push_object(&mut self, obj: Box<Object>, mat: Box<Material>) {
        self.objects.push(obj);
        self.materials.push(mat);
    }

    pub fn propagate_ray(&self, r: &Ray, t_min: f32, t_max: f32) -> HitResult {
        let mut closest = t_max;
        let mut h = HitResult::new();
        for (obj, mat) in self.objects.iter().zip(&self.materials) {
            match obj.test_hit(r, t_min, closest) {
                Some(hit) => {
                    closest = hit.t;
                    h.reflected = mat.scatter(r, &hit);
                    h.emitted = mat.emit();
                },
                None => {},
            }
        }

        return h;
    }
}
