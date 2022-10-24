#[doc = "Register `REF_2P5V` reader"]
pub struct R(crate::R<REF_2P5V_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REF_2P5V_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REF_2P5V_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REF_2P5V_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "REF 2.5V Reference\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ref_2p5v](index.html) module"]
pub struct REF_2P5V_SPEC;
impl crate::RegisterSpec for REF_2P5V_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ref_2p5v::R](R) reader structure"]
impl crate::Readable for REF_2P5V_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets REF_2P5V to value 0"]
impl crate::Resettable for REF_2P5V_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
