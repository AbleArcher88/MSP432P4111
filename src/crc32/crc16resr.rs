#[doc = "Register `CRC16RESR` reader"]
pub struct R(crate::R<CRC16RESR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC16RESR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC16RESR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC16RESR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC16RESR` writer"]
pub struct W(crate::W<CRC16RESR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC16RESR_SPEC>;
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
impl From<crate::W<CRC16RESR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC16RESR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC16RESR` reader - CRC16 reverse result"]
pub type CRC16RESR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CRC16RESR` writer - CRC16 reverse result"]
pub type CRC16RESR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, CRC16RESR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - CRC16 reverse result"]
    #[inline(always)]
    pub fn crc16resr(&self) -> CRC16RESR_R {
        CRC16RESR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC16 reverse result"]
    #[inline(always)]
    pub fn crc16resr(&mut self) -> CRC16RESR_W<0> {
        CRC16RESR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC16 Result Reverse\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc16resr](index.html) module"]
pub struct CRC16RESR_SPEC;
impl crate::RegisterSpec for CRC16RESR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [crc16resr::R](R) reader structure"]
impl crate::Readable for CRC16RESR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc16resr::W](W) writer structure"]
impl crate::Writable for CRC16RESR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRC16RESR to value 0xffff"]
impl crate::Resettable for CRC16RESR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
