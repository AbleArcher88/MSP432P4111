#[doc = "Register `UCAxRXBUF` reader"]
pub struct R(crate::R<UCAX_RXBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCAX_RXBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCAX_RXBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCAX_RXBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UCRXBUF` reader - Receive data buffer"]
pub type UCRXBUF_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Receive data buffer"]
    #[inline(always)]
    pub fn ucrxbuf(&self) -> UCRXBUF_R {
        UCRXBUF_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "eUSCI_Ax Receive Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucax_rxbuf](index.html) module"]
pub struct UCAX_RXBUF_SPEC;
impl crate::RegisterSpec for UCAX_RXBUF_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucax_rxbuf::R](R) reader structure"]
impl crate::Readable for UCAX_RXBUF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UCAxRXBUF to value 0"]
impl crate::Resettable for UCAX_RXBUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
