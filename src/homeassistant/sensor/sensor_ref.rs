use super::*;

pub trait SensorRef<'a> {
    /// Returns the name of the sensor
    ///
    /// https://developers.home-assistant.io/docs/core/entity/sensor for more information
    fn get_name(&self) -> &'a str;
    /// Returns the unique id of this sensor within this device / address
    ///
    /// https://developers.home-assistant.io/docs/core/entity/sensor for more information
    fn get_unique_id(&self) -> &'a str;
    /// The native unit of measurement
    ///
    /// https://developers.home-assistant.io/docs/core/entity/sensor for more information
    fn get_native_unit_of_measurement(&self) -> &'a str;
    /// The device class this sensor represents
    ///
    /// https://developers.home-assistant.io/docs/core/entity/sensor for more information
    fn get_device_class(&self) -> DeviceClass;
    /// The state class (type of measurement)
    ///
    /// https://developers.home-assistant.io/docs/core/entity/sensor for more information
    fn get_state_class(&self) -> StateClass;
    /// Update the payload reference to the sensor value for transmission
    /// # Arguments
    /// * `len` - A mutable reference to the payload len
    /// * `payload` - A mutable reference to the payload array
    fn get_payload(&self, len: &mut u8, payload: &mut [u8; u8::MAX as usize + 1]);
}

/// Implement the SensorRef trait for the Sensor
impl<'a, T: SensorValue> SensorRef<'a> for Sensor<'a, T> {
    fn get_name(&self) -> &'a str {
        self.name
    }

    fn get_unique_id(&self) -> &'a str {
        self.unique_id
    }

    fn get_native_unit_of_measurement(&self) -> &'a str {
        self.native_unit_of_measurement
    }

    fn get_device_class(&self) -> DeviceClass {
        self.device_class
    }

    fn get_state_class(&self) -> StateClass {
        self.state_class
    }

    fn get_payload(&self, len: &mut u8, payload: &mut [u8; u8::MAX as usize + 1]) {
        self.value.borrow().to_payload(len, payload)
    }
}