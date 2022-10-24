#[doc = "Register `LCDCSSEL1` reader"]
pub struct R(crate::R<LCDCSSEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDCSSEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDCSSEL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDCSSEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDCSSEL1` writer"]
pub struct W(crate::W<LCDCSSEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDCSSEL1_SPEC>;
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
impl From<crate::W<LCDCSSEL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCDCSSEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDCSS32` reader - L32 Com Seg select"]
pub type LCDCSS32_R = crate::BitReader<LCDCSS32_A>;
#[doc = "L32 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS32_A {
    #[doc = "0: Segment line"]
    LCDCSS32_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS32_1 = 1,
}
impl From<LCDCSS32_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS32_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS32_A {
        match self.bits {
            false => LCDCSS32_A::LCDCSS32_0,
            true => LCDCSS32_A::LCDCSS32_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS32_0`"]
    #[inline(always)]
    pub fn is_lcdcss32_0(&self) -> bool {
        *self == LCDCSS32_A::LCDCSS32_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS32_1`"]
    #[inline(always)]
    pub fn is_lcdcss32_1(&self) -> bool {
        *self == LCDCSS32_A::LCDCSS32_1
    }
}
#[doc = "Field `LCDCSS32` writer - L32 Com Seg select"]
pub type LCDCSS32_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS32_A, O>;
impl<'a, const O: u8> LCDCSS32_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss32_0(self) -> &'a mut W {
        self.variant(LCDCSS32_A::LCDCSS32_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss32_1(self) -> &'a mut W {
        self.variant(LCDCSS32_A::LCDCSS32_1)
    }
}
#[doc = "Field `LCDCSS33` reader - L33 Com Seg select"]
pub type LCDCSS33_R = crate::BitReader<LCDCSS33_A>;
#[doc = "L33 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS33_A {
    #[doc = "0: Segment line"]
    LCDCSS33_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS33_1 = 1,
}
impl From<LCDCSS33_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS33_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS33_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS33_A {
        match self.bits {
            false => LCDCSS33_A::LCDCSS33_0,
            true => LCDCSS33_A::LCDCSS33_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS33_0`"]
    #[inline(always)]
    pub fn is_lcdcss33_0(&self) -> bool {
        *self == LCDCSS33_A::LCDCSS33_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS33_1`"]
    #[inline(always)]
    pub fn is_lcdcss33_1(&self) -> bool {
        *self == LCDCSS33_A::LCDCSS33_1
    }
}
#[doc = "Field `LCDCSS33` writer - L33 Com Seg select"]
pub type LCDCSS33_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS33_A, O>;
impl<'a, const O: u8> LCDCSS33_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss33_0(self) -> &'a mut W {
        self.variant(LCDCSS33_A::LCDCSS33_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss33_1(self) -> &'a mut W {
        self.variant(LCDCSS33_A::LCDCSS33_1)
    }
}
#[doc = "Field `LCDCSS34` reader - L34 Com Seg select"]
pub type LCDCSS34_R = crate::BitReader<LCDCSS34_A>;
#[doc = "L34 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS34_A {
    #[doc = "0: Segment line"]
    LCDCSS34_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS34_1 = 1,
}
impl From<LCDCSS34_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS34_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS34_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS34_A {
        match self.bits {
            false => LCDCSS34_A::LCDCSS34_0,
            true => LCDCSS34_A::LCDCSS34_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS34_0`"]
    #[inline(always)]
    pub fn is_lcdcss34_0(&self) -> bool {
        *self == LCDCSS34_A::LCDCSS34_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS34_1`"]
    #[inline(always)]
    pub fn is_lcdcss34_1(&self) -> bool {
        *self == LCDCSS34_A::LCDCSS34_1
    }
}
#[doc = "Field `LCDCSS34` writer - L34 Com Seg select"]
pub type LCDCSS34_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS34_A, O>;
impl<'a, const O: u8> LCDCSS34_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss34_0(self) -> &'a mut W {
        self.variant(LCDCSS34_A::LCDCSS34_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss34_1(self) -> &'a mut W {
        self.variant(LCDCSS34_A::LCDCSS34_1)
    }
}
#[doc = "Field `LCDCSS35` reader - L35 Com Seg select"]
pub type LCDCSS35_R = crate::BitReader<LCDCSS35_A>;
#[doc = "L35 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS35_A {
    #[doc = "0: Segment line"]
    LCDCSS35_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS35_1 = 1,
}
impl From<LCDCSS35_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS35_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS35_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS35_A {
        match self.bits {
            false => LCDCSS35_A::LCDCSS35_0,
            true => LCDCSS35_A::LCDCSS35_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS35_0`"]
    #[inline(always)]
    pub fn is_lcdcss35_0(&self) -> bool {
        *self == LCDCSS35_A::LCDCSS35_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS35_1`"]
    #[inline(always)]
    pub fn is_lcdcss35_1(&self) -> bool {
        *self == LCDCSS35_A::LCDCSS35_1
    }
}
#[doc = "Field `LCDCSS35` writer - L35 Com Seg select"]
pub type LCDCSS35_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS35_A, O>;
impl<'a, const O: u8> LCDCSS35_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss35_0(self) -> &'a mut W {
        self.variant(LCDCSS35_A::LCDCSS35_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss35_1(self) -> &'a mut W {
        self.variant(LCDCSS35_A::LCDCSS35_1)
    }
}
#[doc = "Field `LCDCSS36` reader - L36 Com Seg select"]
pub type LCDCSS36_R = crate::BitReader<LCDCSS36_A>;
#[doc = "L36 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS36_A {
    #[doc = "0: Segment line"]
    LCDCSS36_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS36_1 = 1,
}
impl From<LCDCSS36_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS36_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS36_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS36_A {
        match self.bits {
            false => LCDCSS36_A::LCDCSS36_0,
            true => LCDCSS36_A::LCDCSS36_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS36_0`"]
    #[inline(always)]
    pub fn is_lcdcss36_0(&self) -> bool {
        *self == LCDCSS36_A::LCDCSS36_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS36_1`"]
    #[inline(always)]
    pub fn is_lcdcss36_1(&self) -> bool {
        *self == LCDCSS36_A::LCDCSS36_1
    }
}
#[doc = "Field `LCDCSS36` writer - L36 Com Seg select"]
pub type LCDCSS36_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS36_A, O>;
impl<'a, const O: u8> LCDCSS36_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss36_0(self) -> &'a mut W {
        self.variant(LCDCSS36_A::LCDCSS36_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss36_1(self) -> &'a mut W {
        self.variant(LCDCSS36_A::LCDCSS36_1)
    }
}
#[doc = "Field `LCDCSS37` reader - L37 Com Seg select"]
pub type LCDCSS37_R = crate::BitReader<LCDCSS37_A>;
#[doc = "L37 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS37_A {
    #[doc = "0: Segment line"]
    LCDCSS37_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS37_1 = 1,
}
impl From<LCDCSS37_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS37_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS37_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS37_A {
        match self.bits {
            false => LCDCSS37_A::LCDCSS37_0,
            true => LCDCSS37_A::LCDCSS37_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS37_0`"]
    #[inline(always)]
    pub fn is_lcdcss37_0(&self) -> bool {
        *self == LCDCSS37_A::LCDCSS37_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS37_1`"]
    #[inline(always)]
    pub fn is_lcdcss37_1(&self) -> bool {
        *self == LCDCSS37_A::LCDCSS37_1
    }
}
#[doc = "Field `LCDCSS37` writer - L37 Com Seg select"]
pub type LCDCSS37_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS37_A, O>;
impl<'a, const O: u8> LCDCSS37_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss37_0(self) -> &'a mut W {
        self.variant(LCDCSS37_A::LCDCSS37_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss37_1(self) -> &'a mut W {
        self.variant(LCDCSS37_A::LCDCSS37_1)
    }
}
#[doc = "Field `LCDCSS38` reader - L38 Com Seg select"]
pub type LCDCSS38_R = crate::BitReader<LCDCSS38_A>;
#[doc = "L38 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS38_A {
    #[doc = "0: Segment line"]
    LCDCSS38_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS38_1 = 1,
}
impl From<LCDCSS38_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS38_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS38_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS38_A {
        match self.bits {
            false => LCDCSS38_A::LCDCSS38_0,
            true => LCDCSS38_A::LCDCSS38_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS38_0`"]
    #[inline(always)]
    pub fn is_lcdcss38_0(&self) -> bool {
        *self == LCDCSS38_A::LCDCSS38_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS38_1`"]
    #[inline(always)]
    pub fn is_lcdcss38_1(&self) -> bool {
        *self == LCDCSS38_A::LCDCSS38_1
    }
}
#[doc = "Field `LCDCSS38` writer - L38 Com Seg select"]
pub type LCDCSS38_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS38_A, O>;
impl<'a, const O: u8> LCDCSS38_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss38_0(self) -> &'a mut W {
        self.variant(LCDCSS38_A::LCDCSS38_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss38_1(self) -> &'a mut W {
        self.variant(LCDCSS38_A::LCDCSS38_1)
    }
}
#[doc = "Field `LCDCSS39` reader - L39 Com Seg select"]
pub type LCDCSS39_R = crate::BitReader<LCDCSS39_A>;
#[doc = "L39 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS39_A {
    #[doc = "0: Segment line"]
    LCDCSS39_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS39_1 = 1,
}
impl From<LCDCSS39_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS39_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS39_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS39_A {
        match self.bits {
            false => LCDCSS39_A::LCDCSS39_0,
            true => LCDCSS39_A::LCDCSS39_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS39_0`"]
    #[inline(always)]
    pub fn is_lcdcss39_0(&self) -> bool {
        *self == LCDCSS39_A::LCDCSS39_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS39_1`"]
    #[inline(always)]
    pub fn is_lcdcss39_1(&self) -> bool {
        *self == LCDCSS39_A::LCDCSS39_1
    }
}
#[doc = "Field `LCDCSS39` writer - L39 Com Seg select"]
pub type LCDCSS39_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS39_A, O>;
impl<'a, const O: u8> LCDCSS39_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss39_0(self) -> &'a mut W {
        self.variant(LCDCSS39_A::LCDCSS39_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss39_1(self) -> &'a mut W {
        self.variant(LCDCSS39_A::LCDCSS39_1)
    }
}
#[doc = "Field `LCDCSS40` reader - L40 Com Seg select"]
pub type LCDCSS40_R = crate::BitReader<LCDCSS40_A>;
#[doc = "L40 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS40_A {
    #[doc = "0: Segment line"]
    LCDCSS40_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS40_1 = 1,
}
impl From<LCDCSS40_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS40_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS40_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS40_A {
        match self.bits {
            false => LCDCSS40_A::LCDCSS40_0,
            true => LCDCSS40_A::LCDCSS40_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS40_0`"]
    #[inline(always)]
    pub fn is_lcdcss40_0(&self) -> bool {
        *self == LCDCSS40_A::LCDCSS40_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS40_1`"]
    #[inline(always)]
    pub fn is_lcdcss40_1(&self) -> bool {
        *self == LCDCSS40_A::LCDCSS40_1
    }
}
#[doc = "Field `LCDCSS40` writer - L40 Com Seg select"]
pub type LCDCSS40_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS40_A, O>;
impl<'a, const O: u8> LCDCSS40_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss40_0(self) -> &'a mut W {
        self.variant(LCDCSS40_A::LCDCSS40_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss40_1(self) -> &'a mut W {
        self.variant(LCDCSS40_A::LCDCSS40_1)
    }
}
#[doc = "Field `LCDCSS41` reader - L41 Com Seg select"]
pub type LCDCSS41_R = crate::BitReader<LCDCSS41_A>;
#[doc = "L41 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS41_A {
    #[doc = "0: Segment line"]
    LCDCSS41_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS41_1 = 1,
}
impl From<LCDCSS41_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS41_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS41_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS41_A {
        match self.bits {
            false => LCDCSS41_A::LCDCSS41_0,
            true => LCDCSS41_A::LCDCSS41_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS41_0`"]
    #[inline(always)]
    pub fn is_lcdcss41_0(&self) -> bool {
        *self == LCDCSS41_A::LCDCSS41_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS41_1`"]
    #[inline(always)]
    pub fn is_lcdcss41_1(&self) -> bool {
        *self == LCDCSS41_A::LCDCSS41_1
    }
}
#[doc = "Field `LCDCSS41` writer - L41 Com Seg select"]
pub type LCDCSS41_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS41_A, O>;
impl<'a, const O: u8> LCDCSS41_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss41_0(self) -> &'a mut W {
        self.variant(LCDCSS41_A::LCDCSS41_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss41_1(self) -> &'a mut W {
        self.variant(LCDCSS41_A::LCDCSS41_1)
    }
}
#[doc = "Field `LCDCSS42` reader - L42 Com Seg select"]
pub type LCDCSS42_R = crate::BitReader<LCDCSS42_A>;
#[doc = "L42 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS42_A {
    #[doc = "0: Segment line"]
    LCDCSS42_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS42_1 = 1,
}
impl From<LCDCSS42_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS42_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS42_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS42_A {
        match self.bits {
            false => LCDCSS42_A::LCDCSS42_0,
            true => LCDCSS42_A::LCDCSS42_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS42_0`"]
    #[inline(always)]
    pub fn is_lcdcss42_0(&self) -> bool {
        *self == LCDCSS42_A::LCDCSS42_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS42_1`"]
    #[inline(always)]
    pub fn is_lcdcss42_1(&self) -> bool {
        *self == LCDCSS42_A::LCDCSS42_1
    }
}
#[doc = "Field `LCDCSS42` writer - L42 Com Seg select"]
pub type LCDCSS42_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS42_A, O>;
impl<'a, const O: u8> LCDCSS42_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss42_0(self) -> &'a mut W {
        self.variant(LCDCSS42_A::LCDCSS42_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss42_1(self) -> &'a mut W {
        self.variant(LCDCSS42_A::LCDCSS42_1)
    }
}
#[doc = "Field `LCDCSS43` reader - L43 Com Seg select"]
pub type LCDCSS43_R = crate::BitReader<LCDCSS43_A>;
#[doc = "L43 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS43_A {
    #[doc = "0: Segment line"]
    LCDCSS43_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS43_1 = 1,
}
impl From<LCDCSS43_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS43_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS43_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS43_A {
        match self.bits {
            false => LCDCSS43_A::LCDCSS43_0,
            true => LCDCSS43_A::LCDCSS43_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS43_0`"]
    #[inline(always)]
    pub fn is_lcdcss43_0(&self) -> bool {
        *self == LCDCSS43_A::LCDCSS43_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS43_1`"]
    #[inline(always)]
    pub fn is_lcdcss43_1(&self) -> bool {
        *self == LCDCSS43_A::LCDCSS43_1
    }
}
#[doc = "Field `LCDCSS43` writer - L43 Com Seg select"]
pub type LCDCSS43_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS43_A, O>;
impl<'a, const O: u8> LCDCSS43_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss43_0(self) -> &'a mut W {
        self.variant(LCDCSS43_A::LCDCSS43_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss43_1(self) -> &'a mut W {
        self.variant(LCDCSS43_A::LCDCSS43_1)
    }
}
#[doc = "Field `LCDCSS44` reader - L44 Com Seg select"]
pub type LCDCSS44_R = crate::BitReader<LCDCSS44_A>;
#[doc = "L44 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS44_A {
    #[doc = "0: Segment line"]
    LCDCSS44_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS44_1 = 1,
}
impl From<LCDCSS44_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS44_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS44_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS44_A {
        match self.bits {
            false => LCDCSS44_A::LCDCSS44_0,
            true => LCDCSS44_A::LCDCSS44_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS44_0`"]
    #[inline(always)]
    pub fn is_lcdcss44_0(&self) -> bool {
        *self == LCDCSS44_A::LCDCSS44_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS44_1`"]
    #[inline(always)]
    pub fn is_lcdcss44_1(&self) -> bool {
        *self == LCDCSS44_A::LCDCSS44_1
    }
}
#[doc = "Field `LCDCSS44` writer - L44 Com Seg select"]
pub type LCDCSS44_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS44_A, O>;
impl<'a, const O: u8> LCDCSS44_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss44_0(self) -> &'a mut W {
        self.variant(LCDCSS44_A::LCDCSS44_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss44_1(self) -> &'a mut W {
        self.variant(LCDCSS44_A::LCDCSS44_1)
    }
}
#[doc = "Field `LCDCSS45` reader - L45 Com Seg select"]
pub type LCDCSS45_R = crate::BitReader<LCDCSS45_A>;
#[doc = "L45 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS45_A {
    #[doc = "0: Segment line"]
    LCDCSS45_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS45_1 = 1,
}
impl From<LCDCSS45_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS45_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS45_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS45_A {
        match self.bits {
            false => LCDCSS45_A::LCDCSS45_0,
            true => LCDCSS45_A::LCDCSS45_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS45_0`"]
    #[inline(always)]
    pub fn is_lcdcss45_0(&self) -> bool {
        *self == LCDCSS45_A::LCDCSS45_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS45_1`"]
    #[inline(always)]
    pub fn is_lcdcss45_1(&self) -> bool {
        *self == LCDCSS45_A::LCDCSS45_1
    }
}
#[doc = "Field `LCDCSS45` writer - L45 Com Seg select"]
pub type LCDCSS45_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS45_A, O>;
impl<'a, const O: u8> LCDCSS45_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss45_0(self) -> &'a mut W {
        self.variant(LCDCSS45_A::LCDCSS45_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss45_1(self) -> &'a mut W {
        self.variant(LCDCSS45_A::LCDCSS45_1)
    }
}
#[doc = "Field `LCDCSS46` reader - L46 Com Seg select"]
pub type LCDCSS46_R = crate::BitReader<LCDCSS46_A>;
#[doc = "L46 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS46_A {
    #[doc = "0: Segment line"]
    LCDCSS46_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS46_1 = 1,
}
impl From<LCDCSS46_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS46_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS46_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS46_A {
        match self.bits {
            false => LCDCSS46_A::LCDCSS46_0,
            true => LCDCSS46_A::LCDCSS46_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS46_0`"]
    #[inline(always)]
    pub fn is_lcdcss46_0(&self) -> bool {
        *self == LCDCSS46_A::LCDCSS46_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS46_1`"]
    #[inline(always)]
    pub fn is_lcdcss46_1(&self) -> bool {
        *self == LCDCSS46_A::LCDCSS46_1
    }
}
#[doc = "Field `LCDCSS46` writer - L46 Com Seg select"]
pub type LCDCSS46_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS46_A, O>;
impl<'a, const O: u8> LCDCSS46_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss46_0(self) -> &'a mut W {
        self.variant(LCDCSS46_A::LCDCSS46_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss46_1(self) -> &'a mut W {
        self.variant(LCDCSS46_A::LCDCSS46_1)
    }
}
#[doc = "Field `LCDCSS47` reader - L47 Com Seg select"]
pub type LCDCSS47_R = crate::BitReader<LCDCSS47_A>;
#[doc = "L47 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS47_A {
    #[doc = "0: Segment line"]
    LCDCSS47_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS47_1 = 1,
}
impl From<LCDCSS47_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS47_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS47_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS47_A {
        match self.bits {
            false => LCDCSS47_A::LCDCSS47_0,
            true => LCDCSS47_A::LCDCSS47_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS47_0`"]
    #[inline(always)]
    pub fn is_lcdcss47_0(&self) -> bool {
        *self == LCDCSS47_A::LCDCSS47_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS47_1`"]
    #[inline(always)]
    pub fn is_lcdcss47_1(&self) -> bool {
        *self == LCDCSS47_A::LCDCSS47_1
    }
}
#[doc = "Field `LCDCSS47` writer - L47 Com Seg select"]
pub type LCDCSS47_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS47_A, O>;
impl<'a, const O: u8> LCDCSS47_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss47_0(self) -> &'a mut W {
        self.variant(LCDCSS47_A::LCDCSS47_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss47_1(self) -> &'a mut W {
        self.variant(LCDCSS47_A::LCDCSS47_1)
    }
}
#[doc = "Field `LCDCSS48` reader - L48 Com Seg select"]
pub type LCDCSS48_R = crate::BitReader<LCDCSS48_A>;
#[doc = "L48 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS48_A {
    #[doc = "0: Segment line"]
    LCDCSS48_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS48_1 = 1,
}
impl From<LCDCSS48_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS48_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS48_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS48_A {
        match self.bits {
            false => LCDCSS48_A::LCDCSS48_0,
            true => LCDCSS48_A::LCDCSS48_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS48_0`"]
    #[inline(always)]
    pub fn is_lcdcss48_0(&self) -> bool {
        *self == LCDCSS48_A::LCDCSS48_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS48_1`"]
    #[inline(always)]
    pub fn is_lcdcss48_1(&self) -> bool {
        *self == LCDCSS48_A::LCDCSS48_1
    }
}
#[doc = "Field `LCDCSS48` writer - L48 Com Seg select"]
pub type LCDCSS48_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS48_A, O>;
impl<'a, const O: u8> LCDCSS48_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss48_0(self) -> &'a mut W {
        self.variant(LCDCSS48_A::LCDCSS48_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss48_1(self) -> &'a mut W {
        self.variant(LCDCSS48_A::LCDCSS48_1)
    }
}
#[doc = "Field `LCDCSS49` reader - L49 Com Seg select"]
pub type LCDCSS49_R = crate::BitReader<LCDCSS49_A>;
#[doc = "L49 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS49_A {
    #[doc = "0: Segment line"]
    LCDCSS49_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS49_1 = 1,
}
impl From<LCDCSS49_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS49_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS49_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS49_A {
        match self.bits {
            false => LCDCSS49_A::LCDCSS49_0,
            true => LCDCSS49_A::LCDCSS49_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS49_0`"]
    #[inline(always)]
    pub fn is_lcdcss49_0(&self) -> bool {
        *self == LCDCSS49_A::LCDCSS49_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS49_1`"]
    #[inline(always)]
    pub fn is_lcdcss49_1(&self) -> bool {
        *self == LCDCSS49_A::LCDCSS49_1
    }
}
#[doc = "Field `LCDCSS49` writer - L49 Com Seg select"]
pub type LCDCSS49_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS49_A, O>;
impl<'a, const O: u8> LCDCSS49_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss49_0(self) -> &'a mut W {
        self.variant(LCDCSS49_A::LCDCSS49_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss49_1(self) -> &'a mut W {
        self.variant(LCDCSS49_A::LCDCSS49_1)
    }
}
#[doc = "Field `LCDCSS50` reader - L50 Com Seg select"]
pub type LCDCSS50_R = crate::BitReader<LCDCSS50_A>;
#[doc = "L50 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS50_A {
    #[doc = "0: Segment line"]
    LCDCSS50_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS50_1 = 1,
}
impl From<LCDCSS50_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS50_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS50_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS50_A {
        match self.bits {
            false => LCDCSS50_A::LCDCSS50_0,
            true => LCDCSS50_A::LCDCSS50_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS50_0`"]
    #[inline(always)]
    pub fn is_lcdcss50_0(&self) -> bool {
        *self == LCDCSS50_A::LCDCSS50_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS50_1`"]
    #[inline(always)]
    pub fn is_lcdcss50_1(&self) -> bool {
        *self == LCDCSS50_A::LCDCSS50_1
    }
}
#[doc = "Field `LCDCSS50` writer - L50 Com Seg select"]
pub type LCDCSS50_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS50_A, O>;
impl<'a, const O: u8> LCDCSS50_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss50_0(self) -> &'a mut W {
        self.variant(LCDCSS50_A::LCDCSS50_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss50_1(self) -> &'a mut W {
        self.variant(LCDCSS50_A::LCDCSS50_1)
    }
}
#[doc = "Field `LCDCSS51` reader - L51 Com Seg select"]
pub type LCDCSS51_R = crate::BitReader<LCDCSS51_A>;
#[doc = "L51 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS51_A {
    #[doc = "0: Segment line"]
    LCDCSS51_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS51_1 = 1,
}
impl From<LCDCSS51_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS51_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS51_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS51_A {
        match self.bits {
            false => LCDCSS51_A::LCDCSS51_0,
            true => LCDCSS51_A::LCDCSS51_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS51_0`"]
    #[inline(always)]
    pub fn is_lcdcss51_0(&self) -> bool {
        *self == LCDCSS51_A::LCDCSS51_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS51_1`"]
    #[inline(always)]
    pub fn is_lcdcss51_1(&self) -> bool {
        *self == LCDCSS51_A::LCDCSS51_1
    }
}
#[doc = "Field `LCDCSS51` writer - L51 Com Seg select"]
pub type LCDCSS51_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS51_A, O>;
impl<'a, const O: u8> LCDCSS51_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss51_0(self) -> &'a mut W {
        self.variant(LCDCSS51_A::LCDCSS51_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss51_1(self) -> &'a mut W {
        self.variant(LCDCSS51_A::LCDCSS51_1)
    }
}
#[doc = "Field `LCDCSS52` reader - L52 Com Seg select"]
pub type LCDCSS52_R = crate::BitReader<LCDCSS52_A>;
#[doc = "L52 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS52_A {
    #[doc = "0: Segment line"]
    LCDCSS52_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS52_1 = 1,
}
impl From<LCDCSS52_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS52_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS52_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS52_A {
        match self.bits {
            false => LCDCSS52_A::LCDCSS52_0,
            true => LCDCSS52_A::LCDCSS52_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS52_0`"]
    #[inline(always)]
    pub fn is_lcdcss52_0(&self) -> bool {
        *self == LCDCSS52_A::LCDCSS52_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS52_1`"]
    #[inline(always)]
    pub fn is_lcdcss52_1(&self) -> bool {
        *self == LCDCSS52_A::LCDCSS52_1
    }
}
#[doc = "Field `LCDCSS52` writer - L52 Com Seg select"]
pub type LCDCSS52_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS52_A, O>;
impl<'a, const O: u8> LCDCSS52_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss52_0(self) -> &'a mut W {
        self.variant(LCDCSS52_A::LCDCSS52_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss52_1(self) -> &'a mut W {
        self.variant(LCDCSS52_A::LCDCSS52_1)
    }
}
#[doc = "Field `LCDCSS53` reader - L53 Com Seg select"]
pub type LCDCSS53_R = crate::BitReader<LCDCSS53_A>;
#[doc = "L53 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS53_A {
    #[doc = "0: Segment line"]
    LCDCSS53_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS53_1 = 1,
}
impl From<LCDCSS53_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS53_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS53_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS53_A {
        match self.bits {
            false => LCDCSS53_A::LCDCSS53_0,
            true => LCDCSS53_A::LCDCSS53_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS53_0`"]
    #[inline(always)]
    pub fn is_lcdcss53_0(&self) -> bool {
        *self == LCDCSS53_A::LCDCSS53_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS53_1`"]
    #[inline(always)]
    pub fn is_lcdcss53_1(&self) -> bool {
        *self == LCDCSS53_A::LCDCSS53_1
    }
}
#[doc = "Field `LCDCSS53` writer - L53 Com Seg select"]
pub type LCDCSS53_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS53_A, O>;
impl<'a, const O: u8> LCDCSS53_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss53_0(self) -> &'a mut W {
        self.variant(LCDCSS53_A::LCDCSS53_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss53_1(self) -> &'a mut W {
        self.variant(LCDCSS53_A::LCDCSS53_1)
    }
}
#[doc = "Field `LCDCSS54` reader - L54 Com Seg select"]
pub type LCDCSS54_R = crate::BitReader<LCDCSS54_A>;
#[doc = "L54 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS54_A {
    #[doc = "0: Segment line"]
    LCDCSS54_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS54_1 = 1,
}
impl From<LCDCSS54_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS54_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS54_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS54_A {
        match self.bits {
            false => LCDCSS54_A::LCDCSS54_0,
            true => LCDCSS54_A::LCDCSS54_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS54_0`"]
    #[inline(always)]
    pub fn is_lcdcss54_0(&self) -> bool {
        *self == LCDCSS54_A::LCDCSS54_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS54_1`"]
    #[inline(always)]
    pub fn is_lcdcss54_1(&self) -> bool {
        *self == LCDCSS54_A::LCDCSS54_1
    }
}
#[doc = "Field `LCDCSS54` writer - L54 Com Seg select"]
pub type LCDCSS54_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS54_A, O>;
impl<'a, const O: u8> LCDCSS54_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss54_0(self) -> &'a mut W {
        self.variant(LCDCSS54_A::LCDCSS54_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss54_1(self) -> &'a mut W {
        self.variant(LCDCSS54_A::LCDCSS54_1)
    }
}
#[doc = "Field `LCDCSS55` reader - L55 Com Seg select"]
pub type LCDCSS55_R = crate::BitReader<LCDCSS55_A>;
#[doc = "L55 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS55_A {
    #[doc = "0: Segment line"]
    LCDCSS55_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS55_1 = 1,
}
impl From<LCDCSS55_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS55_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS55_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS55_A {
        match self.bits {
            false => LCDCSS55_A::LCDCSS55_0,
            true => LCDCSS55_A::LCDCSS55_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS55_0`"]
    #[inline(always)]
    pub fn is_lcdcss55_0(&self) -> bool {
        *self == LCDCSS55_A::LCDCSS55_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS55_1`"]
    #[inline(always)]
    pub fn is_lcdcss55_1(&self) -> bool {
        *self == LCDCSS55_A::LCDCSS55_1
    }
}
#[doc = "Field `LCDCSS55` writer - L55 Com Seg select"]
pub type LCDCSS55_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS55_A, O>;
impl<'a, const O: u8> LCDCSS55_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss55_0(self) -> &'a mut W {
        self.variant(LCDCSS55_A::LCDCSS55_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss55_1(self) -> &'a mut W {
        self.variant(LCDCSS55_A::LCDCSS55_1)
    }
}
#[doc = "Field `LCDCSS56` reader - L56 Com Seg select"]
pub type LCDCSS56_R = crate::BitReader<LCDCSS56_A>;
#[doc = "L56 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS56_A {
    #[doc = "0: Segment line"]
    LCDCSS56_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS56_1 = 1,
}
impl From<LCDCSS56_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS56_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS56_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS56_A {
        match self.bits {
            false => LCDCSS56_A::LCDCSS56_0,
            true => LCDCSS56_A::LCDCSS56_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS56_0`"]
    #[inline(always)]
    pub fn is_lcdcss56_0(&self) -> bool {
        *self == LCDCSS56_A::LCDCSS56_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS56_1`"]
    #[inline(always)]
    pub fn is_lcdcss56_1(&self) -> bool {
        *self == LCDCSS56_A::LCDCSS56_1
    }
}
#[doc = "Field `LCDCSS56` writer - L56 Com Seg select"]
pub type LCDCSS56_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS56_A, O>;
impl<'a, const O: u8> LCDCSS56_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss56_0(self) -> &'a mut W {
        self.variant(LCDCSS56_A::LCDCSS56_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss56_1(self) -> &'a mut W {
        self.variant(LCDCSS56_A::LCDCSS56_1)
    }
}
#[doc = "Field `LCDCSS57` reader - L57 Com Seg select"]
pub type LCDCSS57_R = crate::BitReader<LCDCSS57_A>;
#[doc = "L57 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS57_A {
    #[doc = "0: Segment line"]
    LCDCSS57_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS57_1 = 1,
}
impl From<LCDCSS57_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS57_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS57_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS57_A {
        match self.bits {
            false => LCDCSS57_A::LCDCSS57_0,
            true => LCDCSS57_A::LCDCSS57_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS57_0`"]
    #[inline(always)]
    pub fn is_lcdcss57_0(&self) -> bool {
        *self == LCDCSS57_A::LCDCSS57_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS57_1`"]
    #[inline(always)]
    pub fn is_lcdcss57_1(&self) -> bool {
        *self == LCDCSS57_A::LCDCSS57_1
    }
}
#[doc = "Field `LCDCSS57` writer - L57 Com Seg select"]
pub type LCDCSS57_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS57_A, O>;
impl<'a, const O: u8> LCDCSS57_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss57_0(self) -> &'a mut W {
        self.variant(LCDCSS57_A::LCDCSS57_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss57_1(self) -> &'a mut W {
        self.variant(LCDCSS57_A::LCDCSS57_1)
    }
}
#[doc = "Field `LCDCSS58` reader - L58 Com Seg select"]
pub type LCDCSS58_R = crate::BitReader<LCDCSS58_A>;
#[doc = "L58 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS58_A {
    #[doc = "0: Segment line"]
    LCDCSS58_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS58_1 = 1,
}
impl From<LCDCSS58_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS58_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS58_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS58_A {
        match self.bits {
            false => LCDCSS58_A::LCDCSS58_0,
            true => LCDCSS58_A::LCDCSS58_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS58_0`"]
    #[inline(always)]
    pub fn is_lcdcss58_0(&self) -> bool {
        *self == LCDCSS58_A::LCDCSS58_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS58_1`"]
    #[inline(always)]
    pub fn is_lcdcss58_1(&self) -> bool {
        *self == LCDCSS58_A::LCDCSS58_1
    }
}
#[doc = "Field `LCDCSS58` writer - L58 Com Seg select"]
pub type LCDCSS58_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS58_A, O>;
impl<'a, const O: u8> LCDCSS58_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss58_0(self) -> &'a mut W {
        self.variant(LCDCSS58_A::LCDCSS58_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss58_1(self) -> &'a mut W {
        self.variant(LCDCSS58_A::LCDCSS58_1)
    }
}
#[doc = "Field `LCDCSS59` reader - L59 Com Seg select"]
pub type LCDCSS59_R = crate::BitReader<LCDCSS59_A>;
#[doc = "L59 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS59_A {
    #[doc = "0: Segment line"]
    LCDCSS59_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS59_1 = 1,
}
impl From<LCDCSS59_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS59_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS59_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS59_A {
        match self.bits {
            false => LCDCSS59_A::LCDCSS59_0,
            true => LCDCSS59_A::LCDCSS59_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS59_0`"]
    #[inline(always)]
    pub fn is_lcdcss59_0(&self) -> bool {
        *self == LCDCSS59_A::LCDCSS59_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS59_1`"]
    #[inline(always)]
    pub fn is_lcdcss59_1(&self) -> bool {
        *self == LCDCSS59_A::LCDCSS59_1
    }
}
#[doc = "Field `LCDCSS59` writer - L59 Com Seg select"]
pub type LCDCSS59_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS59_A, O>;
impl<'a, const O: u8> LCDCSS59_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss59_0(self) -> &'a mut W {
        self.variant(LCDCSS59_A::LCDCSS59_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss59_1(self) -> &'a mut W {
        self.variant(LCDCSS59_A::LCDCSS59_1)
    }
}
#[doc = "Field `LCDCSS60` reader - L60 Com Seg select"]
pub type LCDCSS60_R = crate::BitReader<LCDCSS60_A>;
#[doc = "L60 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS60_A {
    #[doc = "0: Segment line"]
    LCDCSS60_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS60_1 = 1,
}
impl From<LCDCSS60_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS60_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS60_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS60_A {
        match self.bits {
            false => LCDCSS60_A::LCDCSS60_0,
            true => LCDCSS60_A::LCDCSS60_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS60_0`"]
    #[inline(always)]
    pub fn is_lcdcss60_0(&self) -> bool {
        *self == LCDCSS60_A::LCDCSS60_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS60_1`"]
    #[inline(always)]
    pub fn is_lcdcss60_1(&self) -> bool {
        *self == LCDCSS60_A::LCDCSS60_1
    }
}
#[doc = "Field `LCDCSS60` writer - L60 Com Seg select"]
pub type LCDCSS60_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS60_A, O>;
impl<'a, const O: u8> LCDCSS60_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss60_0(self) -> &'a mut W {
        self.variant(LCDCSS60_A::LCDCSS60_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss60_1(self) -> &'a mut W {
        self.variant(LCDCSS60_A::LCDCSS60_1)
    }
}
#[doc = "Field `LCDCSS61` reader - L61 Com Seg select"]
pub type LCDCSS61_R = crate::BitReader<LCDCSS61_A>;
#[doc = "L61 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS61_A {
    #[doc = "0: Segment line"]
    LCDCSS61_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS61_1 = 1,
}
impl From<LCDCSS61_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS61_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS61_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS61_A {
        match self.bits {
            false => LCDCSS61_A::LCDCSS61_0,
            true => LCDCSS61_A::LCDCSS61_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS61_0`"]
    #[inline(always)]
    pub fn is_lcdcss61_0(&self) -> bool {
        *self == LCDCSS61_A::LCDCSS61_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS61_1`"]
    #[inline(always)]
    pub fn is_lcdcss61_1(&self) -> bool {
        *self == LCDCSS61_A::LCDCSS61_1
    }
}
#[doc = "Field `LCDCSS61` writer - L61 Com Seg select"]
pub type LCDCSS61_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS61_A, O>;
impl<'a, const O: u8> LCDCSS61_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss61_0(self) -> &'a mut W {
        self.variant(LCDCSS61_A::LCDCSS61_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss61_1(self) -> &'a mut W {
        self.variant(LCDCSS61_A::LCDCSS61_1)
    }
}
#[doc = "Field `LCDCSS62` reader - L62 Com Seg select"]
pub type LCDCSS62_R = crate::BitReader<LCDCSS62_A>;
#[doc = "L62 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS62_A {
    #[doc = "0: Segment line"]
    LCDCSS62_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS62_1 = 1,
}
impl From<LCDCSS62_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS62_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS62_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS62_A {
        match self.bits {
            false => LCDCSS62_A::LCDCSS62_0,
            true => LCDCSS62_A::LCDCSS62_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS62_0`"]
    #[inline(always)]
    pub fn is_lcdcss62_0(&self) -> bool {
        *self == LCDCSS62_A::LCDCSS62_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS62_1`"]
    #[inline(always)]
    pub fn is_lcdcss62_1(&self) -> bool {
        *self == LCDCSS62_A::LCDCSS62_1
    }
}
#[doc = "Field `LCDCSS62` writer - L62 Com Seg select"]
pub type LCDCSS62_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS62_A, O>;
impl<'a, const O: u8> LCDCSS62_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss62_0(self) -> &'a mut W {
        self.variant(LCDCSS62_A::LCDCSS62_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss62_1(self) -> &'a mut W {
        self.variant(LCDCSS62_A::LCDCSS62_1)
    }
}
#[doc = "Field `LCDCSS63` reader - L63 Com Seg select"]
pub type LCDCSS63_R = crate::BitReader<LCDCSS63_A>;
#[doc = "L63 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS63_A {
    #[doc = "0: Segment line"]
    LCDCSS63_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS63_1 = 1,
}
impl From<LCDCSS63_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS63_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS63_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS63_A {
        match self.bits {
            false => LCDCSS63_A::LCDCSS63_0,
            true => LCDCSS63_A::LCDCSS63_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS63_0`"]
    #[inline(always)]
    pub fn is_lcdcss63_0(&self) -> bool {
        *self == LCDCSS63_A::LCDCSS63_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS63_1`"]
    #[inline(always)]
    pub fn is_lcdcss63_1(&self) -> bool {
        *self == LCDCSS63_A::LCDCSS63_1
    }
}
#[doc = "Field `LCDCSS63` writer - L63 Com Seg select"]
pub type LCDCSS63_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL1_SPEC, LCDCSS63_A, O>;
impl<'a, const O: u8> LCDCSS63_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss63_0(self) -> &'a mut W {
        self.variant(LCDCSS63_A::LCDCSS63_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss63_1(self) -> &'a mut W {
        self.variant(LCDCSS63_A::LCDCSS63_1)
    }
}
impl R {
    #[doc = "Bit 0 - L32 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss32(&self) -> LCDCSS32_R {
        LCDCSS32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - L33 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss33(&self) -> LCDCSS33_R {
        LCDCSS33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - L34 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss34(&self) -> LCDCSS34_R {
        LCDCSS34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - L35 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss35(&self) -> LCDCSS35_R {
        LCDCSS35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - L36 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss36(&self) -> LCDCSS36_R {
        LCDCSS36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - L37 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss37(&self) -> LCDCSS37_R {
        LCDCSS37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - L38 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss38(&self) -> LCDCSS38_R {
        LCDCSS38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - L39 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss39(&self) -> LCDCSS39_R {
        LCDCSS39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - L40 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss40(&self) -> LCDCSS40_R {
        LCDCSS40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - L41 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss41(&self) -> LCDCSS41_R {
        LCDCSS41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - L42 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss42(&self) -> LCDCSS42_R {
        LCDCSS42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - L43 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss43(&self) -> LCDCSS43_R {
        LCDCSS43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - L44 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss44(&self) -> LCDCSS44_R {
        LCDCSS44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - L45 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss45(&self) -> LCDCSS45_R {
        LCDCSS45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - L46 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss46(&self) -> LCDCSS46_R {
        LCDCSS46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - L47 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss47(&self) -> LCDCSS47_R {
        LCDCSS47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - L48 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss48(&self) -> LCDCSS48_R {
        LCDCSS48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - L49 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss49(&self) -> LCDCSS49_R {
        LCDCSS49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - L50 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss50(&self) -> LCDCSS50_R {
        LCDCSS50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - L51 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss51(&self) -> LCDCSS51_R {
        LCDCSS51_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - L52 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss52(&self) -> LCDCSS52_R {
        LCDCSS52_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - L53 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss53(&self) -> LCDCSS53_R {
        LCDCSS53_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - L54 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss54(&self) -> LCDCSS54_R {
        LCDCSS54_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - L55 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss55(&self) -> LCDCSS55_R {
        LCDCSS55_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - L56 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss56(&self) -> LCDCSS56_R {
        LCDCSS56_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - L57 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss57(&self) -> LCDCSS57_R {
        LCDCSS57_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - L58 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss58(&self) -> LCDCSS58_R {
        LCDCSS58_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - L59 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss59(&self) -> LCDCSS59_R {
        LCDCSS59_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - L60 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss60(&self) -> LCDCSS60_R {
        LCDCSS60_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - L61 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss61(&self) -> LCDCSS61_R {
        LCDCSS61_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - L62 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss62(&self) -> LCDCSS62_R {
        LCDCSS62_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - L63 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss63(&self) -> LCDCSS63_R {
        LCDCSS63_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - L32 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss32(&mut self) -> LCDCSS32_W<0> {
        LCDCSS32_W::new(self)
    }
    #[doc = "Bit 1 - L33 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss33(&mut self) -> LCDCSS33_W<1> {
        LCDCSS33_W::new(self)
    }
    #[doc = "Bit 2 - L34 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss34(&mut self) -> LCDCSS34_W<2> {
        LCDCSS34_W::new(self)
    }
    #[doc = "Bit 3 - L35 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss35(&mut self) -> LCDCSS35_W<3> {
        LCDCSS35_W::new(self)
    }
    #[doc = "Bit 4 - L36 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss36(&mut self) -> LCDCSS36_W<4> {
        LCDCSS36_W::new(self)
    }
    #[doc = "Bit 5 - L37 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss37(&mut self) -> LCDCSS37_W<5> {
        LCDCSS37_W::new(self)
    }
    #[doc = "Bit 6 - L38 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss38(&mut self) -> LCDCSS38_W<6> {
        LCDCSS38_W::new(self)
    }
    #[doc = "Bit 7 - L39 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss39(&mut self) -> LCDCSS39_W<7> {
        LCDCSS39_W::new(self)
    }
    #[doc = "Bit 8 - L40 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss40(&mut self) -> LCDCSS40_W<8> {
        LCDCSS40_W::new(self)
    }
    #[doc = "Bit 9 - L41 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss41(&mut self) -> LCDCSS41_W<9> {
        LCDCSS41_W::new(self)
    }
    #[doc = "Bit 10 - L42 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss42(&mut self) -> LCDCSS42_W<10> {
        LCDCSS42_W::new(self)
    }
    #[doc = "Bit 11 - L43 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss43(&mut self) -> LCDCSS43_W<11> {
        LCDCSS43_W::new(self)
    }
    #[doc = "Bit 12 - L44 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss44(&mut self) -> LCDCSS44_W<12> {
        LCDCSS44_W::new(self)
    }
    #[doc = "Bit 13 - L45 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss45(&mut self) -> LCDCSS45_W<13> {
        LCDCSS45_W::new(self)
    }
    #[doc = "Bit 14 - L46 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss46(&mut self) -> LCDCSS46_W<14> {
        LCDCSS46_W::new(self)
    }
    #[doc = "Bit 15 - L47 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss47(&mut self) -> LCDCSS47_W<15> {
        LCDCSS47_W::new(self)
    }
    #[doc = "Bit 16 - L48 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss48(&mut self) -> LCDCSS48_W<16> {
        LCDCSS48_W::new(self)
    }
    #[doc = "Bit 17 - L49 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss49(&mut self) -> LCDCSS49_W<17> {
        LCDCSS49_W::new(self)
    }
    #[doc = "Bit 18 - L50 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss50(&mut self) -> LCDCSS50_W<18> {
        LCDCSS50_W::new(self)
    }
    #[doc = "Bit 19 - L51 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss51(&mut self) -> LCDCSS51_W<19> {
        LCDCSS51_W::new(self)
    }
    #[doc = "Bit 20 - L52 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss52(&mut self) -> LCDCSS52_W<20> {
        LCDCSS52_W::new(self)
    }
    #[doc = "Bit 21 - L53 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss53(&mut self) -> LCDCSS53_W<21> {
        LCDCSS53_W::new(self)
    }
    #[doc = "Bit 22 - L54 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss54(&mut self) -> LCDCSS54_W<22> {
        LCDCSS54_W::new(self)
    }
    #[doc = "Bit 23 - L55 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss55(&mut self) -> LCDCSS55_W<23> {
        LCDCSS55_W::new(self)
    }
    #[doc = "Bit 24 - L56 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss56(&mut self) -> LCDCSS56_W<24> {
        LCDCSS56_W::new(self)
    }
    #[doc = "Bit 25 - L57 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss57(&mut self) -> LCDCSS57_W<25> {
        LCDCSS57_W::new(self)
    }
    #[doc = "Bit 26 - L58 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss58(&mut self) -> LCDCSS58_W<26> {
        LCDCSS58_W::new(self)
    }
    #[doc = "Bit 27 - L59 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss59(&mut self) -> LCDCSS59_W<27> {
        LCDCSS59_W::new(self)
    }
    #[doc = "Bit 28 - L60 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss60(&mut self) -> LCDCSS60_W<28> {
        LCDCSS60_W::new(self)
    }
    #[doc = "Bit 29 - L61 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss61(&mut self) -> LCDCSS61_W<29> {
        LCDCSS61_W::new(self)
    }
    #[doc = "Bit 30 - L62 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss62(&mut self) -> LCDCSS62_W<30> {
        LCDCSS62_W::new(self)
    }
    #[doc = "Bit 31 - L63 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss63(&mut self) -> LCDCSS63_W<31> {
        LCDCSS63_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_F COM/SEG select register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdcssel1](index.html) module"]
pub struct LCDCSSEL1_SPEC;
impl crate::RegisterSpec for LCDCSSEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcdcssel1::R](R) reader structure"]
impl crate::Readable for LCDCSSEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdcssel1::W](W) writer structure"]
impl crate::Writable for LCDCSSEL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDCSSEL1 to value 0"]
impl crate::Resettable for LCDCSSEL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
