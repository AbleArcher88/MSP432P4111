#[doc = "Register `CSCTL2` reader"]
pub struct R(crate::R<CSCTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSCTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSCTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSCTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSCTL2` writer"]
pub struct W(crate::W<CSCTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSCTL2_SPEC>;
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
impl From<crate::W<CSCTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSCTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LFXTDRIVE` reader - LFXT oscillator current can be adjusted to its drive needs"]
pub type LFXTDRIVE_R = crate::FieldReader<u8, LFXTDRIVE_A>;
#[doc = "LFXT oscillator current can be adjusted to its drive needs\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LFXTDRIVE_A {
    #[doc = "0: Lowest drive strength and current consumption LFXT oscillator."]
    LFXTDRIVE_0 = 0,
    #[doc = "1: Increased drive strength LFXT oscillator."]
    LFXTDRIVE_1 = 1,
    #[doc = "2: Increased drive strength LFXT oscillator."]
    LFXTDRIVE_2 = 2,
    #[doc = "3: Maximum drive strength and maximum current consumption LFXT oscillator."]
    LFXTDRIVE_3 = 3,
}
impl From<LFXTDRIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: LFXTDRIVE_A) -> Self {
        variant as _
    }
}
impl LFXTDRIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFXTDRIVE_A {
        match self.bits {
            0 => LFXTDRIVE_A::LFXTDRIVE_0,
            1 => LFXTDRIVE_A::LFXTDRIVE_1,
            2 => LFXTDRIVE_A::LFXTDRIVE_2,
            3 => LFXTDRIVE_A::LFXTDRIVE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LFXTDRIVE_0`"]
    #[inline(always)]
    pub fn is_lfxtdrive_0(&self) -> bool {
        *self == LFXTDRIVE_A::LFXTDRIVE_0
    }
    #[doc = "Checks if the value of the field is `LFXTDRIVE_1`"]
    #[inline(always)]
    pub fn is_lfxtdrive_1(&self) -> bool {
        *self == LFXTDRIVE_A::LFXTDRIVE_1
    }
    #[doc = "Checks if the value of the field is `LFXTDRIVE_2`"]
    #[inline(always)]
    pub fn is_lfxtdrive_2(&self) -> bool {
        *self == LFXTDRIVE_A::LFXTDRIVE_2
    }
    #[doc = "Checks if the value of the field is `LFXTDRIVE_3`"]
    #[inline(always)]
    pub fn is_lfxtdrive_3(&self) -> bool {
        *self == LFXTDRIVE_A::LFXTDRIVE_3
    }
}
#[doc = "Field `LFXTDRIVE` writer - LFXT oscillator current can be adjusted to its drive needs"]
pub type LFXTDRIVE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CSCTL2_SPEC, u8, LFXTDRIVE_A, 2, O>;
impl<'a, const O: u8> LFXTDRIVE_W<'a, O> {
    #[doc = "Lowest drive strength and current consumption LFXT oscillator."]
    #[inline(always)]
    pub fn lfxtdrive_0(self) -> &'a mut W {
        self.variant(LFXTDRIVE_A::LFXTDRIVE_0)
    }
    #[doc = "Increased drive strength LFXT oscillator."]
    #[inline(always)]
    pub fn lfxtdrive_1(self) -> &'a mut W {
        self.variant(LFXTDRIVE_A::LFXTDRIVE_1)
    }
    #[doc = "Increased drive strength LFXT oscillator."]
    #[inline(always)]
    pub fn lfxtdrive_2(self) -> &'a mut W {
        self.variant(LFXTDRIVE_A::LFXTDRIVE_2)
    }
    #[doc = "Maximum drive strength and maximum current consumption LFXT oscillator."]
    #[inline(always)]
    pub fn lfxtdrive_3(self) -> &'a mut W {
        self.variant(LFXTDRIVE_A::LFXTDRIVE_3)
    }
}
#[doc = "Field `LFXTAGCOFF` reader - Disables the automatic gain control of the LFXT crystal"]
pub type LFXTAGCOFF_R = crate::BitReader<LFXTAGCOFF_A>;
#[doc = "Disables the automatic gain control of the LFXT crystal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LFXTAGCOFF_A {
    #[doc = "0: AGC enabled."]
    LFXTAGCOFF_0 = 0,
    #[doc = "1: AGC disabled."]
    LFXTAGCOFF_1 = 1,
}
impl From<LFXTAGCOFF_A> for bool {
    #[inline(always)]
    fn from(variant: LFXTAGCOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl LFXTAGCOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFXTAGCOFF_A {
        match self.bits {
            false => LFXTAGCOFF_A::LFXTAGCOFF_0,
            true => LFXTAGCOFF_A::LFXTAGCOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `LFXTAGCOFF_0`"]
    #[inline(always)]
    pub fn is_lfxtagcoff_0(&self) -> bool {
        *self == LFXTAGCOFF_A::LFXTAGCOFF_0
    }
    #[doc = "Checks if the value of the field is `LFXTAGCOFF_1`"]
    #[inline(always)]
    pub fn is_lfxtagcoff_1(&self) -> bool {
        *self == LFXTAGCOFF_A::LFXTAGCOFF_1
    }
}
#[doc = "Field `LFXTAGCOFF` writer - Disables the automatic gain control of the LFXT crystal"]
pub type LFXTAGCOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSCTL2_SPEC, LFXTAGCOFF_A, O>;
impl<'a, const O: u8> LFXTAGCOFF_W<'a, O> {
    #[doc = "AGC enabled."]
    #[inline(always)]
    pub fn lfxtagcoff_0(self) -> &'a mut W {
        self.variant(LFXTAGCOFF_A::LFXTAGCOFF_0)
    }
    #[doc = "AGC disabled."]
    #[inline(always)]
    pub fn lfxtagcoff_1(self) -> &'a mut W {
        self.variant(LFXTAGCOFF_A::LFXTAGCOFF_1)
    }
}
#[doc = "Field `LFXT_EN` reader - Turns on the LFXT oscillator regardless if used as a clock resource"]
pub type LFXT_EN_R = crate::BitReader<LFXT_EN_A>;
#[doc = "Turns on the LFXT oscillator regardless if used as a clock resource\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LFXT_EN_A {
    #[doc = "0: LFXT is on if it is used as a source for ACLK, MCLK, HSMCLK , or SMCLK and is selected via the port selection and not in bypass mode of operation."]
    LFXT_EN_0 = 0,
    #[doc = "1: LFXT is on if LFXT is selected via the port selection and LFXT is not in bypass mode of operation."]
    LFXT_EN_1 = 1,
}
impl From<LFXT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: LFXT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl LFXT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFXT_EN_A {
        match self.bits {
            false => LFXT_EN_A::LFXT_EN_0,
            true => LFXT_EN_A::LFXT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `LFXT_EN_0`"]
    #[inline(always)]
    pub fn is_lfxt_en_0(&self) -> bool {
        *self == LFXT_EN_A::LFXT_EN_0
    }
    #[doc = "Checks if the value of the field is `LFXT_EN_1`"]
    #[inline(always)]
    pub fn is_lfxt_en_1(&self) -> bool {
        *self == LFXT_EN_A::LFXT_EN_1
    }
}
#[doc = "Field `LFXT_EN` writer - Turns on the LFXT oscillator regardless if used as a clock resource"]
pub type LFXT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSCTL2_SPEC, LFXT_EN_A, O>;
impl<'a, const O: u8> LFXT_EN_W<'a, O> {
    #[doc = "LFXT is on if it is used as a source for ACLK, MCLK, HSMCLK , or SMCLK and is selected via the port selection and not in bypass mode of operation."]
    #[inline(always)]
    pub fn lfxt_en_0(self) -> &'a mut W {
        self.variant(LFXT_EN_A::LFXT_EN_0)
    }
    #[doc = "LFXT is on if LFXT is selected via the port selection and LFXT is not in bypass mode of operation."]
    #[inline(always)]
    pub fn lfxt_en_1(self) -> &'a mut W {
        self.variant(LFXT_EN_A::LFXT_EN_1)
    }
}
#[doc = "Field `LFXTBYPASS` reader - LFXT bypass select"]
pub type LFXTBYPASS_R = crate::BitReader<LFXTBYPASS_A>;
#[doc = "LFXT bypass select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LFXTBYPASS_A {
    #[doc = "0: LFXT sourced by external crystal."]
    LFXTBYPASS_0 = 0,
    #[doc = "1: LFXT sourced by external square wave."]
    LFXTBYPASS_1 = 1,
}
impl From<LFXTBYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: LFXTBYPASS_A) -> Self {
        variant as u8 != 0
    }
}
impl LFXTBYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFXTBYPASS_A {
        match self.bits {
            false => LFXTBYPASS_A::LFXTBYPASS_0,
            true => LFXTBYPASS_A::LFXTBYPASS_1,
        }
    }
    #[doc = "Checks if the value of the field is `LFXTBYPASS_0`"]
    #[inline(always)]
    pub fn is_lfxtbypass_0(&self) -> bool {
        *self == LFXTBYPASS_A::LFXTBYPASS_0
    }
    #[doc = "Checks if the value of the field is `LFXTBYPASS_1`"]
    #[inline(always)]
    pub fn is_lfxtbypass_1(&self) -> bool {
        *self == LFXTBYPASS_A::LFXTBYPASS_1
    }
}
#[doc = "Field `LFXTBYPASS` writer - LFXT bypass select"]
pub type LFXTBYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSCTL2_SPEC, LFXTBYPASS_A, O>;
impl<'a, const O: u8> LFXTBYPASS_W<'a, O> {
    #[doc = "LFXT sourced by external crystal."]
    #[inline(always)]
    pub fn lfxtbypass_0(self) -> &'a mut W {
        self.variant(LFXTBYPASS_A::LFXTBYPASS_0)
    }
    #[doc = "LFXT sourced by external square wave."]
    #[inline(always)]
    pub fn lfxtbypass_1(self) -> &'a mut W {
        self.variant(LFXTBYPASS_A::LFXTBYPASS_1)
    }
}
#[doc = "Field `HFXTDRIVE` reader - HFXT oscillator drive selection"]
pub type HFXTDRIVE_R = crate::BitReader<HFXTDRIVE_A>;
#[doc = "HFXT oscillator drive selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HFXTDRIVE_A {
    #[doc = "0: To be used for HFXTFREQ setting 000b"]
    HFXTDRIVE_0 = 0,
    #[doc = "1: To be used for HFXTFREQ settings 001b to 110b"]
    HFXTDRIVE_1 = 1,
}
impl From<HFXTDRIVE_A> for bool {
    #[inline(always)]
    fn from(variant: HFXTDRIVE_A) -> Self {
        variant as u8 != 0
    }
}
impl HFXTDRIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFXTDRIVE_A {
        match self.bits {
            false => HFXTDRIVE_A::HFXTDRIVE_0,
            true => HFXTDRIVE_A::HFXTDRIVE_1,
        }
    }
    #[doc = "Checks if the value of the field is `HFXTDRIVE_0`"]
    #[inline(always)]
    pub fn is_hfxtdrive_0(&self) -> bool {
        *self == HFXTDRIVE_A::HFXTDRIVE_0
    }
    #[doc = "Checks if the value of the field is `HFXTDRIVE_1`"]
    #[inline(always)]
    pub fn is_hfxtdrive_1(&self) -> bool {
        *self == HFXTDRIVE_A::HFXTDRIVE_1
    }
}
#[doc = "Field `HFXTDRIVE` writer - HFXT oscillator drive selection"]
pub type HFXTDRIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSCTL2_SPEC, HFXTDRIVE_A, O>;
impl<'a, const O: u8> HFXTDRIVE_W<'a, O> {
    #[doc = "To be used for HFXTFREQ setting 000b"]
    #[inline(always)]
    pub fn hfxtdrive_0(self) -> &'a mut W {
        self.variant(HFXTDRIVE_A::HFXTDRIVE_0)
    }
    #[doc = "To be used for HFXTFREQ settings 001b to 110b"]
    #[inline(always)]
    pub fn hfxtdrive_1(self) -> &'a mut W {
        self.variant(HFXTDRIVE_A::HFXTDRIVE_1)
    }
}
#[doc = "Field `HFXTFREQ` reader - HFXT frequency selection"]
pub type HFXTFREQ_R = crate::FieldReader<u8, HFXTFREQ_A>;
#[doc = "HFXT frequency selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HFXTFREQ_A {
    #[doc = "0: 1 MHz to 4 MHz"]
    HFXTFREQ_0 = 0,
    #[doc = "1: >4 MHz to 8 MHz"]
    HFXTFREQ_1 = 1,
    #[doc = "2: >8 MHz to 16 MHz"]
    HFXTFREQ_2 = 2,
    #[doc = "3: >16 MHz to 24 MHz"]
    HFXTFREQ_3 = 3,
    #[doc = "4: >24 MHz to 32 MHz"]
    HFXTFREQ_4 = 4,
    #[doc = "5: >32 MHz to 40 MHz"]
    HFXTFREQ_5 = 5,
    #[doc = "6: >40 MHz to 48 MHz"]
    HFXTFREQ_6 = 6,
}
impl From<HFXTFREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: HFXTFREQ_A) -> Self {
        variant as _
    }
}
impl HFXTFREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HFXTFREQ_A> {
        match self.bits {
            0 => Some(HFXTFREQ_A::HFXTFREQ_0),
            1 => Some(HFXTFREQ_A::HFXTFREQ_1),
            2 => Some(HFXTFREQ_A::HFXTFREQ_2),
            3 => Some(HFXTFREQ_A::HFXTFREQ_3),
            4 => Some(HFXTFREQ_A::HFXTFREQ_4),
            5 => Some(HFXTFREQ_A::HFXTFREQ_5),
            6 => Some(HFXTFREQ_A::HFXTFREQ_6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HFXTFREQ_0`"]
    #[inline(always)]
    pub fn is_hfxtfreq_0(&self) -> bool {
        *self == HFXTFREQ_A::HFXTFREQ_0
    }
    #[doc = "Checks if the value of the field is `HFXTFREQ_1`"]
    #[inline(always)]
    pub fn is_hfxtfreq_1(&self) -> bool {
        *self == HFXTFREQ_A::HFXTFREQ_1
    }
    #[doc = "Checks if the value of the field is `HFXTFREQ_2`"]
    #[inline(always)]
    pub fn is_hfxtfreq_2(&self) -> bool {
        *self == HFXTFREQ_A::HFXTFREQ_2
    }
    #[doc = "Checks if the value of the field is `HFXTFREQ_3`"]
    #[inline(always)]
    pub fn is_hfxtfreq_3(&self) -> bool {
        *self == HFXTFREQ_A::HFXTFREQ_3
    }
    #[doc = "Checks if the value of the field is `HFXTFREQ_4`"]
    #[inline(always)]
    pub fn is_hfxtfreq_4(&self) -> bool {
        *self == HFXTFREQ_A::HFXTFREQ_4
    }
    #[doc = "Checks if the value of the field is `HFXTFREQ_5`"]
    #[inline(always)]
    pub fn is_hfxtfreq_5(&self) -> bool {
        *self == HFXTFREQ_A::HFXTFREQ_5
    }
    #[doc = "Checks if the value of the field is `HFXTFREQ_6`"]
    #[inline(always)]
    pub fn is_hfxtfreq_6(&self) -> bool {
        *self == HFXTFREQ_A::HFXTFREQ_6
    }
}
#[doc = "Field `HFXTFREQ` writer - HFXT frequency selection"]
pub type HFXTFREQ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSCTL2_SPEC, u8, HFXTFREQ_A, 3, O>;
impl<'a, const O: u8> HFXTFREQ_W<'a, O> {
    #[doc = "1 MHz to 4 MHz"]
    #[inline(always)]
    pub fn hfxtfreq_0(self) -> &'a mut W {
        self.variant(HFXTFREQ_A::HFXTFREQ_0)
    }
    #[doc = ">4 MHz to 8 MHz"]
    #[inline(always)]
    pub fn hfxtfreq_1(self) -> &'a mut W {
        self.variant(HFXTFREQ_A::HFXTFREQ_1)
    }
    #[doc = ">8 MHz to 16 MHz"]
    #[inline(always)]
    pub fn hfxtfreq_2(self) -> &'a mut W {
        self.variant(HFXTFREQ_A::HFXTFREQ_2)
    }
    #[doc = ">16 MHz to 24 MHz"]
    #[inline(always)]
    pub fn hfxtfreq_3(self) -> &'a mut W {
        self.variant(HFXTFREQ_A::HFXTFREQ_3)
    }
    #[doc = ">24 MHz to 32 MHz"]
    #[inline(always)]
    pub fn hfxtfreq_4(self) -> &'a mut W {
        self.variant(HFXTFREQ_A::HFXTFREQ_4)
    }
    #[doc = ">32 MHz to 40 MHz"]
    #[inline(always)]
    pub fn hfxtfreq_5(self) -> &'a mut W {
        self.variant(HFXTFREQ_A::HFXTFREQ_5)
    }
    #[doc = ">40 MHz to 48 MHz"]
    #[inline(always)]
    pub fn hfxtfreq_6(self) -> &'a mut W {
        self.variant(HFXTFREQ_A::HFXTFREQ_6)
    }
}
#[doc = "Field `HFXT_EN` reader - Turns on the HFXT oscillator regardless if used as a clock resource"]
pub type HFXT_EN_R = crate::BitReader<HFXT_EN_A>;
#[doc = "Turns on the HFXT oscillator regardless if used as a clock resource\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HFXT_EN_A {
    #[doc = "0: HFXT is on if it is used as a source for MCLK, HSMCLK , or SMCLK and is selected via the port selection and not in bypass mode of operation."]
    HFXT_EN_0 = 0,
    #[doc = "1: HFXT is on if HFXT is selected via the port selection and HFXT is not in bypass mode of operation."]
    HFXT_EN_1 = 1,
}
impl From<HFXT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HFXT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl HFXT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFXT_EN_A {
        match self.bits {
            false => HFXT_EN_A::HFXT_EN_0,
            true => HFXT_EN_A::HFXT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `HFXT_EN_0`"]
    #[inline(always)]
    pub fn is_hfxt_en_0(&self) -> bool {
        *self == HFXT_EN_A::HFXT_EN_0
    }
    #[doc = "Checks if the value of the field is `HFXT_EN_1`"]
    #[inline(always)]
    pub fn is_hfxt_en_1(&self) -> bool {
        *self == HFXT_EN_A::HFXT_EN_1
    }
}
#[doc = "Field `HFXT_EN` writer - Turns on the HFXT oscillator regardless if used as a clock resource"]
pub type HFXT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSCTL2_SPEC, HFXT_EN_A, O>;
impl<'a, const O: u8> HFXT_EN_W<'a, O> {
    #[doc = "HFXT is on if it is used as a source for MCLK, HSMCLK , or SMCLK and is selected via the port selection and not in bypass mode of operation."]
    #[inline(always)]
    pub fn hfxt_en_0(self) -> &'a mut W {
        self.variant(HFXT_EN_A::HFXT_EN_0)
    }
    #[doc = "HFXT is on if HFXT is selected via the port selection and HFXT is not in bypass mode of operation."]
    #[inline(always)]
    pub fn hfxt_en_1(self) -> &'a mut W {
        self.variant(HFXT_EN_A::HFXT_EN_1)
    }
}
#[doc = "Field `HFXTBYPASS` reader - HFXT bypass select"]
pub type HFXTBYPASS_R = crate::BitReader<HFXTBYPASS_A>;
#[doc = "HFXT bypass select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HFXTBYPASS_A {
    #[doc = "0: HFXT sourced by external crystal."]
    HFXTBYPASS_0 = 0,
    #[doc = "1: HFXT sourced by external square wave."]
    HFXTBYPASS_1 = 1,
}
impl From<HFXTBYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: HFXTBYPASS_A) -> Self {
        variant as u8 != 0
    }
}
impl HFXTBYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFXTBYPASS_A {
        match self.bits {
            false => HFXTBYPASS_A::HFXTBYPASS_0,
            true => HFXTBYPASS_A::HFXTBYPASS_1,
        }
    }
    #[doc = "Checks if the value of the field is `HFXTBYPASS_0`"]
    #[inline(always)]
    pub fn is_hfxtbypass_0(&self) -> bool {
        *self == HFXTBYPASS_A::HFXTBYPASS_0
    }
    #[doc = "Checks if the value of the field is `HFXTBYPASS_1`"]
    #[inline(always)]
    pub fn is_hfxtbypass_1(&self) -> bool {
        *self == HFXTBYPASS_A::HFXTBYPASS_1
    }
}
#[doc = "Field `HFXTBYPASS` writer - HFXT bypass select"]
pub type HFXTBYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSCTL2_SPEC, HFXTBYPASS_A, O>;
impl<'a, const O: u8> HFXTBYPASS_W<'a, O> {
    #[doc = "HFXT sourced by external crystal."]
    #[inline(always)]
    pub fn hfxtbypass_0(self) -> &'a mut W {
        self.variant(HFXTBYPASS_A::HFXTBYPASS_0)
    }
    #[doc = "HFXT sourced by external square wave."]
    #[inline(always)]
    pub fn hfxtbypass_1(self) -> &'a mut W {
        self.variant(HFXTBYPASS_A::HFXTBYPASS_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - LFXT oscillator current can be adjusted to its drive needs"]
    #[inline(always)]
    pub fn lfxtdrive(&self) -> LFXTDRIVE_R {
        LFXTDRIVE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 7 - Disables the automatic gain control of the LFXT crystal"]
    #[inline(always)]
    pub fn lfxtagcoff(&self) -> LFXTAGCOFF_R {
        LFXTAGCOFF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Turns on the LFXT oscillator regardless if used as a clock resource"]
    #[inline(always)]
    pub fn lfxt_en(&self) -> LFXT_EN_R {
        LFXT_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LFXT bypass select"]
    #[inline(always)]
    pub fn lfxtbypass(&self) -> LFXTBYPASS_R {
        LFXTBYPASS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - HFXT oscillator drive selection"]
    #[inline(always)]
    pub fn hfxtdrive(&self) -> HFXTDRIVE_R {
        HFXTDRIVE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:22 - HFXT frequency selection"]
    #[inline(always)]
    pub fn hfxtfreq(&self) -> HFXTFREQ_R {
        HFXTFREQ_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 24 - Turns on the HFXT oscillator regardless if used as a clock resource"]
    #[inline(always)]
    pub fn hfxt_en(&self) -> HFXT_EN_R {
        HFXT_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - HFXT bypass select"]
    #[inline(always)]
    pub fn hfxtbypass(&self) -> HFXTBYPASS_R {
        HFXTBYPASS_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - LFXT oscillator current can be adjusted to its drive needs"]
    #[inline(always)]
    pub fn lfxtdrive(&mut self) -> LFXTDRIVE_W<0> {
        LFXTDRIVE_W::new(self)
    }
    #[doc = "Bit 7 - Disables the automatic gain control of the LFXT crystal"]
    #[inline(always)]
    pub fn lfxtagcoff(&mut self) -> LFXTAGCOFF_W<7> {
        LFXTAGCOFF_W::new(self)
    }
    #[doc = "Bit 8 - Turns on the LFXT oscillator regardless if used as a clock resource"]
    #[inline(always)]
    pub fn lfxt_en(&mut self) -> LFXT_EN_W<8> {
        LFXT_EN_W::new(self)
    }
    #[doc = "Bit 9 - LFXT bypass select"]
    #[inline(always)]
    pub fn lfxtbypass(&mut self) -> LFXTBYPASS_W<9> {
        LFXTBYPASS_W::new(self)
    }
    #[doc = "Bit 16 - HFXT oscillator drive selection"]
    #[inline(always)]
    pub fn hfxtdrive(&mut self) -> HFXTDRIVE_W<16> {
        HFXTDRIVE_W::new(self)
    }
    #[doc = "Bits 20:22 - HFXT frequency selection"]
    #[inline(always)]
    pub fn hfxtfreq(&mut self) -> HFXTFREQ_W<20> {
        HFXTFREQ_W::new(self)
    }
    #[doc = "Bit 24 - Turns on the HFXT oscillator regardless if used as a clock resource"]
    #[inline(always)]
    pub fn hfxt_en(&mut self) -> HFXT_EN_W<24> {
        HFXT_EN_W::new(self)
    }
    #[doc = "Bit 25 - HFXT bypass select"]
    #[inline(always)]
    pub fn hfxtbypass(&mut self) -> HFXTBYPASS_W<25> {
        HFXTBYPASS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csctl2](index.html) module"]
pub struct CSCTL2_SPEC;
impl crate::RegisterSpec for CSCTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csctl2::R](R) reader structure"]
impl crate::Readable for CSCTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csctl2::W](W) writer structure"]
impl crate::Writable for CSCTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSCTL2 to value 0x0001_0003"]
impl crate::Resettable for CSCTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0003
    }
}
