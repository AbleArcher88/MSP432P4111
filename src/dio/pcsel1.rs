#[doc = "Register `PCSEL1` reader"]
pub struct R(crate::R<PCSEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCSEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCSEL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCSEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCSEL1` writer"]
pub struct W(crate::W<PCSEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCSEL1_SPEC>;
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
impl From<crate::W<PCSEL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCSEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P5SEL1` reader - Port 5 Select 1"]
pub type P5SEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P5SEL1` writer - Port 5 Select 1"]
pub type P5SEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PCSEL1_SPEC, u8, u8, 8, O>;
#[doc = "Field `P6SEL1` reader - Port 6 Select 1"]
pub type P6SEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P6SEL1` writer - Port 6 Select 1"]
pub type P6SEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PCSEL1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Port 5 Select 1"]
    #[inline(always)]
    pub fn p5sel1(&self) -> P5SEL1_R {
        P5SEL1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Select 1"]
    #[inline(always)]
    pub fn p6sel1(&self) -> P6SEL1_R {
        P6SEL1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Select 1"]
    #[inline(always)]
    pub fn p5sel1(&mut self) -> P5SEL1_W<0> {
        P5SEL1_W::new(self)
    }
    #[doc = "Bits 8:15 - Port 6 Select 1"]
    #[inline(always)]
    pub fn p6sel1(&mut self) -> P6SEL1_W<8> {
        P6SEL1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port C Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcsel1](index.html) module"]
pub struct PCSEL1_SPEC;
impl crate::RegisterSpec for PCSEL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pcsel1::R](R) reader structure"]
impl crate::Readable for PCSEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcsel1::W](W) writer structure"]
impl crate::Writable for PCSEL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCSEL1 to value 0"]
impl crate::Resettable for PCSEL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
