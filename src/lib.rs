use std::fs;
use std::iter::Iterator;

use clap::Parser;
use image;
use image::{DynamicImage, Pixel};
use image::imageops;
use imageproc::drawing::Canvas;
use termsize;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(short, long, value_name = "IMAGE_PATH")]
    pub image: String,

    #[arg(short, long, value_name = "OUTPUT_PATH")]
    pub output: String,

    #[arg(long, value_name = "W")]
    pub width: Option<u32>,

    #[arg(long, value_name = "H")]
    pub height: Option<u32>,
}

pub fn parse_config() -> Config {
    Config::parse()
}

pub fn read_image(file_name: &str) -> DynamicImage {
    image::open(file_name).unwrap()
}

pub fn write_text(file_name: String, lines: Vec<String>) {
    fs::write(file_name, lines.join("\n")).unwrap();
}

pub fn img_to_ascii_art(img: DynamicImage, config: & Config) -> Vec<String> {
    let (w, h) = img.dimensions();
    let (w, h) = build_target_dimensions(w, h, config.width, config.height);
    generate_ascii_art(img, w, h)
}

fn build_target_dimensions(img_w: u32, img_h: u32, target_w: Option<u32>, target_h: Option<u32>) -> (u32, u32) {
    let w;
    let h;
    match (target_w, target_h) {
        (Some(ww), Some(hh)) => {
            w = ww;
            h = hh;
        }
        (None, Some(hh)) => {
            w = (img_w as f32 / img_h as f32 * hh as f32) as u32;
            h = hh;
        }
        (Some(ww), None) => {
            w = ww;
            h = (img_h as f32 / img_w as f32 * ww as f32) as u32;
        }
        _ => {
            let term_size = termsize::get().unwrap();
            let tw = term_size.cols as u32;
            let th = term_size.rows as u32;

            let img_aspect_ratio = img_w as f32 / img_h as f32;

            if tw as f32 / th as f32 > img_aspect_ratio {
                h = th;
                w = (th as f32 * img_aspect_ratio) as u32;
            } else {
                w = tw;
                h = (tw as f32 / img_aspect_ratio) as u32;
            }
            println!("Get term size: {tw}, {th}");
        }
    };
    if w == 0 {
        panic!("Width {w} is too small")
    }
    if h == 0 {
        panic!("Height {h} is too small")
    }
    return (w, h);
}

const BRIGHTNESS_CHARS: &[u8] = ".\'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxuvczUJCLQOZmwqpdbkho*#MW&8%B@$".as_bytes();
const BRIGHTNESS_FACTOR: u8 = ((u8::MAX as usize + 1) / BRIGHTNESS_CHARS.len()) as u8;

fn generate_ascii_art(image: DynamicImage, target_w: u32, target_h: u32) -> Vec<String> {
    let image = imageops::grayscale(&image);
    let image = imageops::resize(&image, target_w, target_h, imageops::FilterType::Nearest);

    let mut res: Vec<String> = Vec::with_capacity(image.height() as usize);
    for row in image.rows() {
        let s: String = row.map(|pixel| {
            let brightness = pixel.channels()[0] / BRIGHTNESS_FACTOR;
            BRIGHTNESS_CHARS[brightness as usize] as char
        }).collect();
        res.push(s);
    }
    res
}
