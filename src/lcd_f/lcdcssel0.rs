#[doc = "Register `LCDCSSEL0` reader"]
pub struct R(crate::R<LCDCSSEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDCSSEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDCSSEL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDCSSEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDCSSEL0` writer"]
pub struct W(crate::W<LCDCSSEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDCSSEL0_SPEC>;
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
impl From<crate::W<LCDCSSEL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCDCSSEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDCSS0` reader - L0 Com Seg select"]
pub type LCDCSS0_R = crate::BitReader<LCDCSS0_A>;
#[doc = "L0 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS0_A {
    #[doc = "0: Segment line"]
    LCDCSS0_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS0_1 = 1,
}
impl From<LCDCSS0_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS0_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS0_A {
        match self.bits {
            false => LCDCSS0_A::LCDCSS0_0,
            true => LCDCSS0_A::LCDCSS0_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS0_0`"]
    #[inline(always)]
    pub fn is_lcdcss0_0(&self) -> bool {
        *self == LCDCSS0_A::LCDCSS0_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS0_1`"]
    #[inline(always)]
    pub fn is_lcdcss0_1(&self) -> bool {
        *self == LCDCSS0_A::LCDCSS0_1
    }
}
#[doc = "Field `LCDCSS0` writer - L0 Com Seg select"]
pub type LCDCSS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS0_A, O>;
impl<'a, const O: u8> LCDCSS0_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss0_0(self) -> &'a mut W {
        self.variant(LCDCSS0_A::LCDCSS0_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss0_1(self) -> &'a mut W {
        self.variant(LCDCSS0_A::LCDCSS0_1)
    }
}
#[doc = "Field `LCDCSS1` reader - L1 Com Seg select"]
pub type LCDCSS1_R = crate::BitReader<LCDCSS1_A>;
#[doc = "L1 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS1_A {
    #[doc = "0: Segment line"]
    LCDCSS1_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS1_1 = 1,
}
impl From<LCDCSS1_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS1_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS1_A {
        match self.bits {
            false => LCDCSS1_A::LCDCSS1_0,
            true => LCDCSS1_A::LCDCSS1_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS1_0`"]
    #[inline(always)]
    pub fn is_lcdcss1_0(&self) -> bool {
        *self == LCDCSS1_A::LCDCSS1_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS1_1`"]
    #[inline(always)]
    pub fn is_lcdcss1_1(&self) -> bool {
        *self == LCDCSS1_A::LCDCSS1_1
    }
}
#[doc = "Field `LCDCSS1` writer - L1 Com Seg select"]
pub type LCDCSS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS1_A, O>;
impl<'a, const O: u8> LCDCSS1_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss1_0(self) -> &'a mut W {
        self.variant(LCDCSS1_A::LCDCSS1_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss1_1(self) -> &'a mut W {
        self.variant(LCDCSS1_A::LCDCSS1_1)
    }
}
#[doc = "Field `LCDCSS2` reader - L2 Com Seg select"]
pub type LCDCSS2_R = crate::BitReader<LCDCSS2_A>;
#[doc = "L2 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS2_A {
    #[doc = "0: Segment line"]
    LCDCSS2_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS2_1 = 1,
}
impl From<LCDCSS2_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS2_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS2_A {
        match self.bits {
            false => LCDCSS2_A::LCDCSS2_0,
            true => LCDCSS2_A::LCDCSS2_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS2_0`"]
    #[inline(always)]
    pub fn is_lcdcss2_0(&self) -> bool {
        *self == LCDCSS2_A::LCDCSS2_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS2_1`"]
    #[inline(always)]
    pub fn is_lcdcss2_1(&self) -> bool {
        *self == LCDCSS2_A::LCDCSS2_1
    }
}
#[doc = "Field `LCDCSS2` writer - L2 Com Seg select"]
pub type LCDCSS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS2_A, O>;
impl<'a, const O: u8> LCDCSS2_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss2_0(self) -> &'a mut W {
        self.variant(LCDCSS2_A::LCDCSS2_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss2_1(self) -> &'a mut W {
        self.variant(LCDCSS2_A::LCDCSS2_1)
    }
}
#[doc = "Field `LCDCSS3` reader - L3 Com Seg select"]
pub type LCDCSS3_R = crate::BitReader<LCDCSS3_A>;
#[doc = "L3 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS3_A {
    #[doc = "0: Segment line"]
    LCDCSS3_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS3_1 = 1,
}
impl From<LCDCSS3_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS3_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS3_A {
        match self.bits {
            false => LCDCSS3_A::LCDCSS3_0,
            true => LCDCSS3_A::LCDCSS3_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS3_0`"]
    #[inline(always)]
    pub fn is_lcdcss3_0(&self) -> bool {
        *self == LCDCSS3_A::LCDCSS3_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS3_1`"]
    #[inline(always)]
    pub fn is_lcdcss3_1(&self) -> bool {
        *self == LCDCSS3_A::LCDCSS3_1
    }
}
#[doc = "Field `LCDCSS3` writer - L3 Com Seg select"]
pub type LCDCSS3_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS3_A, O>;
impl<'a, const O: u8> LCDCSS3_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss3_0(self) -> &'a mut W {
        self.variant(LCDCSS3_A::LCDCSS3_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss3_1(self) -> &'a mut W {
        self.variant(LCDCSS3_A::LCDCSS3_1)
    }
}
#[doc = "Field `LCDCSS4` reader - L4 Com Seg select"]
pub type LCDCSS4_R = crate::BitReader<LCDCSS4_A>;
#[doc = "L4 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS4_A {
    #[doc = "0: Segment line"]
    LCDCSS4_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS4_1 = 1,
}
impl From<LCDCSS4_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS4_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS4_A {
        match self.bits {
            false => LCDCSS4_A::LCDCSS4_0,
            true => LCDCSS4_A::LCDCSS4_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS4_0`"]
    #[inline(always)]
    pub fn is_lcdcss4_0(&self) -> bool {
        *self == LCDCSS4_A::LCDCSS4_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS4_1`"]
    #[inline(always)]
    pub fn is_lcdcss4_1(&self) -> bool {
        *self == LCDCSS4_A::LCDCSS4_1
    }
}
#[doc = "Field `LCDCSS4` writer - L4 Com Seg select"]
pub type LCDCSS4_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS4_A, O>;
impl<'a, const O: u8> LCDCSS4_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss4_0(self) -> &'a mut W {
        self.variant(LCDCSS4_A::LCDCSS4_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss4_1(self) -> &'a mut W {
        self.variant(LCDCSS4_A::LCDCSS4_1)
    }
}
#[doc = "Field `LCDCSS5` reader - L5 Com Seg select"]
pub type LCDCSS5_R = crate::BitReader<LCDCSS5_A>;
#[doc = "L5 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS5_A {
    #[doc = "0: Segment line"]
    LCDCSS5_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS5_1 = 1,
}
impl From<LCDCSS5_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS5_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS5_A {
        match self.bits {
            false => LCDCSS5_A::LCDCSS5_0,
            true => LCDCSS5_A::LCDCSS5_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS5_0`"]
    #[inline(always)]
    pub fn is_lcdcss5_0(&self) -> bool {
        *self == LCDCSS5_A::LCDCSS5_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS5_1`"]
    #[inline(always)]
    pub fn is_lcdcss5_1(&self) -> bool {
        *self == LCDCSS5_A::LCDCSS5_1
    }
}
#[doc = "Field `LCDCSS5` writer - L5 Com Seg select"]
pub type LCDCSS5_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS5_A, O>;
impl<'a, const O: u8> LCDCSS5_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss5_0(self) -> &'a mut W {
        self.variant(LCDCSS5_A::LCDCSS5_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss5_1(self) -> &'a mut W {
        self.variant(LCDCSS5_A::LCDCSS5_1)
    }
}
#[doc = "Field `LCDCSS6` reader - L6 Com Seg select"]
pub type LCDCSS6_R = crate::BitReader<LCDCSS6_A>;
#[doc = "L6 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS6_A {
    #[doc = "0: Segment line"]
    LCDCSS6_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS6_1 = 1,
}
impl From<LCDCSS6_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS6_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS6_A {
        match self.bits {
            false => LCDCSS6_A::LCDCSS6_0,
            true => LCDCSS6_A::LCDCSS6_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS6_0`"]
    #[inline(always)]
    pub fn is_lcdcss6_0(&self) -> bool {
        *self == LCDCSS6_A::LCDCSS6_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS6_1`"]
    #[inline(always)]
    pub fn is_lcdcss6_1(&self) -> bool {
        *self == LCDCSS6_A::LCDCSS6_1
    }
}
#[doc = "Field `LCDCSS6` writer - L6 Com Seg select"]
pub type LCDCSS6_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS6_A, O>;
impl<'a, const O: u8> LCDCSS6_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss6_0(self) -> &'a mut W {
        self.variant(LCDCSS6_A::LCDCSS6_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss6_1(self) -> &'a mut W {
        self.variant(LCDCSS6_A::LCDCSS6_1)
    }
}
#[doc = "Field `LCDCSS7` reader - L7 Com Seg select"]
pub type LCDCSS7_R = crate::BitReader<LCDCSS7_A>;
#[doc = "L7 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS7_A {
    #[doc = "0: Segment line"]
    LCDCSS7_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS7_1 = 1,
}
impl From<LCDCSS7_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS7_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS7_A {
        match self.bits {
            false => LCDCSS7_A::LCDCSS7_0,
            true => LCDCSS7_A::LCDCSS7_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS7_0`"]
    #[inline(always)]
    pub fn is_lcdcss7_0(&self) -> bool {
        *self == LCDCSS7_A::LCDCSS7_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS7_1`"]
    #[inline(always)]
    pub fn is_lcdcss7_1(&self) -> bool {
        *self == LCDCSS7_A::LCDCSS7_1
    }
}
#[doc = "Field `LCDCSS7` writer - L7 Com Seg select"]
pub type LCDCSS7_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS7_A, O>;
impl<'a, const O: u8> LCDCSS7_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss7_0(self) -> &'a mut W {
        self.variant(LCDCSS7_A::LCDCSS7_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss7_1(self) -> &'a mut W {
        self.variant(LCDCSS7_A::LCDCSS7_1)
    }
}
#[doc = "Field `LCDCSS8` reader - L8 Com Seg select"]
pub type LCDCSS8_R = crate::BitReader<LCDCSS8_A>;
#[doc = "L8 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS8_A {
    #[doc = "0: Segment line"]
    LCDCSS8_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS8_1 = 1,
}
impl From<LCDCSS8_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS8_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS8_A {
        match self.bits {
            false => LCDCSS8_A::LCDCSS8_0,
            true => LCDCSS8_A::LCDCSS8_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS8_0`"]
    #[inline(always)]
    pub fn is_lcdcss8_0(&self) -> bool {
        *self == LCDCSS8_A::LCDCSS8_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS8_1`"]
    #[inline(always)]
    pub fn is_lcdcss8_1(&self) -> bool {
        *self == LCDCSS8_A::LCDCSS8_1
    }
}
#[doc = "Field `LCDCSS8` writer - L8 Com Seg select"]
pub type LCDCSS8_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS8_A, O>;
impl<'a, const O: u8> LCDCSS8_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss8_0(self) -> &'a mut W {
        self.variant(LCDCSS8_A::LCDCSS8_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss8_1(self) -> &'a mut W {
        self.variant(LCDCSS8_A::LCDCSS8_1)
    }
}
#[doc = "Field `LCDCSS9` reader - L9 Com Seg select"]
pub type LCDCSS9_R = crate::BitReader<LCDCSS9_A>;
#[doc = "L9 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS9_A {
    #[doc = "0: Segment line"]
    LCDCSS9_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS9_1 = 1,
}
impl From<LCDCSS9_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS9_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS9_A {
        match self.bits {
            false => LCDCSS9_A::LCDCSS9_0,
            true => LCDCSS9_A::LCDCSS9_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS9_0`"]
    #[inline(always)]
    pub fn is_lcdcss9_0(&self) -> bool {
        *self == LCDCSS9_A::LCDCSS9_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS9_1`"]
    #[inline(always)]
    pub fn is_lcdcss9_1(&self) -> bool {
        *self == LCDCSS9_A::LCDCSS9_1
    }
}
#[doc = "Field `LCDCSS9` writer - L9 Com Seg select"]
pub type LCDCSS9_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS9_A, O>;
impl<'a, const O: u8> LCDCSS9_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss9_0(self) -> &'a mut W {
        self.variant(LCDCSS9_A::LCDCSS9_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss9_1(self) -> &'a mut W {
        self.variant(LCDCSS9_A::LCDCSS9_1)
    }
}
#[doc = "Field `LCDCSS10` reader - L10 Com Seg select"]
pub type LCDCSS10_R = crate::BitReader<LCDCSS10_A>;
#[doc = "L10 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS10_A {
    #[doc = "0: Segment line"]
    LCDCSS10_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS10_1 = 1,
}
impl From<LCDCSS10_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS10_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS10_A {
        match self.bits {
            false => LCDCSS10_A::LCDCSS10_0,
            true => LCDCSS10_A::LCDCSS10_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS10_0`"]
    #[inline(always)]
    pub fn is_lcdcss10_0(&self) -> bool {
        *self == LCDCSS10_A::LCDCSS10_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS10_1`"]
    #[inline(always)]
    pub fn is_lcdcss10_1(&self) -> bool {
        *self == LCDCSS10_A::LCDCSS10_1
    }
}
#[doc = "Field `LCDCSS10` writer - L10 Com Seg select"]
pub type LCDCSS10_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS10_A, O>;
impl<'a, const O: u8> LCDCSS10_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss10_0(self) -> &'a mut W {
        self.variant(LCDCSS10_A::LCDCSS10_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss10_1(self) -> &'a mut W {
        self.variant(LCDCSS10_A::LCDCSS10_1)
    }
}
#[doc = "Field `LCDCSS11` reader - L11 Com Seg select"]
pub type LCDCSS11_R = crate::BitReader<LCDCSS11_A>;
#[doc = "L11 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS11_A {
    #[doc = "0: Segment line"]
    LCDCSS11_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS11_1 = 1,
}
impl From<LCDCSS11_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS11_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS11_A {
        match self.bits {
            false => LCDCSS11_A::LCDCSS11_0,
            true => LCDCSS11_A::LCDCSS11_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS11_0`"]
    #[inline(always)]
    pub fn is_lcdcss11_0(&self) -> bool {
        *self == LCDCSS11_A::LCDCSS11_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS11_1`"]
    #[inline(always)]
    pub fn is_lcdcss11_1(&self) -> bool {
        *self == LCDCSS11_A::LCDCSS11_1
    }
}
#[doc = "Field `LCDCSS11` writer - L11 Com Seg select"]
pub type LCDCSS11_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS11_A, O>;
impl<'a, const O: u8> LCDCSS11_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss11_0(self) -> &'a mut W {
        self.variant(LCDCSS11_A::LCDCSS11_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss11_1(self) -> &'a mut W {
        self.variant(LCDCSS11_A::LCDCSS11_1)
    }
}
#[doc = "Field `LCDCSS12` reader - L12 Com Seg select"]
pub type LCDCSS12_R = crate::BitReader<LCDCSS12_A>;
#[doc = "L12 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS12_A {
    #[doc = "0: Segment line"]
    LCDCSS12_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS12_1 = 1,
}
impl From<LCDCSS12_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS12_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS12_A {
        match self.bits {
            false => LCDCSS12_A::LCDCSS12_0,
            true => LCDCSS12_A::LCDCSS12_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS12_0`"]
    #[inline(always)]
    pub fn is_lcdcss12_0(&self) -> bool {
        *self == LCDCSS12_A::LCDCSS12_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS12_1`"]
    #[inline(always)]
    pub fn is_lcdcss12_1(&self) -> bool {
        *self == LCDCSS12_A::LCDCSS12_1
    }
}
#[doc = "Field `LCDCSS12` writer - L12 Com Seg select"]
pub type LCDCSS12_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS12_A, O>;
impl<'a, const O: u8> LCDCSS12_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss12_0(self) -> &'a mut W {
        self.variant(LCDCSS12_A::LCDCSS12_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss12_1(self) -> &'a mut W {
        self.variant(LCDCSS12_A::LCDCSS12_1)
    }
}
#[doc = "Field `LCDCSS13` reader - L13 Com Seg select"]
pub type LCDCSS13_R = crate::BitReader<LCDCSS13_A>;
#[doc = "L13 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS13_A {
    #[doc = "0: Segment line"]
    LCDCSS13_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS13_1 = 1,
}
impl From<LCDCSS13_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS13_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS13_A {
        match self.bits {
            false => LCDCSS13_A::LCDCSS13_0,
            true => LCDCSS13_A::LCDCSS13_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS13_0`"]
    #[inline(always)]
    pub fn is_lcdcss13_0(&self) -> bool {
        *self == LCDCSS13_A::LCDCSS13_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS13_1`"]
    #[inline(always)]
    pub fn is_lcdcss13_1(&self) -> bool {
        *self == LCDCSS13_A::LCDCSS13_1
    }
}
#[doc = "Field `LCDCSS13` writer - L13 Com Seg select"]
pub type LCDCSS13_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS13_A, O>;
impl<'a, const O: u8> LCDCSS13_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss13_0(self) -> &'a mut W {
        self.variant(LCDCSS13_A::LCDCSS13_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss13_1(self) -> &'a mut W {
        self.variant(LCDCSS13_A::LCDCSS13_1)
    }
}
#[doc = "Field `LCDCSS14` reader - L14 Com Seg select"]
pub type LCDCSS14_R = crate::BitReader<LCDCSS14_A>;
#[doc = "L14 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS14_A {
    #[doc = "0: Segment line"]
    LCDCSS14_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS14_1 = 1,
}
impl From<LCDCSS14_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS14_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS14_A {
        match self.bits {
            false => LCDCSS14_A::LCDCSS14_0,
            true => LCDCSS14_A::LCDCSS14_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS14_0`"]
    #[inline(always)]
    pub fn is_lcdcss14_0(&self) -> bool {
        *self == LCDCSS14_A::LCDCSS14_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS14_1`"]
    #[inline(always)]
    pub fn is_lcdcss14_1(&self) -> bool {
        *self == LCDCSS14_A::LCDCSS14_1
    }
}
#[doc = "Field `LCDCSS14` writer - L14 Com Seg select"]
pub type LCDCSS14_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS14_A, O>;
impl<'a, const O: u8> LCDCSS14_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss14_0(self) -> &'a mut W {
        self.variant(LCDCSS14_A::LCDCSS14_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss14_1(self) -> &'a mut W {
        self.variant(LCDCSS14_A::LCDCSS14_1)
    }
}
#[doc = "Field `LCDCSS15` reader - L15 Com Seg select"]
pub type LCDCSS15_R = crate::BitReader<LCDCSS15_A>;
#[doc = "L15 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS15_A {
    #[doc = "0: Segment line"]
    LCDCSS15_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS15_1 = 1,
}
impl From<LCDCSS15_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS15_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS15_A {
        match self.bits {
            false => LCDCSS15_A::LCDCSS15_0,
            true => LCDCSS15_A::LCDCSS15_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS15_0`"]
    #[inline(always)]
    pub fn is_lcdcss15_0(&self) -> bool {
        *self == LCDCSS15_A::LCDCSS15_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS15_1`"]
    #[inline(always)]
    pub fn is_lcdcss15_1(&self) -> bool {
        *self == LCDCSS15_A::LCDCSS15_1
    }
}
#[doc = "Field `LCDCSS15` writer - L15 Com Seg select"]
pub type LCDCSS15_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS15_A, O>;
impl<'a, const O: u8> LCDCSS15_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss15_0(self) -> &'a mut W {
        self.variant(LCDCSS15_A::LCDCSS15_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss15_1(self) -> &'a mut W {
        self.variant(LCDCSS15_A::LCDCSS15_1)
    }
}
#[doc = "Field `LCDCSS16` reader - L16 Com Seg select"]
pub type LCDCSS16_R = crate::BitReader<LCDCSS16_A>;
#[doc = "L16 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS16_A {
    #[doc = "0: Segment line"]
    LCDCSS16_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS16_1 = 1,
}
impl From<LCDCSS16_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS16_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS16_A {
        match self.bits {
            false => LCDCSS16_A::LCDCSS16_0,
            true => LCDCSS16_A::LCDCSS16_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS16_0`"]
    #[inline(always)]
    pub fn is_lcdcss16_0(&self) -> bool {
        *self == LCDCSS16_A::LCDCSS16_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS16_1`"]
    #[inline(always)]
    pub fn is_lcdcss16_1(&self) -> bool {
        *self == LCDCSS16_A::LCDCSS16_1
    }
}
#[doc = "Field `LCDCSS16` writer - L16 Com Seg select"]
pub type LCDCSS16_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS16_A, O>;
impl<'a, const O: u8> LCDCSS16_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss16_0(self) -> &'a mut W {
        self.variant(LCDCSS16_A::LCDCSS16_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss16_1(self) -> &'a mut W {
        self.variant(LCDCSS16_A::LCDCSS16_1)
    }
}
#[doc = "Field `LCDCSS17` reader - L17 Com Seg select"]
pub type LCDCSS17_R = crate::BitReader<LCDCSS17_A>;
#[doc = "L17 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS17_A {
    #[doc = "0: Segment line"]
    LCDCSS17_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS17_1 = 1,
}
impl From<LCDCSS17_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS17_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS17_A {
        match self.bits {
            false => LCDCSS17_A::LCDCSS17_0,
            true => LCDCSS17_A::LCDCSS17_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS17_0`"]
    #[inline(always)]
    pub fn is_lcdcss17_0(&self) -> bool {
        *self == LCDCSS17_A::LCDCSS17_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS17_1`"]
    #[inline(always)]
    pub fn is_lcdcss17_1(&self) -> bool {
        *self == LCDCSS17_A::LCDCSS17_1
    }
}
#[doc = "Field `LCDCSS17` writer - L17 Com Seg select"]
pub type LCDCSS17_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS17_A, O>;
impl<'a, const O: u8> LCDCSS17_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss17_0(self) -> &'a mut W {
        self.variant(LCDCSS17_A::LCDCSS17_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss17_1(self) -> &'a mut W {
        self.variant(LCDCSS17_A::LCDCSS17_1)
    }
}
#[doc = "Field `LCDCSS18` reader - L18 Com Seg select"]
pub type LCDCSS18_R = crate::BitReader<LCDCSS18_A>;
#[doc = "L18 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS18_A {
    #[doc = "0: Segment line"]
    LCDCSS18_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS18_1 = 1,
}
impl From<LCDCSS18_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS18_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS18_A {
        match self.bits {
            false => LCDCSS18_A::LCDCSS18_0,
            true => LCDCSS18_A::LCDCSS18_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS18_0`"]
    #[inline(always)]
    pub fn is_lcdcss18_0(&self) -> bool {
        *self == LCDCSS18_A::LCDCSS18_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS18_1`"]
    #[inline(always)]
    pub fn is_lcdcss18_1(&self) -> bool {
        *self == LCDCSS18_A::LCDCSS18_1
    }
}
#[doc = "Field `LCDCSS18` writer - L18 Com Seg select"]
pub type LCDCSS18_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS18_A, O>;
impl<'a, const O: u8> LCDCSS18_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss18_0(self) -> &'a mut W {
        self.variant(LCDCSS18_A::LCDCSS18_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss18_1(self) -> &'a mut W {
        self.variant(LCDCSS18_A::LCDCSS18_1)
    }
}
#[doc = "Field `LCDCSS19` reader - L19 Com Seg select"]
pub type LCDCSS19_R = crate::BitReader<LCDCSS19_A>;
#[doc = "L19 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS19_A {
    #[doc = "0: Segment line"]
    LCDCSS19_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS19_1 = 1,
}
impl From<LCDCSS19_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS19_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS19_A {
        match self.bits {
            false => LCDCSS19_A::LCDCSS19_0,
            true => LCDCSS19_A::LCDCSS19_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS19_0`"]
    #[inline(always)]
    pub fn is_lcdcss19_0(&self) -> bool {
        *self == LCDCSS19_A::LCDCSS19_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS19_1`"]
    #[inline(always)]
    pub fn is_lcdcss19_1(&self) -> bool {
        *self == LCDCSS19_A::LCDCSS19_1
    }
}
#[doc = "Field `LCDCSS19` writer - L19 Com Seg select"]
pub type LCDCSS19_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS19_A, O>;
impl<'a, const O: u8> LCDCSS19_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss19_0(self) -> &'a mut W {
        self.variant(LCDCSS19_A::LCDCSS19_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss19_1(self) -> &'a mut W {
        self.variant(LCDCSS19_A::LCDCSS19_1)
    }
}
#[doc = "Field `LCDCSS20` reader - L20 Com Seg select"]
pub type LCDCSS20_R = crate::BitReader<LCDCSS20_A>;
#[doc = "L20 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS20_A {
    #[doc = "0: Segment line"]
    LCDCSS20_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS20_1 = 1,
}
impl From<LCDCSS20_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS20_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS20_A {
        match self.bits {
            false => LCDCSS20_A::LCDCSS20_0,
            true => LCDCSS20_A::LCDCSS20_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS20_0`"]
    #[inline(always)]
    pub fn is_lcdcss20_0(&self) -> bool {
        *self == LCDCSS20_A::LCDCSS20_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS20_1`"]
    #[inline(always)]
    pub fn is_lcdcss20_1(&self) -> bool {
        *self == LCDCSS20_A::LCDCSS20_1
    }
}
#[doc = "Field `LCDCSS20` writer - L20 Com Seg select"]
pub type LCDCSS20_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS20_A, O>;
impl<'a, const O: u8> LCDCSS20_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss20_0(self) -> &'a mut W {
        self.variant(LCDCSS20_A::LCDCSS20_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss20_1(self) -> &'a mut W {
        self.variant(LCDCSS20_A::LCDCSS20_1)
    }
}
#[doc = "Field `LCDCSS21` reader - L21 Com Seg select"]
pub type LCDCSS21_R = crate::BitReader<LCDCSS21_A>;
#[doc = "L21 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS21_A {
    #[doc = "0: Segment line"]
    LCDCSS21_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS21_1 = 1,
}
impl From<LCDCSS21_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS21_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS21_A {
        match self.bits {
            false => LCDCSS21_A::LCDCSS21_0,
            true => LCDCSS21_A::LCDCSS21_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS21_0`"]
    #[inline(always)]
    pub fn is_lcdcss21_0(&self) -> bool {
        *self == LCDCSS21_A::LCDCSS21_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS21_1`"]
    #[inline(always)]
    pub fn is_lcdcss21_1(&self) -> bool {
        *self == LCDCSS21_A::LCDCSS21_1
    }
}
#[doc = "Field `LCDCSS21` writer - L21 Com Seg select"]
pub type LCDCSS21_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS21_A, O>;
impl<'a, const O: u8> LCDCSS21_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss21_0(self) -> &'a mut W {
        self.variant(LCDCSS21_A::LCDCSS21_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss21_1(self) -> &'a mut W {
        self.variant(LCDCSS21_A::LCDCSS21_1)
    }
}
#[doc = "Field `LCDCSS22` reader - L22 Com Seg select"]
pub type LCDCSS22_R = crate::BitReader<LCDCSS22_A>;
#[doc = "L22 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS22_A {
    #[doc = "0: Segment line"]
    LCDCSS22_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS22_1 = 1,
}
impl From<LCDCSS22_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS22_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS22_A {
        match self.bits {
            false => LCDCSS22_A::LCDCSS22_0,
            true => LCDCSS22_A::LCDCSS22_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS22_0`"]
    #[inline(always)]
    pub fn is_lcdcss22_0(&self) -> bool {
        *self == LCDCSS22_A::LCDCSS22_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS22_1`"]
    #[inline(always)]
    pub fn is_lcdcss22_1(&self) -> bool {
        *self == LCDCSS22_A::LCDCSS22_1
    }
}
#[doc = "Field `LCDCSS22` writer - L22 Com Seg select"]
pub type LCDCSS22_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS22_A, O>;
impl<'a, const O: u8> LCDCSS22_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss22_0(self) -> &'a mut W {
        self.variant(LCDCSS22_A::LCDCSS22_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss22_1(self) -> &'a mut W {
        self.variant(LCDCSS22_A::LCDCSS22_1)
    }
}
#[doc = "Field `LCDCSS23` reader - L23 Com Seg select"]
pub type LCDCSS23_R = crate::BitReader<LCDCSS23_A>;
#[doc = "L23 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS23_A {
    #[doc = "0: Segment line"]
    LCDCSS23_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS23_1 = 1,
}
impl From<LCDCSS23_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS23_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS23_A {
        match self.bits {
            false => LCDCSS23_A::LCDCSS23_0,
            true => LCDCSS23_A::LCDCSS23_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS23_0`"]
    #[inline(always)]
    pub fn is_lcdcss23_0(&self) -> bool {
        *self == LCDCSS23_A::LCDCSS23_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS23_1`"]
    #[inline(always)]
    pub fn is_lcdcss23_1(&self) -> bool {
        *self == LCDCSS23_A::LCDCSS23_1
    }
}
#[doc = "Field `LCDCSS23` writer - L23 Com Seg select"]
pub type LCDCSS23_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS23_A, O>;
impl<'a, const O: u8> LCDCSS23_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss23_0(self) -> &'a mut W {
        self.variant(LCDCSS23_A::LCDCSS23_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss23_1(self) -> &'a mut W {
        self.variant(LCDCSS23_A::LCDCSS23_1)
    }
}
#[doc = "Field `LCDCSS24` reader - L24 Com Seg select"]
pub type LCDCSS24_R = crate::BitReader<LCDCSS24_A>;
#[doc = "L24 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS24_A {
    #[doc = "0: Segment line"]
    LCDCSS24_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS24_1 = 1,
}
impl From<LCDCSS24_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS24_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS24_A {
        match self.bits {
            false => LCDCSS24_A::LCDCSS24_0,
            true => LCDCSS24_A::LCDCSS24_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS24_0`"]
    #[inline(always)]
    pub fn is_lcdcss24_0(&self) -> bool {
        *self == LCDCSS24_A::LCDCSS24_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS24_1`"]
    #[inline(always)]
    pub fn is_lcdcss24_1(&self) -> bool {
        *self == LCDCSS24_A::LCDCSS24_1
    }
}
#[doc = "Field `LCDCSS24` writer - L24 Com Seg select"]
pub type LCDCSS24_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS24_A, O>;
impl<'a, const O: u8> LCDCSS24_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss24_0(self) -> &'a mut W {
        self.variant(LCDCSS24_A::LCDCSS24_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss24_1(self) -> &'a mut W {
        self.variant(LCDCSS24_A::LCDCSS24_1)
    }
}
#[doc = "Field `LCDCSS25` reader - L25 Com Seg select"]
pub type LCDCSS25_R = crate::BitReader<LCDCSS25_A>;
#[doc = "L25 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS25_A {
    #[doc = "0: Segment line"]
    LCDCSS25_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS25_1 = 1,
}
impl From<LCDCSS25_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS25_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS25_A {
        match self.bits {
            false => LCDCSS25_A::LCDCSS25_0,
            true => LCDCSS25_A::LCDCSS25_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS25_0`"]
    #[inline(always)]
    pub fn is_lcdcss25_0(&self) -> bool {
        *self == LCDCSS25_A::LCDCSS25_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS25_1`"]
    #[inline(always)]
    pub fn is_lcdcss25_1(&self) -> bool {
        *self == LCDCSS25_A::LCDCSS25_1
    }
}
#[doc = "Field `LCDCSS25` writer - L25 Com Seg select"]
pub type LCDCSS25_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS25_A, O>;
impl<'a, const O: u8> LCDCSS25_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss25_0(self) -> &'a mut W {
        self.variant(LCDCSS25_A::LCDCSS25_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss25_1(self) -> &'a mut W {
        self.variant(LCDCSS25_A::LCDCSS25_1)
    }
}
#[doc = "Field `LCDCSS26` reader - L26 Com Seg select"]
pub type LCDCSS26_R = crate::BitReader<LCDCSS26_A>;
#[doc = "L26 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS26_A {
    #[doc = "0: Segment line"]
    LCDCSS26_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS26_1 = 1,
}
impl From<LCDCSS26_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS26_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS26_A {
        match self.bits {
            false => LCDCSS26_A::LCDCSS26_0,
            true => LCDCSS26_A::LCDCSS26_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS26_0`"]
    #[inline(always)]
    pub fn is_lcdcss26_0(&self) -> bool {
        *self == LCDCSS26_A::LCDCSS26_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS26_1`"]
    #[inline(always)]
    pub fn is_lcdcss26_1(&self) -> bool {
        *self == LCDCSS26_A::LCDCSS26_1
    }
}
#[doc = "Field `LCDCSS26` writer - L26 Com Seg select"]
pub type LCDCSS26_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS26_A, O>;
impl<'a, const O: u8> LCDCSS26_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss26_0(self) -> &'a mut W {
        self.variant(LCDCSS26_A::LCDCSS26_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss26_1(self) -> &'a mut W {
        self.variant(LCDCSS26_A::LCDCSS26_1)
    }
}
#[doc = "Field `LCDCSS27` reader - L27 Com Seg select"]
pub type LCDCSS27_R = crate::BitReader<LCDCSS27_A>;
#[doc = "L27 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS27_A {
    #[doc = "0: Segment line"]
    LCDCSS27_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS27_1 = 1,
}
impl From<LCDCSS27_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS27_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS27_A {
        match self.bits {
            false => LCDCSS27_A::LCDCSS27_0,
            true => LCDCSS27_A::LCDCSS27_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS27_0`"]
    #[inline(always)]
    pub fn is_lcdcss27_0(&self) -> bool {
        *self == LCDCSS27_A::LCDCSS27_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS27_1`"]
    #[inline(always)]
    pub fn is_lcdcss27_1(&self) -> bool {
        *self == LCDCSS27_A::LCDCSS27_1
    }
}
#[doc = "Field `LCDCSS27` writer - L27 Com Seg select"]
pub type LCDCSS27_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS27_A, O>;
impl<'a, const O: u8> LCDCSS27_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss27_0(self) -> &'a mut W {
        self.variant(LCDCSS27_A::LCDCSS27_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss27_1(self) -> &'a mut W {
        self.variant(LCDCSS27_A::LCDCSS27_1)
    }
}
#[doc = "Field `LCDCSS28` reader - L28 Com Seg select"]
pub type LCDCSS28_R = crate::BitReader<LCDCSS28_A>;
#[doc = "L28 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS28_A {
    #[doc = "0: Segment line"]
    LCDCSS28_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS28_1 = 1,
}
impl From<LCDCSS28_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS28_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS28_A {
        match self.bits {
            false => LCDCSS28_A::LCDCSS28_0,
            true => LCDCSS28_A::LCDCSS28_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS28_0`"]
    #[inline(always)]
    pub fn is_lcdcss28_0(&self) -> bool {
        *self == LCDCSS28_A::LCDCSS28_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS28_1`"]
    #[inline(always)]
    pub fn is_lcdcss28_1(&self) -> bool {
        *self == LCDCSS28_A::LCDCSS28_1
    }
}
#[doc = "Field `LCDCSS28` writer - L28 Com Seg select"]
pub type LCDCSS28_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS28_A, O>;
impl<'a, const O: u8> LCDCSS28_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss28_0(self) -> &'a mut W {
        self.variant(LCDCSS28_A::LCDCSS28_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss28_1(self) -> &'a mut W {
        self.variant(LCDCSS28_A::LCDCSS28_1)
    }
}
#[doc = "Field `LCDCSS29` reader - L29 Com Seg select"]
pub type LCDCSS29_R = crate::BitReader<LCDCSS29_A>;
#[doc = "L29 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS29_A {
    #[doc = "0: Segment line"]
    LCDCSS29_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS29_1 = 1,
}
impl From<LCDCSS29_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS29_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS29_A {
        match self.bits {
            false => LCDCSS29_A::LCDCSS29_0,
            true => LCDCSS29_A::LCDCSS29_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS29_0`"]
    #[inline(always)]
    pub fn is_lcdcss29_0(&self) -> bool {
        *self == LCDCSS29_A::LCDCSS29_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS29_1`"]
    #[inline(always)]
    pub fn is_lcdcss29_1(&self) -> bool {
        *self == LCDCSS29_A::LCDCSS29_1
    }
}
#[doc = "Field `LCDCSS29` writer - L29 Com Seg select"]
pub type LCDCSS29_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS29_A, O>;
impl<'a, const O: u8> LCDCSS29_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss29_0(self) -> &'a mut W {
        self.variant(LCDCSS29_A::LCDCSS29_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss29_1(self) -> &'a mut W {
        self.variant(LCDCSS29_A::LCDCSS29_1)
    }
}
#[doc = "Field `LCDCSS30` reader - L30 Com Seg select"]
pub type LCDCSS30_R = crate::BitReader<LCDCSS30_A>;
#[doc = "L30 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS30_A {
    #[doc = "0: Segment line"]
    LCDCSS30_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS30_1 = 1,
}
impl From<LCDCSS30_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS30_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS30_A {
        match self.bits {
            false => LCDCSS30_A::LCDCSS30_0,
            true => LCDCSS30_A::LCDCSS30_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS30_0`"]
    #[inline(always)]
    pub fn is_lcdcss30_0(&self) -> bool {
        *self == LCDCSS30_A::LCDCSS30_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS30_1`"]
    #[inline(always)]
    pub fn is_lcdcss30_1(&self) -> bool {
        *self == LCDCSS30_A::LCDCSS30_1
    }
}
#[doc = "Field `LCDCSS30` writer - L30 Com Seg select"]
pub type LCDCSS30_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS30_A, O>;
impl<'a, const O: u8> LCDCSS30_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss30_0(self) -> &'a mut W {
        self.variant(LCDCSS30_A::LCDCSS30_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss30_1(self) -> &'a mut W {
        self.variant(LCDCSS30_A::LCDCSS30_1)
    }
}
#[doc = "Field `LCDCSS31` reader - L31 Com Seg select"]
pub type LCDCSS31_R = crate::BitReader<LCDCSS31_A>;
#[doc = "L31 Com Seg select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDCSS31_A {
    #[doc = "0: Segment line"]
    LCDCSS31_0 = 0,
    #[doc = "1: Common line"]
    LCDCSS31_1 = 1,
}
impl From<LCDCSS31_A> for bool {
    #[inline(always)]
    fn from(variant: LCDCSS31_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDCSS31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDCSS31_A {
        match self.bits {
            false => LCDCSS31_A::LCDCSS31_0,
            true => LCDCSS31_A::LCDCSS31_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDCSS31_0`"]
    #[inline(always)]
    pub fn is_lcdcss31_0(&self) -> bool {
        *self == LCDCSS31_A::LCDCSS31_0
    }
    #[doc = "Checks if the value of the field is `LCDCSS31_1`"]
    #[inline(always)]
    pub fn is_lcdcss31_1(&self) -> bool {
        *self == LCDCSS31_A::LCDCSS31_1
    }
}
#[doc = "Field `LCDCSS31` writer - L31 Com Seg select"]
pub type LCDCSS31_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDCSSEL0_SPEC, LCDCSS31_A, O>;
impl<'a, const O: u8> LCDCSS31_W<'a, O> {
    #[doc = "Segment line"]
    #[inline(always)]
    pub fn lcdcss31_0(self) -> &'a mut W {
        self.variant(LCDCSS31_A::LCDCSS31_0)
    }
    #[doc = "Common line"]
    #[inline(always)]
    pub fn lcdcss31_1(self) -> &'a mut W {
        self.variant(LCDCSS31_A::LCDCSS31_1)
    }
}
impl R {
    #[doc = "Bit 0 - L0 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss0(&self) -> LCDCSS0_R {
        LCDCSS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - L1 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss1(&self) -> LCDCSS1_R {
        LCDCSS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - L2 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss2(&self) -> LCDCSS2_R {
        LCDCSS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - L3 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss3(&self) -> LCDCSS3_R {
        LCDCSS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - L4 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss4(&self) -> LCDCSS4_R {
        LCDCSS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - L5 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss5(&self) -> LCDCSS5_R {
        LCDCSS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - L6 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss6(&self) -> LCDCSS6_R {
        LCDCSS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - L7 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss7(&self) -> LCDCSS7_R {
        LCDCSS7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - L8 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss8(&self) -> LCDCSS8_R {
        LCDCSS8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - L9 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss9(&self) -> LCDCSS9_R {
        LCDCSS9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - L10 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss10(&self) -> LCDCSS10_R {
        LCDCSS10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - L11 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss11(&self) -> LCDCSS11_R {
        LCDCSS11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - L12 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss12(&self) -> LCDCSS12_R {
        LCDCSS12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - L13 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss13(&self) -> LCDCSS13_R {
        LCDCSS13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - L14 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss14(&self) -> LCDCSS14_R {
        LCDCSS14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - L15 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss15(&self) -> LCDCSS15_R {
        LCDCSS15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - L16 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss16(&self) -> LCDCSS16_R {
        LCDCSS16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - L17 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss17(&self) -> LCDCSS17_R {
        LCDCSS17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - L18 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss18(&self) -> LCDCSS18_R {
        LCDCSS18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - L19 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss19(&self) -> LCDCSS19_R {
        LCDCSS19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - L20 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss20(&self) -> LCDCSS20_R {
        LCDCSS20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - L21 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss21(&self) -> LCDCSS21_R {
        LCDCSS21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - L22 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss22(&self) -> LCDCSS22_R {
        LCDCSS22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - L23 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss23(&self) -> LCDCSS23_R {
        LCDCSS23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - L24 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss24(&self) -> LCDCSS24_R {
        LCDCSS24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - L25 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss25(&self) -> LCDCSS25_R {
        LCDCSS25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - L26 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss26(&self) -> LCDCSS26_R {
        LCDCSS26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - L27 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss27(&self) -> LCDCSS27_R {
        LCDCSS27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - L28 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss28(&self) -> LCDCSS28_R {
        LCDCSS28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - L29 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss29(&self) -> LCDCSS29_R {
        LCDCSS29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - L30 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss30(&self) -> LCDCSS30_R {
        LCDCSS30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - L31 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss31(&self) -> LCDCSS31_R {
        LCDCSS31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - L0 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss0(&mut self) -> LCDCSS0_W<0> {
        LCDCSS0_W::new(self)
    }
    #[doc = "Bit 1 - L1 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss1(&mut self) -> LCDCSS1_W<1> {
        LCDCSS1_W::new(self)
    }
    #[doc = "Bit 2 - L2 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss2(&mut self) -> LCDCSS2_W<2> {
        LCDCSS2_W::new(self)
    }
    #[doc = "Bit 3 - L3 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss3(&mut self) -> LCDCSS3_W<3> {
        LCDCSS3_W::new(self)
    }
    #[doc = "Bit 4 - L4 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss4(&mut self) -> LCDCSS4_W<4> {
        LCDCSS4_W::new(self)
    }
    #[doc = "Bit 5 - L5 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss5(&mut self) -> LCDCSS5_W<5> {
        LCDCSS5_W::new(self)
    }
    #[doc = "Bit 6 - L6 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss6(&mut self) -> LCDCSS6_W<6> {
        LCDCSS6_W::new(self)
    }
    #[doc = "Bit 7 - L7 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss7(&mut self) -> LCDCSS7_W<7> {
        LCDCSS7_W::new(self)
    }
    #[doc = "Bit 8 - L8 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss8(&mut self) -> LCDCSS8_W<8> {
        LCDCSS8_W::new(self)
    }
    #[doc = "Bit 9 - L9 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss9(&mut self) -> LCDCSS9_W<9> {
        LCDCSS9_W::new(self)
    }
    #[doc = "Bit 10 - L10 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss10(&mut self) -> LCDCSS10_W<10> {
        LCDCSS10_W::new(self)
    }
    #[doc = "Bit 11 - L11 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss11(&mut self) -> LCDCSS11_W<11> {
        LCDCSS11_W::new(self)
    }
    #[doc = "Bit 12 - L12 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss12(&mut self) -> LCDCSS12_W<12> {
        LCDCSS12_W::new(self)
    }
    #[doc = "Bit 13 - L13 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss13(&mut self) -> LCDCSS13_W<13> {
        LCDCSS13_W::new(self)
    }
    #[doc = "Bit 14 - L14 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss14(&mut self) -> LCDCSS14_W<14> {
        LCDCSS14_W::new(self)
    }
    #[doc = "Bit 15 - L15 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss15(&mut self) -> LCDCSS15_W<15> {
        LCDCSS15_W::new(self)
    }
    #[doc = "Bit 16 - L16 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss16(&mut self) -> LCDCSS16_W<16> {
        LCDCSS16_W::new(self)
    }
    #[doc = "Bit 17 - L17 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss17(&mut self) -> LCDCSS17_W<17> {
        LCDCSS17_W::new(self)
    }
    #[doc = "Bit 18 - L18 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss18(&mut self) -> LCDCSS18_W<18> {
        LCDCSS18_W::new(self)
    }
    #[doc = "Bit 19 - L19 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss19(&mut self) -> LCDCSS19_W<19> {
        LCDCSS19_W::new(self)
    }
    #[doc = "Bit 20 - L20 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss20(&mut self) -> LCDCSS20_W<20> {
        LCDCSS20_W::new(self)
    }
    #[doc = "Bit 21 - L21 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss21(&mut self) -> LCDCSS21_W<21> {
        LCDCSS21_W::new(self)
    }
    #[doc = "Bit 22 - L22 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss22(&mut self) -> LCDCSS22_W<22> {
        LCDCSS22_W::new(self)
    }
    #[doc = "Bit 23 - L23 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss23(&mut self) -> LCDCSS23_W<23> {
        LCDCSS23_W::new(self)
    }
    #[doc = "Bit 24 - L24 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss24(&mut self) -> LCDCSS24_W<24> {
        LCDCSS24_W::new(self)
    }
    #[doc = "Bit 25 - L25 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss25(&mut self) -> LCDCSS25_W<25> {
        LCDCSS25_W::new(self)
    }
    #[doc = "Bit 26 - L26 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss26(&mut self) -> LCDCSS26_W<26> {
        LCDCSS26_W::new(self)
    }
    #[doc = "Bit 27 - L27 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss27(&mut self) -> LCDCSS27_W<27> {
        LCDCSS27_W::new(self)
    }
    #[doc = "Bit 28 - L28 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss28(&mut self) -> LCDCSS28_W<28> {
        LCDCSS28_W::new(self)
    }
    #[doc = "Bit 29 - L29 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss29(&mut self) -> LCDCSS29_W<29> {
        LCDCSS29_W::new(self)
    }
    #[doc = "Bit 30 - L30 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss30(&mut self) -> LCDCSS30_W<30> {
        LCDCSS30_W::new(self)
    }
    #[doc = "Bit 31 - L31 Com Seg select"]
    #[inline(always)]
    pub fn lcdcss31(&mut self) -> LCDCSS31_W<31> {
        LCDCSS31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_F COM/SEG select register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdcssel0](index.html) module"]
pub struct LCDCSSEL0_SPEC;
impl crate::RegisterSpec for LCDCSSEL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcdcssel0::R](R) reader structure"]
impl crate::Readable for LCDCSSEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdcssel0::W](W) writer structure"]
impl crate::Writable for LCDCSSEL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDCSSEL0 to value 0"]
impl crate::Resettable for LCDCSSEL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
