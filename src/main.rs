//#![deny(unsafe_code)]
//#![deny(warnings)]
#![no_main]
#![no_std]

use defmt_rtt as _;
use panic_probe as _;

use nrf52840_hal as hal;

use {
    core::fmt::Write,
    hal::pac::{TIMER1, UARTE0},
    hal::{
        gpio::Level,
        prelude::*,
        timer::{Periodic, Timer},
        uarte::{Baudrate, Parity, Pins, Uarte},
    },
    rtic::app,
};

use core::mem;
use nrf_softdevice::ble::{peripheral, Uuid};
use nrf_softdevice::{raw, Error, Softdevice};

use defmt::debug;

#[app(device = nrf52840_hal::pac, peripherals = true)]
const APP: () = {
    /*
    struct Resources {
        serial: Uarte<UARTE0>,
        timer: Timer<TIMER1, Periodic>,
    }
    */

    #[init()]
    fn init(mut cx: init::Context) { //-> init::LateResources {
        debug!("Starting up");
        let config = nrf_softdevice::Config {
            clock: Some(raw::nrf_clock_lf_cfg_t {
                source: raw::NRF_CLOCK_LF_SRC_XTAL as u8,
                rc_ctiv: 0,
                rc_temp_ctiv: 0,
                accuracy: 7,
            }),
            conn_gap: Some(raw::ble_gap_conn_cfg_t {
                conn_count: 6,
                event_length: 6,
            }),
            conn_gatt: Some(raw::ble_gatt_conn_cfg_t { att_mtu: 128 }),
            gatts_attr_tab_size: Some(raw::ble_gatts_cfg_attr_tab_size_t {
                attr_tab_size: 32768,
            }),
            gap_role_count: Some(raw::ble_gap_cfg_role_count_t {
                adv_set_count: 1,
                periph_role_count: 3,
                central_role_count: 3,
                central_sec_count: 0,
                _bitfield_1: raw::ble_gap_cfg_role_count_t::new_bitfield_1(0),
            }),
            gap_device_name: Some(raw::ble_gap_cfg_device_name_t {
                p_value: b"HelloRust" as *const u8 as _,
                current_len: 9,
                max_len: 9,
                write_perm: unsafe { mem::zeroed() },
                _bitfield_1: raw::ble_gap_cfg_device_name_t::new_bitfield_1(
                    raw::BLE_GATTS_VLOC_STACK as u8,
                ),
            }),
            ..Default::default()
        };
        debug!("Config created");

        let sd = Softdevice::enable(&config);

        // Enable external crystal

        /*
        let _clocks =
            hal::clocks::Clocks::new(cx.device.CLOCK).enable_ext_hfosc();
        */

        /*
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

        let mut timer = Timer::new(cx.device.TIMER1);
        timer.enable_interrupt();
        let mut timer = timer.into_periodic();
        timer.start(1_000_000u32); // 1Mhz, so once per second

        writeln!(serial, "\n--- END --- \r\n").unwrap();

        init::LateResources { serial, timer }
        */
    }
    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {}
    }

    /*
    #[task(binds = TIMER1, resources = [serial, timer])]
    fn exec(cx: exec::Context) {
        cx.resources.timer.wait().unwrap();
        writeln!(cx.resources.serial, "\n--- TIMER EXEC ---\r\n").unwrap();
    }
    */
};
