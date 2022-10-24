#[doc = "Register `P4IV` reader"]
pub struct R(crate::R<P4IV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P4IV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P4IV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P4IV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `P4IV` reader - Port 4 interrupt vector value"]
pub type P4IV_R = crate::FieldReader<u8, P4IV_ENUM_READ_A>;
#[doc = "Port 4 interrupt vector value"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum P4IV_ENUM_READ_A {
    #[doc = "0: No interrupt pending"]
    P4IV_0 = 0,
    #[doc = "2: Interrupt Source: Port 4.0 interrupt; Interrupt Flag: P4IFG0; Interrupt Priority: Highest"]
    P4IV_2 = 2,
    #[doc = "4: Interrupt Source: Port 4.1 interrupt; Interrupt Flag: P4IFG1"]
    P4IV_4 = 4,
    #[doc = "6: Interrupt Source: Port 4.2 interrupt; Interrupt Flag: P4IFG2"]
    P4IV_6 = 6,
    #[doc = "8: Interrupt Source: Port 4.3 interrupt; Interrupt Flag: P4IFG3"]
    P4IV_8 = 8,
    #[doc = "10: Interrupt Source: Port 4.4 interrupt; Interrupt Flag: P4IFG4"]
    P4IV_10 = 10,
    #[doc = "12: Interrupt Source: Port 4.5 interrupt; Interrupt Flag: P4IFG5"]
    P4IV_12 = 12,
    #[doc = "14: Interrupt Source: Port 4.6 interrupt; Interrupt Flag: P4IFG6"]
    P4IV_14 = 14,
    #[doc = "16: Interrupt Source: Port 4.7 interrupt; Interrupt Flag: P4IFG7; Interrupt Priority: Lowest"]
    P4IV_16 = 16,
}
impl From<P4IV_ENUM_READ_A> for u8 {
    #[inline(always)]
    fn from(variant: P4IV_ENUM_READ_A) -> Self {
        variant as _
    }
}
impl P4IV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<P4IV_ENUM_READ_A> {
        match self.bits {
            0 => Some(P4IV_ENUM_READ_A::P4IV_0),
            2 => Some(P4IV_ENUM_READ_A::P4IV_2),
            4 => Some(P4IV_ENUM_READ_A::P4IV_4),
            6 => Some(P4IV_ENUM_READ_A::P4IV_6),
            8 => Some(P4IV_ENUM_READ_A::P4IV_8),
            10 => Some(P4IV_ENUM_READ_A::P4IV_10),
            12 => Some(P4IV_ENUM_READ_A::P4IV_12),
            14 => Some(P4IV_ENUM_READ_A::P4IV_14),
            16 => Some(P4IV_ENUM_READ_A::P4IV_16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `P4IV_0`"]
    #[inline(always)]
    pub fn is_p4iv_0(&self) -> bool {
        *self == P4IV_ENUM_READ_A::P4IV_0
    }
    #[doc = "Checks if the value of the field is `P4IV_2`"]
    #[inline(always)]
    pub fn is_p4iv_2(&self) -> bool {
        *self == P4IV_ENUM_READ_A::P4IV_2
    }
    #[doc = "Checks if the value of the field is `P4IV_4`"]
    #[inline(always)]
    pub fn is_p4iv_4(&self) -> bool {
        *self == P4IV_ENUM_READ_A::P4IV_4
    }
    #[doc = "Checks if the value of the field is `P4IV_6`"]
    #[inline(always)]
    pub fn is_p4iv_6(&self) -> bool {
        *self == P4IV_ENUM_READ_A::P4IV_6
    }
    #[doc = "Checks if the value of the field is `P4IV_8`"]
    #[inline(always)]
    pub fn is_p4iv_8(&self) -> bool {
        *self == P4IV_ENUM_READ_A::P4IV_8
    }
    #[doc = "Checks if the value of the field is `P4IV_10`"]
    #[inline(always)]
    pub fn is_p4iv_10(&self) -> bool {
        *self == P4IV_ENUM_READ_A::P4IV_10
    }
    #[doc = "Checks if the value of the field is `P4IV_12`"]
    #[inline(always)]
    pub fn is_p4iv_12(&self) -> bool {
        *self == P4IV_ENUM_READ_A::P4IV_12
    }
    #[doc = "Checks if the value of the field is `P4IV_14`"]
    #[inline(always)]
    pub fn is_p4iv_14(&self) -> bool {
        *self == P4IV_ENUM_READ_A::P4IV_14
    }
    #[doc = "Checks if the value of the field is `P4IV_16`"]
    #[inline(always)]
    pub fn is_p4iv_16(&self) -> bool {
        *self == P4IV_ENUM_READ_A::P4IV_16
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 4 interrupt vector value"]
    #[inline(always)]
    pub fn p4iv(&self) -> P4IV_R {
        P4IV_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Port 4 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p4iv](index.html) module"]
pub struct P4IV_SPEC;
impl crate::RegisterSpec for P4IV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [p4iv::R](R) reader structure"]
impl crate::Readable for P4IV_SPEC {
    type Reader = R;
}
