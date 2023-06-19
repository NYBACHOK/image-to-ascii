use crate::character_set::CHARACTER_SET;
use crate::resize::resize_frame;

use eyre::Result;
use opencv::core::*;
use opencv::imgproc;
use opencv::prelude::*;
use opencv::videoio::{VideoCapture, CAP_ANY, CAP_FFMPEG, CAP_ARAVIS};
use rgb::RGB8;

use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

#[inline]
fn brightness(pixel: &RGB8) -> f32 {
    0.3 * pixel.r as f32 + 0.59 * pixel.g as f32 + 0.11 * pixel.b as f32
}

fn pixel_to_ascii(pixel: &RGB8) -> &str {
    let brightness = brightness(pixel);
    let max_idx = CHARACTER_SET.len();
    let idx = (brightness / 256.0 * max_idx as f32).floor() as usize;
    CHARACTER_SET[idx]
}

pub fn render_ascii(img: &Mat, _colored: bool) -> String {
    let mut buf = String::with_capacity((img.rows() * (img.cols() + 1)) as usize);
    for i in 0..img.rows() {
        for j in 0..img.cols() {
            let pixel = img.at_2d::<RGB8>(i, j).unwrap();
            let ch = pixel_to_ascii(pixel);
            buf.push_str(ch);
        }
        buf.push('\n');
    }
    buf
}

pub fn bgr2rgb(img: Mat) -> Result<Mat> {
    let mut rgb = Mat::default();
    imgproc::cvt_color(&img, &mut rgb, imgproc::COLOR_BGR2RGB, 0)?;
    Ok(rgb)
}

pub fn start_playing(path: &str) -> Result<()> {
    let mut file = VideoCapture::from_file(path, CAP_ANY).unwrap();
    let (tx, rx) = mpsc::sync_channel(100);

    thread::spawn(move || {
        let mut img = Mat::default();
        while file.read(&mut img).unwrap_or(false) {
            if let Ok(img) = resize_frame(&img) {
                if let Ok(img) = bgr2rgb(img) {
                    let buf = render_ascii(&img, false);
                    tx.send(buf).unwrap();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    });

    while let Ok(buf) = rx.recv() {
        let timer = Instant::now();
        if String::is_empty(&buf) {
            break;
        } else {
            print!("{}", buf);
            println!("{0}", termion::clear::All)
        }
        thread::sleep(Duration::from_micros(41666) - timer.elapsed())
    }

    Ok(())
}
