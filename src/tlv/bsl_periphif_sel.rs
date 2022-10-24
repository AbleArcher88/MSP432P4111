#[doc = "Register `BSL_PERIPHIF_SEL` reader"]
pub struct R(crate::R<BSL_PERIPHIF_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSL_PERIPHIF_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSL_PERIPHIF_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSL_PERIPHIF_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "BSL Peripheral Interface Selection\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsl_periphif_sel](index.html) module"]
pub struct BSL_PERIPHIF_SEL_SPEC;
impl crate::RegisterSpec for BSL_PERIPHIF_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bsl_periphif_sel::R](R) reader structure"]
impl crate::Readable for BSL_PERIPHIF_SEL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BSL_PERIPHIF_SEL to value 0"]
impl crate::Resettable for BSL_PERIPHIF_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
