#[doc = "Register `PBSEL1` reader"]
pub struct R(crate::R<PBSEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBSEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBSEL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBSEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBSEL1` writer"]
pub struct W(crate::W<PBSEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBSEL1_SPEC>;
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
impl From<crate::W<PBSEL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBSEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P3SEL1` reader - Port 3 Select 1"]
pub type P3SEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P3SEL1` writer - Port 3 Select 1"]
pub type P3SEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PBSEL1_SPEC, u8, u8, 8, O>;
#[doc = "Field `P4SEL1` reader - Port 4 Select 1"]
pub type P4SEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P4SEL1` writer - Port 4 Select 1"]
pub type P4SEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PBSEL1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Port 3 Select 1"]
    #[inline(always)]
    pub fn p3sel1(&self) -> P3SEL1_R {
        P3SEL1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Select 1"]
    #[inline(always)]
    pub fn p4sel1(&self) -> P4SEL1_R {
        P4SEL1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 3 Select 1"]
    #[inline(always)]
    pub fn p3sel1(&mut self) -> P3SEL1_W<0> {
        P3SEL1_W::new(self)
    }
    #[doc = "Bits 8:15 - Port 4 Select 1"]
    #[inline(always)]
    pub fn p4sel1(&mut self) -> P4SEL1_W<8> {
        P4SEL1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port B Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbsel1](index.html) module"]
pub struct PBSEL1_SPEC;
impl crate::RegisterSpec for PBSEL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pbsel1::R](R) reader structure"]
impl crate::Readable for PBSEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbsel1::W](W) writer structure"]
impl crate::Writable for PBSEL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PBSEL1 to value 0"]
impl crate::Resettable for PBSEL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
