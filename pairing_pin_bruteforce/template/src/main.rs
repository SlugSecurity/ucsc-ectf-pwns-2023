use std::{thread, time::Duration};

use clap::{arg, Parser};
use serialport::SerialPort;

#[derive(Parser)]
struct Args {
    /// PIN to start cracking from. Default = 000000.
    #[arg(short, long)]
    start_pin: Option<String>,

    /// PIN to stop cracking at (inclusive). Default = ffffff.
    #[arg(short, long)]
    stop_pin: Option<String>,

    /// How often to output the current PIN attempt in seconds. Default = 1.
    #[arg(short, long)]
    current_pin_interval: Option<u32>,

    /// Delay between PIN attempts in milliseconds. Default = 10.
    #[arg(short, long)]
    pin_attempt_delay: Option<u32>,

    /// Board number to run the script on according to the ESP32 (0 or 1).
    board_number: u8,

    /// Serial device of the paired fob's UART0.
    uart0_serial_file_name: String,

    /// Serial device of the paired fob's UART1.
    uart1_serial_file_name: String,

    /// Serial device of the ESP32 TI board controller.
    esp32_serial_file_name: String,
}

const BAUD_RATE: u32 = 115200;
const DEFAULT_START_PIN: u32 = 0;
const DEFAULT_STOP_PIN: u32 = 0xffffff;
const DEFAULT_CURRENT_PIN_INTERVAL: u32 = 1; // In seconds.
const DEFAULT_PIN_ATTEMPT_DELAY: u32 = 10; // In milliseconds.
const RESET_HOLD_TIME: u64 = 10; // In microseconds.

fn pair(uart0: &mut Box<dyn SerialPort>, pin: u32) {
    // TODO: Fill in for a design.
}

fn main() {
    let args = Args::parse();
    let pin_attempt_delay = args.pin_attempt_delay.unwrap_or(DEFAULT_PIN_ATTEMPT_DELAY);

    let start_pin =
        match u32::from_str_radix(&args.start_pin.unwrap_or(DEFAULT_START_PIN.to_string()), 16) {
            Ok(pin) => pin,
            Err(_) => {
                println!("Failed to parse start PIN.");
                return;
            }
        };

    if start_pin > DEFAULT_STOP_PIN {
        println!("Start PIN is greater than the highest PIN.");
        return;
    }

    let stop_pin =
        match u32::from_str_radix(&args.stop_pin.unwrap_or(DEFAULT_STOP_PIN.to_string()), 16) {
            Ok(pin) => pin,
            Err(_) => {
                println!("Failed to parse stop PIN.");
                return;
            }
        };

    if stop_pin > DEFAULT_STOP_PIN {
        println!("Stop PIN is greater than the highest PIN.");
        return;
    }

    if start_pin > stop_pin {
        println!("Start PIN is greater than stop PIN.");
        return;
    }

    let current_pin_interval = args
        .current_pin_interval
        .unwrap_or(DEFAULT_CURRENT_PIN_INTERVAL);

    let iterations_between_current_pin_output = (current_pin_interval * 1000) / pin_attempt_delay;

    let mut uart0 = serialport::new(args.uart0_serial_file_name, BAUD_RATE)
        .open()
        .expect("Failed to open UART0 serial port.");

    let mut uart1 = serialport::new(args.uart1_serial_file_name, BAUD_RATE)
        .timeout(Duration::from_millis(pin_attempt_delay.into()))
        .open()
        .expect("Failed to open UART1 serial port.");

    uart1
        .clear(serialport::ClearBuffer::All)
        .expect("Failed to clear UART1 serial port.");

    let mut esp32 = serialport::new(args.esp32_serial_file_name, BAUD_RATE)
        .open()
        .expect("Failed to open ESP32 serial port.");

    esp32.write_all(b"h02").expect("Failed to write to ESP32."); // Make sure SW2 is not pressed.

    for pin in start_pin..=stop_pin {
        esp32.write_all(b"l0r").expect("Failed to write to ESP32.");
        thread::sleep(Duration::from_micros(RESET_HOLD_TIME));
        esp32.write_all(b"h0r").expect("Failed to write to ESP32.");

        pair(&mut uart0, pin);

        if pin % iterations_between_current_pin_output == 0 {
            println!("Last PIN tried: {:06x}", pin);
        }

        match uart1.read(&mut [0u8; 1]) {
            Ok(_) => {
                println!("Found PIN: {:06x}", pin);
                return;
            }
            Err(e) => match e.kind() {
                std::io::ErrorKind::TimedOut => {}
                _ => panic!("Failed to read from UART1: {}", e),
            },
        }
    }
}
