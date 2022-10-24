#[doc = "Register `FLCTL_RDBRST_STARTADDR` reader"]
pub struct R(crate::R<FLCTL_RDBRST_STARTADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_RDBRST_STARTADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_RDBRST_STARTADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_RDBRST_STARTADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLCTL_RDBRST_STARTADDR` writer"]
pub struct W(crate::W<FLCTL_RDBRST_STARTADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLCTL_RDBRST_STARTADDR_SPEC>;
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
impl From<crate::W<FLCTL_RDBRST_STARTADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLCTL_RDBRST_STARTADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START_ADDRESS` reader - Start Address of Burst Operation"]
pub type START_ADDRESS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `START_ADDRESS` writer - Start Address of Burst Operation"]
pub type START_ADDRESS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLCTL_RDBRST_STARTADDR_SPEC, u32, u32, 21, O>;
impl R {
    #[doc = "Bits 0:20 - Start Address of Burst Operation"]
    #[inline(always)]
    pub fn start_address(&self) -> START_ADDRESS_R {
        START_ADDRESS_R::new((self.bits & 0x001f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:20 - Start Address of Burst Operation"]
    #[inline(always)]
    pub fn start_address(&mut self) -> START_ADDRESS_W<0> {
        START_ADDRESS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read Burst/Compare Start Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_rdbrst_startaddr](index.html) module"]
pub struct FLCTL_RDBRST_STARTADDR_SPEC;
impl crate::RegisterSpec for FLCTL_RDBRST_STARTADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_rdbrst_startaddr::R](R) reader structure"]
impl crate::Readable for FLCTL_RDBRST_STARTADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flctl_rdbrst_startaddr::W](W) writer structure"]
impl crate::Writable for FLCTL_RDBRST_STARTADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLCTL_RDBRST_STARTADDR to value 0"]
impl crate::Resettable for FLCTL_RDBRST_STARTADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
