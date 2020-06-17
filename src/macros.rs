#[cfg(feature = "log")]
#[macro_use]
mod log {
    macro_rules! sdmmc_log {
        (trace, $($arg:expr),*) => { trace!($($arg),*); };
    }
}

#[cfg(not(feature = "log"))]
#[macro_use]
mod log {
    macro_rules! sdmmc_log {
        ($level:ident, $($arg:expr),*) => { $( let _ = $arg; )* }
    }
}

macro_rules! sdmmc_trace {
    ($($arg:expr),*) => (sdmmc_log!(trace, $($arg),*));
}
