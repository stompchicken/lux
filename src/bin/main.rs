extern crate lux;

use lux::vector::{Vec3, Ray, dot};
use lux::image::{Colour, Bitmap, lerp};
use std::path::Path;


fn colour(r: Ray) ->  Colour {
    let v = Vec3::new(0.0, 0.0, -1.0);
    let t = hit_sphere(v, 0.5, r);

    if t > 0.0 {
        let N = (r.point_at(t) - v).normalise();
        let dN = 0.5 * (N + Vec3::new(1.0, 1.0, 1.0));
        return Colour::new(dN.x, dN.y, dN.z)
    } else {
        let d = r.direction.normalise();
        let t = 0.5 * (d.y + 1.0);
        let c = Colour::new(0.5, 0.7, 1.0);
        lerp(t, c, Colour::white())
    }
}

fn hit_sphere(center: Vec3, radius: f32, r: Ray) -> f32 {
    let oc = r.origin - center;
    let a = dot(r.direction, r.direction);
    let b = 2.0 * dot(oc, r.direction);
    let c = dot(oc, oc) - radius * radius;
    let d = b*b - 4.0*a*c;
    if d < 0.0 {
        return -1.0
    } else {
        return (-b - d.sqrt()) / (2.0 * a)
    }
}

fn main() {
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::origin();

    let width = 400;
    let height = 200;

    let mut image = Bitmap::new(width, height);

    
    for y in 0..height {
        for x in 0..width {
            let u = x as f32 / width as f32;
            let v = 1.0 - (y as f32 / height as f32);

            let r = Ray::new(origin, lower_left_corner + (u*horizontal) + (v*vertical));
            let c = colour(r);

            image.pixel(x, y).r = c.r;
            image.pixel(x, y).g = c.g;
            image.pixel(x, y).b = c.b;
        }
    }



    let result = image.write_ppm(Path::new("out.ppm"));
    match result {
        Ok(_) => println!("Wrote image to {:?}", "out.ppm"),
        Err(e) => println!("Error writing: {:?}", e),
    }

}
