#[doc = "Register `CSCTL3` reader"]
pub struct R(crate::R<CSCTL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSCTL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSCTL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSCTL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSCTL3` writer"]
pub struct W(crate::W<CSCTL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSCTL3_SPEC>;
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
impl From<crate::W<CSCTL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSCTL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FCNTLF` reader - Start flag counter for LFXT"]
pub type FCNTLF_R = crate::FieldReader<u8, FCNTLF_A>;
#[doc = "Start flag counter for LFXT\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FCNTLF_A {
    #[doc = "0: 4096 cycles"]
    FCNTLF_0 = 0,
    #[doc = "1: 8192 cycles"]
    FCNTLF_1 = 1,
    #[doc = "2: 16384 cycles"]
    FCNTLF_2 = 2,
    #[doc = "3: 32768 cycles"]
    FCNTLF_3 = 3,
}
impl From<FCNTLF_A> for u8 {
    #[inline(always)]
    fn from(variant: FCNTLF_A) -> Self {
        variant as _
    }
}
impl FCNTLF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCNTLF_A {
        match self.bits {
            0 => FCNTLF_A::FCNTLF_0,
            1 => FCNTLF_A::FCNTLF_1,
            2 => FCNTLF_A::FCNTLF_2,
            3 => FCNTLF_A::FCNTLF_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FCNTLF_0`"]
    #[inline(always)]
    pub fn is_fcntlf_0(&self) -> bool {
        *self == FCNTLF_A::FCNTLF_0
    }
    #[doc = "Checks if the value of the field is `FCNTLF_1`"]
    #[inline(always)]
    pub fn is_fcntlf_1(&self) -> bool {
        *self == FCNTLF_A::FCNTLF_1
    }
    #[doc = "Checks if the value of the field is `FCNTLF_2`"]
    #[inline(always)]
    pub fn is_fcntlf_2(&self) -> bool {
        *self == FCNTLF_A::FCNTLF_2
    }
    #[doc = "Checks if the value of the field is `FCNTLF_3`"]
    #[inline(always)]
    pub fn is_fcntlf_3(&self) -> bool {
        *self == FCNTLF_A::FCNTLF_3
    }
}
#[doc = "Field `FCNTLF` writer - Start flag counter for LFXT"]
pub type FCNTLF_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CSCTL3_SPEC, u8, FCNTLF_A, 2, O>;
impl<'a, const O: u8> FCNTLF_W<'a, O> {
    #[doc = "4096 cycles"]
    #[inline(always)]
    pub fn fcntlf_0(self) -> &'a mut W {
        self.variant(FCNTLF_A::FCNTLF_0)
    }
    #[doc = "8192 cycles"]
    #[inline(always)]
    pub fn fcntlf_1(self) -> &'a mut W {
        self.variant(FCNTLF_A::FCNTLF_1)
    }
    #[doc = "16384 cycles"]
    #[inline(always)]
    pub fn fcntlf_2(self) -> &'a mut W {
        self.variant(FCNTLF_A::FCNTLF_2)
    }
    #[doc = "32768 cycles"]
    #[inline(always)]
    pub fn fcntlf_3(self) -> &'a mut W {
        self.variant(FCNTLF_A::FCNTLF_3)
    }
}
#[doc = "Reset start fault counter for LFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFCNTLF_ENUM_WRITE_AW {
    #[doc = "0: Not applicable. Always reads as zero due to self clearing."]
    RFCNTLF_0 = 0,
    #[doc = "1: Restarts the counter immediately."]
    RFCNTLF_1 = 1,
}
impl From<RFCNTLF_ENUM_WRITE_AW> for bool {
    #[inline(always)]
    fn from(variant: RFCNTLF_ENUM_WRITE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFCNTLF` writer - Reset start fault counter for LFXT"]
pub type RFCNTLF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSCTL3_SPEC, RFCNTLF_ENUM_WRITE_AW, O>;
impl<'a, const O: u8> RFCNTLF_W<'a, O> {
    #[doc = "Not applicable. Always reads as zero due to self clearing."]
    #[inline(always)]
    pub fn rfcntlf_0(self) -> &'a mut W {
        self.variant(RFCNTLF_ENUM_WRITE_AW::RFCNTLF_0)
    }
    #[doc = "Restarts the counter immediately."]
    #[inline(always)]
    pub fn rfcntlf_1(self) -> &'a mut W {
        self.variant(RFCNTLF_ENUM_WRITE_AW::RFCNTLF_1)
    }
}
#[doc = "Field `FCNTLF_EN` reader - Enable start fault counter for LFXT"]
pub type FCNTLF_EN_R = crate::BitReader<FCNTLF_EN_A>;
#[doc = "Enable start fault counter for LFXT\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCNTLF_EN_A {
    #[doc = "0: Startup fault counter disabled. Counter is cleared."]
    FCNTLF_EN_0 = 0,
    #[doc = "1: Startup fault counter enabled."]
    FCNTLF_EN_1 = 1,
}
impl From<FCNTLF_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FCNTLF_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FCNTLF_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCNTLF_EN_A {
        match self.bits {
            false => FCNTLF_EN_A::FCNTLF_EN_0,
            true => FCNTLF_EN_A::FCNTLF_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `FCNTLF_EN_0`"]
    #[inline(always)]
    pub fn is_fcntlf_en_0(&self) -> bool {
        *self == FCNTLF_EN_A::FCNTLF_EN_0
    }
    #[doc = "Checks if the value of the field is `FCNTLF_EN_1`"]
    #[inline(always)]
    pub fn is_fcntlf_en_1(&self) -> bool {
        *self == FCNTLF_EN_A::FCNTLF_EN_1
    }
}
#[doc = "Field `FCNTLF_EN` writer - Enable start fault counter for LFXT"]
pub type FCNTLF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSCTL3_SPEC, FCNTLF_EN_A, O>;
impl<'a, const O: u8> FCNTLF_EN_W<'a, O> {
    #[doc = "Startup fault counter disabled. Counter is cleared."]
    #[inline(always)]
    pub fn fcntlf_en_0(self) -> &'a mut W {
        self.variant(FCNTLF_EN_A::FCNTLF_EN_0)
    }
    #[doc = "Startup fault counter enabled."]
    #[inline(always)]
    pub fn fcntlf_en_1(self) -> &'a mut W {
        self.variant(FCNTLF_EN_A::FCNTLF_EN_1)
    }
}
#[doc = "Field `FCNTHF` reader - Start flag counter for HFXT"]
pub type FCNTHF_R = crate::FieldReader<u8, FCNTHF_A>;
#[doc = "Start flag counter for HFXT\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FCNTHF_A {
    #[doc = "0: 2048 cycles"]
    FCNTHF_0 = 0,
    #[doc = "1: 4096 cycles"]
    FCNTHF_1 = 1,
    #[doc = "2: 8192 cycles"]
    FCNTHF_2 = 2,
    #[doc = "3: 16384 cycles"]
    FCNTHF_3 = 3,
}
impl From<FCNTHF_A> for u8 {
    #[inline(always)]
    fn from(variant: FCNTHF_A) -> Self {
        variant as _
    }
}
impl FCNTHF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCNTHF_A {
        match self.bits {
            0 => FCNTHF_A::FCNTHF_0,
            1 => FCNTHF_A::FCNTHF_1,
            2 => FCNTHF_A::FCNTHF_2,
            3 => FCNTHF_A::FCNTHF_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FCNTHF_0`"]
    #[inline(always)]
    pub fn is_fcnthf_0(&self) -> bool {
        *self == FCNTHF_A::FCNTHF_0
    }
    #[doc = "Checks if the value of the field is `FCNTHF_1`"]
    #[inline(always)]
    pub fn is_fcnthf_1(&self) -> bool {
        *self == FCNTHF_A::FCNTHF_1
    }
    #[doc = "Checks if the value of the field is `FCNTHF_2`"]
    #[inline(always)]
    pub fn is_fcnthf_2(&self) -> bool {
        *self == FCNTHF_A::FCNTHF_2
    }
    #[doc = "Checks if the value of the field is `FCNTHF_3`"]
    #[inline(always)]
    pub fn is_fcnthf_3(&self) -> bool {
        *self == FCNTHF_A::FCNTHF_3
    }
}
#[doc = "Field `FCNTHF` writer - Start flag counter for HFXT"]
pub type FCNTHF_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CSCTL3_SPEC, u8, FCNTHF_A, 2, O>;
impl<'a, const O: u8> FCNTHF_W<'a, O> {
    #[doc = "2048 cycles"]
    #[inline(always)]
    pub fn fcnthf_0(self) -> &'a mut W {
        self.variant(FCNTHF_A::FCNTHF_0)
    }
    #[doc = "4096 cycles"]
    #[inline(always)]
    pub fn fcnthf_1(self) -> &'a mut W {
        self.variant(FCNTHF_A::FCNTHF_1)
    }
    #[doc = "8192 cycles"]
    #[inline(always)]
    pub fn fcnthf_2(self) -> &'a mut W {
        self.variant(FCNTHF_A::FCNTHF_2)
    }
    #[doc = "16384 cycles"]
    #[inline(always)]
    pub fn fcnthf_3(self) -> &'a mut W {
        self.variant(FCNTHF_A::FCNTHF_3)
    }
}
#[doc = "Reset start fault counter for HFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFCNTHF_ENUM_WRITE_AW {
    #[doc = "0: Not applicable. Always reads as zero due to self clearing."]
    RFCNTHF_0 = 0,
    #[doc = "1: Restarts the counter immediately."]
    RFCNTHF_1 = 1,
}
impl From<RFCNTHF_ENUM_WRITE_AW> for bool {
    #[inline(always)]
    fn from(variant: RFCNTHF_ENUM_WRITE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFCNTHF` writer - Reset start fault counter for HFXT"]
pub type RFCNTHF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSCTL3_SPEC, RFCNTHF_ENUM_WRITE_AW, O>;
impl<'a, const O: u8> RFCNTHF_W<'a, O> {
    #[doc = "Not applicable. Always reads as zero due to self clearing."]
    #[inline(always)]
    pub fn rfcnthf_0(self) -> &'a mut W {
        self.variant(RFCNTHF_ENUM_WRITE_AW::RFCNTHF_0)
    }
    #[doc = "Restarts the counter immediately."]
    #[inline(always)]
    pub fn rfcnthf_1(self) -> &'a mut W {
        self.variant(RFCNTHF_ENUM_WRITE_AW::RFCNTHF_1)
    }
}
#[doc = "Field `FCNTHF_EN` reader - Enable start fault counter for HFXT"]
pub type FCNTHF_EN_R = crate::BitReader<FCNTHF_EN_A>;
#[doc = "Enable start fault counter for HFXT\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCNTHF_EN_A {
    #[doc = "0: Startup fault counter disabled. Counter is cleared."]
    FCNTHF_EN_0 = 0,
    #[doc = "1: Startup fault counter enabled."]
    FCNTHF_EN_1 = 1,
}
impl From<FCNTHF_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FCNTHF_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FCNTHF_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCNTHF_EN_A {
        match self.bits {
            false => FCNTHF_EN_A::FCNTHF_EN_0,
            true => FCNTHF_EN_A::FCNTHF_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `FCNTHF_EN_0`"]
    #[inline(always)]
    pub fn is_fcnthf_en_0(&self) -> bool {
        *self == FCNTHF_EN_A::FCNTHF_EN_0
    }
    #[doc = "Checks if the value of the field is `FCNTHF_EN_1`"]
    #[inline(always)]
    pub fn is_fcnthf_en_1(&self) -> bool {
        *self == FCNTHF_EN_A::FCNTHF_EN_1
    }
}
#[doc = "Field `FCNTHF_EN` writer - Enable start fault counter for HFXT"]
pub type FCNTHF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSCTL3_SPEC, FCNTHF_EN_A, O>;
impl<'a, const O: u8> FCNTHF_EN_W<'a, O> {
    #[doc = "Startup fault counter disabled. Counter is cleared."]
    #[inline(always)]
    pub fn fcnthf_en_0(self) -> &'a mut W {
        self.variant(FCNTHF_EN_A::FCNTHF_EN_0)
    }
    #[doc = "Startup fault counter enabled."]
    #[inline(always)]
    pub fn fcnthf_en_1(self) -> &'a mut W {
        self.variant(FCNTHF_EN_A::FCNTHF_EN_1)
    }
}
#[doc = "Field `FCNTHF2` reader - Start flag counter for HFXT2"]
pub type FCNTHF2_R = crate::FieldReader<u8, FCNTHF2_A>;
#[doc = "Start flag counter for HFXT2\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FCNTHF2_A {
    #[doc = "0: 2048 cycles"]
    FCNTHF2_0 = 0,
    #[doc = "1: 4096 cycles"]
    FCNTHF2_1 = 1,
    #[doc = "2: 8192 cycles"]
    FCNTHF2_2 = 2,
    #[doc = "3: 16384 cycles"]
    FCNTHF2_3 = 3,
}
impl From<FCNTHF2_A> for u8 {
    #[inline(always)]
    fn from(variant: FCNTHF2_A) -> Self {
        variant as _
    }
}
impl FCNTHF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCNTHF2_A {
        match self.bits {
            0 => FCNTHF2_A::FCNTHF2_0,
            1 => FCNTHF2_A::FCNTHF2_1,
            2 => FCNTHF2_A::FCNTHF2_2,
            3 => FCNTHF2_A::FCNTHF2_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FCNTHF2_0`"]
    #[inline(always)]
    pub fn is_fcnthf2_0(&self) -> bool {
        *self == FCNTHF2_A::FCNTHF2_0
    }
    #[doc = "Checks if the value of the field is `FCNTHF2_1`"]
    #[inline(always)]
    pub fn is_fcnthf2_1(&self) -> bool {
        *self == FCNTHF2_A::FCNTHF2_1
    }
    #[doc = "Checks if the value of the field is `FCNTHF2_2`"]
    #[inline(always)]
    pub fn is_fcnthf2_2(&self) -> bool {
        *self == FCNTHF2_A::FCNTHF2_2
    }
    #[doc = "Checks if the value of the field is `FCNTHF2_3`"]
    #[inline(always)]
    pub fn is_fcnthf2_3(&self) -> bool {
        *self == FCNTHF2_A::FCNTHF2_3
    }
}
#[doc = "Field `FCNTHF2` writer - Start flag counter for HFXT2"]
pub type FCNTHF2_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CSCTL3_SPEC, u8, FCNTHF2_A, 2, O>;
impl<'a, const O: u8> FCNTHF2_W<'a, O> {
    #[doc = "2048 cycles"]
    #[inline(always)]
    pub fn fcnthf2_0(self) -> &'a mut W {
        self.variant(FCNTHF2_A::FCNTHF2_0)
    }
    #[doc = "4096 cycles"]
    #[inline(always)]
    pub fn fcnthf2_1(self) -> &'a mut W {
        self.variant(FCNTHF2_A::FCNTHF2_1)
    }
    #[doc = "8192 cycles"]
    #[inline(always)]
    pub fn fcnthf2_2(self) -> &'a mut W {
        self.variant(FCNTHF2_A::FCNTHF2_2)
    }
    #[doc = "16384 cycles"]
    #[inline(always)]
    pub fn fcnthf2_3(self) -> &'a mut W {
        self.variant(FCNTHF2_A::FCNTHF2_3)
    }
}
#[doc = "Reset start fault counter for HFXT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFCNTHF2_ENUM_WRITE_AW {
    #[doc = "0: Not applicable. Always reads as zero due to self clearing."]
    RFCNTHF2_0 = 0,
    #[doc = "1: Restarts the counter immediately."]
    RFCNTHF2_1 = 1,
}
impl From<RFCNTHF2_ENUM_WRITE_AW> for bool {
    #[inline(always)]
    fn from(variant: RFCNTHF2_ENUM_WRITE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFCNTHF2` writer - Reset start fault counter for HFXT2"]
pub type RFCNTHF2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSCTL3_SPEC, RFCNTHF2_ENUM_WRITE_AW, O>;
impl<'a, const O: u8> RFCNTHF2_W<'a, O> {
    #[doc = "Not applicable. Always reads as zero due to self clearing."]
    #[inline(always)]
    pub fn rfcnthf2_0(self) -> &'a mut W {
        self.variant(RFCNTHF2_ENUM_WRITE_AW::RFCNTHF2_0)
    }
    #[doc = "Restarts the counter immediately."]
    #[inline(always)]
    pub fn rfcnthf2_1(self) -> &'a mut W {
        self.variant(RFCNTHF2_ENUM_WRITE_AW::RFCNTHF2_1)
    }
}
#[doc = "Field `FCNTHF2_EN` reader - Enable start fault counter for HFXT2"]
pub type FCNTHF2_EN_R = crate::BitReader<FCNTHF2_EN_A>;
#[doc = "Enable start fault counter for HFXT2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCNTHF2_EN_A {
    #[doc = "0: Startup fault counter disabled. Counter is cleared."]
    FCNTHF2_EN_0 = 0,
    #[doc = "1: Startup fault counter enabled."]
    FCNTHF2_EN_1 = 1,
}
impl From<FCNTHF2_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FCNTHF2_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FCNTHF2_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCNTHF2_EN_A {
        match self.bits {
            false => FCNTHF2_EN_A::FCNTHF2_EN_0,
            true => FCNTHF2_EN_A::FCNTHF2_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `FCNTHF2_EN_0`"]
    #[inline(always)]
    pub fn is_fcnthf2_en_0(&self) -> bool {
        *self == FCNTHF2_EN_A::FCNTHF2_EN_0
    }
    #[doc = "Checks if the value of the field is `FCNTHF2_EN_1`"]
    #[inline(always)]
    pub fn is_fcnthf2_en_1(&self) -> bool {
        *self == FCNTHF2_EN_A::FCNTHF2_EN_1
    }
}
#[doc = "Field `FCNTHF2_EN` writer - Enable start fault counter for HFXT2"]
pub type FCNTHF2_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSCTL3_SPEC, FCNTHF2_EN_A, O>;
impl<'a, const O: u8> FCNTHF2_EN_W<'a, O> {
    #[doc = "Startup fault counter disabled. Counter is cleared."]
    #[inline(always)]
    pub fn fcnthf2_en_0(self) -> &'a mut W {
        self.variant(FCNTHF2_EN_A::FCNTHF2_EN_0)
    }
    #[doc = "Startup fault counter enabled."]
    #[inline(always)]
    pub fn fcnthf2_en_1(self) -> &'a mut W {
        self.variant(FCNTHF2_EN_A::FCNTHF2_EN_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Start flag counter for LFXT"]
    #[inline(always)]
    pub fn fcntlf(&self) -> FCNTLF_R {
        FCNTLF_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Enable start fault counter for LFXT"]
    #[inline(always)]
    pub fn fcntlf_en(&self) -> FCNTLF_EN_R {
        FCNTLF_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Start flag counter for HFXT"]
    #[inline(always)]
    pub fn fcnthf(&self) -> FCNTHF_R {
        FCNTHF_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Enable start fault counter for HFXT"]
    #[inline(always)]
    pub fn fcnthf_en(&self) -> FCNTHF_EN_R {
        FCNTHF_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Start flag counter for HFXT2"]
    #[inline(always)]
    pub fn fcnthf2(&self) -> FCNTHF2_R {
        FCNTHF2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - Enable start fault counter for HFXT2"]
    #[inline(always)]
    pub fn fcnthf2_en(&self) -> FCNTHF2_EN_R {
        FCNTHF2_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Start flag counter for LFXT"]
    #[inline(always)]
    pub fn fcntlf(&mut self) -> FCNTLF_W<0> {
        FCNTLF_W::new(self)
    }
    #[doc = "Bit 2 - Reset start fault counter for LFXT"]
    #[inline(always)]
    pub fn rfcntlf(&mut self) -> RFCNTLF_W<2> {
        RFCNTLF_W::new(self)
    }
    #[doc = "Bit 3 - Enable start fault counter for LFXT"]
    #[inline(always)]
    pub fn fcntlf_en(&mut self) -> FCNTLF_EN_W<3> {
        FCNTLF_EN_W::new(self)
    }
    #[doc = "Bits 4:5 - Start flag counter for HFXT"]
    #[inline(always)]
    pub fn fcnthf(&mut self) -> FCNTHF_W<4> {
        FCNTHF_W::new(self)
    }
    #[doc = "Bit 6 - Reset start fault counter for HFXT"]
    #[inline(always)]
    pub fn rfcnthf(&mut self) -> RFCNTHF_W<6> {
        RFCNTHF_W::new(self)
    }
    #[doc = "Bit 7 - Enable start fault counter for HFXT"]
    #[inline(always)]
    pub fn fcnthf_en(&mut self) -> FCNTHF_EN_W<7> {
        FCNTHF_EN_W::new(self)
    }
    #[doc = "Bits 8:9 - Start flag counter for HFXT2"]
    #[inline(always)]
    pub fn fcnthf2(&mut self) -> FCNTHF2_W<8> {
        FCNTHF2_W::new(self)
    }
    #[doc = "Bit 10 - Reset start fault counter for HFXT2"]
    #[inline(always)]
    pub fn rfcnthf2(&mut self) -> RFCNTHF2_W<10> {
        RFCNTHF2_W::new(self)
    }
    #[doc = "Bit 11 - Enable start fault counter for HFXT2"]
    #[inline(always)]
    pub fn fcnthf2_en(&mut self) -> FCNTHF2_EN_W<11> {
        FCNTHF2_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csctl3](index.html) module"]
pub struct CSCTL3_SPEC;
impl crate::RegisterSpec for CSCTL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csctl3::R](R) reader structure"]
impl crate::Readable for CSCTL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csctl3::W](W) writer structure"]
impl crate::Writable for CSCTL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSCTL3 to value 0x0bbb"]
impl crate::Resettable for CSCTL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0bbb
    }
}
