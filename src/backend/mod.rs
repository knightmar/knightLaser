pub mod laser;
pub mod motors;

use crate::backend::laser::Laser;
use crate::backend::motors::Motor;

async fn start<'a>() -> Result<(), &'a str> {
    println!("Starting backend");

    loop {
        println!("Backend is running");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

#[derive(Clone)]
pub struct Backend {
    pub(crate) x_motor: Motor,
    pub(crate) y_motor: Motor,
    pub(crate) laser: Laser,
}

impl Backend {
    pub fn new(port1: &str, port2: &str, rate: u32) -> Result<Backend, Box<dyn std::error::Error>> {
        let x_motor = Motor::new(port1, rate)?;
        let y_motor = Motor::new(port2, rate)?;
        let laser = Laser {};

        Ok(Backend {
            x_motor,
            y_motor,
            laser,
        })
    }

    pub fn rotate_x(&mut self, angle: i32) {
        println!("Rotate x by {}", angle);
        self.x_motor.rotate(angle);
    }

    pub fn rotate_y(&mut self, angle: i32) {
        println!("Rotate y by {}", angle);
        self.y_motor.rotate(angle);
    }

    pub async fn start_backend<'a>(&self) -> Result<(), &'a str> {
        start().await
    }

    pub fn get_int(&self) -> i32 {
        42
    }
}
