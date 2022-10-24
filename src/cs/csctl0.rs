#[doc = "Register `CSCTL0` reader"]
pub struct R(crate::R<CSCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSCTL0` writer"]
pub struct W(crate::W<CSCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSCTL0_SPEC>;
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
impl From<crate::W<CSCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCOTUNE` reader - DCO frequency tuning select"]
pub type DCOTUNE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DCOTUNE` writer - DCO frequency tuning select"]
pub type DCOTUNE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSCTL0_SPEC, u16, u16, 10, O>;
#[doc = "Field `DCORSEL` reader - DCO frequency range select"]
pub type DCORSEL_R = crate::FieldReader<u8, DCORSEL_A>;
#[doc = "DCO frequency range select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DCORSEL_A {
    #[doc = "0: Nominal DCO Frequency Range (MHz): 1 to 2"]
    DCORSEL_0 = 0,
    #[doc = "1: Nominal DCO Frequency Range (MHz): 2 to 4"]
    DCORSEL_1 = 1,
    #[doc = "2: Nominal DCO Frequency Range (MHz): 4 to 8"]
    DCORSEL_2 = 2,
    #[doc = "3: Nominal DCO Frequency Range (MHz): 8 to 16"]
    DCORSEL_3 = 3,
    #[doc = "4: Nominal DCO Frequency Range (MHz): 16 to 32"]
    DCORSEL_4 = 4,
    #[doc = "5: Nominal DCO Frequency Range (MHz): 32 to 64"]
    DCORSEL_5 = 5,
}
impl From<DCORSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DCORSEL_A) -> Self {
        variant as _
    }
}
impl DCORSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DCORSEL_A> {
        match self.bits {
            0 => Some(DCORSEL_A::DCORSEL_0),
            1 => Some(DCORSEL_A::DCORSEL_1),
            2 => Some(DCORSEL_A::DCORSEL_2),
            3 => Some(DCORSEL_A::DCORSEL_3),
            4 => Some(DCORSEL_A::DCORSEL_4),
            5 => Some(DCORSEL_A::DCORSEL_5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DCORSEL_0`"]
    #[inline(always)]
    pub fn is_dcorsel_0(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_0
    }
    #[doc = "Checks if the value of the field is `DCORSEL_1`"]
    #[inline(always)]
    pub fn is_dcorsel_1(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_1
    }
    #[doc = "Checks if the value of the field is `DCORSEL_2`"]
    #[inline(always)]
    pub fn is_dcorsel_2(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_2
    }
    #[doc = "Checks if the value of the field is `DCORSEL_3`"]
    #[inline(always)]
    pub fn is_dcorsel_3(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_3
    }
    #[doc = "Checks if the value of the field is `DCORSEL_4`"]
    #[inline(always)]
    pub fn is_dcorsel_4(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_4
    }
    #[doc = "Checks if the value of the field is `DCORSEL_5`"]
    #[inline(always)]
    pub fn is_dcorsel_5(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_5
    }
}
#[doc = "Field `DCORSEL` writer - DCO frequency range select"]
pub type DCORSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSCTL0_SPEC, u8, DCORSEL_A, 3, O>;
impl<'a, const O: u8> DCORSEL_W<'a, O> {
    #[doc = "Nominal DCO Frequency Range (MHz): 1 to 2"]
    #[inline(always)]
    pub fn dcorsel_0(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_0)
    }
    #[doc = "Nominal DCO Frequency Range (MHz): 2 to 4"]
    #[inline(always)]
    pub fn dcorsel_1(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_1)
    }
    #[doc = "Nominal DCO Frequency Range (MHz): 4 to 8"]
    #[inline(always)]
    pub fn dcorsel_2(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_2)
    }
    #[doc = "Nominal DCO Frequency Range (MHz): 8 to 16"]
    #[inline(always)]
    pub fn dcorsel_3(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_3)
    }
    #[doc = "Nominal DCO Frequency Range (MHz): 16 to 32"]
    #[inline(always)]
    pub fn dcorsel_4(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_4)
    }
    #[doc = "Nominal DCO Frequency Range (MHz): 32 to 64"]
    #[inline(always)]
    pub fn dcorsel_5(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_5)
    }
}
#[doc = "Field `DCORES` reader - Enables the DCO external resistor mode"]
pub type DCORES_R = crate::BitReader<DCORES_A>;
#[doc = "Enables the DCO external resistor mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCORES_A {
    #[doc = "0: Internal resistor mode"]
    DCORES_0 = 0,
    #[doc = "1: External resistor mode"]
    DCORES_1 = 1,
}
impl From<DCORES_A> for bool {
    #[inline(always)]
    fn from(variant: DCORES_A) -> Self {
        variant as u8 != 0
    }
}
impl DCORES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCORES_A {
        match self.bits {
            false => DCORES_A::DCORES_0,
            true => DCORES_A::DCORES_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCORES_0`"]
    #[inline(always)]
    pub fn is_dcores_0(&self) -> bool {
        *self == DCORES_A::DCORES_0
    }
    #[doc = "Checks if the value of the field is `DCORES_1`"]
    #[inline(always)]
    pub fn is_dcores_1(&self) -> bool {
        *self == DCORES_A::DCORES_1
    }
}
#[doc = "Field `DCORES` writer - Enables the DCO external resistor mode"]
pub type DCORES_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSCTL0_SPEC, DCORES_A, O>;
impl<'a, const O: u8> DCORES_W<'a, O> {
    #[doc = "Internal resistor mode"]
    #[inline(always)]
    pub fn dcores_0(self) -> &'a mut W {
        self.variant(DCORES_A::DCORES_0)
    }
    #[doc = "External resistor mode"]
    #[inline(always)]
    pub fn dcores_1(self) -> &'a mut W {
        self.variant(DCORES_A::DCORES_1)
    }
}
#[doc = "Field `DCOEN` reader - Enables the DCO oscillator"]
pub type DCOEN_R = crate::BitReader<DCOEN_A>;
#[doc = "Enables the DCO oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCOEN_A {
    #[doc = "0: DCO is on if it is used as a source for MCLK, HSMCLK , or SMCLK and clock is requested, otherwise it is disabled."]
    DCOEN_0 = 0,
    #[doc = "1: DCO is on"]
    DCOEN_1 = 1,
}
impl From<DCOEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCOEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DCOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCOEN_A {
        match self.bits {
            false => DCOEN_A::DCOEN_0,
            true => DCOEN_A::DCOEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCOEN_0`"]
    #[inline(always)]
    pub fn is_dcoen_0(&self) -> bool {
        *self == DCOEN_A::DCOEN_0
    }
    #[doc = "Checks if the value of the field is `DCOEN_1`"]
    #[inline(always)]
    pub fn is_dcoen_1(&self) -> bool {
        *self == DCOEN_A::DCOEN_1
    }
}
#[doc = "Field `DCOEN` writer - Enables the DCO oscillator"]
pub type DCOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSCTL0_SPEC, DCOEN_A, O>;
impl<'a, const O: u8> DCOEN_W<'a, O> {
    #[doc = "DCO is on if it is used as a source for MCLK, HSMCLK , or SMCLK and clock is requested, otherwise it is disabled."]
    #[inline(always)]
    pub fn dcoen_0(self) -> &'a mut W {
        self.variant(DCOEN_A::DCOEN_0)
    }
    #[doc = "DCO is on"]
    #[inline(always)]
    pub fn dcoen_1(self) -> &'a mut W {
        self.variant(DCOEN_A::DCOEN_1)
    }
}
impl R {
    #[doc = "Bits 0:9 - DCO frequency tuning select"]
    #[inline(always)]
    pub fn dcotune(&self) -> DCOTUNE_R {
        DCOTUNE_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:18 - DCO frequency range select"]
    #[inline(always)]
    pub fn dcorsel(&self) -> DCORSEL_R {
        DCORSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 22 - Enables the DCO external resistor mode"]
    #[inline(always)]
    pub fn dcores(&self) -> DCORES_R {
        DCORES_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enables the DCO oscillator"]
    #[inline(always)]
    pub fn dcoen(&self) -> DCOEN_R {
        DCOEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - DCO frequency tuning select"]
    #[inline(always)]
    pub fn dcotune(&mut self) -> DCOTUNE_W<0> {
        DCOTUNE_W::new(self)
    }
    #[doc = "Bits 16:18 - DCO frequency range select"]
    #[inline(always)]
    pub fn dcorsel(&mut self) -> DCORSEL_W<16> {
        DCORSEL_W::new(self)
    }
    #[doc = "Bit 22 - Enables the DCO external resistor mode"]
    #[inline(always)]
    pub fn dcores(&mut self) -> DCORES_W<22> {
        DCORES_W::new(self)
    }
    #[doc = "Bit 23 - Enables the DCO oscillator"]
    #[inline(always)]
    pub fn dcoen(&mut self) -> DCOEN_W<23> {
        DCOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csctl0](index.html) module"]
pub struct CSCTL0_SPEC;
impl crate::RegisterSpec for CSCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csctl0::R](R) reader structure"]
impl crate::Readable for CSCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csctl0::W](W) writer structure"]
impl crate::Writable for CSCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSCTL0 to value 0x0001_0000"]
impl crate::Resettable for CSCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0000
    }
}
