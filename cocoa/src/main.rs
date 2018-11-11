extern crate rppal;

use rppal::gpio::{Gpio, Mode, Level};
use rppal::system::DeviceInfo;

use std::thread;
use std::time::Duration;


struct MotorController {
    pin_a: u8,
    pin_b: u8,
    pin_e: u8,
    gpio: Gpio
}
impl MotorController {
    fn new(mut gpio: Gpio, pin_a: u8, pin_b: u8, pin_e: u8) -> MotorController {
        gpio.set_mode(pin_a, Mode::Output);
        gpio.set_mode(pin_a, Mode::Output);
        gpio.set_mode(pin_a, Mode::Output);

        MotorController {
            pin_a, pin_b, pin_e, gpio
        }
    }

    fn forward(&self) {
        self.gpio.write(self.pin_a, Level::High);
        self.gpio.write(self.pin_b, Level::Low);
        self.gpio.write(self.pin_e, Level::High);
    }

    fn backward(&self) {
        self.gpio.write(self.pin_a, Level::Low);
        self.gpio.write(self.pin_b, Level::High);
        self.gpio.write(self.pin_e, Level::High);
    }

    fn stop(&self) {
        self.gpio.write(self.pin_e, Level::Low);
    }
}

fn main() {
    let device_info = DeviceInfo::new().unwrap();
    println!("Model: {} (SoC: {})", device_info.model(), device_info.soc());

    let gpio = match Gpio::new() {
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

    let motor = MotorController::new(gpio, 23, 24, 25);

    println!("Forward...");
    motor.forward();
    thread::sleep(Duration::from_millis(2000));
    println!("Stop...");
    motor.stop();
}
