#[doc = "Register `DMA_INT3_SRCCFG` reader"]
pub struct R(crate::R<DMA_INT3_SRCCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_INT3_SRCCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_INT3_SRCCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_INT3_SRCCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_INT3_SRCCFG` writer"]
pub struct W(crate::W<DMA_INT3_SRCCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_INT3_SRCCFG_SPEC>;
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
impl From<crate::W<DMA_INT3_SRCCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_INT3_SRCCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_SRC` reader - Controls which channel's completion event is mapped as a source of this Interrupt"]
pub type INT_SRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT_SRC` writer - Controls which channel's completion event is mapped as a source of this Interrupt"]
pub type INT_SRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_INT3_SRCCFG_SPEC, u8, u8, 5, O>;
#[doc = "Field `EN` reader - Enables DMA_INT3 mapping"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Enables DMA_INT3 mapping"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT3_SRCCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - Controls which channel's completion event is mapped as a source of this Interrupt"]
    #[inline(always)]
    pub fn int_src(&self) -> INT_SRC_R {
        INT_SRC_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Enables DMA_INT3 mapping"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Controls which channel's completion event is mapped as a source of this Interrupt"]
    #[inline(always)]
    pub fn int_src(&mut self) -> INT_SRC_W<0> {
        INT_SRC_W::new(self)
    }
    #[doc = "Bit 5 - Enables DMA_INT3 mapping"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<5> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt 3 Source Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int3_srccfg](index.html) module"]
pub struct DMA_INT3_SRCCFG_SPEC;
impl crate::RegisterSpec for DMA_INT3_SRCCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_int3_srccfg::R](R) reader structure"]
impl crate::Readable for DMA_INT3_SRCCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_int3_srccfg::W](W) writer structure"]
impl crate::Writable for DMA_INT3_SRCCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_INT3_SRCCFG to value 0"]
impl crate::Resettable for DMA_INT3_SRCCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
