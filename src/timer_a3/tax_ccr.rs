#[doc = "Register `TAxCCR[%s]` reader"]
pub struct R(crate::R<TAX_CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAX_CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAX_CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAX_CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAxCCR[%s]` writer"]
pub struct W(crate::W<TAX_CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAX_CCR_SPEC>;
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
impl From<crate::W<TAX_CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAX_CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAxR` reader - TimerA register"]
pub type TAX_R_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TAxR` writer - TimerA register"]
pub type TAX_R_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TAX_CCR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - TimerA register"]
    #[inline(always)]
    pub fn tax_r(&self) -> TAX_R_R {
        TAX_R_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - TimerA register"]
    #[inline(always)]
    pub fn tax_r(&mut self) -> TAX_R_W<0> {
        TAX_R_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer_A Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tax_ccr](index.html) module"]
pub struct TAX_CCR_SPEC;
impl crate::RegisterSpec for TAX_CCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tax_ccr::R](R) reader structure"]
impl crate::Readable for TAX_CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tax_ccr::W](W) writer structure"]
impl crate::Writable for TAX_CCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAxCCR[%s]
to value 0"]
impl crate::Resettable for TAX_CCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
