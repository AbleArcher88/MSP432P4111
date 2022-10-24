#[doc = "Register `LCDCLRIFG` writer"]
pub struct W(crate::W<LCDCLRIFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDCLRIFG_SPEC>;
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
impl From<crate::W<LCDCLRIFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCDCLRIFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clears LCDBLKOFFIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRLCDBLKOFFIFG_AW {
    #[doc = "0: No effect"]
    CLRLCDBLKOFFIFG_0 = 0,
    #[doc = "1: Clears pending interrupt flag"]
    CLRLCDBLKOFFIFG_1 = 1,
}
impl From<CLRLCDBLKOFFIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRLCDBLKOFFIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRLCDBLKOFFIFG` writer - Clears LCDBLKOFFIFG"]
pub type CLRLCDBLKOFFIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCDCLRIFG_SPEC, CLRLCDBLKOFFIFG_AW, O>;
impl<'a, const O: u8> CLRLCDBLKOFFIFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clrlcdblkoffifg_0(self) -> &'a mut W {
        self.variant(CLRLCDBLKOFFIFG_AW::CLRLCDBLKOFFIFG_0)
    }
    #[doc = "Clears pending interrupt flag"]
    #[inline(always)]
    pub fn clrlcdblkoffifg_1(self) -> &'a mut W {
        self.variant(CLRLCDBLKOFFIFG_AW::CLRLCDBLKOFFIFG_1)
    }
}
#[doc = "Clears LCDBLKONIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRLCDBLKONIFG_AW {
    #[doc = "0: No effect"]
    CLRLCDBLKONIFG_0 = 0,
    #[doc = "1: Clears pending interrupt flag"]
    CLRLCDBLKONIFG_1 = 1,
}
impl From<CLRLCDBLKONIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRLCDBLKONIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRLCDBLKONIFG` writer - Clears LCDBLKONIFG"]
pub type CLRLCDBLKONIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCDCLRIFG_SPEC, CLRLCDBLKONIFG_AW, O>;
impl<'a, const O: u8> CLRLCDBLKONIFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clrlcdblkonifg_0(self) -> &'a mut W {
        self.variant(CLRLCDBLKONIFG_AW::CLRLCDBLKONIFG_0)
    }
    #[doc = "Clears pending interrupt flag"]
    #[inline(always)]
    pub fn clrlcdblkonifg_1(self) -> &'a mut W {
        self.variant(CLRLCDBLKONIFG_AW::CLRLCDBLKONIFG_1)
    }
}
#[doc = "Clears LCDFRMIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRLCDFRMIFG_AW {
    #[doc = "0: No effect"]
    CLRLCDFRMIFG_0 = 0,
    #[doc = "1: Clears pending interrupt flag"]
    CLRLCDFRMIFG_1 = 1,
}
impl From<CLRLCDFRMIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRLCDFRMIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRLCDFRMIFG` writer - Clears LCDFRMIFG"]
pub type CLRLCDFRMIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCDCLRIFG_SPEC, CLRLCDFRMIFG_AW, O>;
impl<'a, const O: u8> CLRLCDFRMIFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clrlcdfrmifg_0(self) -> &'a mut W {
        self.variant(CLRLCDFRMIFG_AW::CLRLCDFRMIFG_0)
    }
    #[doc = "Clears pending interrupt flag"]
    #[inline(always)]
    pub fn clrlcdfrmifg_1(self) -> &'a mut W {
        self.variant(CLRLCDFRMIFG_AW::CLRLCDFRMIFG_1)
    }
}
#[doc = "Clears LCDANMSTPIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRLCDANMSTPIFG_AW {
    #[doc = "0: No effect"]
    CLRLCDANMSTPIFG_0 = 0,
    #[doc = "1: Clears pending interrupt flag"]
    CLRLCDANMSTPIFG_1 = 1,
}
impl From<CLRLCDANMSTPIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRLCDANMSTPIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRLCDANMSTPIFG` writer - Clears LCDANMSTPIFG"]
pub type CLRLCDANMSTPIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCDCLRIFG_SPEC, CLRLCDANMSTPIFG_AW, O>;
impl<'a, const O: u8> CLRLCDANMSTPIFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clrlcdanmstpifg_0(self) -> &'a mut W {
        self.variant(CLRLCDANMSTPIFG_AW::CLRLCDANMSTPIFG_0)
    }
    #[doc = "Clears pending interrupt flag"]
    #[inline(always)]
    pub fn clrlcdanmstpifg_1(self) -> &'a mut W {
        self.variant(CLRLCDANMSTPIFG_AW::CLRLCDANMSTPIFG_1)
    }
}
#[doc = "Clears LCDANMLOOPIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRLCDANMLOOPIFG_AW {
    #[doc = "0: No effect"]
    CLRLCDANMLOOPIFG_0 = 0,
    #[doc = "1: Clears pending interrupt flag"]
    CLRLCDANMLOOPIFG_1 = 1,
}
impl From<CLRLCDANMLOOPIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRLCDANMLOOPIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRLCDANMLOOPIFG` writer - Clears LCDANMLOOPIFG"]
pub type CLRLCDANMLOOPIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCDCLRIFG_SPEC, CLRLCDANMLOOPIFG_AW, O>;
impl<'a, const O: u8> CLRLCDANMLOOPIFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clrlcdanmloopifg_0(self) -> &'a mut W {
        self.variant(CLRLCDANMLOOPIFG_AW::CLRLCDANMLOOPIFG_0)
    }
    #[doc = "Clears pending interrupt flag"]
    #[inline(always)]
    pub fn clrlcdanmloopifg_1(self) -> &'a mut W {
        self.variant(CLRLCDANMLOOPIFG_AW::CLRLCDANMLOOPIFG_1)
    }
}
impl W {
    #[doc = "Bit 1 - Clears LCDBLKOFFIFG"]
    #[inline(always)]
    pub fn clrlcdblkoffifg(&mut self) -> CLRLCDBLKOFFIFG_W<1> {
        CLRLCDBLKOFFIFG_W::new(self)
    }
    #[doc = "Bit 2 - Clears LCDBLKONIFG"]
    #[inline(always)]
    pub fn clrlcdblkonifg(&mut self) -> CLRLCDBLKONIFG_W<2> {
        CLRLCDBLKONIFG_W::new(self)
    }
    #[doc = "Bit 3 - Clears LCDFRMIFG"]
    #[inline(always)]
    pub fn clrlcdfrmifg(&mut self) -> CLRLCDFRMIFG_W<3> {
        CLRLCDFRMIFG_W::new(self)
    }
    #[doc = "Bit 8 - Clears LCDANMSTPIFG"]
    #[inline(always)]
    pub fn clrlcdanmstpifg(&mut self) -> CLRLCDANMSTPIFG_W<8> {
        CLRLCDANMSTPIFG_W::new(self)
    }
    #[doc = "Bit 9 - Clears LCDANMLOOPIFG"]
    #[inline(always)]
    pub fn clrlcdanmloopifg(&mut self) -> CLRLCDANMLOOPIFG_W<9> {
        CLRLCDANMLOOPIFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_F clear interrupt flag register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdclrifg](index.html) module"]
pub struct LCDCLRIFG_SPEC;
impl crate::RegisterSpec for LCDCLRIFG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [lcdclrifg::W](W) writer structure"]
impl crate::Writable for LCDCLRIFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDCLRIFG to value 0"]
impl crate::Resettable for LCDCLRIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
