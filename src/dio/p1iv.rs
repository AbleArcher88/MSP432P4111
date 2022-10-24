#[doc = "Register `P1IV` reader"]
pub struct R(crate::R<P1IV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P1IV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P1IV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P1IV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `P1IV` reader - Port 1 interrupt vector value"]
pub type P1IV_R = crate::FieldReader<u8, P1IV_ENUM_READ_A>;
#[doc = "Port 1 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P1IV_ENUM_READ_A {
    #[doc = "0: No interrupt pending"]
    P1IV_0 = 0,
    #[doc = "2: Interrupt Source: Port 1.0 interrupt; Interrupt Flag: P1IFG0; Interrupt Priority: Highest"]
    P1IV_2 = 2,
    #[doc = "4: Interrupt Source: Port 1.1 interrupt; Interrupt Flag: P1IFG1"]
    P1IV_4 = 4,
    #[doc = "6: Interrupt Source: Port 1.2 interrupt; Interrupt Flag: P1IFG2"]
    P1IV_6 = 6,
    #[doc = "8: Interrupt Source: Port 1.3 interrupt; Interrupt Flag: P1IFG3"]
    P1IV_8 = 8,
    #[doc = "10: Interrupt Source: Port 1.4 interrupt; Interrupt Flag: P1IFG4"]
    P1IV_10 = 10,
    #[doc = "12: Interrupt Source: Port 1.5 interrupt; Interrupt Flag: P1IFG5"]
    P1IV_12 = 12,
    #[doc = "14: Interrupt Source: Port 1.6 interrupt; Interrupt Flag: P1IFG6"]
    P1IV_14 = 14,
    #[doc = "16: Interrupt Source: Port 1.7 interrupt; Interrupt Flag: P1IFG7; Interrupt Priority: Lowest"]
    P1IV_16 = 16,
}
impl From<P1IV_ENUM_READ_A> for u8 {
    #[inline(always)]
    fn from(variant: P1IV_ENUM_READ_A) -> Self {
        variant as _
    }
}
impl P1IV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<P1IV_ENUM_READ_A> {
        match self.bits {
            0 => Some(P1IV_ENUM_READ_A::P1IV_0),
            2 => Some(P1IV_ENUM_READ_A::P1IV_2),
            4 => Some(P1IV_ENUM_READ_A::P1IV_4),
            6 => Some(P1IV_ENUM_READ_A::P1IV_6),
            8 => Some(P1IV_ENUM_READ_A::P1IV_8),
            10 => Some(P1IV_ENUM_READ_A::P1IV_10),
            12 => Some(P1IV_ENUM_READ_A::P1IV_12),
            14 => Some(P1IV_ENUM_READ_A::P1IV_14),
            16 => Some(P1IV_ENUM_READ_A::P1IV_16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `P1IV_0`"]
    #[inline(always)]
    pub fn is_p1iv_0(&self) -> bool {
        *self == P1IV_ENUM_READ_A::P1IV_0
    }
    #[doc = "Checks if the value of the field is `P1IV_2`"]
    #[inline(always)]
    pub fn is_p1iv_2(&self) -> bool {
        *self == P1IV_ENUM_READ_A::P1IV_2
    }
    #[doc = "Checks if the value of the field is `P1IV_4`"]
    #[inline(always)]
    pub fn is_p1iv_4(&self) -> bool {
        *self == P1IV_ENUM_READ_A::P1IV_4
    }
    #[doc = "Checks if the value of the field is `P1IV_6`"]
    #[inline(always)]
    pub fn is_p1iv_6(&self) -> bool {
        *self == P1IV_ENUM_READ_A::P1IV_6
    }
    #[doc = "Checks if the value of the field is `P1IV_8`"]
    #[inline(always)]
    pub fn is_p1iv_8(&self) -> bool {
        *self == P1IV_ENUM_READ_A::P1IV_8
    }
    #[doc = "Checks if the value of the field is `P1IV_10`"]
    #[inline(always)]
    pub fn is_p1iv_10(&self) -> bool {
        *self == P1IV_ENUM_READ_A::P1IV_10
    }
    #[doc = "Checks if the value of the field is `P1IV_12`"]
    #[inline(always)]
    pub fn is_p1iv_12(&self) -> bool {
        *self == P1IV_ENUM_READ_A::P1IV_12
    }
    #[doc = "Checks if the value of the field is `P1IV_14`"]
    #[inline(always)]
    pub fn is_p1iv_14(&self) -> bool {
        *self == P1IV_ENUM_READ_A::P1IV_14
    }
    #[doc = "Checks if the value of the field is `P1IV_16`"]
    #[inline(always)]
    pub fn is_p1iv_16(&self) -> bool {
        *self == P1IV_ENUM_READ_A::P1IV_16
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 1 interrupt vector value"]
    #[inline(always)]
    pub fn p1iv(&self) -> P1IV_R {
        P1IV_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Port 1 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1iv](index.html) module"]
pub struct P1IV_SPEC;
impl crate::RegisterSpec for P1IV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [p1iv::R](R) reader structure"]
impl crate::Readable for P1IV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets P1IV to value 0"]
impl crate::Resettable for P1IV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
