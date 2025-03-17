# Timex Datalink Rust Library

This is a Rust implementation of the Timex Datalink protocol, ported from the Ruby gem `timex_datalink_client`.

## Running Examples

The repository includes example code for Protocol 4 watches:

```bash
# Run the Protocol 4 example with a watch connected to /dev/ttyACM0
cargo run --example protocol4_example /dev/ttyACM0
```

Replace `/dev/ttyACM0` with the appropriate port where your Datalink USB adapter is connected.

## Features

- Protocol 4 implementation
- NotebookAdapter for serial communication with Timex watches
- Character encoding utilities
- Support for various data types (Time, Alarms, Appointments, etc.)