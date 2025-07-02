use crate::backend::Backend;
use rocket::data::FromData;
use rocket::futures::SinkExt;
use rocket::http::RawStr;
use rocket::serde::{json::Json, Deserialize};
use rocket::State;

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
    if move_motor.motor == "X" {
        backend.rotate_x(angle);
    }
    if move_motor.motor == "Y" {
        backend.rotate_y(angle);
    }
}
