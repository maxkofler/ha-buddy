use crate::{network::DataFrame, sensor::SensorRef};

pub struct HandlerPins {}

/// Handles an incoming frame
/// # Arguments
/// * `frame` - The frame to process
/// * `pins` - Pins that are exposed for the handler
/// * `sensors` - The sensors to handle
/// # Returns
/// A `DataFrame` as response, else None
pub fn handle_frame(
    frame: DataFrame,
    pins: &mut HandlerPins,
    sensors: &[&dyn SensorRef],
) -> Option<DataFrame> {
    let mut res = DataFrame::default();
    res.cmd = frame.cmd + 1;
    match frame.cmd {
        0x0000 => {
            // Echo
            for i in 0..(frame.payload_len as usize) {
                res.payload[i] = frame.payload[i]
            }

            Some(res)
        }
        0x0100 => {
            // sensor count
            let num_sensors = sensors.len() as u32;

            res.payload_len = 4;

            res.payload[0] = num_sensors as u8 & 0xff;
            res.payload[1] = (num_sensors >> 8) as u8 & 0xff;
            res.payload[2] = (num_sensors >> 16) as u8 & 0xff;
            res.payload[3] = (num_sensors >> 24) as u8 & 0xff;

            Some(res)
        }
        0x0102 => {
            // Sensor unique_id

            let sensor_id: u32 = match unpack_u32(&frame.payload[0..4]) {
                None => return None,
                Some(id) => id,
            };

            if sensor_id as usize >= sensors.len() {
                res.payload_len = 0;
                return Some(res);
            }

            let string = sensors[sensor_id as usize].get_unique_id();

            res.payload_len = string.len() as u8;
            let bytes = string.as_bytes();

            for i in 0..(res.payload_len as usize) {
                res.payload[i] = bytes[i];
            }

            Some(res)
        }
        0x0104 => {
            // Sensor native_unit_of_measurement

            let sensor_id: u32 = match unpack_u32(&frame.payload[0..4]) {
                None => return None,
                Some(id) => id,
            };

            if sensor_id as usize >= sensors.len() {
                res.payload_len = 0;
                return Some(res);
            }

            let string = sensors[sensor_id as usize].get_native_unit_of_measurement();

            res.payload_len = string.len() as u8;
            let bytes = string.as_bytes();

            for i in 0..(res.payload_len as usize) {
                res.payload[i] = bytes[i];
            }

            Some(res)
        }
        0x0106 => {
            // Sensor device_class

            let sensor_id: u32 = match unpack_u32(&frame.payload[0..4]) {
                None => return None,
                Some(id) => id,
            };

            if sensor_id as usize >= sensors.len() {
                res.payload_len = 0;
                return Some(res);
            }

            let string = sensors[sensor_id as usize].get_device_class().as_str();

            res.payload_len = string.len() as u8;
            let bytes = string.as_bytes();

            for i in 0..(res.payload_len as usize) {
                res.payload[i] = bytes[i];
            }

            Some(res)
        }
        0x0108 => {
            // Sensor state_class

            let sensor_id: u32 = match unpack_u32(&frame.payload[0..4]) {
                None => return None,
                Some(id) => id,
            };

            if sensor_id as usize >= sensors.len() {
                res.payload_len = 0;
                return Some(res);
            }

            let string = sensors[sensor_id as usize].get_state_class().as_str();

            res.payload_len = string.len() as u8;
            let bytes = string.as_bytes();

            for i in 0..(res.payload_len as usize) {
                res.payload[i] = bytes[i];
            }

            Some(res)
        }
        0x0110 => {
            // Sensor device_class

            let sensor_id: u32 = match unpack_u32(&frame.payload[0..4]) {
                None => return None,
                Some(id) => id,
            };

            if sensor_id as usize >= sensors.len() {
                res.payload_len = 0;
                return Some(res);
            }

            let string = sensors[sensor_id as usize].get_name();

            res.payload_len = string.len() as u8;
            let bytes = string.as_bytes();

            for i in 0..(res.payload_len as usize) {
                res.payload[i] = bytes[i];
            }

            Some(res)
        }
        0x0112 => {
            // Sensor value

            let sensor_id: u32 = match unpack_u32(&frame.payload[0..4]) {
                None => return None,
                Some(id) => id,
            };

            if sensor_id as usize >= sensors.len() {
                res.payload_len = 0;
                return Some(res);
            }

            sensors[sensor_id as usize].get_payload(&mut res.payload_len, &mut res.payload);

            Some(res)
        }
        _ => None,
    }
}

/// Unpacks a `u32` value from 4 bytes of `u8`
/// # Arguments
/// * `bytes` - The bytes to unpack
fn unpack_u32(bytes: &[u8]) -> Option<u32> {
    if bytes.len() < 4 {
        return None;
    }

    let mut res: u32 = bytes[0] as u32;
    res |= (bytes[1] as u32) << 8;
    res |= (bytes[2] as u32) << 16;
    res |= (bytes[3] as u32) << 24;
    Some(res)
}
