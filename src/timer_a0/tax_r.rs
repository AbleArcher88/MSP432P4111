#[doc = "Register `TAxR` reader"]
pub struct R(crate::R<TAX_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAX_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAX_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAX_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAxR` writer"]
pub struct W(crate::W<TAX_R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAX_R_SPEC>;
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
impl From<crate::W<TAX_R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAX_R_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TimerA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tax_r](index.html) module"]
pub struct TAX_R_SPEC;
impl crate::RegisterSpec for TAX_R_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tax_r::R](R) reader structure"]
impl crate::Readable for TAX_R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tax_r::W](W) writer structure"]
impl crate::Writable for TAX_R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAxR to value 0"]
impl crate::Resettable for TAX_R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
