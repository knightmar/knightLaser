#[macro_use]
extern crate rocket;

use crate::backend::Backend;
use rocket::fs::FileServer;
use std::path::Path;
use crate::api::send_pos;

mod api;
mod backend;
mod utils;
mod website;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let backend = Backend::new("", "", 9600).unwrap();
    let x = backend.clone();
    tokio::spawn(async move {
        x.start_backend().await;
    });
    let _rocket = rocket::build()
        .manage(backend)
        .mount("/", FileServer::from(Path::new("./content")))
        .mount("/send_pos", routes![send_pos])
        .launch()
        .await?;

    Ok(())
}
