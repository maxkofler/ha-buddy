use crc::{Crc, CRC_32_BZIP2};

const CRC_ALGORITHM: Crc<u32> = Crc::<u32>::new(&CRC_32_BZIP2);

/// A frame from the Data Link Layer
#[derive(Clone)]
pub struct DataFrame {
    /// The destination address
    pub addr: u16,
    /// The amount of bytes in the payload
    pub payload_len: u8,
    /// The payload itself
    pub payload: [u8; u8::MAX as usize + 1],
    /// The CRC of the frame
    pub crc: u32,
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
        let mut digest = CRC_ALGORITHM.digest();
        digest.update(&[(self.addr & 0xff) as u8, (self.addr >> 8 & 0xff) as u8]);
        digest.update(&[self.payload_len]);

        for pos in 0..self.payload_len {
            digest.update(&[self.payload[pos as usize]]);
        }

        digest.finalize()
    }

    /// Updates the internal CRC to the appropriate value for the frame in this state
    pub fn update_crc(&mut self) {
        self.crc = self.crc();
    }

    /// Checks if the received CRC and the calculated CRC are the same
    /// # Returns
    /// `true` if the CRC is valid, else `false`
    pub fn check_crc(&self) -> bool {
        self.crc == self.crc()
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

    /// Moves this value, checks if the address is the desired one and returns the frame, else discards it
    /// # Arguments
    /// * `addr` - The address to match against
    /// # Returns
    /// `Some(Self)` if the address matches, else `None`
    pub fn addr_guard(self, addr: u16) -> Option<Self> {
        match self.addr == addr {
            true => Some(self),
            false => None,
        }
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
                // addr low byte
                self.cur_frame.addr = data as u16;
            }
            1 => {
                // addr low byte
                self.cur_frame.addr |= (data as u16) << 8;
            }
            2 => {
                // len
                self.cur_frame.payload_len = data;
            }
            _ => {
                // The index of the last payload byte
                let payload_last = 2 + self.cur_frame.payload_len as u16;
                // The index of the last CRC byte
                let crc_last = payload_last + 4;

                if self.cur_frame.in_len <= payload_last {
                    //Payload
                    self.cur_frame.payload[self.cur_frame.in_len as usize - 3] = data;
                } else if self.cur_frame.in_len <= crc_last {
                    //CRC
                    let crc_pos =
                        self.cur_frame.in_len as usize - 3 - self.cur_frame.payload_len as usize;
                    self.cur_frame.crc |= (data as u32) << (crc_pos * 8);
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
            addr: 0,
            payload_len: 0,
            payload: [0; u8::MAX as usize + 1],
            crc: 0,
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
