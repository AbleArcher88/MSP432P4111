#[doc = "Register `LCDPCTL1` reader"]
pub struct R(crate::R<LCDPCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDPCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDPCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDPCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDPCTL1` writer"]
pub struct W(crate::W<LCDPCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDPCTL1_SPEC>;
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
impl From<crate::W<LCDPCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCDPCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDS32` reader - LCD pin 32 enable"]
pub type LCDS32_R = crate::BitReader<LCDS32_A>;
#[doc = "LCD pin 32 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS32_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS32_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS32_1 = 1,
}
impl From<LCDS32_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS32_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS32_A {
        match self.bits {
            false => LCDS32_A::LCDS32_0,
            true => LCDS32_A::LCDS32_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS32_0`"]
    #[inline(always)]
    pub fn is_lcds32_0(&self) -> bool {
        *self == LCDS32_A::LCDS32_0
    }
    #[doc = "Checks if the value of the field is `LCDS32_1`"]
    #[inline(always)]
    pub fn is_lcds32_1(&self) -> bool {
        *self == LCDS32_A::LCDS32_1
    }
}
#[doc = "Field `LCDS32` writer - LCD pin 32 enable"]
pub type LCDS32_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS32_A, O>;
impl<'a, const O: u8> LCDS32_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds32_0(self) -> &'a mut W {
        self.variant(LCDS32_A::LCDS32_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds32_1(self) -> &'a mut W {
        self.variant(LCDS32_A::LCDS32_1)
    }
}
#[doc = "Field `LCDS33` reader - LCD pin 33 enable"]
pub type LCDS33_R = crate::BitReader<LCDS33_A>;
#[doc = "LCD pin 33 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS33_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS33_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS33_1 = 1,
}
impl From<LCDS33_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS33_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS33_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS33_A {
        match self.bits {
            false => LCDS33_A::LCDS33_0,
            true => LCDS33_A::LCDS33_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS33_0`"]
    #[inline(always)]
    pub fn is_lcds33_0(&self) -> bool {
        *self == LCDS33_A::LCDS33_0
    }
    #[doc = "Checks if the value of the field is `LCDS33_1`"]
    #[inline(always)]
    pub fn is_lcds33_1(&self) -> bool {
        *self == LCDS33_A::LCDS33_1
    }
}
#[doc = "Field `LCDS33` writer - LCD pin 33 enable"]
pub type LCDS33_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS33_A, O>;
impl<'a, const O: u8> LCDS33_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds33_0(self) -> &'a mut W {
        self.variant(LCDS33_A::LCDS33_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds33_1(self) -> &'a mut W {
        self.variant(LCDS33_A::LCDS33_1)
    }
}
#[doc = "Field `LCDS34` reader - LCD pin 34 enable"]
pub type LCDS34_R = crate::BitReader<LCDS34_A>;
#[doc = "LCD pin 34 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS34_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS34_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS34_1 = 1,
}
impl From<LCDS34_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS34_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS34_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS34_A {
        match self.bits {
            false => LCDS34_A::LCDS34_0,
            true => LCDS34_A::LCDS34_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS34_0`"]
    #[inline(always)]
    pub fn is_lcds34_0(&self) -> bool {
        *self == LCDS34_A::LCDS34_0
    }
    #[doc = "Checks if the value of the field is `LCDS34_1`"]
    #[inline(always)]
    pub fn is_lcds34_1(&self) -> bool {
        *self == LCDS34_A::LCDS34_1
    }
}
#[doc = "Field `LCDS34` writer - LCD pin 34 enable"]
pub type LCDS34_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS34_A, O>;
impl<'a, const O: u8> LCDS34_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds34_0(self) -> &'a mut W {
        self.variant(LCDS34_A::LCDS34_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds34_1(self) -> &'a mut W {
        self.variant(LCDS34_A::LCDS34_1)
    }
}
#[doc = "Field `LCDS35` reader - LCD pin 35 enable"]
pub type LCDS35_R = crate::BitReader<LCDS35_A>;
#[doc = "LCD pin 35 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS35_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS35_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS35_1 = 1,
}
impl From<LCDS35_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS35_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS35_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS35_A {
        match self.bits {
            false => LCDS35_A::LCDS35_0,
            true => LCDS35_A::LCDS35_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS35_0`"]
    #[inline(always)]
    pub fn is_lcds35_0(&self) -> bool {
        *self == LCDS35_A::LCDS35_0
    }
    #[doc = "Checks if the value of the field is `LCDS35_1`"]
    #[inline(always)]
    pub fn is_lcds35_1(&self) -> bool {
        *self == LCDS35_A::LCDS35_1
    }
}
#[doc = "Field `LCDS35` writer - LCD pin 35 enable"]
pub type LCDS35_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS35_A, O>;
impl<'a, const O: u8> LCDS35_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds35_0(self) -> &'a mut W {
        self.variant(LCDS35_A::LCDS35_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds35_1(self) -> &'a mut W {
        self.variant(LCDS35_A::LCDS35_1)
    }
}
#[doc = "Field `LCDS36` reader - LCD pin 36 enable"]
pub type LCDS36_R = crate::BitReader<LCDS36_A>;
#[doc = "LCD pin 36 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS36_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS36_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS36_1 = 1,
}
impl From<LCDS36_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS36_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS36_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS36_A {
        match self.bits {
            false => LCDS36_A::LCDS36_0,
            true => LCDS36_A::LCDS36_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS36_0`"]
    #[inline(always)]
    pub fn is_lcds36_0(&self) -> bool {
        *self == LCDS36_A::LCDS36_0
    }
    #[doc = "Checks if the value of the field is `LCDS36_1`"]
    #[inline(always)]
    pub fn is_lcds36_1(&self) -> bool {
        *self == LCDS36_A::LCDS36_1
    }
}
#[doc = "Field `LCDS36` writer - LCD pin 36 enable"]
pub type LCDS36_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS36_A, O>;
impl<'a, const O: u8> LCDS36_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds36_0(self) -> &'a mut W {
        self.variant(LCDS36_A::LCDS36_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds36_1(self) -> &'a mut W {
        self.variant(LCDS36_A::LCDS36_1)
    }
}
#[doc = "Field `LCDS37` reader - LCD pin 37 enable"]
pub type LCDS37_R = crate::BitReader<LCDS37_A>;
#[doc = "LCD pin 37 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS37_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS37_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS37_1 = 1,
}
impl From<LCDS37_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS37_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS37_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS37_A {
        match self.bits {
            false => LCDS37_A::LCDS37_0,
            true => LCDS37_A::LCDS37_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS37_0`"]
    #[inline(always)]
    pub fn is_lcds37_0(&self) -> bool {
        *self == LCDS37_A::LCDS37_0
    }
    #[doc = "Checks if the value of the field is `LCDS37_1`"]
    #[inline(always)]
    pub fn is_lcds37_1(&self) -> bool {
        *self == LCDS37_A::LCDS37_1
    }
}
#[doc = "Field `LCDS37` writer - LCD pin 37 enable"]
pub type LCDS37_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS37_A, O>;
impl<'a, const O: u8> LCDS37_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds37_0(self) -> &'a mut W {
        self.variant(LCDS37_A::LCDS37_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds37_1(self) -> &'a mut W {
        self.variant(LCDS37_A::LCDS37_1)
    }
}
#[doc = "Field `LCDS38` reader - LCD pin 38 enable"]
pub type LCDS38_R = crate::BitReader<LCDS38_A>;
#[doc = "LCD pin 38 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS38_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS38_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS38_1 = 1,
}
impl From<LCDS38_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS38_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS38_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS38_A {
        match self.bits {
            false => LCDS38_A::LCDS38_0,
            true => LCDS38_A::LCDS38_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS38_0`"]
    #[inline(always)]
    pub fn is_lcds38_0(&self) -> bool {
        *self == LCDS38_A::LCDS38_0
    }
    #[doc = "Checks if the value of the field is `LCDS38_1`"]
    #[inline(always)]
    pub fn is_lcds38_1(&self) -> bool {
        *self == LCDS38_A::LCDS38_1
    }
}
#[doc = "Field `LCDS38` writer - LCD pin 38 enable"]
pub type LCDS38_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS38_A, O>;
impl<'a, const O: u8> LCDS38_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds38_0(self) -> &'a mut W {
        self.variant(LCDS38_A::LCDS38_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds38_1(self) -> &'a mut W {
        self.variant(LCDS38_A::LCDS38_1)
    }
}
#[doc = "Field `LCDS39` reader - LCD pin 39 enable"]
pub type LCDS39_R = crate::BitReader<LCDS39_A>;
#[doc = "LCD pin 39 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS39_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS39_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS39_1 = 1,
}
impl From<LCDS39_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS39_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS39_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS39_A {
        match self.bits {
            false => LCDS39_A::LCDS39_0,
            true => LCDS39_A::LCDS39_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS39_0`"]
    #[inline(always)]
    pub fn is_lcds39_0(&self) -> bool {
        *self == LCDS39_A::LCDS39_0
    }
    #[doc = "Checks if the value of the field is `LCDS39_1`"]
    #[inline(always)]
    pub fn is_lcds39_1(&self) -> bool {
        *self == LCDS39_A::LCDS39_1
    }
}
#[doc = "Field `LCDS39` writer - LCD pin 39 enable"]
pub type LCDS39_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS39_A, O>;
impl<'a, const O: u8> LCDS39_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds39_0(self) -> &'a mut W {
        self.variant(LCDS39_A::LCDS39_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds39_1(self) -> &'a mut W {
        self.variant(LCDS39_A::LCDS39_1)
    }
}
#[doc = "Field `LCDS40` reader - LCD pin 40 enable"]
pub type LCDS40_R = crate::BitReader<LCDS40_A>;
#[doc = "LCD pin 40 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS40_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS40_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS40_1 = 1,
}
impl From<LCDS40_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS40_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS40_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS40_A {
        match self.bits {
            false => LCDS40_A::LCDS40_0,
            true => LCDS40_A::LCDS40_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS40_0`"]
    #[inline(always)]
    pub fn is_lcds40_0(&self) -> bool {
        *self == LCDS40_A::LCDS40_0
    }
    #[doc = "Checks if the value of the field is `LCDS40_1`"]
    #[inline(always)]
    pub fn is_lcds40_1(&self) -> bool {
        *self == LCDS40_A::LCDS40_1
    }
}
#[doc = "Field `LCDS40` writer - LCD pin 40 enable"]
pub type LCDS40_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS40_A, O>;
impl<'a, const O: u8> LCDS40_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds40_0(self) -> &'a mut W {
        self.variant(LCDS40_A::LCDS40_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds40_1(self) -> &'a mut W {
        self.variant(LCDS40_A::LCDS40_1)
    }
}
#[doc = "Field `LCDS41` reader - LCD pin 41 enable"]
pub type LCDS41_R = crate::BitReader<LCDS41_A>;
#[doc = "LCD pin 41 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS41_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS41_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS41_1 = 1,
}
impl From<LCDS41_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS41_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS41_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS41_A {
        match self.bits {
            false => LCDS41_A::LCDS41_0,
            true => LCDS41_A::LCDS41_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS41_0`"]
    #[inline(always)]
    pub fn is_lcds41_0(&self) -> bool {
        *self == LCDS41_A::LCDS41_0
    }
    #[doc = "Checks if the value of the field is `LCDS41_1`"]
    #[inline(always)]
    pub fn is_lcds41_1(&self) -> bool {
        *self == LCDS41_A::LCDS41_1
    }
}
#[doc = "Field `LCDS41` writer - LCD pin 41 enable"]
pub type LCDS41_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS41_A, O>;
impl<'a, const O: u8> LCDS41_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds41_0(self) -> &'a mut W {
        self.variant(LCDS41_A::LCDS41_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds41_1(self) -> &'a mut W {
        self.variant(LCDS41_A::LCDS41_1)
    }
}
#[doc = "Field `LCDS42` reader - LCD pin 42 enable"]
pub type LCDS42_R = crate::BitReader<LCDS42_A>;
#[doc = "LCD pin 42 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS42_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS42_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS42_1 = 1,
}
impl From<LCDS42_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS42_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS42_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS42_A {
        match self.bits {
            false => LCDS42_A::LCDS42_0,
            true => LCDS42_A::LCDS42_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS42_0`"]
    #[inline(always)]
    pub fn is_lcds42_0(&self) -> bool {
        *self == LCDS42_A::LCDS42_0
    }
    #[doc = "Checks if the value of the field is `LCDS42_1`"]
    #[inline(always)]
    pub fn is_lcds42_1(&self) -> bool {
        *self == LCDS42_A::LCDS42_1
    }
}
#[doc = "Field `LCDS42` writer - LCD pin 42 enable"]
pub type LCDS42_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS42_A, O>;
impl<'a, const O: u8> LCDS42_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds42_0(self) -> &'a mut W {
        self.variant(LCDS42_A::LCDS42_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds42_1(self) -> &'a mut W {
        self.variant(LCDS42_A::LCDS42_1)
    }
}
#[doc = "Field `LCDS43` reader - LCD pin 43 enable"]
pub type LCDS43_R = crate::BitReader<LCDS43_A>;
#[doc = "LCD pin 43 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS43_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS43_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS43_1 = 1,
}
impl From<LCDS43_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS43_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS43_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS43_A {
        match self.bits {
            false => LCDS43_A::LCDS43_0,
            true => LCDS43_A::LCDS43_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS43_0`"]
    #[inline(always)]
    pub fn is_lcds43_0(&self) -> bool {
        *self == LCDS43_A::LCDS43_0
    }
    #[doc = "Checks if the value of the field is `LCDS43_1`"]
    #[inline(always)]
    pub fn is_lcds43_1(&self) -> bool {
        *self == LCDS43_A::LCDS43_1
    }
}
#[doc = "Field `LCDS43` writer - LCD pin 43 enable"]
pub type LCDS43_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS43_A, O>;
impl<'a, const O: u8> LCDS43_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds43_0(self) -> &'a mut W {
        self.variant(LCDS43_A::LCDS43_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds43_1(self) -> &'a mut W {
        self.variant(LCDS43_A::LCDS43_1)
    }
}
#[doc = "Field `LCDS44` reader - LCD pin 44 enable"]
pub type LCDS44_R = crate::BitReader<LCDS44_A>;
#[doc = "LCD pin 44 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS44_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS44_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS44_1 = 1,
}
impl From<LCDS44_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS44_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS44_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS44_A {
        match self.bits {
            false => LCDS44_A::LCDS44_0,
            true => LCDS44_A::LCDS44_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS44_0`"]
    #[inline(always)]
    pub fn is_lcds44_0(&self) -> bool {
        *self == LCDS44_A::LCDS44_0
    }
    #[doc = "Checks if the value of the field is `LCDS44_1`"]
    #[inline(always)]
    pub fn is_lcds44_1(&self) -> bool {
        *self == LCDS44_A::LCDS44_1
    }
}
#[doc = "Field `LCDS44` writer - LCD pin 44 enable"]
pub type LCDS44_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS44_A, O>;
impl<'a, const O: u8> LCDS44_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds44_0(self) -> &'a mut W {
        self.variant(LCDS44_A::LCDS44_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds44_1(self) -> &'a mut W {
        self.variant(LCDS44_A::LCDS44_1)
    }
}
#[doc = "Field `LCDS45` reader - LCD pin 45 enable"]
pub type LCDS45_R = crate::BitReader<LCDS45_A>;
#[doc = "LCD pin 45 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS45_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS45_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS45_1 = 1,
}
impl From<LCDS45_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS45_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS45_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS45_A {
        match self.bits {
            false => LCDS45_A::LCDS45_0,
            true => LCDS45_A::LCDS45_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS45_0`"]
    #[inline(always)]
    pub fn is_lcds45_0(&self) -> bool {
        *self == LCDS45_A::LCDS45_0
    }
    #[doc = "Checks if the value of the field is `LCDS45_1`"]
    #[inline(always)]
    pub fn is_lcds45_1(&self) -> bool {
        *self == LCDS45_A::LCDS45_1
    }
}
#[doc = "Field `LCDS45` writer - LCD pin 45 enable"]
pub type LCDS45_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS45_A, O>;
impl<'a, const O: u8> LCDS45_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds45_0(self) -> &'a mut W {
        self.variant(LCDS45_A::LCDS45_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds45_1(self) -> &'a mut W {
        self.variant(LCDS45_A::LCDS45_1)
    }
}
#[doc = "Field `LCDS46` reader - LCD pin 46 enable"]
pub type LCDS46_R = crate::BitReader<LCDS46_A>;
#[doc = "LCD pin 46 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS46_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS46_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS46_1 = 1,
}
impl From<LCDS46_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS46_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS46_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS46_A {
        match self.bits {
            false => LCDS46_A::LCDS46_0,
            true => LCDS46_A::LCDS46_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS46_0`"]
    #[inline(always)]
    pub fn is_lcds46_0(&self) -> bool {
        *self == LCDS46_A::LCDS46_0
    }
    #[doc = "Checks if the value of the field is `LCDS46_1`"]
    #[inline(always)]
    pub fn is_lcds46_1(&self) -> bool {
        *self == LCDS46_A::LCDS46_1
    }
}
#[doc = "Field `LCDS46` writer - LCD pin 46 enable"]
pub type LCDS46_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS46_A, O>;
impl<'a, const O: u8> LCDS46_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds46_0(self) -> &'a mut W {
        self.variant(LCDS46_A::LCDS46_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds46_1(self) -> &'a mut W {
        self.variant(LCDS46_A::LCDS46_1)
    }
}
#[doc = "Field `LCDS47` reader - LCD pin 47 enable"]
pub type LCDS47_R = crate::BitReader<LCDS47_A>;
#[doc = "LCD pin 47 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS47_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS47_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS47_1 = 1,
}
impl From<LCDS47_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS47_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS47_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS47_A {
        match self.bits {
            false => LCDS47_A::LCDS47_0,
            true => LCDS47_A::LCDS47_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS47_0`"]
    #[inline(always)]
    pub fn is_lcds47_0(&self) -> bool {
        *self == LCDS47_A::LCDS47_0
    }
    #[doc = "Checks if the value of the field is `LCDS47_1`"]
    #[inline(always)]
    pub fn is_lcds47_1(&self) -> bool {
        *self == LCDS47_A::LCDS47_1
    }
}
#[doc = "Field `LCDS47` writer - LCD pin 47 enable"]
pub type LCDS47_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS47_A, O>;
impl<'a, const O: u8> LCDS47_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds47_0(self) -> &'a mut W {
        self.variant(LCDS47_A::LCDS47_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds47_1(self) -> &'a mut W {
        self.variant(LCDS47_A::LCDS47_1)
    }
}
#[doc = "Field `LCDS48` reader - LCD pin 48 enable"]
pub type LCDS48_R = crate::BitReader<LCDS48_A>;
#[doc = "LCD pin 48 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS48_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS48_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS48_1 = 1,
}
impl From<LCDS48_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS48_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS48_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS48_A {
        match self.bits {
            false => LCDS48_A::LCDS48_0,
            true => LCDS48_A::LCDS48_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS48_0`"]
    #[inline(always)]
    pub fn is_lcds48_0(&self) -> bool {
        *self == LCDS48_A::LCDS48_0
    }
    #[doc = "Checks if the value of the field is `LCDS48_1`"]
    #[inline(always)]
    pub fn is_lcds48_1(&self) -> bool {
        *self == LCDS48_A::LCDS48_1
    }
}
#[doc = "Field `LCDS48` writer - LCD pin 48 enable"]
pub type LCDS48_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS48_A, O>;
impl<'a, const O: u8> LCDS48_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds48_0(self) -> &'a mut W {
        self.variant(LCDS48_A::LCDS48_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds48_1(self) -> &'a mut W {
        self.variant(LCDS48_A::LCDS48_1)
    }
}
#[doc = "Field `LCDS49` reader - LCD pin 49 enable"]
pub type LCDS49_R = crate::BitReader<LCDS49_A>;
#[doc = "LCD pin 49 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS49_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS49_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS49_1 = 1,
}
impl From<LCDS49_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS49_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS49_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS49_A {
        match self.bits {
            false => LCDS49_A::LCDS49_0,
            true => LCDS49_A::LCDS49_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS49_0`"]
    #[inline(always)]
    pub fn is_lcds49_0(&self) -> bool {
        *self == LCDS49_A::LCDS49_0
    }
    #[doc = "Checks if the value of the field is `LCDS49_1`"]
    #[inline(always)]
    pub fn is_lcds49_1(&self) -> bool {
        *self == LCDS49_A::LCDS49_1
    }
}
#[doc = "Field `LCDS49` writer - LCD pin 49 enable"]
pub type LCDS49_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS49_A, O>;
impl<'a, const O: u8> LCDS49_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds49_0(self) -> &'a mut W {
        self.variant(LCDS49_A::LCDS49_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds49_1(self) -> &'a mut W {
        self.variant(LCDS49_A::LCDS49_1)
    }
}
#[doc = "Field `LCDS50` reader - LCD pin 50 enable"]
pub type LCDS50_R = crate::BitReader<LCDS50_A>;
#[doc = "LCD pin 50 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS50_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS50_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS50_1 = 1,
}
impl From<LCDS50_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS50_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS50_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS50_A {
        match self.bits {
            false => LCDS50_A::LCDS50_0,
            true => LCDS50_A::LCDS50_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS50_0`"]
    #[inline(always)]
    pub fn is_lcds50_0(&self) -> bool {
        *self == LCDS50_A::LCDS50_0
    }
    #[doc = "Checks if the value of the field is `LCDS50_1`"]
    #[inline(always)]
    pub fn is_lcds50_1(&self) -> bool {
        *self == LCDS50_A::LCDS50_1
    }
}
#[doc = "Field `LCDS50` writer - LCD pin 50 enable"]
pub type LCDS50_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS50_A, O>;
impl<'a, const O: u8> LCDS50_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds50_0(self) -> &'a mut W {
        self.variant(LCDS50_A::LCDS50_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds50_1(self) -> &'a mut W {
        self.variant(LCDS50_A::LCDS50_1)
    }
}
#[doc = "Field `LCDS51` reader - LCD pin 51 enable"]
pub type LCDS51_R = crate::BitReader<LCDS51_A>;
#[doc = "LCD pin 51 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS51_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS51_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS51_1 = 1,
}
impl From<LCDS51_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS51_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS51_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS51_A {
        match self.bits {
            false => LCDS51_A::LCDS51_0,
            true => LCDS51_A::LCDS51_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS51_0`"]
    #[inline(always)]
    pub fn is_lcds51_0(&self) -> bool {
        *self == LCDS51_A::LCDS51_0
    }
    #[doc = "Checks if the value of the field is `LCDS51_1`"]
    #[inline(always)]
    pub fn is_lcds51_1(&self) -> bool {
        *self == LCDS51_A::LCDS51_1
    }
}
#[doc = "Field `LCDS51` writer - LCD pin 51 enable"]
pub type LCDS51_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS51_A, O>;
impl<'a, const O: u8> LCDS51_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds51_0(self) -> &'a mut W {
        self.variant(LCDS51_A::LCDS51_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds51_1(self) -> &'a mut W {
        self.variant(LCDS51_A::LCDS51_1)
    }
}
#[doc = "Field `LCDS52` reader - LCD pin 52 enable"]
pub type LCDS52_R = crate::BitReader<LCDS52_A>;
#[doc = "LCD pin 52 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS52_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS52_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS52_1 = 1,
}
impl From<LCDS52_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS52_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS52_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS52_A {
        match self.bits {
            false => LCDS52_A::LCDS52_0,
            true => LCDS52_A::LCDS52_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS52_0`"]
    #[inline(always)]
    pub fn is_lcds52_0(&self) -> bool {
        *self == LCDS52_A::LCDS52_0
    }
    #[doc = "Checks if the value of the field is `LCDS52_1`"]
    #[inline(always)]
    pub fn is_lcds52_1(&self) -> bool {
        *self == LCDS52_A::LCDS52_1
    }
}
#[doc = "Field `LCDS52` writer - LCD pin 52 enable"]
pub type LCDS52_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS52_A, O>;
impl<'a, const O: u8> LCDS52_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds52_0(self) -> &'a mut W {
        self.variant(LCDS52_A::LCDS52_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds52_1(self) -> &'a mut W {
        self.variant(LCDS52_A::LCDS52_1)
    }
}
#[doc = "Field `LCDS53` reader - LCD pin 53 enable"]
pub type LCDS53_R = crate::BitReader<LCDS53_A>;
#[doc = "LCD pin 53 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS53_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS53_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS53_1 = 1,
}
impl From<LCDS53_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS53_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS53_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS53_A {
        match self.bits {
            false => LCDS53_A::LCDS53_0,
            true => LCDS53_A::LCDS53_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS53_0`"]
    #[inline(always)]
    pub fn is_lcds53_0(&self) -> bool {
        *self == LCDS53_A::LCDS53_0
    }
    #[doc = "Checks if the value of the field is `LCDS53_1`"]
    #[inline(always)]
    pub fn is_lcds53_1(&self) -> bool {
        *self == LCDS53_A::LCDS53_1
    }
}
#[doc = "Field `LCDS53` writer - LCD pin 53 enable"]
pub type LCDS53_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS53_A, O>;
impl<'a, const O: u8> LCDS53_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds53_0(self) -> &'a mut W {
        self.variant(LCDS53_A::LCDS53_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds53_1(self) -> &'a mut W {
        self.variant(LCDS53_A::LCDS53_1)
    }
}
#[doc = "Field `LCDS54` reader - LCD pin 54 enable"]
pub type LCDS54_R = crate::BitReader<LCDS54_A>;
#[doc = "LCD pin 54 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS54_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS54_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS54_1 = 1,
}
impl From<LCDS54_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS54_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS54_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS54_A {
        match self.bits {
            false => LCDS54_A::LCDS54_0,
            true => LCDS54_A::LCDS54_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS54_0`"]
    #[inline(always)]
    pub fn is_lcds54_0(&self) -> bool {
        *self == LCDS54_A::LCDS54_0
    }
    #[doc = "Checks if the value of the field is `LCDS54_1`"]
    #[inline(always)]
    pub fn is_lcds54_1(&self) -> bool {
        *self == LCDS54_A::LCDS54_1
    }
}
#[doc = "Field `LCDS54` writer - LCD pin 54 enable"]
pub type LCDS54_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS54_A, O>;
impl<'a, const O: u8> LCDS54_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds54_0(self) -> &'a mut W {
        self.variant(LCDS54_A::LCDS54_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds54_1(self) -> &'a mut W {
        self.variant(LCDS54_A::LCDS54_1)
    }
}
#[doc = "Field `LCDS55` reader - LCD pin 55 enable"]
pub type LCDS55_R = crate::BitReader<LCDS55_A>;
#[doc = "LCD pin 55 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS55_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS55_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS55_1 = 1,
}
impl From<LCDS55_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS55_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS55_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS55_A {
        match self.bits {
            false => LCDS55_A::LCDS55_0,
            true => LCDS55_A::LCDS55_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS55_0`"]
    #[inline(always)]
    pub fn is_lcds55_0(&self) -> bool {
        *self == LCDS55_A::LCDS55_0
    }
    #[doc = "Checks if the value of the field is `LCDS55_1`"]
    #[inline(always)]
    pub fn is_lcds55_1(&self) -> bool {
        *self == LCDS55_A::LCDS55_1
    }
}
#[doc = "Field `LCDS55` writer - LCD pin 55 enable"]
pub type LCDS55_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS55_A, O>;
impl<'a, const O: u8> LCDS55_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds55_0(self) -> &'a mut W {
        self.variant(LCDS55_A::LCDS55_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds55_1(self) -> &'a mut W {
        self.variant(LCDS55_A::LCDS55_1)
    }
}
#[doc = "Field `LCDS56` reader - LCD pin 56 enable"]
pub type LCDS56_R = crate::BitReader<LCDS56_A>;
#[doc = "LCD pin 56 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS56_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS56_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS56_1 = 1,
}
impl From<LCDS56_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS56_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS56_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS56_A {
        match self.bits {
            false => LCDS56_A::LCDS56_0,
            true => LCDS56_A::LCDS56_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS56_0`"]
    #[inline(always)]
    pub fn is_lcds56_0(&self) -> bool {
        *self == LCDS56_A::LCDS56_0
    }
    #[doc = "Checks if the value of the field is `LCDS56_1`"]
    #[inline(always)]
    pub fn is_lcds56_1(&self) -> bool {
        *self == LCDS56_A::LCDS56_1
    }
}
#[doc = "Field `LCDS56` writer - LCD pin 56 enable"]
pub type LCDS56_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS56_A, O>;
impl<'a, const O: u8> LCDS56_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds56_0(self) -> &'a mut W {
        self.variant(LCDS56_A::LCDS56_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds56_1(self) -> &'a mut W {
        self.variant(LCDS56_A::LCDS56_1)
    }
}
#[doc = "Field `LCDS57` reader - LCD pin 57 enable"]
pub type LCDS57_R = crate::BitReader<LCDS57_A>;
#[doc = "LCD pin 57 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS57_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS57_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS57_1 = 1,
}
impl From<LCDS57_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS57_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS57_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS57_A {
        match self.bits {
            false => LCDS57_A::LCDS57_0,
            true => LCDS57_A::LCDS57_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS57_0`"]
    #[inline(always)]
    pub fn is_lcds57_0(&self) -> bool {
        *self == LCDS57_A::LCDS57_0
    }
    #[doc = "Checks if the value of the field is `LCDS57_1`"]
    #[inline(always)]
    pub fn is_lcds57_1(&self) -> bool {
        *self == LCDS57_A::LCDS57_1
    }
}
#[doc = "Field `LCDS57` writer - LCD pin 57 enable"]
pub type LCDS57_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS57_A, O>;
impl<'a, const O: u8> LCDS57_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds57_0(self) -> &'a mut W {
        self.variant(LCDS57_A::LCDS57_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds57_1(self) -> &'a mut W {
        self.variant(LCDS57_A::LCDS57_1)
    }
}
#[doc = "Field `LCDS58` reader - LCD pin 58 enable"]
pub type LCDS58_R = crate::BitReader<LCDS58_A>;
#[doc = "LCD pin 58 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS58_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS58_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS58_1 = 1,
}
impl From<LCDS58_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS58_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS58_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS58_A {
        match self.bits {
            false => LCDS58_A::LCDS58_0,
            true => LCDS58_A::LCDS58_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS58_0`"]
    #[inline(always)]
    pub fn is_lcds58_0(&self) -> bool {
        *self == LCDS58_A::LCDS58_0
    }
    #[doc = "Checks if the value of the field is `LCDS58_1`"]
    #[inline(always)]
    pub fn is_lcds58_1(&self) -> bool {
        *self == LCDS58_A::LCDS58_1
    }
}
#[doc = "Field `LCDS58` writer - LCD pin 58 enable"]
pub type LCDS58_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS58_A, O>;
impl<'a, const O: u8> LCDS58_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds58_0(self) -> &'a mut W {
        self.variant(LCDS58_A::LCDS58_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds58_1(self) -> &'a mut W {
        self.variant(LCDS58_A::LCDS58_1)
    }
}
#[doc = "Field `LCDS59` reader - LCD pin 59 enable"]
pub type LCDS59_R = crate::BitReader<LCDS59_A>;
#[doc = "LCD pin 59 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS59_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS59_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS59_1 = 1,
}
impl From<LCDS59_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS59_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS59_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS59_A {
        match self.bits {
            false => LCDS59_A::LCDS59_0,
            true => LCDS59_A::LCDS59_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS59_0`"]
    #[inline(always)]
    pub fn is_lcds59_0(&self) -> bool {
        *self == LCDS59_A::LCDS59_0
    }
    #[doc = "Checks if the value of the field is `LCDS59_1`"]
    #[inline(always)]
    pub fn is_lcds59_1(&self) -> bool {
        *self == LCDS59_A::LCDS59_1
    }
}
#[doc = "Field `LCDS59` writer - LCD pin 59 enable"]
pub type LCDS59_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS59_A, O>;
impl<'a, const O: u8> LCDS59_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds59_0(self) -> &'a mut W {
        self.variant(LCDS59_A::LCDS59_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds59_1(self) -> &'a mut W {
        self.variant(LCDS59_A::LCDS59_1)
    }
}
#[doc = "Field `LCDS60` reader - LCD pin 60 enable"]
pub type LCDS60_R = crate::BitReader<LCDS60_A>;
#[doc = "LCD pin 60 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS60_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS60_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS60_1 = 1,
}
impl From<LCDS60_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS60_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS60_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS60_A {
        match self.bits {
            false => LCDS60_A::LCDS60_0,
            true => LCDS60_A::LCDS60_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS60_0`"]
    #[inline(always)]
    pub fn is_lcds60_0(&self) -> bool {
        *self == LCDS60_A::LCDS60_0
    }
    #[doc = "Checks if the value of the field is `LCDS60_1`"]
    #[inline(always)]
    pub fn is_lcds60_1(&self) -> bool {
        *self == LCDS60_A::LCDS60_1
    }
}
#[doc = "Field `LCDS60` writer - LCD pin 60 enable"]
pub type LCDS60_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS60_A, O>;
impl<'a, const O: u8> LCDS60_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds60_0(self) -> &'a mut W {
        self.variant(LCDS60_A::LCDS60_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds60_1(self) -> &'a mut W {
        self.variant(LCDS60_A::LCDS60_1)
    }
}
#[doc = "Field `LCDS61` reader - LCD pin 61 enable"]
pub type LCDS61_R = crate::BitReader<LCDS61_A>;
#[doc = "LCD pin 61 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS61_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS61_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS61_1 = 1,
}
impl From<LCDS61_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS61_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS61_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS61_A {
        match self.bits {
            false => LCDS61_A::LCDS61_0,
            true => LCDS61_A::LCDS61_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS61_0`"]
    #[inline(always)]
    pub fn is_lcds61_0(&self) -> bool {
        *self == LCDS61_A::LCDS61_0
    }
    #[doc = "Checks if the value of the field is `LCDS61_1`"]
    #[inline(always)]
    pub fn is_lcds61_1(&self) -> bool {
        *self == LCDS61_A::LCDS61_1
    }
}
#[doc = "Field `LCDS61` writer - LCD pin 61 enable"]
pub type LCDS61_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS61_A, O>;
impl<'a, const O: u8> LCDS61_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds61_0(self) -> &'a mut W {
        self.variant(LCDS61_A::LCDS61_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds61_1(self) -> &'a mut W {
        self.variant(LCDS61_A::LCDS61_1)
    }
}
#[doc = "Field `LCDS62` reader - LCD pin 62 enable"]
pub type LCDS62_R = crate::BitReader<LCDS62_A>;
#[doc = "LCD pin 62 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS62_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS62_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS62_1 = 1,
}
impl From<LCDS62_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS62_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS62_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS62_A {
        match self.bits {
            false => LCDS62_A::LCDS62_0,
            true => LCDS62_A::LCDS62_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS62_0`"]
    #[inline(always)]
    pub fn is_lcds62_0(&self) -> bool {
        *self == LCDS62_A::LCDS62_0
    }
    #[doc = "Checks if the value of the field is `LCDS62_1`"]
    #[inline(always)]
    pub fn is_lcds62_1(&self) -> bool {
        *self == LCDS62_A::LCDS62_1
    }
}
#[doc = "Field `LCDS62` writer - LCD pin 62 enable"]
pub type LCDS62_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS62_A, O>;
impl<'a, const O: u8> LCDS62_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds62_0(self) -> &'a mut W {
        self.variant(LCDS62_A::LCDS62_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds62_1(self) -> &'a mut W {
        self.variant(LCDS62_A::LCDS62_1)
    }
}
#[doc = "Field `LCDS63` reader - LCD pin 63 enable"]
pub type LCDS63_R = crate::BitReader<LCDS63_A>;
#[doc = "LCD pin 63 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDS63_A {
    #[doc = "0: Multiplexed pins are port functions"]
    LCDS63_0 = 0,
    #[doc = "1: Pins are LCD functions"]
    LCDS63_1 = 1,
}
impl From<LCDS63_A> for bool {
    #[inline(always)]
    fn from(variant: LCDS63_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDS63_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDS63_A {
        match self.bits {
            false => LCDS63_A::LCDS63_0,
            true => LCDS63_A::LCDS63_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDS63_0`"]
    #[inline(always)]
    pub fn is_lcds63_0(&self) -> bool {
        *self == LCDS63_A::LCDS63_0
    }
    #[doc = "Checks if the value of the field is `LCDS63_1`"]
    #[inline(always)]
    pub fn is_lcds63_1(&self) -> bool {
        *self == LCDS63_A::LCDS63_1
    }
}
#[doc = "Field `LCDS63` writer - LCD pin 63 enable"]
pub type LCDS63_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDPCTL1_SPEC, LCDS63_A, O>;
impl<'a, const O: u8> LCDS63_W<'a, O> {
    #[doc = "Multiplexed pins are port functions"]
    #[inline(always)]
    pub fn lcds63_0(self) -> &'a mut W {
        self.variant(LCDS63_A::LCDS63_0)
    }
    #[doc = "Pins are LCD functions"]
    #[inline(always)]
    pub fn lcds63_1(self) -> &'a mut W {
        self.variant(LCDS63_A::LCDS63_1)
    }
}
impl R {
    #[doc = "Bit 0 - LCD pin 32 enable"]
    #[inline(always)]
    pub fn lcds32(&self) -> LCDS32_R {
        LCDS32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD pin 33 enable"]
    #[inline(always)]
    pub fn lcds33(&self) -> LCDS33_R {
        LCDS33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD pin 34 enable"]
    #[inline(always)]
    pub fn lcds34(&self) -> LCDS34_R {
        LCDS34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LCD pin 35 enable"]
    #[inline(always)]
    pub fn lcds35(&self) -> LCDS35_R {
        LCDS35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LCD pin 36 enable"]
    #[inline(always)]
    pub fn lcds36(&self) -> LCDS36_R {
        LCDS36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LCD pin 37 enable"]
    #[inline(always)]
    pub fn lcds37(&self) -> LCDS37_R {
        LCDS37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LCD pin 38 enable"]
    #[inline(always)]
    pub fn lcds38(&self) -> LCDS38_R {
        LCDS38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LCD pin 39 enable"]
    #[inline(always)]
    pub fn lcds39(&self) -> LCDS39_R {
        LCDS39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LCD pin 40 enable"]
    #[inline(always)]
    pub fn lcds40(&self) -> LCDS40_R {
        LCDS40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD pin 41 enable"]
    #[inline(always)]
    pub fn lcds41(&self) -> LCDS41_R {
        LCDS41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LCD pin 42 enable"]
    #[inline(always)]
    pub fn lcds42(&self) -> LCDS42_R {
        LCDS42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LCD pin 43 enable"]
    #[inline(always)]
    pub fn lcds43(&self) -> LCDS43_R {
        LCDS43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LCD pin 44 enable"]
    #[inline(always)]
    pub fn lcds44(&self) -> LCDS44_R {
        LCDS44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LCD pin 45 enable"]
    #[inline(always)]
    pub fn lcds45(&self) -> LCDS45_R {
        LCDS45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LCD pin 46 enable"]
    #[inline(always)]
    pub fn lcds46(&self) -> LCDS46_R {
        LCDS46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LCD pin 47 enable"]
    #[inline(always)]
    pub fn lcds47(&self) -> LCDS47_R {
        LCDS47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - LCD pin 48 enable"]
    #[inline(always)]
    pub fn lcds48(&self) -> LCDS48_R {
        LCDS48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - LCD pin 49 enable"]
    #[inline(always)]
    pub fn lcds49(&self) -> LCDS49_R {
        LCDS49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - LCD pin 50 enable"]
    #[inline(always)]
    pub fn lcds50(&self) -> LCDS50_R {
        LCDS50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - LCD pin 51 enable"]
    #[inline(always)]
    pub fn lcds51(&self) -> LCDS51_R {
        LCDS51_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LCD pin 52 enable"]
    #[inline(always)]
    pub fn lcds52(&self) -> LCDS52_R {
        LCDS52_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - LCD pin 53 enable"]
    #[inline(always)]
    pub fn lcds53(&self) -> LCDS53_R {
        LCDS53_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - LCD pin 54 enable"]
    #[inline(always)]
    pub fn lcds54(&self) -> LCDS54_R {
        LCDS54_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - LCD pin 55 enable"]
    #[inline(always)]
    pub fn lcds55(&self) -> LCDS55_R {
        LCDS55_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - LCD pin 56 enable"]
    #[inline(always)]
    pub fn lcds56(&self) -> LCDS56_R {
        LCDS56_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - LCD pin 57 enable"]
    #[inline(always)]
    pub fn lcds57(&self) -> LCDS57_R {
        LCDS57_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LCD pin 58 enable"]
    #[inline(always)]
    pub fn lcds58(&self) -> LCDS58_R {
        LCDS58_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LCD pin 59 enable"]
    #[inline(always)]
    pub fn lcds59(&self) -> LCDS59_R {
        LCDS59_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - LCD pin 60 enable"]
    #[inline(always)]
    pub fn lcds60(&self) -> LCDS60_R {
        LCDS60_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - LCD pin 61 enable"]
    #[inline(always)]
    pub fn lcds61(&self) -> LCDS61_R {
        LCDS61_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - LCD pin 62 enable"]
    #[inline(always)]
    pub fn lcds62(&self) -> LCDS62_R {
        LCDS62_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - LCD pin 63 enable"]
    #[inline(always)]
    pub fn lcds63(&self) -> LCDS63_R {
        LCDS63_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD pin 32 enable"]
    #[inline(always)]
    pub fn lcds32(&mut self) -> LCDS32_W<0> {
        LCDS32_W::new(self)
    }
    #[doc = "Bit 1 - LCD pin 33 enable"]
    #[inline(always)]
    pub fn lcds33(&mut self) -> LCDS33_W<1> {
        LCDS33_W::new(self)
    }
    #[doc = "Bit 2 - LCD pin 34 enable"]
    #[inline(always)]
    pub fn lcds34(&mut self) -> LCDS34_W<2> {
        LCDS34_W::new(self)
    }
    #[doc = "Bit 3 - LCD pin 35 enable"]
    #[inline(always)]
    pub fn lcds35(&mut self) -> LCDS35_W<3> {
        LCDS35_W::new(self)
    }
    #[doc = "Bit 4 - LCD pin 36 enable"]
    #[inline(always)]
    pub fn lcds36(&mut self) -> LCDS36_W<4> {
        LCDS36_W::new(self)
    }
    #[doc = "Bit 5 - LCD pin 37 enable"]
    #[inline(always)]
    pub fn lcds37(&mut self) -> LCDS37_W<5> {
        LCDS37_W::new(self)
    }
    #[doc = "Bit 6 - LCD pin 38 enable"]
    #[inline(always)]
    pub fn lcds38(&mut self) -> LCDS38_W<6> {
        LCDS38_W::new(self)
    }
    #[doc = "Bit 7 - LCD pin 39 enable"]
    #[inline(always)]
    pub fn lcds39(&mut self) -> LCDS39_W<7> {
        LCDS39_W::new(self)
    }
    #[doc = "Bit 8 - LCD pin 40 enable"]
    #[inline(always)]
    pub fn lcds40(&mut self) -> LCDS40_W<8> {
        LCDS40_W::new(self)
    }
    #[doc = "Bit 9 - LCD pin 41 enable"]
    #[inline(always)]
    pub fn lcds41(&mut self) -> LCDS41_W<9> {
        LCDS41_W::new(self)
    }
    #[doc = "Bit 10 - LCD pin 42 enable"]
    #[inline(always)]
    pub fn lcds42(&mut self) -> LCDS42_W<10> {
        LCDS42_W::new(self)
    }
    #[doc = "Bit 11 - LCD pin 43 enable"]
    #[inline(always)]
    pub fn lcds43(&mut self) -> LCDS43_W<11> {
        LCDS43_W::new(self)
    }
    #[doc = "Bit 12 - LCD pin 44 enable"]
    #[inline(always)]
    pub fn lcds44(&mut self) -> LCDS44_W<12> {
        LCDS44_W::new(self)
    }
    #[doc = "Bit 13 - LCD pin 45 enable"]
    #[inline(always)]
    pub fn lcds45(&mut self) -> LCDS45_W<13> {
        LCDS45_W::new(self)
    }
    #[doc = "Bit 14 - LCD pin 46 enable"]
    #[inline(always)]
    pub fn lcds46(&mut self) -> LCDS46_W<14> {
        LCDS46_W::new(self)
    }
    #[doc = "Bit 15 - LCD pin 47 enable"]
    #[inline(always)]
    pub fn lcds47(&mut self) -> LCDS47_W<15> {
        LCDS47_W::new(self)
    }
    #[doc = "Bit 16 - LCD pin 48 enable"]
    #[inline(always)]
    pub fn lcds48(&mut self) -> LCDS48_W<16> {
        LCDS48_W::new(self)
    }
    #[doc = "Bit 17 - LCD pin 49 enable"]
    #[inline(always)]
    pub fn lcds49(&mut self) -> LCDS49_W<17> {
        LCDS49_W::new(self)
    }
    #[doc = "Bit 18 - LCD pin 50 enable"]
    #[inline(always)]
    pub fn lcds50(&mut self) -> LCDS50_W<18> {
        LCDS50_W::new(self)
    }
    #[doc = "Bit 19 - LCD pin 51 enable"]
    #[inline(always)]
    pub fn lcds51(&mut self) -> LCDS51_W<19> {
        LCDS51_W::new(self)
    }
    #[doc = "Bit 20 - LCD pin 52 enable"]
    #[inline(always)]
    pub fn lcds52(&mut self) -> LCDS52_W<20> {
        LCDS52_W::new(self)
    }
    #[doc = "Bit 21 - LCD pin 53 enable"]
    #[inline(always)]
    pub fn lcds53(&mut self) -> LCDS53_W<21> {
        LCDS53_W::new(self)
    }
    #[doc = "Bit 22 - LCD pin 54 enable"]
    #[inline(always)]
    pub fn lcds54(&mut self) -> LCDS54_W<22> {
        LCDS54_W::new(self)
    }
    #[doc = "Bit 23 - LCD pin 55 enable"]
    #[inline(always)]
    pub fn lcds55(&mut self) -> LCDS55_W<23> {
        LCDS55_W::new(self)
    }
    #[doc = "Bit 24 - LCD pin 56 enable"]
    #[inline(always)]
    pub fn lcds56(&mut self) -> LCDS56_W<24> {
        LCDS56_W::new(self)
    }
    #[doc = "Bit 25 - LCD pin 57 enable"]
    #[inline(always)]
    pub fn lcds57(&mut self) -> LCDS57_W<25> {
        LCDS57_W::new(self)
    }
    #[doc = "Bit 26 - LCD pin 58 enable"]
    #[inline(always)]
    pub fn lcds58(&mut self) -> LCDS58_W<26> {
        LCDS58_W::new(self)
    }
    #[doc = "Bit 27 - LCD pin 59 enable"]
    #[inline(always)]
    pub fn lcds59(&mut self) -> LCDS59_W<27> {
        LCDS59_W::new(self)
    }
    #[doc = "Bit 28 - LCD pin 60 enable"]
    #[inline(always)]
    pub fn lcds60(&mut self) -> LCDS60_W<28> {
        LCDS60_W::new(self)
    }
    #[doc = "Bit 29 - LCD pin 61 enable"]
    #[inline(always)]
    pub fn lcds61(&mut self) -> LCDS61_W<29> {
        LCDS61_W::new(self)
    }
    #[doc = "Bit 30 - LCD pin 62 enable"]
    #[inline(always)]
    pub fn lcds62(&mut self) -> LCDS62_W<30> {
        LCDS62_W::new(self)
    }
    #[doc = "Bit 31 - LCD pin 63 enable"]
    #[inline(always)]
    pub fn lcds63(&mut self) -> LCDS63_W<31> {
        LCDS63_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_F port control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdpctl1](index.html) module"]
pub struct LCDPCTL1_SPEC;
impl crate::RegisterSpec for LCDPCTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcdpctl1::R](R) reader structure"]
impl crate::Readable for LCDPCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdpctl1::W](W) writer structure"]
impl crate::Writable for LCDPCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDPCTL1 to value 0"]
impl crate::Resettable for LCDPCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
