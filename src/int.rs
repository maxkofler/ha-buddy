use avr_device::atmega2560::USART0;

use crate::{counter, MsgFrame};

static mut cur_frame: MsgFrame = MsgFrame {
    command: 0,
    len: 0,
    payload: [0; u8::MAX as usize + 1],
    crc: 0,
    in_len: 0,
};

static mut ms_running: u64 = 0;
pub fn uptime_ms() -> u64 {
    unsafe { ms_running }
}

pub struct UARTBuffer {
    buffer: [u8; u8::MAX as usize + 1],
    pos_in: u8,
    pos_out: u8,
}

impl UARTBuffer {
    pub fn available(&self) -> u8 {
        return self.pos_out.overflowing_sub(self.pos_in).0;
    }

    pub fn push(&mut self, byte: u8) -> bool {
        if self.pos_in.overflowing_add(1).0 == self.pos_out {
            false
        } else {
            self.buffer[self.pos_in as usize] = byte;
            self.pos_in = self.pos_in.wrapping_add(1);
            true
        }
    }

    pub fn pop(&mut self) -> Option<u8> {
        if self.pos_in == self.pos_out {
            return None;
        }

        let b = self.buffer[self.pos_out as usize];
        self.pos_out = self.pos_out.overflowing_add(1).0;
        Some(b)
    }
}

pub struct UART0 {}
impl UART0 {
    pub fn available() -> u8 {
        unsafe { usart_0_buffer.available() }
    }

    pub fn pop() -> Option<u8> {
        unsafe { usart_0_buffer.pop() }
    }
}

pub static mut usart_0_buffer: UARTBuffer = UARTBuffer {
    buffer: [0; u8::MAX as usize + 1],
    pos_in: 0,
    pos_out: 0,
};

impl UARTBuffer {}

#[avr_device::interrupt(atmega2560)]
#[allow(non_snake_case)]
fn USART0_RX() {
    unsafe { counter += 1 };

    let udr = unsafe { &(*USART0::ptr()).udr0 };
    let byte: u8 = udr.read().bits();

    unsafe {
        usart_0_buffer.push(byte);
    }
}

#[avr_device::interrupt(atmega2560)]
fn TIMER1_COMPA() {
    unsafe { ms_running += 1 };
}
