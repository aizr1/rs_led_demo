use rust_gpiozero::*;
use std::thread;
use std::time::Duration;

fn main() {
    // Create a new LED attached to Pin 17
    let mut led = LED::new(17);
    led.blink(0.01,1.0);
    led.wait();
}
