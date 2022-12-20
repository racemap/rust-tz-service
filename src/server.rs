use lazy_static::lazy_static;
use std::convert::Infallible;
use std::sync::Arc;
use tzf_rs::DefaultFinder;
use warp::{http::StatusCode, Filter, Rejection, Reply};

pub mod tz;

use crate::utils::{BoxUnitResult, UnitOk};

lazy_static! {
    pub static ref PORT: u16 = std::env::var("PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(3000);
}
#[derive(Clone)]
pub struct Context {
    tz_finder: Arc<DefaultFinder>,
}

fn with_ctx<C: Clone + Send>(ctx: C) -> impl Filter<Extract = (C,), Error = Infallible> + Clone {
    warp::any().map(move || ctx.clone())
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

async fn health_handler(_ctx: Context) -> Result<impl Reply, Rejection> {
    Ok(format!("OK"))
}

pub async fn run_server() -> BoxUnitResult {
    let logger = warp::log("rust_tz_service::server");
    let ctx = with_ctx(Context {
        tz_finder: Arc::new(DefaultFinder::new()),
    });

    let health_route = warp::path!("health")
        .and(warp::get())
        .and(ctx.clone())
        .and_then(health_handler);

    let tz_route = {
        use tz::tz_handler;

        warp::path!("api" / "tz")
            .and(warp::get())
            .and(warp::filters::query::query())
            .and(ctx.clone())
            .and_then(tz_handler)
    };

    let routes = health_route
        .or(tz_route)
        .with(logger)
        .recover(handle_rejection);

    log::info!("Starting server on port {}", *PORT);
    warp::serve(routes).run(([0, 0, 0, 0], *PORT)).await;
    UnitOk
}
