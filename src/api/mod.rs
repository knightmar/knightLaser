use crate::backend::Backend;
use rocket::serde::json::Json;
use rocket::State;

#[get("/")]
pub fn get_int(backend: &State<Backend>) -> Json<i32> {
    Json(backend.get_int())
}
