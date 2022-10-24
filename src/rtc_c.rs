#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTCCTL0 Register"]
    pub rtcctl0: RTCCTL0,
    #[doc = "0x02 - RTCCTL13 Register"]
    pub rtcctl13: RTCCTL13,
    #[doc = "0x04 - RTCOCAL Register"]
    pub rtcocal: RTCOCAL,
    #[doc = "0x06 - RTCTCMP Register"]
    pub rtctcmp: RTCTCMP,
    #[doc = "0x08 - Real-Time Clock Prescale Timer 0 Control Register"]
    pub rtcps0ctl: RTCPS0CTL,
    #[doc = "0x0a - Real-Time Clock Prescale Timer 1 Control Register"]
    pub rtcps1ctl: RTCPS1CTL,
    #[doc = "0x0c - Real-Time Clock Prescale Timer Counter Register"]
    pub rtcps: RTCPS,
    #[doc = "0x0e - Real-Time Clock Interrupt Vector Register"]
    pub rtciv: RTCIV,
    #[doc = "0x10 - RTCTIM0 Register Hexadecimal Format"]
    pub rtctim0: RTCTIM0,
    #[doc = "0x12 - Real-Time Clock Hour, Day of Week"]
    pub rtctim1: RTCTIM1,
    #[doc = "0x14 - RTCDATE - Hexadecimal Format"]
    pub rtcdate: RTCDATE,
    #[doc = "0x16 - RTCYEAR Register Hexadecimal Format"]
    pub rtcyear: RTCYEAR,
    #[doc = "0x18 - RTCMINHR - Hexadecimal Format"]
    pub rtcaminhr: RTCAMINHR,
    #[doc = "0x1a - RTCADOWDAY - Hexadecimal Format"]
    pub rtcadowday: RTCADOWDAY,
    #[doc = "0x1c - Binary-to-BCD Conversion Register"]
    pub rtcbin2bcd: RTCBIN2BCD,
    #[doc = "0x1e - BCD-to-Binary Conversion Register"]
    pub rtcbcd2bin: RTCBCD2BIN,
}
#[doc = "RTCCTL0 (rw) register accessor: an alias for `Reg<RTCCTL0_SPEC>`"]
pub type RTCCTL0 = crate::Reg<rtcctl0::RTCCTL0_SPEC>;
#[doc = "RTCCTL0 Register"]
pub mod rtcctl0;
#[doc = "RTCCTL13 (rw) register accessor: an alias for `Reg<RTCCTL13_SPEC>`"]
pub type RTCCTL13 = crate::Reg<rtcctl13::RTCCTL13_SPEC>;
#[doc = "RTCCTL13 Register"]
pub mod rtcctl13;
#[doc = "RTCOCAL (rw) register accessor: an alias for `Reg<RTCOCAL_SPEC>`"]
pub type RTCOCAL = crate::Reg<rtcocal::RTCOCAL_SPEC>;
#[doc = "RTCOCAL Register"]
pub mod rtcocal;
#[doc = "RTCTCMP (rw) register accessor: an alias for `Reg<RTCTCMP_SPEC>`"]
pub type RTCTCMP = crate::Reg<rtctcmp::RTCTCMP_SPEC>;
#[doc = "RTCTCMP Register"]
pub mod rtctcmp;
#[doc = "RTCPS0CTL (rw) register accessor: an alias for `Reg<RTCPS0CTL_SPEC>`"]
pub type RTCPS0CTL = crate::Reg<rtcps0ctl::RTCPS0CTL_SPEC>;
#[doc = "Real-Time Clock Prescale Timer 0 Control Register"]
pub mod rtcps0ctl;
#[doc = "RTCPS1CTL (rw) register accessor: an alias for `Reg<RTCPS1CTL_SPEC>`"]
pub type RTCPS1CTL = crate::Reg<rtcps1ctl::RTCPS1CTL_SPEC>;
#[doc = "Real-Time Clock Prescale Timer 1 Control Register"]
pub mod rtcps1ctl;
#[doc = "RTCPS (rw) register accessor: an alias for `Reg<RTCPS_SPEC>`"]
pub type RTCPS = crate::Reg<rtcps::RTCPS_SPEC>;
#[doc = "Real-Time Clock Prescale Timer Counter Register"]
pub mod rtcps;
#[doc = "RTCIV (r) register accessor: an alias for `Reg<RTCIV_SPEC>`"]
pub type RTCIV = crate::Reg<rtciv::RTCIV_SPEC>;
#[doc = "Real-Time Clock Interrupt Vector Register"]
pub mod rtciv;
#[doc = "RTCTIM0 (rw) register accessor: an alias for `Reg<RTCTIM0_SPEC>`"]
pub type RTCTIM0 = crate::Reg<rtctim0::RTCTIM0_SPEC>;
#[doc = "RTCTIM0 Register Hexadecimal Format"]
pub mod rtctim0;
#[doc = "RTCTIM1 (rw) register accessor: an alias for `Reg<RTCTIM1_SPEC>`"]
pub type RTCTIM1 = crate::Reg<rtctim1::RTCTIM1_SPEC>;
#[doc = "Real-Time Clock Hour, Day of Week"]
pub mod rtctim1;
#[doc = "RTCDATE (rw) register accessor: an alias for `Reg<RTCDATE_SPEC>`"]
pub type RTCDATE = crate::Reg<rtcdate::RTCDATE_SPEC>;
#[doc = "RTCDATE - Hexadecimal Format"]
pub mod rtcdate;
#[doc = "RTCYEAR (rw) register accessor: an alias for `Reg<RTCYEAR_SPEC>`"]
pub type RTCYEAR = crate::Reg<rtcyear::RTCYEAR_SPEC>;
#[doc = "RTCYEAR Register Hexadecimal Format"]
pub mod rtcyear;
#[doc = "RTCAMINHR (rw) register accessor: an alias for `Reg<RTCAMINHR_SPEC>`"]
pub type RTCAMINHR = crate::Reg<rtcaminhr::RTCAMINHR_SPEC>;
#[doc = "RTCMINHR - Hexadecimal Format"]
pub mod rtcaminhr;
#[doc = "RTCADOWDAY (rw) register accessor: an alias for `Reg<RTCADOWDAY_SPEC>`"]
pub type RTCADOWDAY = crate::Reg<rtcadowday::RTCADOWDAY_SPEC>;
#[doc = "RTCADOWDAY - Hexadecimal Format"]
pub mod rtcadowday;
#[doc = "RTCBIN2BCD (rw) register accessor: an alias for `Reg<RTCBIN2BCD_SPEC>`"]
pub type RTCBIN2BCD = crate::Reg<rtcbin2bcd::RTCBIN2BCD_SPEC>;
#[doc = "Binary-to-BCD Conversion Register"]
pub mod rtcbin2bcd;
#[doc = "RTCBCD2BIN (rw) register accessor: an alias for `Reg<RTCBCD2BIN_SPEC>`"]
pub type RTCBCD2BIN = crate::Reg<rtcbcd2bin::RTCBCD2BIN_SPEC>;
#[doc = "BCD-to-Binary Conversion Register"]
pub mod rtcbcd2bin;
