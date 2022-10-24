#[doc = "Register `TLV_END` reader"]
pub struct R(crate::R<TLV_END_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TLV_END_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TLV_END_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TLV_END_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "TLV End Word\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tlv_end](index.html) module"]
pub struct TLV_END_SPEC;
impl crate::RegisterSpec for TLV_END_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tlv_end::R](R) reader structure"]
impl crate::Readable for TLV_END_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TLV_END to value 0x0bd0_e11d"]
impl crate::Resettable for TLV_END_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0bd0_e11d
    }
}
