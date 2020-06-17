//! Hardware Abstraction Layer for 'Secure' digital input/output MultiMediaCard
//! interface (SDMMC) on the STM32H7.
//!
//! This crate depends on the GPIO and Clock functionality from
//! [`stm32h7xx-hal`].
//!
//! # SDMMC
//!
//! The H7 has two SDMMC peripherals, `SDMMC1` and `SDMMC2`.
//!
//! ## IO Setup
//!
//! For high speed signalling (bus clock > 16MHz), the IO speed needs to be
//! increased from the default.
//!
//! ```
//! use stm32h7xx_hal::gpio::Speed;
//!
//! let d0 = d0.set_speed(Speed::VeryHigh);
//! ```
//!
//! ## Usage
//!
//! By default the SDMMC bus clock is derived from the `pll1_q_ck`. This can be
//! set when initialising the RCC.
//!
//! ```
//! let ccdr = rcc
//!     .pll1_q_ck(100.mhz())
//!     .freeze(vos, &dp.SYSCFG);
//! ```
//!
//! There is an [extension trait](crate::SdmmcExt) implemented for the `SDMMC1`
//! and `SDMMC2` periperhals for easy initialisation.
//!
//! ```
//! // Create SDMMC
//! let mut sdmmc = dp.SDMMC1.sdmmc(
//!     (clk, cmd, d0, d1, d2, d3),
//!     ccdr.peripheral.SDMMC1,
//!     &ccdr.clocks,
//! );
//! ```
//!
//! The next step is to initialise a card. The bus speed is also set.
//!
//! ```
//! if let Err(err) = sdmmc.init_card(10.mhz()) {
//!     info!("Init err: {:?}", err);
//! }
//! ```
//!
//! The [`card()`](crate::Sdmmc::card) method returns useful information about
//! the card.
//!
//! ```
//! let card = sdmmc.card();
//! if let Some(card) = sdmmc.card() {
//!     info!("SD Card Connected: {:?}", card);
//! }
//! ```
//!
//! ## License
//!
//! Licensed under either of
//!
//!  * Apache License, Version 2.0
//!    ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
//!  * MIT license
//!    ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
//!
//! at your option.
//!
//! ## Contribution
//!
//! Unless you explicitly state otherwise, any contribution
//! intentionally submitted for inclusion in the work by you, as
//! defined in the Apache-2.0 license, shall be dual licensed as
//! above, without any additional terms or conditions.
//!
//! [`stm32h7xx-hal`]: https://crates.io/crates/stm32h7xx-hal
#![no_std]
// rustc lints.
#![warn(
    bare_trait_objects,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

#[cfg(feature = "log")]
#[macro_use(trace)]
extern crate log;

mod sd_registers;
pub use sd_registers::{SDStatus, CID, CSD, OCR, SCR};

mod sdmmc;
pub use sdmmc::{BusWidth, Card, CardType, Error, Sdmmc, SdmmcExt, Signalling};
