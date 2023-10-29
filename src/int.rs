use avr_device::atmega328p::USART0;

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

pub struct UART2 {}
#[allow(dead_code)]
impl UART2 {
    pub fn available() -> u8 {
        unsafe { USART_2_BUFFER.available() }
    }

    pub fn pop() -> Option<u8> {
        unsafe { USART_2_BUFFER.pop() }
    }
}

pub static mut USART_2_BUFFER: UARTBuffer = UARTBuffer {
    buffer: [0; u8::MAX as usize + 1],
    pos_in: 0,
    pos_out: 0,
};

impl UARTBuffer {}

#[avr_device::interrupt(atmega328p)]
#[allow(non_snake_case)]
fn USART_RX() {
    let udr = unsafe { &(*USART0::ptr()).udr0 };
    let byte: u8 = udr.read().bits();

    unsafe {
        USART_2_BUFFER.push(byte);
    }
}
