#[macro_use]
extern crate rocket;

use crate::api::send_pos;
use crate::backend::Backend;
use rocket::fs::FileServer;
use std::path::Path;

mod api;
mod backend;
mod utils;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let backend = Backend::new("/dev/ttyACM0", "/dev/ttyACM1", 9600).unwrap();
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
