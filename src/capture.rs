use anyhow::{Context, Result};
use image::DynamicImage;
use screenshots::Screen;

pub fn capture_region(x: i32, y: i32, width: u32, height: u32) -> Result<DynamicImage> {
    let screen = Screen::from_point(x, y).context("no screen found at the requested point")?;
    let image = screen.capture_area(x, y, width, height)?;
    let buffer = image.buffer().to_vec();
    let image = image::RgbaImage::from_raw(image.width(), image.height(), buffer)
        .context("captured image has an invalid pixel buffer")?;
    Ok(DynamicImage::ImageRgba8(image))
}
