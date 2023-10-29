mod device_class;
pub use device_class::*;

/// Common shared attributes for a homeassistant entity
pub trait Entity<'a> {
    /// Returns the unique id of this entity within this device / address
    ///
    /// https://developers.home-assistant.io/docs/core/entity for more information
    fn get_unique_id(&self) -> &'a str;
    /// Returns the name of the entity
    ///
    /// https://developers.home-assistant.io/docs/core/entity for more information
    fn get_name(&self) -> &'a str;
    /// The device class this entity represents
    ///
    /// https://developers.home-assistant.io/docs/core/entity for more information
    fn get_device_class(&self) -> DeviceClass;
}
