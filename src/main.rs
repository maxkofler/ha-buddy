#![no_std]
#![no_main]
#![feature(exclusive_range_pattern)]
#![feature(abi_avr_interrupt)]

mod int;
mod network;
mod panic;

const BAUDRATE: u32 = 57600;

use arduino_hal::{
    delay_ms,
    hal::{
        port::Dynamic,
        usart::{BaudrateArduinoExt, Event, Usart0},
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

    // Enable interrupts
    unsafe {
        avr_device::interrupt::enable();
    }

    let mut l_status = pins.d13.into_output().downgrade();
    let mut l_ok = pins.d12.into_output().downgrade();
    let mut l_err = pins.d11.into_output().downgrade();

    write_init_data(&mut serial);

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
                    if uptime_ms() - last_reset > 1000 {
                        l_status.blink(20);

                        // Reset the current frame in flight
                        dl_layer.reset();
                        last_reset = uptime_ms();
                    }

                    continue 'recv_loop;
                }
            };

            //l_status.toggle();
            match dl_layer.handle_byte(byte) {
                Some(frame) => {
                    if let Some(frame) = frame.crc_guard() {
                        l_ok.blink(50);
                        delay_ms(50);
                    } else {
                        l_err.blink(500);
                        delay_ms(50);
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

fn write_init_data<T>(serial: &mut Usart0<T>) {
    for _ in 0..10 {
        serial.write_byte(0xff);
    }
}
