/// Each sensor has a `device_class` associated to it, refer to https://developers.home-assistant.io/docs/core/entity/sensor for more information
#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum DeviceClass {
    Temperature,
    Switch,
}

impl DeviceClass {
    /// Returns the DeviceClass in string form for transmission and use withing HomeAssistant
    pub fn as_str(&self) -> &'static str {
        match self {
            DeviceClass::Temperature => "DeviceClass.TEMPERATURE",
            DeviceClass::Switch => "SwitchDeviceClass.SWITCH",
        }
    }
}
