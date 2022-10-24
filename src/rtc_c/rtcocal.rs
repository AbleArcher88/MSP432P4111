#[doc = "Register `RTCOCAL` reader"]
pub struct R(crate::R<RTCOCAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCOCAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCOCAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCOCAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCOCAL` writer"]
pub struct W(crate::W<RTCOCAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCOCAL_SPEC>;
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
impl From<crate::W<RTCOCAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCOCAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCOCAL` reader - Real-time clock offset error calibration"]
pub type RTCOCAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTCOCAL` writer - Real-time clock offset error calibration"]
pub type RTCOCAL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, RTCOCAL_SPEC, u8, u8, 8, O>;
#[doc = "Field `RTCOCALS` reader - Real-time clock offset error calibration sign"]
pub type RTCOCALS_R = crate::BitReader<RTCOCALS_A>;
#[doc = "Real-time clock offset error calibration sign\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCOCALS_A {
    #[doc = "0: Down calibration. Frequency adjusted down."]
    RTCOCALS_0 = 0,
    #[doc = "1: Up calibration. Frequency adjusted up."]
    RTCOCALS_1 = 1,
}
impl From<RTCOCALS_A> for bool {
    #[inline(always)]
    fn from(variant: RTCOCALS_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCOCALS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCOCALS_A {
        match self.bits {
            false => RTCOCALS_A::RTCOCALS_0,
            true => RTCOCALS_A::RTCOCALS_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCOCALS_0`"]
    #[inline(always)]
    pub fn is_rtcocals_0(&self) -> bool {
        *self == RTCOCALS_A::RTCOCALS_0
    }
    #[doc = "Checks if the value of the field is `RTCOCALS_1`"]
    #[inline(always)]
    pub fn is_rtcocals_1(&self) -> bool {
        *self == RTCOCALS_A::RTCOCALS_1
    }
}
#[doc = "Field `RTCOCALS` writer - Real-time clock offset error calibration sign"]
pub type RTCOCALS_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCOCAL_SPEC, RTCOCALS_A, O>;
impl<'a, const O: u8> RTCOCALS_W<'a, O> {
    #[doc = "Down calibration. Frequency adjusted down."]
    #[inline(always)]
    pub fn rtcocals_0(self) -> &'a mut W {
        self.variant(RTCOCALS_A::RTCOCALS_0)
    }
    #[doc = "Up calibration. Frequency adjusted up."]
    #[inline(always)]
    pub fn rtcocals_1(self) -> &'a mut W {
        self.variant(RTCOCALS_A::RTCOCALS_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Real-time clock offset error calibration"]
    #[inline(always)]
    pub fn rtcocal(&self) -> RTCOCAL_R {
        RTCOCAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 15 - Real-time clock offset error calibration sign"]
    #[inline(always)]
    pub fn rtcocals(&self) -> RTCOCALS_R {
        RTCOCALS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Real-time clock offset error calibration"]
    #[inline(always)]
    pub fn rtcocal(&mut self) -> RTCOCAL_W<0> {
        RTCOCAL_W::new(self)
    }
    #[doc = "Bit 15 - Real-time clock offset error calibration sign"]
    #[inline(always)]
    pub fn rtcocals(&mut self) -> RTCOCALS_W<15> {
        RTCOCALS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTCOCAL Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcocal](index.html) module"]
pub struct RTCOCAL_SPEC;
impl crate::RegisterSpec for RTCOCAL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtcocal::R](R) reader structure"]
impl crate::Readable for RTCOCAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcocal::W](W) writer structure"]
impl crate::Writable for RTCOCAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCOCAL to value 0"]
impl crate::Resettable for RTCOCAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
