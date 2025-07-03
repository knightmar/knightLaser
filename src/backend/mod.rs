pub mod laser;
pub mod motors;

use crate::backend::laser::Laser;
use crate::backend::motors::Motor;
use crate::utils::Error;
use crate::utils::Error::MotorError;

#[derive(Clone)]
pub struct Backend {
    x_motor: Option<Motor>,
    y_motor: Option<Motor>,
    laser: Laser,
    error: Option<Error>,
}

impl Backend {
    pub fn new(port1: &str, port2: &str, rate: u32) -> Result<Backend, Error> {
        let mut x_motor = None;
        let mut y_motor = None;
        if port1 != "" {
            x_motor = Some(Motor::new(port1, rate)?);
        }
        if port2 != "" {
            y_motor = Some(Motor::new(port2, rate)?);
        }
        let laser = Laser {};

        Ok(Backend {
            x_motor: x_motor.clone(),
            y_motor: y_motor.clone(),
            laser,
            error: if x_motor.is_none() || y_motor.is_none() {
                Some(MotorError("A motor is missing".to_string()))
            } else {
                None
            },
        })
    }

    pub fn rotate_x(&self, angle: i32) -> Result<(), String> {
        if self.x_motor.is_none() {
            return Err("x_motor is None".to_string());
        }
        self.x_motor.clone().unwrap().rotate(angle);
        Ok(())
    }

    pub fn rotate_y(&self, angle: i32) -> Result<(), String> {
        if self.y_motor.is_none() {
            return Err("y_motor is None".to_string());
        }
        self.y_motor.clone().unwrap().rotate(angle);
        Ok(())
    }

    pub async fn start_backend<'a>(&self) -> Result<(), &'a str> {
        println!("Starting backend");

        loop {
            // self.rotate_x(-100);
            // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            // self.rotate_x(100);
            // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }

    pub fn get_int(&self) -> i32 {
        42
    }

    pub fn x_motor(&self) -> &Option<Motor> {
        &self.x_motor
    }

    pub fn y_motor(&self) -> &Option<Motor> {
        &self.y_motor
    }

    pub fn laser(&self) -> &Laser {
        &self.laser
    }

    pub fn error(&self) -> &Option<Error> {
        &self.error
    }
}
