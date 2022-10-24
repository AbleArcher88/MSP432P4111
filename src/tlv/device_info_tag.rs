#[doc = "Register `DEVICE_INFO_TAG` reader"]
pub struct R(crate::R<DEVICE_INFO_TAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVICE_INFO_TAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVICE_INFO_TAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVICE_INFO_TAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Device Info Tag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [device_info_tag](index.html) module"]
pub struct DEVICE_INFO_TAG_SPEC;
impl crate::RegisterSpec for DEVICE_INFO_TAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [device_info_tag::R](R) reader structure"]
impl crate::Readable for DEVICE_INFO_TAG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVICE_INFO_TAG to value 0x0b"]
impl crate::Resettable for DEVICE_INFO_TAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0b
    }
}
