#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0f34],
    #[doc = "0xf34 - Floating Point Context Control Register"]
    pub fpccr: FPCCR,
    #[doc = "0xf38 - Floating-Point Context Address Register"]
    pub fpcar: FPCAR,
    #[doc = "0xf3c - Floating Point Default Status Control Register"]
    pub fpdscr: FPDSCR,
    #[doc = "0xf40 - Media and FP Feature Register 0 (MVFR0)"]
    pub mvfr0: MVFR0,
    #[doc = "0xf44 - Media and FP Feature Register 1 (MVFR1)"]
    pub mvfr1: MVFR1,
}
#[doc = "FPCCR (rw) register accessor: an alias for `Reg<FPCCR_SPEC>`"]
pub type FPCCR = crate::Reg<fpccr::FPCCR_SPEC>;
#[doc = "Floating Point Context Control Register"]
pub mod fpccr;
#[doc = "FPCAR (rw) register accessor: an alias for `Reg<FPCAR_SPEC>`"]
pub type FPCAR = crate::Reg<fpcar::FPCAR_SPEC>;
#[doc = "Floating-Point Context Address Register"]
pub mod fpcar;
#[doc = "FPDSCR (rw) register accessor: an alias for `Reg<FPDSCR_SPEC>`"]
pub type FPDSCR = crate::Reg<fpdscr::FPDSCR_SPEC>;
#[doc = "Floating Point Default Status Control Register"]
pub mod fpdscr;
#[doc = "MVFR0 (r) register accessor: an alias for `Reg<MVFR0_SPEC>`"]
pub type MVFR0 = crate::Reg<mvfr0::MVFR0_SPEC>;
#[doc = "Media and FP Feature Register 0 (MVFR0)"]
pub mod mvfr0;
#[doc = "MVFR1 (r) register accessor: an alias for `Reg<MVFR1_SPEC>`"]
pub type MVFR1 = crate::Reg<mvfr1::MVFR1_SPEC>;
#[doc = "Media and FP Feature Register 1 (MVFR1)"]
pub mod mvfr1;
