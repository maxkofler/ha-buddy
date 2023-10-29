use crate::{
    homeassistant::{
        sensor::SensorRef,
        switch::{SwitchRef, SwitchRequest},
    },
    DataFrame,
};

pub struct HandlerPins {}

/// Handles an incoming frame and possibly mutates the incoming frame for a response.
///
/// The `frame` argument gets mutated and prepared as the response structure.
/// If this function returns true, it shall be transmitted
/// # Arguments
/// * `frame` - The frame to process and mutate for responses
/// * `pins` - Pins that are exposed for the handler
/// * `sensors` - The sensors to handle
/// # Returns
/// True if the modified frame is to be sent
pub fn handle_frame(
    frame: &mut DataFrame,
    _pins: &mut HandlerPins,
    sensors: &[&dyn SensorRef],
    switches: &mut [&mut dyn SwitchRef],
) -> bool {
    match frame.cmd {
        0x0000 => {
            // Echo

            // We do not do anything, the payload stays the same

            true
        }
        0x0100 => {
            // sensor count
            let num_sensors = sensors.len() as u32;

            frame.payload_len = 4;

            frame.payload[0] = num_sensors as u8 & 0xff;
            frame.payload[1] = (num_sensors >> 8) as u8 & 0xff;
            frame.payload[2] = (num_sensors >> 16) as u8 & 0xff;
            frame.payload[3] = (num_sensors >> 24) as u8 & 0xff;

            true
        }
        0x0102 => {
            // Sensor unique_id

            let sensor_id: u32 = match unpack_u32(&frame.payload[0..4]) {
                None => return false,
                Some(id) => id,
            };

            if sensor_id as usize >= sensors.len() {
                frame.payload_len = 0;
                return true;
            }

            let string = sensors[sensor_id as usize].get_unique_id();

            frame.payload_len = string.len() as u8;
            let bytes = string.as_bytes();

            for i in 0..(frame.payload_len as usize) {
                frame.payload[i] = bytes[i];
            }

            true
        }
        0x0104 => {
            // Sensor native_unit_of_measurement

            let sensor_id: u32 = match unpack_u32(&frame.payload[0..4]) {
                None => return false,
                Some(id) => id,
            };

            if sensor_id as usize >= sensors.len() {
                frame.payload_len = 0;
                return true;
            }

            let string = sensors[sensor_id as usize].get_native_unit_of_measurement();

            frame.payload_len = string.len() as u8;
            let bytes = string.as_bytes();

            for i in 0..(frame.payload_len as usize) {
                frame.payload[i] = bytes[i];
            }

            true
        }
        0x0106 => {
            // Sensor device_class

            let sensor_id: u32 = match unpack_u32(&frame.payload[0..4]) {
                None => return false,
                Some(id) => id,
            };

            if sensor_id as usize >= sensors.len() {
                frame.payload_len = 0;
                return true;
            }

            let string = sensors[sensor_id as usize].get_device_class().as_str();

            frame.payload_len = string.len() as u8;
            let bytes = string.as_bytes();

            for i in 0..(frame.payload_len as usize) {
                frame.payload[i] = bytes[i];
            }

            true
        }
        0x0108 => {
            // Sensor state_class

            let sensor_id: u32 = match unpack_u32(&frame.payload[0..4]) {
                None => return false,
                Some(id) => id,
            };

            if sensor_id as usize >= sensors.len() {
                frame.payload_len = 0;
                return true;
            }

            let string = sensors[sensor_id as usize].get_state_class().as_str();

            frame.payload_len = string.len() as u8;
            let bytes = string.as_bytes();

            for i in 0..(frame.payload_len as usize) {
                frame.payload[i] = bytes[i];
            }

            true
        }
        0x0110 => {
            // Sensor device_class

            let sensor_id: u32 = match unpack_u32(&frame.payload[0..4]) {
                None => return false,
                Some(id) => id,
            };

            if sensor_id as usize >= sensors.len() {
                frame.payload_len = 0;
                return true;
            }

            let string = sensors[sensor_id as usize].get_name();

            frame.payload_len = string.len() as u8;
            let bytes = string.as_bytes();

            for i in 0..(frame.payload_len as usize) {
                frame.payload[i] = bytes[i];
            }

            true
        }
        0x0112 => {
            // Sensor value

            let sensor_id: u32 = match unpack_u32(&frame.payload[0..4]) {
                None => return false,
                Some(id) => id,
            };

            if sensor_id as usize >= sensors.len() {
                frame.payload_len = 0;
                return true;
            }

            sensors[sensor_id as usize].get_payload(&mut frame.payload_len, &mut frame.payload);

            true
        }
        0x0200 => {
            // Switch discovery

            let num = switches.len() as u32;

            frame.payload_len = 4;

            frame.payload[0] = num as u8 & 0xff;
            frame.payload[1] = (num >> 8) as u8 & 0xff;
            frame.payload[2] = (num >> 16) as u8 & 0xff;
            frame.payload[3] = (num >> 24) as u8 & 0xff;

            true
        }
        0x0202 => {
            // Switch unique_id

            let switch_id: u32 = match unpack_u32(&frame.payload[0..4]) {
                None => return false,
                Some(id) => id,
            };

            if switch_id as usize >= switches.len() {
                frame.payload_len = 0;
                return true;
            }

            let string = switches[switch_id as usize].get_unique_id();

            frame.payload_len = string.len() as u8;
            let bytes = string.as_bytes();

            for i in 0..(frame.payload_len as usize) {
                frame.payload[i] = bytes[i];
            }

            true
        }
        0x0204 => {
            // Switch name

            let switch_id: u32 = match unpack_u32(&frame.payload[0..4]) {
                None => return false,
                Some(id) => id,
            };

            if switch_id as usize >= switches.len() {
                frame.payload_len = 0;
                return true;
            }

            let string = switches[switch_id as usize].get_name();

            frame.payload_len = string.len() as u8;
            let bytes = string.as_bytes();

            for i in 0..(frame.payload_len as usize) {
                frame.payload[i] = bytes[i];
            }

            true
        }
        0x0206 => {
            // Switch state

            let switch_id: u32 = match unpack_u32(&frame.payload[0..4]) {
                None => return false,
                Some(id) => id,
            };

            if switch_id as usize >= switches.len() {
                frame.payload_len = 0;
                return true;
            }

            frame.payload_len = 1;
            frame.payload[0] = switches[switch_id as usize].exec_request(SwitchRequest::Get) as u8;

            true
        }
        0x0208 => {
            // Switch exec

            let switch_id: u32 = match unpack_u32(&frame.payload[0..4]) {
                None => return false,
                Some(id) => id,
            };

            if switch_id as usize >= switches.len() {
                frame.payload_len = 0;
                return true;
            }

            let req = match frame.payload[4] {
                0 => SwitchRequest::TurnOFF,
                1 => SwitchRequest::TurnON,
                2 => SwitchRequest::Toggle,
                _ => {
                    frame.payload_len = 0;
                    return true;
                }
            };

            switches[switch_id as usize].exec_request(req);

            frame.payload_len = 1;
            frame.payload[0] = switches[switch_id as usize].exec_request(SwitchRequest::Get) as u8;

            true
        }
        _ => false,
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
