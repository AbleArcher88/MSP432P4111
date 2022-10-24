#[doc = "Register `CAPTIOxCTL` reader"]
pub struct R(crate::R<CAPTIOX_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPTIOX_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPTIOX_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPTIOX_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAPTIOxCTL` writer"]
pub struct W(crate::W<CAPTIOX_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAPTIOX_CTL_SPEC>;
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
impl From<crate::W<CAPTIOX_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAPTIOX_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPTIOPISELx` reader - Capacitive Touch IO pin select"]
pub type CAPTIOPISELX_R = crate::FieldReader<u8, CAPTIOPISELX_A>;
#[doc = "Capacitive Touch IO pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAPTIOPISELX_A {
    #[doc = "0: Px.0"]
    CAPTIOPISELX_0 = 0,
    #[doc = "1: Px.1"]
    CAPTIOPISELX_1 = 1,
    #[doc = "2: Px.2"]
    CAPTIOPISELX_2 = 2,
    #[doc = "3: Px.3"]
    CAPTIOPISELX_3 = 3,
    #[doc = "4: Px.4"]
    CAPTIOPISELX_4 = 4,
    #[doc = "5: Px.5"]
    CAPTIOPISELX_5 = 5,
    #[doc = "6: Px.6"]
    CAPTIOPISELX_6 = 6,
    #[doc = "7: Px.7"]
    CAPTIOPISELX_7 = 7,
}
impl From<CAPTIOPISELX_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPTIOPISELX_A) -> Self {
        variant as _
    }
}
impl CAPTIOPISELX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTIOPISELX_A {
        match self.bits {
            0 => CAPTIOPISELX_A::CAPTIOPISELX_0,
            1 => CAPTIOPISELX_A::CAPTIOPISELX_1,
            2 => CAPTIOPISELX_A::CAPTIOPISELX_2,
            3 => CAPTIOPISELX_A::CAPTIOPISELX_3,
            4 => CAPTIOPISELX_A::CAPTIOPISELX_4,
            5 => CAPTIOPISELX_A::CAPTIOPISELX_5,
            6 => CAPTIOPISELX_A::CAPTIOPISELX_6,
            7 => CAPTIOPISELX_A::CAPTIOPISELX_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CAPTIOPISELX_0`"]
    #[inline(always)]
    pub fn is_captiopiselx_0(&self) -> bool {
        *self == CAPTIOPISELX_A::CAPTIOPISELX_0
    }
    #[doc = "Checks if the value of the field is `CAPTIOPISELX_1`"]
    #[inline(always)]
    pub fn is_captiopiselx_1(&self) -> bool {
        *self == CAPTIOPISELX_A::CAPTIOPISELX_1
    }
    #[doc = "Checks if the value of the field is `CAPTIOPISELX_2`"]
    #[inline(always)]
    pub fn is_captiopiselx_2(&self) -> bool {
        *self == CAPTIOPISELX_A::CAPTIOPISELX_2
    }
    #[doc = "Checks if the value of the field is `CAPTIOPISELX_3`"]
    #[inline(always)]
    pub fn is_captiopiselx_3(&self) -> bool {
        *self == CAPTIOPISELX_A::CAPTIOPISELX_3
    }
    #[doc = "Checks if the value of the field is `CAPTIOPISELX_4`"]
    #[inline(always)]
    pub fn is_captiopiselx_4(&self) -> bool {
        *self == CAPTIOPISELX_A::CAPTIOPISELX_4
    }
    #[doc = "Checks if the value of the field is `CAPTIOPISELX_5`"]
    #[inline(always)]
    pub fn is_captiopiselx_5(&self) -> bool {
        *self == CAPTIOPISELX_A::CAPTIOPISELX_5
    }
    #[doc = "Checks if the value of the field is `CAPTIOPISELX_6`"]
    #[inline(always)]
    pub fn is_captiopiselx_6(&self) -> bool {
        *self == CAPTIOPISELX_A::CAPTIOPISELX_6
    }
    #[doc = "Checks if the value of the field is `CAPTIOPISELX_7`"]
    #[inline(always)]
    pub fn is_captiopiselx_7(&self) -> bool {
        *self == CAPTIOPISELX_A::CAPTIOPISELX_7
    }
}
#[doc = "Field `CAPTIOPISELx` writer - Capacitive Touch IO pin select"]
pub type CAPTIOPISELX_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CAPTIOX_CTL_SPEC, u8, CAPTIOPISELX_A, 3, O>;
impl<'a, const O: u8> CAPTIOPISELX_W<'a, O> {
    #[doc = "Px.0"]
    #[inline(always)]
    pub fn captiopiselx_0(self) -> &'a mut W {
        self.variant(CAPTIOPISELX_A::CAPTIOPISELX_0)
    }
    #[doc = "Px.1"]
    #[inline(always)]
    pub fn captiopiselx_1(self) -> &'a mut W {
        self.variant(CAPTIOPISELX_A::CAPTIOPISELX_1)
    }
    #[doc = "Px.2"]
    #[inline(always)]
    pub fn captiopiselx_2(self) -> &'a mut W {
        self.variant(CAPTIOPISELX_A::CAPTIOPISELX_2)
    }
    #[doc = "Px.3"]
    #[inline(always)]
    pub fn captiopiselx_3(self) -> &'a mut W {
        self.variant(CAPTIOPISELX_A::CAPTIOPISELX_3)
    }
    #[doc = "Px.4"]
    #[inline(always)]
    pub fn captiopiselx_4(self) -> &'a mut W {
        self.variant(CAPTIOPISELX_A::CAPTIOPISELX_4)
    }
    #[doc = "Px.5"]
    #[inline(always)]
    pub fn captiopiselx_5(self) -> &'a mut W {
        self.variant(CAPTIOPISELX_A::CAPTIOPISELX_5)
    }
    #[doc = "Px.6"]
    #[inline(always)]
    pub fn captiopiselx_6(self) -> &'a mut W {
        self.variant(CAPTIOPISELX_A::CAPTIOPISELX_6)
    }
    #[doc = "Px.7"]
    #[inline(always)]
    pub fn captiopiselx_7(self) -> &'a mut W {
        self.variant(CAPTIOPISELX_A::CAPTIOPISELX_7)
    }
}
#[doc = "Field `CAPTIOPOSELx` reader - Capacitive Touch IO port select"]
pub type CAPTIOPOSELX_R = crate::FieldReader<u8, CAPTIOPOSELX_A>;
#[doc = "Capacitive Touch IO port select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAPTIOPOSELX_A {
    #[doc = "0: Px = PJ"]
    CAPTIOPOSELX_0 = 0,
    #[doc = "1: Px = P1"]
    CAPTIOPOSELX_1 = 1,
    #[doc = "2: Px = P2"]
    CAPTIOPOSELX_2 = 2,
    #[doc = "3: Px = P3"]
    CAPTIOPOSELX_3 = 3,
    #[doc = "4: Px = P4"]
    CAPTIOPOSELX_4 = 4,
    #[doc = "5: Px = P5"]
    CAPTIOPOSELX_5 = 5,
    #[doc = "6: Px = P6"]
    CAPTIOPOSELX_6 = 6,
    #[doc = "7: Px = P7"]
    CAPTIOPOSELX_7 = 7,
    #[doc = "8: Px = P8"]
    CAPTIOPOSELX_8 = 8,
    #[doc = "9: Px = P9"]
    CAPTIOPOSELX_9 = 9,
    #[doc = "10: Px = P10"]
    CAPTIOPOSELX_10 = 10,
    #[doc = "11: Px = P11"]
    CAPTIOPOSELX_11 = 11,
    #[doc = "12: Px = P12"]
    CAPTIOPOSELX_12 = 12,
    #[doc = "13: Px = P13"]
    CAPTIOPOSELX_13 = 13,
    #[doc = "14: Px = P14"]
    CAPTIOPOSELX_14 = 14,
    #[doc = "15: Px = P15"]
    CAPTIOPOSELX_15 = 15,
}
impl From<CAPTIOPOSELX_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPTIOPOSELX_A) -> Self {
        variant as _
    }
}
impl CAPTIOPOSELX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTIOPOSELX_A {
        match self.bits {
            0 => CAPTIOPOSELX_A::CAPTIOPOSELX_0,
            1 => CAPTIOPOSELX_A::CAPTIOPOSELX_1,
            2 => CAPTIOPOSELX_A::CAPTIOPOSELX_2,
            3 => CAPTIOPOSELX_A::CAPTIOPOSELX_3,
            4 => CAPTIOPOSELX_A::CAPTIOPOSELX_4,
            5 => CAPTIOPOSELX_A::CAPTIOPOSELX_5,
            6 => CAPTIOPOSELX_A::CAPTIOPOSELX_6,
            7 => CAPTIOPOSELX_A::CAPTIOPOSELX_7,
            8 => CAPTIOPOSELX_A::CAPTIOPOSELX_8,
            9 => CAPTIOPOSELX_A::CAPTIOPOSELX_9,
            10 => CAPTIOPOSELX_A::CAPTIOPOSELX_10,
            11 => CAPTIOPOSELX_A::CAPTIOPOSELX_11,
            12 => CAPTIOPOSELX_A::CAPTIOPOSELX_12,
            13 => CAPTIOPOSELX_A::CAPTIOPOSELX_13,
            14 => CAPTIOPOSELX_A::CAPTIOPOSELX_14,
            15 => CAPTIOPOSELX_A::CAPTIOPOSELX_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CAPTIOPOSELX_0`"]
    #[inline(always)]
    pub fn is_captioposelx_0(&self) -> bool {
        *self == CAPTIOPOSELX_A::CAPTIOPOSELX_0
    }
    #[doc = "Checks if the value of the field is `CAPTIOPOSELX_1`"]
    #[inline(always)]
    pub fn is_captioposelx_1(&self) -> bool {
        *self == CAPTIOPOSELX_A::CAPTIOPOSELX_1
    }
    #[doc = "Checks if the value of the field is `CAPTIOPOSELX_2`"]
    #[inline(always)]
    pub fn is_captioposelx_2(&self) -> bool {
        *self == CAPTIOPOSELX_A::CAPTIOPOSELX_2
    }
    #[doc = "Checks if the value of the field is `CAPTIOPOSELX_3`"]
    #[inline(always)]
    pub fn is_captioposelx_3(&self) -> bool {
        *self == CAPTIOPOSELX_A::CAPTIOPOSELX_3
    }
    #[doc = "Checks if the value of the field is `CAPTIOPOSELX_4`"]
    #[inline(always)]
    pub fn is_captioposelx_4(&self) -> bool {
        *self == CAPTIOPOSELX_A::CAPTIOPOSELX_4
    }
    #[doc = "Checks if the value of the field is `CAPTIOPOSELX_5`"]
    #[inline(always)]
    pub fn is_captioposelx_5(&self) -> bool {
        *self == CAPTIOPOSELX_A::CAPTIOPOSELX_5
    }
    #[doc = "Checks if the value of the field is `CAPTIOPOSELX_6`"]
    #[inline(always)]
    pub fn is_captioposelx_6(&self) -> bool {
        *self == CAPTIOPOSELX_A::CAPTIOPOSELX_6
    }
    #[doc = "Checks if the value of the field is `CAPTIOPOSELX_7`"]
    #[inline(always)]
    pub fn is_captioposelx_7(&self) -> bool {
        *self == CAPTIOPOSELX_A::CAPTIOPOSELX_7
    }
    #[doc = "Checks if the value of the field is `CAPTIOPOSELX_8`"]
    #[inline(always)]
    pub fn is_captioposelx_8(&self) -> bool {
        *self == CAPTIOPOSELX_A::CAPTIOPOSELX_8
    }
    #[doc = "Checks if the value of the field is `CAPTIOPOSELX_9`"]
    #[inline(always)]
    pub fn is_captioposelx_9(&self) -> bool {
        *self == CAPTIOPOSELX_A::CAPTIOPOSELX_9
    }
    #[doc = "Checks if the value of the field is `CAPTIOPOSELX_10`"]
    #[inline(always)]
    pub fn is_captioposelx_10(&self) -> bool {
        *self == CAPTIOPOSELX_A::CAPTIOPOSELX_10
    }
    #[doc = "Checks if the value of the field is `CAPTIOPOSELX_11`"]
    #[inline(always)]
    pub fn is_captioposelx_11(&self) -> bool {
        *self == CAPTIOPOSELX_A::CAPTIOPOSELX_11
    }
    #[doc = "Checks if the value of the field is `CAPTIOPOSELX_12`"]
    #[inline(always)]
    pub fn is_captioposelx_12(&self) -> bool {
        *self == CAPTIOPOSELX_A::CAPTIOPOSELX_12
    }
    #[doc = "Checks if the value of the field is `CAPTIOPOSELX_13`"]
    #[inline(always)]
    pub fn is_captioposelx_13(&self) -> bool {
        *self == CAPTIOPOSELX_A::CAPTIOPOSELX_13
    }
    #[doc = "Checks if the value of the field is `CAPTIOPOSELX_14`"]
    #[inline(always)]
    pub fn is_captioposelx_14(&self) -> bool {
        *self == CAPTIOPOSELX_A::CAPTIOPOSELX_14
    }
    #[doc = "Checks if the value of the field is `CAPTIOPOSELX_15`"]
    #[inline(always)]
    pub fn is_captioposelx_15(&self) -> bool {
        *self == CAPTIOPOSELX_A::CAPTIOPOSELX_15
    }
}
#[doc = "Field `CAPTIOPOSELx` writer - Capacitive Touch IO port select"]
pub type CAPTIOPOSELX_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CAPTIOX_CTL_SPEC, u8, CAPTIOPOSELX_A, 4, O>;
impl<'a, const O: u8> CAPTIOPOSELX_W<'a, O> {
    #[doc = "Px = PJ"]
    #[inline(always)]
    pub fn captioposelx_0(self) -> &'a mut W {
        self.variant(CAPTIOPOSELX_A::CAPTIOPOSELX_0)
    }
    #[doc = "Px = P1"]
    #[inline(always)]
    pub fn captioposelx_1(self) -> &'a mut W {
        self.variant(CAPTIOPOSELX_A::CAPTIOPOSELX_1)
    }
    #[doc = "Px = P2"]
    #[inline(always)]
    pub fn captioposelx_2(self) -> &'a mut W {
        self.variant(CAPTIOPOSELX_A::CAPTIOPOSELX_2)
    }
    #[doc = "Px = P3"]
    #[inline(always)]
    pub fn captioposelx_3(self) -> &'a mut W {
        self.variant(CAPTIOPOSELX_A::CAPTIOPOSELX_3)
    }
    #[doc = "Px = P4"]
    #[inline(always)]
    pub fn captioposelx_4(self) -> &'a mut W {
        self.variant(CAPTIOPOSELX_A::CAPTIOPOSELX_4)
    }
    #[doc = "Px = P5"]
    #[inline(always)]
    pub fn captioposelx_5(self) -> &'a mut W {
        self.variant(CAPTIOPOSELX_A::CAPTIOPOSELX_5)
    }
    #[doc = "Px = P6"]
    #[inline(always)]
    pub fn captioposelx_6(self) -> &'a mut W {
        self.variant(CAPTIOPOSELX_A::CAPTIOPOSELX_6)
    }
    #[doc = "Px = P7"]
    #[inline(always)]
    pub fn captioposelx_7(self) -> &'a mut W {
        self.variant(CAPTIOPOSELX_A::CAPTIOPOSELX_7)
    }
    #[doc = "Px = P8"]
    #[inline(always)]
    pub fn captioposelx_8(self) -> &'a mut W {
        self.variant(CAPTIOPOSELX_A::CAPTIOPOSELX_8)
    }
    #[doc = "Px = P9"]
    #[inline(always)]
    pub fn captioposelx_9(self) -> &'a mut W {
        self.variant(CAPTIOPOSELX_A::CAPTIOPOSELX_9)
    }
    #[doc = "Px = P10"]
    #[inline(always)]
    pub fn captioposelx_10(self) -> &'a mut W {
        self.variant(CAPTIOPOSELX_A::CAPTIOPOSELX_10)
    }
    #[doc = "Px = P11"]
    #[inline(always)]
    pub fn captioposelx_11(self) -> &'a mut W {
        self.variant(CAPTIOPOSELX_A::CAPTIOPOSELX_11)
    }
    #[doc = "Px = P12"]
    #[inline(always)]
    pub fn captioposelx_12(self) -> &'a mut W {
        self.variant(CAPTIOPOSELX_A::CAPTIOPOSELX_12)
    }
    #[doc = "Px = P13"]
    #[inline(always)]
    pub fn captioposelx_13(self) -> &'a mut W {
        self.variant(CAPTIOPOSELX_A::CAPTIOPOSELX_13)
    }
    #[doc = "Px = P14"]
    #[inline(always)]
    pub fn captioposelx_14(self) -> &'a mut W {
        self.variant(CAPTIOPOSELX_A::CAPTIOPOSELX_14)
    }
    #[doc = "Px = P15"]
    #[inline(always)]
    pub fn captioposelx_15(self) -> &'a mut W {
        self.variant(CAPTIOPOSELX_A::CAPTIOPOSELX_15)
    }
}
#[doc = "Field `CAPTIOEN` reader - Capacitive Touch IO enable"]
pub type CAPTIOEN_R = crate::BitReader<CAPTIOEN_A>;
#[doc = "Capacitive Touch IO enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAPTIOEN_A {
    #[doc = "0: All Capacitive Touch IOs are disabled. Signal towards timers is 0."]
    CAPTIOEN_0 = 0,
    #[doc = "1: Selected Capacitive Touch IO is enabled"]
    CAPTIOEN_1 = 1,
}
impl From<CAPTIOEN_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTIOEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CAPTIOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTIOEN_A {
        match self.bits {
            false => CAPTIOEN_A::CAPTIOEN_0,
            true => CAPTIOEN_A::CAPTIOEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAPTIOEN_0`"]
    #[inline(always)]
    pub fn is_captioen_0(&self) -> bool {
        *self == CAPTIOEN_A::CAPTIOEN_0
    }
    #[doc = "Checks if the value of the field is `CAPTIOEN_1`"]
    #[inline(always)]
    pub fn is_captioen_1(&self) -> bool {
        *self == CAPTIOEN_A::CAPTIOEN_1
    }
}
#[doc = "Field `CAPTIOEN` writer - Capacitive Touch IO enable"]
pub type CAPTIOEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CAPTIOX_CTL_SPEC, CAPTIOEN_A, O>;
impl<'a, const O: u8> CAPTIOEN_W<'a, O> {
    #[doc = "All Capacitive Touch IOs are disabled. Signal towards timers is 0."]
    #[inline(always)]
    pub fn captioen_0(self) -> &'a mut W {
        self.variant(CAPTIOEN_A::CAPTIOEN_0)
    }
    #[doc = "Selected Capacitive Touch IO is enabled"]
    #[inline(always)]
    pub fn captioen_1(self) -> &'a mut W {
        self.variant(CAPTIOEN_A::CAPTIOEN_1)
    }
}
#[doc = "Field `CAPTIOSTATE` reader - Capacitive Touch IO state"]
pub type CAPTIOSTATE_R = crate::BitReader<CAPTIOSTATE_ENUM_READ_A>;
#[doc = "Capacitive Touch IO state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAPTIOSTATE_ENUM_READ_A {
    #[doc = "0: Curent state 0 or Capacitive Touch IO is disabled"]
    CAPTIOSTATE_0 = 0,
    #[doc = "1: Current state 1"]
    CAPTIOSTATE_1 = 1,
}
impl From<CAPTIOSTATE_ENUM_READ_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTIOSTATE_ENUM_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl CAPTIOSTATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTIOSTATE_ENUM_READ_A {
        match self.bits {
            false => CAPTIOSTATE_ENUM_READ_A::CAPTIOSTATE_0,
            true => CAPTIOSTATE_ENUM_READ_A::CAPTIOSTATE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAPTIOSTATE_0`"]
    #[inline(always)]
    pub fn is_captiostate_0(&self) -> bool {
        *self == CAPTIOSTATE_ENUM_READ_A::CAPTIOSTATE_0
    }
    #[doc = "Checks if the value of the field is `CAPTIOSTATE_1`"]
    #[inline(always)]
    pub fn is_captiostate_1(&self) -> bool {
        *self == CAPTIOSTATE_ENUM_READ_A::CAPTIOSTATE_1
    }
}
impl R {
    #[doc = "Bits 1:3 - Capacitive Touch IO pin select"]
    #[inline(always)]
    pub fn captiopiselx(&self) -> CAPTIOPISELX_R {
        CAPTIOPISELX_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:7 - Capacitive Touch IO port select"]
    #[inline(always)]
    pub fn captioposelx(&self) -> CAPTIOPOSELX_R {
        CAPTIOPOSELX_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Capacitive Touch IO enable"]
    #[inline(always)]
    pub fn captioen(&self) -> CAPTIOEN_R {
        CAPTIOEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capacitive Touch IO state"]
    #[inline(always)]
    pub fn captiostate(&self) -> CAPTIOSTATE_R {
        CAPTIOSTATE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:3 - Capacitive Touch IO pin select"]
    #[inline(always)]
    pub fn captiopiselx(&mut self) -> CAPTIOPISELX_W<1> {
        CAPTIOPISELX_W::new(self)
    }
    #[doc = "Bits 4:7 - Capacitive Touch IO port select"]
    #[inline(always)]
    pub fn captioposelx(&mut self) -> CAPTIOPOSELX_W<4> {
        CAPTIOPOSELX_W::new(self)
    }
    #[doc = "Bit 8 - Capacitive Touch IO enable"]
    #[inline(always)]
    pub fn captioen(&mut self) -> CAPTIOEN_W<8> {
        CAPTIOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capacitive Touch IO x Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [captiox_ctl](index.html) module"]
pub struct CAPTIOX_CTL_SPEC;
impl crate::RegisterSpec for CAPTIOX_CTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [captiox_ctl::R](R) reader structure"]
impl crate::Readable for CAPTIOX_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [captiox_ctl::W](W) writer structure"]
impl crate::Writable for CAPTIOX_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAPTIOxCTL to value 0"]
impl crate::Resettable for CAPTIOX_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
