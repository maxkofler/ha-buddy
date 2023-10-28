use crate::homeassistant::entity::{DeviceClass, Entity};

use super::*;

pub trait SwitchRef<'a>: Entity<'a> {
    /// Execute a SwitchRequest on the switch
    /// # Arguments
    /// * `req` - The `SwitchRequest` to execute
    /// # Returns
    /// In case of `SwitchRequest::Get` the state
    fn exec_request(&mut self, req: SwitchRequest) -> bool;
}

impl<'a, F: FnMut(SwitchRequest) -> bool> Entity<'a> for Switch<'a, F> {
    fn get_unique_id(&self) -> &'a str {
        self.unique_id
    }

    fn get_name(&self) -> &'a str {
        self.name
    }

    fn get_device_class(&self) -> DeviceClass {
        DeviceClass::Switch
    }
}

impl<'a, F: FnMut(SwitchRequest) -> bool> SwitchRef<'a> for Switch<'a, F> {
    fn exec_request(&mut self, req: SwitchRequest) -> bool {
        (self.callback)(req)
    }
}

impl<'a, PIN: PinOps> Entity<'a> for PinSwitch<'a, PIN> {
    fn get_unique_id(&self) -> &'a str {
        self.unique_id
    }

    fn get_name(&self) -> &'a str {
        self.name
    }

    fn get_device_class(&self) -> DeviceClass {
        DeviceClass::Switch
    }
}

impl<'a, PIN: PinOps> SwitchRef<'a> for PinSwitch<'a, PIN> {
    fn exec_request(&mut self, req: SwitchRequest) -> bool {
        self.callback(req)
    }
}
