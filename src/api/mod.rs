mod structs;

use crate::api::structs::{Error, MoveMotor};
use crate::backend::Backend;
use crate::utils::get_error_string;
use rocket::serde::json::Json;
use rocket::State;

#[post("/", format = "application/json", data = "<move_motor>")]
pub fn send_pos(move_motor: Json<MoveMotor>, backend: &State<Backend>) {
    let angle = move_motor.angle().parse::<i32>().unwrap();
    println!("{:?} {}", move_motor.motor(), angle);
    if move_motor.motor() == "X" {
        backend.rotate_x(angle);
    }
    if move_motor.motor() == "Y" {
        backend.rotate_y(angle);
    }
}

#[get("/", format = "application/json")]

pub fn get_error_presence<'r>(backend: &State<Backend>) -> Json<Error> {
    Json(Error {
        message: if backend.error().is_some() {
            get_error_string(&backend.error().clone().unwrap())
        } else {
            "".to_string()
        },
    })
}
