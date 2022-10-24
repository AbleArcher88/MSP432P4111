#[doc = "Register `PEIN` reader"]
pub struct R(crate::R<PEIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `P9IN` reader - Port 9 Input"]
pub type P9IN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P10IN` reader - Port 10 Input"]
pub type P10IN_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Port 9 Input"]
    #[inline(always)]
    pub fn p9in(&self) -> P9IN_R {
        P9IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Input"]
    #[inline(always)]
    pub fn p10in(&self) -> P10IN_R {
        P10IN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Port E Input\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pein](index.html) module"]
pub struct PEIN_SPEC;
impl crate::RegisterSpec for PEIN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pein::R](R) reader structure"]
impl crate::Readable for PEIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PEIN to value 0"]
impl crate::Resettable for PEIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
