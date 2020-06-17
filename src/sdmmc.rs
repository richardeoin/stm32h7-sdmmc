//! # SD MultiMediaCard interface (SDMMC)
//!
//! For HDHC / SDXC / SDUC cards. SDSC cards are not supported.
//!
//! Adapted from stm32f4xx-hal
//! https://github.com/stm32-rs/stm32f4xx-hal/blob/master/src/sdio.rs

use core::fmt;

use crate::sd_registers::*;

use stm32h7xx_hal::gpio::gpioa::PA0;
use stm32h7xx_hal::gpio::gpiob::{PB14, PB15, PB3, PB4, PB8, PB9};
use stm32h7xx_hal::gpio::gpioc::{PC1, PC10, PC11, PC12, PC6, PC7, PC8, PC9};
use stm32h7xx_hal::gpio::gpiod::{PD2, PD6, PD7};
use stm32h7xx_hal::gpio::gpiog::PG11;
use stm32h7xx_hal::time::Hertz;

use stm32h7xx_hal::gpio::{Alternate, AF10, AF11, AF12, AF9};
//use stm32h7xx_hal::gpio:::{AF7, AF8};
use stm32h7xx_hal::rcc::rec::{ResetEnable, SdmmcClkSelGetter};
use stm32h7xx_hal::rcc::{rec, CoreClocks};
use stm32h7xx_hal::stm32::{SDMMC1, SDMMC2};

pub trait PinClk<SDMMC> {}
pub trait PinCmd<SDMMC> {}
pub trait PinD0<SDMMC> {}
pub trait PinD1<SDMMC> {}
pub trait PinD2<SDMMC> {}
pub trait PinD3<SDMMC> {}
pub trait PinD4<SDMMC> {}
pub trait PinD5<SDMMC> {}
pub trait PinD6<SDMMC> {}
pub trait PinD7<SDMMC> {}

pub trait Pins<SDMMC> {
    const BUSWIDTH: BusWidth;
}

impl<SDMMC, CLK, CMD, D0, D1, D2, D3, D4, D5, D6, D7> Pins<SDMMC>
    for (CLK, CMD, D0, D1, D2, D3, D4, D5, D6, D7)
where
    CLK: PinClk<SDMMC>,
    CMD: PinCmd<SDMMC>,
    D0: PinD0<SDMMC>,
    D1: PinD1<SDMMC>,
    D2: PinD2<SDMMC>,
    D3: PinD3<SDMMC>,
    D4: PinD4<SDMMC>,
    D5: PinD5<SDMMC>,
    D6: PinD6<SDMMC>,
    D7: PinD7<SDMMC>,
{
    const BUSWIDTH: BusWidth = BusWidth::Eight;
}

impl<SDMMC, CLK, CMD, D0, D1, D2, D3> Pins<SDMMC> for (CLK, CMD, D0, D1, D2, D3)
where
    CLK: PinClk<SDMMC>,
    CMD: PinCmd<SDMMC>,
    D0: PinD0<SDMMC>,
    D1: PinD1<SDMMC>,
    D2: PinD2<SDMMC>,
    D3: PinD3<SDMMC>,
{
    const BUSWIDTH: BusWidth = BusWidth::Four;
}

impl<SDMMC, CLK, CMD, D0> Pins<SDMMC> for (CLK, CMD, D0)
where
    CLK: PinClk<SDMMC>,
    CMD: PinCmd<SDMMC>,
    D0: PinD0<SDMMC>,
{
    const BUSWIDTH: BusWidth = BusWidth::One;
}

macro_rules! pins {
    ($($SDMMCX:ty: CLK: [$($CLK:ty),*] CMD: [$($CMD:ty),*]
       D0: [$($D0:ty),*] D1: [$($D1:ty),*] D2: [$($D2:ty),*] D3: [$($D3:ty),*]
       D4: [$($D4:ty),*] D5: [$($D5:ty),*] D6: [$($D6:ty),*] D7: [$($D7:ty),*]
       CKIN: [$($CKIN:ty),*] CDIR: [$($CDIR:ty),*]
       D0DIR: [$($D0DIR:ty),*] D123DIR: [$($D123DIR:ty),*]
    )+) => {
        $(
            $(
                impl PinClk<$SDMMCX> for $CLK {}
            )*
                $(
                    impl PinCmd<$SDMMCX> for $CMD {}
                )*
                $(
                    impl PinD0<$SDMMCX> for $D0 {}
                )*
                $(
                    impl PinD1<$SDMMCX> for $D1 {}
                )*
                $(
                    impl PinD2<$SDMMCX> for $D2 {}
                )*
                $(
                    impl PinD3<$SDMMCX> for $D3 {}
                )*
                $(
                    impl PinD4<$SDMMCX> for $D4 {}
                )*
                $(
                    impl PinD5<$SDMMCX> for $D5 {}
                )*
                $(
                    impl PinD6<$SDMMCX> for $D6 {}
                )*
                $(
                    impl PinD7<$SDMMCX> for $D7 {}
                )*
        )+
    }
}

pins! {
    SDMMC1:
    CLK: [PC12<Alternate<AF12>>]
        CMD: [PD2<Alternate<AF12>>]
        D0: [PC8<Alternate<AF12>>]
        D1: [PC9<Alternate<AF12>>]
        D2: [PC10<Alternate<AF12>>]
        D3: [PC11<Alternate<AF12>>]
        D4: [PB8<Alternate<AF12>>]
        D5: [PB9<Alternate<AF12>>]
        D6: [PC6<Alternate<AF12>>]
        D7: [PC7<Alternate<AF12>>]
        CKIN: [PB8<Alternate<AF7>>]
        CDIR: [PB9<Alternate<AF7>>]
        D0DIR: [PC6<Alternate<AF8>>]
        D123DIR: [PC7<Alternate<AF8>>]
        SDMMC2:
    CLK: [PC1<Alternate<AF9>>, PD6<Alternate<AF11>>]
        CMD: [PA0<Alternate<AF9>>, PD7<Alternate<AF11>>]
        D0: [PB14<Alternate<AF9>>]
        D1: [PB15<Alternate<AF9>>]
        D2: [PB3<Alternate<AF9>>, PG11<Alternate<AF10>>]
        D3: [PB4<Alternate<AF9>>]
        D4: [PB8<Alternate<AF10>>]
        D5: [PB9<Alternate<AF10>>]
        D6: [PC6<Alternate<AF10>>]
        D7: [PC7<Alternate<AF10>>]
        CKIN: []
        CDIR: []
        D0DIR: []
        D123DIR: []
}

/// The number of lines used on the SDMMC bus
#[derive(Copy, Clone, Debug)]
#[allow(missing_docs)]
pub enum BusWidth {
    One = 1,
    Four = 4,
    Eight = 8,
}

/// Types of SD Card
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum CardType {
    /// Standard Capacity (< 2Gb)
    SDSC,
    /// High capacity (< 32Gb)
    SDHC,
}
impl Default for CardType {
    fn default() -> Self {
        CardType::SDSC
    }
}

/// The signalling scheme used on the SDMMC bus
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Signalling {
    SDR12,
    SDR25,
    SDR50,
    SDR104,
    DDR50,
}
impl Default for Signalling {
    fn default() -> Self {
        Signalling::SDR12
    }
}

/// Errors
#[non_exhaustive]
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone)]
pub enum Error {
    Timeout,
    SoftwareTimeout,
    UnsupportedCardVersion,
    UnsupportedCardType,
    Crc,
    DataCrcFail,
    RxOverFlow,
    NoCard,
    BadClock,
    SignalingSwitchFailed,
}

/// A SD command
struct Cmd {
    cmd: u8,
    arg: u32,
    resp: Response,
}

#[derive(Clone, Copy, Debug, Default)]
/// SD Card
pub struct Card {
    /// The type of this card
    pub card_type: CardType,
    /// Operation Conditions Register
    pub ocr: OCR,
    /// Relative Card Address
    pub rca: u32,
    /// Card ID
    pub cid: CID,
    /// Card Specific Data
    pub csd: CSD,
    /// SD CARD Configuration Register
    pub scr: SCR,
    /// SD Status
    pub status: SDStatus,
}
impl Card {
    /// Size in bytes
    pub fn size(&self) -> u64 {
        // SDHC / SDXC / SDUC
        u64::from(self.csd.block_count()) * 512 * 1024
    }
}

macro_rules! err_from_datapath_sm {
    ($status:ident) => {
        if $status.dcrcfail().bit() {
            return Err(Error::DataCrcFail);
        } else if $status.rxoverr().bit() {
            return Err(Error::RxOverFlow);
        } else if $status.dtimeout().bit() {
            return Err(Error::Timeout);
        }
    };
}

/// Indicates transfer direction
enum Dir {
    CardToHost,
    HostToCard,
}

enum PowerCtrl {
    Off = 0b00,
    On = 0b11,
}

#[repr(u32)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
enum CmdAppOper {
    VOLTAGE_WINDOW_SD = 0x8010_0000,
    HIGH_CAPACITY = 0x4000_0000,
    SDMMC_STD_CAPACITY = 0x0000_0000,
    SDMMC_CHECK_PATTERN = 0x0000_01AA,
    SD_SWITCH_1_8V_CAPACITY = 0x0100_0000,
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Response {
    None = 0,
    Short = 1,
    Long = 3,
}

#[derive(Eq, PartialEq, Copy, Clone)]
#[allow(dead_code)]
enum CardStatus {
    /// Card state is ready
    Ready = 1,
    /// Card is in identification state
    Identification = 2,
    /// Card is in standby state
    Standby = 3,
    /// Card is in transfer state
    Transfer = 4,
    /// Card is sending an operation
    Sending = 5,
    /// Card is receiving operation information
    Receiving = 6,
    /// Card is in programming state
    Programming = 7,
    /// Card is disconnected
    Disconnected = 8,
    /// Error
    Error,
}
impl From<u8> for CardStatus {
    fn from(n: u8) -> Self {
        match n {
            1 => Self::Ready,
            2 => Self::Identification,
            3 => Self::Standby,
            4 => Self::Transfer,
            5 => Self::Sending,
            6 => Self::Receiving,
            7 => Self::Programming,
            8 => Self::Disconnected,
            _ => Self::Error,
        }
    }
}

/// Sdmmc device
pub struct Sdmmc<SDMMC> {
    sdmmc: SDMMC,
    /// SDMMC kernel clock
    ker_ck: Hertz,
    /// AHB clock
    hclk: Hertz,
    /// Data bus width
    bus_width: BusWidth,
    /// Current clock to card
    clock: Hertz,
    /// Current signalling scheme to card
    signalling: Signalling,
    /// Card
    card: Option<Card>,
}
impl<SDMMC> fmt::Debug for Sdmmc<SDMMC> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SDMMC Peripheral")
            .field("Card detected", &self.card.is_some())
            .field("Bus Width (bits)", &self.bus_width)
            .field("Signalling", &self.signalling)
            .field("Bus Clock", &self.clock)
            .finish()
    }
}

/// Extension trait for SDMMC peripherals
pub trait SdmmcExt<SDMMC>: Sized {
    /// The `ResetEnable` singleton for this peripheral
    type Rec: ResetEnable;

    /// Create and enable the Sdmmc device. Initially the bus is clocked at
    /// <400kHz, so that SD cards can be initialised.
    fn sdmmc<PINS>(
        self,
        _pins: PINS,
        prec: Self::Rec,
        clocks: &CoreClocks,
    ) -> Sdmmc<SDMMC>
    where
        PINS: Pins<SDMMC>;

    /// Create and enable the Sdmmc device. Initially the bus is clocked
    /// <400kHz, so that SD cards can be initialised. `bus_width` is the bus
    /// width to configure on this interface.
    fn sdmmc_unchecked(
        self,
        bus_width: BusWidth,
        prec: Self::Rec,
        clocks: &CoreClocks,
    ) -> Sdmmc<SDMMC>;
}

impl<S> Sdmmc<S> {
    /// Calculate clock divisor. Returns a SDMMC_CK less than or equal to
    /// `sdmmc_ck` in Hertz.
    ///
    /// Returns `(clk_div, clk_f)`, where `clk_div` is the divisor register
    /// value and `clk_f` is the resulting new clock frequency.
    fn clk_div(ker_ck: Hertz, sdmmc_ck: u32) -> Result<(u16, Hertz), Error> {
        match (ker_ck.0 + sdmmc_ck - 1) / sdmmc_ck {
            0 | 1 => Ok((0, ker_ck)),
            x @ 2..=2046 => {
                let clk_div = ((x + 1) / 2) as u16;
                let clk = Hertz(ker_ck.0 / (clk_div as u32 * 2));

                Ok((clk_div, clk))
            }
            _ => Err(Error::BadClock),
        }
    }
}

macro_rules! sdmmc {
    ($($SDMMCX:ident: ($sdmmcX:ident, $Rec:ident),)+) => {
        $(
            impl SdmmcExt<$SDMMCX> for $SDMMCX {
                type Rec = rec::$Rec;

                fn sdmmc<PINS>(self, _pins: PINS,
                               prec: rec::$Rec,
                               clocks: &CoreClocks) -> Sdmmc<$SDMMCX>
                where
                    PINS: Pins<$SDMMCX>,
                {
                    Sdmmc::$sdmmcX(self, PINS::BUSWIDTH, prec, clocks)
                }

                fn sdmmc_unchecked(self, bus_width: BusWidth,
                                   prec: rec::$Rec,
                                   clocks: &CoreClocks) -> Sdmmc<$SDMMCX>
                {
                    Sdmmc::$sdmmcX(self, bus_width, prec, clocks)
                }
            }

            impl Sdmmc<$SDMMCX> {
                /// Sets the CLKDIV field in CLKCR. Updates clock field in self
                fn clkcr_set_clkdiv(
                    &mut self,
                    freq: u32,
                    width: BusWidth,
                ) -> Result<(), Error> {
                    let (clkdiv, new_clock) = Self::clk_div(self.ker_ck, freq)?;
                    // Enforce AHB and SDMMC_CK clock relation. See RM0433 Rev 7
                    // Section 55.5.8
                    let sdmmc_bus_bandwidth = new_clock.0 * (width as u32);
                    debug_assert!(self.hclk.0 > 3 * sdmmc_bus_bandwidth / 32);
                    self.clock = new_clock;

                    // CPSMACT and DPSMACT must be 0 to set CLKDIV
                    while self.sdmmc.star.read().dpsmact().bit_is_set()
                        || self.sdmmc.star.read().cpsmact().bit_is_set()
                    {}

                    self.sdmmc
                        .clkcr
                        .modify(|_, w| unsafe { w.clkdiv().bits(clkdiv) });

                    Ok(())
                }

                /// Initialise SDMMC peripheral
                pub fn $sdmmcX(
                    sdmmc: $SDMMCX,
                    bus_width: BusWidth,
                    prec: rec::$Rec,
                    clocks: &CoreClocks,
                ) -> Self {
                    // Enable and reset peripheral
                    let prec = prec.enable().reset();

                    let hclk = clocks.hclk();
                    let ker_ck = match prec.get_kernel_clk_mux() {
                        rec::SdmmcClkSel::PLL1_Q => clocks.pll1_q_ck(),
                        rec::SdmmcClkSel::PLL2_R => clocks.pll2_r_ck(),
                    }
                    .expect("sdmmc_ker_ck not running!");

                    // For tuning the phase of the receive sampling clock, a
                    // DLYB block can be connected between sdmmc_io_in_ck and
                    // sdmmc_fb_ck

                    // While the SD/SDIO card or eMMC is in identification mode,
                    // the SDMMC_CK frequency must be less than 400 kHz.
                    let (clkdiv, clock) = Self::clk_div(ker_ck, 400_000)
                        .expect("SDMMC too slow. Cannot be generated from ker_ck");

                    // Configure clock
                    sdmmc.clkcr.write(|w| unsafe {
                        w.widbus()
                            .bits(0) // 1-bit wide bus
                            .clkdiv()
                            .bits(clkdiv)
                            .pwrsav() // power saving
                            .clear_bit()
                            .negedge() // dephasing selection bit
                            .clear_bit()
                            .hwfc_en() // hardware flow control
                            .set_bit()
                    });

                    // Power off, writen 00: Clock to the card is stopped;
                    // D[7:0], CMD, and CK are driven high.
                    sdmmc
                        .power
                        .modify(|_, w| unsafe { w.pwrctrl().bits(PowerCtrl::Off as u8) });

                    Sdmmc {
                        sdmmc,
                        ker_ck,
                        hclk,
                        bus_width,
                        card: None,
                        clock,
                        signalling: Default::default(),
                    }

                    // drop prec: ker_ck can no longer be modified
                }

                /// Initializes card (if present) and sets the bus at the
                /// specified frequency.
                pub fn init_card(&mut self, freq: impl Into<Hertz>) -> Result<(), Error> {
                    let freq = freq.into();

                    // Enable power to card
                    self.sdmmc
                        .power
                        .modify(|_, w| unsafe { w.pwrctrl().bits(PowerCtrl::On as u8) });

                    self.cmd(Cmd::idle())?;

                    // Check if cards supports CMD8 (with pattern)
                    self.cmd(Cmd::hs_send_ext_csd(0x1AA))?;
                    let r1 = self.sdmmc.resp1r.read().bits();

                    let mut card = if r1 == 0x1AA {
                        // Card echoed back the pattern. Must be at least v2
                        Card::default()
                    } else {
                        return Err(Error::UnsupportedCardVersion);
                    };

                    let ocr = loop {
                        // Signal that next command is a app command
                        self.cmd(Cmd::app_cmd(0))?; // CMD55

                        let arg = CmdAppOper::VOLTAGE_WINDOW_SD as u32
                            | CmdAppOper::HIGH_CAPACITY as u32
                            | CmdAppOper::SD_SWITCH_1_8V_CAPACITY as u32;

                        // Initialize card
                        match self.cmd(Cmd::app_op_cmd(arg)) {
                            // ACMD41
                            Ok(_) => (),
                            Err(Error::Crc) => (),
                            Err(err) => return Err(err),
                        }
                        let ocr = OCR(self.sdmmc.resp1r.read().bits());
                        if !ocr.is_busy() {
                            // Power up done
                            break ocr;
                        }
                    };

                    if ocr.ccs() {
                        // Card is SDHC or SDXC or SDUC
                        card.card_type = CardType::SDHC;
                    } else {
                        return Err(Error::UnsupportedCardType);
                    }
                    card.ocr = ocr;

                    // Get CID
                    self.cmd(Cmd::all_send_cid())?; // CMD2
                    let cid = ((self.sdmmc.resp1r.read().bits() as u128) << 96)
                        | ((self.sdmmc.resp2r.read().bits() as u128) << 64)
                        | ((self.sdmmc.resp3r.read().bits() as u128) << 32)
                        | self.sdmmc.resp4r.read().bits() as u128;
                    card.cid = CID::new(cid);

                    // Get RCA
                    self.cmd(Cmd::send_rel_addr())?;
                    card.rca = self.sdmmc.resp1r.read().bits() >> 16;

                    // Get CSD
                    self.cmd(Cmd::send_csd(card.rca << 16))?;
                    let csd = ((self.sdmmc.resp1r.read().bits() as u128) << 96)
                        | ((self.sdmmc.resp2r.read().bits() as u128) << 64)
                        | ((self.sdmmc.resp3r.read().bits() as u128) << 32)
                        | self.sdmmc.resp4r.read().bits() as u128;
                    card.csd = CSD(csd);

                    self.select_card(Some(&card))?;

                    self.get_scr(&mut card)?;

                    // Set bus width
                    let (width, acmd_arg) = match self.bus_width {
                        BusWidth::Eight => unimplemented!(),
                        BusWidth::Four if card.scr.bus_width_four() => (BusWidth::Four, 2),
                        _ => (BusWidth::One, 0),
                    };
                    self.cmd(Cmd::app_cmd(card.rca << 16))?;
                    self.cmd(Cmd::cmd6(acmd_arg))?; // ACMD6: Bus Width

                    // CPSMACT and DPSMACT must be 0 to set WIDBUS
                    while self.sdmmc.star.read().dpsmact().bit_is_set()
                        || self.sdmmc.star.read().cpsmact().bit_is_set()
                    {}
                    self.sdmmc.clkcr.modify(|_, w| unsafe {
                        w.widbus().bits(match width {
                            BusWidth::One => 0,
                            BusWidth::Four => 1,
                            BusWidth::Eight => 2,
                        })
                    });

                    // Set Clock
                    if freq.0 <= 25_000_000 {
                        // Final clock frequency
                        self.clkcr_set_clkdiv(freq.0, width)?;
                    } else {
                        // Switch to max clock for SDR12
                        self.clkcr_set_clkdiv(25_000_000, width)?;
                    }

                    // Read status
                    let _old = self.card.replace(card);
                    self.read_sd_status()?;

                    if freq.0 > 25_000_000 {
                        // Switch to SDR25
                        self.signalling = self.switch_signalling_mode(Signalling::SDR25)?;

                        if self.signalling == Signalling::SDR25 {
                            self.clkcr_set_clkdiv(freq.0, width)?;

                            if self.send_status()? != CardStatus::Transfer {
                                return Err(Error::SignalingSwitchFailed);
                            }
                        }
                    }

                    Ok(())
                }

                /// Get a reference to the initialized card
                ///
                /// # Errors
                ///
                /// Returns Error::NoCard if [`init_card`](#method.init_card)
                /// has not previously succeeded
                pub fn card(&self) -> Result<&Card, Error> {
                    self.card.as_ref().ok_or(Error::NoCard)
                }

                /// Get the current SDMMC bus clock
                ///
                pub fn clock(&self) -> Hertz {
                    self.clock
                }

                /// Start a transfer
                fn start_datapath_transfer(
                    &self,
                    length_bytes: u32,
                    block_size: u8,
                    direction: Dir,
                ) {
                    // Block Size up to 2^14 bytes
                    assert!(block_size <= 14);

                    // Block Size must be greater than 0 ( != 1 byte) in DDR mode
                    let ddr = self.sdmmc.clkcr.read().ddr().bit_is_set();
                    assert!(
                        !ddr || block_size != 0,
                        "Block size must be >= 1, or >= 2 is DDR mode"
                    );

                    let dtdir = match direction {
                        Dir::CardToHost => true,
                        _ => false,
                    };

                    // Command AND Data state machines must be idle
                    while self.sdmmc.star.read().dpsmact().bit_is_set()
                        || self.sdmmc.star.read().cpsmact().bit_is_set()
                    {}

                    // Data timeout, in bus cycles
                    self.sdmmc
                        .dtimer
                        .write(|w| unsafe { w.datatime().bits(5_000_000) });
                    // Data length, in bytes
                    self.sdmmc
                        .dlenr
                        .write(|w| unsafe { w.datalength().bits(length_bytes) });
                    // Transfer
                    self.sdmmc.dctrl.write(|w| unsafe {
                        w.dblocksize()
                            .bits(block_size) // 2^n bytes block size
                            .dtdir()
                            .bit(dtdir)
                            .dten()
                            .set_bit() // Enable transfer
                    });
                }

                /// Read block from card.
                ///
                /// `address` is the block address.
                pub fn read_block(
                    &mut self,
                    address: u32,
                    buffer: &mut [u8; 512],
                ) -> Result<(), Error> {
                    let _card = self.card()?;

                    self.cmd(Cmd::set_block_length(512))?; // CMD16

                    // Setup read command
                    self.start_datapath_transfer(512, 9, Dir::CardToHost);
                    self.cmd(Cmd::read_single_block(address))?;

                    let mut i = 0;
                    let mut status;
                    while {
                        status = self.sdmmc.star.read();
                        !(status.rxoverr().bit()
                          || status.dcrcfail().bit()
                          || status.dtimeout().bit()
                          || status.dataend().bit())
                    } {
                        if status.rxfifohf().bit() {
                            for _ in 0..8 {
                                let bytes = self.sdmmc.fifor.read().bits().to_le_bytes();
                                buffer[i..i + 4].copy_from_slice(&bytes);
                                i += 4;
                            }
                        }

                        if i >= buffer.len() {
                            break;
                        }
                    }

                    err_from_datapath_sm!(status);

                    Ok(())
                }

                /// Read mutliple blocks from card. The length of the buffer
                /// must be multiple of 512.
                ///
                /// `address` is the block address.
                pub fn read_blocks(
                    &mut self,
                    address: u32,
                    buffer: &mut [u8],
                ) -> Result<(), Error> {
                    let _card = self.card()?;

                    assert!(buffer.len() % 512 == 0);
                    let n_blocks = buffer.len() / 512;
                    self.cmd(Cmd::set_block_length(512))?; // CMD16

                    // Setup read command
                    self.start_datapath_transfer(512 * n_blocks as u32, 9, Dir::CardToHost);
                    self.cmd(Cmd::read_multiple_blocks(address))?;

                    let mut i = 0;
                    let mut status;
                    while {
                        status = self.sdmmc.star.read();
                        !(status.rxoverr().bit()
                          || status.dcrcfail().bit()
                          || status.dtimeout().bit()
                          || status.dataend().bit())
                    } {
                        if status.rxfifohf().bit() {
                            for _ in 0..8 {
                                let bytes = self.sdmmc.fifor.read().bits().to_le_bytes();
                                buffer[i..i + 4].copy_from_slice(&bytes);
                                i += 4;
                            }
                        }

                        if i >= buffer.len() {
                            break;
                        }
                    }

                    self.cmd(Cmd::stop_transmission())?; // CMD12

                    err_from_datapath_sm!(status);

                    Ok(())
                }

                /// Write block to card. Buffer must be 512 bytes
                pub fn write_block(
                    &mut self,
                    address: u32,
                    buffer: &[u8; 512]
                ) -> Result<(), Error> {
                    let _card = self.card()?;

                    self.cmd(Cmd::set_block_length(512))?; // CMD16

                    // Setup write command
                    self.start_datapath_transfer(512, 9, Dir::HostToCard);
                    self.cmd(Cmd::write_single_block(address))?; // CMD24

                    let mut i = 0;
                    let mut status;
                    while {
                        status = self.sdmmc.star.read();
                        !(status.txunderr().bit()
                          || status.dcrcfail().bit()
                          || status.dtimeout().bit()
                          || status.dataend().bit())
                    } {
                        if status.txfifohe().bit() {
                            for _ in 0..8 {
                                let mut wb = [0u8; 4];
                                wb.copy_from_slice(&buffer[i..i + 4]);
                                let word = u32::from_le_bytes(wb);
                                self.sdmmc.fifor.write(|w| unsafe { w.bits(word) });
                                i += 4;
                            }
                        }

                        if i >= buffer.len() {
                            break;
                        }
                    }

                    err_from_datapath_sm!(status);
                    self.clear_static_interrupt_flags();

                    let mut timeout: u32 = 0xFFFF_FFFF;

                    // Try to read card status (ACMD13)
                    while timeout > 0 {
                        match self.read_sd_status() {
                            Ok(_) => return Ok(()),
                            Err(Error::Timeout) => (), // Try again
                            Err(e) => return Err(e),
                        }

                        timeout -= 1;
                    }
                    Err(Error::SoftwareTimeout)
                }

                /// Query the card's status register (CMD13).
                ///
                /// Returns the 'card state' bits
                fn send_status(&self) -> Result<CardStatus, Error> {
                    let card = self.card()?;

                    // SEND_STATUS
                    self.cmd(Cmd::card_status(card.rca << 16))?; // CMD13
                    let r1 = self.sdmmc.resp1r.read().bits();

                    let cardstate = (r1 >> 9) as u8 & 0xF;

                    Ok(cardstate.into())
                }

                /// Reads the SD Status (ACMD13)
                ///
                fn read_sd_status(&mut self) -> Result<(), Error> {
                    let card = self.card()?;

                    self.cmd(Cmd::set_block_length(64))?; // CMD16
                    self.cmd(Cmd::app_cmd(card.rca << 16))?; // APP

                    // Prepare the transfer
                    self.start_datapath_transfer(64, 6, Dir::CardToHost);
                    self.cmd(Cmd::card_status(0))?; // ACMD13

                    let mut status = [0u32; 16];
                    let mut idx = 0;
                    let mut sta_reg;
                    while {
                        sta_reg = self.sdmmc.star.read();
                        !(sta_reg.rxoverr().bit()
                          || sta_reg.dcrcfail().bit()
                          || sta_reg.dtimeout().bit()
                          || sta_reg.dbckend().bit())
                    } {
                        if sta_reg.rxfifohf().bit() {
                            for _ in 0..8 {
                                status[idx] = self.sdmmc.fifor.read().bits();
                                idx += 1;
                            }
                        }

                        if idx == status.len() {
                            break;
                        }
                    }

                    err_from_datapath_sm!(sta_reg);

                    let card = self.card.as_mut().ok_or(Error::NoCard)?;
                    card.status = SDStatus::new(status);

                    Ok(())
                }

                /// Get SD CARD Configuration Register (SCR)
                fn get_scr(&self, card: &mut Card) -> Result<(), Error> {
                    // Read the the 64-bit SCR register
                    self.cmd(Cmd::set_block_length(8))?; // CMD16
                    self.cmd(Cmd::app_cmd(card.rca << 16))?;

                    self.start_datapath_transfer(8, 3, Dir::CardToHost);
                    self.cmd(Cmd::cmd51())?;

                    let mut scr = [0; 2];
                    let mut i = 0;
                    let mut status;
                    while {
                        status = self.sdmmc.star.read();

                        !(status.rxoverr().bit()
                          || status.dcrcfail().bit()
                          || status.dtimeout().bit()
                          || status.dbckend().bit())
                    } {
                        if status.rxfifoe().bit_is_clear() {
                            // FIFO not empty
                            scr[i] = self.sdmmc.fifor.read().bits();
                            i += 1;
                        }

                        if i == 2 {
                            break;
                        }
                    }

                    err_from_datapath_sm!(status);

                    // Bytes from wire are Big Endian
                    let scr = ((scr[1] as u64) << 32) | scr[0] as u64;
                    card.scr = SCR(u64::from_be(scr));

                    Ok(())
                }

                /// Switch mode using CMD6.
                ///
                /// Attempt to set a new signalling mode. The selected
                /// signalling mode is returned. Expects the current clock
                /// frequency to be > 12.5MHz.
                fn switch_signalling_mode(
                    &self,
                    signalling: Signalling,
                ) -> Result<Signalling, Error> {
                    // NB PLSS v7_10 4.3.10.4: "the use of SET_BLK_LEN command is not
                    // necessary"

                    let set_function = 0x8000_0000
                        | match signalling {
                            // See PLSS v7_10 Table 4-11
                            Signalling::DDR50 => 0xFF_FF04,
                            Signalling::SDR104 => 0xFF_1F03,
                            Signalling::SDR50 => 0xFF_1F02,
                            Signalling::SDR25 => 0xFF_FF01,
                            Signalling::SDR12 => 0xFF_FF00,
                        };

                    // Prepare the transfer
                    self.start_datapath_transfer(64, 6, Dir::CardToHost);
                    self.cmd(Cmd::cmd6(set_function))?; // CMD6

                    let mut status = [0u32; 16];
                    let mut idx = 0;
                    let mut sta_reg;
                    while {
                        sta_reg = self.sdmmc.star.read();
                        !(sta_reg.rxoverr().bit()
                          || sta_reg.dcrcfail().bit()
                          || sta_reg.dtimeout().bit()
                          || sta_reg.dbckend().bit())
                    } {
                        if sta_reg.rxfifohf().bit() {
                            for _ in 0..8 {
                                status[idx] = self.sdmmc.fifor.read().bits();
                                idx += 1;
                            }
                        }

                        if idx == status.len() {
                            break;
                        }
                    }

                    err_from_datapath_sm!(sta_reg);

                    // Host is allowed to use the new functions at least 8
                    // clocks after the end of the switch command
                    // transaction. We know the current clock period is < 80ns,
                    // so a total delay of 640ns is required here
                    for _ in 0..300 {
                        cortex_m::asm::nop();
                    }

                    // Support Bits of Functions in Function Group 1
                    let _support_bits = u32::from_be(status[3]) >> 16;
                    // Function Selection of Function Group 1
                    let selection = (u32::from_be(status[4]) >> 24) & 0xF;

                    match selection {
                        0 => Ok(Signalling::SDR12),
                        1 => Ok(Signalling::SDR25),
                        2 => Ok(Signalling::SDR50),
                        3 => Ok(Signalling::SDR104),
                        4 => Ok(Signalling::DDR50),
                        _ => Err(Error::UnsupportedCardType),
                    }
                }

                /// Select one card and place it into the _Tranfer State_
                ///
                /// If `None` is specifed for `card`, all cards are put back into
                /// _Stand-by State_
                fn select_card(&self, card: Option<&Card>) -> Result<(), Error> {
                    // Determine Relative Card Address (RCA) of given card
                    let rca = card.map(|c| c.rca << 16).unwrap_or(0);

                    let r = self.cmd(Cmd::sel_desel_card(rca));
                    match (r, rca) {
                        (Err(Error::Timeout), 0) => Ok(()),
                        _ => r,
                    }
                }

                /// Clear "static flags" in interrupt clear register
                fn clear_static_interrupt_flags(&self) {
                    self.sdmmc.icr.modify(|_, w| {
                        w.dcrcfailc()
                            .set_bit()
                            .dtimeoutc()
                            .set_bit()
                            .txunderrc()
                            .set_bit()
                            .rxoverrc()
                            .set_bit()
                            .dataendc()
                            .set_bit()
                            .dholdc()
                            .set_bit()
                            .dbckendc()
                            .set_bit()
                            .dabortc()
                            .set_bit()
                            .idmatec()
                            .set_bit()
                            .idmabtcc()
                            .set_bit()
                    });
                }

                /// Send command to card
                fn cmd(&self, cmd: Cmd) -> Result<(), Error> {
                    // Clear interrupts
                    self.sdmmc.icr.modify(|_, w| {
                        w.ccrcfailc() // CRC FAIL
                            .set_bit()
                            .ctimeoutc() // TIMEOUT
                            .set_bit()
                            .cmdrendc() // CMD R END
                            .set_bit()
                            .cmdsentc() // CMD SENT
                            .set_bit()
                            .dataendc()
                            .set_bit()
                            .dbckendc()
                            .set_bit()
                            .dcrcfailc()
                            .set_bit()
                            .dtimeoutc()
                            .set_bit()
                            .sdioitc() // SDIO IT
                            .set_bit()
                            .rxoverrc()
                            .set_bit()
                            .txunderrc()
                            .set_bit()
                    });

                    // CP state machine must be idle
                    while self.sdmmc.star.read().cpsmact().bit_is_set() {}

                    // Command arg
                    self.sdmmc
                        .argr
                        .write(|w| unsafe { w.cmdarg().bits(cmd.arg) });

                    // Special mode in CP State Machine
                    // CMD12: Stop Transmission
                    let cpsm_stop_transmission = (cmd.cmd == 12);

                    // Command index and start CP State Machine
                    self.sdmmc.cmdr.write(|w| unsafe {
                        w.waitint()
                            .clear_bit()
                            .waitresp() // No / Short / Long
                            .bits(cmd.resp as u8)
                            .cmdstop() // CPSM Stop Transmission
                            .bit(cpsm_stop_transmission)
                            .cmdindex()
                            .bits(cmd.cmd)
                            .cpsmen()
                            .set_bit()
                    });

                    let mut timeout: u32 = 0xFFFF_FFFF;

                    let mut status;
                    if cmd.resp == Response::None {
                        // Wait for CMDSENT or a timeout
                        while {
                            status = self.sdmmc.star.read();
                            !(status.ctimeout().bit() || status.cmdsent().bit())
                                && timeout > 0
                        } {
                            timeout -= 1;
                        }
                    } else {
                        // Wait for CMDREND or CCRCFAIL or a timeout
                        while {
                            status = self.sdmmc.star.read();
                            !(status.ctimeout().bit()
                              || status.cmdrend().bit()
                              || status.ccrcfail().bit())
                                && timeout > 0
                        } {
                            timeout -= 1;
                        }
                    }

                    if status.ctimeout().bit_is_set() {
                        return Err(Error::Timeout);
                    } else if timeout == 0 {
                        return Err(Error::SoftwareTimeout);
                    } else if status.ccrcfail().bit() {
                        return Err(Error::Crc);
                    }

                    Ok(())
                }

            }
        )+
    };
}

sdmmc! {
    SDMMC1: (sdmmc1, Sdmmc1),
    SDMMC2: (sdmmc2, Sdmmc2),
}

/// SD card Commands
impl Cmd {
    const fn new(cmd: u8, arg: u32, resp: Response) -> Cmd {
        Cmd { cmd, arg, resp }
    }

    /// CMD0: Idle
    const fn idle() -> Cmd {
        Cmd::new(0, 0, Response::None)
    }

    /// CMD2: Send CID
    const fn all_send_cid() -> Cmd {
        Cmd::new(2, 0, Response::Long)
    }

    /// CMD3: Send Relative Address
    const fn send_rel_addr() -> Cmd {
        Cmd::new(3, 0, Response::Short)
    }

    /// CMD6: Switch Function Command
    /// ACMD6: Bus Width
    const fn cmd6(arg: u32) -> Cmd {
        Cmd::new(6, arg, Response::Short)
    }

    /// CMD7: Select one card and put it into the _Tranfer State_
    const fn sel_desel_card(rca: u32) -> Cmd {
        Cmd::new(7, rca, Response::Short)
    }

    /// CMD8:
    const fn hs_send_ext_csd(arg: u32) -> Cmd {
        Cmd::new(8, arg, Response::Short)
    }

    /// CMD9:
    const fn send_csd(rca: u32) -> Cmd {
        Cmd::new(9, rca, Response::Long)
    }

    /// CMD12:
    const fn stop_transmission() -> Cmd {
        Cmd::new(12, 0, Response::Short)
    }

    /// CMD13: Ask card to send status register
    /// ACMD13: SD Status
    const fn card_status(rca: u32) -> Cmd {
        Cmd::new(13, rca, Response::Short)
    }

    /// CMD16:
    const fn set_block_length(blocklen: u32) -> Cmd {
        Cmd::new(16, blocklen, Response::Short)
    }

    /// CMD17: Block Read
    const fn read_single_block(addr: u32) -> Cmd {
        Cmd::new(17, addr, Response::Short)
    }

    /// CMD18: Multiple Block Read
    const fn read_multiple_blocks(addr: u32) -> Cmd {
        Cmd::new(18, addr, Response::Short)
    }

    /// CMD24: Block Write
    const fn write_single_block(addr: u32) -> Cmd {
        Cmd::new(24, addr, Response::Short)
    }

    const fn app_op_cmd(arg: u32) -> Cmd {
        Cmd::new(41, arg, Response::Short)
    }

    const fn cmd51() -> Cmd {
        Cmd::new(51, 0, Response::Short)
    }

    /// App Command. Indicates that next command will be a app command
    const fn app_cmd(rca: u32) -> Cmd {
        Cmd::new(55, rca, Response::Short)
    }
}
