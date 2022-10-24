#[doc = "Register `PCIN` reader"]
pub struct R(crate::R<PCIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `P5IN` reader - Port 5 Input"]
pub type P5IN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P6IN` reader - Port 6 Input"]
pub type P6IN_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Port 5 Input"]
    #[inline(always)]
    pub fn p5in(&self) -> P5IN_R {
        P5IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Input"]
    #[inline(always)]
    pub fn p6in(&self) -> P6IN_R {
        P6IN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Port C Input\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcin](index.html) module"]
pub struct PCIN_SPEC;
impl crate::RegisterSpec for PCIN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pcin::R](R) reader structure"]
impl crate::Readable for PCIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PCIN to value 0"]
impl crate::Resettable for PCIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
