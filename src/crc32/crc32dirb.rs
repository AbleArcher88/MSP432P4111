#[doc = "Register `CRC32DIRB` reader"]
pub struct R(crate::R<CRC32DIRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC32DIRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC32DIRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC32DIRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC32DIRB` writer"]
pub struct W(crate::W<CRC32DIRB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC32DIRB_SPEC>;
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
impl From<crate::W<CRC32DIRB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC32DIRB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC32DIRB` reader - Data input register reversed"]
pub type CRC32DIRB_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CRC32DIRB` writer - Data input register reversed"]
pub type CRC32DIRB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, CRC32DIRB_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Data input register reversed"]
    #[inline(always)]
    pub fn crc32dirb(&self) -> CRC32DIRB_R {
        CRC32DIRB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data input register reversed"]
    #[inline(always)]
    pub fn crc32dirb(&mut self) -> CRC32DIRB_W<0> {
        CRC32DIRB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data In Reverse for CRC32 Computation\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc32dirb](index.html) module"]
pub struct CRC32DIRB_SPEC;
impl crate::RegisterSpec for CRC32DIRB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [crc32dirb::R](R) reader structure"]
impl crate::Readable for CRC32DIRB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc32dirb::W](W) writer structure"]
impl crate::Writable for CRC32DIRB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRC32DIRB to value 0"]
impl crate::Resettable for CRC32DIRB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
