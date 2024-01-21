use crate::ui::*;

use image::io::Reader as ImageReader;
use log::info;
use serde::{Deserialize, Serialize};
use slint::{Image, Rgba8Pixel, SharedPixelBuffer, Weak};
use std::{error, io::Cursor, thread};

const XKCD_URL: &str = "https://xkcd.com/info.0.json";

#[derive(Debug, Deserialize, Serialize)]
pub struct XkcdJson {
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

pub fn setup(window: &MainWindow) {
    let window_weak = window.as_weak();
    thread::spawn(move || {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(xkcd_worker_loop(window_weak))
    });
}

async fn xkcd_worker_loop(window: Weak<MainWindow>) {
    loop {
        let xkcd = get_current_xkcd().await;
        display_xkcd(&window, xkcd.unwrap()).await;
        tokio::time::sleep(std::time::Duration::from_secs(60 * 15)).await;
    }
}

async fn display_xkcd(window_weak: &Weak<MainWindow>, xkcd_metadata: XkcdJson) {

    let img_shared_pixel_buffer = get_current_xkcd_image(xkcd_metadata.img).await;

    window_weak
        .upgrade_in_event_loop(move |window: MainWindow| {
            let image = Image::from_rgba8(img_shared_pixel_buffer.unwrap());

            let xkcd = Xkcd {
                title: xkcd_metadata.title.into(),
                image,
                flavor_text: xkcd_metadata.alt.into(),
            };

            window.set_xkcd(xkcd);
        })
        .unwrap();
}

pub async fn get_current_xkcd() -> Result<XkcdJson, Box<dyn error::Error>> {
    let response = reqwest::get(XKCD_URL).await?;

    let xkcd_metadata = response.json::<XkcdJson>().await?;

    info!("Loaded xkcd: {}", xkcd_metadata.title);

    Ok(xkcd_metadata)
}

pub async fn get_current_xkcd_image(url: String) -> Result<SharedPixelBuffer<Rgba8Pixel>, reqwest::Error> {
    //TODO: Error handling, aka don't crash if not able to load image
    let response = reqwest::get(url).await?;
    let image_data = response.bytes().await.expect("Failed to read image data");

    // Wrap the image data in a `Cursor` to allow reading from it
    let cursor = Cursor::new(image_data.as_ref());

    let dynamic_image = ImageReader::new(cursor)
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    
    let rgba_image = dynamic_image.into_rgba8();

    let buffer: SharedPixelBuffer<Rgba8Pixel> = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
        rgba_image.as_raw(),
        rgba_image.width(),
        rgba_image.height(),
    );
    Ok(buffer)
}
