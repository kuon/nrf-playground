## Nrf dev playground

This repository is a playground for the nrf52840 chip.

### Requirements

- `openocd` >= `0.10.0+dev-00941-g3a50bb46d`
- `cgdb` >= `20190813`
- `arm-none-eabi-gdb` >= `8.3.1`
- `rustup` >= `1.20`
- `cargo` >= `1.38.0`

Rustup target:

`rustup target add thumbv7em-none-eabihf`

### How to run a debug session

#### With the laird dev board

The following hardware is used:

- Laird dev board for BL654

1. Plug the board on USB 2 port (Labeled Atmel USB).
2. Start `openocd` with `openocd -f config/laird.cfg`. This starts an OpenOCD
   gdb listener on port 3333 and a telnet listener on port 4444.
3. Then run the debug session with `cargo run`, it will start a GDB session and
   connect to the OpenOCD debugger.


#### With the olimex debugger

The following hardware is used:

- Olimex programmer *ARM-USB-TINY-H* <https://www.olimex.com/Products/ARM/JTAG/ARM-USB-TINY-H/>
- Olimex SWD adapter *ARM-JTAG-SWD* <https://www.olimex.com/Products/ARM/JTAG/ARM-JTAG-SWD/>
- Olimex JTAG adapter *ARM-JTAG-20-10* <https://www.olimex.com/Products/ARM/JTAG/ARM-JTAG-20-10/>
- Diuriflux sensor board

1. Plug the JTAG on the board and power the board.
2. Start `openocd` with `openocd -f config/olimex.cfg`. This starts an OpenOCD
   gdb listener on port 3333 and a telnet listener on port 4444.
3. Then run the debug session with `cargo run`, it will start a GDB session and
   connect to the OpenOCD debugger.

