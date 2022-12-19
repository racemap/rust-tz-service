mod server;
mod utils;

use futures::try_join;
use server::run_server;
use utils::{BoxUnitResult, UnitOk};

extern crate pretty_env_logger;

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
