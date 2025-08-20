
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

use rp_pico::hal;
use rp_pico::hal::pac;
use rp_pico::XOSC_CRYSTAL_FREQ;
use rp_pico::hal::Clock;
use rp_pico::hal::fugit::RateExtU32;

use usb_device::prelude::*;
use usb_device::class_prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};
use embedded_hal::digital::OutputPin;

// Source identification for debugging
#[derive(Clone, Copy, PartialEq)]
enum DataSource {
    PC,     // Data from PC (CDC0)
    DUT,    // Data from Device Under Test (UART)
}

// Convert number to hex string (no_std compatible)
fn u8_to_hex(byte: u8) -> [u8; 2] {
    let hex_chars = b"0123456789abcdef";
    [hex_chars[(byte >> 4) as usize], hex_chars[(byte & 0x0F) as usize]]
}

#[entry]
fn main() -> ! {
    // ---- RP2040 Peripherals ----
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    ).ok().unwrap();

    let sio = hal::Sio::new(pac.SIO);
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Setup onboard LED for data indication
    let mut led_pin = pins.led.into_push_pull_output();

    // ---- Hardware UART0 ----
    let uart_pins = (
        pins.gpio0.into_function::<hal::gpio::FunctionUart>(), // TX
        pins.gpio1.into_function::<hal::gpio::FunctionUart>(), // RX
    );
    
    // Store UART configuration for easy recreation
    let mut uart_config = hal::uart::UartConfig::new(
        115200u32.Hz(), 
        hal::uart::DataBits::Eight, 
        None, 
        hal::uart::StopBits::One
    );
    
    let mut uart = hal::uart::UartPeripheral::new(
        pac.UART0, uart_pins, &mut pac.RESETS
    )
    .enable(uart_config, clocks.peripheral_clock.freq())
    .unwrap();

    // ---- USB setup ----
    let usb_bus = hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    );
    let usb_bus = UsbBusAllocator::new(usb_bus);

    let mut cdc0 = SerialPort::new(&usb_bus);
    let mut cdc1 = SerialPort::new(&usb_bus); // debug output

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .device_class(USB_CLASS_CDC)
        .build();

    let mut buf = [0u8; 64];
    let mut current_baudrate = 115200u32;
    
    // Debug output state
    let mut current_source: Option<DataSource> = None;
    let mut bytes_on_line: u8 = 0;
    
    // LED state for data indication
    let mut led_counter = 0u16;

    loop {
        if usb_dev.poll(&mut [&mut cdc0, &mut cdc1]) {
            // Check for CDC0 line coding changes (baudrate)
            let line_coding = cdc0.line_coding();
            let new_baudrate = line_coding.data_rate();
            if new_baudrate != current_baudrate && new_baudrate > 0 {
                // Add newline if we were outputting data
                if current_source.is_some() {
                    let _ = cdc1.write(b"\r\n");
                    current_source = None;
                    bytes_on_line = 0;
                }
                
                // Reconfigure UART with new baudrate
                uart_config = hal::uart::UartConfig::new(
                    new_baudrate.Hz(),
                    hal::uart::DataBits::Eight,
                    None,
                    hal::uart::StopBits::One,
                );
                
                // Disable and re-enable UART with new config
                uart = uart.disable().enable(uart_config, clocks.peripheral_clock.freq()).unwrap();
                current_baudrate = new_baudrate;
                
                // Send baudrate change notification to debug port
                let _ = cdc1.write(b"[INFO] Baudrate changed to ");
                // Convert baudrate to string manually
                let mut baud_str = [0u8; 10];
                let mut temp = new_baudrate;
                let mut i = 0;
                while temp > 0 && i < 10 {
                    baud_str[9-i] = b'0' + (temp % 10) as u8;
                    temp /= 10;
                    i += 1;
                }
                let _ = cdc1.write(&baud_str[10-i..]);
                let _ = cdc1.write(b" bps\r\n");
            }
            
            // --- USB CDC0 -> UART0 ---
            if let Ok(count) = cdc0.read(&mut buf) {
                if count > 0 {
                    let data = &buf[..count];
                    let _ = uart.write_full_blocking(data);
                    
                    // Blink LED for data activity
                    led_pin.set_high().unwrap();
                    led_counter = 500; // Keep LED on for ~500 polls
                    
                    // Output to debug with grouping logic
                    for &byte in data {
                        // Check if source changed
                        if current_source != Some(DataSource::PC) {
                            // If we were already outputting data, add newline first
                            if current_source.is_some() {
                                let _ = cdc1.write(b"\r\n");
                            }
                            
                            // Output new source header
                            let _ = cdc1.write(b"[PC->DUT]\r\n");
                            current_source = Some(DataSource::PC);
                            bytes_on_line = 0;
                        }

                        // If we've reached console width (16 bytes per line), start new line
                        if bytes_on_line >= 16 {
                            let _ = cdc1.write(b"\r\n");
                            bytes_on_line = 0;
                        }

                        // Output hex byte
                        let hex_byte = u8_to_hex(byte);
                        let _ = cdc1.write(&hex_byte);
                        let _ = cdc1.write(b" ");
                        
                        bytes_on_line += 1;
                    }
                }
            }

            // --- UART0 -> USB CDC0 ---
            if uart.uart_is_readable() {
                let mut byte_buf = [0u8; 1];
                if let Ok(1) = uart.read_raw(&mut byte_buf) {
                    let _ = cdc0.write(&byte_buf);
                    
                    // Blink LED for data activity
                    led_pin.set_high().unwrap();
                    led_counter = 500; // Keep LED on for ~500 polls
                    
                    // Output to debug with grouping logic
                    let byte = byte_buf[0];
                    
                    // Check if source changed
                    if current_source != Some(DataSource::DUT) {
                        // If we were already outputting data, add newline first
                        if current_source.is_some() {
                            let _ = cdc1.write(b"\r\n");
                        }
                        
                        // Output new source header
                        let _ = cdc1.write(b"[DUT->PC]\r\n");
                        current_source = Some(DataSource::DUT);
                        bytes_on_line = 0;
                    }

                    // If we've reached console width (16 bytes per line), start new line
                    if bytes_on_line >= 16 {
                        let _ = cdc1.write(b"\r\n");
                        bytes_on_line = 0;
                    }

                    // Output hex byte
                    let hex_byte = u8_to_hex(byte);
                    let _ = cdc1.write(&hex_byte);
                    let _ = cdc1.write(b" ");
                    
                    bytes_on_line += 1;
                }
            }


            
            // --- Ignore anything typed into CDC1 ---
            if let Ok(count) = cdc1.read(&mut buf) {
                if count > 0 {
                    // discard any input
                }
            }
            
            // --- LED management ---
            if led_counter > 0 {
                led_counter -= 1;
            } else {
                led_pin.set_low().unwrap();
            }
        }
    }
}
