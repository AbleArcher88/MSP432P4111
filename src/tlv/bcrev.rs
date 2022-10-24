#[doc = "Register `BCREV` reader"]
pub struct R(crate::R<BCREV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCREV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCREV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCREV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Boot Code Revision\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcrev](index.html) module"]
pub struct BCREV_SPEC;
impl crate::RegisterSpec for BCREV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bcrev::R](R) reader structure"]
impl crate::Readable for BCREV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BCREV to value 0"]
impl crate::Resettable for BCREV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
