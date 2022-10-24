#[doc = "Register `SYS_SRAM_STAT` reader"]
pub struct R(crate::R<SYS_SRAM_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_SRAM_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_SRAM_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_SRAM_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BNKEN_RDY` reader - When 1, indicates SRAM is ready for access and banks can be enabled/disabled."]
pub type BNKEN_RDY_R = crate::BitReader<BNKEN_RDY_ENUM_READ_A>;
#[doc = "When 1, indicates SRAM is ready for access and banks can be enabled/disabled."]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNKEN_RDY_ENUM_READ_A {
    #[doc = "0: SRAM is not ready for accesses. Banks are undergoing an enable or disable sequence, and reads or writes to SRAM are stalled until the banks are ready."]
    BNKEN_RDY_0 = 0,
    #[doc = "1: SRAM is ready for accesses. All SRAM banks are enabled/disabled according to values of registers SYS_SRAM_BANKEN_CTLx (x=0,1,2,3)"]
    BNKEN_RDY_1 = 1,
}
impl From<BNKEN_RDY_ENUM_READ_A> for bool {
    #[inline(always)]
    fn from(variant: BNKEN_RDY_ENUM_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl BNKEN_RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNKEN_RDY_ENUM_READ_A {
        match self.bits {
            false => BNKEN_RDY_ENUM_READ_A::BNKEN_RDY_0,
            true => BNKEN_RDY_ENUM_READ_A::BNKEN_RDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNKEN_RDY_0`"]
    #[inline(always)]
    pub fn is_bnken_rdy_0(&self) -> bool {
        *self == BNKEN_RDY_ENUM_READ_A::BNKEN_RDY_0
    }
    #[doc = "Checks if the value of the field is `BNKEN_RDY_1`"]
    #[inline(always)]
    pub fn is_bnken_rdy_1(&self) -> bool {
        *self == BNKEN_RDY_ENUM_READ_A::BNKEN_RDY_1
    }
}
#[doc = "Field `BLKRET_RDY` reader - When 1, indicates SRAM is ready for access and blocks can be enabled/disabled for retention."]
pub type BLKRET_RDY_R = crate::BitReader<BLKRET_RDY_ENUM_READ_A>;
#[doc = "When 1, indicates SRAM is ready for access and blocks can be enabled/disabled for retention."]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLKRET_RDY_ENUM_READ_A {
    #[doc = "0: SRAM blocks are being set up for retention. Entry into LPM3, LPM4 should not be attempted until this bit is set to 1"]
    BLKRET_RDY_0 = 0,
    #[doc = "1: SRAM is ready for accesses. All SRAM blocks are enabled/disabled for retention according to values of registers SYS_SRAM_BLKRET_CTLx (x = 0,1,2,3)"]
    BLKRET_RDY_1 = 1,
}
impl From<BLKRET_RDY_ENUM_READ_A> for bool {
    #[inline(always)]
    fn from(variant: BLKRET_RDY_ENUM_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl BLKRET_RDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLKRET_RDY_ENUM_READ_A {
        match self.bits {
            false => BLKRET_RDY_ENUM_READ_A::BLKRET_RDY_0,
            true => BLKRET_RDY_ENUM_READ_A::BLKRET_RDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLKRET_RDY_0`"]
    #[inline(always)]
    pub fn is_blkret_rdy_0(&self) -> bool {
        *self == BLKRET_RDY_ENUM_READ_A::BLKRET_RDY_0
    }
    #[doc = "Checks if the value of the field is `BLKRET_RDY_1`"]
    #[inline(always)]
    pub fn is_blkret_rdy_1(&self) -> bool {
        *self == BLKRET_RDY_ENUM_READ_A::BLKRET_RDY_1
    }
}
impl R {
    #[doc = "Bit 0 - When 1, indicates SRAM is ready for access and banks can be enabled/disabled."]
    #[inline(always)]
    pub fn bnken_rdy(&self) -> BNKEN_RDY_R {
        BNKEN_RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, indicates SRAM is ready for access and blocks can be enabled/disabled for retention."]
    #[inline(always)]
    pub fn blkret_rdy(&self) -> BLKRET_RDY_R {
        BLKRET_RDY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "SRAM Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_sram_stat](index.html) module"]
pub struct SYS_SRAM_STAT_SPEC;
impl crate::RegisterSpec for SYS_SRAM_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_sram_stat::R](R) reader structure"]
impl crate::Readable for SYS_SRAM_STAT_SPEC {
    type Reader = R;
}
