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

use int::{uptime_ms, UART0};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    int::setup_timer(&dp);

    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::Usart::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(),
        BAUDRATE.into_baudrate(),
    );
    serial.listen(Event::RxComplete);
    serial.flush();

    let mut handler_pins = handler::HandlerPins {
        l_status: pins.d12.into_output().downgrade(),
    };

    // Enable interrupts
    unsafe {
        avr_device::interrupt::enable();
    }

    let mut dl_layer = network::DataLinkLayer::default();

    loop {
        let mut last_reset: u64 = 0;

        'recv_loop: loop {
            avr_device::asm::sleep();

            let byte = match UART0::pop() {
                Some(b) => {
                    last_reset = uptime_ms();
                    b
                }
                None => {
                    if (uptime_ms() - last_reset) > 1000 {
                        delay_ms(1);

                        last_reset = uptime_ms();
                        // Reset the current frame in flight
                        dl_layer.reset();
                    }

                    continue 'recv_loop;
                }
            };

            match dl_layer.handle_byte(byte) {
                Some(frame) => {
                    if let Some(frame) = frame.crc_guard() {
                        if let Some(frame) = frame.addr_guard(MY_ADDR) {
                            match handler::handle_frame(frame, &mut handler_pins) {
                                Some(mut frame) => {
                                    // Calculate the CRC and set the address
                                    frame.update_crc();
                                    frame.addr = MY_ADDR;

                                    // Write the address
                                    serial.write_byte((frame.addr & 0xff) as u8);
                                    serial.write_byte((frame.addr >> 8 & 0xff) as u8);

                                    // Write the payload
                                    serial.write_byte(frame.payload_len);
                                    for i in 0..frame.payload_len {
                                        serial.write_byte(frame.payload[i as usize]);
                                    }

                                    // Write the CRC
                                    serial.write_byte((frame.crc & 0xff) as u8);
                                    serial.write_byte((frame.crc >> 8 & 0xff) as u8);
                                    serial.write_byte((frame.crc >> 16 & 0xff) as u8);
                                    serial.write_byte((frame.crc >> 24 & 0xff) as u8);
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
