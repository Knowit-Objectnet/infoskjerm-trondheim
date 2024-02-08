use std::{error, io::Cursor, thread};

use tide::Request;
use tokio::runtime::Runtime;

pub fn setup() {
    thread::spawn(move || Runtime::new().unwrap().block_on(food_endpoint_server()));
}

async fn food_endpoint_server() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/id").post(get_tracking_ID);
    app.at("/").serve_file("src/food/index.html")?;
    app.listen("127.0.0.1:1337").await?;
    Ok(())
}

async fn get_tracking_ID(mut req: Request<()>) -> tide::Result {
    Ok(format!("Food!").into())
}
