/// The initial value for the CRC calculation
pub const CRC8_AUTOSAR_INIT: u8 = 0xff;
/// The polynomial for computating the CRC checksum
pub const CRC8_AUTOSAR_POLY: u8 = 0x2f;
/// The value to XOR the final result with
pub const CRC8_AUTOSAR_XOROUT: u8 = 0xff;

pub trait CRC<T> {
    /// Creates a new CRC algorithm and computing instance
    fn new() -> Self;

    /// Resets the CRC computing instance as if it were recreated
    fn reset(&mut self);

    /// Update the CRC computing instance with the supplied bytes
    /// # Arguments
    /// * `data` - The data to calculate into the CRC sum
    fn update(&mut self, data: &[u8]);

    /// Non-destructively finalize the CRC, apply the last XOR and return
    /// the result
    ///
    /// This will not alter any values and can be called multiple times
    fn finalize(&self) -> T;
}

/// An implementation of the AUTOSAR CRC8 algorithm
pub struct CRC8Autosar {
    pub crc: u8,
}

impl CRC<u8> for CRC8Autosar {
    fn new() -> Self {
        Self {
            crc: CRC8_AUTOSAR_INIT,
        }
    }

    fn reset(&mut self) {
        self.crc = CRC8_AUTOSAR_INIT;
    }

    fn update(&mut self, t: &[u8]) {
        for t in t {
            self.crc ^= t;
            for _ in 0..8 {
                if self.crc & 0x80 != 0 {
                    self.crc = (self.crc << 1) ^ CRC8_AUTOSAR_POLY;
                } else {
                    self.crc <<= 1;
                }
            }
        }
    }

    fn finalize(&self) -> u8 {
        self.crc ^ CRC8_AUTOSAR_XOROUT
    }
}
