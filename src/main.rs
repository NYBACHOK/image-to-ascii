use clap::Parser;
use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::{GenericImageView};



#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(short, long)]
    path: String,
}

const WIDTH_SCALE: u32 = 2;

fn main() {
    let mut img = ImageReader::open(Args::parse().path)
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

    let (width, height) = {
        let (terminal_width, terminal_height) = termion::terminal_size().unwrap();
        let scale = std::cmp::max(
            img.width() * WIDTH_SCALE / (terminal_width as u32 - 1),
            img.height() / terminal_height as u32,
        );

        ((img.width() / scale * WIDTH_SCALE), img.height() / scale)
    };

    img = img.resize(width, height, FilterType::Nearest);

    const CHARACTER_SET: [&str; 11] = ["@", "#", "0", "O", "L", ";", ":", ".", ",", "'", " "];

    let mut art = String::with_capacity((img.height()*img.width()) as usize);
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

    println!("{}", art);
}
