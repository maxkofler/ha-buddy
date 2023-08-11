use arduino_hal::{
    hal::port::Dynamic,
    port::{mode::Output, Pin},
};

use crate::network::DataFrame;

pub struct HandlerPins {
    pub l_status: Pin<Output, Dynamic>,
}

/// Handles an incoming frame
/// # Arguments
/// * `frame` - The frame to process
/// * `pins` - Pins that are exposed for the handler
/// # Returns
/// A `DataFrame` as response, else None
pub fn handle_frame(frame: DataFrame, pins: &mut HandlerPins) -> Option<DataFrame> {
    match frame.payload_len {
        0 => None,
        _ => {
            match frame.payload[0] {
                0x00 => {
                    // Echo
                    let mut res = DataFrame::default();
                    res.payload_len = 1;
                    res.payload[0] = frame.payload[1];

                    pins.l_status.toggle();

                    Some(res)
                }
                _ => None,
            }
        }
    }
}
