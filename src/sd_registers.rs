//! SD card register representations

use core::fmt;
use core::str;

#[derive(Debug, Copy, Clone)]
pub enum CardVersion {
    V1_0,
    V1_1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
pub enum BlockSize {
    B512 = 9,
    B1024 = 10,
    B2048 = 11,
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum CurrentConsumption {
    I_0mA,
    I_1mA,
    I_5mA,
    I_10mA,
    I_25mA,
    I_35mA,
    I_45mA,
    I_60mA,
    I_80mA,
    I_100mA,
    I_200mA,
}
impl From<&CurrentConsumption> for u32 {
    fn from(i: &CurrentConsumption) -> u32 {
        match i {
            CurrentConsumption::I_0mA => 0,
            CurrentConsumption::I_1mA => 1,
            CurrentConsumption::I_5mA => 5,
            CurrentConsumption::I_10mA => 10,
            CurrentConsumption::I_25mA => 25,
            CurrentConsumption::I_35mA => 35,
            CurrentConsumption::I_45mA => 45,
            CurrentConsumption::I_60mA => 60,
            CurrentConsumption::I_80mA => 80,
            CurrentConsumption::I_100mA => 100,
            CurrentConsumption::I_200mA => 200,
        }
    }
}
impl CurrentConsumption {
    fn from_minimum_reg(reg: u128) -> CurrentConsumption {
        match reg {
            0 => CurrentConsumption::I_0mA,
            1 => CurrentConsumption::I_1mA,
            2 => CurrentConsumption::I_5mA,
            3 => CurrentConsumption::I_10mA,
            4 => CurrentConsumption::I_25mA,
            5 => CurrentConsumption::I_35mA,
            6 => CurrentConsumption::I_60mA,
            _ => CurrentConsumption::I_100mA,
        }
    }
    fn from_maximum_reg(reg: u128) -> CurrentConsumption {
        match reg {
            0 => CurrentConsumption::I_0mA,
            1 => CurrentConsumption::I_5mA,
            2 => CurrentConsumption::I_10mA,
            3 => CurrentConsumption::I_25mA,
            4 => CurrentConsumption::I_35mA,
            5 => CurrentConsumption::I_45mA,
            6 => CurrentConsumption::I_80mA,
            _ => CurrentConsumption::I_200mA,
        }
    }
}
impl fmt::Debug for CurrentConsumption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ma: u32 = self.into();
        write!(f, "{} mA", ma)
    }
}

/// Operation Conditions Register (OCR)
#[derive(Clone, Copy, Default)]
pub struct OCR(pub u32);
impl OCR {
    /// VDD voltage window
    pub fn voltage_window_mv(&self) -> Option<(u16, u16)> {
        let mut window = (self.0 >> 15) & 0x1FF;
        let mut min = 2_700;

        while window & 1 == 0 && window != 0 {
            min += 100;
            window >>= 1;
        }
        let mut max = min;
        while window != 0 {
            max += 100;
            window >>= 1;
        }

        if max == min {
            None
        } else {
            Some((min, max))
        }
    }
    /// Switching to 1.8V Accepted (S18A). Only UHS-I cards support this bit
    pub fn s18a(&self) -> bool {
        self.0 & 0x0100_0000 != 0
    }
    /// Indicates whether the card supports UHS-II Interface
    pub fn uhs_ii(&self) -> bool {
        self.0 & 0x2000_0000 != 0
    }
    /// Card Capacity Status (CCS). True for SDHC/SDXC/SDUC
    pub fn ccs(&self) -> bool {
        self.0 & 0x4000_0000 != 0
    }
    /// Card power up status bit (busy)
    pub fn is_busy(&self) -> bool {
        self.0 & 0x8000_0000 == 0 // Set active LOW
    }
}
impl fmt::Debug for OCR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OCR: Operation Conditions Register")
            .field(
                "Voltage Window (mV)",
                &self.voltage_window_mv().unwrap_or((0, 0)),
            )
            .field("S18A (UHS-I only)", &self.s18a())
            .field("UHS-II Card", &self.uhs_ii())
            .field("CSS", &if self.ccs() { "SDHC/SDXC/SDUC" } else { "SDSC" })
            .field("Busy", &self.is_busy())
            .finish()
    }
}
/// Card Identification Register (CID)
#[derive(Clone, Copy, Default)]
pub struct CID {
    inner: u128,
    bytes: [u8; 16],
}
impl CID {
    /// A new CID from 128 bits
    pub fn new(inner: u128) -> Self {
        Self {
            inner,
            bytes: inner.to_be_bytes(),
        }
    }
    /// Manufacturer ID
    pub fn manufacturer_id(&self) -> u8 {
        self.bytes[0]
    }
    /// OEM/Application ID
    pub fn oem_id(&self) -> &str {
        str::from_utf8(&self.bytes[1..3]).unwrap_or(&"<ERR>")
    }
    /// Product name
    pub fn product_name(&self) -> &str {
        str::from_utf8(&self.bytes[3..8]).unwrap_or(&"<ERR>")
    }
    /// Product revision
    pub fn product_revision(&self) -> u8 {
        self.bytes[8]
    }
    /// Product serial number
    pub fn product_serial_number(&self) -> u32 {
        (self.inner >> 24) as u32
    }
    /// Manufacturing date
    pub fn manufacturing_date(&self) -> (u8, u16) {
        (
            (self.inner >> 8) as u8 & 0xF,             // Month
            ((self.inner >> 12) as u16 & 0xFF) + 2000, // Year
        )
    }
}
impl fmt::Debug for CID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CID: Card Identification")
            .field("Manufacturer ID", &self.manufacturer_id())
            .field("OEM ID", &self.oem_id())
            .field("Product Name", &self.product_name())
            .field("Product Revision", &self.product_revision())
            .field("Product Serial Number", &self.product_serial_number())
            .field("Manufacturing Date", &self.manufacturing_date())
            .finish()
    }
}
/// Card Specific Data (CSD)
#[derive(Clone, Copy, Default)]
pub struct CSD(pub u128);
impl CSD {
    /// CSD structure version
    fn csd_version(&self) -> u8 {
        (self.0 >> 126) as u8 & 3
    }
    /// Maximum data transfer rate per one data line
    pub fn tranfer_rate(&self) -> u8 {
        (self.0 >> 96) as u8
    }
    /// Maximum block length. In an SD Memory Card the WRITE_BL_LEN is
    /// always equal to READ_BL_LEN
    pub fn block_length(&self) -> Option<BlockSize> {
        // Read block length
        match self.0 >> 80 {
            9 => Some(BlockSize::B512),
            10 => Some(BlockSize::B1024),
            11 => Some(BlockSize::B2048),
            _ => None,
        }
    }
    /// Number of blocks in the card
    pub fn block_count(&self) -> u32 {
        match self.csd_version() {
            0 => {
                // SDSC
                let c_size: u16 = ((self.0 >> 62) as u16) & 0xFFF;
                let c_size_mult: u8 = ((self.0 >> 47) as u8) & 7;

                ((c_size + 1) as u32) * ((1 << (c_size_mult + 2)) as u32)
            }
            1 => {
                // SDHC / SDXC
                ((self.0 >> 48) as u32 & 0x3F_FFFF) + 1
            }
            2 => {
                // SDUC
                ((self.0 >> 48) as u32 & 0xFFF_FFFF) + 1
            }
            _ => 0,
        }
    }
    /// Maximum read current at the minimum VDD
    pub fn read_current_minimum_vdd(&self) -> CurrentConsumption {
        CurrentConsumption::from_minimum_reg(self.0 >> 59)
    }
    /// Maximum write current at the minimum VDD
    pub fn write_current_minimum_vdd(&self) -> CurrentConsumption {
        CurrentConsumption::from_minimum_reg(self.0 >> 56)
    }
    /// Maximum read current at the maximum VDD
    pub fn read_current_maximum_vdd(&self) -> CurrentConsumption {
        CurrentConsumption::from_maximum_reg(self.0 >> 53)
    }
    /// Maximum write current at the maximum VDD
    pub fn write_current_maximum_vdd(&self) -> CurrentConsumption {
        CurrentConsumption::from_maximum_reg(self.0 >> 50)
    }
    /// Erase size (in blocks)
    pub fn erase_size_blocks(&self) -> u32 {
        if (self.0 >> 46) & 1 == 1 {
            // ERASE_BLK_EN
            1
        } else {
            let sector_size_tens = (self.0 >> 43) & 0x7;
            let sector_size_units = (self.0 >> 39) & 0xF;

            (sector_size_tens as u32 * 10) + (sector_size_units as u32)
        }
    }
}
impl fmt::Debug for CSD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CSD: Card Specific Data")
            .field("Tranfer Rate", &self.tranfer_rate())
            .field("Block Count", &self.block_count())
            .field("Read I (@min VDD)", &self.read_current_minimum_vdd())
            .field("Write I (@min VDD)", &self.write_current_minimum_vdd())
            .field("Read I (@max VDD)", &self.read_current_maximum_vdd())
            .field("Write I (@max VDD)", &self.write_current_maximum_vdd())
            .field("Erase Size (Blocks)", &self.erase_size_blocks())
            .finish()
    }
}
/// SD CARD Configuration Register (SCR)
#[derive(Clone, Copy, Default)]
pub struct SCR(pub u64);
impl SCR {
    /// Physical Layer Specification Version Number
    pub fn version(&self) -> CardVersion {
        let spec = (self.0 >> 56) & 0xF;
        let spec3 = (self.0 >> 47) & 1;
        let spec4 = (self.0 >> 42) & 1;
        let specx = (self.0 >> 38) & 0xF;

        // Ref PLSS_v7_10 Table 5-17
        match (spec, spec3, spec4, specx) {
            (0, 0, 0, 0) => CardVersion::V1_0,
            (1, 0, 0, 0) => CardVersion::V1_1,
            (2, 0, 0, 0) => CardVersion::V2,
            (2, 1, 0, 0) => CardVersion::V3,
            (2, 1, 1, 0) => CardVersion::V4,
            (2, 1, _, 1) => CardVersion::V5,
            (2, 1, _, 2) => CardVersion::V6,
            (2, 1, _, 3) => CardVersion::V7,
            _ => CardVersion::Unknown,
        }
    }
    /// Supports 1-bit bus width
    pub fn bus_width_one(&self) -> bool {
        (self.0 >> 48) & 1 != 0
    }
    /// Supports 4-bit bus width
    pub fn bus_width_four(&self) -> bool {
        (self.0 >> 50) & 1 != 0
    }
}
impl fmt::Debug for SCR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SCR: SD CARD Configuration Register")
            .field("Version", &self.version())
            .field("1-bit width", &self.bus_width_one())
            .field("4-bit width", &self.bus_width_four())
            .finish()
    }
}
/// SD Status
#[derive(Clone, Copy, Default)]
pub struct SDStatus {
    inner: [u32; 16],
}
impl SDStatus {
    /// A new SD Status from a slice (512 bits)
    pub fn new(inner: [u32; 16]) -> Self {
        SDStatus { inner }
    }
    /// SDHC / SDXC: Capacity of Protected Area in bytes
    pub fn size_of_protected_area(&self) -> u32 {
        u32::from_be(self.inner[1])
    }
    /// Speed Class
    pub fn speed_class(&self) -> u8 {
        self.inner[2] as u8
    }
    /// "Performance Move" indicator in 1 MB/s units
    pub fn move_performance(&self) -> u8 {
        (self.inner[2] >> 8) as u8
    }
    /// Allocation Unit (AU) size. Lookup in PLSS v7_10 Table 4-47
    pub fn allocation_unit_size(&self) -> u8 {
        (self.inner[3] >> 20) as u8 & 0xF
    }
    /// Indicates N_Erase, in units of AU
    pub fn erase_size(&self) -> u16 {
        ((self.inner[2] & 0xFF00_0000) >> 16) as u16
            | (self.inner[3] & 0xFF) as u16
    }
    /// Indicates T_Erase
    pub fn erase_timeout(&self) -> u8 {
        (self.inner[3] >> 10) as u8 & 0x3F
    }
}
impl fmt::Debug for SDStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SD Status")
            .field("Protected Area Size (B)", &self.size_of_protected_area())
            .field("Speed Class", &self.speed_class())
            .field("Move Performance (MB/s)", &self.move_performance())
            .field("AU Size", &self.allocation_unit_size())
            .field("Erase Size (AU)", &self.erase_size())
            .field("Erase Timeout (s)", &self.erase_timeout())
            .finish()
    }
}
