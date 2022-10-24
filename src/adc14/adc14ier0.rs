#[doc = "Register `ADC14IER0` reader"]
pub struct R(crate::R<ADC14IER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC14IER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC14IER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC14IER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC14IER0` writer"]
pub struct W(crate::W<ADC14IER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC14IER0_SPEC>;
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
impl From<crate::W<ADC14IER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC14IER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC14IE0` reader - Interrupt enable"]
pub type ADC14IE0_R = crate::BitReader<ADC14IE0_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE0_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE0_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE0_1 = 1,
}
impl From<ADC14IE0_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE0_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE0_A {
        match self.bits {
            false => ADC14IE0_A::ADC14IE0_0,
            true => ADC14IE0_A::ADC14IE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE0_0`"]
    #[inline(always)]
    pub fn is_adc14ie0_0(&self) -> bool {
        *self == ADC14IE0_A::ADC14IE0_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE0_1`"]
    #[inline(always)]
    pub fn is_adc14ie0_1(&self) -> bool {
        *self == ADC14IE0_A::ADC14IE0_1
    }
}
#[doc = "Field `ADC14IE0` writer - Interrupt enable"]
pub type ADC14IE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE0_A, O>;
impl<'a, const O: u8> ADC14IE0_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie0_0(self) -> &'a mut W {
        self.variant(ADC14IE0_A::ADC14IE0_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie0_1(self) -> &'a mut W {
        self.variant(ADC14IE0_A::ADC14IE0_1)
    }
}
#[doc = "Field `ADC14IE1` reader - Interrupt enable"]
pub type ADC14IE1_R = crate::BitReader<ADC14IE1_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE1_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE1_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE1_1 = 1,
}
impl From<ADC14IE1_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE1_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE1_A {
        match self.bits {
            false => ADC14IE1_A::ADC14IE1_0,
            true => ADC14IE1_A::ADC14IE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE1_0`"]
    #[inline(always)]
    pub fn is_adc14ie1_0(&self) -> bool {
        *self == ADC14IE1_A::ADC14IE1_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE1_1`"]
    #[inline(always)]
    pub fn is_adc14ie1_1(&self) -> bool {
        *self == ADC14IE1_A::ADC14IE1_1
    }
}
#[doc = "Field `ADC14IE1` writer - Interrupt enable"]
pub type ADC14IE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE1_A, O>;
impl<'a, const O: u8> ADC14IE1_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie1_0(self) -> &'a mut W {
        self.variant(ADC14IE1_A::ADC14IE1_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie1_1(self) -> &'a mut W {
        self.variant(ADC14IE1_A::ADC14IE1_1)
    }
}
#[doc = "Field `ADC14IE2` reader - Interrupt enable"]
pub type ADC14IE2_R = crate::BitReader<ADC14IE2_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE2_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE2_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE2_1 = 1,
}
impl From<ADC14IE2_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE2_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE2_A {
        match self.bits {
            false => ADC14IE2_A::ADC14IE2_0,
            true => ADC14IE2_A::ADC14IE2_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE2_0`"]
    #[inline(always)]
    pub fn is_adc14ie2_0(&self) -> bool {
        *self == ADC14IE2_A::ADC14IE2_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE2_1`"]
    #[inline(always)]
    pub fn is_adc14ie2_1(&self) -> bool {
        *self == ADC14IE2_A::ADC14IE2_1
    }
}
#[doc = "Field `ADC14IE2` writer - Interrupt enable"]
pub type ADC14IE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE2_A, O>;
impl<'a, const O: u8> ADC14IE2_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie2_0(self) -> &'a mut W {
        self.variant(ADC14IE2_A::ADC14IE2_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie2_1(self) -> &'a mut W {
        self.variant(ADC14IE2_A::ADC14IE2_1)
    }
}
#[doc = "Field `ADC14IE3` reader - Interrupt enable"]
pub type ADC14IE3_R = crate::BitReader<ADC14IE3_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE3_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE3_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE3_1 = 1,
}
impl From<ADC14IE3_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE3_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE3_A {
        match self.bits {
            false => ADC14IE3_A::ADC14IE3_0,
            true => ADC14IE3_A::ADC14IE3_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE3_0`"]
    #[inline(always)]
    pub fn is_adc14ie3_0(&self) -> bool {
        *self == ADC14IE3_A::ADC14IE3_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE3_1`"]
    #[inline(always)]
    pub fn is_adc14ie3_1(&self) -> bool {
        *self == ADC14IE3_A::ADC14IE3_1
    }
}
#[doc = "Field `ADC14IE3` writer - Interrupt enable"]
pub type ADC14IE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE3_A, O>;
impl<'a, const O: u8> ADC14IE3_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie3_0(self) -> &'a mut W {
        self.variant(ADC14IE3_A::ADC14IE3_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie3_1(self) -> &'a mut W {
        self.variant(ADC14IE3_A::ADC14IE3_1)
    }
}
#[doc = "Field `ADC14IE4` reader - Interrupt enable"]
pub type ADC14IE4_R = crate::BitReader<ADC14IE4_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE4_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE4_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE4_1 = 1,
}
impl From<ADC14IE4_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE4_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE4_A {
        match self.bits {
            false => ADC14IE4_A::ADC14IE4_0,
            true => ADC14IE4_A::ADC14IE4_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE4_0`"]
    #[inline(always)]
    pub fn is_adc14ie4_0(&self) -> bool {
        *self == ADC14IE4_A::ADC14IE4_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE4_1`"]
    #[inline(always)]
    pub fn is_adc14ie4_1(&self) -> bool {
        *self == ADC14IE4_A::ADC14IE4_1
    }
}
#[doc = "Field `ADC14IE4` writer - Interrupt enable"]
pub type ADC14IE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE4_A, O>;
impl<'a, const O: u8> ADC14IE4_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie4_0(self) -> &'a mut W {
        self.variant(ADC14IE4_A::ADC14IE4_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie4_1(self) -> &'a mut W {
        self.variant(ADC14IE4_A::ADC14IE4_1)
    }
}
#[doc = "Field `ADC14IE5` reader - Interrupt enable"]
pub type ADC14IE5_R = crate::BitReader<ADC14IE5_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE5_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE5_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE5_1 = 1,
}
impl From<ADC14IE5_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE5_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE5_A {
        match self.bits {
            false => ADC14IE5_A::ADC14IE5_0,
            true => ADC14IE5_A::ADC14IE5_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE5_0`"]
    #[inline(always)]
    pub fn is_adc14ie5_0(&self) -> bool {
        *self == ADC14IE5_A::ADC14IE5_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE5_1`"]
    #[inline(always)]
    pub fn is_adc14ie5_1(&self) -> bool {
        *self == ADC14IE5_A::ADC14IE5_1
    }
}
#[doc = "Field `ADC14IE5` writer - Interrupt enable"]
pub type ADC14IE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE5_A, O>;
impl<'a, const O: u8> ADC14IE5_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie5_0(self) -> &'a mut W {
        self.variant(ADC14IE5_A::ADC14IE5_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie5_1(self) -> &'a mut W {
        self.variant(ADC14IE5_A::ADC14IE5_1)
    }
}
#[doc = "Field `ADC14IE6` reader - Interrupt enable"]
pub type ADC14IE6_R = crate::BitReader<ADC14IE6_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE6_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE6_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE6_1 = 1,
}
impl From<ADC14IE6_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE6_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE6_A {
        match self.bits {
            false => ADC14IE6_A::ADC14IE6_0,
            true => ADC14IE6_A::ADC14IE6_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE6_0`"]
    #[inline(always)]
    pub fn is_adc14ie6_0(&self) -> bool {
        *self == ADC14IE6_A::ADC14IE6_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE6_1`"]
    #[inline(always)]
    pub fn is_adc14ie6_1(&self) -> bool {
        *self == ADC14IE6_A::ADC14IE6_1
    }
}
#[doc = "Field `ADC14IE6` writer - Interrupt enable"]
pub type ADC14IE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE6_A, O>;
impl<'a, const O: u8> ADC14IE6_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie6_0(self) -> &'a mut W {
        self.variant(ADC14IE6_A::ADC14IE6_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie6_1(self) -> &'a mut W {
        self.variant(ADC14IE6_A::ADC14IE6_1)
    }
}
#[doc = "Field `ADC14IE7` reader - Interrupt enable"]
pub type ADC14IE7_R = crate::BitReader<ADC14IE7_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE7_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE7_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE7_1 = 1,
}
impl From<ADC14IE7_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE7_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE7_A {
        match self.bits {
            false => ADC14IE7_A::ADC14IE7_0,
            true => ADC14IE7_A::ADC14IE7_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE7_0`"]
    #[inline(always)]
    pub fn is_adc14ie7_0(&self) -> bool {
        *self == ADC14IE7_A::ADC14IE7_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE7_1`"]
    #[inline(always)]
    pub fn is_adc14ie7_1(&self) -> bool {
        *self == ADC14IE7_A::ADC14IE7_1
    }
}
#[doc = "Field `ADC14IE7` writer - Interrupt enable"]
pub type ADC14IE7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE7_A, O>;
impl<'a, const O: u8> ADC14IE7_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie7_0(self) -> &'a mut W {
        self.variant(ADC14IE7_A::ADC14IE7_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie7_1(self) -> &'a mut W {
        self.variant(ADC14IE7_A::ADC14IE7_1)
    }
}
#[doc = "Field `ADC14IE8` reader - Interrupt enable"]
pub type ADC14IE8_R = crate::BitReader<ADC14IE8_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE8_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE8_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE8_1 = 1,
}
impl From<ADC14IE8_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE8_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE8_A {
        match self.bits {
            false => ADC14IE8_A::ADC14IE8_0,
            true => ADC14IE8_A::ADC14IE8_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE8_0`"]
    #[inline(always)]
    pub fn is_adc14ie8_0(&self) -> bool {
        *self == ADC14IE8_A::ADC14IE8_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE8_1`"]
    #[inline(always)]
    pub fn is_adc14ie8_1(&self) -> bool {
        *self == ADC14IE8_A::ADC14IE8_1
    }
}
#[doc = "Field `ADC14IE8` writer - Interrupt enable"]
pub type ADC14IE8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE8_A, O>;
impl<'a, const O: u8> ADC14IE8_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie8_0(self) -> &'a mut W {
        self.variant(ADC14IE8_A::ADC14IE8_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie8_1(self) -> &'a mut W {
        self.variant(ADC14IE8_A::ADC14IE8_1)
    }
}
#[doc = "Field `ADC14IE9` reader - Interrupt enable"]
pub type ADC14IE9_R = crate::BitReader<ADC14IE9_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE9_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE9_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE9_1 = 1,
}
impl From<ADC14IE9_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE9_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE9_A {
        match self.bits {
            false => ADC14IE9_A::ADC14IE9_0,
            true => ADC14IE9_A::ADC14IE9_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE9_0`"]
    #[inline(always)]
    pub fn is_adc14ie9_0(&self) -> bool {
        *self == ADC14IE9_A::ADC14IE9_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE9_1`"]
    #[inline(always)]
    pub fn is_adc14ie9_1(&self) -> bool {
        *self == ADC14IE9_A::ADC14IE9_1
    }
}
#[doc = "Field `ADC14IE9` writer - Interrupt enable"]
pub type ADC14IE9_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE9_A, O>;
impl<'a, const O: u8> ADC14IE9_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie9_0(self) -> &'a mut W {
        self.variant(ADC14IE9_A::ADC14IE9_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie9_1(self) -> &'a mut W {
        self.variant(ADC14IE9_A::ADC14IE9_1)
    }
}
#[doc = "Field `ADC14IE10` reader - Interrupt enable"]
pub type ADC14IE10_R = crate::BitReader<ADC14IE10_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE10_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE10_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE10_1 = 1,
}
impl From<ADC14IE10_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE10_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE10_A {
        match self.bits {
            false => ADC14IE10_A::ADC14IE10_0,
            true => ADC14IE10_A::ADC14IE10_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE10_0`"]
    #[inline(always)]
    pub fn is_adc14ie10_0(&self) -> bool {
        *self == ADC14IE10_A::ADC14IE10_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE10_1`"]
    #[inline(always)]
    pub fn is_adc14ie10_1(&self) -> bool {
        *self == ADC14IE10_A::ADC14IE10_1
    }
}
#[doc = "Field `ADC14IE10` writer - Interrupt enable"]
pub type ADC14IE10_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE10_A, O>;
impl<'a, const O: u8> ADC14IE10_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie10_0(self) -> &'a mut W {
        self.variant(ADC14IE10_A::ADC14IE10_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie10_1(self) -> &'a mut W {
        self.variant(ADC14IE10_A::ADC14IE10_1)
    }
}
#[doc = "Field `ADC14IE11` reader - Interrupt enable"]
pub type ADC14IE11_R = crate::BitReader<ADC14IE11_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE11_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE11_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE11_1 = 1,
}
impl From<ADC14IE11_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE11_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE11_A {
        match self.bits {
            false => ADC14IE11_A::ADC14IE11_0,
            true => ADC14IE11_A::ADC14IE11_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE11_0`"]
    #[inline(always)]
    pub fn is_adc14ie11_0(&self) -> bool {
        *self == ADC14IE11_A::ADC14IE11_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE11_1`"]
    #[inline(always)]
    pub fn is_adc14ie11_1(&self) -> bool {
        *self == ADC14IE11_A::ADC14IE11_1
    }
}
#[doc = "Field `ADC14IE11` writer - Interrupt enable"]
pub type ADC14IE11_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE11_A, O>;
impl<'a, const O: u8> ADC14IE11_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie11_0(self) -> &'a mut W {
        self.variant(ADC14IE11_A::ADC14IE11_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie11_1(self) -> &'a mut W {
        self.variant(ADC14IE11_A::ADC14IE11_1)
    }
}
#[doc = "Field `ADC14IE12` reader - Interrupt enable"]
pub type ADC14IE12_R = crate::BitReader<ADC14IE12_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE12_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE12_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE12_1 = 1,
}
impl From<ADC14IE12_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE12_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE12_A {
        match self.bits {
            false => ADC14IE12_A::ADC14IE12_0,
            true => ADC14IE12_A::ADC14IE12_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE12_0`"]
    #[inline(always)]
    pub fn is_adc14ie12_0(&self) -> bool {
        *self == ADC14IE12_A::ADC14IE12_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE12_1`"]
    #[inline(always)]
    pub fn is_adc14ie12_1(&self) -> bool {
        *self == ADC14IE12_A::ADC14IE12_1
    }
}
#[doc = "Field `ADC14IE12` writer - Interrupt enable"]
pub type ADC14IE12_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE12_A, O>;
impl<'a, const O: u8> ADC14IE12_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie12_0(self) -> &'a mut W {
        self.variant(ADC14IE12_A::ADC14IE12_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie12_1(self) -> &'a mut W {
        self.variant(ADC14IE12_A::ADC14IE12_1)
    }
}
#[doc = "Field `ADC14IE13` reader - Interrupt enable"]
pub type ADC14IE13_R = crate::BitReader<ADC14IE13_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE13_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE13_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE13_1 = 1,
}
impl From<ADC14IE13_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE13_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE13_A {
        match self.bits {
            false => ADC14IE13_A::ADC14IE13_0,
            true => ADC14IE13_A::ADC14IE13_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE13_0`"]
    #[inline(always)]
    pub fn is_adc14ie13_0(&self) -> bool {
        *self == ADC14IE13_A::ADC14IE13_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE13_1`"]
    #[inline(always)]
    pub fn is_adc14ie13_1(&self) -> bool {
        *self == ADC14IE13_A::ADC14IE13_1
    }
}
#[doc = "Field `ADC14IE13` writer - Interrupt enable"]
pub type ADC14IE13_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE13_A, O>;
impl<'a, const O: u8> ADC14IE13_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie13_0(self) -> &'a mut W {
        self.variant(ADC14IE13_A::ADC14IE13_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie13_1(self) -> &'a mut W {
        self.variant(ADC14IE13_A::ADC14IE13_1)
    }
}
#[doc = "Field `ADC14IE14` reader - Interrupt enable"]
pub type ADC14IE14_R = crate::BitReader<ADC14IE14_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE14_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE14_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE14_1 = 1,
}
impl From<ADC14IE14_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE14_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE14_A {
        match self.bits {
            false => ADC14IE14_A::ADC14IE14_0,
            true => ADC14IE14_A::ADC14IE14_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE14_0`"]
    #[inline(always)]
    pub fn is_adc14ie14_0(&self) -> bool {
        *self == ADC14IE14_A::ADC14IE14_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE14_1`"]
    #[inline(always)]
    pub fn is_adc14ie14_1(&self) -> bool {
        *self == ADC14IE14_A::ADC14IE14_1
    }
}
#[doc = "Field `ADC14IE14` writer - Interrupt enable"]
pub type ADC14IE14_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE14_A, O>;
impl<'a, const O: u8> ADC14IE14_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie14_0(self) -> &'a mut W {
        self.variant(ADC14IE14_A::ADC14IE14_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie14_1(self) -> &'a mut W {
        self.variant(ADC14IE14_A::ADC14IE14_1)
    }
}
#[doc = "Field `ADC14IE15` reader - Interrupt enable"]
pub type ADC14IE15_R = crate::BitReader<ADC14IE15_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE15_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE15_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE15_1 = 1,
}
impl From<ADC14IE15_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE15_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE15_A {
        match self.bits {
            false => ADC14IE15_A::ADC14IE15_0,
            true => ADC14IE15_A::ADC14IE15_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE15_0`"]
    #[inline(always)]
    pub fn is_adc14ie15_0(&self) -> bool {
        *self == ADC14IE15_A::ADC14IE15_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE15_1`"]
    #[inline(always)]
    pub fn is_adc14ie15_1(&self) -> bool {
        *self == ADC14IE15_A::ADC14IE15_1
    }
}
#[doc = "Field `ADC14IE15` writer - Interrupt enable"]
pub type ADC14IE15_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE15_A, O>;
impl<'a, const O: u8> ADC14IE15_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie15_0(self) -> &'a mut W {
        self.variant(ADC14IE15_A::ADC14IE15_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie15_1(self) -> &'a mut W {
        self.variant(ADC14IE15_A::ADC14IE15_1)
    }
}
#[doc = "Field `ADC14IE16` reader - Interrupt enable"]
pub type ADC14IE16_R = crate::BitReader<ADC14IE16_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE16_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE16_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE16_1 = 1,
}
impl From<ADC14IE16_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE16_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE16_A {
        match self.bits {
            false => ADC14IE16_A::ADC14IE16_0,
            true => ADC14IE16_A::ADC14IE16_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE16_0`"]
    #[inline(always)]
    pub fn is_adc14ie16_0(&self) -> bool {
        *self == ADC14IE16_A::ADC14IE16_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE16_1`"]
    #[inline(always)]
    pub fn is_adc14ie16_1(&self) -> bool {
        *self == ADC14IE16_A::ADC14IE16_1
    }
}
#[doc = "Field `ADC14IE16` writer - Interrupt enable"]
pub type ADC14IE16_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE16_A, O>;
impl<'a, const O: u8> ADC14IE16_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie16_0(self) -> &'a mut W {
        self.variant(ADC14IE16_A::ADC14IE16_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie16_1(self) -> &'a mut W {
        self.variant(ADC14IE16_A::ADC14IE16_1)
    }
}
#[doc = "Field `ADC14IE17` reader - Interrupt enable"]
pub type ADC14IE17_R = crate::BitReader<ADC14IE17_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE17_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE17_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE17_1 = 1,
}
impl From<ADC14IE17_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE17_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE17_A {
        match self.bits {
            false => ADC14IE17_A::ADC14IE17_0,
            true => ADC14IE17_A::ADC14IE17_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE17_0`"]
    #[inline(always)]
    pub fn is_adc14ie17_0(&self) -> bool {
        *self == ADC14IE17_A::ADC14IE17_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE17_1`"]
    #[inline(always)]
    pub fn is_adc14ie17_1(&self) -> bool {
        *self == ADC14IE17_A::ADC14IE17_1
    }
}
#[doc = "Field `ADC14IE17` writer - Interrupt enable"]
pub type ADC14IE17_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE17_A, O>;
impl<'a, const O: u8> ADC14IE17_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie17_0(self) -> &'a mut W {
        self.variant(ADC14IE17_A::ADC14IE17_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie17_1(self) -> &'a mut W {
        self.variant(ADC14IE17_A::ADC14IE17_1)
    }
}
#[doc = "Field `ADC14IE18` reader - Interrupt enable"]
pub type ADC14IE18_R = crate::BitReader<ADC14IE18_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE18_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE18_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE18_1 = 1,
}
impl From<ADC14IE18_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE18_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE18_A {
        match self.bits {
            false => ADC14IE18_A::ADC14IE18_0,
            true => ADC14IE18_A::ADC14IE18_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE18_0`"]
    #[inline(always)]
    pub fn is_adc14ie18_0(&self) -> bool {
        *self == ADC14IE18_A::ADC14IE18_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE18_1`"]
    #[inline(always)]
    pub fn is_adc14ie18_1(&self) -> bool {
        *self == ADC14IE18_A::ADC14IE18_1
    }
}
#[doc = "Field `ADC14IE18` writer - Interrupt enable"]
pub type ADC14IE18_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE18_A, O>;
impl<'a, const O: u8> ADC14IE18_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie18_0(self) -> &'a mut W {
        self.variant(ADC14IE18_A::ADC14IE18_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie18_1(self) -> &'a mut W {
        self.variant(ADC14IE18_A::ADC14IE18_1)
    }
}
#[doc = "Field `ADC14IE19` reader - Interrupt enable"]
pub type ADC14IE19_R = crate::BitReader<ADC14IE19_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE19_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE19_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE19_1 = 1,
}
impl From<ADC14IE19_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE19_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE19_A {
        match self.bits {
            false => ADC14IE19_A::ADC14IE19_0,
            true => ADC14IE19_A::ADC14IE19_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE19_0`"]
    #[inline(always)]
    pub fn is_adc14ie19_0(&self) -> bool {
        *self == ADC14IE19_A::ADC14IE19_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE19_1`"]
    #[inline(always)]
    pub fn is_adc14ie19_1(&self) -> bool {
        *self == ADC14IE19_A::ADC14IE19_1
    }
}
#[doc = "Field `ADC14IE19` writer - Interrupt enable"]
pub type ADC14IE19_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE19_A, O>;
impl<'a, const O: u8> ADC14IE19_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie19_0(self) -> &'a mut W {
        self.variant(ADC14IE19_A::ADC14IE19_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie19_1(self) -> &'a mut W {
        self.variant(ADC14IE19_A::ADC14IE19_1)
    }
}
#[doc = "Field `ADC14IE20` reader - Interrupt enable"]
pub type ADC14IE20_R = crate::BitReader<ADC14IE20_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE20_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE20_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE20_1 = 1,
}
impl From<ADC14IE20_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE20_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE20_A {
        match self.bits {
            false => ADC14IE20_A::ADC14IE20_0,
            true => ADC14IE20_A::ADC14IE20_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE20_0`"]
    #[inline(always)]
    pub fn is_adc14ie20_0(&self) -> bool {
        *self == ADC14IE20_A::ADC14IE20_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE20_1`"]
    #[inline(always)]
    pub fn is_adc14ie20_1(&self) -> bool {
        *self == ADC14IE20_A::ADC14IE20_1
    }
}
#[doc = "Field `ADC14IE20` writer - Interrupt enable"]
pub type ADC14IE20_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE20_A, O>;
impl<'a, const O: u8> ADC14IE20_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie20_0(self) -> &'a mut W {
        self.variant(ADC14IE20_A::ADC14IE20_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie20_1(self) -> &'a mut W {
        self.variant(ADC14IE20_A::ADC14IE20_1)
    }
}
#[doc = "Field `ADC14IE21` reader - Interrupt enable"]
pub type ADC14IE21_R = crate::BitReader<ADC14IE21_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE21_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE21_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE21_1 = 1,
}
impl From<ADC14IE21_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE21_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE21_A {
        match self.bits {
            false => ADC14IE21_A::ADC14IE21_0,
            true => ADC14IE21_A::ADC14IE21_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE21_0`"]
    #[inline(always)]
    pub fn is_adc14ie21_0(&self) -> bool {
        *self == ADC14IE21_A::ADC14IE21_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE21_1`"]
    #[inline(always)]
    pub fn is_adc14ie21_1(&self) -> bool {
        *self == ADC14IE21_A::ADC14IE21_1
    }
}
#[doc = "Field `ADC14IE21` writer - Interrupt enable"]
pub type ADC14IE21_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE21_A, O>;
impl<'a, const O: u8> ADC14IE21_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie21_0(self) -> &'a mut W {
        self.variant(ADC14IE21_A::ADC14IE21_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie21_1(self) -> &'a mut W {
        self.variant(ADC14IE21_A::ADC14IE21_1)
    }
}
#[doc = "Field `ADC14IE22` reader - Interrupt enable"]
pub type ADC14IE22_R = crate::BitReader<ADC14IE22_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE22_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE22_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE22_1 = 1,
}
impl From<ADC14IE22_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE22_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE22_A {
        match self.bits {
            false => ADC14IE22_A::ADC14IE22_0,
            true => ADC14IE22_A::ADC14IE22_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE22_0`"]
    #[inline(always)]
    pub fn is_adc14ie22_0(&self) -> bool {
        *self == ADC14IE22_A::ADC14IE22_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE22_1`"]
    #[inline(always)]
    pub fn is_adc14ie22_1(&self) -> bool {
        *self == ADC14IE22_A::ADC14IE22_1
    }
}
#[doc = "Field `ADC14IE22` writer - Interrupt enable"]
pub type ADC14IE22_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE22_A, O>;
impl<'a, const O: u8> ADC14IE22_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie22_0(self) -> &'a mut W {
        self.variant(ADC14IE22_A::ADC14IE22_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie22_1(self) -> &'a mut W {
        self.variant(ADC14IE22_A::ADC14IE22_1)
    }
}
#[doc = "Field `ADC14IE23` reader - Interrupt enable"]
pub type ADC14IE23_R = crate::BitReader<ADC14IE23_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE23_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE23_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE23_1 = 1,
}
impl From<ADC14IE23_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE23_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE23_A {
        match self.bits {
            false => ADC14IE23_A::ADC14IE23_0,
            true => ADC14IE23_A::ADC14IE23_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE23_0`"]
    #[inline(always)]
    pub fn is_adc14ie23_0(&self) -> bool {
        *self == ADC14IE23_A::ADC14IE23_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE23_1`"]
    #[inline(always)]
    pub fn is_adc14ie23_1(&self) -> bool {
        *self == ADC14IE23_A::ADC14IE23_1
    }
}
#[doc = "Field `ADC14IE23` writer - Interrupt enable"]
pub type ADC14IE23_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE23_A, O>;
impl<'a, const O: u8> ADC14IE23_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie23_0(self) -> &'a mut W {
        self.variant(ADC14IE23_A::ADC14IE23_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie23_1(self) -> &'a mut W {
        self.variant(ADC14IE23_A::ADC14IE23_1)
    }
}
#[doc = "Field `ADC14IE24` reader - Interrupt enable"]
pub type ADC14IE24_R = crate::BitReader<ADC14IE24_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE24_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE24_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE24_1 = 1,
}
impl From<ADC14IE24_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE24_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE24_A {
        match self.bits {
            false => ADC14IE24_A::ADC14IE24_0,
            true => ADC14IE24_A::ADC14IE24_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE24_0`"]
    #[inline(always)]
    pub fn is_adc14ie24_0(&self) -> bool {
        *self == ADC14IE24_A::ADC14IE24_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE24_1`"]
    #[inline(always)]
    pub fn is_adc14ie24_1(&self) -> bool {
        *self == ADC14IE24_A::ADC14IE24_1
    }
}
#[doc = "Field `ADC14IE24` writer - Interrupt enable"]
pub type ADC14IE24_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE24_A, O>;
impl<'a, const O: u8> ADC14IE24_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie24_0(self) -> &'a mut W {
        self.variant(ADC14IE24_A::ADC14IE24_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie24_1(self) -> &'a mut W {
        self.variant(ADC14IE24_A::ADC14IE24_1)
    }
}
#[doc = "Field `ADC14IE25` reader - Interrupt enable"]
pub type ADC14IE25_R = crate::BitReader<ADC14IE25_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE25_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE25_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE25_1 = 1,
}
impl From<ADC14IE25_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE25_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE25_A {
        match self.bits {
            false => ADC14IE25_A::ADC14IE25_0,
            true => ADC14IE25_A::ADC14IE25_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE25_0`"]
    #[inline(always)]
    pub fn is_adc14ie25_0(&self) -> bool {
        *self == ADC14IE25_A::ADC14IE25_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE25_1`"]
    #[inline(always)]
    pub fn is_adc14ie25_1(&self) -> bool {
        *self == ADC14IE25_A::ADC14IE25_1
    }
}
#[doc = "Field `ADC14IE25` writer - Interrupt enable"]
pub type ADC14IE25_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE25_A, O>;
impl<'a, const O: u8> ADC14IE25_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie25_0(self) -> &'a mut W {
        self.variant(ADC14IE25_A::ADC14IE25_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie25_1(self) -> &'a mut W {
        self.variant(ADC14IE25_A::ADC14IE25_1)
    }
}
#[doc = "Field `ADC14IE26` reader - Interrupt enable"]
pub type ADC14IE26_R = crate::BitReader<ADC14IE26_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE26_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE26_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE26_1 = 1,
}
impl From<ADC14IE26_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE26_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE26_A {
        match self.bits {
            false => ADC14IE26_A::ADC14IE26_0,
            true => ADC14IE26_A::ADC14IE26_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE26_0`"]
    #[inline(always)]
    pub fn is_adc14ie26_0(&self) -> bool {
        *self == ADC14IE26_A::ADC14IE26_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE26_1`"]
    #[inline(always)]
    pub fn is_adc14ie26_1(&self) -> bool {
        *self == ADC14IE26_A::ADC14IE26_1
    }
}
#[doc = "Field `ADC14IE26` writer - Interrupt enable"]
pub type ADC14IE26_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE26_A, O>;
impl<'a, const O: u8> ADC14IE26_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie26_0(self) -> &'a mut W {
        self.variant(ADC14IE26_A::ADC14IE26_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie26_1(self) -> &'a mut W {
        self.variant(ADC14IE26_A::ADC14IE26_1)
    }
}
#[doc = "Field `ADC14IE27` reader - Interrupt enable"]
pub type ADC14IE27_R = crate::BitReader<ADC14IE27_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE27_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE27_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE27_1 = 1,
}
impl From<ADC14IE27_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE27_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE27_A {
        match self.bits {
            false => ADC14IE27_A::ADC14IE27_0,
            true => ADC14IE27_A::ADC14IE27_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE27_0`"]
    #[inline(always)]
    pub fn is_adc14ie27_0(&self) -> bool {
        *self == ADC14IE27_A::ADC14IE27_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE27_1`"]
    #[inline(always)]
    pub fn is_adc14ie27_1(&self) -> bool {
        *self == ADC14IE27_A::ADC14IE27_1
    }
}
#[doc = "Field `ADC14IE27` writer - Interrupt enable"]
pub type ADC14IE27_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE27_A, O>;
impl<'a, const O: u8> ADC14IE27_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie27_0(self) -> &'a mut W {
        self.variant(ADC14IE27_A::ADC14IE27_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie27_1(self) -> &'a mut W {
        self.variant(ADC14IE27_A::ADC14IE27_1)
    }
}
#[doc = "Field `ADC14IE28` reader - Interrupt enable"]
pub type ADC14IE28_R = crate::BitReader<ADC14IE28_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE28_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE28_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE28_1 = 1,
}
impl From<ADC14IE28_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE28_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE28_A {
        match self.bits {
            false => ADC14IE28_A::ADC14IE28_0,
            true => ADC14IE28_A::ADC14IE28_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE28_0`"]
    #[inline(always)]
    pub fn is_adc14ie28_0(&self) -> bool {
        *self == ADC14IE28_A::ADC14IE28_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE28_1`"]
    #[inline(always)]
    pub fn is_adc14ie28_1(&self) -> bool {
        *self == ADC14IE28_A::ADC14IE28_1
    }
}
#[doc = "Field `ADC14IE28` writer - Interrupt enable"]
pub type ADC14IE28_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE28_A, O>;
impl<'a, const O: u8> ADC14IE28_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie28_0(self) -> &'a mut W {
        self.variant(ADC14IE28_A::ADC14IE28_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie28_1(self) -> &'a mut W {
        self.variant(ADC14IE28_A::ADC14IE28_1)
    }
}
#[doc = "Field `ADC14IE29` reader - Interrupt enable"]
pub type ADC14IE29_R = crate::BitReader<ADC14IE29_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE29_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE29_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE29_1 = 1,
}
impl From<ADC14IE29_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE29_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE29_A {
        match self.bits {
            false => ADC14IE29_A::ADC14IE29_0,
            true => ADC14IE29_A::ADC14IE29_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE29_0`"]
    #[inline(always)]
    pub fn is_adc14ie29_0(&self) -> bool {
        *self == ADC14IE29_A::ADC14IE29_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE29_1`"]
    #[inline(always)]
    pub fn is_adc14ie29_1(&self) -> bool {
        *self == ADC14IE29_A::ADC14IE29_1
    }
}
#[doc = "Field `ADC14IE29` writer - Interrupt enable"]
pub type ADC14IE29_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE29_A, O>;
impl<'a, const O: u8> ADC14IE29_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie29_0(self) -> &'a mut W {
        self.variant(ADC14IE29_A::ADC14IE29_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie29_1(self) -> &'a mut W {
        self.variant(ADC14IE29_A::ADC14IE29_1)
    }
}
#[doc = "Field `ADC14IE30` reader - Interrupt enable"]
pub type ADC14IE30_R = crate::BitReader<ADC14IE30_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE30_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE30_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE30_1 = 1,
}
impl From<ADC14IE30_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE30_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE30_A {
        match self.bits {
            false => ADC14IE30_A::ADC14IE30_0,
            true => ADC14IE30_A::ADC14IE30_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE30_0`"]
    #[inline(always)]
    pub fn is_adc14ie30_0(&self) -> bool {
        *self == ADC14IE30_A::ADC14IE30_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE30_1`"]
    #[inline(always)]
    pub fn is_adc14ie30_1(&self) -> bool {
        *self == ADC14IE30_A::ADC14IE30_1
    }
}
#[doc = "Field `ADC14IE30` writer - Interrupt enable"]
pub type ADC14IE30_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE30_A, O>;
impl<'a, const O: u8> ADC14IE30_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie30_0(self) -> &'a mut W {
        self.variant(ADC14IE30_A::ADC14IE30_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie30_1(self) -> &'a mut W {
        self.variant(ADC14IE30_A::ADC14IE30_1)
    }
}
#[doc = "Field `ADC14IE31` reader - Interrupt enable"]
pub type ADC14IE31_R = crate::BitReader<ADC14IE31_A>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC14IE31_A {
    #[doc = "0: Interrupt disabled"]
    ADC14IE31_0 = 0,
    #[doc = "1: Interrupt enabled"]
    ADC14IE31_1 = 1,
}
impl From<ADC14IE31_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14IE31_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC14IE31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14IE31_A {
        match self.bits {
            false => ADC14IE31_A::ADC14IE31_0,
            true => ADC14IE31_A::ADC14IE31_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14IE31_0`"]
    #[inline(always)]
    pub fn is_adc14ie31_0(&self) -> bool {
        *self == ADC14IE31_A::ADC14IE31_0
    }
    #[doc = "Checks if the value of the field is `ADC14IE31_1`"]
    #[inline(always)]
    pub fn is_adc14ie31_1(&self) -> bool {
        *self == ADC14IE31_A::ADC14IE31_1
    }
}
#[doc = "Field `ADC14IE31` writer - Interrupt enable"]
pub type ADC14IE31_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC14IER0_SPEC, ADC14IE31_A, O>;
impl<'a, const O: u8> ADC14IE31_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn adc14ie31_0(self) -> &'a mut W {
        self.variant(ADC14IE31_A::ADC14IE31_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn adc14ie31_1(self) -> &'a mut W {
        self.variant(ADC14IE31_A::ADC14IE31_1)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie0(&self) -> ADC14IE0_R {
        ADC14IE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie1(&self) -> ADC14IE1_R {
        ADC14IE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie2(&self) -> ADC14IE2_R {
        ADC14IE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie3(&self) -> ADC14IE3_R {
        ADC14IE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie4(&self) -> ADC14IE4_R {
        ADC14IE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie5(&self) -> ADC14IE5_R {
        ADC14IE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie6(&self) -> ADC14IE6_R {
        ADC14IE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie7(&self) -> ADC14IE7_R {
        ADC14IE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie8(&self) -> ADC14IE8_R {
        ADC14IE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie9(&self) -> ADC14IE9_R {
        ADC14IE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie10(&self) -> ADC14IE10_R {
        ADC14IE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie11(&self) -> ADC14IE11_R {
        ADC14IE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie12(&self) -> ADC14IE12_R {
        ADC14IE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie13(&self) -> ADC14IE13_R {
        ADC14IE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie14(&self) -> ADC14IE14_R {
        ADC14IE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie15(&self) -> ADC14IE15_R {
        ADC14IE15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie16(&self) -> ADC14IE16_R {
        ADC14IE16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie17(&self) -> ADC14IE17_R {
        ADC14IE17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie18(&self) -> ADC14IE18_R {
        ADC14IE18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie19(&self) -> ADC14IE19_R {
        ADC14IE19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie20(&self) -> ADC14IE20_R {
        ADC14IE20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie21(&self) -> ADC14IE21_R {
        ADC14IE21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie22(&self) -> ADC14IE22_R {
        ADC14IE22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie23(&self) -> ADC14IE23_R {
        ADC14IE23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie24(&self) -> ADC14IE24_R {
        ADC14IE24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie25(&self) -> ADC14IE25_R {
        ADC14IE25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie26(&self) -> ADC14IE26_R {
        ADC14IE26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie27(&self) -> ADC14IE27_R {
        ADC14IE27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie28(&self) -> ADC14IE28_R {
        ADC14IE28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie29(&self) -> ADC14IE29_R {
        ADC14IE29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie30(&self) -> ADC14IE30_R {
        ADC14IE30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie31(&self) -> ADC14IE31_R {
        ADC14IE31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie0(&mut self) -> ADC14IE0_W<0> {
        ADC14IE0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie1(&mut self) -> ADC14IE1_W<1> {
        ADC14IE1_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie2(&mut self) -> ADC14IE2_W<2> {
        ADC14IE2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie3(&mut self) -> ADC14IE3_W<3> {
        ADC14IE3_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie4(&mut self) -> ADC14IE4_W<4> {
        ADC14IE4_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie5(&mut self) -> ADC14IE5_W<5> {
        ADC14IE5_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie6(&mut self) -> ADC14IE6_W<6> {
        ADC14IE6_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie7(&mut self) -> ADC14IE7_W<7> {
        ADC14IE7_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie8(&mut self) -> ADC14IE8_W<8> {
        ADC14IE8_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie9(&mut self) -> ADC14IE9_W<9> {
        ADC14IE9_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie10(&mut self) -> ADC14IE10_W<10> {
        ADC14IE10_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie11(&mut self) -> ADC14IE11_W<11> {
        ADC14IE11_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie12(&mut self) -> ADC14IE12_W<12> {
        ADC14IE12_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie13(&mut self) -> ADC14IE13_W<13> {
        ADC14IE13_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie14(&mut self) -> ADC14IE14_W<14> {
        ADC14IE14_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie15(&mut self) -> ADC14IE15_W<15> {
        ADC14IE15_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie16(&mut self) -> ADC14IE16_W<16> {
        ADC14IE16_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie17(&mut self) -> ADC14IE17_W<17> {
        ADC14IE17_W::new(self)
    }
    #[doc = "Bit 18 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie18(&mut self) -> ADC14IE18_W<18> {
        ADC14IE18_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie19(&mut self) -> ADC14IE19_W<19> {
        ADC14IE19_W::new(self)
    }
    #[doc = "Bit 20 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie20(&mut self) -> ADC14IE20_W<20> {
        ADC14IE20_W::new(self)
    }
    #[doc = "Bit 21 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie21(&mut self) -> ADC14IE21_W<21> {
        ADC14IE21_W::new(self)
    }
    #[doc = "Bit 22 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie22(&mut self) -> ADC14IE22_W<22> {
        ADC14IE22_W::new(self)
    }
    #[doc = "Bit 23 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie23(&mut self) -> ADC14IE23_W<23> {
        ADC14IE23_W::new(self)
    }
    #[doc = "Bit 24 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie24(&mut self) -> ADC14IE24_W<24> {
        ADC14IE24_W::new(self)
    }
    #[doc = "Bit 25 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie25(&mut self) -> ADC14IE25_W<25> {
        ADC14IE25_W::new(self)
    }
    #[doc = "Bit 26 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie26(&mut self) -> ADC14IE26_W<26> {
        ADC14IE26_W::new(self)
    }
    #[doc = "Bit 27 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie27(&mut self) -> ADC14IE27_W<27> {
        ADC14IE27_W::new(self)
    }
    #[doc = "Bit 28 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie28(&mut self) -> ADC14IE28_W<28> {
        ADC14IE28_W::new(self)
    }
    #[doc = "Bit 29 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie29(&mut self) -> ADC14IE29_W<29> {
        ADC14IE29_W::new(self)
    }
    #[doc = "Bit 30 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie30(&mut self) -> ADC14IE30_W<30> {
        ADC14IE30_W::new(self)
    }
    #[doc = "Bit 31 - Interrupt enable"]
    #[inline(always)]
    pub fn adc14ie31(&mut self) -> ADC14IE31_W<31> {
        ADC14IE31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14ier0](index.html) module"]
pub struct ADC14IER0_SPEC;
impl crate::RegisterSpec for ADC14IER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc14ier0::R](R) reader structure"]
impl crate::Readable for ADC14IER0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc14ier0::W](W) writer structure"]
impl crate::Writable for ADC14IER0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC14IER0 to value 0"]
impl crate::Resettable for ADC14IER0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
