
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

    let (motor_a, motor_b, motor_e) = (23, 24, 18);
    gpio.set_mode(motor_a, Mode::Output);
    gpio.set_mode(motor_b, Mode::Output);
    gpio.set_mode(motor_e, Mode::Output);
    let (sonic_t, sonic_e) = (20, 21);
    gpio.set_mode(sonic_t, Mode::Output);
    gpio.set_mode(sonic_e, Mode::Input);

    let motor = MotorController::new(&gpio, motor_a, motor_b, motor_e);
    let mut ultrasound = UltrasonicSensor::new(&gpio, sonic_t, sonic_e);
    println!("Initializing sensor...");
    ultrasound.init();

    const CUP_DISTANCE_THRESHOLD: f64 = 10.0;
    const CUP_VERIFICATION_CHECKS: usize = 50;
    const TIME_TO_BREW: Duration = Duration::from_secs(120);
    const CYCLE_RESET_TIME: Duration = Duration::from_secs(20);

    println!("Ready!");
    'main: loop {
        let dist = ultrasound.read_distance();

        // Wait for cup to be detected
        if dist > CUP_DISTANCE_THRESHOLD {
            thread::sleep(Duration::from_millis(10));
            continue;
        }

        // Verify that the cup is really there
        for _ in 0..CUP_VERIFICATION_CHECKS {
            let dist = ultrasound.read_distance();
            if dist > CUP_DISTANCE_THRESHOLD {
                thread::sleep(Duration::from_millis(5));
                continue 'main;
            }
        }
        println!("Cup detected!");

        // Activate the coffee maker
        println!("Brewing that good good");
        motor.backward_time(Duration::from_secs(2));

        thread::sleep(TIME_TO_BREW);
        println!("That good good is done");

        // Turn off the coffee maker
        println!("Turning off the coffee maker");
        motor.forward_time(Duration::from_secs(2));

        // Wait for the cup to be removed
        println!("Waiting for cup to be removed");
        'rem: loop {
            let dist = ultrasound.read_distance();
            if dist < CUP_DISTANCE_THRESHOLD {
                thread::sleep(Duration::from_millis(10));
                continue;
            }

            // Verify that the cup is really not there anymore
            for _ in 0..CUP_VERIFICATION_CHECKS {
                let dist = ultrasound.read_distance();
                if dist < CUP_DISTANCE_THRESHOLD {
                    thread::sleep(Duration::from_millis(5));
                    continue 'rem;
                }
            }
            break;
        }

        thread::sleep(CYCLE_RESET_TIME);
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
