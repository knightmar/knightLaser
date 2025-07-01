use crate::backend::Backend;
use rocket::futures::SinkExt;
use rocket::http::RawStr;
use rocket::serde::json::Json;
use rocket::State;

#[post("/", data = "<angle>")]
pub fn send_pos(angle: String, backend: &State<Backend>) {
    println!("{:?}", angle);
    let angle = angle.parse::<i32>().unwrap();
    backend.x_motor.clone().unwrap().rotate(angle);
}
