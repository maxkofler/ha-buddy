use onewire::OneWire;

/// Initiates a measurement for all sensors on the OneWire bus
/// # Arguments
/// * `ow` - The BUS to talk on
/// * `wait` - If this function should wait for the measurement to finish, polling
pub fn initiate_measuremet<'a, E: core::fmt::Debug>(ow: &mut OneWire<'a, E>, wait: bool) -> bool {
    let mut delay = arduino_hal::Delay::new();

    if ow.reset(&mut delay).is_err() {
        return false;
    }

    if ow.write_bytes(&mut delay, &[0xcc, 0x44]).is_err() {
        return false;
    }

    if wait {
        let mut one_byte = [0];
        // Wait for the temperature conversion to finish
        while one_byte[0] == 0 {
            if ow.read_bytes(&mut delay, &mut one_byte).is_err() {
                return false;
            }
        }
    }

    true
}

/// Reads the scratchpad of the sensor, assuming only one sensor on the bus
/// # Arguments
/// * `ow` - The BUS to talk on
pub fn read_temp<'a, E: core::fmt::Debug>(ow: &mut OneWire<'a, E>) -> Option<f32> {
    let mut delay = arduino_hal::Delay::new();
    let mut scratchpad = [0; 2];

    // Reset and read the scratchpad
    if ow.reset(&mut delay).is_err() {
        return None;
    }

    if ow.write_bytes(&mut delay, &[0xcc, 0xbe]).is_err() {
        return None;
    }
    if ow.read_bytes(&mut delay, &mut scratchpad).is_err() {
        return None;
    }

    let t = u16::from_le_bytes([scratchpad[0], scratchpad[1]]) as f32 / 16.0;

    Some(t)
}

/// Commands the sensor on the BUS to initiate a temperature measurement,
/// waits for the data to become ready and pulls it
/// # Arguments
/// * `ow` - The BUS to talk on
pub fn measure_and_read<'a, E: core::fmt::Debug>(ow: &mut OneWire<'a, E>) -> Option<f32> {
    if !initiate_measuremet(ow, true) {
        return None;
    }

    read_temp(ow)
}
