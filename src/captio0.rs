#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0e],
    #[doc = "0x0e - Capacitive Touch IO x Control Register"]
    pub captiox_ctl: CAPTIOX_CTL,
}
#[doc = "CAPTIOxCTL (rw) register accessor: an alias for `Reg<CAPTIOX_CTL_SPEC>`"]
pub type CAPTIOX_CTL = crate::Reg<captiox_ctl::CAPTIOX_CTL_SPEC>;
#[doc = "Capacitive Touch IO x Control Register"]
pub mod captiox_ctl;
