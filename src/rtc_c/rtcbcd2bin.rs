#[doc = "Register `RTCBCD2BIN` reader"]
pub struct R(crate::R<RTCBCD2BIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCBCD2BIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCBCD2BIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCBCD2BIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCBCD2BIN` writer"]
pub struct W(crate::W<RTCBCD2BIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCBCD2BIN_SPEC>;
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
impl From<crate::W<RTCBCD2BIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCBCD2BIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCD2BIN` reader - bcd to bin conversion"]
pub type BCD2BIN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BCD2BIN` writer - bcd to bin conversion"]
pub type BCD2BIN_W<'a, const O: u8> = crate::FieldWriter<'a, u16, RTCBCD2BIN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - bcd to bin conversion"]
    #[inline(always)]
    pub fn bcd2bin(&self) -> BCD2BIN_R {
        BCD2BIN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - bcd to bin conversion"]
    #[inline(always)]
    pub fn bcd2bin(&mut self) -> BCD2BIN_W<0> {
        BCD2BIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BCD-to-Binary Conversion Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcbcd2bin](index.html) module"]
pub struct RTCBCD2BIN_SPEC;
impl crate::RegisterSpec for RTCBCD2BIN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtcbcd2bin::R](R) reader structure"]
impl crate::Readable for RTCBCD2BIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcbcd2bin::W](W) writer structure"]
impl crate::Writable for RTCBCD2BIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCBCD2BIN to value 0"]
impl crate::Resettable for RTCBCD2BIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
