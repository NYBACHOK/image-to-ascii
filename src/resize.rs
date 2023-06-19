use opencv::core::{Mat, MatTraitConstManual, Size_};
use opencv::imgproc;
use opencv::Result;

const WIDTH_SCALE: u32 = 2;

pub fn get_resized(width: u32, height: u32) -> (u32, u32) {
    let (terminal_width, terminal_height) = termion::terminal_size().unwrap();
    let scale = std::cmp::max(
        width * WIDTH_SCALE / (terminal_width as u32 - 1),
        height / terminal_height as u32,
    );

    ((width / scale * WIDTH_SCALE), height / scale)
}

pub fn resize_frame(img: &Mat) -> Result<Mat> {
    let img_size = img.size()?;

    let (width, height) = get_resized(img_size.width as u32, img_size.height as u32);

    let mut resized = Mat::default();
    imgproc::resize(
        &img,
        &mut resized,
        Size_::new(0, 0),
        width as f64,
        height as f64,
        imgproc::INTER_CUBIC,
    )?;
    Ok(resized)
}
