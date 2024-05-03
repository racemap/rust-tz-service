use crate::server::{Arc, Context};
use chrono::{Offset, TimeZone, Utc};
use chrono_tz::{OffsetName, Tz};
use serde::{Deserialize, Serialize};
use tzf_rs::DefaultFinder;
use warp::{reject::Reject, Rejection, Reply};


#[derive(Deserialize, Copy, Clone)]
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

pub async fn tz_handler(query: TimezoneQuery, ctx: Context) -> Result<impl Reply, Rejection> {
    validated_query(query)?;
    
    let timezone_name = get_timezone_for_location(ctx.tz_finder, query.lat, query.lng);
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

fn get_timezone_for_location(finder: Arc<DefaultFinder>, lat: f64, lng: f64) -> String {
    String::from(finder.get_tz_name(lng, lat))
}

pub fn validated_query(query: TimezoneQuery) -> Result<TimezoneQuery, Rejection> {
    if query.lat >= -90.0 && query.lat <= 90.0 && query.lng >= -180.0 && query.lng <= 180.0 {
        Ok(query)
    } else {
        Err(warp::reject::custom(InvalidLatLon { lat: query.lat.to_string(), lng: query.lng.to_string() }))
    }
}

#[derive(Debug)]
pub struct InvalidLatLon {
    pub lat: String,
    pub lng: String,
}
impl Reject for InvalidLatLon {}