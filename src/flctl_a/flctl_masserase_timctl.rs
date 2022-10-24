#[doc = "Register `FLCTL_MASSERASE_TIMCTL` reader"]
pub struct R(crate::R<FLCTL_MASSERASE_TIMCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_MASSERASE_TIMCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_MASSERASE_TIMCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_MASSERASE_TIMCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BOOST_ACTIVE` reader - Length of the time for which LDO Boost Signal is kept active"]
pub type BOOST_ACTIVE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BOOST_HOLD` reader - Length for which Flash deactivates the LDO Boost signal before processing any new commands"]
pub type BOOST_HOLD_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Length of the time for which LDO Boost Signal is kept active"]
    #[inline(always)]
    pub fn boost_active(&self) -> BOOST_ACTIVE_R {
        BOOST_ACTIVE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Length for which Flash deactivates the LDO Boost signal before processing any new commands"]
    #[inline(always)]
    pub fn boost_hold(&self) -> BOOST_HOLD_R {
        BOOST_HOLD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Mass Erase Timing Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_masserase_timctl](index.html) module"]
pub struct FLCTL_MASSERASE_TIMCTL_SPEC;
impl crate::RegisterSpec for FLCTL_MASSERASE_TIMCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_masserase_timctl::R](R) reader structure"]
impl crate::Readable for FLCTL_MASSERASE_TIMCTL_SPEC {
    type Reader = R;
}
