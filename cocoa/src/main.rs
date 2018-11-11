
extern crate rppal;

use rppal::gpio::{Gpio, Mode, Level};
use rppal::system::DeviceInfo;

use std::thread;
use std::time::Duration;

use cocoa::{MotorController, UltrasonicSensor};

fn main() {
    let device_info = DeviceInfo::new().unwrap();
    println!("Model: {} (SoC: {})", device_info.model(), device_info.soc());

    let mut gpio = match Gpio::new() {
        Ok(g) => g,
        Err(rppal::gpio::Error::PermissionDenied) => {
            println!("Permission denied! Run as root");
            return;
        },
        Err(rppal::gpio::Error::Io(ref e)) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            println!("Permission denied! Run as root");
            return;
        },
        e => e.unwrap()
    };

    // let motor = MotorController::new(&mut gpio, 23, 24, 25);

    // println!("Forward...");
    // motor.forward();
    // thread::sleep(Duration::from_millis(2000));
    // println!("Stop...");
    // motor.stop();

    let mut ultrasound = UltrasonicSensor::new(&mut gpio, 17, 21);
    println!("Initializing sensor...");
    ultrasound.init();

    println!("Reading distances...");
    for _ in 0..50 {
        ultrasound.read_distance();
        thread::sleep(Duration::from_secs(1));
    }
}
