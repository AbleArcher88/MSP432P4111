#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TimerAx Control Register"]
    pub tax_ctl: TAX_CTL,
    #[doc = "0x02..0x0c - Timer_A Capture/Compare Control Register"]
    pub tax_cctl: [TAX_CCTL; 5],
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - TimerA register"]
    pub tax_r: TAX_R,
    #[doc = "0x12..0x1c - Timer_A Capture/Compare Register"]
    pub tax_ccr: [TAX_CCR; 5],
    _reserved4: [u8; 0x04],
    #[doc = "0x20 - TimerAx Expansion 0 Register"]
    pub tax_ex0: TAX_EX0,
    _reserved5: [u8; 0x0c],
    #[doc = "0x2e - TimerAx Interrupt Vector Register"]
    pub tax_iv: TAX_IV,
}
#[doc = "TAxCTL (rw) register accessor: an alias for `Reg<TAX_CTL_SPEC>`"]
pub type TAX_CTL = crate::Reg<tax_ctl::TAX_CTL_SPEC>;
#[doc = "TimerAx Control Register"]
pub mod tax_ctl;
#[doc = "TAxCCTL (rw) register accessor: an alias for `Reg<TAX_CCTL_SPEC>`"]
pub type TAX_CCTL = crate::Reg<tax_cctl::TAX_CCTL_SPEC>;
#[doc = "Timer_A Capture/Compare Control Register"]
pub mod tax_cctl;
#[doc = "TAxR (rw) register accessor: an alias for `Reg<TAX_R_SPEC>`"]
pub type TAX_R = crate::Reg<tax_r::TAX_R_SPEC>;
#[doc = "TimerA register"]
pub mod tax_r;
#[doc = "TAxCCR (rw) register accessor: an alias for `Reg<TAX_CCR_SPEC>`"]
pub type TAX_CCR = crate::Reg<tax_ccr::TAX_CCR_SPEC>;
#[doc = "Timer_A Capture/Compare Register"]
pub mod tax_ccr;
#[doc = "TAxEX0 (rw) register accessor: an alias for `Reg<TAX_EX0_SPEC>`"]
pub type TAX_EX0 = crate::Reg<tax_ex0::TAX_EX0_SPEC>;
#[doc = "TimerAx Expansion 0 Register"]
pub mod tax_ex0;
#[doc = "TAxIV (r) register accessor: an alias for `Reg<TAX_IV_SPEC>`"]
pub type TAX_IV = crate::Reg<tax_iv::TAX_IV_SPEC>;
#[doc = "TimerAx Interrupt Vector Register"]
pub mod tax_iv;
