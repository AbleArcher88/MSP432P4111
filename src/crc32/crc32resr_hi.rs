#[doc = "Register `CRC32RESR_HI` reader"]
pub struct R(crate::R<CRC32RESR_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC32RESR_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC32RESR_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC32RESR_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC32RESR_HI` writer"]
pub struct W(crate::W<CRC32RESR_HI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC32RESR_HI_SPEC>;
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
impl From<crate::W<CRC32RESR_HI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC32RESR_HI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC32RESR_HI` reader - CRC32 reverse result, upper 16 bits"]
pub type CRC32RESR_HI_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CRC32RESR_HI` writer - CRC32 reverse result, upper 16 bits"]
pub type CRC32RESR_HI_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, CRC32RESR_HI_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - CRC32 reverse result, upper 16 bits"]
    #[inline(always)]
    pub fn crc32resr_hi(&self) -> CRC32RESR_HI_R {
        CRC32RESR_HI_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC32 reverse result, upper 16 bits"]
    #[inline(always)]
    pub fn crc32resr_hi(&mut self) -> CRC32RESR_HI_W<0> {
        CRC32RESR_HI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC32 Result Reverse, Upper 16 bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc32resr_hi](index.html) module"]
pub struct CRC32RESR_HI_SPEC;
impl crate::RegisterSpec for CRC32RESR_HI_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [crc32resr_hi::R](R) reader structure"]
impl crate::Readable for CRC32RESR_HI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc32resr_hi::W](W) writer structure"]
impl crate::Writable for CRC32RESR_HI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRC32RESR_HI to value 0xffff"]
impl crate::Resettable for CRC32RESR_HI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
