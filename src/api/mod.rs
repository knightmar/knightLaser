use rocket::data::FromData;
use crate::backend::Backend;
use rocket::futures::SinkExt;
use rocket::http::RawStr;
use rocket::State;
use rocket::serde::{Deserialize, json::Json};



#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct MoveMotor<'r> {
    motor: &'r str,
    angle: &'r str,
}

#[post("/", format = "application/json", data = "<move_motor>")]
pub fn send_pos(move_motor: Json<MoveMotor>, backend: &State<Backend>) {
    let angle = move_motor.angle.parse::<i32>().unwrap();
    println!("{:?} {}", move_motor.motor, angle);
    println!("{:?}", backend.rotate_x(angle));
}
