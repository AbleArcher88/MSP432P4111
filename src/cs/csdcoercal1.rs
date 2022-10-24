#[doc = "Register `CSDCOERCAL1` reader"]
pub struct R(crate::R<CSDCOERCAL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSDCOERCAL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSDCOERCAL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSDCOERCAL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSDCOERCAL1` writer"]
pub struct W(crate::W<CSDCOERCAL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSDCOERCAL1_SPEC>;
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
impl From<crate::W<CSDCOERCAL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSDCOERCAL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCO_FCAL_RSEL5` reader - DCO frequency calibration for DCO frequency range (DCORSEL) 5."]
pub type DCO_FCAL_RSEL5_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DCO_FCAL_RSEL5` writer - DCO frequency calibration for DCO frequency range (DCORSEL) 5."]
pub type DCO_FCAL_RSEL5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSDCOERCAL1_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - DCO frequency calibration for DCO frequency range (DCORSEL) 5."]
    #[inline(always)]
    pub fn dco_fcal_rsel5(&self) -> DCO_FCAL_RSEL5_R {
        DCO_FCAL_RSEL5_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DCO frequency calibration for DCO frequency range (DCORSEL) 5."]
    #[inline(always)]
    pub fn dco_fcal_rsel5(&mut self) -> DCO_FCAL_RSEL5_W<0> {
        DCO_FCAL_RSEL5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCO External Resistor Calibration 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csdcoercal1](index.html) module"]
pub struct CSDCOERCAL1_SPEC;
impl crate::RegisterSpec for CSDCOERCAL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csdcoercal1::R](R) reader structure"]
impl crate::Readable for CSDCOERCAL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csdcoercal1::W](W) writer structure"]
impl crate::Writable for CSDCOERCAL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSDCOERCAL1 to value 0x0100"]
impl crate::Resettable for CSDCOERCAL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}
