use image::io::Reader as ImageReader;
use slint::{Image, Rgba8Pixel, SharedPixelBuffer};
use std::io::Cursor;

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

pub struct Xkcd {
    pub title: String,
    pub image: Image,
    pub flavor_text: String,
}

pub fn get_current_xkcd() -> Xkcd {
    let xkcd = Xkcd {
        title: "Hehe".to_owned(),
        image: get_current_xkcd_image("https://imgs.xkcd.com/comics/what_to_do_2x.png".to_owned()),
        flavor_text: "Flavortown".to_owned(),
    };

    xkcd
}

pub fn get_current_xkcd_image(url: String) -> Image {
    //TODO: Error handling, aka don't crash if not able to load image
    let response = reqwest::blocking::get(url).expect("Failed to download XKCD image");
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
