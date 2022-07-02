extern crate termion;
extern crate image;
use termion::{color, style};
use image::GenericImageView;
use std::env;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Colour {
    r: u8,
    g: u8,
    b: u8
}

impl Colour {
    fn new(red: u8, green: u8, blue: u8) -> Colour {
        Colour {
            r: red,
            g: green,
            b: blue
        }
    }

    #[allow(dead_code)]
    fn p_rgb(&self) {
        println!("{}[{},{},{}]{}",
                 color::Rgb(self.r, self.g, self.b).bg_string(),
                 self.r,self.g,self.b,
                 style::Reset
        );
    }

    #[allow(dead_code)]
    fn p_hex(&self) {
        println!("{}#{:02X}{:02X}{:02X}{}",
                 color::Rgb(self.r, self.g, self.b).bg_string(),
                 self.r as f32 as u8,
                 self.g as f32 as u8,
                 self.b as f32 as u8,
                 style::Reset
        )
    }

    fn to_hex(&self) -> String {
        format!("{}#{:02X}{:02X}{:02X}{}",
                color::Rgb(self.r, self.g, self.b).bg_string(),
                self.r as f32 as u8,
                self.g as f32 as u8,
                self.b as f32 as u8,
                style::Reset
        )
    }
}

impl std::fmt::Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}#{}{}",
               color::Rgb(self.r, self.g, self.b).bg_string(),
               self.to_hex(),
               style::Reset
        )
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // TODO: Better error messages
    if args.len() == 1 {panic!("Expected more at least one argument.")}
    let img = image::open(&args[1]).expect("Invalid image");

    let mut hm: HashMap<Colour, i32> = HashMap::new();

    for x in 0..img.dimensions().0 {
        for y in 0..img.dimensions().1 {
            let pixel = img.get_pixel(x,y);
            let c = Colour::new(pixel[0], pixel[1], pixel[2]);
            *hm.entry(c).or_insert(0) += 1;
        }
    }

    let total: u32 = img.dimensions().0 * img.dimensions().1;
    // TODO: Sorted list of colours
    for (colour, count) in &hm {
        println!("{} | {:.2}%", colour.to_hex(), (*count as f32)*100.0/total as f32);
    }
}
