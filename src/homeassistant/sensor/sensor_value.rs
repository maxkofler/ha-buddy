/// The payload type transmitted
#[repr(u8)]
#[allow(dead_code)]
pub enum PayloadType {
    /// All bytes of the payload are the characters in UTF-8 encoding, no null terminator
    String = 0,
    /// 4 byte integer, LSB first
    Int = 1,
    /// 4 byte float, LSB first
    Float = 2,
}

pub trait SensorValue {
    /// Fills a sensor value into a payload array, adjusting the payload length accordingly
    /// # Arguments
    /// * `len` - A mutable reference to the payload len
    /// * `payload` - A mutable reference to the payload array
    fn to_payload(&self, len: &mut u8, payload: &mut [u8; u8::MAX as usize + 1]);
}

impl SensorValue for i32 {
    fn to_payload(&self, len: &mut u8, payload: &mut [u8; u8::MAX as usize + 1]) {
        payload[0] = PayloadType::Int as u8;

        let bytes = self.to_le_bytes();

        payload[1] = bytes[0];
        payload[2] = bytes[1];
        payload[3] = bytes[2];
        payload[4] = bytes[3];

        *len = (bytes.len() + 1) as u8;
    }
}

impl SensorValue for f32 {
    fn to_payload(&self, len: &mut u8, payload: &mut [u8; u8::MAX as usize + 1]) {
        payload[0] = PayloadType::Float as u8;

        let bytes = self.to_le_bytes();

        payload[1] = bytes[0];
        payload[2] = bytes[1];
        payload[3] = bytes[2];
        payload[4] = bytes[3];

        *len = (bytes.len() + 1) as u8;
    }
}
