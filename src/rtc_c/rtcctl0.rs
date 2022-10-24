#[doc = "Register `RTCCTL0` reader"]
pub struct R(crate::R<RTCCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCCTL0` writer"]
pub struct W(crate::W<RTCCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCCTL0_SPEC>;
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
impl From<crate::W<RTCCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCRDYIFG` reader - Real-time clock ready interrupt flag"]
pub type RTCRDYIFG_R = crate::BitReader<RTCRDYIFG_A>;
#[doc = "Real-time clock ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCRDYIFG_A {
    #[doc = "0: RTC cannot be read safely"]
    RTCRDYIFG_0 = 0,
    #[doc = "1: RTC can be read safely"]
    RTCRDYIFG_1 = 1,
}
impl From<RTCRDYIFG_A> for bool {
    #[inline(always)]
    fn from(variant: RTCRDYIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCRDYIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCRDYIFG_A {
        match self.bits {
            false => RTCRDYIFG_A::RTCRDYIFG_0,
            true => RTCRDYIFG_A::RTCRDYIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCRDYIFG_0`"]
    #[inline(always)]
    pub fn is_rtcrdyifg_0(&self) -> bool {
        *self == RTCRDYIFG_A::RTCRDYIFG_0
    }
    #[doc = "Checks if the value of the field is `RTCRDYIFG_1`"]
    #[inline(always)]
    pub fn is_rtcrdyifg_1(&self) -> bool {
        *self == RTCRDYIFG_A::RTCRDYIFG_1
    }
}
#[doc = "Field `RTCRDYIFG` writer - Real-time clock ready interrupt flag"]
pub type RTCRDYIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCCTL0_SPEC, RTCRDYIFG_A, O>;
impl<'a, const O: u8> RTCRDYIFG_W<'a, O> {
    #[doc = "RTC cannot be read safely"]
    #[inline(always)]
    pub fn rtcrdyifg_0(self) -> &'a mut W {
        self.variant(RTCRDYIFG_A::RTCRDYIFG_0)
    }
    #[doc = "RTC can be read safely"]
    #[inline(always)]
    pub fn rtcrdyifg_1(self) -> &'a mut W {
        self.variant(RTCRDYIFG_A::RTCRDYIFG_1)
    }
}
#[doc = "Field `RTCAIFG` reader - Real-time clock alarm interrupt flag"]
pub type RTCAIFG_R = crate::BitReader<RTCAIFG_A>;
#[doc = "Real-time clock alarm interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCAIFG_A {
    #[doc = "0: No time event occurred"]
    RTCAIFG_0 = 0,
    #[doc = "1: Time event occurred"]
    RTCAIFG_1 = 1,
}
impl From<RTCAIFG_A> for bool {
    #[inline(always)]
    fn from(variant: RTCAIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCAIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCAIFG_A {
        match self.bits {
            false => RTCAIFG_A::RTCAIFG_0,
            true => RTCAIFG_A::RTCAIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCAIFG_0`"]
    #[inline(always)]
    pub fn is_rtcaifg_0(&self) -> bool {
        *self == RTCAIFG_A::RTCAIFG_0
    }
    #[doc = "Checks if the value of the field is `RTCAIFG_1`"]
    #[inline(always)]
    pub fn is_rtcaifg_1(&self) -> bool {
        *self == RTCAIFG_A::RTCAIFG_1
    }
}
#[doc = "Field `RTCAIFG` writer - Real-time clock alarm interrupt flag"]
pub type RTCAIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCCTL0_SPEC, RTCAIFG_A, O>;
impl<'a, const O: u8> RTCAIFG_W<'a, O> {
    #[doc = "No time event occurred"]
    #[inline(always)]
    pub fn rtcaifg_0(self) -> &'a mut W {
        self.variant(RTCAIFG_A::RTCAIFG_0)
    }
    #[doc = "Time event occurred"]
    #[inline(always)]
    pub fn rtcaifg_1(self) -> &'a mut W {
        self.variant(RTCAIFG_A::RTCAIFG_1)
    }
}
#[doc = "Field `RTCTEVIFG` reader - Real-time clock time event interrupt flag"]
pub type RTCTEVIFG_R = crate::BitReader<RTCTEVIFG_A>;
#[doc = "Real-time clock time event interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCTEVIFG_A {
    #[doc = "0: No time event occurred"]
    RTCTEVIFG_0 = 0,
    #[doc = "1: Time event occurred"]
    RTCTEVIFG_1 = 1,
}
impl From<RTCTEVIFG_A> for bool {
    #[inline(always)]
    fn from(variant: RTCTEVIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCTEVIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCTEVIFG_A {
        match self.bits {
            false => RTCTEVIFG_A::RTCTEVIFG_0,
            true => RTCTEVIFG_A::RTCTEVIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCTEVIFG_0`"]
    #[inline(always)]
    pub fn is_rtctevifg_0(&self) -> bool {
        *self == RTCTEVIFG_A::RTCTEVIFG_0
    }
    #[doc = "Checks if the value of the field is `RTCTEVIFG_1`"]
    #[inline(always)]
    pub fn is_rtctevifg_1(&self) -> bool {
        *self == RTCTEVIFG_A::RTCTEVIFG_1
    }
}
#[doc = "Field `RTCTEVIFG` writer - Real-time clock time event interrupt flag"]
pub type RTCTEVIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCCTL0_SPEC, RTCTEVIFG_A, O>;
impl<'a, const O: u8> RTCTEVIFG_W<'a, O> {
    #[doc = "No time event occurred"]
    #[inline(always)]
    pub fn rtctevifg_0(self) -> &'a mut W {
        self.variant(RTCTEVIFG_A::RTCTEVIFG_0)
    }
    #[doc = "Time event occurred"]
    #[inline(always)]
    pub fn rtctevifg_1(self) -> &'a mut W {
        self.variant(RTCTEVIFG_A::RTCTEVIFG_1)
    }
}
#[doc = "Field `RTCOFIFG` reader - 32-kHz crystal oscillator fault interrupt flag"]
pub type RTCOFIFG_R = crate::BitReader<RTCOFIFG_A>;
#[doc = "32-kHz crystal oscillator fault interrupt flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCOFIFG_A {
    #[doc = "0: No interrupt pending"]
    RTCOFIFG_0 = 0,
    #[doc = "1: Interrupt pending. A 32-kHz crystal oscillator fault occurred after last reset."]
    RTCOFIFG_1 = 1,
}
impl From<RTCOFIFG_A> for bool {
    #[inline(always)]
    fn from(variant: RTCOFIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCOFIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCOFIFG_A {
        match self.bits {
            false => RTCOFIFG_A::RTCOFIFG_0,
            true => RTCOFIFG_A::RTCOFIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCOFIFG_0`"]
    #[inline(always)]
    pub fn is_rtcofifg_0(&self) -> bool {
        *self == RTCOFIFG_A::RTCOFIFG_0
    }
    #[doc = "Checks if the value of the field is `RTCOFIFG_1`"]
    #[inline(always)]
    pub fn is_rtcofifg_1(&self) -> bool {
        *self == RTCOFIFG_A::RTCOFIFG_1
    }
}
#[doc = "Field `RTCOFIFG` writer - 32-kHz crystal oscillator fault interrupt flag"]
pub type RTCOFIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCCTL0_SPEC, RTCOFIFG_A, O>;
impl<'a, const O: u8> RTCOFIFG_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn rtcofifg_0(self) -> &'a mut W {
        self.variant(RTCOFIFG_A::RTCOFIFG_0)
    }
    #[doc = "Interrupt pending. A 32-kHz crystal oscillator fault occurred after last reset."]
    #[inline(always)]
    pub fn rtcofifg_1(self) -> &'a mut W {
        self.variant(RTCOFIFG_A::RTCOFIFG_1)
    }
}
#[doc = "Field `RTCRDYIE` reader - Real-time clock ready interrupt enable"]
pub type RTCRDYIE_R = crate::BitReader<RTCRDYIE_A>;
#[doc = "Real-time clock ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCRDYIE_A {
    #[doc = "0: Interrupt not enabled"]
    RTCRDYIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    RTCRDYIE_1 = 1,
}
impl From<RTCRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: RTCRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCRDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCRDYIE_A {
        match self.bits {
            false => RTCRDYIE_A::RTCRDYIE_0,
            true => RTCRDYIE_A::RTCRDYIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCRDYIE_0`"]
    #[inline(always)]
    pub fn is_rtcrdyie_0(&self) -> bool {
        *self == RTCRDYIE_A::RTCRDYIE_0
    }
    #[doc = "Checks if the value of the field is `RTCRDYIE_1`"]
    #[inline(always)]
    pub fn is_rtcrdyie_1(&self) -> bool {
        *self == RTCRDYIE_A::RTCRDYIE_1
    }
}
#[doc = "Field `RTCRDYIE` writer - Real-time clock ready interrupt enable"]
pub type RTCRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCCTL0_SPEC, RTCRDYIE_A, O>;
impl<'a, const O: u8> RTCRDYIE_W<'a, O> {
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn rtcrdyie_0(self) -> &'a mut W {
        self.variant(RTCRDYIE_A::RTCRDYIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn rtcrdyie_1(self) -> &'a mut W {
        self.variant(RTCRDYIE_A::RTCRDYIE_1)
    }
}
#[doc = "Field `RTCAIE` reader - Real-time clock alarm interrupt enable"]
pub type RTCAIE_R = crate::BitReader<RTCAIE_A>;
#[doc = "Real-time clock alarm interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCAIE_A {
    #[doc = "0: Interrupt not enabled"]
    RTCAIE_0 = 0,
    #[doc = "1: Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    RTCAIE_1 = 1,
}
impl From<RTCAIE_A> for bool {
    #[inline(always)]
    fn from(variant: RTCAIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCAIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCAIE_A {
        match self.bits {
            false => RTCAIE_A::RTCAIE_0,
            true => RTCAIE_A::RTCAIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCAIE_0`"]
    #[inline(always)]
    pub fn is_rtcaie_0(&self) -> bool {
        *self == RTCAIE_A::RTCAIE_0
    }
    #[doc = "Checks if the value of the field is `RTCAIE_1`"]
    #[inline(always)]
    pub fn is_rtcaie_1(&self) -> bool {
        *self == RTCAIE_A::RTCAIE_1
    }
}
#[doc = "Field `RTCAIE` writer - Real-time clock alarm interrupt enable"]
pub type RTCAIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCCTL0_SPEC, RTCAIE_A, O>;
impl<'a, const O: u8> RTCAIE_W<'a, O> {
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn rtcaie_0(self) -> &'a mut W {
        self.variant(RTCAIE_A::RTCAIE_0)
    }
    #[doc = "Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    #[inline(always)]
    pub fn rtcaie_1(self) -> &'a mut W {
        self.variant(RTCAIE_A::RTCAIE_1)
    }
}
#[doc = "Field `RTCTEVIE` reader - Real-time clock time event interrupt enable"]
pub type RTCTEVIE_R = crate::BitReader<RTCTEVIE_A>;
#[doc = "Real-time clock time event interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCTEVIE_A {
    #[doc = "0: Interrupt not enabled"]
    RTCTEVIE_0 = 0,
    #[doc = "1: Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    RTCTEVIE_1 = 1,
}
impl From<RTCTEVIE_A> for bool {
    #[inline(always)]
    fn from(variant: RTCTEVIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCTEVIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCTEVIE_A {
        match self.bits {
            false => RTCTEVIE_A::RTCTEVIE_0,
            true => RTCTEVIE_A::RTCTEVIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCTEVIE_0`"]
    #[inline(always)]
    pub fn is_rtctevie_0(&self) -> bool {
        *self == RTCTEVIE_A::RTCTEVIE_0
    }
    #[doc = "Checks if the value of the field is `RTCTEVIE_1`"]
    #[inline(always)]
    pub fn is_rtctevie_1(&self) -> bool {
        *self == RTCTEVIE_A::RTCTEVIE_1
    }
}
#[doc = "Field `RTCTEVIE` writer - Real-time clock time event interrupt enable"]
pub type RTCTEVIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCCTL0_SPEC, RTCTEVIE_A, O>;
impl<'a, const O: u8> RTCTEVIE_W<'a, O> {
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn rtctevie_0(self) -> &'a mut W {
        self.variant(RTCTEVIE_A::RTCTEVIE_0)
    }
    #[doc = "Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    #[inline(always)]
    pub fn rtctevie_1(self) -> &'a mut W {
        self.variant(RTCTEVIE_A::RTCTEVIE_1)
    }
}
#[doc = "Field `RTCOFIE` reader - 32-kHz crystal oscillator fault interrupt enable"]
pub type RTCOFIE_R = crate::BitReader<RTCOFIE_A>;
#[doc = "32-kHz crystal oscillator fault interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCOFIE_A {
    #[doc = "0: Interrupt not enabled"]
    RTCOFIE_0 = 0,
    #[doc = "1: Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    RTCOFIE_1 = 1,
}
impl From<RTCOFIE_A> for bool {
    #[inline(always)]
    fn from(variant: RTCOFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCOFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCOFIE_A {
        match self.bits {
            false => RTCOFIE_A::RTCOFIE_0,
            true => RTCOFIE_A::RTCOFIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCOFIE_0`"]
    #[inline(always)]
    pub fn is_rtcofie_0(&self) -> bool {
        *self == RTCOFIE_A::RTCOFIE_0
    }
    #[doc = "Checks if the value of the field is `RTCOFIE_1`"]
    #[inline(always)]
    pub fn is_rtcofie_1(&self) -> bool {
        *self == RTCOFIE_A::RTCOFIE_1
    }
}
#[doc = "Field `RTCOFIE` writer - 32-kHz crystal oscillator fault interrupt enable"]
pub type RTCOFIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCCTL0_SPEC, RTCOFIE_A, O>;
impl<'a, const O: u8> RTCOFIE_W<'a, O> {
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn rtcofie_0(self) -> &'a mut W {
        self.variant(RTCOFIE_A::RTCOFIE_0)
    }
    #[doc = "Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    #[inline(always)]
    pub fn rtcofie_1(self) -> &'a mut W {
        self.variant(RTCOFIE_A::RTCOFIE_1)
    }
}
#[doc = "Field `RTCKEY` reader - Real-time clock key"]
pub type RTCKEY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTCKEY` writer - Real-time clock key"]
pub type RTCKEY_W<'a, const O: u8> = crate::FieldWriter<'a, u16, RTCCTL0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Real-time clock ready interrupt flag"]
    #[inline(always)]
    pub fn rtcrdyifg(&self) -> RTCRDYIFG_R {
        RTCRDYIFG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Real-time clock alarm interrupt flag"]
    #[inline(always)]
    pub fn rtcaifg(&self) -> RTCAIFG_R {
        RTCAIFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Real-time clock time event interrupt flag"]
    #[inline(always)]
    pub fn rtctevifg(&self) -> RTCTEVIFG_R {
        RTCTEVIFG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 32-kHz crystal oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn rtcofifg(&self) -> RTCOFIFG_R {
        RTCOFIFG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Real-time clock ready interrupt enable"]
    #[inline(always)]
    pub fn rtcrdyie(&self) -> RTCRDYIE_R {
        RTCRDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Real-time clock alarm interrupt enable"]
    #[inline(always)]
    pub fn rtcaie(&self) -> RTCAIE_R {
        RTCAIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Real-time clock time event interrupt enable"]
    #[inline(always)]
    pub fn rtctevie(&self) -> RTCTEVIE_R {
        RTCTEVIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 32-kHz crystal oscillator fault interrupt enable"]
    #[inline(always)]
    pub fn rtcofie(&self) -> RTCOFIE_R {
        RTCOFIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Real-time clock key"]
    #[inline(always)]
    pub fn rtckey(&self) -> RTCKEY_R {
        RTCKEY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Real-time clock ready interrupt flag"]
    #[inline(always)]
    pub fn rtcrdyifg(&mut self) -> RTCRDYIFG_W<0> {
        RTCRDYIFG_W::new(self)
    }
    #[doc = "Bit 1 - Real-time clock alarm interrupt flag"]
    #[inline(always)]
    pub fn rtcaifg(&mut self) -> RTCAIFG_W<1> {
        RTCAIFG_W::new(self)
    }
    #[doc = "Bit 2 - Real-time clock time event interrupt flag"]
    #[inline(always)]
    pub fn rtctevifg(&mut self) -> RTCTEVIFG_W<2> {
        RTCTEVIFG_W::new(self)
    }
    #[doc = "Bit 3 - 32-kHz crystal oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn rtcofifg(&mut self) -> RTCOFIFG_W<3> {
        RTCOFIFG_W::new(self)
    }
    #[doc = "Bit 4 - Real-time clock ready interrupt enable"]
    #[inline(always)]
    pub fn rtcrdyie(&mut self) -> RTCRDYIE_W<4> {
        RTCRDYIE_W::new(self)
    }
    #[doc = "Bit 5 - Real-time clock alarm interrupt enable"]
    #[inline(always)]
    pub fn rtcaie(&mut self) -> RTCAIE_W<5> {
        RTCAIE_W::new(self)
    }
    #[doc = "Bit 6 - Real-time clock time event interrupt enable"]
    #[inline(always)]
    pub fn rtctevie(&mut self) -> RTCTEVIE_W<6> {
        RTCTEVIE_W::new(self)
    }
    #[doc = "Bit 7 - 32-kHz crystal oscillator fault interrupt enable"]
    #[inline(always)]
    pub fn rtcofie(&mut self) -> RTCOFIE_W<7> {
        RTCOFIE_W::new(self)
    }
    #[doc = "Bits 8:15 - Real-time clock key"]
    #[inline(always)]
    pub fn rtckey(&mut self) -> RTCKEY_W<8> {
        RTCKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTCCTL0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcctl0](index.html) module"]
pub struct RTCCTL0_SPEC;
impl crate::RegisterSpec for RTCCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtcctl0::R](R) reader structure"]
impl crate::Readable for RTCCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcctl0::W](W) writer structure"]
impl crate::Writable for RTCCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCCTL0 to value 0x9608"]
impl crate::Resettable for RTCCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x9608
    }
}
