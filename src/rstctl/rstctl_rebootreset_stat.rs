#[doc = "Register `RSTCTL_REBOOTRESET_STAT` reader"]
pub struct R(crate::R<RSTCTL_REBOOTRESET_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCTL_REBOOTRESET_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCTL_REBOOTRESET_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCTL_REBOOTRESET_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REBOOT` reader - Indicates if Reboot reset was caused by the SYSCTL module."]
pub type REBOOT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Indicates if Reboot reset was caused by the SYSCTL module."]
    #[inline(always)]
    pub fn reboot(&self) -> REBOOT_R {
        REBOOT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Reboot Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl_rebootreset_stat](index.html) module"]
pub struct RSTCTL_REBOOTRESET_STAT_SPEC;
impl crate::RegisterSpec for RSTCTL_REBOOTRESET_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstctl_rebootreset_stat::R](R) reader structure"]
impl crate::Readable for RSTCTL_REBOOTRESET_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSTCTL_REBOOTRESET_STAT to value 0"]
impl crate::Resettable for RSTCTL_REBOOTRESET_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
