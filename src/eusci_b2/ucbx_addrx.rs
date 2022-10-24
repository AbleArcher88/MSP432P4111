#[doc = "Register `UCBxADDRX` reader"]
pub struct R(crate::R<UCBX_ADDRX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCBX_ADDRX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCBX_ADDRX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCBX_ADDRX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADDRX` reader - Received Address Register"]
pub type ADDRX_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Received Address Register"]
    #[inline(always)]
    pub fn addrx(&self) -> ADDRX_R {
        ADDRX_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "eUSCI_Bx I2C Received Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_addrx](index.html) module"]
pub struct UCBX_ADDRX_SPEC;
impl crate::RegisterSpec for UCBX_ADDRX_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucbx_addrx::R](R) reader structure"]
impl crate::Readable for UCBX_ADDRX_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UCBxADDRX to value 0"]
impl crate::Resettable for UCBX_ADDRX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
