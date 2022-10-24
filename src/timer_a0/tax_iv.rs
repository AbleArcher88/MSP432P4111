#[doc = "Register `TAxIV` reader"]
pub struct R(crate::R<TAX_IV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAX_IV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAX_IV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAX_IV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TAIV` reader - TimerA interrupt vector value"]
pub type TAIV_R = crate::FieldReader<u16, TAIV_ENUM_READ_A>;
#[doc = "TimerA interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum TAIV_ENUM_READ_A {
    #[doc = "0: No interrupt pending"]
    TAIV_0 = 0,
    #[doc = "2: Interrupt Source: Capture/compare 1; Interrupt Flag: TAxCCR1 CCIFG; Interrupt Priority: Highest"]
    TAIV_2 = 2,
    #[doc = "4: Interrupt Source: Capture/compare 2; Interrupt Flag: TAxCCR2 CCIFG"]
    TAIV_4 = 4,
    #[doc = "6: Interrupt Source: Capture/compare 3; Interrupt Flag: TAxCCR3 CCIFG"]
    TAIV_6 = 6,
    #[doc = "8: Interrupt Source: Capture/compare 4; Interrupt Flag: TAxCCR4 CCIFG"]
    TAIV_8 = 8,
    #[doc = "10: Interrupt Source: Capture/compare 5; Interrupt Flag: TAxCCR5 CCIFG"]
    TAIV_10 = 10,
    #[doc = "12: Interrupt Source: Capture/compare 6; Interrupt Flag: TAxCCR6 CCIFG"]
    TAIV_12 = 12,
    #[doc = "14: Interrupt Source: Timer overflow; Interrupt Flag: TAxCTL TAIFG; Interrupt Priority: Lowest"]
    TAIV_14 = 14,
}
impl From<TAIV_ENUM_READ_A> for u16 {
    #[inline(always)]
    fn from(variant: TAIV_ENUM_READ_A) -> Self {
        variant as _
    }
}
impl TAIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TAIV_ENUM_READ_A> {
        match self.bits {
            0 => Some(TAIV_ENUM_READ_A::TAIV_0),
            2 => Some(TAIV_ENUM_READ_A::TAIV_2),
            4 => Some(TAIV_ENUM_READ_A::TAIV_4),
            6 => Some(TAIV_ENUM_READ_A::TAIV_6),
            8 => Some(TAIV_ENUM_READ_A::TAIV_8),
            10 => Some(TAIV_ENUM_READ_A::TAIV_10),
            12 => Some(TAIV_ENUM_READ_A::TAIV_12),
            14 => Some(TAIV_ENUM_READ_A::TAIV_14),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TAIV_0`"]
    #[inline(always)]
    pub fn is_taiv_0(&self) -> bool {
        *self == TAIV_ENUM_READ_A::TAIV_0
    }
    #[doc = "Checks if the value of the field is `TAIV_2`"]
    #[inline(always)]
    pub fn is_taiv_2(&self) -> bool {
        *self == TAIV_ENUM_READ_A::TAIV_2
    }
    #[doc = "Checks if the value of the field is `TAIV_4`"]
    #[inline(always)]
    pub fn is_taiv_4(&self) -> bool {
        *self == TAIV_ENUM_READ_A::TAIV_4
    }
    #[doc = "Checks if the value of the field is `TAIV_6`"]
    #[inline(always)]
    pub fn is_taiv_6(&self) -> bool {
        *self == TAIV_ENUM_READ_A::TAIV_6
    }
    #[doc = "Checks if the value of the field is `TAIV_8`"]
    #[inline(always)]
    pub fn is_taiv_8(&self) -> bool {
        *self == TAIV_ENUM_READ_A::TAIV_8
    }
    #[doc = "Checks if the value of the field is `TAIV_10`"]
    #[inline(always)]
    pub fn is_taiv_10(&self) -> bool {
        *self == TAIV_ENUM_READ_A::TAIV_10
    }
    #[doc = "Checks if the value of the field is `TAIV_12`"]
    #[inline(always)]
    pub fn is_taiv_12(&self) -> bool {
        *self == TAIV_ENUM_READ_A::TAIV_12
    }
    #[doc = "Checks if the value of the field is `TAIV_14`"]
    #[inline(always)]
    pub fn is_taiv_14(&self) -> bool {
        *self == TAIV_ENUM_READ_A::TAIV_14
    }
}
impl R {
    #[doc = "Bits 0:15 - TimerA interrupt vector value"]
    #[inline(always)]
    pub fn taiv(&self) -> TAIV_R {
        TAIV_R::new(self.bits)
    }
}
#[doc = "TimerAx Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tax_iv](index.html) module"]
pub struct TAX_IV_SPEC;
impl crate::RegisterSpec for TAX_IV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tax_iv::R](R) reader structure"]
impl crate::Readable for TAX_IV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TAxIV to value 0"]
impl crate::Resettable for TAX_IV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
