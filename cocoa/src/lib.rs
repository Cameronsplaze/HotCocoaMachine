#![feature(duration_as_u128)]

use rppal::gpio::{Gpio, Mode, Level};

use std::thread;
use std::time::{Duration, Instant};


type Pin = u8;

pub struct MotorController<'a> {
    a: Pin,
    b: Pin,
    enable: Pin,
    gpio: &'a Gpio
}
impl<'a> MotorController<'a> {
    pub fn new(gpio: &'a Gpio, a: Pin, b: Pin, enable: Pin) -> MotorController {
        MotorController {
            a, b, enable, gpio
        }
    }

    pub fn forward(&self) {
        self.gpio.write(self.a, Level::High);
        self.gpio.write(self.b, Level::Low);
        self.gpio.write(self.enable, Level::High);
    }

    pub fn backward(&self) {
        self.gpio.write(self.a, Level::Low);
        self.gpio.write(self.b, Level::High);
        self.gpio.write(self.enable, Level::High);
    }

    pub fn stop(&self) {
        self.gpio.write(self.enable, Level::Low);
    }

    pub fn forward_time(&self, duration: Duration) {
        self.forward();
        thread::sleep(duration);
        self.stop();
    }

    pub fn backward_time(&self, duration: Duration) {
        self.backward();
        thread::sleep(duration);
        self.stop();
    }
}
impl<'a> Drop for MotorController<'a> {
    fn drop(&mut self) {
        self.stop();
    }
}

pub struct UltrasonicSensor<'a> {
    trigger: Pin,
    echo: Pin,
    gpio: &'a Gpio,
    is_initialized: bool,
}
impl<'a> UltrasonicSensor<'a> {
    pub fn new(gpio: &'a Gpio, trigger: Pin, echo: Pin) -> UltrasonicSensor {
        UltrasonicSensor { trigger, echo, gpio, is_initialized: false }
    }

    /// Sets the trigger to 0 and blocks for 2 seconds to let it settle
    pub fn init(&mut self) {
        if self.is_initialized { return; }

        self.gpio.write(self.trigger, Level::Low);
        thread::sleep(Duration::from_millis(2000));

        self.is_initialized = true;
    }

    pub fn read_distance(&self) -> f64 {
        debug_assert!(self.is_initialized, true);
        const CM_PER_SEC: f64 = 17150.0;
        const CM_PER_USEC: f64 = CM_PER_SEC / 1_000_000.0;

        // Send the trigger pulse
        self.gpio.write(self.trigger, Level::High);
        thread::sleep(Duration::from_micros(10));
        self.gpio.write(self.trigger, Level::Low);

        let pulse_start = loop {
            if self.gpio.read(self.echo).expect("Read error") == Level::Low { continue; }
            break Instant::now();
        };

        let pulse_end = loop {
            if self.gpio.read(self.echo).expect("Read error") == Level::High { continue; }
            break Instant::now();
        };

        let pulse_duration = pulse_end - pulse_start;
        let cm = pulse_duration.as_micros() as f64 * CM_PER_USEC;
        cm
    }
}
