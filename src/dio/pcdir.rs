#[doc = "Register `PCDIR` reader"]
pub struct R(crate::R<PCDIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCDIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCDIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCDIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCDIR` writer"]
pub struct W(crate::W<PCDIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCDIR_SPEC>;
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
impl From<crate::W<PCDIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCDIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P5DIR` reader - Port 5 Direction"]
pub type P5DIR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P5DIR` writer - Port 5 Direction"]
pub type P5DIR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PCDIR_SPEC, u8, u8, 8, O>;
#[doc = "Field `P6DIR` reader - Port 6 Direction"]
pub type P6DIR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P6DIR` writer - Port 6 Direction"]
pub type P6DIR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PCDIR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Port 5 Direction"]
    #[inline(always)]
    pub fn p5dir(&self) -> P5DIR_R {
        P5DIR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Direction"]
    #[inline(always)]
    pub fn p6dir(&self) -> P6DIR_R {
        P6DIR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Direction"]
    #[inline(always)]
    pub fn p5dir(&mut self) -> P5DIR_W<0> {
        P5DIR_W::new(self)
    }
    #[doc = "Bits 8:15 - Port 6 Direction"]
    #[inline(always)]
    pub fn p6dir(&mut self) -> P6DIR_W<8> {
        P6DIR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port C Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcdir](index.html) module"]
pub struct PCDIR_SPEC;
impl crate::RegisterSpec for PCDIR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pcdir::R](R) reader structure"]
impl crate::Readable for PCDIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcdir::W](W) writer structure"]
impl crate::Writable for PCDIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCDIR to value 0"]
impl crate::Resettable for PCDIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
