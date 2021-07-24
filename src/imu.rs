use std::time::Duration;

use crate::constants::BUF_SIZE;

#[derive(Debug)]
#[repr(C)]
pub struct Imu {
    pub acc: [f32; 3],
    pub gyr: [f32; 3],
    pub mag: [f32; 3],
}

impl std::fmt::Display for Imu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.acc[0], self.acc[1], self.acc[2])
    }
}

pub struct ImuReader {
    port: Box<dyn serialport::SerialPort>,
    buf: [u8; BUF_SIZE],
}

impl ImuReader {
    pub fn new() -> Self {
        let ports = serialport::available_ports().expect("No ports found!");
        let first_port = ports.first().expect("Is your Arduino plugged in?");
        let port = serialport::new(&first_port.port_name, 115_200)
            .timeout(Duration::from_millis(100))
            .open()
            .expect("Failed to open port");
        let buf = [0; BUF_SIZE];
        ImuReader { port, buf }
    }

    pub fn read(&mut self) -> Imu {
        self.port.read(&mut self.buf).expect("Found no data!");
        let imu = unsafe { std::mem::transmute::<[u8; BUF_SIZE], Imu>(self.buf) };
        imu
    }
}
