#[doc = "Register `PEDS` reader"]
pub struct R(crate::R<PEDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEDS` writer"]
pub struct W(crate::W<PEDS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PEDS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEDS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P9DS` reader - Port 9 Drive Strength"]
pub type P9DS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P9DS` writer - Port 9 Drive Strength"]
pub type P9DS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PEDS_SPEC, u8, u8, 8, O>;
#[doc = "Field `P10DS` reader - Port 10 Drive Strength"]
pub type P10DS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P10DS` writer - Port 10 Drive Strength"]
pub type P10DS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PEDS_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Port 9 Drive Strength"]
    #[inline(always)]
    pub fn p9ds(&self) -> P9DS_R {
        P9DS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Drive Strength"]
    #[inline(always)]
    pub fn p10ds(&self) -> P10DS_R {
        P10DS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Drive Strength"]
    #[inline(always)]
    pub fn p9ds(&mut self) -> P9DS_W<0> {
        P9DS_W::new(self)
    }
    #[doc = "Bits 8:15 - Port 10 Drive Strength"]
    #[inline(always)]
    pub fn p10ds(&mut self) -> P10DS_W<8> {
        P10DS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port E Drive Strength\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peds](index.html) module"]
pub struct PEDS_SPEC;
impl crate::RegisterSpec for PEDS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [peds::R](R) reader structure"]
impl crate::Readable for PEDS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peds::W](W) writer structure"]
impl crate::Writable for PEDS_SPEC {
    type Writer = W;
}
