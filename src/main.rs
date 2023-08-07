#![no_std]
#![no_main]

mod panic;

use arduino_hal::{delay_ms, hal::usart::Usart0, prelude::*};

use embedded_hal::serial::Read;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::Usart::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(),
        57600.into_baudrate(),
    );
    serial.flush();

    let mut led = pins.d13.into_output().downgrade();
    led.set_high();

    for _ in 0..10 {
        serial.write_byte(0xff);
    }

    let mut reset_counter = 0;

    led.set_low();

    loop {
        // Read a byte from the serial connection
        //let b = nb::block!(serial.read()).void_unwrap();
        let b = serial.read_byte();

        led.set_high();
        // If the byte is 0x0 10 times, reset
        if b == 0xff {
            reset_counter += 1;
            if reset_counter >= 10 {
                // Read all remaining bytes to clear the buffer
                while serial.read().is_ok() {}

                for _ in 0..5 {
                    let del = 40;
                    led.set_high();
                    delay_ms(del);
                    led.set_low();
                    delay_ms(del);
                }

                for _ in 0..10 {
                    serial.write_byte(0xff);
                }
            }
        } else {
            reset_counter = 0;

            match command_from_u8(b) {
                Some(cmd) => command(&mut serial, cmd),
                None => {
                    for _ in 0..5 {
                        let del = 100;
                        led.set_high();
                        delay_ms(del);
                        led.set_low();
                        delay_ms(del);
                    }
                }
            }
        }

        led.set_low();
    }
}

#[repr(u8)]
enum Command {
    Echo = 0xfe,
}

fn command_from_u8(u: u8) -> Option<Command> {
    match u {
        0xfe => Some(Command::Echo),
        _ => None,
    }
}

fn command<T>(serial: &mut Usart0<T>, command: Command) {
    match command {
        Command::Echo => serial.write_byte(0xfe),
    }
}
