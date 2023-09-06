#![no_std]
#![no_main]
#![feature(exclusive_range_pattern)]
#![feature(abi_avr_interrupt)]

mod handler;
mod int;
mod network;
mod panic;

const BAUDRATE: u32 = 57600;
const MY_ADDR: u16 = 0x1000;

use arduino_hal::{
    delay_ms,
    hal::{
        port::Dynamic,
        usart::{BaudrateArduinoExt, Event},
    },
    port::{mode::Output, Pin},
};

use int::UART2;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::Usart::new(
        dp.USART2,
        pins.d17,
        pins.d16.into_output(),
        BAUDRATE.into_baudrate(),
    );
    serial.listen(Event::RxComplete);
    serial.flush();

    let mut handler_pins = handler::HandlerPins {
        l_status: pins.d13.into_output().downgrade(),
    };

    let mut p_de = pins.d2.into_output().downgrade();
    let mut p_re = pins.d3.into_output().downgrade();

    p_re.set_low();
    p_de.set_low();

    // Enable interrupts
    unsafe {
        avr_device::interrupt::enable();
    }

    let mut dl_layer = network::DataLinkLayer::default();

    loop {
        'recv_loop: loop {
            avr_device::asm::sleep();

            let byte = match UART2::pop() {
                Some(b) => b,
                None => continue 'recv_loop,
            };

            match dl_layer.handle_byte(byte) {
                Some(frame) => {
                    if let Some(frame) = frame.crc_guard() {
                        if let Some(frame) = frame.addr_guard(MY_ADDR) {
                            match handler::handle_frame(frame, &mut handler_pins) {
                                Some(mut frame) => {
                                    // Set addresses
                                    frame.src = MY_ADDR;
                                    frame.dst = 0;

                                    // Enable RS485 driver
                                    p_de.set_high();

                                    frame.send(&mut serial).unwrap();

                                    // Flush contents, wait for data send and disable RS485 driver
                                    serial.flush();
                                    delay_ms(1);
                                    p_de.set_low();
                                }
                                None => {}
                            }
                        }
                    }
                }
                _ => {}
            }
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
