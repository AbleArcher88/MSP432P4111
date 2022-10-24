#[doc = "Register `PCMCLRIFG` writer"]
pub struct W(crate::W<PCMCLRIFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCMCLRIFG_SPEC>;
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
impl From<crate::W<PCMCLRIFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCMCLRIFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLR_LPM_INVALID_TR_IFG` writer - Clear LPM invalid transition flag"]
pub type CLR_LPM_INVALID_TR_IFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PCMCLRIFG_SPEC, bool, O>;
#[doc = "Field `CLR_LPM_INVALID_CLK_IFG` writer - Clear LPM invalid clock flag"]
pub type CLR_LPM_INVALID_CLK_IFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PCMCLRIFG_SPEC, bool, O>;
#[doc = "Field `CLR_AM_INVALID_TR_IFG` writer - Clear active mode invalid transition flag"]
pub type CLR_AM_INVALID_TR_IFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PCMCLRIFG_SPEC, bool, O>;
#[doc = "Field `CLR_DCDC_ERROR_IFG` writer - Clear DC-DC error flag"]
pub type CLR_DCDC_ERROR_IFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCMCLRIFG_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Clear LPM invalid transition flag"]
    #[inline(always)]
    pub fn clr_lpm_invalid_tr_ifg(&mut self) -> CLR_LPM_INVALID_TR_IFG_W<0> {
        CLR_LPM_INVALID_TR_IFG_W::new(self)
    }
    #[doc = "Bit 1 - Clear LPM invalid clock flag"]
    #[inline(always)]
    pub fn clr_lpm_invalid_clk_ifg(&mut self) -> CLR_LPM_INVALID_CLK_IFG_W<1> {
        CLR_LPM_INVALID_CLK_IFG_W::new(self)
    }
    #[doc = "Bit 2 - Clear active mode invalid transition flag"]
    #[inline(always)]
    pub fn clr_am_invalid_tr_ifg(&mut self) -> CLR_AM_INVALID_TR_IFG_W<2> {
        CLR_AM_INVALID_TR_IFG_W::new(self)
    }
    #[doc = "Bit 6 - Clear DC-DC error flag"]
    #[inline(always)]
    pub fn clr_dcdc_error_ifg(&mut self) -> CLR_DCDC_ERROR_IFG_W<6> {
        CLR_DCDC_ERROR_IFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear Interrupt Flag Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcmclrifg](index.html) module"]
pub struct PCMCLRIFG_SPEC;
impl crate::RegisterSpec for PCMCLRIFG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pcmclrifg::W](W) writer structure"]
impl crate::Writable for PCMCLRIFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCMCLRIFG to value 0"]
impl crate::Resettable for PCMCLRIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
