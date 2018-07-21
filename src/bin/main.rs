#![allow(unused_imports)]

extern crate solaire;

use solaire::vector::{Vector};
use solaire::camera::{Camera};
use solaire::light::{Ray, Colour, HitResult};
use solaire::material::{NormalSurface, Diffuse, DiffuseLight};
use solaire::world::{World, Geometry, SimpleObject};
use solaire::geometry::cuboid::{Cuboid};
use solaire::geometry::sphere::{Sphere};
use solaire::image::{Bitmap};
use solaire::stats::{Stats};
use solaire::random::{thread_rng};
use std::path::Path;
use std::time::SystemTime;

fn trace_ray(r: &Ray, world: &World, stats: &mut Stats, depth: i32) -> Colour {

    let hit_result = world.propagate_ray(&r, 0.001, 1000.0);
    let background = Colour::new(0.0, 0.0, 0.0);

    stats.rays += 1;

    match hit_result {
        HitResult::Emitted(_t, colour) => {
            stats.hit_emitter += 1;
            return colour;
        },
        HitResult::Reflected(_t, reflected_ray) => {
            if depth > 5 {
                stats.hit_maxdepth += 1;
                return background;
            } else {
                return r.colour * trace_ray(&reflected_ray, world, stats, depth + 1);
            }
        },
        HitResult::None => {
            stats.hit_nothing += 1;
            return background;
        }
    }
}

fn generate_rays(camera: &Camera, x: u32, y: u32, width: u32, height: u32, n_rays: usize) -> Vec<Ray> {

    let mut rng = thread_rng();
    let mut rays = Vec::<Ray>::with_capacity(n_rays);

    for _n in 0..n_rays {
        let px: f32 = rng.gen();
        let py: f32 = rng.gen();

        let u = ((x as f32) + px) / width as f32;
        let v = 1.0 - ((y as f32) + py) / height as f32;

        rays.push(camera.get_ray(u, v));
    }

    return rays;
}


fn construct_world(world: &mut World) {

    world.push_object(Box::new(SimpleObject::new(
        Box::new(Cuboid::new(1.0)),
        //Box::new(NormalSurface::new())
        Box::new(Diffuse::new(Colour::white()))
    )));

    world.push_object(Box::new(SimpleObject::new(
        Box::new(Cuboid::new(0.1)),
        Box::new(DiffuseLight::new(Colour::new(1.0, 1.0, 1.0)))
    )));

}


fn main() {

    let width = 500;
    let height = 500;
    let n_rays = 100;

    let camera = Camera::new(
        Vector::new(0.0, 0.0, -1.0),
        Vector::new(0.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        90.0,
        width as f32 / height as f32);

    let mut world = World::new();

    construct_world(&mut world);

    let mut image = Bitmap::new(width, height);
    let mut stats = Stats::new(SystemTime::now());

    for y in 0..height {
        for x in 0..width {
            let mut colour = Colour::black();

            for ray in generate_rays(&camera, x, y, width, height, n_rays) {
                colour = colour + trace_ray(&ray, &world, &mut stats, 0);
            }

            image.pixel(x, y).r = colour.r / (n_rays as f32);
            image.pixel(x, y).g = colour.g / (n_rays as f32);
            image.pixel(x, y).b = colour.b / (n_rays as f32);
        }
    }

    stats.end_time = SystemTime::now();

    println!("Rendered image in {:.6}s", stats.duration());
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

    println!("Praise the sun!");

}
