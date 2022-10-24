#[doc = "Register `FLASH_INFO_TAG` reader"]
pub struct R(crate::R<FLASH_INFO_TAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_INFO_TAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_INFO_TAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_INFO_TAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Flash Info Tag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_info_tag](index.html) module"]
pub struct FLASH_INFO_TAG_SPEC;
impl crate::RegisterSpec for FLASH_INFO_TAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_info_tag::R](R) reader structure"]
impl crate::Readable for FLASH_INFO_TAG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FLASH_INFO_TAG to value 0x04"]
impl crate::Resettable for FLASH_INFO_TAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
