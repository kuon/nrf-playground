//#![deny(unsafe_code)]
//#![deny(warnings)]
#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

mod common;
use common::*;

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
use defmt::{debug, info};
use nrf_softdevice::ble::{gatt_server, gatt_server::*, peripheral, Uuid};
use nrf_softdevice::{raw, temperature_celsius, Softdevice};

#[app(device = nrf52840_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        serial: Uarte<UARTE0>,
        timer: Timer<TIMER1, Periodic>,
        #[init(None)]
        ble: Option<DataServiceServer>,
        #[init(None)]
        sd: Option<&'static Softdevice>,
    }

    #[init()]
    fn init(cx: init::Context) -> init::LateResources {
        debug!("Starting up");

        debug!("UART setup");
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

        write!(serial, "\n--- INIT ---\r\n").unwrap();

        debug!("TIMER setup");
        let mut timer = Timer::new(cx.device.TIMER1);
        timer.enable_interrupt();
        let mut timer = timer.into_periodic();
        timer.start(1_000_000u32); // 1Mhz, so once per second

        init::LateResources { serial, timer }
    }

    #[idle(resources = [ble, sd])]
    fn idle(mut cx: idle::Context) -> ! {
        debug!("Softdevice setup");
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
                p_value: b"Sensor" as *const u8 as _,
                current_len: 6,
                max_len: 6,
                write_perm: unsafe { mem::zeroed() },
                _bitfield_1: raw::ble_gap_cfg_device_name_t::new_bitfield_1(
                    raw::BLE_GATTS_VLOC_STACK as u8,
                ),
            }),
            ..Default::default()
        };

        debug!("Softdevice enable");
        let sd = Softdevice::enable(&config);

        cx.resources.sd.lock(|val| {
            *val = Some(sd);
        });

        unsafe {
            let server: DataServiceServer = gatt_server::register(sd).dewrap();

            cx.resources.ble.lock(|ble| {
                *ble = Some(server.clone());
            });

            run_gatt_server.spawn(sd, server).dewrap();
            run_bluetooth.spawn(sd).dewrap();
            run_softdevice.spawn(sd).dewrap();
            static_executor::run();
        }
    }

    #[task(binds = TIMER1, resources = [serial, timer, ble, sd])]
    fn exec(cx: exec::Context) {
        cx.resources.timer.wait().unwrap();
        //debug!("TIMER exec");
        write!(cx.resources.serial, "TIMER EXEC {:?}\r\n", cx.resources.ble)
            .unwrap();

        if let (Some(ble), Some(sd)) = (cx.resources.ble, cx.resources.sd) {
            /*
            let temp = temperature_celsius(&sd).dewrap();
            let temp = temp.to_num::<i32>();

            write!(cx.resources.serial, "TEMP {:?}\r\n", temp).unwrap();

            gatt_server::set_value(sd, ble.value_handle, &temp.to_le_bytes())
                .dewrap();

            gatt_server::set_value(sd, ble.value_handle, &temp.to_le_bytes())
                .dewrap();
            */

            let mut buf: [u8; 4] = [0, 0, 0, 0];
            unsafe {
                if raw::sd_rand_application_vector_get(buf.as_mut_ptr(), 4)
                    != raw::NRF_SUCCESS
                {
                    panic!("Cannot generate random bytes");
                }
            }

            write!(cx.resources.serial, "RAND {:?}\r\n", buf).unwrap();
            gatt_server::set_value(sd, ble.value_handle, &buf).dewrap();
        }
    }

    extern "C" {
        fn SWI0_EGU0();
    }
};

const GATT_UUID: Uuid = Uuid::new_16(0x181C);
const GATT_CHAR_UUID: Uuid = Uuid::new_16(0x1010);

#[derive(Clone, Debug)]
pub struct DataServiceServer {
    value_handle: u16,
    cccd_handle: u16,
}

impl gatt_server::Server for DataServiceServer {
    fn uuid() -> Uuid {
        GATT_UUID
    }

    fn register<F>(
        service_handle: u16,
        mut register_char: F,
    ) -> Result<Self, RegisterError>
    where
        F: FnMut(
            Characteristic,
            &[u8],
        ) -> Result<CharacteristicHandles, RegisterError>,
    {
        let handles = register_char(
            Characteristic {
                uuid: GATT_CHAR_UUID,
                can_indicate: false,
                can_notify: true,
                can_read: true,
                can_write: false,
                max_len: 4,
            },
            &[0, 0, 0, 0],
        )?;

        Ok(Self {
            cccd_handle: handles.cccd_handle,
            value_handle: handles.value_handle,
        })
    }
}

#[static_executor::task]
async fn run_softdevice(sd: &'static Softdevice) {
    debug!("Softdevice run");
    sd.run().await;
}

#[static_executor::task]
async fn run_gatt_server(sd: &'static Softdevice, server: DataServiceServer) {
    debug!("GATT run");
    gatt_server::run(sd, &server).await
}

#[static_executor::task]
async fn run_bluetooth(sd: &'static Softdevice) {
    debug!("Run bluetooth!");

    #[rustfmt::skip]
    let adv_data = &[
        0x02, 0x01, raw::BLE_GAP_ADV_FLAGS_LE_ONLY_GENERAL_DISC_MODE as u8,
        0x03, 0x03, 0x1C, 0x18,
        0x07, 0x09, b'S', b'e', b'n', b's', b'o', b'r'
    ];
    #[rustfmt::skip]
    let scan_data = &[
        0x03, 0x03, 0x1C, 0x18,
    ];

    loop {
        info!("Advertising start!");
        let conn = peripheral::advertise(
            sd,
            peripheral::ConnectableAdvertisement::ScannableUndirected {
                adv_data,
                scan_data,
            },
        )
        .await
        .dewrap();

        info!("Advertising done!");

        // Detach the connection so it isn't disconnected when dropped.
        //conn.detach();
    }
}
