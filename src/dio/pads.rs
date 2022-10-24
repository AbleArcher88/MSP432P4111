#[doc = "Register `PADS` reader"]
pub struct R(crate::R<PADS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADS` writer"]
pub struct W(crate::W<PADS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADS_SPEC>;
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
impl From<crate::W<PADS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P1DS` reader - Port 1 Drive Strength"]
pub type P1DS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P1DS` writer - Port 1 Drive Strength"]
pub type P1DS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PADS_SPEC, u8, u8, 8, O>;
#[doc = "Field `P2DS` reader - Port 2 Drive Strength"]
pub type P2DS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P2DS` writer - Port 2 Drive Strength"]
pub type P2DS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PADS_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Port 1 Drive Strength"]
    #[inline(always)]
    pub fn p1ds(&self) -> P1DS_R {
        P1DS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Drive Strength"]
    #[inline(always)]
    pub fn p2ds(&self) -> P2DS_R {
        P2DS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 1 Drive Strength"]
    #[inline(always)]
    pub fn p1ds(&mut self) -> P1DS_W<0> {
        P1DS_W::new(self)
    }
    #[doc = "Bits 8:15 - Port 2 Drive Strength"]
    #[inline(always)]
    pub fn p2ds(&mut self) -> P2DS_W<8> {
        P2DS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port A Drive Strength\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pads](index.html) module"]
pub struct PADS_SPEC;
impl crate::RegisterSpec for PADS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pads::R](R) reader structure"]
impl crate::Readable for PADS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pads::W](W) writer structure"]
impl crate::Writable for PADS_SPEC {
    type Writer = W;
}
