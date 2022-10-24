#[doc = "Register `LCDANM[%s]` reader"]
pub struct R(crate::R<LCDANM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDANM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDANM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDANM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDANM[%s]` writer"]
pub struct W(crate::W<LCDANM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDANM_SPEC>;
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
impl From<crate::W<LCDANM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCDANM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDANIMMEMORY` reader - LCD Animation memory Contains Animation data for animation step Tx"]
pub type LCDANIMMEMORY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LCDANIMMEMORY` writer - LCD Animation memory Contains Animation data for animation step Tx"]
pub type LCDANIMMEMORY_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LCDANM_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - LCD Animation memory Contains Animation data for animation step Tx"]
    #[inline(always)]
    pub fn lcdanimmemory(&self) -> LCDANIMMEMORY_R {
        LCDANIMMEMORY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - LCD Animation memory Contains Animation data for animation step Tx"]
    #[inline(always)]
    pub fn lcdanimmemory(&mut self) -> LCDANIMMEMORY_W<0> {
        LCDANIMMEMORY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Animation memory registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdanm](index.html) module"]
pub struct LCDANM_SPEC;
impl crate::RegisterSpec for LCDANM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lcdanm::R](R) reader structure"]
impl crate::Readable for LCDANM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdanm::W](W) writer structure"]
impl crate::Writable for LCDANM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDANM[%s]
to value 0"]
impl crate::Resettable for LCDANM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
