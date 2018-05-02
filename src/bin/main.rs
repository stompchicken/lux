extern crate lux;
extern crate rand;

use lux::vector::{Vec3};
use lux::camera::{Camera, Ray};
use lux::world::{Sphere, World, Material};
use lux::image::{Colour, Bitmap, lerp};
use std::path::Path;
use rand::{Rng};

fn trace_ray(r: Ray, world: &World, depth: i32) -> Colour {

    match world.test_hit(r, 0.001, 1000.0) {
        Some(hit) => {
            if depth < 50 {
                match hit.material.scatter(r, hit) {
                    Some((scattered, attn)) => {
                        return attn * trace_ray(scattered, world, depth+1);
                    },
                    None => Colour::black()
                }
            } else {
                return Colour::black();
            }
        }
        None => {
            let d = r.direction.normalise();
            let t = 0.5 * (d.y + 1.0);
            let c = Colour::new(0.5, 0.7, 1.0);
            return lerp(t, c, Colour::white());
        }
    }
}


fn main() {

    let width = 400;
    let height = 200;
    let n_rays = 1000;

    let camera = Camera::new(45.0,
                             width as f32 / height as f32);

    let mut world = World::new();
    world.objects.push(
        Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Material::new(Colour::new(0.8, 0.3, 0.3), false, 0.0)));

    world.objects.push(
        Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Material::new(Colour::new(0.8,0.8,0.0), false, 0.0)));

    world.objects.push(
        Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            Material::new(Colour::new(0.8,0.6,0.2), true, 0.3)));

    world.objects.push(
        Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            Material::new(Colour::new(0.8,0.8,0.8), true, 1.0)));

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
                let c = trace_ray(r, &world, 0);

                col.r += c.r;
                col.g += c.g;
                col.b += c.b;

            }

            image.pixel(x, y).r = col.r / (n_rays as f32);
            image.pixel(x, y).g = col.g / (n_rays as f32);
            image.pixel(x, y).b = col.b / (n_rays as f32);


        }
    }



    let result = image.write_ppm(Path::new("test_image.ppm"));
    match result {
        Ok(_) => println!("Wrote image to {:?}", "test_image.ppm"),
        Err(e) => println!("Error writing: {:?}", e),
    }

}
