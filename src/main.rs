#![deny(unsafe_code)]
//#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting as _;

use nrf52840_hal as hal;

use {
    core::fmt::Write,
    hal::pac::{TIMER0, UARTE0},
    hal::{
        gpio::Level,
        prelude::*,
        timer::{Periodic, Timer},
        uarte::{Baudrate, Parity, Pins, Uarte},
    },
    rtic::app,
};

#[app(device = nrf52840_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        serial: Uarte<UARTE0>,
        timer: Timer<TIMER0, Periodic>,
    }

    #[init()]
    fn init(mut cx: init::Context) -> init::LateResources {
        // Enable external crystal
        let _clocks =
            hal::clocks::Clocks::new(cx.device.CLOCK).enable_ext_hfosc();

        let p0 = hal::gpio::p0::Parts::new(cx.device.P0);

        let mut serial = Uarte::new(
            cx.device.UARTE0,
            Pins {
                txd: p0.p0_06.into_push_pull_output(Level::High).degrade(),
                rxd: p0.p0_08.into_floating_input().degrade(),
                cts: None,
                rts: None,
            },
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );

        writeln!(serial, "\n--- INIT ---\r\n").unwrap();

        cx.core.DCB.enable_trace();
        cx.core.DWT.enable_cycle_counter();

        let mut timer = Timer::new(cx.device.TIMER0);
        timer.enable_interrupt();
        let mut timer = timer.into_periodic();
        timer.start(1_000_000u32); // 1Mhz, so once per second


        writeln!(serial, "\n--- END --- {:?}\r\n", res).unwrap();

        init::LateResources { serial, timer }
    }
    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {}
    }
    #[task(binds = TIMER0, resources = [serial, timer])]
    fn exec(cx: exec::Context) {
        cx.resources.timer.wait().unwrap();
        writeln!(cx.resources.serial, "\n--- TIMER EXEC ---\r\n").unwrap();
    }
};
