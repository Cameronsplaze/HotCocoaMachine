
extern crate rppal;

use rppal::gpio::{Gpio, Mode};
use rppal::system::DeviceInfo;

use std::thread;
use std::time::Duration;

use cocoa::{MotorController, UltrasonicSensor};
use std::process;

fn main() {
    let device_info = DeviceInfo::new().unwrap();
    println!("Model: {} (SoC: {})", device_info.model(), device_info.soc());

    let mut gpio = make_gpio();

    let (motor_a, motor_b, motor_e) = (23, 24, 25);
    gpio.set_mode(motor_a, Mode::Output);
    gpio.set_mode(motor_b, Mode::Output);
    gpio.set_mode(motor_e, Mode::Output);
    let (sonic_t, sonic_e) = (17, 21);
    gpio.set_mode(sonic_t, Mode::Output);
    gpio.set_mode(sonic_e, Mode::Input);

    let motor = MotorController::new(&gpio, motor_a, motor_b, motor_e);
    let mut ultrasound = UltrasonicSensor::new(&gpio, sonic_t, sonic_e);
    println!("Initializing sensor...");
    ultrasound.init();

    const CUP_DISTANCE_THRESHOLD: f64 = 10.0;
    const TIME_TO_BREW: Duration = Duration::from_secs(300);

    println!("Ready!");
    loop {
        let dist = ultrasound.read_distance();

        // Wait for cup to be detected
        if dist > CUP_DISTANCE_THRESHOLD { continue; }
        println!("Cup detected!");

        // Activate the coffee maker
        println!("Brewing that good good");
        motor.forward_time(Duration::from_millis(2000));

        thread::sleep(TIME_TO_BREW);
        // Turn off the coffee maker
        motor.backward_time(Duration::from_millis(2000));

        // Wait for the cup to be removed
        loop {
            let dist = ultrasound.read_distance();
            if dist < CUP_DISTANCE_THRESHOLD { break; }

            thread::sleep(Duration::from_millis(10));
        }

        println!("Waiting for cup...");
    }
}

fn make_gpio() -> Gpio {
    match Gpio::new() {
        Ok(g) => g,
        Err(rppal::gpio::Error::PermissionDenied) => permission_error(),
        Err(rppal::gpio::Error::Io(ref e))
            if e.kind() == std::io::ErrorKind::PermissionDenied => permission_error(),
        e => e.unwrap()
    }
}

fn permission_error() -> ! {
    println!("Permission denied! Run as root");
    process::exit(-1)
}
