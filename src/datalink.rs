use embedded_hal::serial::Write;
use nb::block;

use crate::crc::{CRC8Autosar, CRC, CRC8_AUTOSAR_INIT};

const START_BYTE_0: u8 = 0xaa;
const START_BYTE_1: u8 = 0x55;

static mut CRC_CALCULATOR: CRC8Autosar = CRC8Autosar {
    crc: CRC8_AUTOSAR_INIT,
};

/// A frame from the Data Link Layer
pub struct DataFrame {
    /// The source address
    pub src: u16,
    /// The destination address
    pub dst: u16,
    /// The message type / command
    pub cmd: u16,
    /// The amount of bytes in the payload
    pub payload_len: u8,
    /// The header CRC
    pub h_crc: u8,
    /// The payload itself
    pub payload: [u8; u8::MAX as usize + 1],
    /// The CRC of the frame
    pub f_crc: u8,
    /// (internal) The incoming length of an unassembled frame
    pub in_len: u16,
}

impl DataFrame {
    /// Calculates the frame checksum for this DataFrame
    pub fn f_crc(&self) -> u8 {
        let digest = unsafe { &mut CRC_CALCULATOR };
        digest.reset();

        digest.update(&[START_BYTE_0, START_BYTE_1]);
        digest.update(&[(self.src & 0xff) as u8, (self.src >> 8 & 0xff) as u8]);
        digest.update(&[(self.dst & 0xff) as u8, (self.dst >> 8 & 0xff) as u8]);
        digest.update(&[(self.cmd & 0xff) as u8, (self.cmd >> 8 & 0xff) as u8]);
        digest.update(&[self.payload_len]);
        digest.update(&[self.h_crc]);

        for pos in 0..self.payload_len {
            digest.update(&[self.payload[pos as usize]]);
        }

        digest.finalize()
    }

    /// Calculates the header checksum for this DataFrame
    pub fn h_crc(&self) -> u8 {
        let digest = unsafe { &mut CRC_CALCULATOR };
        digest.reset();

        digest.update(&[START_BYTE_0, START_BYTE_1]);
        digest.update(&[(self.src & 0xff) as u8, (self.src >> 8 & 0xff) as u8]);
        digest.update(&[(self.dst & 0xff) as u8, (self.dst >> 8 & 0xff) as u8]);
        digest.update(&[(self.cmd & 0xff) as u8, (self.cmd >> 8 & 0xff) as u8]);
        digest.update(&[self.payload_len]);

        digest.finalize()
    }

    /// Updates the internal CRC to the appropriate value for the frame in this state
    pub fn update_crc(&mut self) {
        self.h_crc = self.h_crc();
        self.f_crc = self.f_crc();
    }

    /// Checks if the received CRC and the calculated CRC are the same
    /// # Returns
    /// `true` if the CRC is valid, else `false`
    pub fn check_crc(&self) -> bool {
        self.f_crc == self.f_crc()
    }

    /// Sends this frame to the provided writer, consumes this frame
    /// # Arguments
    /// * `serial` - The writer to send this frame to
    pub fn send<S>(&mut self, serial: &mut S) -> nb::Result<(), S::Error>
    where
        S: Write<u8>,
    {
        self.update_crc();

        // Write start bytes
        block!(serial.write(START_BYTE_0))?;
        block!(serial.write(START_BYTE_1))?;

        // Write src address
        block!(serial.write((self.src & 0xff) as u8))?;
        block!(serial.write((self.src >> 8 & 0xff) as u8))?;

        // Write dst address
        block!(serial.write((self.dst & 0xff) as u8))?;
        block!(serial.write((self.dst >> 8 & 0xff) as u8))?;

        // Write cmd
        block!(serial.write((self.cmd & 0xff) as u8))?;
        block!(serial.write((self.cmd >> 8 & 0xff) as u8))?;

        // Write payload len
        block!(serial.write(self.payload_len))?;

        // Write header crc
        block!(serial.write(self.h_crc))?;

        // Write payload
        for i in 0..self.payload_len {
            block!(serial.write(self.payload[i as usize]))?;
        }

        // Write CRC
        block!(serial.write(self.f_crc))?;

        Ok(())
    }

    fn reset(&mut self) {
        self.in_len = 0;
    }

    pub fn handle_byte(&mut self, byte: u8) -> bool {
        match self.in_len {
            0 => {
                // Start byte 0: 0xaa
                if byte != 0xaa {
                    self.reset();
                    return false;
                }
            }
            1 => {
                // Start byte 1: 0x55
                if byte != 0x55 {
                    self.reset();
                    return false;
                }
            }
            2 => {
                // src low byte
                self.src = byte as u16;
            }
            3 => {
                // src high byte
                self.src |= (byte as u16) << 8;
            }
            4 => {
                // dst low byte
                self.dst = byte as u16;
            }
            5 => {
                // dst high byte
                self.dst |= (byte as u16) << 8;
            }
            6 => {
                // cmd low byte
                self.cmd = byte as u16;
            }
            7 => {
                // cmd high byte
                self.cmd |= (byte as u16) << 8;
            }
            8 => {
                // len
                self.payload_len = byte;
            }
            9 => {
                // Check if the header is valid, else drop the frame
                if byte != self.h_crc() {
                    self.reset();
                    return false;
                }

                // h_crc
                self.h_crc = byte;
            }
            _ => {
                // The index of the last payload byte
                let payload_last = 9 + self.payload_len as u16;
                // The index of the last CRC byte
                let crc_last = payload_last + 1;

                if self.in_len <= payload_last {
                    //Payload
                    self.payload[self.in_len as usize - 10] = byte;
                } else if self.in_len <= crc_last {
                    //CRC
                    self.f_crc = byte;
                }

                // This marks the end of a frame
                if self.in_len == crc_last {
                    // Save the frame and reset the internal one
                    //let frame = self.clone();
                    self.in_len = 0;
                    //self = DataFrame::default();

                    // Return the new frame
                    return true;
                }
            }
        }

        // Increment the counter
        self.in_len += 1;

        false
    }
}
