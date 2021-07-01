use rust_gpiozero::LED;

/// **LED BLINK GPIO 17**
/// Lets the LED Flash on GPIO 17 using the rust_gpiozero ports
pub fn led_blink(duration: f32) {
    let mut led = LED::new(17);
    led.blink(duration,1.0);
    led.wait();
}
