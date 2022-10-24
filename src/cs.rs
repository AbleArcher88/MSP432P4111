#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Key Register"]
    pub cskey: CSKEY,
    #[doc = "0x04 - Control 0 Register"]
    pub csctl0: CSCTL0,
    #[doc = "0x08 - Control 1 Register"]
    pub csctl1: CSCTL1,
    #[doc = "0x0c - Control 2 Register"]
    pub csctl2: CSCTL2,
    #[doc = "0x10 - Control 3 Register"]
    pub csctl3: CSCTL3,
    _reserved5: [u8; 0x1c],
    #[doc = "0x30 - Clock Enable Register"]
    pub csclken: CSCLKEN,
    #[doc = "0x34 - Status Register"]
    pub csstat: CSSTAT,
    _reserved7: [u8; 0x08],
    #[doc = "0x40 - Interrupt Enable Register"]
    pub csie: CSIE,
    _reserved8: [u8; 0x04],
    #[doc = "0x48 - Interrupt Flag Register"]
    pub csifg: CSIFG,
    _reserved9: [u8; 0x04],
    #[doc = "0x50 - Clear Interrupt Flag Register"]
    pub csclrifg: CSCLRIFG,
    _reserved10: [u8; 0x04],
    #[doc = "0x58 - Set Interrupt Flag Register"]
    pub cssetifg: CSSETIFG,
    _reserved11: [u8; 0x04],
    #[doc = "0x60 - DCO External Resistor Cailbration 0 Register"]
    pub csdcoercal0: CSDCOERCAL0,
    #[doc = "0x64 - DCO External Resistor Calibration 1 Register"]
    pub csdcoercal1: CSDCOERCAL1,
}
#[doc = "CSKEY (rw) register accessor: an alias for `Reg<CSKEY_SPEC>`"]
pub type CSKEY = crate::Reg<cskey::CSKEY_SPEC>;
#[doc = "Key Register"]
pub mod cskey;
#[doc = "CSCTL0 (rw) register accessor: an alias for `Reg<CSCTL0_SPEC>`"]
pub type CSCTL0 = crate::Reg<csctl0::CSCTL0_SPEC>;
#[doc = "Control 0 Register"]
pub mod csctl0;
#[doc = "CSCTL1 (rw) register accessor: an alias for `Reg<CSCTL1_SPEC>`"]
pub type CSCTL1 = crate::Reg<csctl1::CSCTL1_SPEC>;
#[doc = "Control 1 Register"]
pub mod csctl1;
#[doc = "CSCTL2 (rw) register accessor: an alias for `Reg<CSCTL2_SPEC>`"]
pub type CSCTL2 = crate::Reg<csctl2::CSCTL2_SPEC>;
#[doc = "Control 2 Register"]
pub mod csctl2;
#[doc = "CSCTL3 (rw) register accessor: an alias for `Reg<CSCTL3_SPEC>`"]
pub type CSCTL3 = crate::Reg<csctl3::CSCTL3_SPEC>;
#[doc = "Control 3 Register"]
pub mod csctl3;
#[doc = "CSCLKEN (rw) register accessor: an alias for `Reg<CSCLKEN_SPEC>`"]
pub type CSCLKEN = crate::Reg<csclken::CSCLKEN_SPEC>;
#[doc = "Clock Enable Register"]
pub mod csclken;
#[doc = "CSSTAT (r) register accessor: an alias for `Reg<CSSTAT_SPEC>`"]
pub type CSSTAT = crate::Reg<csstat::CSSTAT_SPEC>;
#[doc = "Status Register"]
pub mod csstat;
#[doc = "CSIE (rw) register accessor: an alias for `Reg<CSIE_SPEC>`"]
pub type CSIE = crate::Reg<csie::CSIE_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod csie;
#[doc = "CSIFG (r) register accessor: an alias for `Reg<CSIFG_SPEC>`"]
pub type CSIFG = crate::Reg<csifg::CSIFG_SPEC>;
#[doc = "Interrupt Flag Register"]
pub mod csifg;
#[doc = "CSCLRIFG (w) register accessor: an alias for `Reg<CSCLRIFG_SPEC>`"]
pub type CSCLRIFG = crate::Reg<csclrifg::CSCLRIFG_SPEC>;
#[doc = "Clear Interrupt Flag Register"]
pub mod csclrifg;
#[doc = "CSSETIFG (w) register accessor: an alias for `Reg<CSSETIFG_SPEC>`"]
pub type CSSETIFG = crate::Reg<cssetifg::CSSETIFG_SPEC>;
#[doc = "Set Interrupt Flag Register"]
pub mod cssetifg;
#[doc = "CSDCOERCAL0 (rw) register accessor: an alias for `Reg<CSDCOERCAL0_SPEC>`"]
pub type CSDCOERCAL0 = crate::Reg<csdcoercal0::CSDCOERCAL0_SPEC>;
#[doc = "DCO External Resistor Cailbration 0 Register"]
pub mod csdcoercal0;
#[doc = "CSDCOERCAL1 (rw) register accessor: an alias for `Reg<CSDCOERCAL1_SPEC>`"]
pub type CSDCOERCAL1 = crate::Reg<csdcoercal1::CSDCOERCAL1_SPEC>;
#[doc = "DCO External Resistor Calibration 1 Register"]
pub mod csdcoercal1;
