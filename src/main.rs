extern crate rustc_serialize;
extern crate term_painter;

use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::fmt;
use rustc_serialize::json;

use term_painter::ToStyle;
use term_painter::Color::*;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Slide {
    title: String,
    bullet_points: Vec<String>,
}

impl fmt::Debug for Slide {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "title: {}, bullet_points: {:?}", self.title, self.bullet_points)
    }
}

fn enter_to_go () {
    let mut stdin = std::io::stdin();
    stdin.read_line(&mut String::new()).unwrap();
}

fn read_slides (path: &str) {
    let mut file = File::open(path).unwrap();
    let mut contents: Vec<u8> = Vec::new();

    file.read_to_end(&mut contents).unwrap();

    let filestr = String::from_utf8(contents).unwrap();

    let slides: Vec<Slide> = json::decode(&filestr).unwrap();

    fn print_yellow(str: &str) {
        println!("{}", Yellow.paint(str));
    }

    print_yellow("Bonjour! ಠ_ರೃ ");
    print_yellow("Je suis Lumière, your personal projector.");
    print_yellow("Press enter to start.\n");

    for i in 0..slides.len() {
        let ref slide = slides[i];
        println!(">>> {} [{}/{}]", Green.paint(&slide.title), i + 1, slides.len());
        enter_to_go();
        for point in &slide.bullet_points {
            println!("> {}", point);
            enter_to_go();
        }
    }

    print_yellow("All slides have been displayed.");
    print_yellow("Thank you for using Lumière.");
    print_yellow("Au revoir! ಠ_ರೃ  \n");
}

fn main () {
    if let Some(file) = env::args().nth(1) {
        read_slides(&file);
    }
}
