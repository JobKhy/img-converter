#![deny(clippy::all)]
use image::{open, ImageFormat};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::io::Cursor;
use std::path::Path;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn convert_image(input_path: String, output_path: String, format: String) -> Result<()> {
  let img =
    open(&input_path).map_err(|_| Error::new(Status::GenericFailure, "Error loading image"))?;

  let output_format = match format.as_str() {
    "png" => ImageFormat::Png,
    "jpg" | "jpeg" => ImageFormat::Jpeg,
    "bmp" => ImageFormat::Bmp,
    "gif" => ImageFormat::Gif,
    "tiff" => ImageFormat::Tiff,
    "webp" => ImageFormat::WebP,
    _ => return Err(Error::new(Status::InvalidArg, "Unsupported format")),
  };

  img
    .save_with_format(Path::new(&output_path), output_format)
    .map_err(|_| Error::new(Status::GenericFailure, "Error saving image"))?;

  Ok(())
}

#[napi]
pub fn convert_image_file(input_data: Buffer, format: String) -> Result<Buffer> {
  let img = image::load_from_memory(&input_data)
    .map_err(|_| Error::new(Status::GenericFailure, "Error loading image from buffer"))?;

  let output_format = match format.as_str() {
    "png" => ImageFormat::Png,
    "jpg" | "jpeg" => ImageFormat::Jpeg,
    "bmp" => ImageFormat::Bmp,
    "gif" => ImageFormat::Gif,
    "tiff" => ImageFormat::Tiff,
    "webp" => ImageFormat::WebP,
    _ => return Err(Error::new(Status::InvalidArg, "Unsupported format")),
  };

  let mut output_data: Vec<u8> = Vec::new();

  let mut cursor = Cursor::new(&mut output_data);

  img
    .write_to(&mut cursor, output_format)
    .map_err(|_| Error::new(Status::GenericFailure, "Error saving image to buffer"))?;

  Ok(Buffer::from(output_data))
}
