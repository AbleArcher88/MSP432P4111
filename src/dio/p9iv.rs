#[doc = "Register `P9IV` reader"]
pub struct R(crate::R<P9IV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P9IV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P9IV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P9IV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `P9IV` reader - Port 9 interrupt vector value"]
pub type P9IV_R = crate::FieldReader<u8, P9IV_ENUM_READ_A>;
#[doc = "Port 9 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P9IV_ENUM_READ_A {
    #[doc = "0: No interrupt pending"]
    P9IV_0 = 0,
    #[doc = "2: Interrupt Source: Port 9.0 interrupt; Interrupt Flag: P9IFG0; Interrupt Priority: Highest"]
    P9IV_2 = 2,
    #[doc = "4: Interrupt Source: Port 9.1 interrupt; Interrupt Flag: P9IFG1"]
    P9IV_4 = 4,
    #[doc = "6: Interrupt Source: Port 9.2 interrupt; Interrupt Flag: P9IFG2"]
    P9IV_6 = 6,
    #[doc = "8: Interrupt Source: Port 9.3 interrupt; Interrupt Flag: P9IFG3"]
    P9IV_8 = 8,
    #[doc = "10: Interrupt Source: Port 9.4 interrupt; Interrupt Flag: P9IFG4"]
    P9IV_10 = 10,
    #[doc = "12: Interrupt Source: Port 9.5 interrupt; Interrupt Flag: P9IFG5"]
    P9IV_12 = 12,
    #[doc = "14: Interrupt Source: Port 9.6 interrupt; Interrupt Flag: P9IFG6"]
    P9IV_14 = 14,
    #[doc = "16: Interrupt Source: Port 9.7 interrupt; Interrupt Flag: P9IFG7; Interrupt Priority: Lowest"]
    P9IV_16 = 16,
}
impl From<P9IV_ENUM_READ_A> for u8 {
    #[inline(always)]
    fn from(variant: P9IV_ENUM_READ_A) -> Self {
        variant as _
    }
}
impl P9IV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<P9IV_ENUM_READ_A> {
        match self.bits {
            0 => Some(P9IV_ENUM_READ_A::P9IV_0),
            2 => Some(P9IV_ENUM_READ_A::P9IV_2),
            4 => Some(P9IV_ENUM_READ_A::P9IV_4),
            6 => Some(P9IV_ENUM_READ_A::P9IV_6),
            8 => Some(P9IV_ENUM_READ_A::P9IV_8),
            10 => Some(P9IV_ENUM_READ_A::P9IV_10),
            12 => Some(P9IV_ENUM_READ_A::P9IV_12),
            14 => Some(P9IV_ENUM_READ_A::P9IV_14),
            16 => Some(P9IV_ENUM_READ_A::P9IV_16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `P9IV_0`"]
    #[inline(always)]
    pub fn is_p9iv_0(&self) -> bool {
        *self == P9IV_ENUM_READ_A::P9IV_0
    }
    #[doc = "Checks if the value of the field is `P9IV_2`"]
    #[inline(always)]
    pub fn is_p9iv_2(&self) -> bool {
        *self == P9IV_ENUM_READ_A::P9IV_2
    }
    #[doc = "Checks if the value of the field is `P9IV_4`"]
    #[inline(always)]
    pub fn is_p9iv_4(&self) -> bool {
        *self == P9IV_ENUM_READ_A::P9IV_4
    }
    #[doc = "Checks if the value of the field is `P9IV_6`"]
    #[inline(always)]
    pub fn is_p9iv_6(&self) -> bool {
        *self == P9IV_ENUM_READ_A::P9IV_6
    }
    #[doc = "Checks if the value of the field is `P9IV_8`"]
    #[inline(always)]
    pub fn is_p9iv_8(&self) -> bool {
        *self == P9IV_ENUM_READ_A::P9IV_8
    }
    #[doc = "Checks if the value of the field is `P9IV_10`"]
    #[inline(always)]
    pub fn is_p9iv_10(&self) -> bool {
        *self == P9IV_ENUM_READ_A::P9IV_10
    }
    #[doc = "Checks if the value of the field is `P9IV_12`"]
    #[inline(always)]
    pub fn is_p9iv_12(&self) -> bool {
        *self == P9IV_ENUM_READ_A::P9IV_12
    }
    #[doc = "Checks if the value of the field is `P9IV_14`"]
    #[inline(always)]
    pub fn is_p9iv_14(&self) -> bool {
        *self == P9IV_ENUM_READ_A::P9IV_14
    }
    #[doc = "Checks if the value of the field is `P9IV_16`"]
    #[inline(always)]
    pub fn is_p9iv_16(&self) -> bool {
        *self == P9IV_ENUM_READ_A::P9IV_16
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 9 interrupt vector value"]
    #[inline(always)]
    pub fn p9iv(&self) -> P9IV_R {
        P9IV_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Port 9 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p9iv](index.html) module"]
pub struct P9IV_SPEC;
impl crate::RegisterSpec for P9IV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [p9iv::R](R) reader structure"]
impl crate::Readable for P9IV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets P9IV to value 0"]
impl crate::Resettable for P9IV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
