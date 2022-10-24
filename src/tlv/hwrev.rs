#[doc = "Register `HWREV` reader"]
pub struct R(crate::R<HWREV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWREV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWREV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWREV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "HW Revision\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwrev](index.html) module"]
pub struct HWREV_SPEC;
impl crate::RegisterSpec for HWREV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwrev::R](R) reader structure"]
impl crate::Readable for HWREV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HWREV to value 0"]
impl crate::Resettable for HWREV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
