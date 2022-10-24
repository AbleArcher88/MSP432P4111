#[doc = "Register `RESERVED5` reader"]
pub struct R(crate::R<RESERVED5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESERVED5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESERVED5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESERVED5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved5](index.html) module"]
pub struct RESERVED5_SPEC;
impl crate::RegisterSpec for RESERVED5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reserved5::R](R) reader structure"]
impl crate::Readable for RESERVED5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESERVED5 to value 0"]
impl crate::Resettable for RESERVED5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
