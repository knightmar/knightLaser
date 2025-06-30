use serialport::SerialPort;

pub struct Motor {
    serial: Box<dyn SerialPort>,
}

impl Motor {
    pub fn new<'a>(serial: &str, rate: u32) -> Result<Self, &'a str> {
        if let Ok(port) = serialport::new(serial, rate).open() {
            Ok(Motor { serial: port })
        } else {
            Err("Motor port not found")
        }
    }
    pub fn rotate(&mut self, angle: i32) {
        self.serial
            .write_all(angle.to_string().as_bytes())
            .expect("Failed to write to serial port");
    }
}

impl Clone for Motor {
    fn clone(&self) -> Self {
        Motor {
            serial: self.serial.try_clone().expect("Failed to clone SerialPort"),
        }
    }
}
