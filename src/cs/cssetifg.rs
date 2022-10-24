#[doc = "Register `CSSETIFG` writer"]
pub struct W(crate::W<CSSETIFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSSETIFG_SPEC>;
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
impl From<crate::W<CSSETIFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSSETIFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Set LFXT oscillator fault interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SET_LFXTIFG_ENUM_WRITE_AW {
    #[doc = "0: No effect"]
    SET_LFXTIFG_0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SET_LFXTIFG_1 = 1,
}
impl From<SET_LFXTIFG_ENUM_WRITE_AW> for bool {
    #[inline(always)]
    fn from(variant: SET_LFXTIFG_ENUM_WRITE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_LFXTIFG` writer - Set LFXT oscillator fault interrupt flag"]
pub type SET_LFXTIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSSETIFG_SPEC, SET_LFXTIFG_ENUM_WRITE_AW, O>;
impl<'a, const O: u8> SET_LFXTIFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_lfxtifg_0(self) -> &'a mut W {
        self.variant(SET_LFXTIFG_ENUM_WRITE_AW::SET_LFXTIFG_0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_lfxtifg_1(self) -> &'a mut W {
        self.variant(SET_LFXTIFG_ENUM_WRITE_AW::SET_LFXTIFG_1)
    }
}
#[doc = "Set HFXT oscillator fault interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SET_HFXTIFG_ENUM_WRITE_AW {
    #[doc = "0: No effect"]
    SET_HFXTIFG_0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SET_HFXTIFG_1 = 1,
}
impl From<SET_HFXTIFG_ENUM_WRITE_AW> for bool {
    #[inline(always)]
    fn from(variant: SET_HFXTIFG_ENUM_WRITE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_HFXTIFG` writer - Set HFXT oscillator fault interrupt flag"]
pub type SET_HFXTIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSSETIFG_SPEC, SET_HFXTIFG_ENUM_WRITE_AW, O>;
impl<'a, const O: u8> SET_HFXTIFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_hfxtifg_0(self) -> &'a mut W {
        self.variant(SET_HFXTIFG_ENUM_WRITE_AW::SET_HFXTIFG_0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_hfxtifg_1(self) -> &'a mut W {
        self.variant(SET_HFXTIFG_ENUM_WRITE_AW::SET_HFXTIFG_1)
    }
}
#[doc = "Set HFXT2 oscillator fault interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SET_HFXT2IFG_ENUM_WRITE_AW {
    #[doc = "0: No effect"]
    SET_HFXT2IFG_0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SET_HFXT2IFG_1 = 1,
}
impl From<SET_HFXT2IFG_ENUM_WRITE_AW> for bool {
    #[inline(always)]
    fn from(variant: SET_HFXT2IFG_ENUM_WRITE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_HFXT2IFG` writer - Set HFXT2 oscillator fault interrupt flag"]
pub type SET_HFXT2IFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSSETIFG_SPEC, SET_HFXT2IFG_ENUM_WRITE_AW, O>;
impl<'a, const O: u8> SET_HFXT2IFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_hfxt2ifg_0(self) -> &'a mut W {
        self.variant(SET_HFXT2IFG_ENUM_WRITE_AW::SET_HFXT2IFG_0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_hfxt2ifg_1(self) -> &'a mut W {
        self.variant(SET_HFXT2IFG_ENUM_WRITE_AW::SET_HFXT2IFG_1)
    }
}
#[doc = "Set DCO external resistor open circuit fault interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SET_DCOR_OPNIFG_ENUM_WRITE_AW {
    #[doc = "0: No effect"]
    SET_DCOR_OPNIFG_0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SET_DCOR_OPNIFG_1 = 1,
}
impl From<SET_DCOR_OPNIFG_ENUM_WRITE_AW> for bool {
    #[inline(always)]
    fn from(variant: SET_DCOR_OPNIFG_ENUM_WRITE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_DCOR_OPNIFG` writer - Set DCO external resistor open circuit fault interrupt flag."]
pub type SET_DCOR_OPNIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSSETIFG_SPEC, SET_DCOR_OPNIFG_ENUM_WRITE_AW, O>;
impl<'a, const O: u8> SET_DCOR_OPNIFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_dcor_opnifg_0(self) -> &'a mut W {
        self.variant(SET_DCOR_OPNIFG_ENUM_WRITE_AW::SET_DCOR_OPNIFG_0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_dcor_opnifg_1(self) -> &'a mut W {
        self.variant(SET_DCOR_OPNIFG_ENUM_WRITE_AW::SET_DCOR_OPNIFG_1)
    }
}
#[doc = "Start fault counter set interrupt flag LFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SET_FCNTLFIFG_ENUM_WRITE_AW {
    #[doc = "0: No effect"]
    SET_FCNTLFIFG_0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SET_FCNTLFIFG_1 = 1,
}
impl From<SET_FCNTLFIFG_ENUM_WRITE_AW> for bool {
    #[inline(always)]
    fn from(variant: SET_FCNTLFIFG_ENUM_WRITE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_FCNTLFIFG` writer - Start fault counter set interrupt flag LFXT"]
pub type SET_FCNTLFIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSSETIFG_SPEC, SET_FCNTLFIFG_ENUM_WRITE_AW, O>;
impl<'a, const O: u8> SET_FCNTLFIFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_fcntlfifg_0(self) -> &'a mut W {
        self.variant(SET_FCNTLFIFG_ENUM_WRITE_AW::SET_FCNTLFIFG_0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_fcntlfifg_1(self) -> &'a mut W {
        self.variant(SET_FCNTLFIFG_ENUM_WRITE_AW::SET_FCNTLFIFG_1)
    }
}
#[doc = "Start fault counter set interrupt flag HFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SET_FCNTHFIFG_ENUM_WRITE_AW {
    #[doc = "0: No effect"]
    SET_FCNTHFIFG_0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SET_FCNTHFIFG_1 = 1,
}
impl From<SET_FCNTHFIFG_ENUM_WRITE_AW> for bool {
    #[inline(always)]
    fn from(variant: SET_FCNTHFIFG_ENUM_WRITE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_FCNTHFIFG` writer - Start fault counter set interrupt flag HFXT"]
pub type SET_FCNTHFIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSSETIFG_SPEC, SET_FCNTHFIFG_ENUM_WRITE_AW, O>;
impl<'a, const O: u8> SET_FCNTHFIFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_fcnthfifg_0(self) -> &'a mut W {
        self.variant(SET_FCNTHFIFG_ENUM_WRITE_AW::SET_FCNTHFIFG_0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_fcnthfifg_1(self) -> &'a mut W {
        self.variant(SET_FCNTHFIFG_ENUM_WRITE_AW::SET_FCNTHFIFG_1)
    }
}
#[doc = "Start fault counter set interrupt flag HFXT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SET_FCNTHF2IFG_ENUM_WRITE_AW {
    #[doc = "0: No effect"]
    SET_FCNTHF2IFG_0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SET_FCNTHF2IFG_1 = 1,
}
impl From<SET_FCNTHF2IFG_ENUM_WRITE_AW> for bool {
    #[inline(always)]
    fn from(variant: SET_FCNTHF2IFG_ENUM_WRITE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_FCNTHF2IFG` writer - Start fault counter set interrupt flag HFXT2"]
pub type SET_FCNTHF2IFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSSETIFG_SPEC, SET_FCNTHF2IFG_ENUM_WRITE_AW, O>;
impl<'a, const O: u8> SET_FCNTHF2IFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_fcnthf2ifg_0(self) -> &'a mut W {
        self.variant(SET_FCNTHF2IFG_ENUM_WRITE_AW::SET_FCNTHF2IFG_0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_fcnthf2ifg_1(self) -> &'a mut W {
        self.variant(SET_FCNTHF2IFG_ENUM_WRITE_AW::SET_FCNTHF2IFG_1)
    }
}
#[doc = "PLL out-of-lock set interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SET_PLLOOLIFG_ENUM_WRITE_AW {
    #[doc = "0: No effect"]
    SET_PLLOOLIFG_0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SET_PLLOOLIFG_1 = 1,
}
impl From<SET_PLLOOLIFG_ENUM_WRITE_AW> for bool {
    #[inline(always)]
    fn from(variant: SET_PLLOOLIFG_ENUM_WRITE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_PLLOOLIFG` writer - PLL out-of-lock set interrupt flag"]
pub type SET_PLLOOLIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSSETIFG_SPEC, SET_PLLOOLIFG_ENUM_WRITE_AW, O>;
impl<'a, const O: u8> SET_PLLOOLIFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_plloolifg_0(self) -> &'a mut W {
        self.variant(SET_PLLOOLIFG_ENUM_WRITE_AW::SET_PLLOOLIFG_0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_plloolifg_1(self) -> &'a mut W {
        self.variant(SET_PLLOOLIFG_ENUM_WRITE_AW::SET_PLLOOLIFG_1)
    }
}
#[doc = "PLL loss-of-signal set interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SET_PLLLOSIFG_ENUM_WRITE_AW {
    #[doc = "0: No effect"]
    SET_PLLLOSIFG_0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SET_PLLLOSIFG_1 = 1,
}
impl From<SET_PLLLOSIFG_ENUM_WRITE_AW> for bool {
    #[inline(always)]
    fn from(variant: SET_PLLLOSIFG_ENUM_WRITE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_PLLLOSIFG` writer - PLL loss-of-signal set interrupt flag"]
pub type SET_PLLLOSIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSSETIFG_SPEC, SET_PLLLOSIFG_ENUM_WRITE_AW, O>;
impl<'a, const O: u8> SET_PLLLOSIFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_plllosifg_0(self) -> &'a mut W {
        self.variant(SET_PLLLOSIFG_ENUM_WRITE_AW::SET_PLLLOSIFG_0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_plllosifg_1(self) -> &'a mut W {
        self.variant(SET_PLLLOSIFG_ENUM_WRITE_AW::SET_PLLLOSIFG_1)
    }
}
#[doc = "PLL out-of-range set interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SET_PLLOORIFG_ENUM_WRITE_AW {
    #[doc = "0: No effect"]
    SET_PLLOORIFG_0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SET_PLLOORIFG_1 = 1,
}
impl From<SET_PLLOORIFG_ENUM_WRITE_AW> for bool {
    #[inline(always)]
    fn from(variant: SET_PLLOORIFG_ENUM_WRITE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_PLLOORIFG` writer - PLL out-of-range set interrupt flag"]
pub type SET_PLLOORIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSSETIFG_SPEC, SET_PLLOORIFG_ENUM_WRITE_AW, O>;
impl<'a, const O: u8> SET_PLLOORIFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_plloorifg_0(self) -> &'a mut W {
        self.variant(SET_PLLOORIFG_ENUM_WRITE_AW::SET_PLLOORIFG_0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_plloorifg_1(self) -> &'a mut W {
        self.variant(SET_PLLOORIFG_ENUM_WRITE_AW::SET_PLLOORIFG_1)
    }
}
#[doc = "REFCNT period counter set interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SET_CALIFG_ENUM_WRITE_AW {
    #[doc = "0: No effect"]
    SET_CALIFG_0 = 0,
    #[doc = "1: Set pending interrupt flag"]
    SET_CALIFG_1 = 1,
}
impl From<SET_CALIFG_ENUM_WRITE_AW> for bool {
    #[inline(always)]
    fn from(variant: SET_CALIFG_ENUM_WRITE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SET_CALIFG` writer - REFCNT period counter set interrupt flag"]
pub type SET_CALIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CSSETIFG_SPEC, SET_CALIFG_ENUM_WRITE_AW, O>;
impl<'a, const O: u8> SET_CALIFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn set_califg_0(self) -> &'a mut W {
        self.variant(SET_CALIFG_ENUM_WRITE_AW::SET_CALIFG_0)
    }
    #[doc = "Set pending interrupt flag"]
    #[inline(always)]
    pub fn set_califg_1(self) -> &'a mut W {
        self.variant(SET_CALIFG_ENUM_WRITE_AW::SET_CALIFG_1)
    }
}
impl W {
    #[doc = "Bit 0 - Set LFXT oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn set_lfxtifg(&mut self) -> SET_LFXTIFG_W<0> {
        SET_LFXTIFG_W::new(self)
    }
    #[doc = "Bit 1 - Set HFXT oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn set_hfxtifg(&mut self) -> SET_HFXTIFG_W<1> {
        SET_HFXTIFG_W::new(self)
    }
    #[doc = "Bit 2 - Set HFXT2 oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn set_hfxt2ifg(&mut self) -> SET_HFXT2IFG_W<2> {
        SET_HFXT2IFG_W::new(self)
    }
    #[doc = "Bit 6 - Set DCO external resistor open circuit fault interrupt flag."]
    #[inline(always)]
    pub fn set_dcor_opnifg(&mut self) -> SET_DCOR_OPNIFG_W<6> {
        SET_DCOR_OPNIFG_W::new(self)
    }
    #[doc = "Bit 8 - Start fault counter set interrupt flag LFXT"]
    #[inline(always)]
    pub fn set_fcntlfifg(&mut self) -> SET_FCNTLFIFG_W<8> {
        SET_FCNTLFIFG_W::new(self)
    }
    #[doc = "Bit 9 - Start fault counter set interrupt flag HFXT"]
    #[inline(always)]
    pub fn set_fcnthfifg(&mut self) -> SET_FCNTHFIFG_W<9> {
        SET_FCNTHFIFG_W::new(self)
    }
    #[doc = "Bit 10 - Start fault counter set interrupt flag HFXT2"]
    #[inline(always)]
    pub fn set_fcnthf2ifg(&mut self) -> SET_FCNTHF2IFG_W<10> {
        SET_FCNTHF2IFG_W::new(self)
    }
    #[doc = "Bit 12 - PLL out-of-lock set interrupt flag"]
    #[inline(always)]
    pub fn set_plloolifg(&mut self) -> SET_PLLOOLIFG_W<12> {
        SET_PLLOOLIFG_W::new(self)
    }
    #[doc = "Bit 13 - PLL loss-of-signal set interrupt flag"]
    #[inline(always)]
    pub fn set_plllosifg(&mut self) -> SET_PLLLOSIFG_W<13> {
        SET_PLLLOSIFG_W::new(self)
    }
    #[doc = "Bit 14 - PLL out-of-range set interrupt flag"]
    #[inline(always)]
    pub fn set_plloorifg(&mut self) -> SET_PLLOORIFG_W<14> {
        SET_PLLOORIFG_W::new(self)
    }
    #[doc = "Bit 15 - REFCNT period counter set interrupt flag"]
    #[inline(always)]
    pub fn set_califg(&mut self) -> SET_CALIFG_W<15> {
        SET_CALIFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set Interrupt Flag Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cssetifg](index.html) module"]
pub struct CSSETIFG_SPEC;
impl crate::RegisterSpec for CSSETIFG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cssetifg::W](W) writer structure"]
impl crate::Writable for CSSETIFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSSETIFG to value 0"]
impl crate::Resettable for CSSETIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
