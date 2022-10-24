#[doc = "Register `PSSIE` reader"]
pub struct R(crate::R<PSSIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSSIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSSIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSSIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSSIE` writer"]
pub struct W(crate::W<PSSIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSSIE_SPEC>;
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
impl From<crate::W<PSSIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSSIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SVSMHIE` reader - High-side SVSM interrupt enable"]
pub type SVSMHIE_R = crate::BitReader<SVSMHIE_A>;
#[doc = "High-side SVSM interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SVSMHIE_A {
    #[doc = "0: Interrupt disabled"]
    SVSMHIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    SVSMHIE_1 = 1,
}
impl From<SVSMHIE_A> for bool {
    #[inline(always)]
    fn from(variant: SVSMHIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SVSMHIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVSMHIE_A {
        match self.bits {
            false => SVSMHIE_A::SVSMHIE_0,
            true => SVSMHIE_A::SVSMHIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SVSMHIE_0`"]
    #[inline(always)]
    pub fn is_svsmhie_0(&self) -> bool {
        *self == SVSMHIE_A::SVSMHIE_0
    }
    #[doc = "Checks if the value of the field is `SVSMHIE_1`"]
    #[inline(always)]
    pub fn is_svsmhie_1(&self) -> bool {
        *self == SVSMHIE_A::SVSMHIE_1
    }
}
#[doc = "Field `SVSMHIE` writer - High-side SVSM interrupt enable"]
pub type SVSMHIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSSIE_SPEC, SVSMHIE_A, O>;
impl<'a, const O: u8> SVSMHIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn svsmhie_0(self) -> &'a mut W {
        self.variant(SVSMHIE_A::SVSMHIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn svsmhie_1(self) -> &'a mut W {
        self.variant(SVSMHIE_A::SVSMHIE_1)
    }
}
impl R {
    #[doc = "Bit 1 - High-side SVSM interrupt enable"]
    #[inline(always)]
    pub fn svsmhie(&self) -> SVSMHIE_R {
        SVSMHIE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - High-side SVSM interrupt enable"]
    #[inline(always)]
    pub fn svsmhie(&mut self) -> SVSMHIE_W<1> {
        SVSMHIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pssie](index.html) module"]
pub struct PSSIE_SPEC;
impl crate::RegisterSpec for PSSIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pssie::R](R) reader structure"]
impl crate::Readable for PSSIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pssie::W](W) writer structure"]
impl crate::Writable for PSSIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSSIE to value 0"]
impl crate::Resettable for PSSIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
