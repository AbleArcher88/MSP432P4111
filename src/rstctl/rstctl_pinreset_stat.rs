#[doc = "Register `RSTCTL_PINRESET_STAT` reader"]
pub struct R(crate::R<RSTCTL_PINRESET_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCTL_PINRESET_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCTL_PINRESET_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCTL_PINRESET_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RSTNMI` reader - POR was caused by RSTn/NMI pin based reset event"]
pub type RSTNMI_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - POR was caused by RSTn/NMI pin based reset event"]
    #[inline(always)]
    pub fn rstnmi(&self) -> RSTNMI_R {
        RSTNMI_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Pin Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl_pinreset_stat](index.html) module"]
pub struct RSTCTL_PINRESET_STAT_SPEC;
impl crate::RegisterSpec for RSTCTL_PINRESET_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstctl_pinreset_stat::R](R) reader structure"]
impl crate::Readable for RSTCTL_PINRESET_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSTCTL_PINRESET_STAT to value 0"]
impl crate::Resettable for RSTCTL_PINRESET_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
