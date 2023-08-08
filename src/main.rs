#![no_std]
#![no_main]
#![feature(exclusive_range_pattern)]
#![feature(abi_avr_interrupt)]

mod int;
mod panic;

const BAUDRATE: u32 = 57600;

use crc::{Crc, CRC_32_BZIP2};
const CRC_ALGORITHM: Crc<u32> = Crc::<u32>::new(&CRC_32_BZIP2);

static mut counter: u32 = 0;

use arduino_hal::{
    delay_ms,
    hal::{
        port::Dynamic,
        usart::{BaudrateArduinoExt, Event, Usart0},
    },
    port::{mode::Output, Pin},
};

use int::{uptime_ms, UART0};

struct MsgFrame {
    command: u8,
    len: u8,
    payload: [u8; u8::MAX as usize + 1],
    crc: u32,
    in_len: u16,
}

impl Default for MsgFrame {
    fn default() -> Self {
        Self {
            command: 0,
            len: 0,
            payload: [0; u8::MAX as usize + 1],
            crc: 0,
            in_len: 0,
        }
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::Usart::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(),
        BAUDRATE.into_baudrate(),
    );
    serial.listen(Event::RxComplete);
    serial.flush();

    //
    // Setup timer
    //

    let tmr1 = dp.TC1;
    tmr1.tccr1a.write(|w| w.wgm1().bits(0b00));
    tmr1.tccr1b.write(|w| w.cs1().direct().wgm1().bits(0b01));
    tmr1.ocr1a.write(|w| w.bits(15624));

    // Enable the timer interrupt
    tmr1.timsk1.write(|w| w.ocie1a().set_bit());

    //
    //
    //

    unsafe {
        avr_device::interrupt::enable();
    }

    let mut l_status = pins.d13.into_output().downgrade();
    let mut l_ok = pins.d12.into_output().downgrade();
    let mut l_err = pins.d11.into_output().downgrade();

    write_init_data(&mut serial);

    loop {
        let mut last_reset: u64 = 0;
        let mut cur_frame = MsgFrame::default();

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

                        cur_frame = MsgFrame::default();
                        last_reset = uptime_ms();
                    }

                    continue 'recv_loop;
                }
            };

            match cur_frame.in_len {
                0 => {
                    cur_frame.command = byte;
                    cur_frame.in_len += 1;
                }
                1 => {
                    cur_frame.len = byte;
                    for _ in 0..cur_frame.len {
                        l_ok.blink(50);
                        delay_ms(50);
                    }
                    cur_frame.in_len += 1;
                }
                _ => {
                    let payload_len = 1 + cur_frame.len as u16;
                    let crc_len = payload_len + 4;
                    if cur_frame.in_len <= payload_len {
                        //Payload
                    } else if cur_frame.in_len <= crc_len {
                        //CRC
                    }

                    if cur_frame.in_len == crc_len {
                        match cur_frame.command {
                            0xfe => {
                                serial.write_byte(0xfe);
                            }
                            0xfd => {
                                l_ok.blink(1000);
                            }
                            _ => {}
                        }

                        l_err.set_low();
                        l_status.set_low();

                        cur_frame = MsgFrame::default();
                    } else {
                        cur_frame.in_len += 1;
                    }
                }
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
