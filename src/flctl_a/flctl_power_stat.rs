#[doc = "Register `FLCTL_POWER_STAT` reader"]
pub struct R(crate::R<FLCTL_POWER_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_POWER_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_POWER_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_POWER_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PSTAT` reader - Flash power status"]
pub type PSTAT_R = crate::FieldReader<u8, PSTAT_A>;
#[doc = "Flash power status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSTAT_A {
    #[doc = "0: Flash IP in power-down mode"]
    PSTAT_0 = 0,
    #[doc = "1: Flash IP Vdd domain power-up in progress"]
    PSTAT_1 = 1,
    #[doc = "2: PSS LDO_GOOD, IREF_OK and VREF_OK check in progress"]
    PSTAT_2 = 2,
    #[doc = "3: Flash IP SAFE_LV check in progress"]
    PSTAT_3 = 3,
    #[doc = "4: Flash IP Active"]
    PSTAT_4 = 4,
    #[doc = "5: Flash IP Active in Low-Frequency Active and Low-Frequency LPM0 modes."]
    PSTAT_5 = 5,
    #[doc = "6: Flash IP in Standby mode"]
    PSTAT_6 = 6,
    #[doc = "7: Flash IP in Current mirror boost state"]
    PSTAT_7 = 7,
}
impl From<PSTAT_A> for u8 {
    #[inline(always)]
    fn from(variant: PSTAT_A) -> Self {
        variant as _
    }
}
impl PSTAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSTAT_A {
        match self.bits {
            0 => PSTAT_A::PSTAT_0,
            1 => PSTAT_A::PSTAT_1,
            2 => PSTAT_A::PSTAT_2,
            3 => PSTAT_A::PSTAT_3,
            4 => PSTAT_A::PSTAT_4,
            5 => PSTAT_A::PSTAT_5,
            6 => PSTAT_A::PSTAT_6,
            7 => PSTAT_A::PSTAT_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PSTAT_0`"]
    #[inline(always)]
    pub fn is_pstat_0(&self) -> bool {
        *self == PSTAT_A::PSTAT_0
    }
    #[doc = "Checks if the value of the field is `PSTAT_1`"]
    #[inline(always)]
    pub fn is_pstat_1(&self) -> bool {
        *self == PSTAT_A::PSTAT_1
    }
    #[doc = "Checks if the value of the field is `PSTAT_2`"]
    #[inline(always)]
    pub fn is_pstat_2(&self) -> bool {
        *self == PSTAT_A::PSTAT_2
    }
    #[doc = "Checks if the value of the field is `PSTAT_3`"]
    #[inline(always)]
    pub fn is_pstat_3(&self) -> bool {
        *self == PSTAT_A::PSTAT_3
    }
    #[doc = "Checks if the value of the field is `PSTAT_4`"]
    #[inline(always)]
    pub fn is_pstat_4(&self) -> bool {
        *self == PSTAT_A::PSTAT_4
    }
    #[doc = "Checks if the value of the field is `PSTAT_5`"]
    #[inline(always)]
    pub fn is_pstat_5(&self) -> bool {
        *self == PSTAT_A::PSTAT_5
    }
    #[doc = "Checks if the value of the field is `PSTAT_6`"]
    #[inline(always)]
    pub fn is_pstat_6(&self) -> bool {
        *self == PSTAT_A::PSTAT_6
    }
    #[doc = "Checks if the value of the field is `PSTAT_7`"]
    #[inline(always)]
    pub fn is_pstat_7(&self) -> bool {
        *self == PSTAT_A::PSTAT_7
    }
}
#[doc = "Field `LDOSTAT` reader - PSS FLDO GOOD status"]
pub type LDOSTAT_R = crate::BitReader<LDOSTAT_A>;
#[doc = "PSS FLDO GOOD status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDOSTAT_A {
    #[doc = "0: FLDO not GOOD"]
    LDOSTAT_0 = 0,
    #[doc = "1: FLDO GOOD"]
    LDOSTAT_1 = 1,
}
impl From<LDOSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: LDOSTAT_A) -> Self {
        variant as u8 != 0
    }
}
impl LDOSTAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDOSTAT_A {
        match self.bits {
            false => LDOSTAT_A::LDOSTAT_0,
            true => LDOSTAT_A::LDOSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `LDOSTAT_0`"]
    #[inline(always)]
    pub fn is_ldostat_0(&self) -> bool {
        *self == LDOSTAT_A::LDOSTAT_0
    }
    #[doc = "Checks if the value of the field is `LDOSTAT_1`"]
    #[inline(always)]
    pub fn is_ldostat_1(&self) -> bool {
        *self == LDOSTAT_A::LDOSTAT_1
    }
}
#[doc = "Field `VREFSTAT` reader - PSS VREF stable status"]
pub type VREFSTAT_R = crate::BitReader<VREFSTAT_A>;
#[doc = "PSS VREF stable status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFSTAT_A {
    #[doc = "0: Flash LDO not stable"]
    VREFSTAT_0 = 0,
    #[doc = "1: Flash LDO stable"]
    VREFSTAT_1 = 1,
}
impl From<VREFSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: VREFSTAT_A) -> Self {
        variant as u8 != 0
    }
}
impl VREFSTAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFSTAT_A {
        match self.bits {
            false => VREFSTAT_A::VREFSTAT_0,
            true => VREFSTAT_A::VREFSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `VREFSTAT_0`"]
    #[inline(always)]
    pub fn is_vrefstat_0(&self) -> bool {
        *self == VREFSTAT_A::VREFSTAT_0
    }
    #[doc = "Checks if the value of the field is `VREFSTAT_1`"]
    #[inline(always)]
    pub fn is_vrefstat_1(&self) -> bool {
        *self == VREFSTAT_A::VREFSTAT_1
    }
}
#[doc = "Field `IREFSTAT` reader - PSS IREF stable status"]
pub type IREFSTAT_R = crate::BitReader<IREFSTAT_A>;
#[doc = "PSS IREF stable status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IREFSTAT_A {
    #[doc = "0: IREF not stable"]
    IREFSTAT_0 = 0,
    #[doc = "1: IREF stable"]
    IREFSTAT_1 = 1,
}
impl From<IREFSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: IREFSTAT_A) -> Self {
        variant as u8 != 0
    }
}
impl IREFSTAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IREFSTAT_A {
        match self.bits {
            false => IREFSTAT_A::IREFSTAT_0,
            true => IREFSTAT_A::IREFSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `IREFSTAT_0`"]
    #[inline(always)]
    pub fn is_irefstat_0(&self) -> bool {
        *self == IREFSTAT_A::IREFSTAT_0
    }
    #[doc = "Checks if the value of the field is `IREFSTAT_1`"]
    #[inline(always)]
    pub fn is_irefstat_1(&self) -> bool {
        *self == IREFSTAT_A::IREFSTAT_1
    }
}
#[doc = "Field `TRIMSTAT` reader - PSS trim done status"]
pub type TRIMSTAT_R = crate::BitReader<TRIMSTAT_A>;
#[doc = "PSS trim done status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIMSTAT_A {
    #[doc = "0: PSS trim not complete"]
    TRIMSTAT_0 = 0,
    #[doc = "1: PSS trim complete"]
    TRIMSTAT_1 = 1,
}
impl From<TRIMSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: TRIMSTAT_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIMSTAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIMSTAT_A {
        match self.bits {
            false => TRIMSTAT_A::TRIMSTAT_0,
            true => TRIMSTAT_A::TRIMSTAT_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIMSTAT_0`"]
    #[inline(always)]
    pub fn is_trimstat_0(&self) -> bool {
        *self == TRIMSTAT_A::TRIMSTAT_0
    }
    #[doc = "Checks if the value of the field is `TRIMSTAT_1`"]
    #[inline(always)]
    pub fn is_trimstat_1(&self) -> bool {
        *self == TRIMSTAT_A::TRIMSTAT_1
    }
}
#[doc = "Field `RD_2T` reader - Indicates if Flash is being accessed in 2T mode"]
pub type RD_2T_R = crate::BitReader<RD_2T_A>;
#[doc = "Indicates if Flash is being accessed in 2T mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RD_2T_A {
    #[doc = "0: Flash reads are in 1T mode"]
    RD_2T_0 = 0,
    #[doc = "1: Flash reads are in 2T mode"]
    RD_2T_1 = 1,
}
impl From<RD_2T_A> for bool {
    #[inline(always)]
    fn from(variant: RD_2T_A) -> Self {
        variant as u8 != 0
    }
}
impl RD_2T_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RD_2T_A {
        match self.bits {
            false => RD_2T_A::RD_2T_0,
            true => RD_2T_A::RD_2T_1,
        }
    }
    #[doc = "Checks if the value of the field is `RD_2T_0`"]
    #[inline(always)]
    pub fn is_rd_2t_0(&self) -> bool {
        *self == RD_2T_A::RD_2T_0
    }
    #[doc = "Checks if the value of the field is `RD_2T_1`"]
    #[inline(always)]
    pub fn is_rd_2t_1(&self) -> bool {
        *self == RD_2T_A::RD_2T_1
    }
}
impl R {
    #[doc = "Bits 0:2 - Flash power status"]
    #[inline(always)]
    pub fn pstat(&self) -> PSTAT_R {
        PSTAT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - PSS FLDO GOOD status"]
    #[inline(always)]
    pub fn ldostat(&self) -> LDOSTAT_R {
        LDOSTAT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PSS VREF stable status"]
    #[inline(always)]
    pub fn vrefstat(&self) -> VREFSTAT_R {
        VREFSTAT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PSS IREF stable status"]
    #[inline(always)]
    pub fn irefstat(&self) -> IREFSTAT_R {
        IREFSTAT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PSS trim done status"]
    #[inline(always)]
    pub fn trimstat(&self) -> TRIMSTAT_R {
        TRIMSTAT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates if Flash is being accessed in 2T mode"]
    #[inline(always)]
    pub fn rd_2t(&self) -> RD_2T_R {
        RD_2T_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Power Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_power_stat](index.html) module"]
pub struct FLCTL_POWER_STAT_SPEC;
impl crate::RegisterSpec for FLCTL_POWER_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_power_stat::R](R) reader structure"]
impl crate::Readable for FLCTL_POWER_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FLCTL_POWER_STAT to value 0x80"]
impl crate::Resettable for FLCTL_POWER_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
