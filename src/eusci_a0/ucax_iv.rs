#[doc = "Register `UCAxIV` reader"]
pub struct R(crate::R<UCAX_IV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCAX_IV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCAX_IV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCAX_IV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UCIV` reader - eUSCI_A interrupt vector value"]
pub type UCIV_R = crate::FieldReader<u16, UCIV_ENUM_READ_A>;
#[doc = "eUSCI_A interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum UCIV_ENUM_READ_A {
    #[doc = "0: No interrupt pending"]
    UCIV_0 = 0,
    #[doc = "2: Interrupt Source: Receive buffer full; Interrupt Flag: UCRXIFG; Interrupt Priority: Highest"]
    UCIV_2 = 2,
    #[doc = "4: Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG"]
    UCIV_4 = 4,
    #[doc = "6: Interrupt Source: Start bit received; Interrupt Flag: UCSTTIFG"]
    UCIV_6 = 6,
    #[doc = "8: Interrupt Source: Transmit complete; Interrupt Flag: UCTXCPTIFG; Interrupt Priority: Lowest"]
    UCIV_8 = 8,
}
impl From<UCIV_ENUM_READ_A> for u16 {
    #[inline(always)]
    fn from(variant: UCIV_ENUM_READ_A) -> Self {
        variant as _
    }
}
impl UCIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UCIV_ENUM_READ_A> {
        match self.bits {
            0 => Some(UCIV_ENUM_READ_A::UCIV_0),
            2 => Some(UCIV_ENUM_READ_A::UCIV_2),
            4 => Some(UCIV_ENUM_READ_A::UCIV_4),
            6 => Some(UCIV_ENUM_READ_A::UCIV_6),
            8 => Some(UCIV_ENUM_READ_A::UCIV_8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UCIV_0`"]
    #[inline(always)]
    pub fn is_uciv_0(&self) -> bool {
        *self == UCIV_ENUM_READ_A::UCIV_0
    }
    #[doc = "Checks if the value of the field is `UCIV_2`"]
    #[inline(always)]
    pub fn is_uciv_2(&self) -> bool {
        *self == UCIV_ENUM_READ_A::UCIV_2
    }
    #[doc = "Checks if the value of the field is `UCIV_4`"]
    #[inline(always)]
    pub fn is_uciv_4(&self) -> bool {
        *self == UCIV_ENUM_READ_A::UCIV_4
    }
    #[doc = "Checks if the value of the field is `UCIV_6`"]
    #[inline(always)]
    pub fn is_uciv_6(&self) -> bool {
        *self == UCIV_ENUM_READ_A::UCIV_6
    }
    #[doc = "Checks if the value of the field is `UCIV_8`"]
    #[inline(always)]
    pub fn is_uciv_8(&self) -> bool {
        *self == UCIV_ENUM_READ_A::UCIV_8
    }
}
impl R {
    #[doc = "Bits 0:15 - eUSCI_A interrupt vector value"]
    #[inline(always)]
    pub fn uciv(&self) -> UCIV_R {
        UCIV_R::new(self.bits)
    }
}
#[doc = "eUSCI_Ax Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucax_iv](index.html) module"]
pub struct UCAX_IV_SPEC;
impl crate::RegisterSpec for UCAX_IV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucax_iv::R](R) reader structure"]
impl crate::Readable for UCAX_IV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UCAxIV to value 0"]
impl crate::Resettable for UCAX_IV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
