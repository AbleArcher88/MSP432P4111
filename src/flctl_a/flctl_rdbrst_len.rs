#[doc = "Register `FLCTL_RDBRST_LEN` reader"]
pub struct R(crate::R<FLCTL_RDBRST_LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_RDBRST_LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_RDBRST_LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_RDBRST_LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLCTL_RDBRST_LEN` writer"]
pub struct W(crate::W<FLCTL_RDBRST_LEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLCTL_RDBRST_LEN_SPEC>;
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
impl From<crate::W<FLCTL_RDBRST_LEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLCTL_RDBRST_LEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BURST_LENGTH` reader - Length of Burst Operation"]
pub type BURST_LENGTH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BURST_LENGTH` writer - Length of Burst Operation"]
pub type BURST_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLCTL_RDBRST_LEN_SPEC, u32, u32, 21, O>;
impl R {
    #[doc = "Bits 0:20 - Length of Burst Operation"]
    #[inline(always)]
    pub fn burst_length(&self) -> BURST_LENGTH_R {
        BURST_LENGTH_R::new((self.bits & 0x001f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:20 - Length of Burst Operation"]
    #[inline(always)]
    pub fn burst_length(&mut self) -> BURST_LENGTH_W<0> {
        BURST_LENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read Burst/Compare Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_rdbrst_len](index.html) module"]
pub struct FLCTL_RDBRST_LEN_SPEC;
impl crate::RegisterSpec for FLCTL_RDBRST_LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_rdbrst_len::R](R) reader structure"]
impl crate::Readable for FLCTL_RDBRST_LEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flctl_rdbrst_len::W](W) writer structure"]
impl crate::Writable for FLCTL_RDBRST_LEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLCTL_RDBRST_LEN to value 0"]
impl crate::Resettable for FLCTL_RDBRST_LEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
