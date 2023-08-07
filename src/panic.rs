use arduino_hal::delay_ms;
use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    unsafe {
        let dp = arduino_hal::Peripherals::steal();
        let pins = arduino_hal::pins!(dp);
        let mut led = pins.d13.into_output().downgrade();

        let del = 50;
        loop {
            led.set_high();
            delay_ms(del);
            led.set_low();
            delay_ms(del);
        }
    }
}
