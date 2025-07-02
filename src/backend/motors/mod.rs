use serialport::SerialPort;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Motor {
    serial: Arc<Mutex<Box<dyn SerialPort>>>,
}

impl Motor {
    pub fn new<'a>(serial: &str, rate: u32) -> Result<Self, &'a str> {
        if let Ok(port) = serialport::new(serial, rate).open() {
            Ok(Motor {
                serial: Arc::new(Mutex::new(port)),
            })
        } else {
            Err("Motor port not found")
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
        self.serial.lock().unwrap().clear_break();
    }
}
