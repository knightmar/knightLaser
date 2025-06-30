#[macro_use]
extern crate rocket;

use crate::website::index;

mod website;
mod backend;



#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/test", routes![index])
        .launch()
        .await?;

    Ok(())
}