#[doc = "Register `PAIN` reader"]
pub struct R(crate::R<PAIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `P1IN` reader - Port 1 Input"]
pub type P1IN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P2IN` reader - Port 2 Input"]
pub type P2IN_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Port 1 Input"]
    #[inline(always)]
    pub fn p1in(&self) -> P1IN_R {
        P1IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Input"]
    #[inline(always)]
    pub fn p2in(&self) -> P2IN_R {
        P2IN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Port A Input\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pain](index.html) module"]
pub struct PAIN_SPEC;
impl crate::RegisterSpec for PAIN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pain::R](R) reader structure"]
impl crate::Readable for PAIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PAIN to value 0"]
impl crate::Resettable for PAIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
