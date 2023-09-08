/// Each sensor has a `state_class` associated to it, refer to https://developers.home-assistant.io/docs/core/entity/sensor for more information
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum StateClass {
    Measurement,
    Total,
    TotalIncreasing,
}

impl StateClass {
    /// Returns the StateClass in string form for transmission and use withing HomeAssistant
    pub fn as_str(&self) -> &'static str {
        match self {
            StateClass::Measurement => "measurement",
            StateClass::Total => "total",
            StateClass::TotalIncreasing => "total_increasing",
        }
    }
}
