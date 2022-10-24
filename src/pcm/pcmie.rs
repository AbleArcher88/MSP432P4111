#[doc = "Register `PCMIE` reader"]
pub struct R(crate::R<PCMIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCMIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCMIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCMIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCMIE` writer"]
pub struct W(crate::W<PCMIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCMIE_SPEC>;
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
impl From<crate::W<PCMIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCMIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPM_INVALID_TR_IE` reader - LPM invalid transition interrupt enable"]
pub type LPM_INVALID_TR_IE_R = crate::BitReader<LPM_INVALID_TR_IE_A>;
#[doc = "LPM invalid transition interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPM_INVALID_TR_IE_A {
    #[doc = "0: Disabled"]
    LPM_INVALID_TR_IE_0 = 0,
    #[doc = "1: Enabled"]
    LPM_INVALID_TR_IE_1 = 1,
}
impl From<LPM_INVALID_TR_IE_A> for bool {
    #[inline(always)]
    fn from(variant: LPM_INVALID_TR_IE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPM_INVALID_TR_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPM_INVALID_TR_IE_A {
        match self.bits {
            false => LPM_INVALID_TR_IE_A::LPM_INVALID_TR_IE_0,
            true => LPM_INVALID_TR_IE_A::LPM_INVALID_TR_IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPM_INVALID_TR_IE_0`"]
    #[inline(always)]
    pub fn is_lpm_invalid_tr_ie_0(&self) -> bool {
        *self == LPM_INVALID_TR_IE_A::LPM_INVALID_TR_IE_0
    }
    #[doc = "Checks if the value of the field is `LPM_INVALID_TR_IE_1`"]
    #[inline(always)]
    pub fn is_lpm_invalid_tr_ie_1(&self) -> bool {
        *self == LPM_INVALID_TR_IE_A::LPM_INVALID_TR_IE_1
    }
}
#[doc = "Field `LPM_INVALID_TR_IE` writer - LPM invalid transition interrupt enable"]
pub type LPM_INVALID_TR_IE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PCMIE_SPEC, LPM_INVALID_TR_IE_A, O>;
impl<'a, const O: u8> LPM_INVALID_TR_IE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn lpm_invalid_tr_ie_0(self) -> &'a mut W {
        self.variant(LPM_INVALID_TR_IE_A::LPM_INVALID_TR_IE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn lpm_invalid_tr_ie_1(self) -> &'a mut W {
        self.variant(LPM_INVALID_TR_IE_A::LPM_INVALID_TR_IE_1)
    }
}
#[doc = "Field `LPM_INVALID_CLK_IE` reader - LPM invalid clock interrupt enable"]
pub type LPM_INVALID_CLK_IE_R = crate::BitReader<LPM_INVALID_CLK_IE_A>;
#[doc = "LPM invalid clock interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPM_INVALID_CLK_IE_A {
    #[doc = "0: Disabled"]
    LPM_INVALID_CLK_IE_0 = 0,
    #[doc = "1: Enabled"]
    LPM_INVALID_CLK_IE_1 = 1,
}
impl From<LPM_INVALID_CLK_IE_A> for bool {
    #[inline(always)]
    fn from(variant: LPM_INVALID_CLK_IE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPM_INVALID_CLK_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPM_INVALID_CLK_IE_A {
        match self.bits {
            false => LPM_INVALID_CLK_IE_A::LPM_INVALID_CLK_IE_0,
            true => LPM_INVALID_CLK_IE_A::LPM_INVALID_CLK_IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPM_INVALID_CLK_IE_0`"]
    #[inline(always)]
    pub fn is_lpm_invalid_clk_ie_0(&self) -> bool {
        *self == LPM_INVALID_CLK_IE_A::LPM_INVALID_CLK_IE_0
    }
    #[doc = "Checks if the value of the field is `LPM_INVALID_CLK_IE_1`"]
    #[inline(always)]
    pub fn is_lpm_invalid_clk_ie_1(&self) -> bool {
        *self == LPM_INVALID_CLK_IE_A::LPM_INVALID_CLK_IE_1
    }
}
#[doc = "Field `LPM_INVALID_CLK_IE` writer - LPM invalid clock interrupt enable"]
pub type LPM_INVALID_CLK_IE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PCMIE_SPEC, LPM_INVALID_CLK_IE_A, O>;
impl<'a, const O: u8> LPM_INVALID_CLK_IE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn lpm_invalid_clk_ie_0(self) -> &'a mut W {
        self.variant(LPM_INVALID_CLK_IE_A::LPM_INVALID_CLK_IE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn lpm_invalid_clk_ie_1(self) -> &'a mut W {
        self.variant(LPM_INVALID_CLK_IE_A::LPM_INVALID_CLK_IE_1)
    }
}
#[doc = "Field `AM_INVALID_TR_IE` reader - Active mode invalid transition interrupt enable"]
pub type AM_INVALID_TR_IE_R = crate::BitReader<AM_INVALID_TR_IE_A>;
#[doc = "Active mode invalid transition interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AM_INVALID_TR_IE_A {
    #[doc = "0: Disabled"]
    AM_INVALID_TR_IE_0 = 0,
    #[doc = "1: Enabled"]
    AM_INVALID_TR_IE_1 = 1,
}
impl From<AM_INVALID_TR_IE_A> for bool {
    #[inline(always)]
    fn from(variant: AM_INVALID_TR_IE_A) -> Self {
        variant as u8 != 0
    }
}
impl AM_INVALID_TR_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AM_INVALID_TR_IE_A {
        match self.bits {
            false => AM_INVALID_TR_IE_A::AM_INVALID_TR_IE_0,
            true => AM_INVALID_TR_IE_A::AM_INVALID_TR_IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `AM_INVALID_TR_IE_0`"]
    #[inline(always)]
    pub fn is_am_invalid_tr_ie_0(&self) -> bool {
        *self == AM_INVALID_TR_IE_A::AM_INVALID_TR_IE_0
    }
    #[doc = "Checks if the value of the field is `AM_INVALID_TR_IE_1`"]
    #[inline(always)]
    pub fn is_am_invalid_tr_ie_1(&self) -> bool {
        *self == AM_INVALID_TR_IE_A::AM_INVALID_TR_IE_1
    }
}
#[doc = "Field `AM_INVALID_TR_IE` writer - Active mode invalid transition interrupt enable"]
pub type AM_INVALID_TR_IE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PCMIE_SPEC, AM_INVALID_TR_IE_A, O>;
impl<'a, const O: u8> AM_INVALID_TR_IE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn am_invalid_tr_ie_0(self) -> &'a mut W {
        self.variant(AM_INVALID_TR_IE_A::AM_INVALID_TR_IE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn am_invalid_tr_ie_1(self) -> &'a mut W {
        self.variant(AM_INVALID_TR_IE_A::AM_INVALID_TR_IE_1)
    }
}
#[doc = "Field `DCDC_ERROR_IE` reader - DC-DC error interrupt enable"]
pub type DCDC_ERROR_IE_R = crate::BitReader<DCDC_ERROR_IE_A>;
#[doc = "DC-DC error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCDC_ERROR_IE_A {
    #[doc = "0: Disabled"]
    DCDC_ERROR_IE_0 = 0,
    #[doc = "1: Enabled"]
    DCDC_ERROR_IE_1 = 1,
}
impl From<DCDC_ERROR_IE_A> for bool {
    #[inline(always)]
    fn from(variant: DCDC_ERROR_IE_A) -> Self {
        variant as u8 != 0
    }
}
impl DCDC_ERROR_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDC_ERROR_IE_A {
        match self.bits {
            false => DCDC_ERROR_IE_A::DCDC_ERROR_IE_0,
            true => DCDC_ERROR_IE_A::DCDC_ERROR_IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCDC_ERROR_IE_0`"]
    #[inline(always)]
    pub fn is_dcdc_error_ie_0(&self) -> bool {
        *self == DCDC_ERROR_IE_A::DCDC_ERROR_IE_0
    }
    #[doc = "Checks if the value of the field is `DCDC_ERROR_IE_1`"]
    #[inline(always)]
    pub fn is_dcdc_error_ie_1(&self) -> bool {
        *self == DCDC_ERROR_IE_A::DCDC_ERROR_IE_1
    }
}
#[doc = "Field `DCDC_ERROR_IE` writer - DC-DC error interrupt enable"]
pub type DCDC_ERROR_IE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PCMIE_SPEC, DCDC_ERROR_IE_A, O>;
impl<'a, const O: u8> DCDC_ERROR_IE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn dcdc_error_ie_0(self) -> &'a mut W {
        self.variant(DCDC_ERROR_IE_A::DCDC_ERROR_IE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn dcdc_error_ie_1(self) -> &'a mut W {
        self.variant(DCDC_ERROR_IE_A::DCDC_ERROR_IE_1)
    }
}
impl R {
    #[doc = "Bit 0 - LPM invalid transition interrupt enable"]
    #[inline(always)]
    pub fn lpm_invalid_tr_ie(&self) -> LPM_INVALID_TR_IE_R {
        LPM_INVALID_TR_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPM invalid clock interrupt enable"]
    #[inline(always)]
    pub fn lpm_invalid_clk_ie(&self) -> LPM_INVALID_CLK_IE_R {
        LPM_INVALID_CLK_IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Active mode invalid transition interrupt enable"]
    #[inline(always)]
    pub fn am_invalid_tr_ie(&self) -> AM_INVALID_TR_IE_R {
        AM_INVALID_TR_IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - DC-DC error interrupt enable"]
    #[inline(always)]
    pub fn dcdc_error_ie(&self) -> DCDC_ERROR_IE_R {
        DCDC_ERROR_IE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPM invalid transition interrupt enable"]
    #[inline(always)]
    pub fn lpm_invalid_tr_ie(&mut self) -> LPM_INVALID_TR_IE_W<0> {
        LPM_INVALID_TR_IE_W::new(self)
    }
    #[doc = "Bit 1 - LPM invalid clock interrupt enable"]
    #[inline(always)]
    pub fn lpm_invalid_clk_ie(&mut self) -> LPM_INVALID_CLK_IE_W<1> {
        LPM_INVALID_CLK_IE_W::new(self)
    }
    #[doc = "Bit 2 - Active mode invalid transition interrupt enable"]
    #[inline(always)]
    pub fn am_invalid_tr_ie(&mut self) -> AM_INVALID_TR_IE_W<2> {
        AM_INVALID_TR_IE_W::new(self)
    }
    #[doc = "Bit 6 - DC-DC error interrupt enable"]
    #[inline(always)]
    pub fn dcdc_error_ie(&mut self) -> DCDC_ERROR_IE_W<6> {
        DCDC_ERROR_IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcmie](index.html) module"]
pub struct PCMIE_SPEC;
impl crate::RegisterSpec for PCMIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcmie::R](R) reader structure"]
impl crate::Readable for PCMIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcmie::W](W) writer structure"]
impl crate::Writable for PCMIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCMIE to value 0"]
impl crate::Resettable for PCMIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
