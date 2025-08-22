```
░   ░░░  ░░        ░░░      ░░░   ░░░  ░░        ░░░      ░░░       ░░
▒    ▒▒  ▒▒  ▒▒▒▒▒▒▒▒  ▒▒▒▒  ▒▒    ▒▒  ▒▒▒▒▒  ▒▒▒▒▒  ▒▒▒▒  ▒▒  ▒▒▒▒  ▒
▓  ▓  ▓  ▓▓      ▓▓▓▓  ▓▓▓▓  ▓▓  ▓  ▓  ▓▓▓▓▓  ▓▓▓▓▓  ▓▓▓▓  ▓▓       ▓▓
█  ██    ██  ████████  ████  ██  ██    █████  █████        ██  ███████
█  ███   ██        ███      ███  ███   █████  █████  ████  ██  ███████
```
# NeonTap

A USB-to-UART bridge with sniffing build in.

## Features

- **Dual CDC Ports**: Two USB serial ports for control and monitoring
- **Hexdump Output**: Source identification with hex/ASCII formatted monitoring
- **Real-time Analysis**: Live monitoring of bidirectional serial communications
- **Dynamic Baudrate**: Automatically adjusts UART baudrate based on CDC0 settings

## Hardware Setup

**Connections:**

- **GPIO0**: UART TX → Target device RX
- **GPIO1**: UART RX → Target device TX
- **GND**: Common ground
- **USB**: Connect to PC

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
