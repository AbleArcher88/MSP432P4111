#[doc = "Register `FLCTL_SETIFG` writer"]
pub struct W(crate::W<FLCTL_SETIFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLCTL_SETIFG_SPEC>;
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
impl From<crate::W<FLCTL_SETIFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLCTL_SETIFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDBRST` writer - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
pub type RDBRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLCTL_SETIFG_SPEC, bool, O>;
#[doc = "Field `AVPRE` writer - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
pub type AVPRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLCTL_SETIFG_SPEC, bool, O>;
#[doc = "Field `AVPST` writer - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
pub type AVPST_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLCTL_SETIFG_SPEC, bool, O>;
#[doc = "Field `PRG` writer - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
pub type PRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLCTL_SETIFG_SPEC, bool, O>;
#[doc = "Field `PRGB` writer - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
pub type PRGB_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLCTL_SETIFG_SPEC, bool, O>;
#[doc = "Field `ERASE` writer - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
pub type ERASE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLCTL_SETIFG_SPEC, bool, O>;
#[doc = "Field `BMRK` writer - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
pub type BMRK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLCTL_SETIFG_SPEC, bool, O>;
#[doc = "Field `PRG_ERR` writer - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
pub type PRG_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLCTL_SETIFG_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn rdbrst(&mut self) -> RDBRST_W<0> {
        RDBRST_W::new(self)
    }
    #[doc = "Bit 1 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn avpre(&mut self) -> AVPRE_W<1> {
        AVPRE_W::new(self)
    }
    #[doc = "Bit 2 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn avpst(&mut self) -> AVPST_W<2> {
        AVPST_W::new(self)
    }
    #[doc = "Bit 3 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prg(&mut self) -> PRG_W<3> {
        PRG_W::new(self)
    }
    #[doc = "Bit 4 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prgb(&mut self) -> PRGB_W<4> {
        PRGB_W::new(self)
    }
    #[doc = "Bit 5 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn erase(&mut self) -> ERASE_W<5> {
        ERASE_W::new(self)
    }
    #[doc = "Bit 8 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn bmrk(&mut self) -> BMRK_W<8> {
        BMRK_W::new(self)
    }
    #[doc = "Bit 9 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prg_err(&mut self) -> PRG_ERR_W<9> {
        PRG_ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set Interrupt Flag Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_setifg](index.html) module"]
pub struct FLCTL_SETIFG_SPEC;
impl crate::RegisterSpec for FLCTL_SETIFG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [flctl_setifg::W](W) writer structure"]
impl crate::Writable for FLCTL_SETIFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLCTL_SETIFG to value 0"]
impl crate::Resettable for FLCTL_SETIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
