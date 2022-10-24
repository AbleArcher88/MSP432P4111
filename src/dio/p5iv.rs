#[doc = "Register `P5IV` reader"]
pub struct R(crate::R<P5IV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P5IV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P5IV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P5IV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `P5IV` reader - Port 5 interrupt vector value"]
pub type P5IV_R = crate::FieldReader<u8, P5IV_ENUM_READ_A>;
#[doc = "Port 5 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P5IV_ENUM_READ_A {
    #[doc = "0: No interrupt pending"]
    P5IV_0 = 0,
    #[doc = "2: Interrupt Source: Port 5.0 interrupt; Interrupt Flag: P5IFG0; Interrupt Priority: Highest"]
    P5IV_2 = 2,
    #[doc = "4: Interrupt Source: Port 5.1 interrupt; Interrupt Flag: P5IFG1"]
    P5IV_4 = 4,
    #[doc = "6: Interrupt Source: Port 5.2 interrupt; Interrupt Flag: P5IFG2"]
    P5IV_6 = 6,
    #[doc = "8: Interrupt Source: Port 5.3 interrupt; Interrupt Flag: P5IFG3"]
    P5IV_8 = 8,
    #[doc = "10: Interrupt Source: Port 5.4 interrupt; Interrupt Flag: P5IFG4"]
    P5IV_10 = 10,
    #[doc = "12: Interrupt Source: Port 5.5 interrupt; Interrupt Flag: P5IFG5"]
    P5IV_12 = 12,
    #[doc = "14: Interrupt Source: Port 5.6 interrupt; Interrupt Flag: P5IFG6"]
    P5IV_14 = 14,
    #[doc = "16: Interrupt Source: Port 5.7 interrupt; Interrupt Flag: P5IFG7; Interrupt Priority: Lowest"]
    P5IV_16 = 16,
}
impl From<P5IV_ENUM_READ_A> for u8 {
    #[inline(always)]
    fn from(variant: P5IV_ENUM_READ_A) -> Self {
        variant as _
    }
}
impl P5IV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<P5IV_ENUM_READ_A> {
        match self.bits {
            0 => Some(P5IV_ENUM_READ_A::P5IV_0),
            2 => Some(P5IV_ENUM_READ_A::P5IV_2),
            4 => Some(P5IV_ENUM_READ_A::P5IV_4),
            6 => Some(P5IV_ENUM_READ_A::P5IV_6),
            8 => Some(P5IV_ENUM_READ_A::P5IV_8),
            10 => Some(P5IV_ENUM_READ_A::P5IV_10),
            12 => Some(P5IV_ENUM_READ_A::P5IV_12),
            14 => Some(P5IV_ENUM_READ_A::P5IV_14),
            16 => Some(P5IV_ENUM_READ_A::P5IV_16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `P5IV_0`"]
    #[inline(always)]
    pub fn is_p5iv_0(&self) -> bool {
        *self == P5IV_ENUM_READ_A::P5IV_0
    }
    #[doc = "Checks if the value of the field is `P5IV_2`"]
    #[inline(always)]
    pub fn is_p5iv_2(&self) -> bool {
        *self == P5IV_ENUM_READ_A::P5IV_2
    }
    #[doc = "Checks if the value of the field is `P5IV_4`"]
    #[inline(always)]
    pub fn is_p5iv_4(&self) -> bool {
        *self == P5IV_ENUM_READ_A::P5IV_4
    }
    #[doc = "Checks if the value of the field is `P5IV_6`"]
    #[inline(always)]
    pub fn is_p5iv_6(&self) -> bool {
        *self == P5IV_ENUM_READ_A::P5IV_6
    }
    #[doc = "Checks if the value of the field is `P5IV_8`"]
    #[inline(always)]
    pub fn is_p5iv_8(&self) -> bool {
        *self == P5IV_ENUM_READ_A::P5IV_8
    }
    #[doc = "Checks if the value of the field is `P5IV_10`"]
    #[inline(always)]
    pub fn is_p5iv_10(&self) -> bool {
        *self == P5IV_ENUM_READ_A::P5IV_10
    }
    #[doc = "Checks if the value of the field is `P5IV_12`"]
    #[inline(always)]
    pub fn is_p5iv_12(&self) -> bool {
        *self == P5IV_ENUM_READ_A::P5IV_12
    }
    #[doc = "Checks if the value of the field is `P5IV_14`"]
    #[inline(always)]
    pub fn is_p5iv_14(&self) -> bool {
        *self == P5IV_ENUM_READ_A::P5IV_14
    }
    #[doc = "Checks if the value of the field is `P5IV_16`"]
    #[inline(always)]
    pub fn is_p5iv_16(&self) -> bool {
        *self == P5IV_ENUM_READ_A::P5IV_16
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 5 interrupt vector value"]
    #[inline(always)]
    pub fn p5iv(&self) -> P5IV_R {
        P5IV_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Port 5 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p5iv](index.html) module"]
pub struct P5IV_SPEC;
impl crate::RegisterSpec for P5IV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [p5iv::R](R) reader structure"]
impl crate::Readable for P5IV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets P5IV to value 0"]
impl crate::Resettable for P5IV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
