# Rust port of caravel
### WIP
To start caravel cli run the following command in the main folder:
`cargo run`

The cli will start up and expect some user input.
The following commands are accepted:
- `help`
> Shows the available commands
- `send`
> Send a can message. Accepts options such as can id **-i**/**\-\-id**, message to be sent **-m**/**\-\-message** and the cycle time if it should be sent cyclically **-c**/**\-\-cyclic**
- `receive`
> Receive can messages. Accepts options such as can id **-i**/**\-\-id** and number of messages to receive **-n**/**\-\-messages**
- `exit`
> Exits the cli

## Example
1. `send -i 35b -m "some message" -c 0`
Sends 1 time message with can id 35b and message "some message"
2. `receive -i 40a -n 10`
Waits to receive 10 times can message with id 40a

## Build for raspberrypi

`cargo build --target armv7-unknown-linux-gnueabihf`