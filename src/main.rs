use rppal::gpio::Gpio;
use std::error::Error;
use std::fs;

const GPIO_FAN: u8 = 14;
const CORE_TEMP_FILE: &str = "/sys/class/thermal/thermal_zone0/temp";
const OFF_THRESHOLD: f32 = 46.0;
const ON_THRESHOLD: f32 = 50.0;

fn main() -> Result<(), Box<dyn Error>> {

    // Get current Temperature in Celsius
    let temperature: f32 = fs::read_to_string(CORE_TEMP_FILE)?.trim().parse()?;
    let temperature = temperature / 1000.0;

    // Get Fan pin as output pin
    let mut pin = Gpio::new()?.get(GPIO_FAN)?.into_output();

    // Do not restore pin state after executing
    if pin.reset_on_drop() { pin.set_reset_on_drop(false) }

    // If current temperature is greater than ON_THRESHOLD and fan is turned off => Turn on fan
    // If current temperature is lesser than OFF_THRESHOLD and fan is turned on => Turn off fan
    match temperature {
        t if t >= ON_THRESHOLD && pin.is_set_low() => pin.set_high(),
        t if t <= OFF_THRESHOLD && pin.is_set_high() => pin.set_low(),
        _ => (),
    }

    Ok(())
}
