#[doc = "Register `UCBxIV` reader"]
pub struct R(crate::R<UCBX_IV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCBX_IV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCBX_IV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCBX_IV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UCIV` reader - eUSCI_B interrupt vector value"]
pub type UCIV_R = crate::FieldReader<u16, UCIV_ENUM_READ_A>;
#[doc = "eUSCI_B interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum UCIV_ENUM_READ_A {
    #[doc = "0: No interrupt pending"]
    UCIV_0 = 0,
    #[doc = "2: Interrupt Source: Arbitration lost; Interrupt Flag: UCALIFG; Interrupt Priority: Highest"]
    UCIV_2 = 2,
    #[doc = "4: Interrupt Source: Not acknowledgment; Interrupt Flag: UCNACKIFG"]
    UCIV_4 = 4,
    #[doc = "6: Interrupt Source: Start condition received; Interrupt Flag: UCSTTIFG"]
    UCIV_6 = 6,
    #[doc = "8: Interrupt Source: Stop condition received; Interrupt Flag: UCSTPIFG"]
    UCIV_8 = 8,
    #[doc = "10: Interrupt Source: Slave 3 Data received; Interrupt Flag: UCRXIFG3"]
    UCIV_10 = 10,
    #[doc = "12: Interrupt Source: Slave 3 Transmit buffer empty; Interrupt Flag: UCTXIFG3"]
    UCIV_12 = 12,
    #[doc = "14: Interrupt Source: Slave 2 Data received; Interrupt Flag: UCRXIFG2"]
    UCIV_14 = 14,
    #[doc = "16: Interrupt Source: Slave 2 Transmit buffer empty; Interrupt Flag: UCTXIFG2"]
    UCIV_16 = 16,
    #[doc = "18: Interrupt Source: Slave 1 Data received; Interrupt Flag: UCRXIFG1"]
    UCIV_18 = 18,
    #[doc = "20: Interrupt Source: Slave 1 Transmit buffer empty; Interrupt Flag: UCTXIFG1"]
    UCIV_20 = 20,
    #[doc = "22: Interrupt Source: Data received; Interrupt Flag: UCRXIFG0"]
    UCIV_22 = 22,
    #[doc = "24: Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG0"]
    UCIV_24 = 24,
    #[doc = "26: Interrupt Source: Byte counter zero; Interrupt Flag: UCBCNTIFG"]
    UCIV_26 = 26,
    #[doc = "28: Interrupt Source: Clock low timeout; Interrupt Flag: UCCLTOIFG"]
    UCIV_28 = 28,
    #[doc = "30: Interrupt Source: Nineth bit position; Interrupt Flag: UCBIT9IFG; Priority: Lowest"]
    UCIV_30 = 30,
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
            10 => Some(UCIV_ENUM_READ_A::UCIV_10),
            12 => Some(UCIV_ENUM_READ_A::UCIV_12),
            14 => Some(UCIV_ENUM_READ_A::UCIV_14),
            16 => Some(UCIV_ENUM_READ_A::UCIV_16),
            18 => Some(UCIV_ENUM_READ_A::UCIV_18),
            20 => Some(UCIV_ENUM_READ_A::UCIV_20),
            22 => Some(UCIV_ENUM_READ_A::UCIV_22),
            24 => Some(UCIV_ENUM_READ_A::UCIV_24),
            26 => Some(UCIV_ENUM_READ_A::UCIV_26),
            28 => Some(UCIV_ENUM_READ_A::UCIV_28),
            30 => Some(UCIV_ENUM_READ_A::UCIV_30),
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
    #[doc = "Checks if the value of the field is `UCIV_10`"]
    #[inline(always)]
    pub fn is_uciv_10(&self) -> bool {
        *self == UCIV_ENUM_READ_A::UCIV_10
    }
    #[doc = "Checks if the value of the field is `UCIV_12`"]
    #[inline(always)]
    pub fn is_uciv_12(&self) -> bool {
        *self == UCIV_ENUM_READ_A::UCIV_12
    }
    #[doc = "Checks if the value of the field is `UCIV_14`"]
    #[inline(always)]
    pub fn is_uciv_14(&self) -> bool {
        *self == UCIV_ENUM_READ_A::UCIV_14
    }
    #[doc = "Checks if the value of the field is `UCIV_16`"]
    #[inline(always)]
    pub fn is_uciv_16(&self) -> bool {
        *self == UCIV_ENUM_READ_A::UCIV_16
    }
    #[doc = "Checks if the value of the field is `UCIV_18`"]
    #[inline(always)]
    pub fn is_uciv_18(&self) -> bool {
        *self == UCIV_ENUM_READ_A::UCIV_18
    }
    #[doc = "Checks if the value of the field is `UCIV_20`"]
    #[inline(always)]
    pub fn is_uciv_20(&self) -> bool {
        *self == UCIV_ENUM_READ_A::UCIV_20
    }
    #[doc = "Checks if the value of the field is `UCIV_22`"]
    #[inline(always)]
    pub fn is_uciv_22(&self) -> bool {
        *self == UCIV_ENUM_READ_A::UCIV_22
    }
    #[doc = "Checks if the value of the field is `UCIV_24`"]
    #[inline(always)]
    pub fn is_uciv_24(&self) -> bool {
        *self == UCIV_ENUM_READ_A::UCIV_24
    }
    #[doc = "Checks if the value of the field is `UCIV_26`"]
    #[inline(always)]
    pub fn is_uciv_26(&self) -> bool {
        *self == UCIV_ENUM_READ_A::UCIV_26
    }
    #[doc = "Checks if the value of the field is `UCIV_28`"]
    #[inline(always)]
    pub fn is_uciv_28(&self) -> bool {
        *self == UCIV_ENUM_READ_A::UCIV_28
    }
    #[doc = "Checks if the value of the field is `UCIV_30`"]
    #[inline(always)]
    pub fn is_uciv_30(&self) -> bool {
        *self == UCIV_ENUM_READ_A::UCIV_30
    }
}
impl R {
    #[doc = "Bits 0:15 - eUSCI_B interrupt vector value"]
    #[inline(always)]
    pub fn uciv(&self) -> UCIV_R {
        UCIV_R::new(self.bits)
    }
}
#[doc = "eUSCI_Bx Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_iv](index.html) module"]
pub struct UCBX_IV_SPEC;
impl crate::RegisterSpec for UCBX_IV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucbx_iv::R](R) reader structure"]
impl crate::Readable for UCBX_IV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UCBxIV to value 0"]
impl crate::Resettable for UCBX_IV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
