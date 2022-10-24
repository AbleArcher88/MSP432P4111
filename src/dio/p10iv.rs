#[doc = "Register `P10IV` reader"]
pub struct R(crate::R<P10IV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P10IV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P10IV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P10IV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `P10IV` reader - Port 10 interrupt vector value"]
pub type P10IV_R = crate::FieldReader<u8, P10IV_ENUM_READ_A>;
#[doc = "Port 10 interrupt vector value"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P10IV_ENUM_READ_A {
    #[doc = "0: No interrupt pending"]
    P10IV_0 = 0,
    #[doc = "2: Interrupt Source: Port 10.0 interrupt; Interrupt Flag: P10IFG0; Interrupt Priority: Highest"]
    P10IV_2 = 2,
    #[doc = "4: Interrupt Source: Port 10.1 interrupt; Interrupt Flag: P10IFG1"]
    P10IV_4 = 4,
    #[doc = "6: Interrupt Source: Port 10.2 interrupt; Interrupt Flag: P10IFG2"]
    P10IV_6 = 6,
    #[doc = "8: Interrupt Source: Port 10.3 interrupt; Interrupt Flag: P10IFG3"]
    P10IV_8 = 8,
    #[doc = "10: Interrupt Source: Port 10.4 interrupt; Interrupt Flag: P10IFG4"]
    P10IV_10 = 10,
    #[doc = "12: Interrupt Source: Port 10.5 interrupt; Interrupt Flag: P10IFG5"]
    P10IV_12 = 12,
    #[doc = "14: Interrupt Source: Port 10.6 interrupt; Interrupt Flag: P10IFG6"]
    P10IV_14 = 14,
    #[doc = "16: Interrupt Source: Port 10.7 interrupt; Interrupt Flag: P10IFG7; Interrupt Priority: Lowest"]
    P10IV_16 = 16,
}
impl From<P10IV_ENUM_READ_A> for u8 {
    #[inline(always)]
    fn from(variant: P10IV_ENUM_READ_A) -> Self {
        variant as _
    }
}
impl P10IV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<P10IV_ENUM_READ_A> {
        match self.bits {
            0 => Some(P10IV_ENUM_READ_A::P10IV_0),
            2 => Some(P10IV_ENUM_READ_A::P10IV_2),
            4 => Some(P10IV_ENUM_READ_A::P10IV_4),
            6 => Some(P10IV_ENUM_READ_A::P10IV_6),
            8 => Some(P10IV_ENUM_READ_A::P10IV_8),
            10 => Some(P10IV_ENUM_READ_A::P10IV_10),
            12 => Some(P10IV_ENUM_READ_A::P10IV_12),
            14 => Some(P10IV_ENUM_READ_A::P10IV_14),
            16 => Some(P10IV_ENUM_READ_A::P10IV_16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `P10IV_0`"]
    #[inline(always)]
    pub fn is_p10iv_0(&self) -> bool {
        *self == P10IV_ENUM_READ_A::P10IV_0
    }
    #[doc = "Checks if the value of the field is `P10IV_2`"]
    #[inline(always)]
    pub fn is_p10iv_2(&self) -> bool {
        *self == P10IV_ENUM_READ_A::P10IV_2
    }
    #[doc = "Checks if the value of the field is `P10IV_4`"]
    #[inline(always)]
    pub fn is_p10iv_4(&self) -> bool {
        *self == P10IV_ENUM_READ_A::P10IV_4
    }
    #[doc = "Checks if the value of the field is `P10IV_6`"]
    #[inline(always)]
    pub fn is_p10iv_6(&self) -> bool {
        *self == P10IV_ENUM_READ_A::P10IV_6
    }
    #[doc = "Checks if the value of the field is `P10IV_8`"]
    #[inline(always)]
    pub fn is_p10iv_8(&self) -> bool {
        *self == P10IV_ENUM_READ_A::P10IV_8
    }
    #[doc = "Checks if the value of the field is `P10IV_10`"]
    #[inline(always)]
    pub fn is_p10iv_10(&self) -> bool {
        *self == P10IV_ENUM_READ_A::P10IV_10
    }
    #[doc = "Checks if the value of the field is `P10IV_12`"]
    #[inline(always)]
    pub fn is_p10iv_12(&self) -> bool {
        *self == P10IV_ENUM_READ_A::P10IV_12
    }
    #[doc = "Checks if the value of the field is `P10IV_14`"]
    #[inline(always)]
    pub fn is_p10iv_14(&self) -> bool {
        *self == P10IV_ENUM_READ_A::P10IV_14
    }
    #[doc = "Checks if the value of the field is `P10IV_16`"]
    #[inline(always)]
    pub fn is_p10iv_16(&self) -> bool {
        *self == P10IV_ENUM_READ_A::P10IV_16
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 10 interrupt vector value"]
    #[inline(always)]
    pub fn p10iv(&self) -> P10IV_R {
        P10IV_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Port 10 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p10iv](index.html) module"]
pub struct P10IV_SPEC;
impl crate::RegisterSpec for P10IV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [p10iv::R](R) reader structure"]
impl crate::Readable for P10IV_SPEC {
    type Reader = R;
}
