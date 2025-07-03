use crate::utils::Error;
use crate::utils::Error::MotorError;
use serialport::SerialPort;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Motor {
    serial: Arc<Mutex<Box<dyn SerialPort>>>,
}

impl Motor {
    pub fn new<'a>(serial: &str, rate: u32) -> Result<Self, Error> {
        if let Ok(port) = serialport::new(serial, rate).open() {
            Ok(Motor {
                serial: Arc::new(Mutex::new(port)),
            })
        } else {
            Err(MotorError("Motor port not found".to_string()))
        }
    }
    pub fn rotate(&self, angle: i32) {
        let mut serial = self.serial.lock().unwrap();
        println!("{}", serial.name().unwrap());
        serial
            .write_all(angle.to_string().as_bytes())
            .expect("Failed to write to serial port");
    }
}

impl Drop for Motor {
    fn drop(&mut self) {
        self.serial.lock().unwrap().clear_break().expect("Failed to clear while dropping motor");
    }
}
