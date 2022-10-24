#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer 1 Load Register"]
    pub t32load1: T32LOAD1,
    #[doc = "0x04 - Timer 1 Current Value Register"]
    pub t32value1: T32VALUE1,
    #[doc = "0x08 - Timer 1 Timer Control Register"]
    pub t32control1: T32CONTROL1,
    #[doc = "0x0c - Timer 1 Interrupt Clear Register"]
    pub t32intclr1: T32INTCLR1,
    #[doc = "0x10 - Timer 1 Raw Interrupt Status Register"]
    pub t32ris1: T32RIS1,
    #[doc = "0x14 - Timer 1 Interrupt Status Register"]
    pub t32mis1: T32MIS1,
    #[doc = "0x18 - Timer 1 Background Load Register"]
    pub t32bgload1: T32BGLOAD1,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - Timer 2 Load Register"]
    pub t32load2: T32LOAD2,
    #[doc = "0x24 - Timer 2 Current Value Register"]
    pub t32value2: T32VALUE2,
    #[doc = "0x28 - Timer 2 Timer Control Register"]
    pub t32control2: T32CONTROL2,
    #[doc = "0x2c - Timer 2 Interrupt Clear Register"]
    pub t32intclr2: T32INTCLR2,
    #[doc = "0x30 - Timer 2 Raw Interrupt Status Register"]
    pub t32ris2: T32RIS2,
    #[doc = "0x34 - Timer 2 Interrupt Status Register"]
    pub t32mis2: T32MIS2,
    #[doc = "0x38 - Timer 2 Background Load Register"]
    pub t32bgload2: T32BGLOAD2,
}
#[doc = "T32LOAD1 (rw) register accessor: an alias for `Reg<T32LOAD1_SPEC>`"]
pub type T32LOAD1 = crate::Reg<t32load1::T32LOAD1_SPEC>;
#[doc = "Timer 1 Load Register"]
pub mod t32load1;
#[doc = "T32VALUE1 (r) register accessor: an alias for `Reg<T32VALUE1_SPEC>`"]
pub type T32VALUE1 = crate::Reg<t32value1::T32VALUE1_SPEC>;
#[doc = "Timer 1 Current Value Register"]
pub mod t32value1;
#[doc = "T32CONTROL1 (rw) register accessor: an alias for `Reg<T32CONTROL1_SPEC>`"]
pub type T32CONTROL1 = crate::Reg<t32control1::T32CONTROL1_SPEC>;
#[doc = "Timer 1 Timer Control Register"]
pub mod t32control1;
#[doc = "T32INTCLR1 (w) register accessor: an alias for `Reg<T32INTCLR1_SPEC>`"]
pub type T32INTCLR1 = crate::Reg<t32intclr1::T32INTCLR1_SPEC>;
#[doc = "Timer 1 Interrupt Clear Register"]
pub mod t32intclr1;
#[doc = "T32RIS1 (r) register accessor: an alias for `Reg<T32RIS1_SPEC>`"]
pub type T32RIS1 = crate::Reg<t32ris1::T32RIS1_SPEC>;
#[doc = "Timer 1 Raw Interrupt Status Register"]
pub mod t32ris1;
#[doc = "T32MIS1 (r) register accessor: an alias for `Reg<T32MIS1_SPEC>`"]
pub type T32MIS1 = crate::Reg<t32mis1::T32MIS1_SPEC>;
#[doc = "Timer 1 Interrupt Status Register"]
pub mod t32mis1;
#[doc = "T32BGLOAD1 (rw) register accessor: an alias for `Reg<T32BGLOAD1_SPEC>`"]
pub type T32BGLOAD1 = crate::Reg<t32bgload1::T32BGLOAD1_SPEC>;
#[doc = "Timer 1 Background Load Register"]
pub mod t32bgload1;
#[doc = "T32LOAD2 (rw) register accessor: an alias for `Reg<T32LOAD2_SPEC>`"]
pub type T32LOAD2 = crate::Reg<t32load2::T32LOAD2_SPEC>;
#[doc = "Timer 2 Load Register"]
pub mod t32load2;
#[doc = "T32VALUE2 (r) register accessor: an alias for `Reg<T32VALUE2_SPEC>`"]
pub type T32VALUE2 = crate::Reg<t32value2::T32VALUE2_SPEC>;
#[doc = "Timer 2 Current Value Register"]
pub mod t32value2;
#[doc = "T32CONTROL2 (rw) register accessor: an alias for `Reg<T32CONTROL2_SPEC>`"]
pub type T32CONTROL2 = crate::Reg<t32control2::T32CONTROL2_SPEC>;
#[doc = "Timer 2 Timer Control Register"]
pub mod t32control2;
#[doc = "T32INTCLR2 (w) register accessor: an alias for `Reg<T32INTCLR2_SPEC>`"]
pub type T32INTCLR2 = crate::Reg<t32intclr2::T32INTCLR2_SPEC>;
#[doc = "Timer 2 Interrupt Clear Register"]
pub mod t32intclr2;
#[doc = "T32RIS2 (r) register accessor: an alias for `Reg<T32RIS2_SPEC>`"]
pub type T32RIS2 = crate::Reg<t32ris2::T32RIS2_SPEC>;
#[doc = "Timer 2 Raw Interrupt Status Register"]
pub mod t32ris2;
#[doc = "T32MIS2 (r) register accessor: an alias for `Reg<T32MIS2_SPEC>`"]
pub type T32MIS2 = crate::Reg<t32mis2::T32MIS2_SPEC>;
#[doc = "Timer 2 Interrupt Status Register"]
pub mod t32mis2;
#[doc = "T32BGLOAD2 (rw) register accessor: an alias for `Reg<T32BGLOAD2_SPEC>`"]
pub type T32BGLOAD2 = crate::Reg<t32bgload2::T32BGLOAD2_SPEC>;
#[doc = "Timer 2 Background Load Register"]
pub mod t32bgload2;
