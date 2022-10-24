#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LCD_F control"]
    pub lcdctl: LCDCTL,
    #[doc = "0x04 - LCD_F blinking and memory control"]
    pub lcdbmctl: LCDBMCTL,
    #[doc = "0x08 - LCD_F voltage control"]
    pub lcdvctl: LCDVCTL,
    #[doc = "0x0c - LCD_F port control 0"]
    pub lcdpctl0: LCDPCTL0,
    #[doc = "0x10 - LCD_F port control 1"]
    pub lcdpctl1: LCDPCTL1,
    #[doc = "0x14 - LCD_F COM/SEG select register 0"]
    pub lcdcssel0: LCDCSSEL0,
    #[doc = "0x18 - LCD_F COM/SEG select register 1"]
    pub lcdcssel1: LCDCSSEL1,
    #[doc = "0x1c - LCD_F Animation Control Register"]
    pub lcdanmctl: LCDANMCTL,
    _reserved8: [u8; 0xf0],
    #[doc = "0x110 - LCD_F interrupt enable register"]
    pub lcdie: LCDIE,
    #[doc = "0x114 - LCD_F interrupt flag register"]
    pub lcdifg: LCDIFG,
    #[doc = "0x118 - LCD_F set interrupt flag register"]
    pub lcdsetifg: LCDSETIFG,
    #[doc = "0x11c - LCD_F clear interrupt flag register"]
    pub lcdclrifg: LCDCLRIFG,
    #[doc = "0x120..0x150 - LCD memory registers"]
    pub lcdm: [LCDM; 48],
    _reserved13: [u8; 0x10],
    #[doc = "0x160..0x190 - LCD Blinking memory registers"]
    pub lcdbm: [LCDBM; 48],
    _reserved14: [u8; 0x10],
    #[doc = "0x1a0..0x1a8 - LCD Animation memory registers"]
    pub lcdanm: [LCDANM; 8],
}
#[doc = "LCDCTL (rw) register accessor: an alias for `Reg<LCDCTL_SPEC>`"]
pub type LCDCTL = crate::Reg<lcdctl::LCDCTL_SPEC>;
#[doc = "LCD_F control"]
pub mod lcdctl;
#[doc = "LCDBMCTL (rw) register accessor: an alias for `Reg<LCDBMCTL_SPEC>`"]
pub type LCDBMCTL = crate::Reg<lcdbmctl::LCDBMCTL_SPEC>;
#[doc = "LCD_F blinking and memory control"]
pub mod lcdbmctl;
#[doc = "LCDVCTL (rw) register accessor: an alias for `Reg<LCDVCTL_SPEC>`"]
pub type LCDVCTL = crate::Reg<lcdvctl::LCDVCTL_SPEC>;
#[doc = "LCD_F voltage control"]
pub mod lcdvctl;
#[doc = "LCDPCTL0 (rw) register accessor: an alias for `Reg<LCDPCTL0_SPEC>`"]
pub type LCDPCTL0 = crate::Reg<lcdpctl0::LCDPCTL0_SPEC>;
#[doc = "LCD_F port control 0"]
pub mod lcdpctl0;
#[doc = "LCDPCTL1 (rw) register accessor: an alias for `Reg<LCDPCTL1_SPEC>`"]
pub type LCDPCTL1 = crate::Reg<lcdpctl1::LCDPCTL1_SPEC>;
#[doc = "LCD_F port control 1"]
pub mod lcdpctl1;
#[doc = "LCDCSSEL0 (rw) register accessor: an alias for `Reg<LCDCSSEL0_SPEC>`"]
pub type LCDCSSEL0 = crate::Reg<lcdcssel0::LCDCSSEL0_SPEC>;
#[doc = "LCD_F COM/SEG select register 0"]
pub mod lcdcssel0;
#[doc = "LCDCSSEL1 (rw) register accessor: an alias for `Reg<LCDCSSEL1_SPEC>`"]
pub type LCDCSSEL1 = crate::Reg<lcdcssel1::LCDCSSEL1_SPEC>;
#[doc = "LCD_F COM/SEG select register 1"]
pub mod lcdcssel1;
#[doc = "LCDANMCTL (rw) register accessor: an alias for `Reg<LCDANMCTL_SPEC>`"]
pub type LCDANMCTL = crate::Reg<lcdanmctl::LCDANMCTL_SPEC>;
#[doc = "LCD_F Animation Control Register"]
pub mod lcdanmctl;
#[doc = "LCDIE (rw) register accessor: an alias for `Reg<LCDIE_SPEC>`"]
pub type LCDIE = crate::Reg<lcdie::LCDIE_SPEC>;
#[doc = "LCD_F interrupt enable register"]
pub mod lcdie;
#[doc = "LCDIFG (r) register accessor: an alias for `Reg<LCDIFG_SPEC>`"]
pub type LCDIFG = crate::Reg<lcdifg::LCDIFG_SPEC>;
#[doc = "LCD_F interrupt flag register"]
pub mod lcdifg;
#[doc = "LCDSETIFG (w) register accessor: an alias for `Reg<LCDSETIFG_SPEC>`"]
pub type LCDSETIFG = crate::Reg<lcdsetifg::LCDSETIFG_SPEC>;
#[doc = "LCD_F set interrupt flag register"]
pub mod lcdsetifg;
#[doc = "LCDCLRIFG (w) register accessor: an alias for `Reg<LCDCLRIFG_SPEC>`"]
pub type LCDCLRIFG = crate::Reg<lcdclrifg::LCDCLRIFG_SPEC>;
#[doc = "LCD_F clear interrupt flag register"]
pub mod lcdclrifg;
#[doc = "LCDM (rw) register accessor: an alias for `Reg<LCDM_SPEC>`"]
pub type LCDM = crate::Reg<lcdm::LCDM_SPEC>;
#[doc = "LCD memory registers"]
pub mod lcdm;
#[doc = "LCDBM (rw) register accessor: an alias for `Reg<LCDBM_SPEC>`"]
pub type LCDBM = crate::Reg<lcdbm::LCDBM_SPEC>;
#[doc = "LCD Blinking memory registers"]
pub mod lcdbm;
#[doc = "LCDANM (rw) register accessor: an alias for `Reg<LCDANM_SPEC>`"]
pub type LCDANM = crate::Reg<lcdanm::LCDANM_SPEC>;
#[doc = "LCD Animation memory registers"]
pub mod lcdanm;
