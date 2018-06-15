use std::cmp;
use std::error::Error;
use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::path::Path;

use light::{Colour};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Pixel {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Pixel {
    pub fn new(r: f32, g:f32, b:f32) -> Pixel {
        Pixel { r: r, g: g, b: b }
    }

    pub fn from_colour(colour: Colour) -> Pixel {
        Pixel { r: colour.r, g: colour.g, b: colour.b }
    }

    pub fn discretise(self, max: u32) -> (u32, u32, u32) {
        let x = max as f32;
        (cmp::min((self.r * x) as u32, max-1),
         cmp::min((self.g * x) as u32, max-1),
         cmp::min((self.b * x) as u32 , max-1))
    }

    pub fn gamma_correct(self) -> Pixel {
        Pixel { r: self.r.powf(0.5),
                g: self.g.powf(0.5),
                b: self.b.powf(0.5)
        }
    }

}

pub struct Bitmap {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>
}

impl Bitmap {

    pub fn new(width: u32, height: u32) -> Bitmap {
        let size = (width * height) as usize;
        Bitmap { width: width,
                 height: height,
                 pixels: vec![Pixel::new(0.0,0.0,0.0); size] }
    }


    pub fn pixel(&mut self, x: u32, y: u32) -> &mut Pixel {
        &mut self.pixels[(y*self.width + x) as usize]
    }

    pub fn write_ppm(self, path: &Path) -> io::Result<()> {
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}",
                               display,
                               why.description()),
            Ok(file) => file,
        };

        let max_value = 256;

        file.write("P3\n".as_bytes())?;
        file.write(format!("{} {}\n", self.width, self.height).as_bytes())?;
        file.write(format!("{}\n", max_value).as_bytes())?;

        for (i, pixel) in self.pixels.iter().enumerate() {
            let (r, g, b) = pixel.gamma_correct().discretise(max_value);
            let rgb = format!("{} {} {} ", r, g, b);
            file.write(rgb.as_bytes())?;
            if (i+1) % (self.width as usize) == 0 {
                file.write("\n".as_bytes())?;
            }
        }

        Ok(())
    }
}
