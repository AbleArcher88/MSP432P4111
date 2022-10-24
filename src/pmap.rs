#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port Mapping Key Register"]
    pub pmapkeyid: PMAPKEYID,
    #[doc = "0x02 - Port Mapping Control Register"]
    pub pmapctl: PMAPCTL,
    _reserved2: [u8; 0x04],
    #[doc = "0x08 - Port mapping register, P1.0 and P1.1"]
    pub p1map01: P1MAP01,
    #[doc = "0x0a - Port mapping register, P1.2 and P1.3"]
    pub p1map23: P1MAP23,
    #[doc = "0x0c - Port mapping register, P1.4 and P1.5"]
    pub p1map45: P1MAP45,
    #[doc = "0x0e - Port mapping register, P1.6 and P1.7"]
    pub p1map67: P1MAP67,
    #[doc = "0x10 - Port mapping register, P2.0 and P2.1"]
    pub p2map01: P2MAP01,
    #[doc = "0x12 - Port mapping register, P2.2 and P2.3"]
    pub p2map23: P2MAP23,
    #[doc = "0x14 - Port mapping register, P2.4 and P2.5"]
    pub p2map45: P2MAP45,
    #[doc = "0x16 - Port mapping register, P2.6 and P2.7"]
    pub p2map67: P2MAP67,
    #[doc = "0x18 - Port mapping register, P3.0 and P3.1"]
    pub p3map01: P3MAP01,
    #[doc = "0x1a - Port mapping register, P3.2 and P3.3"]
    pub p3map23: P3MAP23,
    #[doc = "0x1c - Port mapping register, P3.4 and P3.5"]
    pub p3map45: P3MAP45,
    #[doc = "0x1e - Port mapping register, P3.6 and P3.7"]
    pub p3map67: P3MAP67,
    #[doc = "0x20 - Port mapping register, P4.0 and P4.1"]
    pub p4map01: P4MAP01,
    #[doc = "0x22 - Port mapping register, P4.2 and P4.3"]
    pub p4map23: P4MAP23,
    #[doc = "0x24 - Port mapping register, P4.4 and P4.5"]
    pub p4map45: P4MAP45,
    #[doc = "0x26 - Port mapping register, P4.6 and P4.7"]
    pub p4map67: P4MAP67,
    #[doc = "0x28 - Port mapping register, P5.0 and P5.1"]
    pub p5map01: P5MAP01,
    #[doc = "0x2a - Port mapping register, P5.2 and P5.3"]
    pub p5map23: P5MAP23,
    #[doc = "0x2c - Port mapping register, P5.4 and P5.5"]
    pub p5map45: P5MAP45,
    #[doc = "0x2e - Port mapping register, P5.6 and P5.7"]
    pub p5map67: P5MAP67,
    #[doc = "0x30 - Port mapping register, P6.0 and P6.1"]
    pub p6map01: P6MAP01,
    #[doc = "0x32 - Port mapping register, P6.2 and P6.3"]
    pub p6map23: P6MAP23,
    #[doc = "0x34 - Port mapping register, P6.4 and P6.5"]
    pub p6map45: P6MAP45,
    #[doc = "0x36 - Port mapping register, P6.6 and P6.7"]
    pub p6map67: P6MAP67,
    #[doc = "0x38 - Port mapping register, P7.0 and P7.1"]
    pub p7map01: P7MAP01,
    #[doc = "0x3a - Port mapping register, P7.2 and P7.3"]
    pub p7map23: P7MAP23,
    #[doc = "0x3c - Port mapping register, P7.4 and P7.5"]
    pub p7map45: P7MAP45,
    #[doc = "0x3e - Port mapping register, P7.6 and P7.7"]
    pub p7map67: P7MAP67,
}
#[doc = "PMAPKEYID (rw) register accessor: an alias for `Reg<PMAPKEYID_SPEC>`"]
pub type PMAPKEYID = crate::Reg<pmapkeyid::PMAPKEYID_SPEC>;
#[doc = "Port Mapping Key Register"]
pub mod pmapkeyid;
#[doc = "PMAPCTL (rw) register accessor: an alias for `Reg<PMAPCTL_SPEC>`"]
pub type PMAPCTL = crate::Reg<pmapctl::PMAPCTL_SPEC>;
#[doc = "Port Mapping Control Register"]
pub mod pmapctl;
#[doc = "P1MAP01 (rw) register accessor: an alias for `Reg<P1MAP01_SPEC>`"]
pub type P1MAP01 = crate::Reg<p1map01::P1MAP01_SPEC>;
#[doc = "Port mapping register, P1.0 and P1.1"]
pub mod p1map01;
#[doc = "P1MAP23 (rw) register accessor: an alias for `Reg<P1MAP23_SPEC>`"]
pub type P1MAP23 = crate::Reg<p1map23::P1MAP23_SPEC>;
#[doc = "Port mapping register, P1.2 and P1.3"]
pub mod p1map23;
#[doc = "P1MAP45 (rw) register accessor: an alias for `Reg<P1MAP45_SPEC>`"]
pub type P1MAP45 = crate::Reg<p1map45::P1MAP45_SPEC>;
#[doc = "Port mapping register, P1.4 and P1.5"]
pub mod p1map45;
#[doc = "P1MAP67 (rw) register accessor: an alias for `Reg<P1MAP67_SPEC>`"]
pub type P1MAP67 = crate::Reg<p1map67::P1MAP67_SPEC>;
#[doc = "Port mapping register, P1.6 and P1.7"]
pub mod p1map67;
#[doc = "P2MAP01 (rw) register accessor: an alias for `Reg<P2MAP01_SPEC>`"]
pub type P2MAP01 = crate::Reg<p2map01::P2MAP01_SPEC>;
#[doc = "Port mapping register, P2.0 and P2.1"]
pub mod p2map01;
#[doc = "P2MAP23 (rw) register accessor: an alias for `Reg<P2MAP23_SPEC>`"]
pub type P2MAP23 = crate::Reg<p2map23::P2MAP23_SPEC>;
#[doc = "Port mapping register, P2.2 and P2.3"]
pub mod p2map23;
#[doc = "P2MAP45 (rw) register accessor: an alias for `Reg<P2MAP45_SPEC>`"]
pub type P2MAP45 = crate::Reg<p2map45::P2MAP45_SPEC>;
#[doc = "Port mapping register, P2.4 and P2.5"]
pub mod p2map45;
#[doc = "P2MAP67 (rw) register accessor: an alias for `Reg<P2MAP67_SPEC>`"]
pub type P2MAP67 = crate::Reg<p2map67::P2MAP67_SPEC>;
#[doc = "Port mapping register, P2.6 and P2.7"]
pub mod p2map67;
#[doc = "P3MAP01 (rw) register accessor: an alias for `Reg<P3MAP01_SPEC>`"]
pub type P3MAP01 = crate::Reg<p3map01::P3MAP01_SPEC>;
#[doc = "Port mapping register, P3.0 and P3.1"]
pub mod p3map01;
#[doc = "P3MAP23 (rw) register accessor: an alias for `Reg<P3MAP23_SPEC>`"]
pub type P3MAP23 = crate::Reg<p3map23::P3MAP23_SPEC>;
#[doc = "Port mapping register, P3.2 and P3.3"]
pub mod p3map23;
#[doc = "P3MAP45 (rw) register accessor: an alias for `Reg<P3MAP45_SPEC>`"]
pub type P3MAP45 = crate::Reg<p3map45::P3MAP45_SPEC>;
#[doc = "Port mapping register, P3.4 and P3.5"]
pub mod p3map45;
#[doc = "P3MAP67 (rw) register accessor: an alias for `Reg<P3MAP67_SPEC>`"]
pub type P3MAP67 = crate::Reg<p3map67::P3MAP67_SPEC>;
#[doc = "Port mapping register, P3.6 and P3.7"]
pub mod p3map67;
#[doc = "P4MAP01 (rw) register accessor: an alias for `Reg<P4MAP01_SPEC>`"]
pub type P4MAP01 = crate::Reg<p4map01::P4MAP01_SPEC>;
#[doc = "Port mapping register, P4.0 and P4.1"]
pub mod p4map01;
#[doc = "P4MAP23 (rw) register accessor: an alias for `Reg<P4MAP23_SPEC>`"]
pub type P4MAP23 = crate::Reg<p4map23::P4MAP23_SPEC>;
#[doc = "Port mapping register, P4.2 and P4.3"]
pub mod p4map23;
#[doc = "P4MAP45 (rw) register accessor: an alias for `Reg<P4MAP45_SPEC>`"]
pub type P4MAP45 = crate::Reg<p4map45::P4MAP45_SPEC>;
#[doc = "Port mapping register, P4.4 and P4.5"]
pub mod p4map45;
#[doc = "P4MAP67 (rw) register accessor: an alias for `Reg<P4MAP67_SPEC>`"]
pub type P4MAP67 = crate::Reg<p4map67::P4MAP67_SPEC>;
#[doc = "Port mapping register, P4.6 and P4.7"]
pub mod p4map67;
#[doc = "P5MAP01 (rw) register accessor: an alias for `Reg<P5MAP01_SPEC>`"]
pub type P5MAP01 = crate::Reg<p5map01::P5MAP01_SPEC>;
#[doc = "Port mapping register, P5.0 and P5.1"]
pub mod p5map01;
#[doc = "P5MAP23 (rw) register accessor: an alias for `Reg<P5MAP23_SPEC>`"]
pub type P5MAP23 = crate::Reg<p5map23::P5MAP23_SPEC>;
#[doc = "Port mapping register, P5.2 and P5.3"]
pub mod p5map23;
#[doc = "P5MAP45 (rw) register accessor: an alias for `Reg<P5MAP45_SPEC>`"]
pub type P5MAP45 = crate::Reg<p5map45::P5MAP45_SPEC>;
#[doc = "Port mapping register, P5.4 and P5.5"]
pub mod p5map45;
#[doc = "P5MAP67 (rw) register accessor: an alias for `Reg<P5MAP67_SPEC>`"]
pub type P5MAP67 = crate::Reg<p5map67::P5MAP67_SPEC>;
#[doc = "Port mapping register, P5.6 and P5.7"]
pub mod p5map67;
#[doc = "P6MAP01 (rw) register accessor: an alias for `Reg<P6MAP01_SPEC>`"]
pub type P6MAP01 = crate::Reg<p6map01::P6MAP01_SPEC>;
#[doc = "Port mapping register, P6.0 and P6.1"]
pub mod p6map01;
#[doc = "P6MAP23 (rw) register accessor: an alias for `Reg<P6MAP23_SPEC>`"]
pub type P6MAP23 = crate::Reg<p6map23::P6MAP23_SPEC>;
#[doc = "Port mapping register, P6.2 and P6.3"]
pub mod p6map23;
#[doc = "P6MAP45 (rw) register accessor: an alias for `Reg<P6MAP45_SPEC>`"]
pub type P6MAP45 = crate::Reg<p6map45::P6MAP45_SPEC>;
#[doc = "Port mapping register, P6.4 and P6.5"]
pub mod p6map45;
#[doc = "P6MAP67 (rw) register accessor: an alias for `Reg<P6MAP67_SPEC>`"]
pub type P6MAP67 = crate::Reg<p6map67::P6MAP67_SPEC>;
#[doc = "Port mapping register, P6.6 and P6.7"]
pub mod p6map67;
#[doc = "P7MAP01 (rw) register accessor: an alias for `Reg<P7MAP01_SPEC>`"]
pub type P7MAP01 = crate::Reg<p7map01::P7MAP01_SPEC>;
#[doc = "Port mapping register, P7.0 and P7.1"]
pub mod p7map01;
#[doc = "P7MAP23 (rw) register accessor: an alias for `Reg<P7MAP23_SPEC>`"]
pub type P7MAP23 = crate::Reg<p7map23::P7MAP23_SPEC>;
#[doc = "Port mapping register, P7.2 and P7.3"]
pub mod p7map23;
#[doc = "P7MAP45 (rw) register accessor: an alias for `Reg<P7MAP45_SPEC>`"]
pub type P7MAP45 = crate::Reg<p7map45::P7MAP45_SPEC>;
#[doc = "Port mapping register, P7.4 and P7.5"]
pub mod p7map45;
#[doc = "P7MAP67 (rw) register accessor: an alias for `Reg<P7MAP67_SPEC>`"]
pub type P7MAP67 = crate::Reg<p7map67::P7MAP67_SPEC>;
#[doc = "Port mapping register, P7.6 and P7.7"]
pub mod p7map67;
