use std::sync::mpsc::Sender;

use reqwest::Url;
use serde::Deserialize;
use tide::Request;

pub async fn food_endpoint_server(tx: Sender<Url>) -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/tracking")
        .post(move |req| start_tracking(tx.clone(), req));
    app.at("/food").serve_file("src/food/index.html")?;
    app.listen("127.0.0.1:1337").await?;
    Ok(())
}

async fn start_tracking(tx: Sender<Url>, mut req: Request<()>) -> tide::Result {
    let tracking: Tracking = req.body_form().await?;
    // Pass tracking url to the worker thread
    tx.send(tracking.url.clone()).unwrap();
    Ok(format!("Got it! Tracking food delivery from {}", tracking.url).into())
}

#[derive(Debug, Deserialize)]
struct Tracking {
    url: Url,
}
