use super::Xkcd;

use image::io::Reader as ImageReader;
use log::info;
use serde::{Deserialize, Serialize};
use slint::{Image, Rgba8Pixel, SharedPixelBuffer};
use std::{error, io::Cursor};

#[derive(Debug, Deserialize, Serialize)]
struct XkcdJson {
    pub month: String,
    pub num: i64,
    pub link: String,
    pub year: String,
    pub news: String,
    pub safe_title: String,
    pub transcript: String,
    pub alt: String,
    pub img: String,
    pub title: String,
    pub day: String,
}

pub fn get_current_xkcd() -> Result<Xkcd, Box<dyn error::Error>> {
    let url = "https://xkcd.com/info.0.json";
    let response = reqwest::blocking::get(url)?;

    let xkcd_metadata = response.json::<XkcdJson>()?;

    let image = get_current_xkcd_image(xkcd_metadata.img)?;

    let xkcd = Xkcd {
        title: xkcd_metadata.title.into(),
        image,
        flavor_text: xkcd_metadata.alt.into(),
    };

    info!("Loaded xkcd: {}", xkcd.title);

    Ok(xkcd)
}

pub fn get_current_xkcd_image(url: String) -> Result<Image, reqwest::Error> {
    //TODO: Error handling, aka don't crash if not able to load image
    let response = reqwest::blocking::get(url)?;
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
    Ok(Image::from_rgba8(buffer))
}
