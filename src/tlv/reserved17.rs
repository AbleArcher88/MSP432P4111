#[doc = "Register `RESERVED17` reader"]
pub struct R(crate::R<RESERVED17_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESERVED17_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESERVED17_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESERVED17_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved17](index.html) module"]
pub struct RESERVED17_SPEC;
impl crate::RegisterSpec for RESERVED17_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reserved17::R](R) reader structure"]
impl crate::Readable for RESERVED17_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESERVED17 to value 0"]
impl crate::Resettable for RESERVED17_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
