#[doc = "Register `LCDIE` reader"]
pub struct R(crate::R<LCDIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCDIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCDIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCDIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCDIE` writer"]
pub struct W(crate::W<LCDIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCDIE_SPEC>;
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
impl From<crate::W<LCDIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCDIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LCDBLKOFFIE` reader - LCD Blink, segments off interrupt enable"]
pub type LCDBLKOFFIE_R = crate::BitReader<LCDBLKOFFIE_A>;
#[doc = "LCD Blink, segments off interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDBLKOFFIE_A {
    #[doc = "0: Interrupt disabled"]
    LCDBLKOFFIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    LCDBLKOFFIE_1 = 1,
}
impl From<LCDBLKOFFIE_A> for bool {
    #[inline(always)]
    fn from(variant: LCDBLKOFFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDBLKOFFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDBLKOFFIE_A {
        match self.bits {
            false => LCDBLKOFFIE_A::LCDBLKOFFIE_0,
            true => LCDBLKOFFIE_A::LCDBLKOFFIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDBLKOFFIE_0`"]
    #[inline(always)]
    pub fn is_lcdblkoffie_0(&self) -> bool {
        *self == LCDBLKOFFIE_A::LCDBLKOFFIE_0
    }
    #[doc = "Checks if the value of the field is `LCDBLKOFFIE_1`"]
    #[inline(always)]
    pub fn is_lcdblkoffie_1(&self) -> bool {
        *self == LCDBLKOFFIE_A::LCDBLKOFFIE_1
    }
}
#[doc = "Field `LCDBLKOFFIE` writer - LCD Blink, segments off interrupt enable"]
pub type LCDBLKOFFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDIE_SPEC, LCDBLKOFFIE_A, O>;
impl<'a, const O: u8> LCDBLKOFFIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn lcdblkoffie_0(self) -> &'a mut W {
        self.variant(LCDBLKOFFIE_A::LCDBLKOFFIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn lcdblkoffie_1(self) -> &'a mut W {
        self.variant(LCDBLKOFFIE_A::LCDBLKOFFIE_1)
    }
}
#[doc = "Field `LCDBLKONIE` reader - LCD Blink, segments on interrupt enable"]
pub type LCDBLKONIE_R = crate::BitReader<LCDBLKONIE_A>;
#[doc = "LCD Blink, segments on interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDBLKONIE_A {
    #[doc = "0: Interrupt disabled"]
    LCDBLKONIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    LCDBLKONIE_1 = 1,
}
impl From<LCDBLKONIE_A> for bool {
    #[inline(always)]
    fn from(variant: LCDBLKONIE_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDBLKONIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDBLKONIE_A {
        match self.bits {
            false => LCDBLKONIE_A::LCDBLKONIE_0,
            true => LCDBLKONIE_A::LCDBLKONIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDBLKONIE_0`"]
    #[inline(always)]
    pub fn is_lcdblkonie_0(&self) -> bool {
        *self == LCDBLKONIE_A::LCDBLKONIE_0
    }
    #[doc = "Checks if the value of the field is `LCDBLKONIE_1`"]
    #[inline(always)]
    pub fn is_lcdblkonie_1(&self) -> bool {
        *self == LCDBLKONIE_A::LCDBLKONIE_1
    }
}
#[doc = "Field `LCDBLKONIE` writer - LCD Blink, segments on interrupt enable"]
pub type LCDBLKONIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDIE_SPEC, LCDBLKONIE_A, O>;
impl<'a, const O: u8> LCDBLKONIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn lcdblkonie_0(self) -> &'a mut W {
        self.variant(LCDBLKONIE_A::LCDBLKONIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn lcdblkonie_1(self) -> &'a mut W {
        self.variant(LCDBLKONIE_A::LCDBLKONIE_1)
    }
}
#[doc = "Field `LCDFRMIE` reader - LCD Frame interrupt enable"]
pub type LCDFRMIE_R = crate::BitReader<LCDFRMIE_A>;
#[doc = "LCD Frame interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDFRMIE_A {
    #[doc = "0: Interrupt disabled"]
    LCDFRMIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    LCDFRMIE_1 = 1,
}
impl From<LCDFRMIE_A> for bool {
    #[inline(always)]
    fn from(variant: LCDFRMIE_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDFRMIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDFRMIE_A {
        match self.bits {
            false => LCDFRMIE_A::LCDFRMIE_0,
            true => LCDFRMIE_A::LCDFRMIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDFRMIE_0`"]
    #[inline(always)]
    pub fn is_lcdfrmie_0(&self) -> bool {
        *self == LCDFRMIE_A::LCDFRMIE_0
    }
    #[doc = "Checks if the value of the field is `LCDFRMIE_1`"]
    #[inline(always)]
    pub fn is_lcdfrmie_1(&self) -> bool {
        *self == LCDFRMIE_A::LCDFRMIE_1
    }
}
#[doc = "Field `LCDFRMIE` writer - LCD Frame interrupt enable"]
pub type LCDFRMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDIE_SPEC, LCDFRMIE_A, O>;
impl<'a, const O: u8> LCDFRMIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn lcdfrmie_0(self) -> &'a mut W {
        self.variant(LCDFRMIE_A::LCDFRMIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn lcdfrmie_1(self) -> &'a mut W {
        self.variant(LCDFRMIE_A::LCDFRMIE_1)
    }
}
#[doc = "Field `LCDANMSTPIE` reader - LCD Animation step interrupt enable"]
pub type LCDANMSTPIE_R = crate::BitReader<LCDANMSTPIE_A>;
#[doc = "LCD Animation step interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDANMSTPIE_A {
    #[doc = "0: Interrupt disabled"]
    LCDANMSTPIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    LCDANMSTPIE_1 = 1,
}
impl From<LCDANMSTPIE_A> for bool {
    #[inline(always)]
    fn from(variant: LCDANMSTPIE_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDANMSTPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDANMSTPIE_A {
        match self.bits {
            false => LCDANMSTPIE_A::LCDANMSTPIE_0,
            true => LCDANMSTPIE_A::LCDANMSTPIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDANMSTPIE_0`"]
    #[inline(always)]
    pub fn is_lcdanmstpie_0(&self) -> bool {
        *self == LCDANMSTPIE_A::LCDANMSTPIE_0
    }
    #[doc = "Checks if the value of the field is `LCDANMSTPIE_1`"]
    #[inline(always)]
    pub fn is_lcdanmstpie_1(&self) -> bool {
        *self == LCDANMSTPIE_A::LCDANMSTPIE_1
    }
}
#[doc = "Field `LCDANMSTPIE` writer - LCD Animation step interrupt enable"]
pub type LCDANMSTPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDIE_SPEC, LCDANMSTPIE_A, O>;
impl<'a, const O: u8> LCDANMSTPIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn lcdanmstpie_0(self) -> &'a mut W {
        self.variant(LCDANMSTPIE_A::LCDANMSTPIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn lcdanmstpie_1(self) -> &'a mut W {
        self.variant(LCDANMSTPIE_A::LCDANMSTPIE_1)
    }
}
#[doc = "Field `LCDANMLOOPIE` reader - LCD Animation loop interrupt enable"]
pub type LCDANMLOOPIE_R = crate::BitReader<LCDANMLOOPIE_A>;
#[doc = "LCD Animation loop interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LCDANMLOOPIE_A {
    #[doc = "0: Interrupt disabled"]
    LCDANMLOOPIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    LCDANMLOOPIE_1 = 1,
}
impl From<LCDANMLOOPIE_A> for bool {
    #[inline(always)]
    fn from(variant: LCDANMLOOPIE_A) -> Self {
        variant as u8 != 0
    }
}
impl LCDANMLOOPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCDANMLOOPIE_A {
        match self.bits {
            false => LCDANMLOOPIE_A::LCDANMLOOPIE_0,
            true => LCDANMLOOPIE_A::LCDANMLOOPIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LCDANMLOOPIE_0`"]
    #[inline(always)]
    pub fn is_lcdanmloopie_0(&self) -> bool {
        *self == LCDANMLOOPIE_A::LCDANMLOOPIE_0
    }
    #[doc = "Checks if the value of the field is `LCDANMLOOPIE_1`"]
    #[inline(always)]
    pub fn is_lcdanmloopie_1(&self) -> bool {
        *self == LCDANMLOOPIE_A::LCDANMLOOPIE_1
    }
}
#[doc = "Field `LCDANMLOOPIE` writer - LCD Animation loop interrupt enable"]
pub type LCDANMLOOPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCDIE_SPEC, LCDANMLOOPIE_A, O>;
impl<'a, const O: u8> LCDANMLOOPIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn lcdanmloopie_0(self) -> &'a mut W {
        self.variant(LCDANMLOOPIE_A::LCDANMLOOPIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn lcdanmloopie_1(self) -> &'a mut W {
        self.variant(LCDANMLOOPIE_A::LCDANMLOOPIE_1)
    }
}
impl R {
    #[doc = "Bit 1 - LCD Blink, segments off interrupt enable"]
    #[inline(always)]
    pub fn lcdblkoffie(&self) -> LCDBLKOFFIE_R {
        LCDBLKOFFIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD Blink, segments on interrupt enable"]
    #[inline(always)]
    pub fn lcdblkonie(&self) -> LCDBLKONIE_R {
        LCDBLKONIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LCD Frame interrupt enable"]
    #[inline(always)]
    pub fn lcdfrmie(&self) -> LCDFRMIE_R {
        LCDFRMIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - LCD Animation step interrupt enable"]
    #[inline(always)]
    pub fn lcdanmstpie(&self) -> LCDANMSTPIE_R {
        LCDANMSTPIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD Animation loop interrupt enable"]
    #[inline(always)]
    pub fn lcdanmloopie(&self) -> LCDANMLOOPIE_R {
        LCDANMLOOPIE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - LCD Blink, segments off interrupt enable"]
    #[inline(always)]
    pub fn lcdblkoffie(&mut self) -> LCDBLKOFFIE_W<1> {
        LCDBLKOFFIE_W::new(self)
    }
    #[doc = "Bit 2 - LCD Blink, segments on interrupt enable"]
    #[inline(always)]
    pub fn lcdblkonie(&mut self) -> LCDBLKONIE_W<2> {
        LCDBLKONIE_W::new(self)
    }
    #[doc = "Bit 3 - LCD Frame interrupt enable"]
    #[inline(always)]
    pub fn lcdfrmie(&mut self) -> LCDFRMIE_W<3> {
        LCDFRMIE_W::new(self)
    }
    #[doc = "Bit 8 - LCD Animation step interrupt enable"]
    #[inline(always)]
    pub fn lcdanmstpie(&mut self) -> LCDANMSTPIE_W<8> {
        LCDANMSTPIE_W::new(self)
    }
    #[doc = "Bit 9 - LCD Animation loop interrupt enable"]
    #[inline(always)]
    pub fn lcdanmloopie(&mut self) -> LCDANMLOOPIE_W<9> {
        LCDANMLOOPIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_F interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcdie](index.html) module"]
pub struct LCDIE_SPEC;
impl crate::RegisterSpec for LCDIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcdie::R](R) reader structure"]
impl crate::Readable for LCDIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcdie::W](W) writer structure"]
impl crate::Writable for LCDIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCDIE to value 0"]
impl crate::Resettable for LCDIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
