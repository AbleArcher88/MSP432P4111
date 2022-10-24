#[doc = "Register `PBDIR` reader"]
pub struct R(crate::R<PBDIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBDIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBDIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBDIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBDIR` writer"]
pub struct W(crate::W<PBDIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBDIR_SPEC>;
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
impl From<crate::W<PBDIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBDIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P3DIR` reader - Port 3 Direction"]
pub type P3DIR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P3DIR` writer - Port 3 Direction"]
pub type P3DIR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PBDIR_SPEC, u8, u8, 8, O>;
#[doc = "Field `P4DIR` reader - Port 4 Direction"]
pub type P4DIR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P4DIR` writer - Port 4 Direction"]
pub type P4DIR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PBDIR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Port 3 Direction"]
    #[inline(always)]
    pub fn p3dir(&self) -> P3DIR_R {
        P3DIR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Direction"]
    #[inline(always)]
    pub fn p4dir(&self) -> P4DIR_R {
        P4DIR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 3 Direction"]
    #[inline(always)]
    pub fn p3dir(&mut self) -> P3DIR_W<0> {
        P3DIR_W::new(self)
    }
    #[doc = "Bits 8:15 - Port 4 Direction"]
    #[inline(always)]
    pub fn p4dir(&mut self) -> P4DIR_W<8> {
        P4DIR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port B Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbdir](index.html) module"]
pub struct PBDIR_SPEC;
impl crate::RegisterSpec for PBDIR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pbdir::R](R) reader structure"]
impl crate::Readable for PBDIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbdir::W](W) writer structure"]
impl crate::Writable for PBDIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PBDIR to value 0"]
impl crate::Resettable for PBDIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
