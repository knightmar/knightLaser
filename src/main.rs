#[macro_use]
extern crate rocket;

use crate::backend::Backend;
use rocket::fs::FileServer;
use std::path::Path;
use crate::api::get_int;

mod api;
mod backend;
mod utils;
mod website;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let backend = Backend::new("/dev/tty", "/dev/tty0", 9600).unwrap();
    let x = backend.clone();
    tokio::spawn(async move {
        x.start_backend().await;
    });
    let _rocket = rocket::build()
        .manage(backend)
        .mount("/", FileServer::from(Path::new("./content")))
        .mount("/int", routes![get_int])
        .launch()
        .await?;

    Ok(())
}
