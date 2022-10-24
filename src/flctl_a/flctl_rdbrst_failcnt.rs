#[doc = "Register `FLCTL_RDBRST_FAILCNT` reader"]
pub struct R(crate::R<FLCTL_RDBRST_FAILCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_RDBRST_FAILCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_RDBRST_FAILCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_RDBRST_FAILCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLCTL_RDBRST_FAILCNT` writer"]
pub struct W(crate::W<FLCTL_RDBRST_FAILCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLCTL_RDBRST_FAILCNT_SPEC>;
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
impl From<crate::W<FLCTL_RDBRST_FAILCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLCTL_RDBRST_FAILCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FAIL_COUNT` reader - Number of failures encountered in burst operation"]
pub type FAIL_COUNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FAIL_COUNT` writer - Number of failures encountered in burst operation"]
pub type FAIL_COUNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLCTL_RDBRST_FAILCNT_SPEC, u32, u32, 17, O>;
impl R {
    #[doc = "Bits 0:16 - Number of failures encountered in burst operation"]
    #[inline(always)]
    pub fn fail_count(&self) -> FAIL_COUNT_R {
        FAIL_COUNT_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - Number of failures encountered in burst operation"]
    #[inline(always)]
    pub fn fail_count(&mut self) -> FAIL_COUNT_W<0> {
        FAIL_COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read Burst/Compare Fail Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_rdbrst_failcnt](index.html) module"]
pub struct FLCTL_RDBRST_FAILCNT_SPEC;
impl crate::RegisterSpec for FLCTL_RDBRST_FAILCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_rdbrst_failcnt::R](R) reader structure"]
impl crate::Readable for FLCTL_RDBRST_FAILCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flctl_rdbrst_failcnt::W](W) writer structure"]
impl crate::Writable for FLCTL_RDBRST_FAILCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLCTL_RDBRST_FAILCNT to value 0"]
impl crate::Resettable for FLCTL_RDBRST_FAILCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
