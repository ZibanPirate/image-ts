use image::{
  imageops::{crop, resize},
  ImageFormat,
};
use std::io::Cursor;
use wasm_bindgen::prelude::*;

// resize image with object-fit: cover
#[wasm_bindgen]
pub fn resize_image(image_binary: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
  let image = image::load_from_memory(&image_binary).unwrap();

  let original_aspect_ratio = image.width() as f32 / image.height() as f32;

  let desired_aspect_ratio = width as f32 / height as f32;

  // find the new image size in which both width and height are equal to or larger than the desired size
  let (new_width, new_height) = if original_aspect_ratio < desired_aspect_ratio
  {
    (width, (width as f32 / original_aspect_ratio) as u32)
  } else {
    ((height as f32 * original_aspect_ratio) as u32, height)
  };

  let mut image = resize(
    &image,
    new_width,
    new_height,
    image::imageops::FilterType::Triangle,
  );

  let center_of_resized_image = (new_width / 2, new_height / 2);

  let cropped_image = crop(
    &mut image,
    (center_of_resized_image.0 - width / 2).max(0),
    (center_of_resized_image.1 - height / 2).max(0),
    width,
    height,
  );
  let image = cropped_image.to_image();

  let mut result_image_binary = Cursor::new(Vec::new());
  image
    .write_to(&mut result_image_binary, ImageFormat::Png)
    .unwrap();

  result_image_binary.into_inner()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_resize_image() {
    let image_binary = include_bytes!("../../../test/sample_1.png").to_vec();
    let width = 300;
    let height = 100;

    let result = resize_image(image_binary, width, height);

    use std::fs::File;
    use std::io::prelude::*;
    let mut file = File::create("../test/sample_1_rust_resized.png").unwrap();
    file.write_all(&result).unwrap();
  }
}
