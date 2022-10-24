#[doc = "Register `DMA_REQMASKCLR` writer"]
pub struct W(crate::W<DMA_REQMASKCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_REQMASKCLR_SPEC>;
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
impl From<crate::W<DMA_REQMASKCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_REQMASKCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Set the appropriate bit to enable DMA requests for the channel corresponding to dma_req and dma_sreq.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum CLR_ENUM_WRITE_AW {
    #[doc = "0: No effect. Use the DMA_REQMASKSET Register to disable dma_req and dma_sreq from generating requests."]
    CLR_0 = 0,
    #[doc = "1: Enables dma_req\\[C\\]
or dma_sreq\\[C\\]
to generate DMA requests. Writing to a bit where a DMA channel is not implemented has no effect."]
    CLR_1 = 1,
}
impl From<CLR_ENUM_WRITE_AW> for u32 {
    #[inline(always)]
    fn from(variant: CLR_ENUM_WRITE_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `CLR` writer - Set the appropriate bit to enable DMA requests for the channel corresponding to dma_req and dma_sreq."]
pub type CLR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_REQMASKCLR_SPEC, u32, CLR_ENUM_WRITE_AW, 32, O>;
impl<'a, const O: u8> CLR_W<'a, O> {
    #[doc = "No effect. Use the DMA_REQMASKSET Register to disable dma_req and dma_sreq from generating requests."]
    #[inline(always)]
    pub fn clr_0(self) -> &'a mut W {
        self.variant(CLR_ENUM_WRITE_AW::CLR_0)
    }
    #[doc = "Enables dma_req\\[C\\]
or dma_sreq\\[C\\]
to generate DMA requests. Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub fn clr_1(self) -> &'a mut W {
        self.variant(CLR_ENUM_WRITE_AW::CLR_1)
    }
}
impl W {
    #[doc = "Bits 0:31 - Set the appropriate bit to enable DMA requests for the channel corresponding to dma_req and dma_sreq."]
    #[inline(always)]
    pub fn clr(&mut self) -> CLR_W<0> {
        CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Request Mask Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_reqmaskclr](index.html) module"]
pub struct DMA_REQMASKCLR_SPEC;
impl crate::RegisterSpec for DMA_REQMASKCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dma_reqmaskclr::W](W) writer structure"]
impl crate::Writable for DMA_REQMASKCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_REQMASKCLR to value 0"]
impl crate::Resettable for DMA_REQMASKCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
