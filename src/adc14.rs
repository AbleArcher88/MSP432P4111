#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control 0 Register"]
    pub adc14ctl0: ADC14CTL0,
    #[doc = "0x04 - Control 1 Register"]
    pub adc14ctl1: ADC14CTL1,
    #[doc = "0x08 - Window Comparator Low Threshold 0 Register"]
    pub adc14lo0: ADC14LO0,
    #[doc = "0x0c - Window Comparator High Threshold 0 Register"]
    pub adc14hi0: ADC14HI0,
    #[doc = "0x10 - Window Comparator Low Threshold 1 Register"]
    pub adc14lo1: ADC14LO1,
    #[doc = "0x14 - Window Comparator High Threshold 1 Register"]
    pub adc14hi1: ADC14HI1,
    #[doc = "0x18..0x98 - Conversion Memory Control Register"]
    pub adc14mctl: [ADC14MCTL; 32],
    #[doc = "0x98..0x118 - Conversion Memory Register"]
    pub adc14mem: [ADC14MEM; 32],
    _reserved8: [u8; 0x24],
    #[doc = "0x13c - Interrupt Enable 0 Register"]
    pub adc14ier0: ADC14IER0,
    #[doc = "0x140 - Interrupt Enable 1 Register"]
    pub adc14ier1: ADC14IER1,
    #[doc = "0x144 - Interrupt Flag 0 Register"]
    pub adc14ifgr0: ADC14IFGR0,
    #[doc = "0x148 - Interrupt Flag 1 Register"]
    pub adc14ifgr1: ADC14IFGR1,
    #[doc = "0x14c - Clear Interrupt Flag 0 Register"]
    pub adc14clrifgr0: ADC14CLRIFGR0,
    #[doc = "0x150 - Clear Interrupt Flag 1 Register"]
    pub adc14clrifgr1: ADC14CLRIFGR1,
    #[doc = "0x154 - Interrupt Vector Register"]
    pub adc14iv: ADC14IV,
}
#[doc = "ADC14CTL0 (rw) register accessor: an alias for `Reg<ADC14CTL0_SPEC>`"]
pub type ADC14CTL0 = crate::Reg<adc14ctl0::ADC14CTL0_SPEC>;
#[doc = "Control 0 Register"]
pub mod adc14ctl0;
#[doc = "ADC14CTL1 (rw) register accessor: an alias for `Reg<ADC14CTL1_SPEC>`"]
pub type ADC14CTL1 = crate::Reg<adc14ctl1::ADC14CTL1_SPEC>;
#[doc = "Control 1 Register"]
pub mod adc14ctl1;
#[doc = "ADC14LO0 (rw) register accessor: an alias for `Reg<ADC14LO0_SPEC>`"]
pub type ADC14LO0 = crate::Reg<adc14lo0::ADC14LO0_SPEC>;
#[doc = "Window Comparator Low Threshold 0 Register"]
pub mod adc14lo0;
#[doc = "ADC14HI0 (rw) register accessor: an alias for `Reg<ADC14HI0_SPEC>`"]
pub type ADC14HI0 = crate::Reg<adc14hi0::ADC14HI0_SPEC>;
#[doc = "Window Comparator High Threshold 0 Register"]
pub mod adc14hi0;
#[doc = "ADC14LO1 (rw) register accessor: an alias for `Reg<ADC14LO1_SPEC>`"]
pub type ADC14LO1 = crate::Reg<adc14lo1::ADC14LO1_SPEC>;
#[doc = "Window Comparator Low Threshold 1 Register"]
pub mod adc14lo1;
#[doc = "ADC14HI1 (rw) register accessor: an alias for `Reg<ADC14HI1_SPEC>`"]
pub type ADC14HI1 = crate::Reg<adc14hi1::ADC14HI1_SPEC>;
#[doc = "Window Comparator High Threshold 1 Register"]
pub mod adc14hi1;
#[doc = "ADC14MCTL (rw) register accessor: an alias for `Reg<ADC14MCTL_SPEC>`"]
pub type ADC14MCTL = crate::Reg<adc14mctl::ADC14MCTL_SPEC>;
#[doc = "Conversion Memory Control Register"]
pub mod adc14mctl;
#[doc = "ADC14MEM (rw) register accessor: an alias for `Reg<ADC14MEM_SPEC>`"]
pub type ADC14MEM = crate::Reg<adc14mem::ADC14MEM_SPEC>;
#[doc = "Conversion Memory Register"]
pub mod adc14mem;
#[doc = "ADC14IER0 (rw) register accessor: an alias for `Reg<ADC14IER0_SPEC>`"]
pub type ADC14IER0 = crate::Reg<adc14ier0::ADC14IER0_SPEC>;
#[doc = "Interrupt Enable 0 Register"]
pub mod adc14ier0;
#[doc = "ADC14IER1 (rw) register accessor: an alias for `Reg<ADC14IER1_SPEC>`"]
pub type ADC14IER1 = crate::Reg<adc14ier1::ADC14IER1_SPEC>;
#[doc = "Interrupt Enable 1 Register"]
pub mod adc14ier1;
#[doc = "ADC14IFGR0 (r) register accessor: an alias for `Reg<ADC14IFGR0_SPEC>`"]
pub type ADC14IFGR0 = crate::Reg<adc14ifgr0::ADC14IFGR0_SPEC>;
#[doc = "Interrupt Flag 0 Register"]
pub mod adc14ifgr0;
#[doc = "ADC14IFGR1 (r) register accessor: an alias for `Reg<ADC14IFGR1_SPEC>`"]
pub type ADC14IFGR1 = crate::Reg<adc14ifgr1::ADC14IFGR1_SPEC>;
#[doc = "Interrupt Flag 1 Register"]
pub mod adc14ifgr1;
#[doc = "ADC14CLRIFGR0 (w) register accessor: an alias for `Reg<ADC14CLRIFGR0_SPEC>`"]
pub type ADC14CLRIFGR0 = crate::Reg<adc14clrifgr0::ADC14CLRIFGR0_SPEC>;
#[doc = "Clear Interrupt Flag 0 Register"]
pub mod adc14clrifgr0;
#[doc = "ADC14CLRIFGR1 (rw) register accessor: an alias for `Reg<ADC14CLRIFGR1_SPEC>`"]
pub type ADC14CLRIFGR1 = crate::Reg<adc14clrifgr1::ADC14CLRIFGR1_SPEC>;
#[doc = "Clear Interrupt Flag 1 Register"]
pub mod adc14clrifgr1;
#[doc = "ADC14IV (rw) register accessor: an alias for `Reg<ADC14IV_SPEC>`"]
pub type ADC14IV = crate::Reg<adc14iv::ADC14IV_SPEC>;
#[doc = "Interrupt Vector Register"]
pub mod adc14iv;
