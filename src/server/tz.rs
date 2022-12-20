use chrono::{Offset, TimeZone, Utc};
use chrono_tz::{OffsetName, Tz};
use serde::{Deserialize, Serialize};
use tzf_rs::DefaultFinder;
use warp::{Rejection, Reply};

#[derive(Deserialize)]
pub struct TimezoneQuery {
    lng: f64,
    lat: f64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimezoneReturnBody {
    name: String,
    offset: i32,
    id: String,
}

pub async fn tz_handler(query: TimezoneQuery) -> Result<impl Reply, Rejection> {
    let timezone_name = get_timezone_for_location(query.lat, query.lng);
    let timezone: Tz = timezone_name.parse().unwrap();
    let now = Utc::now();
    let offset = timezone.offset_from_utc_datetime(&now.naive_utc());

    let output = TimezoneReturnBody {
        name: String::from(timezone.name()),
        offset: offset.fix().local_minus_utc(),
        id: String::from(offset.abbreviation()),
    };
    Ok(warp::reply::json(&output))
}

fn get_timezone_for_location(lat: f64, lng: f64) -> String {
    let finder = DefaultFinder::new();
    String::from(finder.get_tz_name(lng, lat))
}
