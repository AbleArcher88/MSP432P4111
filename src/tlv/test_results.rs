#[doc = "Register `TEST_RESULTS` reader"]
pub struct R(crate::R<TEST_RESULTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST_RESULTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST_RESULTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST_RESULTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Test Results\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test_results](index.html) module"]
pub struct TEST_RESULTS_SPEC;
impl crate::RegisterSpec for TEST_RESULTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [test_results::R](R) reader structure"]
impl crate::Readable for TEST_RESULTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TEST_RESULTS to value 0"]
impl crate::Resettable for TEST_RESULTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
