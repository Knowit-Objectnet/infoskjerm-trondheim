use image::io::Reader as ImageReader;
use slint::{Image, Rgba8Pixel, SharedPixelBuffer};
use std::io::Cursor;

pub fn get_current_xkcd_image() -> Image {
    //TODO: Error handling
    let response = reqwest::blocking::get("https://imgs.xkcd.com/comics/what_to_do_2x.png")
        .expect("Failed to download XKCD image");

    let image_data = response.bytes().expect("Failed to read image data");

    // Wrap the image data in a `Cursor` to allow reading from it
    let cursor = Cursor::new(image_data.as_ref());

    // Decode the image into a `RgbaImage` from the `image` crate
    let dynamic_image = ImageReader::new(cursor)
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    let rgba_image = dynamic_image.into_rgba8();

    let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
        rgba_image.as_raw(),
        rgba_image.width(),
        rgba_image.height(),
    );
    Image::from_rgba8(buffer)
}
