#[doc = "Register `TLV_CHECKSUM` reader"]
pub struct R(crate::R<TLV_CHECKSUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TLV_CHECKSUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TLV_CHECKSUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TLV_CHECKSUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "TLV Checksum\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tlv_checksum](index.html) module"]
pub struct TLV_CHECKSUM_SPEC;
impl crate::RegisterSpec for TLV_CHECKSUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tlv_checksum::R](R) reader structure"]
impl crate::Readable for TLV_CHECKSUM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TLV_CHECKSUM to value 0"]
impl crate::Resettable for TLV_CHECKSUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
