use arduino_hal::port::{mode::Output, Pin, PinOps};

mod switch_ref;
pub use switch_ref::*;

/// Commands a switch can execute
pub enum SwitchRequest {
    /// Turns the switch on
    TurnON,
    /// Turns the switch off
    TurnOFF,
    /// Toggles the switch state
    Toggle,
    /// Returns the current switch state
    Get,
}

/// A HomeAssistant Switch
///
/// https://developers.home-assistant.io/docs/core/entity/switch for more information
pub struct Switch<'a, F: FnMut(SwitchRequest) -> bool> {
    /// The friendly name for the entity
    pub name: &'a str,
    /// The `unique_id` for this entity
    pub unique_id: &'a str,
    /// Update the state of the switch
    pub callback: F,
}

/// A switch that uses a pin directly
pub struct PinSwitch<'a, PIN> {
    /// The friendly name for the entity
    pub name: &'a str,
    /// The `unique_id` for this entity
    pub unique_id: &'a str,
    /// If the pin state should be negated, inverting all pin states and requests
    pub negate: bool,
    /// The pin to operate on
    pin: Pin<Output, PIN>,
}

impl<'a, F: FnMut(SwitchRequest) -> bool> Switch<'a, F> {
    /// Create a new switch
    /// # Arguments
    /// * `name` - The friendly name for the switch
    /// * `unique_id` - The unique id for the switch
    /// * `callback` - The callback to use for incoming SwitchRequests
    pub fn new(name: &'a str, unique_id: &'a str, callback: F) -> Self {
        Self {
            name,
            unique_id,
            callback,
        }
    }
}

impl<'a, PIN: PinOps> PinSwitch<'a, PIN> {
    /// Creates a new PinSwitch
    /// # Arguments
    /// * `name` - The friendly name for the switch
    /// * `unique_id` - The unique id for the switch
    /// * `pin` - The pin to operate on
    /// * `negate` - Negates the pin, inverting all requests
    pub fn new(name: &'a str, unique_id: &'a str, pin: Pin<Output, PIN>, negate: bool) -> Self {
        Self {
            name,
            unique_id,
            negate,
            pin,
        }
    }

    /// The internal callback handler to handle incoming SwitchRequests
    fn callback(&mut self, req: SwitchRequest) -> bool {
        match req {
            SwitchRequest::TurnON => {
                if self.negate {
                    self.pin.set_low();
                } else {
                    self.pin.set_high();
                }
                true
            }
            SwitchRequest::TurnOFF => {
                if self.negate {
                    self.pin.set_high();
                } else {
                    self.pin.set_low();
                }
                true
            }
            SwitchRequest::Toggle => {
                self.pin.toggle();
                true
            }
            SwitchRequest::Get => {
                if self.negate {
                    self.pin.is_set_low()
                } else {
                    self.pin.is_set_high()
                }
            }
        }
    }
}
