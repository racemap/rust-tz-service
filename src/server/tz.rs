use serde::Deserialize;
use tzf_rs::DefaultFinder;
use warp::{Rejection, Reply};

#[derive(Deserialize)]
pub struct TimezoneQuery {
    lng: f64,
    lat: f64,
}

pub async fn tz_handler(query: TimezoneQuery) -> Result<impl Reply, Rejection> {
    let timezone = get_timezone_for_location(query.lat, query.lng);

    Ok(format!(
        "The time zone for the given location is: {}",
        timezone
    ))
}

fn get_timezone_for_location(lat: f64, lng: f64) -> String {
    let finder = DefaultFinder::new();
    String::from(finder.get_tz_name(lng, lat))
}
