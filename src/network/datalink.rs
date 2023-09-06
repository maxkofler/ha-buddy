use crc::{Crc, CRC_32_BZIP2, CRC_8_BLUETOOTH};
use embedded_hal::serial::Write;
use nb::block;

const F_CRC_ALGORITHM: Crc<u32> = Crc::<u32>::new(&CRC_32_BZIP2);
const H_CRC_ALGORITHM: Crc<u8> = Crc::<u8>::new(&CRC_8_BLUETOOTH);

const START_BYTE_0: u8 = 0xaa;
const START_BYTE_1: u8 = 0x55;

/// A frame from the Data Link Layer
#[derive(Clone)]
pub struct DataFrame {
    /// The source address
    pub src: u16,
    /// The destination address
    pub dst: u16,
    /// The amount of bytes in the payload
    pub payload_len: u8,
    /// The header CRC
    pub h_crc: u8,
    /// The payload itself
    pub payload: [u8; u8::MAX as usize + 1],
    /// The CRC of the frame
    pub f_crc: u32,
    /// (internal) The incoming length of an unassembled frame
    in_len: u16,
}

/// Represents a data link layer in any stack
pub struct DataLinkLayer {
    /// The frame that is currently being assembled
    cur_frame: DataFrame,
}

impl DataFrame {
    /// Generates a CRC32 - BZIP2 for this DataFrame
    pub fn crc(&self) -> u32 {
        let mut digest = F_CRC_ALGORITHM.digest();

        digest.update(&[START_BYTE_0, START_BYTE_1]);
        digest.update(&[(self.src & 0xff) as u8, (self.src >> 8 & 0xff) as u8]);
        digest.update(&[(self.dst & 0xff) as u8, (self.dst >> 8 & 0xff) as u8]);
        digest.update(&[self.payload_len]);
        digest.update(&[self.h_crc]);

        for pos in 0..self.payload_len {
            digest.update(&[self.payload[pos as usize]]);
        }

        digest.finalize()
    }

    pub fn h_crc(&self) -> u8 {
        let mut digest = H_CRC_ALGORITHM.digest();

        digest.update(&[START_BYTE_0, START_BYTE_1]);
        digest.update(&[(self.src & 0xff) as u8, (self.src >> 8 & 0xff) as u8]);
        digest.update(&[(self.dst & 0xff) as u8, (self.dst >> 8 & 0xff) as u8]);
        digest.update(&[self.payload_len]);

        digest.finalize()
    }

    /// Updates the internal CRC to the appropriate value for the frame in this state
    pub fn update_crc(&mut self) {
        self.h_crc = self.h_crc();
        self.f_crc = self.crc();
    }

    /// Checks if the received CRC and the calculated CRC are the same
    /// # Returns
    /// `true` if the CRC is valid, else `false`
    pub fn check_crc(&self) -> bool {
        self.f_crc == self.crc()
    }

    /// Moves this value, checks if the CRC is valid and returns the frame, else discards it
    /// # Returns
    /// `Some(Self)` if the CRC is valid, else `None`
    pub fn crc_guard(self) -> Option<Self> {
        match self.check_crc() {
            true => Some(self),
            false => None,
        }
    }

    /// Moves this value, checks if the destination address is the desired one and returns the frame, else discards it
    /// # Arguments
    /// * `addr` - The destination address to match against
    /// # Returns
    /// `Some(Self)` if the address matches, else `None`
    pub fn addr_guard(self, addr: u16) -> Option<Self> {
        match self.dst == addr {
            true => Some(self),
            false => None,
        }
    }

    /// Sends this frame to the provided writer, consumes this frame
    /// # Arguments
    /// * `serial` - The writer to send this frame to
    pub fn send<S>(mut self, serial: &mut S) -> nb::Result<(), S::Error>
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

        // Write payload len
        block!(serial.write(self.payload_len))?;

        // Write header crc
        block!(serial.write(self.h_crc))?;

        // Write payload
        for i in 0..self.payload_len {
            block!(serial.write(self.payload[i as usize]))?;
        }

        // Write CRC
        block!(serial.write((self.f_crc & 0xff) as u8))?;
        block!(serial.write((self.f_crc >> 8 & 0xff) as u8))?;
        block!(serial.write((self.f_crc >> 16 & 0xff) as u8))?;
        block!(serial.write((self.f_crc >> 24 & 0xff) as u8))?;

        Ok(())
    }
}

impl DataLinkLayer {
    /// Processes a received byte using the protocol
    /// # Arguments
    /// * `data` - The data byte to process
    /// # Returns
    /// A complete frame if one has been assembled completely
    pub fn handle_byte(&mut self, data: u8) -> Option<DataFrame> {
        match self.cur_frame.in_len {
            0 => {
                // Start byte 0: 0xaa
                if data != 0xaa {
                    self.reset();
                    return None;
                }
            }
            1 => {
                // Start byte 1: 0x55
                if data != 0x55 {
                    self.reset();
                    return None;
                }
            }
            2 => {
                // src low byte
                self.cur_frame.src = data as u16;
            }
            3 => {
                // src high byte
                self.cur_frame.src |= (data as u16) << 8;
            }
            4 => {
                // dst low byte
                self.cur_frame.dst = data as u16;
            }
            5 => {
                // dst high byte
                self.cur_frame.dst |= (data as u16) << 8;
            }
            6 => {
                // len
                self.cur_frame.payload_len = data;
            }
            7 => {
                // Check if the header is valid, else drop the frame
                if data != self.cur_frame.h_crc() {
                    self.reset();
                    return None;
                }

                // h_crc
                self.cur_frame.h_crc = data;
            }
            _ => {
                // The index of the last payload byte
                let payload_last = 7 + self.cur_frame.payload_len as u16;
                // The index of the last CRC byte
                let crc_last = payload_last + 4;

                if self.cur_frame.in_len <= payload_last {
                    //Payload
                    self.cur_frame.payload[self.cur_frame.in_len as usize - 8] = data;
                } else if self.cur_frame.in_len <= crc_last {
                    //CRC
                    let crc_pos =
                        self.cur_frame.in_len as usize - 8 - self.cur_frame.payload_len as usize;
                    self.cur_frame.f_crc |= (data as u32) << (crc_pos * 8);
                }

                // This marks the end of a frame
                if self.cur_frame.in_len == crc_last {
                    // Save the frame and reset the internal one
                    let frame = self.cur_frame.clone();
                    self.cur_frame = DataFrame::default();

                    // Return the new frame
                    return Some(frame);
                }
            }
        }

        // Increment the counter
        self.cur_frame.in_len += 1;

        None
    }

    /// Resets the current frame that is being assembled to an emtpy one
    pub fn reset(&mut self) {
        self.cur_frame = DataFrame::default();
    }
}

impl Default for DataFrame {
    fn default() -> Self {
        DataFrame {
            src: 0,
            dst: 0,
            payload_len: 0,
            h_crc: 0,
            payload: [0; u8::MAX as usize + 1],
            f_crc: 0,
            in_len: 0,
        }
    }
}

impl Default for DataLinkLayer {
    fn default() -> Self {
        DataLinkLayer {
            cur_frame: DataFrame::default(),
        }
    }
}
