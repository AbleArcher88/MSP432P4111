#[doc = "Register `PJSEL0` reader"]
pub struct R(crate::R<PJSEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PJSEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PJSEL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PJSEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PJSEL0` writer"]
pub struct W(crate::W<PJSEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PJSEL0_SPEC>;
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
impl From<crate::W<PJSEL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PJSEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PJSEL0` reader - Port J Select 0"]
pub type PJSEL0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PJSEL0` writer - Port J Select 0"]
pub type PJSEL0_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PJSEL0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Port J Select 0"]
    #[inline(always)]
    pub fn pjsel0(&self) -> PJSEL0_R {
        PJSEL0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port J Select 0"]
    #[inline(always)]
    pub fn pjsel0(&mut self) -> PJSEL0_W<0> {
        PJSEL0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port J Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjsel0](index.html) module"]
pub struct PJSEL0_SPEC;
impl crate::RegisterSpec for PJSEL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pjsel0::R](R) reader structure"]
impl crate::Readable for PJSEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pjsel0::W](W) writer structure"]
impl crate::Writable for PJSEL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PJSEL0 to value 0"]
impl crate::Resettable for PJSEL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
