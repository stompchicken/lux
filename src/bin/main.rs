extern crate lux;
extern crate rand;

use lux::vector::{Vec3, Ray, Sphere, World, Camera};
use lux::image::{Colour, Bitmap, lerp};
use std::path::Path;
use rand::Rng;

fn colour(r: Ray, world: &World) ->  Colour {

    match world.test_hit(r, 0.0, 1000.0) {
        Some(hit) => Colour::new(0.5 * (hit.normal.x + 1.0),
                                 0.5 * (hit.normal.y + 1.0),
                                 0.5 * (hit.normal.z + 1.0)),
        None => {
            let d = r.direction.normalise();
            let t = 0.5 * (d.y + 1.0);
            let c = Colour::new(0.5, 0.7, 1.0);
            lerp(t, c, Colour::white())
        }
    }
}


fn main() {

    let width = 400;
    let height = 200;
    let n_rays = 100;

    let camera = Camera::new(
        Vec3::origin(),
        Vec3::new(-2.0, -1.0, -1.0),
        Vec3::new(0.0, 2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0));

    let mut world = World::new();
    world.objects.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.objects.push(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    let mut image = Bitmap::new(width, height);

    let mut rng = rand::thread_rng();

    for y in 0..height {
        for x in 0..width {

            let mut col = Colour::black();

            for _n in 0..n_rays {

                let px: f32 = rng.gen();
                let py: f32 = rng.gen();

                let u = ((x as f32) + px) / width as f32;
                let v = 1.0 - ((y as f32) + py) / height as f32;

                let r = camera.get_ray(u, v);
                let c = colour(r, &world);

                col.r += c.r;
                col.g += c.g;
                col.b += c.b;

            }

            image.pixel(x, y).r = col.r / (n_rays as f32);
            image.pixel(x, y).g = col.g / (n_rays as f32);
            image.pixel(x, y).b = col.b / (n_rays as f32);


        }
    }



    let result = image.write_ppm(Path::new("out.ppm"));
    match result {
        Ok(_) => println!("Wrote image to {:?}", "out.ppm"),
        Err(e) => println!("Error writing: {:?}", e),
    }

}
