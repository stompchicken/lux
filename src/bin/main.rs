extern crate lux;

use lux::vector::{Vec3};
use lux::camera::{Camera};
use lux::light::{Ray, Colour};
use lux::material::{Diffuse, DiffuseLight};
use lux::world::{RectXY, RectYZ, RectXZ, World};
use lux::image::{Bitmap};
use lux::stats::{Stats};
use lux::random::{thread_rng};
use std::path::Path;
use std::time::SystemTime;

fn trace_ray(r: &Ray, world: &World, stats: &mut Stats, depth: i32) -> Colour {

    let hit_result = world.propagate_ray(&r, 0.001, 1000.0);
    let background = Colour::new(0.0, 0.0, 0.0);

    stats.rays += 1;

    match hit_result.emitted {
        Some(colour) => {
            stats.hit_emitter += 1;
            return colour;
        },
        None => {
            match hit_result.reflected {
                Some(ray) => {
                    if depth < 10 {
                        ray.colour * trace_ray(&ray, world, stats, depth + 1)
                    } else {
                        stats.hit_maxdepth += 1;
                        return background;
                    }
                },
                None => {
                    stats.hit_nothing += 1;
                    return background;
                }
            }
        }
    }
}

fn main() {

    let width = 500;
    let height = 500;
    let n_rays = 100;

    let camera = Camera::new(
        Vec3::new(0.0, 0.0, -0.5),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        55.0,
        width as f32 / height as f32);

    let mut world = World::new();

    let white = Diffuse::new(Colour::new(1.0, 1.0, 1.0));

    // Top
    world.push_object(
        Box::new(RectXZ::new(-1.0, 1.0, -1.0, 1.0, 1.0, true)),
        Box::new(white));

    // Bottom
    world.push_object(
        Box::new(RectXZ::new(-1.0, 1.0, -1.0, 1.0, -1.0, false)),
        Box::new(white));

    // Front
    world.push_object(
        Box::new(RectXY::new(-1.0, 1.0, -1.0, 1.0, -1.0, false)),
        Box::new(white));

    // Back
    world.push_object(
        Box::new(RectXY::new(-1.0, 1.0, -1.0, 1.0, 1.0, true)),
        Box::new(white));

    // Left
    world.push_object(
        Box::new(RectYZ::new(-1.0, 1.0, -1.0, 1.0, 1.0, true)),
        Box::new(Diffuse::new(Colour::new(0.65, 0.05, 0.05))));

    // Right
    world.push_object(
        Box::new(RectYZ::new(-1.0, 1.0, -1.0, 1.0, -1.0, false)),
        Box::new(Diffuse::new(Colour::new(0.12, 0.45, 0.15))));

    // Light
    world.push_object(
        Box::new(RectXZ::new(-0.25, 0.25, 0.60, 0.70, 1.0, true)),
        Box::new(DiffuseLight::new(Colour::new(5.0, 5.0, 5.0))));

//    world.push_object(
//        Box::new(Cube::new(0.1)),
//        Box::new(Diffuse::new(Colour::new(0.0, 0.0, 1.0))));

    let mut image = Bitmap::new(width, height);

    let mut rng = thread_rng();

    let mut stats = Stats::new(SystemTime::now());

    for y in 0..height {
        for x in 0..width {
            let mut col = Colour::black();

            for _n in 0..n_rays {

                let px: f32 = rng.gen();
                let py: f32 = rng.gen();

                let u = ((x as f32) + px) / width as f32;
                let v = 1.0 - ((y as f32) + py) / height as f32;

                let r = camera.get_ray(u, v);
                let c = trace_ray(&r, &world, &mut stats, 0);

                col.r += c.r;
                col.g += c.g;
                col.b += c.b;

            }

            image.pixel(x, y).r = col.r / (n_rays as f32);
            image.pixel(x, y).g = col.g / (n_rays as f32);
            image.pixel(x, y).b = col.b / (n_rays as f32);
        }
    }

    stats.end_time = SystemTime::now();

    println!("Rendered image in {:.2}s", stats.duration());
    println!("{:.2} Mrays/s", stats.mega_rays_per_second());

    println!("{} rays", stats.rays);
    println!("{:.2}% hits to emitters", (100.0 * stats.hit_emitter as f32) / (stats.hits() as f32));
    println!("{:.2}% hits to max_depth", (100.0 * stats.hit_maxdepth as f32) / (stats.hits() as f32));
    println!("{:.2}% hits to nothing", (100.0 * stats.hit_nothing as f32) / (stats.hits() as f32));

    let result = image.write_ppm(Path::new("test_image.ppm"));
    match result {
        Ok(_) => println!("Wrote image to {:?}", "test_image.ppm"),
        Err(e) => println!("Error writing: {:?}", e),
    }

}
