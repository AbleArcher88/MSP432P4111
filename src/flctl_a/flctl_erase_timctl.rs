#[doc = "Register `FLCTL_ERASE_TIMCTL` reader"]
pub struct R(crate::R<FLCTL_ERASE_TIMCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_ERASE_TIMCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_ERASE_TIMCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_ERASE_TIMCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SETUP` reader - Length of the Setup phase for this operation"]
pub type SETUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACTIVE` reader - Length of the Active phase for this operation"]
pub type ACTIVE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HOLD` reader - Length of the Hold phase for this operation"]
pub type HOLD_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Length of the Setup phase for this operation"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:27 - Length of the Active phase for this operation"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 8) & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 28:31 - Length of the Hold phase for this operation"]
    #[inline(always)]
    pub fn hold(&self) -> HOLD_R {
        HOLD_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Erase Timing Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_erase_timctl](index.html) module"]
pub struct FLCTL_ERASE_TIMCTL_SPEC;
impl crate::RegisterSpec for FLCTL_ERASE_TIMCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_erase_timctl::R](R) reader structure"]
impl crate::Readable for FLCTL_ERASE_TIMCTL_SPEC {
    type Reader = R;
}
