use anyhow::{Context, Result};
use image::{DynamicImage, ImageFormat};
use std::io::Cursor;
use windows::{
    Graphics::Imaging::{BitmapAlphaMode, BitmapPixelFormat, SoftwareBitmap},
    Media::Ocr::OcrEngine,
    Storage::Streams::{DataWriter, InMemoryRandomAccessStream},
};

pub fn recognize(image: &DynamicImage) -> Result<String> {
    let rgba = image.to_rgba8();
    let mut png = Cursor::new(Vec::new());
    DynamicImage::ImageRgba8(rgba).write_to(&mut png, ImageFormat::Png)?;
    let stream = InMemoryRandomAccessStream::new()?;
    let writer = DataWriter::CreateDataWriter(&stream)?;
    writer.WriteBytes(png.get_ref())?;
    writer.StoreAsync()?.get()?;
    writer.FlushAsync()?.get()?;
    stream.Seek(0);

    let decoder = windows::Graphics::Imaging::BitmapDecoder::CreateAsync(
        windows::Graphics::Imaging::BitmapDecoder::PngDecoderId,
        &stream,
    )?
    .get()?;
    let bitmap = decoder
        .GetSoftwareBitmapAsync()?
        .get()?
        .Convert(BitmapPixelFormat::Rgba8, BitmapAlphaMode::Premultiplied)?;
    recognize_bitmap(&bitmap)
}

fn recognize_bitmap(bitmap: &SoftwareBitmap) -> Result<String> {
    let engine = OcrEngine::TryCreateFromUserProfileLanguages()?
        .context("Windows OCR has no supported language installed")?;
    let result = engine.RecognizeAsync(bitmap)?.get()?;
    let mut lines = Vec::new();
    for line in result.Lines()? {
        lines.push(line.Text()?.to_string());
    }
    Ok(lines.join("\n"))
}

pub fn language_hint() -> Result<String> {
    let engine = OcrEngine::TryCreateFromUserProfileLanguages()?
        .context("Windows OCR has no supported language installed")?;
    Ok(engine.RecognizerLanguage()?.Language()?.to_string())
}
