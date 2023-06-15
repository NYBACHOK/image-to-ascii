use clap::Parser;
use eyre::{Result};
use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::GenericImageView;
use std::fs::File;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(short, long)]
    path: String,

    #[arg(short, long, default_value = Option::None)]
    output: Option<String>,
}

const WIDTH_SCALE: u32 = 2;

fn main() -> Result<(), ()> {
    let args = Args::parse();

    let mut img = ImageReader::open(args.path)
        .unwrap_or_else(|_| {
            eprintln!("Failed to open image");
            std::process::exit(1)
        })
        .decode()
        .unwrap_or_else(|_| {
            eprintln!("Failed to decode image");
            std::process::exit(2)
        })
        .grayscale();

    let (width, height) = match args.output {
        None => {
            let (terminal_width, terminal_height) = termion::terminal_size().unwrap();
            let scale = std::cmp::max(
                img.width() * WIDTH_SCALE / (terminal_width as u32 - 1),
                img.height() / terminal_height as u32,
            );

            ((img.width() / scale * WIDTH_SCALE), img.height() / scale)
        }
        Some(_) => (img.width(), img.height()),
    };

    img = img.resize(width, height, FilterType::Nearest);

    const CHARACTER_SET: [&str; 11] = ["@", "#", "0", "O", "L", ";", ":", ".", ",", "'", " "];

    let mut art = String::with_capacity((img.height() * img.width()) as usize);
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
            ((brightness / 255.0) * (CHARACTER_SET.len() - 1) as f64).round() as usize;
        art.push_str(CHARACTER_SET[character_position])
    }

    match args.output {
        None => {
            println!("{}", art);
        }
        Some(path) => {
            let mut file = File::create(path).unwrap_or_else(|_| {
                eprintln!("Failed to write to file");
                std::process::exit(3)
            });

            file.flush().unwrap();
            file.write_all(art.as_bytes()).unwrap();
        }
    };

    Ok(())
}
