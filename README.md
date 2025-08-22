```
░   ░░░  ░░        ░░░      ░░░   ░░░  ░░        ░░░      ░░░       ░░
▒    ▒▒  ▒▒  ▒▒▒▒▒▒▒▒  ▒▒▒▒  ▒▒    ▒▒  ▒▒▒▒▒  ▒▒▒▒▒  ▒▒▒▒  ▒▒  ▒▒▒▒  ▒
▓  ▓  ▓  ▓▓      ▓▓▓▓  ▓▓▓▓  ▓▓  ▓  ▓  ▓▓▓▓▓  ▓▓▓▓▓  ▓▓▓▓  ▓▓       ▓▓
█  ██    ██  ████████  ████  ██  ██    █████  █████        ██  ███████
█  ███   ██        ███      ███  ███   █████  █████  ████  ██  ███████
```
# NeonTap

A USB-to-UART bridge with sniffing built in. Supports both Raspberry Pi Pico and SeeedStudio XIAO RP2040.

## Features

- **Dual CDC Ports**: Two USB serial ports for control and monitoring
- **Hexdump Output**: Source identification with hex/ASCII formatted monitoring
- **Real-time Analysis**: Live monitoring of bidirectional serial communications
- **Dynamic Baudrate**: Automatically adjusts UART baudrate based on CDC0 settings
- **Dual Board Support**: Works with Raspberry Pi Pico and SeeedStudio XIAO RP2040

## Supported Boards

### Raspberry Pi Pico
- **UART Pins**: GPIO0 (TX), GPIO1 (RX)
- **LED**: Built-in LED for data activity indication
- **Build**: `cargo build --features pico`

### SeeedStudio XIAO RP2040
- **UART Pins**: GPIO0 (TX), GPIO1 (RX)
- **LED**: Built-in LED for data activity indication
- **Build**: `cargo build --features xiao`

## Hardware Setup

**Connections (both boards):**

- **GPIO0**: UART TX → Target device RX
- **GPIO1**: UART RX → Target device TX
- **GND**: Common ground
- **USB**: Connect to PC

## Building

### For Raspberry Pi Pico
```bash
cargo build --release --features pico
```

### For SeeedStudio XIAO RP2040
```bash
cargo build --release --features xiao
```

### Generate UF2 Files
```bash
# Generate both UF2 files
./generate_uf2.sh

# Generate specific UF2 file
./generate_uf2.sh pico
./generate_uf2.sh xiao
```

## Usage

The NeonTap enumerates as two USB CDC serial ports:

**CDC0 - Control Port**

- Main communication interface
- Set your desired baudrate, data bits, parity, stop bits
- Data flows: PC ↔ CDC0 ↔ UART ↔ Target Device

**CDC1 - Debug Port**

- Live debugging and analysis output
- Formatted hexdump of all communications

## Debug Output Format

```
[PC->DUT]
48 65 6c 6c 6f 20 57 6f 72 6c 64 21 0a

[DUT->PC]
4f 4b 20 52 65 70 6c 79 0a

[INFO] Baudrate changed to 9600 bps
```

- `PC->DUT`: Data from PC to Device Under Test
- `DUT->PC`: Data from Device Under Test to PC
- Simple hex format with 16 bytes per line
- Automatic baudrate change notifications

## Dynamic Baudrate

1. Connect to CDC0 with your terminal program
2. Change the baudrate setting (9600, 115200, etc.)
3. The physical UART automatically reconfigures
4. CDC1 shows: `[INFO] Baudrate changed to XXXXX bps`

## Flashing

### Raspberry Pi Pico
1. Hold BOOTSEL button while connecting USB
2. Drag and drop `neontap_pico.uf2` to the RPI-RP2 mass storage device

### SeeedStudio XIAO RP2040
1. Double-click the reset button to enter bootloader mode
2. Drag and drop `neontap_xiao.uf2` to the XIAO mass storage device

## Development

### Adding New Boards

To add support for a new RP2040-based board:

1. Add the board's BSP to `Cargo.toml` as an optional dependency
2. Add a new feature in the `[features]` section
3. Add conditional compilation blocks in `src/main.rs` for the new board
4. Update pin mappings and LED configuration as needed
5. Update the build script to handle the new binary name

### Example for a new board:
```rust
#[cfg(feature = "newboard")]
use newboard_bsp::hal;
// ... add pin configuration
#[cfg(feature = "newboard")]
let pins = newboard_bsp::Pins::new(/* ... */);
```
