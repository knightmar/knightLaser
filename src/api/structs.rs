use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MoveMotor<'r> {
    motor: &'r str,
    angle: &'r str,
}

impl<'r> MoveMotor<'r> {
    pub fn motor(&self) -> &'r str {
        self.motor
    }

    pub fn angle(&self) -> &'r str {
        self.angle
    }
}

#[derive(serde::Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Error {
    pub message: String,
}
