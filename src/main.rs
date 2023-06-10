use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

const GPIO_LED: u8 = 23;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();
    pin.set_low();

    // Blink the LED by setting the pin's logic level high for 500 ms.
    loop {
        pin.set_high();
        println!("set pin high");
        thread::sleep(Duration::from_millis(500));
        println!("set pin low");
        pin.set_low();
        thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}
