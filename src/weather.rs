pub struct Forecast {
    pub foo: String,
}

pub fn get_forecast() -> Forecast {
    let api_url =
        "https://api.met.no/weatherapi/locationforecast/2.0/compact.json?lat=63.2549&lon=10.2342";

    Forecast {
        foo: "nope".to_owned(),
    }
}
