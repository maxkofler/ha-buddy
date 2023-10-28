#![no_std]
#![no_main]
#![feature(exclusive_range_pattern)]
#![feature(abi_avr_interrupt)]

mod crc;
mod datalink;
mod handler;
mod homeassistant;
mod int;
mod panic;

use arduino_hal::{
    delay_ms,
    hal::{
        port::Dynamic,
        usart::{BaudrateArduinoExt, Event},
    },
    port::{mode::Output, Pin},
};

use datalink::DataFrame;
use handler::handle_frame;
use homeassistant::{sensor::SensorRef, switch::SwitchRef};
use int::*;

const BAUDRATE: u32 = 57600;
const MY_ADDR: u16 = 0x1000;

/// A static reference to the current frame, to not store it on the stack
static mut FRAME: DataFrame = DataFrame {
    src: 0,
    dst: 0,
    cmd: 0,
    payload_len: 0,
    h_crc: 0,
    payload: [0; 256],
    f_crc: 0,
    in_len: 0,
};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let sensors: [&dyn SensorRef; 0] = [];
    let mut switches: [&mut dyn SwitchRef; 0] = [];

    let mut serial = arduino_hal::Usart::new(
        dp.USART2,
        pins.d17,
        pins.d16.into_output(),
        BAUDRATE.into_baudrate(),
    );
    serial.listen(Event::RxComplete);
    serial.flush();

    let mut handler_pins = handler::HandlerPins {};

    let mut led_status = pins.d13.into_output().downgrade();
    let mut p_de = pins.d2.into_output().downgrade();
    let mut p_re = pins.d3.into_output().downgrade();

    p_re.set_low();
    p_de.set_low();

    // Enable interrupts
    unsafe {
        avr_device::interrupt::enable();
    }

    loop {
        'recv_loop: loop {
            avr_device::asm::sleep();

            let byte = match UART2::pop() {
                Some(b) => b,
                None => continue 'recv_loop,
            };

            if unsafe { FRAME.handle_byte(byte) } {
                if unsafe { FRAME.check_crc() } {
                    if unsafe { FRAME.dst } == MY_ADDR {
                        led_status.set_high();
                        if handle_frame(
                            unsafe { &mut FRAME },
                            &mut handler_pins,
                            &sensors,
                            &mut switches,
                        ) {
                            // Set addresses
                            unsafe { FRAME.src = MY_ADDR };
                            unsafe { FRAME.dst = 0 };
                            unsafe { FRAME.cmd += 1 };

                            // Enable RS485 driver
                            p_de.set_high();

                            unsafe { FRAME.send(&mut serial).unwrap() };

                            // Flush contents, wait for data send and disable RS485 driver
                            serial.flush();
                            delay_ms(1);
                            p_de.set_low();
                        }
                    }
                }
            }
            led_status.set_low();
        }
    }
}

trait Blink {
    fn blink(&mut self, ms: u16);
}

impl Blink for Pin<Output, Dynamic> {
    fn blink(&mut self, ms: u16) {
        self.set_high();
        delay_ms(ms);
        self.set_low();
    }
}
