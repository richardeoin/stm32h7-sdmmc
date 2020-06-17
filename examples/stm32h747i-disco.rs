//! Example for STM32H&47I DISCO board

// #![deny(warnings)]
//#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_itm;

use cortex_m_rt::entry;
use stm32h7xx_hal::gpio::Speed;
use stm32h7xx_hal::hal::digital::v2::ToggleableOutputPin;
use stm32h7xx_hal::{pac, prelude::*};

use stm32h7_sdmmc::SdmmcExt;

use cortex_m_log::println;
use cortex_m_log::{
    destination::Itm, printer::itm::InterruptSync as InterruptSyncItm,
};

#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let mut log = InterruptSyncItm::new(Itm::new(cp.ITM));

    low_level_itm(&dp.DBGMCU, true);

    // Constrain and Freeze power
    println!(log, "Setup PWR...                  ");
    let pwr = dp.PWR.constrain();
    let vos = pwr.smps().freeze();

    // Constrain and Freeze clock
    println!(log, "Setup RCC...                  ");
    let rcc = dp.RCC.constrain();

    let ccdr = rcc
        .sys_ck(400.mhz())
        .pll1_q_ck(100.mhz())
        .freeze(vos, &dp.SYSCFG);

    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
    gpiob.pb3.into_alternate_af0();

    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);
    let gpiod = dp.GPIOD.split(ccdr.peripheral.GPIOD);
    let gpioi = dp.GPIOI.split(ccdr.peripheral.GPIOI);

    let mut led = gpioi.pi12.into_push_pull_output();

    // Get the delay provider.
    let mut delay = cp.SYST.delay(ccdr.clocks);

    // SDMMC pins
    let clk = gpioc
        .pc12
        .into_alternate_af12()
        .internal_pull_up(false)
        .set_speed(Speed::VeryHigh);
    let cmd = gpiod
        .pd2
        .into_alternate_af12()
        .internal_pull_up(true)
        .set_speed(Speed::VeryHigh);
    let d0 = gpioc
        .pc8
        .into_alternate_af12()
        .internal_pull_up(true)
        .set_speed(Speed::VeryHigh);
    let d1 = gpioc
        .pc9
        .into_alternate_af12()
        .internal_pull_up(true)
        .set_speed(Speed::VeryHigh);
    let d2 = gpioc
        .pc10
        .into_alternate_af12()
        .internal_pull_up(true)
        .set_speed(Speed::VeryHigh);
    let d3 = gpioc
        .pc11
        .into_alternate_af12()
        .internal_pull_up(true)
        .set_speed(Speed::VeryHigh);

    // Card detect pin
    let _cd = gpioi.pi8.into_pull_up_input();

    // Create SDMMC
    let mut sdmmc = dp.SDMMC1.sdmmc(
        (clk, cmd, d0, d1, d2, d3),
        ccdr.peripheral.SDMMC1,
        &ccdr.clocks,
    );

    // Loop until we have a card
    loop {
        match sdmmc.init_card(50.mhz()) {
            Ok(_) => break,
            Err(err) => {
                println!(log, "Init err: {:?}", err);
            }
        }

        println!(log, "Waiting for card...");

        delay.delay_ms(1000u32);
        led.toggle().ok();
    }

    // Print card information
    println!(log, "");
    println!(log, "----------------------");

    let size = sdmmc.card().unwrap().size();
    println!(log, "Size: {}", size);

    let ocr = sdmmc.card().unwrap().ocr;
    println!(log, "{:?}", ocr);

    let scr = sdmmc.card().unwrap().scr;
    println!(log, "{:?}", scr);

    let cid = sdmmc.card().unwrap().cid;
    println!(log, "{:?}", cid);

    let csd = sdmmc.card().unwrap().csd;
    println!(log, "{:?}", csd);

    let status = sdmmc.card().unwrap().status;
    println!(log, "{:?}", status);

    println!(log, "Bus Clock: {}", sdmmc.clock());
    println!(log, "----------------------");
    println!(log, "");

    let mut buffer = [0u8; 5120];

    cp.DWT.enable_cycle_counter();
    let start = pac::DWT::get_cycle_count();

    for i in 0..1 {
        sdmmc.read_blocks(10 * i, &mut buffer).unwrap();
    }

    let end = pac::DWT::get_cycle_count();
    let duration = (end - start) as f32 / ccdr.clocks.c_ck().0 as f32;

    println!(log, "Read 10 blocks in {} ms", duration * 1000.);

    let mut write_buffer = [0xC3; 512];
    write_buffer[1] = 0;
    write_buffer[3] = 0;
    write_buffer[5] = 0;
    loop {
        // for i in 0..20 {
        //     write_buffer[0] = i as u8;

        //     if let Err(err) = sdmmc.write_block(i, &write_buffer) {
        //         println!(log, "Failed to write block {}: {:?}", i, err);
        //     } else {
        //         println!(log, "Wrote block {}", i);
        //     }
        // }

        loop {}
    }
}

/// Enables ITM
///
/// If swo_enable is true, then the SWO output pin will be enabled for
/// the CM7 core. This currently results in the core locking up for
/// some operations (including HARDWARE reset!). Use with caution.
pub fn low_level_itm(dbgmcu: &pac::DBGMCU, swo_enable: bool) {
    // ARMv7-M DEMCR: Set TRCENA. Enables DWT and ITM units
    unsafe { *(0xE000_EDFC as *mut u32) |= 1 << 24 };

    // Ensure debug blocks are clocked before interacting with them
    dbgmcu.cr.modify(|_, w| {
        w.d1dbgcken()
            .set_bit()
            .d3dbgcken()
            .set_bit()
            .traceclken()
            .set_bit()
        // .dbgsleep_d1()
        // .set_bit()
        // .dbgsleep_d2()
        // .set_bit()
    });

    // Enabling the SWTF trace funnel specifically appears to cause
    // intermittent issues for the core
    if swo_enable {
        // SWO: Unlock
        unsafe { *(0x5c00_3fb0 as *mut u32) = 0xC5ACCE55 };
        // SWTF: Unlock
        unsafe { *(0x5c00_4fb0 as *mut u32) = 0xC5ACCE55 };

        // SWO CODR Register: Set SWO speed
        // 480MHz max. / 400 = 1.2MHz max
        unsafe { *(0x5c00_3010 as *mut _) = 800 - 1 };

        // // SWO SPPR Register: Manchester
        // unsafe { *(0x5c00_30f0 as *mut _) = 1 };

        // SWTF Trace Funnel: Enable for CM7
        unsafe { *(0x5c00_4000 as *mut u32) |= 1 };
    }

    // ITM: Unlock
    unsafe { *(0xE000_0FB0 as *mut u32) = 0xC5ACCE55 };
    // ITM Trace Enable Register: Enable lower 8 stimulus ports
    unsafe { *(0xE000_0E00 as *mut _) = 0xFF };
    // ITM Trace Control Register: Enable ITM
    unsafe { *(0xE000_0E80 as *mut u32) |= 8 | 1 };
}
