use std::ops as ops;
use std::cmp;
use std::error::Error;
use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::path::Path;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Colour {
    pub fn new(r: f32, g:f32, b:f32) -> Colour {
        Colour { r: r, g: g, b: b }
    }

    pub fn black() -> Colour {
        Colour { r: 0.0, g: 0.0, b: 0.0 }
    }

    pub fn white() -> Colour {
        Colour { r: 1.0, g: 1.0, b: 1.0 }
    }

    pub fn discretise(self, max: u32) -> (u32, u32, u32) {
        let x = max as f32;
        (cmp::min((self.r * x) as u32, max-1),
         cmp::min((self.g * x) as u32, max-1),
         cmp::min((self.b * x) as u32 , max-1))
    }

    pub fn gamma_correct(self) -> Colour {
        Colour { r: self.r.sqrt(),
                 g: self.g.sqrt(),
                 b: self.b.sqrt() }
    }

}

impl ops::Mul<Colour> for Colour {
    type Output = Colour;

    fn mul(self, c: Colour) -> Colour {
        Colour { r: self.r * c.r,
                 g: self.g * c.g,
                 b: self.b * c.b }
    }
}

pub fn lerp(t: f32, c1: Colour, c2: Colour) -> Colour {
    Colour { r: (t * c1.r) + (1.0 - t) * c2.r,
             g: (t * c1.g) + (1.0 - t) * c2.g,
             b: (t * c1.b) + (1.0 - t) * c2.b }
}

pub struct Bitmap {
    width: u32,
    height: u32,
    pixels: Vec<Colour>
}

impl Bitmap {

    pub fn new(width: u32, height: u32) -> Bitmap {
        let size = (width * height) as usize;
        Bitmap { width: width,
                 height: height,
                 pixels: vec![Colour::white(); size] }
    }


    pub fn pixel(&mut self, x: u32, y: u32) -> &mut Colour {
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
