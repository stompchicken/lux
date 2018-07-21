use vector::{Vector};
use light::{Time, Ray, Hit};
use world::{Geometry};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct RectXZ {
    pub x0: f32,
    pub x1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
}

impl RectXZ {

    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32) -> RectXZ {
        RectXZ { x0: x0, x1: x1, z0: z0, z1: z1, k: k }
    }

}

impl Geometry for RectXZ {

    fn test_collision(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
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
            time: t,
            location: r.point_at(t),
            surface_normal: if r.direction.y >= 0.0 {
                Vector::new(0.0, -1.0, 0.0)
            } else {
                Vector::new(0.0, 1.0, 0.0)
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
}

impl RectYZ {

    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32) -> RectYZ {
        RectYZ { y0: y0, y1: y1, z0: z0, z1: z1, k: k }
    }

}

impl Geometry for RectYZ {

    fn test_collision(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
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
            time: t,
            location: r.point_at(t),
            surface_normal: if r.direction.x >= 0.0 {
                Vector::new(-1.0, 0.0, 0.0)
            } else {
                Vector::new(1.0, 0.0, 0.0)
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
    pub k: f32
}

impl RectXY {

    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32) -> RectXY {
        RectXY { x0: x0, x1: x1, y0: y0, y1: y1, k: k }
    }

}

impl Geometry for RectXY {

    fn test_collision(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
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
            time: t,
            location: r.point_at(t),
            surface_normal: if r.direction.z >= 0.0 {
                Vector::new(0.0, 0.0, -1.0)
            } else {
                Vector::new(0.0, 0.0, 1.0)
            }
        };
        return Some(hit);
    }

}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Cuboid {
    pub top: RectXZ,
    pub bottom: RectXZ,
    pub front: RectXY,
    pub back: RectXY,
    pub left: RectYZ,
    pub right: RectYZ,
}

impl Cuboid {
    pub fn new(k: f32) -> Cuboid {
        Cuboid {
            top: RectXZ::new(-k, k, -k, k, k),
            bottom: RectXZ::new(-k, k, -k, k, -k),
            front: RectXY::new(-k, k, -k, k, -k),
            back: RectXY::new(-k, k, -k, k, k),
            left: RectYZ::new(-k, k, -k, k, k),
            right: RectYZ::new(-k, k, -k, k, -k),
        }
    }
}

impl Geometry for Cuboid {

    fn test_collision(&self, r: &Ray, t_min: Time, t_max: Time) -> Option<Hit> {
        let mut hit = Hit { time: t_max, location: Vector::zero(), surface_normal: Vector::zero() };

        match self.front.test_collision(r, t_min, t_max) {
            Some(h) => { hit = h; },
            None => {},
        }
        match self.back.test_collision(r, t_min, hit.time) {
            Some(h) => { hit = h; },
            None => {},
        }
        match self.left.test_collision(r, t_min, hit.time) {
            Some(h) => { hit = h; },
            None => {},
        }
        match self.right.test_collision(r, t_min, hit.time) {
            Some(h) => { hit = h; },
            None => {},
        }
        match self.top.test_collision(r, t_min, hit.time) {
            Some(h) => { hit = h; },
            None => {},
        }
        match self.bottom.test_collision(r, t_min, hit.time) {
            Some(h) => { hit = h; },
            None => {},
        }

        if hit.time == t_max {
            return None;
        } else {
            return Some(hit);
        }
    }

}
