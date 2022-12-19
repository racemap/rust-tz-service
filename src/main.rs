use lazy_static::lazy_static;
use std::convert::Infallible;

mod utils;
use futures::try_join;

use utils::{BoxUnitResult, UnitOk};
use warp::{http::StatusCode, Filter, Rejection, Reply};
extern crate pretty_env_logger;

lazy_static! {
    pub static ref PORT: u16 = std::env::var("PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(3000);
}

#[tokio::main]
async fn main() -> BoxUnitResult {
    pretty_env_logger::init();

    let selected_task = std::env::args().nth(1);

    let server_task = {
        match selected_task.as_ref().map(|s| s.as_str()) {
            None | Some("server") => tokio::spawn(async move {
                run_server().await?;
                UnitOk
            }),
            _ => tokio::spawn(async move { UnitOk }),
        }
    };

    let handles = try_join!(server_task)?;
    handles.0?;

    UnitOk
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message): (StatusCode, String) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, String::from("Not found"))
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        (
            StatusCode::METHOD_NOT_ALLOWED,
            String::from("Method not allowed"),
        )
    } else if let Some(err) = err.find::<warp::reject::InvalidQuery>() {
        log::error!("Invalid query: {:?}", err);
        (StatusCode::BAD_REQUEST, format!("Invalid query: {:?}", err))
    } else {
        log::error!("unhandled rejection: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Internal Server Error"),
        )
    };

    Ok(warp::reply::with_status(message, code))
}

async fn health_handler() -> Result<impl Reply, Rejection> {
    Ok(format!("OK"))
}

pub async fn run_server() -> BoxUnitResult {
    let logger = warp::log("rust_tz_service::server");

    let health_route = warp::path!("health")
        .and(warp::get())
        .and_then(health_handler);

    let routes = health_route.with(logger).recover(handle_rejection);

    log::info!("Starting server on port {}", *PORT);
    warp::serve(routes).run(([0, 0, 0, 0], *PORT)).await;
    UnitOk
}
