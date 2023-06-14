use clap::Parser;
use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::{GenericImageView, Pixel};
use std::alloc::System;
use std::io::Cursor;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(short, long)]
    path: String,
}

const WIDTH_SCALE: u32 = 2;

fn main() {
    let args = Args {
        path: String::from("-p uran.jpg"),
    };
    let mut img = ImageReader::open(Args::try_parse().unwrap_or(args).path)
        .unwrap_or_else(|_| {
            println!("Failed to open image");
            std::process::exit(1)
        })
        .decode()
        .unwrap_or_else(|_| {
            println!("Failed to decode image");
            std::process::exit(2)
        })
        .grayscale();

    let (width, height) = {
        let (terminal_width, terminal_height) = termion::terminal_size().unwrap();
        let scale = std::cmp::max(
            (img.width() * WIDTH_SCALE / (terminal_width as u32 - 1)),
            img.height() / terminal_height as u32,
        );

        ((img.width() / scale * WIDTH_SCALE), img.height() / scale)
    };

    img = img.resize(width, height, FilterType::Nearest);

    let character_set: [&str; 11] = ["@", "#", "0", "O", "L", ";", ":", ".", ",", "'", " "];

    let mut art = String::new();
    let mut last_y = 0;
    for pixel in img.pixels() {
        if last_y != pixel.1 {
            art.push_str("\n");
            last_y = pixel.1;
        }

        let pixel_data = pixel.2;
        let brightness: f64 =
            ((pixel_data[0] as u64 + pixel_data[1] as u64 + pixel_data[2] as u64) / 3) as f64;
        let character_position =
            ((brightness / 255.0) * (character_set.len() - 1) as f64).round() as usize;
        art.push_str(character_set[character_position])
    }

    println!("{}", art);
}
