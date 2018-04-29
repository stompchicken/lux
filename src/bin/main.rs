extern crate lux;

use lux::vector::{Vec3, Ray};
use lux::image::{Colour, Bitmap, lerp};
use std::path::Path;


fn colour(r: Ray) ->  Colour {
    let d = r.direction.normalise();
    let t = 0.5 * (d.y + 1.0);
    let c = Colour::new(0.5, 0.7, 1.0);
    lerp(t, c, Colour::white())
}

fn main() {
    let v = Vec3::origin();
    println!("v={:?}", v);


    let r = Ray::new(v, v);
    println!("r={:?}", r);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::origin();

    let width = 320;
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
