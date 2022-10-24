#[doc = "Register `LCDSETIFG` writer"]
pub struct W(crate::W<LCDSETIFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDSETIFG_SPEC>;
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
impl From<crate::W<LCDSETIFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCDSETIFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Sets LCDBLKOFFIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETLCDBLKOFFIFG_AW {
    #[doc = "0: No effect"]
    SETLCDBLKOFFIFG_0 = 0,
    #[doc = "1: Sets interrupt flag"]
    SETLCDBLKOFFIFG_1 = 1,
}
impl From<SETLCDBLKOFFIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: SETLCDBLKOFFIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETLCDBLKOFFIFG` writer - Sets LCDBLKOFFIFG"]
pub type SETLCDBLKOFFIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCDSETIFG_SPEC, SETLCDBLKOFFIFG_AW, O>;
impl<'a, const O: u8> SETLCDBLKOFFIFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn setlcdblkoffifg_0(self) -> &'a mut W {
        self.variant(SETLCDBLKOFFIFG_AW::SETLCDBLKOFFIFG_0)
    }
    #[doc = "Sets interrupt flag"]
    #[inline(always)]
    pub fn setlcdblkoffifg_1(self) -> &'a mut W {
        self.variant(SETLCDBLKOFFIFG_AW::SETLCDBLKOFFIFG_1)
    }
}
#[doc = "Sets LCDBLKONIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETLCDBLKONIFG_AW {
    #[doc = "0: No effect"]
    SETLCDBLKONIFG_0 = 0,
    #[doc = "1: Sets interrupt flag"]
    SETLCDBLKONIFG_1 = 1,
}
impl From<SETLCDBLKONIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: SETLCDBLKONIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETLCDBLKONIFG` writer - Sets LCDBLKONIFG"]
pub type SETLCDBLKONIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCDSETIFG_SPEC, SETLCDBLKONIFG_AW, O>;
impl<'a, const O: u8> SETLCDBLKONIFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn setlcdblkonifg_0(self) -> &'a mut W {
        self.variant(SETLCDBLKONIFG_AW::SETLCDBLKONIFG_0)
    }
    #[doc = "Sets interrupt flag"]
    #[inline(always)]
    pub fn setlcdblkonifg_1(self) -> &'a mut W {
        self.variant(SETLCDBLKONIFG_AW::SETLCDBLKONIFG_1)
    }
}
#[doc = "Sets LCDFRMIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETLCDFRMIFG_AW {
    #[doc = "0: No effect"]
    SETLCDFRMIFG_0 = 0,
    #[doc = "1: Sets interrupt flag"]
    SETLCDFRMIFG_1 = 1,
}
impl From<SETLCDFRMIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: SETLCDFRMIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETLCDFRMIFG` writer - Sets LCDFRMIFG"]
pub type SETLCDFRMIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCDSETIFG_SPEC, SETLCDFRMIFG_AW, O>;
impl<'a, const O: u8> SETLCDFRMIFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn setlcdfrmifg_0(self) -> &'a mut W {
        self.variant(SETLCDFRMIFG_AW::SETLCDFRMIFG_0)
    }
    #[doc = "Sets interrupt flag"]
    #[inline(always)]
    pub fn setlcdfrmifg_1(self) -> &'a mut W {
        self.variant(SETLCDFRMIFG_AW::SETLCDFRMIFG_1)
    }
}
#[doc = "Sets LCDANMSTPIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETLCDANMSTPIFG_AW {
    #[doc = "0: No effect"]
    SETLCDANMSTPIFG_0 = 0,
    #[doc = "1: Sets interrupt flag"]
    SETLCDANMSTPIFG_1 = 1,
}
impl From<SETLCDANMSTPIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: SETLCDANMSTPIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETLCDANMSTPIFG` writer - Sets LCDANMSTPIFG"]
pub type SETLCDANMSTPIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCDSETIFG_SPEC, SETLCDANMSTPIFG_AW, O>;
impl<'a, const O: u8> SETLCDANMSTPIFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn setlcdanmstpifg_0(self) -> &'a mut W {
        self.variant(SETLCDANMSTPIFG_AW::SETLCDANMSTPIFG_0)
    }
    #[doc = "Sets interrupt flag"]
    #[inline(always)]
    pub fn setlcdanmstpifg_1(self) -> &'a mut W {
        self.variant(SETLCDANMSTPIFG_AW::SETLCDANMSTPIFG_1)
    }
}
#[doc = "Sets LCDANMLOOPIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETLCDANMLOOPIFG_AW {
    #[doc = "0: No effect"]
    SETLCDANMLOOPIFG_0 = 0,
    #[doc = "1: Sets interrupt flag"]
    SETLCDANMLOOPIFG_1 = 1,
}
impl From<SETLCDANMLOOPIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: SETLCDANMLOOPIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SETLCDANMLOOPIFG` writer - Sets LCDANMLOOPIFG"]
pub type SETLCDANMLOOPIFG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LCDSETIFG_SPEC, SETLCDANMLOOPIFG_AW, O>;
impl<'a, const O: u8> SETLCDANMLOOPIFG_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn setlcdanmloopifg_0(self) -> &'a mut W {
        self.variant(SETLCDANMLOOPIFG_AW::SETLCDANMLOOPIFG_0)
    }
    #[doc = "Sets interrupt flag"]
    #[inline(always)]
    pub fn setlcdanmloopifg_1(self) -> &'a mut W {
        self.variant(SETLCDANMLOOPIFG_AW::SETLCDANMLOOPIFG_1)
    }
}
impl W {
    #[doc = "Bit 1 - Sets LCDBLKOFFIFG"]
    #[inline(always)]
    pub fn setlcdblkoffifg(&mut self) -> SETLCDBLKOFFIFG_W<1> {
        SETLCDBLKOFFIFG_W::new(self)
    }
    #[doc = "Bit 2 - Sets LCDBLKONIFG"]
    #[inline(always)]
    pub fn setlcdblkonifg(&mut self) -> SETLCDBLKONIFG_W<2> {
        SETLCDBLKONIFG_W::new(self)
    }
    #[doc = "Bit 3 - Sets LCDFRMIFG"]
    #[inline(always)]
    pub fn setlcdfrmifg(&mut self) -> SETLCDFRMIFG_W<3> {
        SETLCDFRMIFG_W::new(self)
    }
    #[doc = "Bit 8 - Sets LCDANMSTPIFG"]
    #[inline(always)]
    pub fn setlcdanmstpifg(&mut self) -> SETLCDANMSTPIFG_W<8> {
        SETLCDANMSTPIFG_W::new(self)
    }
    #[doc = "Bit 9 - Sets LCDANMLOOPIFG"]
    #[inline(always)]
    pub fn setlcdanmloopifg(&mut self) -> SETLCDANMLOOPIFG_W<9> {
        SETLCDANMLOOPIFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_F set interrupt flag register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdsetifg](index.html) module"]
pub struct LCDSETIFG_SPEC;
impl crate::RegisterSpec for LCDSETIFG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [lcdsetifg::W](W) writer structure"]
impl crate::Writable for LCDSETIFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDSETIFG to value 0"]
impl crate::Resettable for LCDSETIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
