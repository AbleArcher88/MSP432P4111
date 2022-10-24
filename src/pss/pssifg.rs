#[doc = "Register `PSSIFG` reader"]
pub struct R(crate::R<PSSIFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSSIFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSSIFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSSIFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SVSMHIFG` reader - High-side SVSM interrupt flag"]
pub type SVSMHIFG_R = crate::BitReader<SVSMHIFG_ENUM_READ_A>;
#[doc = "High-side SVSM interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVSMHIFG_ENUM_READ_A {
    #[doc = "0: No interrupt pending"]
    SVSMHIFG_0 = 0,
    #[doc = "1: Interrupt due to SVSMH"]
    SVSMHIFG_1 = 1,
}
impl From<SVSMHIFG_ENUM_READ_A> for bool {
    #[inline(always)]
    fn from(variant: SVSMHIFG_ENUM_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl SVSMHIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVSMHIFG_ENUM_READ_A {
        match self.bits {
            false => SVSMHIFG_ENUM_READ_A::SVSMHIFG_0,
            true => SVSMHIFG_ENUM_READ_A::SVSMHIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `SVSMHIFG_0`"]
    #[inline(always)]
    pub fn is_svsmhifg_0(&self) -> bool {
        *self == SVSMHIFG_ENUM_READ_A::SVSMHIFG_0
    }
    #[doc = "Checks if the value of the field is `SVSMHIFG_1`"]
    #[inline(always)]
    pub fn is_svsmhifg_1(&self) -> bool {
        *self == SVSMHIFG_ENUM_READ_A::SVSMHIFG_1
    }
}
impl R {
    #[doc = "Bit 1 - High-side SVSM interrupt flag"]
    #[inline(always)]
    pub fn svsmhifg(&self) -> SVSMHIFG_R {
        SVSMHIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pssifg](index.html) module"]
pub struct PSSIFG_SPEC;
impl crate::RegisterSpec for PSSIFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pssifg::R](R) reader structure"]
impl crate::Readable for PSSIFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PSSIFG to value 0"]
impl crate::Resettable for PSSIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
