#[doc = "Register `SYS_SRAM_NUMBLOCKS` reader"]
pub struct R(crate::R<SYS_SRAM_NUMBLOCKS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_SRAM_NUMBLOCKS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_SRAM_NUMBLOCKS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_SRAM_NUMBLOCKS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NUM` reader - Indicates the number of SRAM blocks on the device."]
pub type NUM_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Indicates the number of SRAM blocks on the device."]
    #[inline(always)]
    pub fn num(&self) -> NUM_R {
        NUM_R::new(self.bits)
    }
}
#[doc = "SRAM Number of Blocks Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_sram_numblocks](index.html) module"]
pub struct SYS_SRAM_NUMBLOCKS_SPEC;
impl crate::RegisterSpec for SYS_SRAM_NUMBLOCKS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_sram_numblocks::R](R) reader structure"]
impl crate::Readable for SYS_SRAM_NUMBLOCKS_SPEC {
    type Reader = R;
}
