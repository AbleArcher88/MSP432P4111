#[doc = "Register `PDIFG` reader"]
pub struct R(crate::R<PDIFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDIFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDIFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDIFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDIFG` writer"]
pub struct W(crate::W<PDIFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDIFG_SPEC>;
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
impl From<crate::W<PDIFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDIFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P7IFG` reader - Port 7 Interrupt Flag"]
pub type P7IFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P7IFG` writer - Port 7 Interrupt Flag"]
pub type P7IFG_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PDIFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `P8IFG` reader - Port 8 Interrupt Flag"]
pub type P8IFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P8IFG` writer - Port 8 Interrupt Flag"]
pub type P8IFG_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PDIFG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Port 7 Interrupt Flag"]
    #[inline(always)]
    pub fn p7ifg(&self) -> P7IFG_R {
        P7IFG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Interrupt Flag"]
    #[inline(always)]
    pub fn p8ifg(&self) -> P8IFG_R {
        P8IFG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 7 Interrupt Flag"]
    #[inline(always)]
    pub fn p7ifg(&mut self) -> P7IFG_W<0> {
        P7IFG_W::new(self)
    }
    #[doc = "Bits 8:15 - Port 8 Interrupt Flag"]
    #[inline(always)]
    pub fn p8ifg(&mut self) -> P8IFG_W<8> {
        P8IFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port D Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdifg](index.html) module"]
pub struct PDIFG_SPEC;
impl crate::RegisterSpec for PDIFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pdifg::R](R) reader structure"]
impl crate::Readable for PDIFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdifg::W](W) writer structure"]
impl crate::Writable for PDIFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDIFG to value 0"]
impl crate::Resettable for PDIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
