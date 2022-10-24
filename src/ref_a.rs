#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - REF Control Register 0"]
    pub refctl0: REFCTL0,
}
#[doc = "REFCTL0 (rw) register accessor: an alias for `Reg<REFCTL0_SPEC>`"]
pub type REFCTL0 = crate::Reg<refctl0::REFCTL0_SPEC>;
#[doc = "REF Control Register 0"]
pub mod refctl0;
