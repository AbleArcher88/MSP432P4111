#[doc = "Register `ADC14MCTL[%s]` reader"]
pub struct R(crate::R<ADC14MCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC14MCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC14MCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC14MCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC14MCTL[%s]` writer"]
pub struct W(crate::W<ADC14MCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC14MCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ADC14MCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC14MCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC14INCH` reader - Input channel select"]
pub type ADC14INCH_R = crate::FieldReader<u8, ADC14INCH_A>;
#[doc = "Input channel select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC14INCH_A {
    #[doc = "0: If ADC14DIF = 0: A0; If ADC14DIF = 1: Ain+ = A0, Ain- = A1"]
    ADC14INCH_0 = 0,
    #[doc = "1: If ADC14DIF = 0: A1; If ADC14DIF = 1: Ain+ = A0, Ain- = A1"]
    ADC14INCH_1 = 1,
    #[doc = "2: If ADC14DIF = 0: A2; If ADC14DIF = 1: Ain+ = A2, Ain- = A3"]
    ADC14INCH_2 = 2,
    #[doc = "3: If ADC14DIF = 0: A3; If ADC14DIF = 1: Ain+ = A2, Ain- = A3"]
    ADC14INCH_3 = 3,
    #[doc = "4: If ADC14DIF = 0: A4; If ADC14DIF = 1: Ain+ = A4, Ain- = A5"]
    ADC14INCH_4 = 4,
    #[doc = "5: If ADC14DIF = 0: A5; If ADC14DIF = 1: Ain+ = A4, Ain- = A5"]
    ADC14INCH_5 = 5,
    #[doc = "6: If ADC14DIF = 0: A6; If ADC14DIF = 1: Ain+ = A6, Ain- = A7"]
    ADC14INCH_6 = 6,
    #[doc = "7: If ADC14DIF = 0: A7; If ADC14DIF = 1: Ain+ = A6, Ain- = A7"]
    ADC14INCH_7 = 7,
    #[doc = "8: If ADC14DIF = 0: A8; If ADC14DIF = 1: Ain+ = A8, Ain- = A9"]
    ADC14INCH_8 = 8,
    #[doc = "9: If ADC14DIF = 0: A9; If ADC14DIF = 1: Ain+ = A8, Ain- = A9"]
    ADC14INCH_9 = 9,
    #[doc = "10: If ADC14DIF = 0: A10; If ADC14DIF = 1: Ain+ = A10, Ain- = A11"]
    ADC14INCH_10 = 10,
    #[doc = "11: If ADC14DIF = 0: A11; If ADC14DIF = 1: Ain+ = A10, Ain- = A11"]
    ADC14INCH_11 = 11,
    #[doc = "12: If ADC14DIF = 0: A12; If ADC14DIF = 1: Ain+ = A12, Ain- = A13"]
    ADC14INCH_12 = 12,
    #[doc = "13: If ADC14DIF = 0: A13; If ADC14DIF = 1: Ain+ = A12, Ain- = A13"]
    ADC14INCH_13 = 13,
    #[doc = "14: If ADC14DIF = 0: A14; If ADC14DIF = 1: Ain+ = A14, Ain- = A15"]
    ADC14INCH_14 = 14,
    #[doc = "15: If ADC14DIF = 0: A15; If ADC14DIF = 1: Ain+ = A14, Ain- = A15"]
    ADC14INCH_15 = 15,
    #[doc = "16: If ADC14DIF = 0: A16; If ADC14DIF = 1: Ain+ = A16, Ain- = A17"]
    ADC14INCH_16 = 16,
    #[doc = "17: If ADC14DIF = 0: A17; If ADC14DIF = 1: Ain+ = A16, Ain- = A17"]
    ADC14INCH_17 = 17,
    #[doc = "18: If ADC14DIF = 0: A18; If ADC14DIF = 1: Ain+ = A18, Ain- = A19"]
    ADC14INCH_18 = 18,
    #[doc = "19: If ADC14DIF = 0: A19; If ADC14DIF = 1: Ain+ = A18, Ain- = A19"]
    ADC14INCH_19 = 19,
    #[doc = "20: If ADC14DIF = 0: A20; If ADC14DIF = 1: Ain+ = A20, Ain- = A21"]
    ADC14INCH_20 = 20,
    #[doc = "21: If ADC14DIF = 0: A21; If ADC14DIF = 1: Ain+ = A20, Ain- = A21"]
    ADC14INCH_21 = 21,
    #[doc = "22: If ADC14DIF = 0: A22; If ADC14DIF = 1: Ain+ = A22, Ain- = A23"]
    ADC14INCH_22 = 22,
    #[doc = "23: If ADC14DIF = 0: A23; If ADC14DIF = 1: Ain+ = A22, Ain- = A23"]
    ADC14INCH_23 = 23,
    #[doc = "24: If ADC14DIF = 0: A24; If ADC14DIF = 1: Ain+ = A24, Ain- = A25"]
    ADC14INCH_24 = 24,
    #[doc = "25: If ADC14DIF = 0: A25; If ADC14DIF = 1: Ain+ = A24, Ain- = A25"]
    ADC14INCH_25 = 25,
    #[doc = "26: If ADC14DIF = 0: A26; If ADC14DIF = 1: Ain+ = A26, Ain- = A27"]
    ADC14INCH_26 = 26,
    #[doc = "27: If ADC14DIF = 0: A27; If ADC14DIF = 1: Ain+ = A26, Ain- = A27"]
    ADC14INCH_27 = 27,
    #[doc = "28: If ADC14DIF = 0: A28; If ADC14DIF = 1: Ain+ = A28, Ain- = A29"]
    ADC14INCH_28 = 28,
    #[doc = "29: If ADC14DIF = 0: A29; If ADC14DIF = 1: Ain+ = A28, Ain- = A29"]
    ADC14INCH_29 = 29,
    #[doc = "30: If ADC14DIF = 0: A30; If ADC14DIF = 1: Ain+ = A30, Ain- = A31"]
    ADC14INCH_30 = 30,
    #[doc = "31: If ADC14DIF = 0: A31; If ADC14DIF = 1: Ain+ = A30, Ain- = A31"]
    ADC14INCH_31 = 31,
}
impl From<ADC14INCH_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC14INCH_A) -> Self {
        variant as _
    }
}
impl ADC14INCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14INCH_A {
        match self.bits {
            0 => ADC14INCH_A::ADC14INCH_0,
            1 => ADC14INCH_A::ADC14INCH_1,
            2 => ADC14INCH_A::ADC14INCH_2,
            3 => ADC14INCH_A::ADC14INCH_3,
            4 => ADC14INCH_A::ADC14INCH_4,
            5 => ADC14INCH_A::ADC14INCH_5,
            6 => ADC14INCH_A::ADC14INCH_6,
            7 => ADC14INCH_A::ADC14INCH_7,
            8 => ADC14INCH_A::ADC14INCH_8,
            9 => ADC14INCH_A::ADC14INCH_9,
            10 => ADC14INCH_A::ADC14INCH_10,
            11 => ADC14INCH_A::ADC14INCH_11,
            12 => ADC14INCH_A::ADC14INCH_12,
            13 => ADC14INCH_A::ADC14INCH_13,
            14 => ADC14INCH_A::ADC14INCH_14,
            15 => ADC14INCH_A::ADC14INCH_15,
            16 => ADC14INCH_A::ADC14INCH_16,
            17 => ADC14INCH_A::ADC14INCH_17,
            18 => ADC14INCH_A::ADC14INCH_18,
            19 => ADC14INCH_A::ADC14INCH_19,
            20 => ADC14INCH_A::ADC14INCH_20,
            21 => ADC14INCH_A::ADC14INCH_21,
            22 => ADC14INCH_A::ADC14INCH_22,
            23 => ADC14INCH_A::ADC14INCH_23,
            24 => ADC14INCH_A::ADC14INCH_24,
            25 => ADC14INCH_A::ADC14INCH_25,
            26 => ADC14INCH_A::ADC14INCH_26,
            27 => ADC14INCH_A::ADC14INCH_27,
            28 => ADC14INCH_A::ADC14INCH_28,
            29 => ADC14INCH_A::ADC14INCH_29,
            30 => ADC14INCH_A::ADC14INCH_30,
            31 => ADC14INCH_A::ADC14INCH_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_0`"]
    #[inline(always)]
    pub fn is_adc14inch_0(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_0
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_1`"]
    #[inline(always)]
    pub fn is_adc14inch_1(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_1
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_2`"]
    #[inline(always)]
    pub fn is_adc14inch_2(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_2
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_3`"]
    #[inline(always)]
    pub fn is_adc14inch_3(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_3
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_4`"]
    #[inline(always)]
    pub fn is_adc14inch_4(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_4
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_5`"]
    #[inline(always)]
    pub fn is_adc14inch_5(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_5
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_6`"]
    #[inline(always)]
    pub fn is_adc14inch_6(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_6
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_7`"]
    #[inline(always)]
    pub fn is_adc14inch_7(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_7
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_8`"]
    #[inline(always)]
    pub fn is_adc14inch_8(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_8
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_9`"]
    #[inline(always)]
    pub fn is_adc14inch_9(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_9
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_10`"]
    #[inline(always)]
    pub fn is_adc14inch_10(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_10
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_11`"]
    #[inline(always)]
    pub fn is_adc14inch_11(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_11
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_12`"]
    #[inline(always)]
    pub fn is_adc14inch_12(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_12
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_13`"]
    #[inline(always)]
    pub fn is_adc14inch_13(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_13
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_14`"]
    #[inline(always)]
    pub fn is_adc14inch_14(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_14
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_15`"]
    #[inline(always)]
    pub fn is_adc14inch_15(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_15
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_16`"]
    #[inline(always)]
    pub fn is_adc14inch_16(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_16
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_17`"]
    #[inline(always)]
    pub fn is_adc14inch_17(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_17
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_18`"]
    #[inline(always)]
    pub fn is_adc14inch_18(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_18
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_19`"]
    #[inline(always)]
    pub fn is_adc14inch_19(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_19
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_20`"]
    #[inline(always)]
    pub fn is_adc14inch_20(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_20
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_21`"]
    #[inline(always)]
    pub fn is_adc14inch_21(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_21
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_22`"]
    #[inline(always)]
    pub fn is_adc14inch_22(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_22
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_23`"]
    #[inline(always)]
    pub fn is_adc14inch_23(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_23
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_24`"]
    #[inline(always)]
    pub fn is_adc14inch_24(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_24
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_25`"]
    #[inline(always)]
    pub fn is_adc14inch_25(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_25
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_26`"]
    #[inline(always)]
    pub fn is_adc14inch_26(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_26
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_27`"]
    #[inline(always)]
    pub fn is_adc14inch_27(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_27
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_28`"]
    #[inline(always)]
    pub fn is_adc14inch_28(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_28
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_29`"]
    #[inline(always)]
    pub fn is_adc14inch_29(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_29
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_30`"]
    #[inline(always)]
    pub fn is_adc14inch_30(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_30
    }
    #[doc = "Checks if the value of the field is `ADC14INCH_31`"]
    #[inline(always)]
    pub fn is_adc14inch_31(&self) -> bool {
        *self == ADC14INCH_A::ADC14INCH_31
    }
}
#[doc = "Field `ADC14INCH` writer - Input channel select"]
pub type ADC14INCH_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADC14MCTL_SPEC, u8, ADC14INCH_A, 5, O>;
impl<'a, const O: u8> ADC14INCH_W<'a, O> {
    #[doc = "If ADC14DIF = 0: A0; If ADC14DIF = 1: Ain+ = A0, Ain- = A1"]
    #[inline(always)]
    pub fn adc14inch_0(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_0)
    }
    #[doc = "If ADC14DIF = 0: A1; If ADC14DIF = 1: Ain+ = A0, Ain- = A1"]
    #[inline(always)]
    pub fn adc14inch_1(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_1)
    }
    #[doc = "If ADC14DIF = 0: A2; If ADC14DIF = 1: Ain+ = A2, Ain- = A3"]
    #[inline(always)]
    pub fn adc14inch_2(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_2)
    }
    #[doc = "If ADC14DIF = 0: A3; If ADC14DIF = 1: Ain+ = A2, Ain- = A3"]
    #[inline(always)]
    pub fn adc14inch_3(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_3)
    }
    #[doc = "If ADC14DIF = 0: A4; If ADC14DIF = 1: Ain+ = A4, Ain- = A5"]
    #[inline(always)]
    pub fn adc14inch_4(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_4)
    }
    #[doc = "If ADC14DIF = 0: A5; If ADC14DIF = 1: Ain+ = A4, Ain- = A5"]
    #[inline(always)]
    pub fn adc14inch_5(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_5)
    }
    #[doc = "If ADC14DIF = 0: A6; If ADC14DIF = 1: Ain+ = A6, Ain- = A7"]
    #[inline(always)]
    pub fn adc14inch_6(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_6)
    }
    #[doc = "If ADC14DIF = 0: A7; If ADC14DIF = 1: Ain+ = A6, Ain- = A7"]
    #[inline(always)]
    pub fn adc14inch_7(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_7)
    }
    #[doc = "If ADC14DIF = 0: A8; If ADC14DIF = 1: Ain+ = A8, Ain- = A9"]
    #[inline(always)]
    pub fn adc14inch_8(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_8)
    }
    #[doc = "If ADC14DIF = 0: A9; If ADC14DIF = 1: Ain+ = A8, Ain- = A9"]
    #[inline(always)]
    pub fn adc14inch_9(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_9)
    }
    #[doc = "If ADC14DIF = 0: A10; If ADC14DIF = 1: Ain+ = A10, Ain- = A11"]
    #[inline(always)]
    pub fn adc14inch_10(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_10)
    }
    #[doc = "If ADC14DIF = 0: A11; If ADC14DIF = 1: Ain+ = A10, Ain- = A11"]
    #[inline(always)]
    pub fn adc14inch_11(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_11)
    }
    #[doc = "If ADC14DIF = 0: A12; If ADC14DIF = 1: Ain+ = A12, Ain- = A13"]
    #[inline(always)]
    pub fn adc14inch_12(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_12)
    }
    #[doc = "If ADC14DIF = 0: A13; If ADC14DIF = 1: Ain+ = A12, Ain- = A13"]
    #[inline(always)]
    pub fn adc14inch_13(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_13)
    }
    #[doc = "If ADC14DIF = 0: A14; If ADC14DIF = 1: Ain+ = A14, Ain- = A15"]
    #[inline(always)]
    pub fn adc14inch_14(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_14)
    }
    #[doc = "If ADC14DIF = 0: A15; If ADC14DIF = 1: Ain+ = A14, Ain- = A15"]
    #[inline(always)]
    pub fn adc14inch_15(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_15)
    }
    #[doc = "If ADC14DIF = 0: A16; If ADC14DIF = 1: Ain+ = A16, Ain- = A17"]
    #[inline(always)]
    pub fn adc14inch_16(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_16)
    }
    #[doc = "If ADC14DIF = 0: A17; If ADC14DIF = 1: Ain+ = A16, Ain- = A17"]
    #[inline(always)]
    pub fn adc14inch_17(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_17)
    }
    #[doc = "If ADC14DIF = 0: A18; If ADC14DIF = 1: Ain+ = A18, Ain- = A19"]
    #[inline(always)]
    pub fn adc14inch_18(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_18)
    }
    #[doc = "If ADC14DIF = 0: A19; If ADC14DIF = 1: Ain+ = A18, Ain- = A19"]
    #[inline(always)]
    pub fn adc14inch_19(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_19)
    }
    #[doc = "If ADC14DIF = 0: A20; If ADC14DIF = 1: Ain+ = A20, Ain- = A21"]
    #[inline(always)]
    pub fn adc14inch_20(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_20)
    }
    #[doc = "If ADC14DIF = 0: A21; If ADC14DIF = 1: Ain+ = A20, Ain- = A21"]
    #[inline(always)]
    pub fn adc14inch_21(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_21)
    }
    #[doc = "If ADC14DIF = 0: A22; If ADC14DIF = 1: Ain+ = A22, Ain- = A23"]
    #[inline(always)]
    pub fn adc14inch_22(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_22)
    }
    #[doc = "If ADC14DIF = 0: A23; If ADC14DIF = 1: Ain+ = A22, Ain- = A23"]
    #[inline(always)]
    pub fn adc14inch_23(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_23)
    }
    #[doc = "If ADC14DIF = 0: A24; If ADC14DIF = 1: Ain+ = A24, Ain- = A25"]
    #[inline(always)]
    pub fn adc14inch_24(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_24)
    }
    #[doc = "If ADC14DIF = 0: A25; If ADC14DIF = 1: Ain+ = A24, Ain- = A25"]
    #[inline(always)]
    pub fn adc14inch_25(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_25)
    }
    #[doc = "If ADC14DIF = 0: A26; If ADC14DIF = 1: Ain+ = A26, Ain- = A27"]
    #[inline(always)]
    pub fn adc14inch_26(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_26)
    }
    #[doc = "If ADC14DIF = 0: A27; If ADC14DIF = 1: Ain+ = A26, Ain- = A27"]
    #[inline(always)]
    pub fn adc14inch_27(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_27)
    }
    #[doc = "If ADC14DIF = 0: A28; If ADC14DIF = 1: Ain+ = A28, Ain- = A29"]
    #[inline(always)]
    pub fn adc14inch_28(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_28)
    }
    #[doc = "If ADC14DIF = 0: A29; If ADC14DIF = 1: Ain+ = A28, Ain- = A29"]
    #[inline(always)]
    pub fn adc14inch_29(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_29)
    }
    #[doc = "If ADC14DIF = 0: A30; If ADC14DIF = 1: Ain+ = A30, Ain- = A31"]
    #[inline(always)]
    pub fn adc14inch_30(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_30)
    }
    #[doc = "If ADC14DIF = 0: A31; If ADC14DIF = 1: Ain+ = A30, Ain- = A31"]
    #[inline(always)]
    pub fn adc14inch_31(self) -> &'a mut W {
        self.variant(ADC14INCH_A::ADC14INCH_31)
    }
}
#[doc = "Field `ADC14EOS` reader - End of sequence"]
pub type ADC14EOS_R = crate::BitReader<ADC14EOS_A>;
#[doc = "End of sequence\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14EOS_A {
    #[doc = "0: Not end of sequence"]
    ADC14EOS_0 = 0,
    #[doc = "1: End of sequence"]
    ADC14EOS_1 = 1,
}
impl From<ADC14EOS_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14EOS_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14EOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14EOS_A {
        match self.bits {
            false => ADC14EOS_A::ADC14EOS_0,
            true => ADC14EOS_A::ADC14EOS_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14EOS_0`"]
    #[inline(always)]
    pub fn is_adc14eos_0(&self) -> bool {
        *self == ADC14EOS_A::ADC14EOS_0
    }
    #[doc = "Checks if the value of the field is `ADC14EOS_1`"]
    #[inline(always)]
    pub fn is_adc14eos_1(&self) -> bool {
        *self == ADC14EOS_A::ADC14EOS_1
    }
}
#[doc = "Field `ADC14EOS` writer - End of sequence"]
pub type ADC14EOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14MCTL_SPEC, ADC14EOS_A, O>;
impl<'a, const O: u8> ADC14EOS_W<'a, O> {
    #[doc = "Not end of sequence"]
    #[inline(always)]
    pub fn adc14eos_0(self) -> &'a mut W {
        self.variant(ADC14EOS_A::ADC14EOS_0)
    }
    #[doc = "End of sequence"]
    #[inline(always)]
    pub fn adc14eos_1(self) -> &'a mut W {
        self.variant(ADC14EOS_A::ADC14EOS_1)
    }
}
#[doc = "Field `ADC14VRSEL` reader - Selects combinations of V(R+) and V(R-) sources"]
pub type ADC14VRSEL_R = crate::FieldReader<u8, ADC14VRSEL_A>;
#[doc = "Selects combinations of V(R+) and V(R-) sources\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC14VRSEL_A {
    #[doc = "0: V(R+) = AVCC, V(R-) = AVSS"]
    ADC14VRSEL_0 = 0,
    #[doc = "1: V(R+) = VREF buffered, V(R-) = AVSS"]
    ADC14VRSEL_1 = 1,
    #[doc = "14: V(R+) = VeREF+, V(R-) = VeREF-"]
    ADC14VRSEL_14 = 14,
    #[doc = "15: V(R+) = VeREF+ buffered, V(R-) = VeREF"]
    ADC14VRSEL_15 = 15,
}
impl From<ADC14VRSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC14VRSEL_A) -> Self {
        variant as _
    }
}
impl ADC14VRSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC14VRSEL_A> {
        match self.bits {
            0 => Some(ADC14VRSEL_A::ADC14VRSEL_0),
            1 => Some(ADC14VRSEL_A::ADC14VRSEL_1),
            14 => Some(ADC14VRSEL_A::ADC14VRSEL_14),
            15 => Some(ADC14VRSEL_A::ADC14VRSEL_15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14VRSEL_0`"]
    #[inline(always)]
    pub fn is_adc14vrsel_0(&self) -> bool {
        *self == ADC14VRSEL_A::ADC14VRSEL_0
    }
    #[doc = "Checks if the value of the field is `ADC14VRSEL_1`"]
    #[inline(always)]
    pub fn is_adc14vrsel_1(&self) -> bool {
        *self == ADC14VRSEL_A::ADC14VRSEL_1
    }
    #[doc = "Checks if the value of the field is `ADC14VRSEL_14`"]
    #[inline(always)]
    pub fn is_adc14vrsel_14(&self) -> bool {
        *self == ADC14VRSEL_A::ADC14VRSEL_14
    }
    #[doc = "Checks if the value of the field is `ADC14VRSEL_15`"]
    #[inline(always)]
    pub fn is_adc14vrsel_15(&self) -> bool {
        *self == ADC14VRSEL_A::ADC14VRSEL_15
    }
}
#[doc = "Field `ADC14VRSEL` writer - Selects combinations of V(R+) and V(R-) sources"]
pub type ADC14VRSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADC14MCTL_SPEC, u8, ADC14VRSEL_A, 4, O>;
impl<'a, const O: u8> ADC14VRSEL_W<'a, O> {
    #[doc = "V(R+) = AVCC, V(R-) = AVSS"]
    #[inline(always)]
    pub fn adc14vrsel_0(self) -> &'a mut W {
        self.variant(ADC14VRSEL_A::ADC14VRSEL_0)
    }
    #[doc = "V(R+) = VREF buffered, V(R-) = AVSS"]
    #[inline(always)]
    pub fn adc14vrsel_1(self) -> &'a mut W {
        self.variant(ADC14VRSEL_A::ADC14VRSEL_1)
    }
    #[doc = "V(R+) = VeREF+, V(R-) = VeREF-"]
    #[inline(always)]
    pub fn adc14vrsel_14(self) -> &'a mut W {
        self.variant(ADC14VRSEL_A::ADC14VRSEL_14)
    }
    #[doc = "V(R+) = VeREF+ buffered, V(R-) = VeREF"]
    #[inline(always)]
    pub fn adc14vrsel_15(self) -> &'a mut W {
        self.variant(ADC14VRSEL_A::ADC14VRSEL_15)
    }
}
#[doc = "Field `ADC14DIF` reader - Differential mode"]
pub type ADC14DIF_R = crate::BitReader<ADC14DIF_A>;
#[doc = "Differential mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14DIF_A {
    #[doc = "0: Single-ended mode enabled"]
    ADC14DIF_0 = 0,
    #[doc = "1: Differential mode enabled"]
    ADC14DIF_1 = 1,
}
impl From<ADC14DIF_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14DIF_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14DIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14DIF_A {
        match self.bits {
            false => ADC14DIF_A::ADC14DIF_0,
            true => ADC14DIF_A::ADC14DIF_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14DIF_0`"]
    #[inline(always)]
    pub fn is_adc14dif_0(&self) -> bool {
        *self == ADC14DIF_A::ADC14DIF_0
    }
    #[doc = "Checks if the value of the field is `ADC14DIF_1`"]
    #[inline(always)]
    pub fn is_adc14dif_1(&self) -> bool {
        *self == ADC14DIF_A::ADC14DIF_1
    }
}
#[doc = "Field `ADC14DIF` writer - Differential mode"]
pub type ADC14DIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14MCTL_SPEC, ADC14DIF_A, O>;
impl<'a, const O: u8> ADC14DIF_W<'a, O> {
    #[doc = "Single-ended mode enabled"]
    #[inline(always)]
    pub fn adc14dif_0(self) -> &'a mut W {
        self.variant(ADC14DIF_A::ADC14DIF_0)
    }
    #[doc = "Differential mode enabled"]
    #[inline(always)]
    pub fn adc14dif_1(self) -> &'a mut W {
        self.variant(ADC14DIF_A::ADC14DIF_1)
    }
}
#[doc = "Field `ADC14WINC` reader - Comparator window enable"]
pub type ADC14WINC_R = crate::BitReader<ADC14WINC_A>;
#[doc = "Comparator window enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14WINC_A {
    #[doc = "0: Comparator window disabled"]
    ADC14WINC_0 = 0,
    #[doc = "1: Comparator window enabled"]
    ADC14WINC_1 = 1,
}
impl From<ADC14WINC_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14WINC_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14WINC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14WINC_A {
        match self.bits {
            false => ADC14WINC_A::ADC14WINC_0,
            true => ADC14WINC_A::ADC14WINC_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14WINC_0`"]
    #[inline(always)]
    pub fn is_adc14winc_0(&self) -> bool {
        *self == ADC14WINC_A::ADC14WINC_0
    }
    #[doc = "Checks if the value of the field is `ADC14WINC_1`"]
    #[inline(always)]
    pub fn is_adc14winc_1(&self) -> bool {
        *self == ADC14WINC_A::ADC14WINC_1
    }
}
#[doc = "Field `ADC14WINC` writer - Comparator window enable"]
pub type ADC14WINC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14MCTL_SPEC, ADC14WINC_A, O>;
impl<'a, const O: u8> ADC14WINC_W<'a, O> {
    #[doc = "Comparator window disabled"]
    #[inline(always)]
    pub fn adc14winc_0(self) -> &'a mut W {
        self.variant(ADC14WINC_A::ADC14WINC_0)
    }
    #[doc = "Comparator window enabled"]
    #[inline(always)]
    pub fn adc14winc_1(self) -> &'a mut W {
        self.variant(ADC14WINC_A::ADC14WINC_1)
    }
}
#[doc = "Field `ADC14WINCTH` reader - Window comparator threshold register selection"]
pub type ADC14WINCTH_R = crate::BitReader<ADC14WINCTH_A>;
#[doc = "Window comparator threshold register selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14WINCTH_A {
    #[doc = "0: Use window comparator thresholds 0, ADC14LO0 and ADC14HI0"]
    ADC14WINCTH_0 = 0,
    #[doc = "1: Use window comparator thresholds 1, ADC14LO1 and ADC14HI1"]
    ADC14WINCTH_1 = 1,
}
impl From<ADC14WINCTH_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14WINCTH_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14WINCTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14WINCTH_A {
        match self.bits {
            false => ADC14WINCTH_A::ADC14WINCTH_0,
            true => ADC14WINCTH_A::ADC14WINCTH_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14WINCTH_0`"]
    #[inline(always)]
    pub fn is_adc14wincth_0(&self) -> bool {
        *self == ADC14WINCTH_A::ADC14WINCTH_0
    }
    #[doc = "Checks if the value of the field is `ADC14WINCTH_1`"]
    #[inline(always)]
    pub fn is_adc14wincth_1(&self) -> bool {
        *self == ADC14WINCTH_A::ADC14WINCTH_1
    }
}
#[doc = "Field `ADC14WINCTH` writer - Window comparator threshold register selection"]
pub type ADC14WINCTH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADC14MCTL_SPEC, ADC14WINCTH_A, O>;
impl<'a, const O: u8> ADC14WINCTH_W<'a, O> {
    #[doc = "Use window comparator thresholds 0, ADC14LO0 and ADC14HI0"]
    #[inline(always)]
    pub fn adc14wincth_0(self) -> &'a mut W {
        self.variant(ADC14WINCTH_A::ADC14WINCTH_0)
    }
    #[doc = "Use window comparator thresholds 1, ADC14LO1 and ADC14HI1"]
    #[inline(always)]
    pub fn adc14wincth_1(self) -> &'a mut W {
        self.variant(ADC14WINCTH_A::ADC14WINCTH_1)
    }
}
impl R {
    #[doc = "Bits 0:4 - Input channel select"]
    #[inline(always)]
    pub fn adc14inch(&self) -> ADC14INCH_R {
        ADC14INCH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - End of sequence"]
    #[inline(always)]
    pub fn adc14eos(&self) -> ADC14EOS_R {
        ADC14EOS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Selects combinations of V(R+) and V(R-) sources"]
    #[inline(always)]
    pub fn adc14vrsel(&self) -> ADC14VRSEL_R {
        ADC14VRSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Differential mode"]
    #[inline(always)]
    pub fn adc14dif(&self) -> ADC14DIF_R {
        ADC14DIF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Comparator window enable"]
    #[inline(always)]
    pub fn adc14winc(&self) -> ADC14WINC_R {
        ADC14WINC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Window comparator threshold register selection"]
    #[inline(always)]
    pub fn adc14wincth(&self) -> ADC14WINCTH_R {
        ADC14WINCTH_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel select"]
    #[inline(always)]
    pub fn adc14inch(&mut self) -> ADC14INCH_W<0> {
        ADC14INCH_W::new(self)
    }
    #[doc = "Bit 7 - End of sequence"]
    #[inline(always)]
    pub fn adc14eos(&mut self) -> ADC14EOS_W<7> {
        ADC14EOS_W::new(self)
    }
    #[doc = "Bits 8:11 - Selects combinations of V(R+) and V(R-) sources"]
    #[inline(always)]
    pub fn adc14vrsel(&mut self) -> ADC14VRSEL_W<8> {
        ADC14VRSEL_W::new(self)
    }
    #[doc = "Bit 13 - Differential mode"]
    #[inline(always)]
    pub fn adc14dif(&mut self) -> ADC14DIF_W<13> {
        ADC14DIF_W::new(self)
    }
    #[doc = "Bit 14 - Comparator window enable"]
    #[inline(always)]
    pub fn adc14winc(&mut self) -> ADC14WINC_W<14> {
        ADC14WINC_W::new(self)
    }
    #[doc = "Bit 15 - Window comparator threshold register selection"]
    #[inline(always)]
    pub fn adc14wincth(&mut self) -> ADC14WINCTH_W<15> {
        ADC14WINCTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Conversion Memory Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14mctl](index.html) module"]
pub struct ADC14MCTL_SPEC;
impl crate::RegisterSpec for ADC14MCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc14mctl::R](R) reader structure"]
impl crate::Readable for ADC14MCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc14mctl::W](W) writer structure"]
impl crate::Writable for ADC14MCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC14MCTL[%s]
to value 0"]
impl crate::Resettable for ADC14MCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
