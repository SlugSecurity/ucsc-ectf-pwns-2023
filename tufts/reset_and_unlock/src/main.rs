#![no_std]
#![no_main]

use embedded_hal::digital::v2::{InputPin, OutputPin};
use fugit::RateExtU32;
use hal::pac;
use hal::uart::{DataBits, StopBits, UartConfig};
use panic_halt as _;
use rp2040_hal as hal;
use rp2040_hal::clocks::Clock;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

const XTAL_FREQ_HZ: u32 = 12_000_000u32;

const UNLOCK_MESSAGE: [u8; 2] = [0x53, 0x0];

#[rp2040_hal::entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let sio = hal::Sio::new(pac.SIO);

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let uart_pins = (
        pins.gpio4.into_mode::<hal::gpio::FunctionUart>(),
        pins.gpio5.into_mode::<hal::gpio::FunctionUart>(),
    );
    let uart = hal::uart::UartPeripheral::new(pac.UART1, uart_pins, &mut pac.RESETS)
        .enable(
            UartConfig::new(115_200.Hz(), DataBits::Eight, None, StopBits::One),
            clocks.peripheral_clock.freq(),
        )
        .unwrap();

    let mut reset_pin = pins.gpio6.into_push_pull_output();
    let board_tx = pins.gpio8.into_floating_input();

    // Reset the car
    reset_pin.set_low().unwrap();
    delay.delay_ms(1);
    reset_pin.set_high().unwrap();

    while board_tx.is_low().unwrap() {}

    // Wait for the car to be ready
    delay.delay_us(60000);

    // Send the unlock message
    uart.write_full_blocking(&UNLOCK_MESSAGE);

    delay.delay_us(1000);
    uart.free().1.0.into_floating_input();
    loop {}
}
