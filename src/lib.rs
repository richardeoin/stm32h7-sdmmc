//! ## This crate has been replaced by [SDMMC support within
//! stm32h7xx-hal](https://docs.rs/stm32h7xx-hal/latest/stm32h7xx_hal/sdmmc/index.html).
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

#[macro_use]
mod macros;

mod sd_registers;
pub use sd_registers::{SDStatus, CID, CSD, OCR, SCR};

mod sdmmc;
pub use sdmmc::{BusWidth, Card, CardType, Error, Sdmmc, SdmmcExt, Signalling};
