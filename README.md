# Madbus TCP Client

A simple **Rust CLI client** for communicating with a **Modbus TCP server (slave)** using the `madbus` library.  
The program allows reading and writing **coils, discrete inputs, holding registers, and input registers** via terminal commands.

---

# Features

- Connects to a Modbus TCP server
- Supports standard Modbus operations:
  - Read Coils
  - Read Discrete Inputs
  - Read Holding Registers
  - Read Input Registers
  - Write Single / Multiple Coils
  - Write Single / Multiple Holding Registers
- Command-line interface built with `clap`
- Prints formatted responses from the server
- Handles Modbus exception codes

---

# Requirements

- Rust (recommended via `rustup`)
- Cargo
- Access to a **Modbus TCP server**

---

# Installation

Clone the project and build it:

```bash
git clone https://github.com/RipInPease/madbus-cli.git
cd madbus
cargo build --release
```

The binary will be located at:

```
target/release/madbus
```

---

# Usage

```
madbus --ip <IP> [--port <PORT>] [--unit-id <ID>] <COMMAND>
```

Defaults:

- `port`: `502`
- `unit-id`: `1`

Example:

```bash
madbus --ip 192.168.1.10 read-coil 0 5
```

---

# Command Overview

## Read Coils

Reads one or more coil states.

```
madbus --ip <IP> read-coil <start> [count]
```

Example:

```bash
madbus --ip 192.168.1.10 read-coil 0 8
```

---

## Read Discrete Inputs

```
madbus --ip <IP> read-di <start> [count]
```

Example:

```bash
madbus --ip 192.168.1.10 read-di 0 4
```

---

## Read Holding Registers

```
madbus --ip <IP> read-holding <start> [count]
```

Example:

```bash
madbus --ip 192.168.1.10 read-holding 0 3
```

---

## Read Input Registers

```
madbus --ip <IP> read-input <start> [count]
```

Example:

```bash
madbus --ip 192.168.1.10 read-input 0 2
```

---

## Write Coil

Write one or more coils.

```
madbus --ip <IP> write-coil <start> <value...>
```

Example:

```bash
madbus --ip 192.168.1.10 write-coil 0 true
```

Write multiple:

```bash
madbus --ip 192.168.1.10 write-coil 0 true false true
```

---

## Write Holding Registers

```
madbus --ip <IP> write-holding <start> <value...>
```

Example:

```bash
madbus --ip 192.168.1.10 write-holding 0 10
```

Multiple registers:

```bash
madbus --ip 192.168.1.10 write-holding 0 10 20 30
```

---

# Output Example

```
Holding reg values from unit 1:
    40000: 12
    40001: 15
    40002: 42
```

---

# Error Handling

The client handles Modbus exception responses including:

- IllegalCode
- IllegalAddress
- IllegalDataValue
- ServerFailure
- ServerBusy
- GatewayUnavail
- BadDevice
- IOError
- Invalid or malformed Modbus responses

---

# Project Structure

```
src/
 ├── main.rs      # Client logic and response handling
 └── input.rs     # CLI argument definitions
```

---

# Example Full Command

```bash
cargo run -- \
  --ip 127.0.0.1 \
  --port 502 \
  --unit-id 1 \
  read-holding 0 5
```

---

# License

MIT