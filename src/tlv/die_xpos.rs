#[doc = "Register `DIE_XPOS` reader"]
pub struct R(crate::R<DIE_XPOS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIE_XPOS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIE_XPOS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIE_XPOS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Die X-Position\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [die_xpos](index.html) module"]
pub struct DIE_XPOS_SPEC;
impl crate::RegisterSpec for DIE_XPOS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [die_xpos::R](R) reader structure"]
impl crate::Readable for DIE_XPOS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIE_XPOS to value 0"]
impl crate::Resettable for DIE_XPOS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
