use core::cell::RefCell;

mod state_class;
pub use state_class::*;

mod sensor_ref;
pub use sensor_ref::*;

mod sensor_value;
pub use sensor_value::*;

use super::entity::DeviceClass;

/// A HomeAssistant Sensor
///
/// https://developers.home-assistant.io/docs/core/entity/sensor for more information
pub struct Sensor<'a, T: SensorValue> {
    /// The friendly name for the sensor
    pub name: &'a str,
    /// The `unique_id` for this sensor
    pub unique_id: &'a str,
    /// The unit of measurement for this sensor
    pub native_unit_of_measurement: &'a str,
    /// The `device_class` for this sensor
    pub device_class: DeviceClass,
    /// The `state_class` for this sensor
    pub state_class: StateClass,
    /// The current value of this sensor
    pub value: RefCell<T>,
}

impl<'a, T: SensorValue> Sensor<'a, T> {
    /// Create a new sensor from the supplied arguments
    pub fn new(
        name: &'a str,
        unique_id: &'a str,
        native_unit_of_measurement: &'a str,
        device_class: DeviceClass,
        state_class: StateClass,
        value: T,
    ) -> Self {
        Self {
            name,
            unique_id,
            native_unit_of_measurement,
            device_class,
            state_class,
            value: RefCell::new(value),
        }
    }

    /// Sets the current value of this sensor
    /// # Arguments
    /// * `value` - The new value to set
    pub fn set_value(&self, value: T) {
        *self.value.borrow_mut() = value;
    }
}
