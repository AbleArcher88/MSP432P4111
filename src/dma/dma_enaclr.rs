#[doc = "Register `DMA_ENACLR` writer"]
pub struct W(crate::W<DMA_ENACLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_ENACLR_SPEC>;
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
impl From<crate::W<DMA_ENACLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_ENACLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Set the appropriate bit to disable the corresponding DMA channel. Note: The controller disables a channel, by setting the appropriate bit, when: a) it completes the DMA cycle b) it reads a channel_cfg memory location which has cycle_ctrl = b000 c) an ERROR occurs on the AHB-Lite bus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum CLR_ENUM_WRITE_AW {
    #[doc = "0: No effect. Use the DMA_ENASET Register to enable DMA channels."]
    CLR_0 = 0,
    #[doc = "1: Disables channel C. Writing to a bit where a DMA channel is not implemented has no effect."]
    CLR_1 = 1,
}
impl From<CLR_ENUM_WRITE_AW> for u32 {
    #[inline(always)]
    fn from(variant: CLR_ENUM_WRITE_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `CLR` writer - Set the appropriate bit to disable the corresponding DMA channel. Note: The controller disables a channel, by setting the appropriate bit, when: a) it completes the DMA cycle b) it reads a channel_cfg memory location which has cycle_ctrl = b000 c) an ERROR occurs on the AHB-Lite bus."]
pub type CLR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_ENACLR_SPEC, u32, CLR_ENUM_WRITE_AW, 32, O>;
impl<'a, const O: u8> CLR_W<'a, O> {
    #[doc = "No effect. Use the DMA_ENASET Register to enable DMA channels."]
    #[inline(always)]
    pub fn clr_0(self) -> &'a mut W {
        self.variant(CLR_ENUM_WRITE_AW::CLR_0)
    }
    #[doc = "Disables channel C. Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub fn clr_1(self) -> &'a mut W {
        self.variant(CLR_ENUM_WRITE_AW::CLR_1)
    }
}
impl W {
    #[doc = "Bits 0:31 - Set the appropriate bit to disable the corresponding DMA channel. Note: The controller disables a channel, by setting the appropriate bit, when: a) it completes the DMA cycle b) it reads a channel_cfg memory location which has cycle_ctrl = b000 c) an ERROR occurs on the AHB-Lite bus."]
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
#[doc = "Channel Enable Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_enaclr](index.html) module"]
pub struct DMA_ENACLR_SPEC;
impl crate::RegisterSpec for DMA_ENACLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dma_enaclr::W](W) writer structure"]
impl crate::Writable for DMA_ENACLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_ENACLR to value 0"]
impl crate::Resettable for DMA_ENACLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
