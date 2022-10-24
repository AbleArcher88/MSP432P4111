#[doc = "Register `FLASH_MAX_PROG_PULSES` reader"]
pub struct R(crate::R<FLASH_MAX_PROG_PULSES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_MAX_PROG_PULSES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_MAX_PROG_PULSES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_MAX_PROG_PULSES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Flash Maximum Programming Pulses\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_max_prog_pulses](index.html) module"]
pub struct FLASH_MAX_PROG_PULSES_SPEC;
impl crate::RegisterSpec for FLASH_MAX_PROG_PULSES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_max_prog_pulses::R](R) reader structure"]
impl crate::Readable for FLASH_MAX_PROG_PULSES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FLASH_MAX_PROG_PULSES to value 0"]
impl crate::Resettable for FLASH_MAX_PROG_PULSES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
