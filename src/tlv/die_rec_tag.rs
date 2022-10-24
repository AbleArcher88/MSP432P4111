#[doc = "Register `DIE_REC_TAG` reader"]
pub struct R(crate::R<DIE_REC_TAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIE_REC_TAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIE_REC_TAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIE_REC_TAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Die Record Tag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [die_rec_tag](index.html) module"]
pub struct DIE_REC_TAG_SPEC;
impl crate::RegisterSpec for DIE_REC_TAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [die_rec_tag::R](R) reader structure"]
impl crate::Readable for DIE_REC_TAG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIE_REC_TAG to value 0x0c"]
impl crate::Resettable for DIE_REC_TAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c
    }
}
