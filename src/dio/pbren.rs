#[doc = "Register `PBREN` reader"]
pub struct R(crate::R<PBREN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBREN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBREN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBREN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBREN` writer"]
pub struct W(crate::W<PBREN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBREN_SPEC>;
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
impl From<crate::W<PBREN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBREN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P3REN` reader - Port 3 Resistor Enable"]
pub type P3REN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P3REN` writer - Port 3 Resistor Enable"]
pub type P3REN_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PBREN_SPEC, u8, u8, 8, O>;
#[doc = "Field `P4REN` reader - Port 4 Resistor Enable"]
pub type P4REN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P4REN` writer - Port 4 Resistor Enable"]
pub type P4REN_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PBREN_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Port 3 Resistor Enable"]
    #[inline(always)]
    pub fn p3ren(&self) -> P3REN_R {
        P3REN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Resistor Enable"]
    #[inline(always)]
    pub fn p4ren(&self) -> P4REN_R {
        P4REN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 3 Resistor Enable"]
    #[inline(always)]
    pub fn p3ren(&mut self) -> P3REN_W<0> {
        P3REN_W::new(self)
    }
    #[doc = "Bits 8:15 - Port 4 Resistor Enable"]
    #[inline(always)]
    pub fn p4ren(&mut self) -> P4REN_W<8> {
        P4REN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port B Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbren](index.html) module"]
pub struct PBREN_SPEC;
impl crate::RegisterSpec for PBREN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pbren::R](R) reader structure"]
impl crate::Readable for PBREN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbren::W](W) writer structure"]
impl crate::Writable for PBREN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PBREN to value 0"]
impl crate::Resettable for PBREN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
