#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TLV Checksum"]
    pub tlv_checksum: TLV_CHECKSUM,
    #[doc = "0x04 - Device Info Tag"]
    pub device_info_tag: DEVICE_INFO_TAG,
    #[doc = "0x08 - Device Info Length"]
    pub device_info_len: DEVICE_INFO_LEN,
    #[doc = "0x0c - Device ID"]
    pub device_id: DEVICE_ID,
    #[doc = "0x10 - HW Revision"]
    pub hwrev: HWREV,
    #[doc = "0x14 - Boot Code Revision"]
    pub bcrev: BCREV,
    #[doc = "0x18 - ROM Driver Library Revision"]
    pub rom_drvlib_rev: ROM_DRVLIB_REV,
    #[doc = "0x1c - Die Record Tag"]
    pub die_rec_tag: DIE_REC_TAG,
    #[doc = "0x20 - Die Record Length"]
    pub die_rec_len: DIE_REC_LEN,
    #[doc = "0x24 - Die X-Position"]
    pub die_xpos: DIE_XPOS,
    #[doc = "0x28 - Die Y-Position"]
    pub die_ypos: DIE_YPOS,
    #[doc = "0x2c - Wafer ID"]
    pub wafer_id: WAFER_ID,
    #[doc = "0x30 - Lot ID"]
    pub lot_id: LOT_ID,
    #[doc = "0x34 - Reserved"]
    pub reserved0: RESERVED0,
    #[doc = "0x38 - Reserved"]
    pub reserved1: RESERVED1,
    #[doc = "0x3c - Reserved"]
    pub reserved2: RESERVED2,
    #[doc = "0x40 - Test Results"]
    pub test_results: TEST_RESULTS,
    #[doc = "0x44 - Clock System Calibration Tag"]
    pub cs_cal_tag: CS_CAL_TAG,
    #[doc = "0x48 - Clock System Calibration Length"]
    pub cs_cal_len: CS_CAL_LEN,
    #[doc = "0x4c - DCO IR mode: Frequency calibration for DCORSEL 0 to 4"]
    pub dcoir_fcal_rsel04: DCOIR_FCAL_RSEL04,
    #[doc = "0x50 - DCO IR mode: Frequency calibration for DCORSEL 5"]
    pub dcoir_fcal_rsel5: DCOIR_FCAL_RSEL5,
    #[doc = "0x54 - Reserved"]
    pub reserved3: RESERVED3,
    #[doc = "0x58 - Reserved"]
    pub reserved4: RESERVED4,
    #[doc = "0x5c - Reserved"]
    pub reserved5: RESERVED5,
    #[doc = "0x60 - Reserved"]
    pub reserved6: RESERVED6,
    #[doc = "0x64 - DCO IR mode: DCO Constant (K) for DCORSEL 0 to 4"]
    pub dcoir_constk_rsel04: DCOIR_CONSTK_RSEL04,
    #[doc = "0x68 - DCO IR mode: DCO Constant (K) for DCORSEL 5"]
    pub dcoir_constk_rsel5: DCOIR_CONSTK_RSEL5,
    #[doc = "0x6c - DCO ER mode: Frequency calibration for DCORSEL 0 to 4"]
    pub dcoer_fcal_rsel04: DCOER_FCAL_RSEL04,
    #[doc = "0x70 - DCO ER mode: Frequency calibration for DCORSEL 5"]
    pub dcoer_fcal_rsel5: DCOER_FCAL_RSEL5,
    #[doc = "0x74 - Reserved"]
    pub reserved7: RESERVED7,
    #[doc = "0x78 - Reserved"]
    pub reserved8: RESERVED8,
    #[doc = "0x7c - Reserved"]
    pub reserved9: RESERVED9,
    #[doc = "0x80 - Reserved"]
    pub reserved10: RESERVED10,
    #[doc = "0x84 - DCO ER mode: DCO Constant (K) for DCORSEL 0 to 4"]
    pub dcoer_constk_rsel04: DCOER_CONSTK_RSEL04,
    #[doc = "0x88 - DCO ER mode: DCO Constant (K) for DCORSEL 5"]
    pub dcoer_constk_rsel5: DCOER_CONSTK_RSEL5,
    #[doc = "0x8c - ADC14 Calibration Tag"]
    pub adc14_cal_tag: ADC14_CAL_TAG,
    #[doc = "0x90 - ADC14 Calibration Length"]
    pub adc14_cal_len: ADC14_CAL_LEN,
    #[doc = "0x94 - ADC Gain Factor"]
    pub adc_gain_factor: ADC_GAIN_FACTOR,
    #[doc = "0x98 - ADC Offset"]
    pub adc_offset: ADC_OFFSET,
    #[doc = "0x9c - Reserved"]
    pub reserved11: RESERVED11,
    #[doc = "0xa0 - Reserved"]
    pub reserved12: RESERVED12,
    #[doc = "0xa4 - Reserved"]
    pub reserved13: RESERVED13,
    #[doc = "0xa8 - Reserved"]
    pub reserved14: RESERVED14,
    #[doc = "0xac - Reserved"]
    pub reserved15: RESERVED15,
    #[doc = "0xb0 - Reserved"]
    pub reserved16: RESERVED16,
    #[doc = "0xb4 - Reserved"]
    pub reserved17: RESERVED17,
    #[doc = "0xb8 - Reserved"]
    pub reserved18: RESERVED18,
    #[doc = "0xbc - Reserved"]
    pub reserved19: RESERVED19,
    #[doc = "0xc0 - Reserved"]
    pub reserved20: RESERVED20,
    #[doc = "0xc4 - Reserved"]
    pub reserved21: RESERVED21,
    #[doc = "0xc8 - Reserved"]
    pub reserved22: RESERVED22,
    #[doc = "0xcc - Reserved"]
    pub reserved23: RESERVED23,
    #[doc = "0xd0 - Reserved"]
    pub reserved24: RESERVED24,
    #[doc = "0xd4 - Reserved"]
    pub reserved25: RESERVED25,
    #[doc = "0xd8 - Reserved"]
    pub reserved26: RESERVED26,
    #[doc = "0xdc - ADC14 1.2V Reference Temp. Sensor 30C"]
    pub adc14_ref1p2v_ts30c: ADC14_REF1P2V_TS30C,
    #[doc = "0xe0 - ADC14 1.2V Reference Temp. Sensor 85C"]
    pub adc14_ref1p2v_ts85c: ADC14_REF1P2V_TS85C,
    #[doc = "0xe4 - ADC14 1.45V Reference Temp. Sensor 30C"]
    pub adc14_ref1p45v_ts30c: ADC14_REF1P45V_TS30C,
    #[doc = "0xe8 - ADC14 1.45V Reference Temp. Sensor 85C"]
    pub adc14_ref1p45v_ts85c: ADC14_REF1P45V_TS85C,
    #[doc = "0xec - ADC14 2.5V Reference Temp. Sensor 30C"]
    pub adc14_ref2p5v_ts30c: ADC14_REF2P5V_TS30C,
    #[doc = "0xf0 - ADC14 2.5V Reference Temp. Sensor 85C"]
    pub adc14_ref2p5v_ts85c: ADC14_REF2P5V_TS85C,
    #[doc = "0xf4 - REF Calibration Tag"]
    pub ref_cal_tag: REF_CAL_TAG,
    #[doc = "0xf8 - REF Calibration Length"]
    pub ref_cal_len: REF_CAL_LEN,
    #[doc = "0xfc - REF 1.2V Reference"]
    pub ref_1p2v: REF_1P2V,
    #[doc = "0x100 - REF 1.45V Reference"]
    pub ref_1p45v: REF_1P45V,
    #[doc = "0x104 - REF 2.5V Reference"]
    pub ref_2p5v: REF_2P5V,
    #[doc = "0x108 - Flash Info Tag"]
    pub flash_info_tag: FLASH_INFO_TAG,
    #[doc = "0x10c - Flash Info Length"]
    pub flash_info_len: FLASH_INFO_LEN,
    #[doc = "0x110 - Flash Maximum Programming Pulses"]
    pub flash_max_prog_pulses: FLASH_MAX_PROG_PULSES,
    #[doc = "0x114 - Flash Maximum Erase Pulses"]
    pub flash_max_erase_pulses: FLASH_MAX_ERASE_PULSES,
    #[doc = "0x118 - 128-bit Random Number Tag"]
    pub random_num_tag: RANDOM_NUM_TAG,
    #[doc = "0x11c - 128-bit Random Number Length"]
    pub random_num_len: RANDOM_NUM_LEN,
    #[doc = "0x120 - 32-bit Random Number 1"]
    pub random_num_1: RANDOM_NUM_1,
    #[doc = "0x124 - 32-bit Random Number 2"]
    pub random_num_2: RANDOM_NUM_2,
    #[doc = "0x128 - 32-bit Random Number 3"]
    pub random_num_3: RANDOM_NUM_3,
    #[doc = "0x12c - 32-bit Random Number 4"]
    pub random_num_4: RANDOM_NUM_4,
    #[doc = "0x130 - BSL Configuration Tag"]
    pub bsl_cfg_tag: BSL_CFG_TAG,
    #[doc = "0x134 - BSL Configuration Length"]
    pub bsl_cfg_len: BSL_CFG_LEN,
    #[doc = "0x138 - BSL Peripheral Interface Selection"]
    pub bsl_periphif_sel: BSL_PERIPHIF_SEL,
    #[doc = "0x13c - BSL Port Interface Configuration for UART"]
    pub bsl_portif_cfg_uart: BSL_PORTIF_CFG_UART,
    #[doc = "0x140 - BSL Port Interface Configuration for SPI"]
    pub bsl_portif_cfg_spi: BSL_PORTIF_CFG_SPI,
    #[doc = "0x144 - BSL Port Interface Configuration for I2C"]
    pub bsl_portif_cfg_i2c: BSL_PORTIF_CFG_I2C,
    #[doc = "0x148 - TLV End Word"]
    pub tlv_end: TLV_END,
}
#[doc = "TLV_CHECKSUM (r) register accessor: an alias for `Reg<TLV_CHECKSUM_SPEC>`"]
pub type TLV_CHECKSUM = crate::Reg<tlv_checksum::TLV_CHECKSUM_SPEC>;
#[doc = "TLV Checksum"]
pub mod tlv_checksum;
#[doc = "DEVICE_INFO_TAG (r) register accessor: an alias for `Reg<DEVICE_INFO_TAG_SPEC>`"]
pub type DEVICE_INFO_TAG = crate::Reg<device_info_tag::DEVICE_INFO_TAG_SPEC>;
#[doc = "Device Info Tag"]
pub mod device_info_tag;
#[doc = "DEVICE_INFO_LEN (r) register accessor: an alias for `Reg<DEVICE_INFO_LEN_SPEC>`"]
pub type DEVICE_INFO_LEN = crate::Reg<device_info_len::DEVICE_INFO_LEN_SPEC>;
#[doc = "Device Info Length"]
pub mod device_info_len;
#[doc = "DEVICE_ID (r) register accessor: an alias for `Reg<DEVICE_ID_SPEC>`"]
pub type DEVICE_ID = crate::Reg<device_id::DEVICE_ID_SPEC>;
#[doc = "Device ID"]
pub mod device_id;
#[doc = "HWREV (r) register accessor: an alias for `Reg<HWREV_SPEC>`"]
pub type HWREV = crate::Reg<hwrev::HWREV_SPEC>;
#[doc = "HW Revision"]
pub mod hwrev;
#[doc = "BCREV (r) register accessor: an alias for `Reg<BCREV_SPEC>`"]
pub type BCREV = crate::Reg<bcrev::BCREV_SPEC>;
#[doc = "Boot Code Revision"]
pub mod bcrev;
#[doc = "ROM_DRVLIB_REV (r) register accessor: an alias for `Reg<ROM_DRVLIB_REV_SPEC>`"]
pub type ROM_DRVLIB_REV = crate::Reg<rom_drvlib_rev::ROM_DRVLIB_REV_SPEC>;
#[doc = "ROM Driver Library Revision"]
pub mod rom_drvlib_rev;
#[doc = "DIE_REC_TAG (r) register accessor: an alias for `Reg<DIE_REC_TAG_SPEC>`"]
pub type DIE_REC_TAG = crate::Reg<die_rec_tag::DIE_REC_TAG_SPEC>;
#[doc = "Die Record Tag"]
pub mod die_rec_tag;
#[doc = "DIE_REC_LEN (r) register accessor: an alias for `Reg<DIE_REC_LEN_SPEC>`"]
pub type DIE_REC_LEN = crate::Reg<die_rec_len::DIE_REC_LEN_SPEC>;
#[doc = "Die Record Length"]
pub mod die_rec_len;
#[doc = "DIE_XPOS (r) register accessor: an alias for `Reg<DIE_XPOS_SPEC>`"]
pub type DIE_XPOS = crate::Reg<die_xpos::DIE_XPOS_SPEC>;
#[doc = "Die X-Position"]
pub mod die_xpos;
#[doc = "DIE_YPOS (r) register accessor: an alias for `Reg<DIE_YPOS_SPEC>`"]
pub type DIE_YPOS = crate::Reg<die_ypos::DIE_YPOS_SPEC>;
#[doc = "Die Y-Position"]
pub mod die_ypos;
#[doc = "WAFER_ID (r) register accessor: an alias for `Reg<WAFER_ID_SPEC>`"]
pub type WAFER_ID = crate::Reg<wafer_id::WAFER_ID_SPEC>;
#[doc = "Wafer ID"]
pub mod wafer_id;
#[doc = "LOT_ID (r) register accessor: an alias for `Reg<LOT_ID_SPEC>`"]
pub type LOT_ID = crate::Reg<lot_id::LOT_ID_SPEC>;
#[doc = "Lot ID"]
pub mod lot_id;
#[doc = "RESERVED0 (r) register accessor: an alias for `Reg<RESERVED0_SPEC>`"]
pub type RESERVED0 = crate::Reg<reserved0::RESERVED0_SPEC>;
#[doc = "Reserved"]
pub mod reserved0;
#[doc = "RESERVED1 (r) register accessor: an alias for `Reg<RESERVED1_SPEC>`"]
pub type RESERVED1 = crate::Reg<reserved1::RESERVED1_SPEC>;
#[doc = "Reserved"]
pub mod reserved1;
#[doc = "RESERVED2 (r) register accessor: an alias for `Reg<RESERVED2_SPEC>`"]
pub type RESERVED2 = crate::Reg<reserved2::RESERVED2_SPEC>;
#[doc = "Reserved"]
pub mod reserved2;
#[doc = "TEST_RESULTS (r) register accessor: an alias for `Reg<TEST_RESULTS_SPEC>`"]
pub type TEST_RESULTS = crate::Reg<test_results::TEST_RESULTS_SPEC>;
#[doc = "Test Results"]
pub mod test_results;
#[doc = "CS_CAL_TAG (r) register accessor: an alias for `Reg<CS_CAL_TAG_SPEC>`"]
pub type CS_CAL_TAG = crate::Reg<cs_cal_tag::CS_CAL_TAG_SPEC>;
#[doc = "Clock System Calibration Tag"]
pub mod cs_cal_tag;
#[doc = "CS_CAL_LEN (r) register accessor: an alias for `Reg<CS_CAL_LEN_SPEC>`"]
pub type CS_CAL_LEN = crate::Reg<cs_cal_len::CS_CAL_LEN_SPEC>;
#[doc = "Clock System Calibration Length"]
pub mod cs_cal_len;
#[doc = "DCOIR_FCAL_RSEL04 (r) register accessor: an alias for `Reg<DCOIR_FCAL_RSEL04_SPEC>`"]
pub type DCOIR_FCAL_RSEL04 = crate::Reg<dcoir_fcal_rsel04::DCOIR_FCAL_RSEL04_SPEC>;
#[doc = "DCO IR mode: Frequency calibration for DCORSEL 0 to 4"]
pub mod dcoir_fcal_rsel04;
#[doc = "DCOIR_FCAL_RSEL5 (r) register accessor: an alias for `Reg<DCOIR_FCAL_RSEL5_SPEC>`"]
pub type DCOIR_FCAL_RSEL5 = crate::Reg<dcoir_fcal_rsel5::DCOIR_FCAL_RSEL5_SPEC>;
#[doc = "DCO IR mode: Frequency calibration for DCORSEL 5"]
pub mod dcoir_fcal_rsel5;
#[doc = "RESERVED3 (r) register accessor: an alias for `Reg<RESERVED3_SPEC>`"]
pub type RESERVED3 = crate::Reg<reserved3::RESERVED3_SPEC>;
#[doc = "Reserved"]
pub mod reserved3;
#[doc = "RESERVED4 (r) register accessor: an alias for `Reg<RESERVED4_SPEC>`"]
pub type RESERVED4 = crate::Reg<reserved4::RESERVED4_SPEC>;
#[doc = "Reserved"]
pub mod reserved4;
#[doc = "RESERVED5 (r) register accessor: an alias for `Reg<RESERVED5_SPEC>`"]
pub type RESERVED5 = crate::Reg<reserved5::RESERVED5_SPEC>;
#[doc = "Reserved"]
pub mod reserved5;
#[doc = "RESERVED6 (r) register accessor: an alias for `Reg<RESERVED6_SPEC>`"]
pub type RESERVED6 = crate::Reg<reserved6::RESERVED6_SPEC>;
#[doc = "Reserved"]
pub mod reserved6;
#[doc = "DCOIR_CONSTK_RSEL04 (r) register accessor: an alias for `Reg<DCOIR_CONSTK_RSEL04_SPEC>`"]
pub type DCOIR_CONSTK_RSEL04 = crate::Reg<dcoir_constk_rsel04::DCOIR_CONSTK_RSEL04_SPEC>;
#[doc = "DCO IR mode: DCO Constant (K) for DCORSEL 0 to 4"]
pub mod dcoir_constk_rsel04;
#[doc = "DCOIR_CONSTK_RSEL5 (r) register accessor: an alias for `Reg<DCOIR_CONSTK_RSEL5_SPEC>`"]
pub type DCOIR_CONSTK_RSEL5 = crate::Reg<dcoir_constk_rsel5::DCOIR_CONSTK_RSEL5_SPEC>;
#[doc = "DCO IR mode: DCO Constant (K) for DCORSEL 5"]
pub mod dcoir_constk_rsel5;
#[doc = "DCOER_FCAL_RSEL04 (r) register accessor: an alias for `Reg<DCOER_FCAL_RSEL04_SPEC>`"]
pub type DCOER_FCAL_RSEL04 = crate::Reg<dcoer_fcal_rsel04::DCOER_FCAL_RSEL04_SPEC>;
#[doc = "DCO ER mode: Frequency calibration for DCORSEL 0 to 4"]
pub mod dcoer_fcal_rsel04;
#[doc = "DCOER_FCAL_RSEL5 (r) register accessor: an alias for `Reg<DCOER_FCAL_RSEL5_SPEC>`"]
pub type DCOER_FCAL_RSEL5 = crate::Reg<dcoer_fcal_rsel5::DCOER_FCAL_RSEL5_SPEC>;
#[doc = "DCO ER mode: Frequency calibration for DCORSEL 5"]
pub mod dcoer_fcal_rsel5;
#[doc = "RESERVED7 (r) register accessor: an alias for `Reg<RESERVED7_SPEC>`"]
pub type RESERVED7 = crate::Reg<reserved7::RESERVED7_SPEC>;
#[doc = "Reserved"]
pub mod reserved7;
#[doc = "RESERVED8 (r) register accessor: an alias for `Reg<RESERVED8_SPEC>`"]
pub type RESERVED8 = crate::Reg<reserved8::RESERVED8_SPEC>;
#[doc = "Reserved"]
pub mod reserved8;
#[doc = "RESERVED9 (r) register accessor: an alias for `Reg<RESERVED9_SPEC>`"]
pub type RESERVED9 = crate::Reg<reserved9::RESERVED9_SPEC>;
#[doc = "Reserved"]
pub mod reserved9;
#[doc = "RESERVED10 (r) register accessor: an alias for `Reg<RESERVED10_SPEC>`"]
pub type RESERVED10 = crate::Reg<reserved10::RESERVED10_SPEC>;
#[doc = "Reserved"]
pub mod reserved10;
#[doc = "DCOER_CONSTK_RSEL04 (r) register accessor: an alias for `Reg<DCOER_CONSTK_RSEL04_SPEC>`"]
pub type DCOER_CONSTK_RSEL04 = crate::Reg<dcoer_constk_rsel04::DCOER_CONSTK_RSEL04_SPEC>;
#[doc = "DCO ER mode: DCO Constant (K) for DCORSEL 0 to 4"]
pub mod dcoer_constk_rsel04;
#[doc = "DCOER_CONSTK_RSEL5 (r) register accessor: an alias for `Reg<DCOER_CONSTK_RSEL5_SPEC>`"]
pub type DCOER_CONSTK_RSEL5 = crate::Reg<dcoer_constk_rsel5::DCOER_CONSTK_RSEL5_SPEC>;
#[doc = "DCO ER mode: DCO Constant (K) for DCORSEL 5"]
pub mod dcoer_constk_rsel5;
#[doc = "ADC14_CAL_TAG (r) register accessor: an alias for `Reg<ADC14_CAL_TAG_SPEC>`"]
pub type ADC14_CAL_TAG = crate::Reg<adc14_cal_tag::ADC14_CAL_TAG_SPEC>;
#[doc = "ADC14 Calibration Tag"]
pub mod adc14_cal_tag;
#[doc = "ADC14_CAL_LEN (r) register accessor: an alias for `Reg<ADC14_CAL_LEN_SPEC>`"]
pub type ADC14_CAL_LEN = crate::Reg<adc14_cal_len::ADC14_CAL_LEN_SPEC>;
#[doc = "ADC14 Calibration Length"]
pub mod adc14_cal_len;
#[doc = "ADC_GAIN_FACTOR (r) register accessor: an alias for `Reg<ADC_GAIN_FACTOR_SPEC>`"]
pub type ADC_GAIN_FACTOR = crate::Reg<adc_gain_factor::ADC_GAIN_FACTOR_SPEC>;
#[doc = "ADC Gain Factor"]
pub mod adc_gain_factor;
#[doc = "ADC_OFFSET (r) register accessor: an alias for `Reg<ADC_OFFSET_SPEC>`"]
pub type ADC_OFFSET = crate::Reg<adc_offset::ADC_OFFSET_SPEC>;
#[doc = "ADC Offset"]
pub mod adc_offset;
#[doc = "RESERVED11 (r) register accessor: an alias for `Reg<RESERVED11_SPEC>`"]
pub type RESERVED11 = crate::Reg<reserved11::RESERVED11_SPEC>;
#[doc = "Reserved"]
pub mod reserved11;
#[doc = "RESERVED12 (r) register accessor: an alias for `Reg<RESERVED12_SPEC>`"]
pub type RESERVED12 = crate::Reg<reserved12::RESERVED12_SPEC>;
#[doc = "Reserved"]
pub mod reserved12;
#[doc = "RESERVED13 (r) register accessor: an alias for `Reg<RESERVED13_SPEC>`"]
pub type RESERVED13 = crate::Reg<reserved13::RESERVED13_SPEC>;
#[doc = "Reserved"]
pub mod reserved13;
#[doc = "RESERVED14 (r) register accessor: an alias for `Reg<RESERVED14_SPEC>`"]
pub type RESERVED14 = crate::Reg<reserved14::RESERVED14_SPEC>;
#[doc = "Reserved"]
pub mod reserved14;
#[doc = "RESERVED15 (r) register accessor: an alias for `Reg<RESERVED15_SPEC>`"]
pub type RESERVED15 = crate::Reg<reserved15::RESERVED15_SPEC>;
#[doc = "Reserved"]
pub mod reserved15;
#[doc = "RESERVED16 (r) register accessor: an alias for `Reg<RESERVED16_SPEC>`"]
pub type RESERVED16 = crate::Reg<reserved16::RESERVED16_SPEC>;
#[doc = "Reserved"]
pub mod reserved16;
#[doc = "RESERVED17 (r) register accessor: an alias for `Reg<RESERVED17_SPEC>`"]
pub type RESERVED17 = crate::Reg<reserved17::RESERVED17_SPEC>;
#[doc = "Reserved"]
pub mod reserved17;
#[doc = "RESERVED18 (r) register accessor: an alias for `Reg<RESERVED18_SPEC>`"]
pub type RESERVED18 = crate::Reg<reserved18::RESERVED18_SPEC>;
#[doc = "Reserved"]
pub mod reserved18;
#[doc = "RESERVED19 (r) register accessor: an alias for `Reg<RESERVED19_SPEC>`"]
pub type RESERVED19 = crate::Reg<reserved19::RESERVED19_SPEC>;
#[doc = "Reserved"]
pub mod reserved19;
#[doc = "RESERVED20 (r) register accessor: an alias for `Reg<RESERVED20_SPEC>`"]
pub type RESERVED20 = crate::Reg<reserved20::RESERVED20_SPEC>;
#[doc = "Reserved"]
pub mod reserved20;
#[doc = "RESERVED21 (r) register accessor: an alias for `Reg<RESERVED21_SPEC>`"]
pub type RESERVED21 = crate::Reg<reserved21::RESERVED21_SPEC>;
#[doc = "Reserved"]
pub mod reserved21;
#[doc = "RESERVED22 (r) register accessor: an alias for `Reg<RESERVED22_SPEC>`"]
pub type RESERVED22 = crate::Reg<reserved22::RESERVED22_SPEC>;
#[doc = "Reserved"]
pub mod reserved22;
#[doc = "RESERVED23 (r) register accessor: an alias for `Reg<RESERVED23_SPEC>`"]
pub type RESERVED23 = crate::Reg<reserved23::RESERVED23_SPEC>;
#[doc = "Reserved"]
pub mod reserved23;
#[doc = "RESERVED24 (r) register accessor: an alias for `Reg<RESERVED24_SPEC>`"]
pub type RESERVED24 = crate::Reg<reserved24::RESERVED24_SPEC>;
#[doc = "Reserved"]
pub mod reserved24;
#[doc = "RESERVED25 (r) register accessor: an alias for `Reg<RESERVED25_SPEC>`"]
pub type RESERVED25 = crate::Reg<reserved25::RESERVED25_SPEC>;
#[doc = "Reserved"]
pub mod reserved25;
#[doc = "RESERVED26 (r) register accessor: an alias for `Reg<RESERVED26_SPEC>`"]
pub type RESERVED26 = crate::Reg<reserved26::RESERVED26_SPEC>;
#[doc = "Reserved"]
pub mod reserved26;
#[doc = "ADC14_REF1P2V_TS30C (r) register accessor: an alias for `Reg<ADC14_REF1P2V_TS30C_SPEC>`"]
pub type ADC14_REF1P2V_TS30C = crate::Reg<adc14_ref1p2v_ts30c::ADC14_REF1P2V_TS30C_SPEC>;
#[doc = "ADC14 1.2V Reference Temp. Sensor 30C"]
pub mod adc14_ref1p2v_ts30c;
#[doc = "ADC14_REF1P2V_TS85C (r) register accessor: an alias for `Reg<ADC14_REF1P2V_TS85C_SPEC>`"]
pub type ADC14_REF1P2V_TS85C = crate::Reg<adc14_ref1p2v_ts85c::ADC14_REF1P2V_TS85C_SPEC>;
#[doc = "ADC14 1.2V Reference Temp. Sensor 85C"]
pub mod adc14_ref1p2v_ts85c;
#[doc = "ADC14_REF1P45V_TS30C (r) register accessor: an alias for `Reg<ADC14_REF1P45V_TS30C_SPEC>`"]
pub type ADC14_REF1P45V_TS30C = crate::Reg<adc14_ref1p45v_ts30c::ADC14_REF1P45V_TS30C_SPEC>;
#[doc = "ADC14 1.45V Reference Temp. Sensor 30C"]
pub mod adc14_ref1p45v_ts30c;
#[doc = "ADC14_REF1P45V_TS85C (r) register accessor: an alias for `Reg<ADC14_REF1P45V_TS85C_SPEC>`"]
pub type ADC14_REF1P45V_TS85C = crate::Reg<adc14_ref1p45v_ts85c::ADC14_REF1P45V_TS85C_SPEC>;
#[doc = "ADC14 1.45V Reference Temp. Sensor 85C"]
pub mod adc14_ref1p45v_ts85c;
#[doc = "ADC14_REF2P5V_TS30C (r) register accessor: an alias for `Reg<ADC14_REF2P5V_TS30C_SPEC>`"]
pub type ADC14_REF2P5V_TS30C = crate::Reg<adc14_ref2p5v_ts30c::ADC14_REF2P5V_TS30C_SPEC>;
#[doc = "ADC14 2.5V Reference Temp. Sensor 30C"]
pub mod adc14_ref2p5v_ts30c;
#[doc = "ADC14_REF2P5V_TS85C (r) register accessor: an alias for `Reg<ADC14_REF2P5V_TS85C_SPEC>`"]
pub type ADC14_REF2P5V_TS85C = crate::Reg<adc14_ref2p5v_ts85c::ADC14_REF2P5V_TS85C_SPEC>;
#[doc = "ADC14 2.5V Reference Temp. Sensor 85C"]
pub mod adc14_ref2p5v_ts85c;
#[doc = "REF_CAL_TAG (r) register accessor: an alias for `Reg<REF_CAL_TAG_SPEC>`"]
pub type REF_CAL_TAG = crate::Reg<ref_cal_tag::REF_CAL_TAG_SPEC>;
#[doc = "REF Calibration Tag"]
pub mod ref_cal_tag;
#[doc = "REF_CAL_LEN (r) register accessor: an alias for `Reg<REF_CAL_LEN_SPEC>`"]
pub type REF_CAL_LEN = crate::Reg<ref_cal_len::REF_CAL_LEN_SPEC>;
#[doc = "REF Calibration Length"]
pub mod ref_cal_len;
#[doc = "REF_1P2V (r) register accessor: an alias for `Reg<REF_1P2V_SPEC>`"]
pub type REF_1P2V = crate::Reg<ref_1p2v::REF_1P2V_SPEC>;
#[doc = "REF 1.2V Reference"]
pub mod ref_1p2v;
#[doc = "REF_1P45V (r) register accessor: an alias for `Reg<REF_1P45V_SPEC>`"]
pub type REF_1P45V = crate::Reg<ref_1p45v::REF_1P45V_SPEC>;
#[doc = "REF 1.45V Reference"]
pub mod ref_1p45v;
#[doc = "REF_2P5V (r) register accessor: an alias for `Reg<REF_2P5V_SPEC>`"]
pub type REF_2P5V = crate::Reg<ref_2p5v::REF_2P5V_SPEC>;
#[doc = "REF 2.5V Reference"]
pub mod ref_2p5v;
#[doc = "FLASH_INFO_TAG (r) register accessor: an alias for `Reg<FLASH_INFO_TAG_SPEC>`"]
pub type FLASH_INFO_TAG = crate::Reg<flash_info_tag::FLASH_INFO_TAG_SPEC>;
#[doc = "Flash Info Tag"]
pub mod flash_info_tag;
#[doc = "FLASH_INFO_LEN (r) register accessor: an alias for `Reg<FLASH_INFO_LEN_SPEC>`"]
pub type FLASH_INFO_LEN = crate::Reg<flash_info_len::FLASH_INFO_LEN_SPEC>;
#[doc = "Flash Info Length"]
pub mod flash_info_len;
#[doc = "FLASH_MAX_PROG_PULSES (r) register accessor: an alias for `Reg<FLASH_MAX_PROG_PULSES_SPEC>`"]
pub type FLASH_MAX_PROG_PULSES = crate::Reg<flash_max_prog_pulses::FLASH_MAX_PROG_PULSES_SPEC>;
#[doc = "Flash Maximum Programming Pulses"]
pub mod flash_max_prog_pulses;
#[doc = "FLASH_MAX_ERASE_PULSES (r) register accessor: an alias for `Reg<FLASH_MAX_ERASE_PULSES_SPEC>`"]
pub type FLASH_MAX_ERASE_PULSES = crate::Reg<flash_max_erase_pulses::FLASH_MAX_ERASE_PULSES_SPEC>;
#[doc = "Flash Maximum Erase Pulses"]
pub mod flash_max_erase_pulses;
#[doc = "RANDOM_NUM_TAG (r) register accessor: an alias for `Reg<RANDOM_NUM_TAG_SPEC>`"]
pub type RANDOM_NUM_TAG = crate::Reg<random_num_tag::RANDOM_NUM_TAG_SPEC>;
#[doc = "128-bit Random Number Tag"]
pub mod random_num_tag;
#[doc = "RANDOM_NUM_LEN (r) register accessor: an alias for `Reg<RANDOM_NUM_LEN_SPEC>`"]
pub type RANDOM_NUM_LEN = crate::Reg<random_num_len::RANDOM_NUM_LEN_SPEC>;
#[doc = "128-bit Random Number Length"]
pub mod random_num_len;
#[doc = "RANDOM_NUM_1 (r) register accessor: an alias for `Reg<RANDOM_NUM_1_SPEC>`"]
pub type RANDOM_NUM_1 = crate::Reg<random_num_1::RANDOM_NUM_1_SPEC>;
#[doc = "32-bit Random Number 1"]
pub mod random_num_1;
#[doc = "RANDOM_NUM_2 (r) register accessor: an alias for `Reg<RANDOM_NUM_2_SPEC>`"]
pub type RANDOM_NUM_2 = crate::Reg<random_num_2::RANDOM_NUM_2_SPEC>;
#[doc = "32-bit Random Number 2"]
pub mod random_num_2;
#[doc = "RANDOM_NUM_3 (r) register accessor: an alias for `Reg<RANDOM_NUM_3_SPEC>`"]
pub type RANDOM_NUM_3 = crate::Reg<random_num_3::RANDOM_NUM_3_SPEC>;
#[doc = "32-bit Random Number 3"]
pub mod random_num_3;
#[doc = "RANDOM_NUM_4 (r) register accessor: an alias for `Reg<RANDOM_NUM_4_SPEC>`"]
pub type RANDOM_NUM_4 = crate::Reg<random_num_4::RANDOM_NUM_4_SPEC>;
#[doc = "32-bit Random Number 4"]
pub mod random_num_4;
#[doc = "BSL_CFG_TAG (r) register accessor: an alias for `Reg<BSL_CFG_TAG_SPEC>`"]
pub type BSL_CFG_TAG = crate::Reg<bsl_cfg_tag::BSL_CFG_TAG_SPEC>;
#[doc = "BSL Configuration Tag"]
pub mod bsl_cfg_tag;
#[doc = "BSL_CFG_LEN (r) register accessor: an alias for `Reg<BSL_CFG_LEN_SPEC>`"]
pub type BSL_CFG_LEN = crate::Reg<bsl_cfg_len::BSL_CFG_LEN_SPEC>;
#[doc = "BSL Configuration Length"]
pub mod bsl_cfg_len;
#[doc = "BSL_PERIPHIF_SEL (r) register accessor: an alias for `Reg<BSL_PERIPHIF_SEL_SPEC>`"]
pub type BSL_PERIPHIF_SEL = crate::Reg<bsl_periphif_sel::BSL_PERIPHIF_SEL_SPEC>;
#[doc = "BSL Peripheral Interface Selection"]
pub mod bsl_periphif_sel;
#[doc = "BSL_PORTIF_CFG_UART (r) register accessor: an alias for `Reg<BSL_PORTIF_CFG_UART_SPEC>`"]
pub type BSL_PORTIF_CFG_UART = crate::Reg<bsl_portif_cfg_uart::BSL_PORTIF_CFG_UART_SPEC>;
#[doc = "BSL Port Interface Configuration for UART"]
pub mod bsl_portif_cfg_uart;
#[doc = "BSL_PORTIF_CFG_SPI (r) register accessor: an alias for `Reg<BSL_PORTIF_CFG_SPI_SPEC>`"]
pub type BSL_PORTIF_CFG_SPI = crate::Reg<bsl_portif_cfg_spi::BSL_PORTIF_CFG_SPI_SPEC>;
#[doc = "BSL Port Interface Configuration for SPI"]
pub mod bsl_portif_cfg_spi;
#[doc = "BSL_PORTIF_CFG_I2C (r) register accessor: an alias for `Reg<BSL_PORTIF_CFG_I2C_SPEC>`"]
pub type BSL_PORTIF_CFG_I2C = crate::Reg<bsl_portif_cfg_i2c::BSL_PORTIF_CFG_I2C_SPEC>;
#[doc = "BSL Port Interface Configuration for I2C"]
pub mod bsl_portif_cfg_i2c;
#[doc = "TLV_END (r) register accessor: an alias for `Reg<TLV_END_SPEC>`"]
pub type TLV_END = crate::Reg<tlv_end::TLV_END_SPEC>;
#[doc = "TLV End Word"]
pub mod tlv_end;
