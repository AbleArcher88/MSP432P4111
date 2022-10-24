#[doc = "Register `CSIE` reader"]
pub struct R(crate::R<CSIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSIE` writer"]
pub struct W(crate::W<CSIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIE_SPEC>;
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
impl From<crate::W<CSIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LFXTIE` reader - LFXT oscillator fault flag interrupt enable"]
pub type LFXTIE_R = crate::BitReader<LFXTIE_A>;
#[doc = "LFXT oscillator fault flag interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LFXTIE_A {
    #[doc = "0: Interrupt disabled"]
    LFXTIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    LFXTIE_1 = 1,
}
impl From<LFXTIE_A> for bool {
    #[inline(always)]
    fn from(variant: LFXTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl LFXTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFXTIE_A {
        match self.bits {
            false => LFXTIE_A::LFXTIE_0,
            true => LFXTIE_A::LFXTIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LFXTIE_0`"]
    #[inline(always)]
    pub fn is_lfxtie_0(&self) -> bool {
        *self == LFXTIE_A::LFXTIE_0
    }
    #[doc = "Checks if the value of the field is `LFXTIE_1`"]
    #[inline(always)]
    pub fn is_lfxtie_1(&self) -> bool {
        *self == LFXTIE_A::LFXTIE_1
    }
}
#[doc = "Field `LFXTIE` writer - LFXT oscillator fault flag interrupt enable"]
pub type LFXTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSIE_SPEC, LFXTIE_A, O>;
impl<'a, const O: u8> LFXTIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn lfxtie_0(self) -> &'a mut W {
        self.variant(LFXTIE_A::LFXTIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn lfxtie_1(self) -> &'a mut W {
        self.variant(LFXTIE_A::LFXTIE_1)
    }
}
#[doc = "Field `HFXTIE` reader - HFXT oscillator fault flag interrupt enable"]
pub type HFXTIE_R = crate::BitReader<HFXTIE_A>;
#[doc = "HFXT oscillator fault flag interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HFXTIE_A {
    #[doc = "0: Interrupt disabled"]
    HFXTIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    HFXTIE_1 = 1,
}
impl From<HFXTIE_A> for bool {
    #[inline(always)]
    fn from(variant: HFXTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl HFXTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFXTIE_A {
        match self.bits {
            false => HFXTIE_A::HFXTIE_0,
            true => HFXTIE_A::HFXTIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `HFXTIE_0`"]
    #[inline(always)]
    pub fn is_hfxtie_0(&self) -> bool {
        *self == HFXTIE_A::HFXTIE_0
    }
    #[doc = "Checks if the value of the field is `HFXTIE_1`"]
    #[inline(always)]
    pub fn is_hfxtie_1(&self) -> bool {
        *self == HFXTIE_A::HFXTIE_1
    }
}
#[doc = "Field `HFXTIE` writer - HFXT oscillator fault flag interrupt enable"]
pub type HFXTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSIE_SPEC, HFXTIE_A, O>;
impl<'a, const O: u8> HFXTIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn hfxtie_0(self) -> &'a mut W {
        self.variant(HFXTIE_A::HFXTIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn hfxtie_1(self) -> &'a mut W {
        self.variant(HFXTIE_A::HFXTIE_1)
    }
}
#[doc = "Field `HFXT2IE` reader - HFXT2 oscillator fault flag interrupt enable"]
pub type HFXT2IE_R = crate::BitReader<HFXT2IE_A>;
#[doc = "HFXT2 oscillator fault flag interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HFXT2IE_A {
    #[doc = "0: Interrupt disabled"]
    HFXT2IE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    HFXT2IE_1 = 1,
}
impl From<HFXT2IE_A> for bool {
    #[inline(always)]
    fn from(variant: HFXT2IE_A) -> Self {
        variant as u8 != 0
    }
}
impl HFXT2IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFXT2IE_A {
        match self.bits {
            false => HFXT2IE_A::HFXT2IE_0,
            true => HFXT2IE_A::HFXT2IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `HFXT2IE_0`"]
    #[inline(always)]
    pub fn is_hfxt2ie_0(&self) -> bool {
        *self == HFXT2IE_A::HFXT2IE_0
    }
    #[doc = "Checks if the value of the field is `HFXT2IE_1`"]
    #[inline(always)]
    pub fn is_hfxt2ie_1(&self) -> bool {
        *self == HFXT2IE_A::HFXT2IE_1
    }
}
#[doc = "Field `HFXT2IE` writer - HFXT2 oscillator fault flag interrupt enable"]
pub type HFXT2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSIE_SPEC, HFXT2IE_A, O>;
impl<'a, const O: u8> HFXT2IE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn hfxt2ie_0(self) -> &'a mut W {
        self.variant(HFXT2IE_A::HFXT2IE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn hfxt2ie_1(self) -> &'a mut W {
        self.variant(HFXT2IE_A::HFXT2IE_1)
    }
}
#[doc = "Field `DCOR_OPNIE` reader - DCO external resistor open circuit fault flag interrupt enable."]
pub type DCOR_OPNIE_R = crate::BitReader<DCOR_OPNIE_A>;
#[doc = "DCO external resistor open circuit fault flag interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCOR_OPNIE_A {
    #[doc = "0: Interrupt disabled"]
    DCOR_OPNIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    DCOR_OPNIE_1 = 1,
}
impl From<DCOR_OPNIE_A> for bool {
    #[inline(always)]
    fn from(variant: DCOR_OPNIE_A) -> Self {
        variant as u8 != 0
    }
}
impl DCOR_OPNIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCOR_OPNIE_A {
        match self.bits {
            false => DCOR_OPNIE_A::DCOR_OPNIE_0,
            true => DCOR_OPNIE_A::DCOR_OPNIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCOR_OPNIE_0`"]
    #[inline(always)]
    pub fn is_dcor_opnie_0(&self) -> bool {
        *self == DCOR_OPNIE_A::DCOR_OPNIE_0
    }
    #[doc = "Checks if the value of the field is `DCOR_OPNIE_1`"]
    #[inline(always)]
    pub fn is_dcor_opnie_1(&self) -> bool {
        *self == DCOR_OPNIE_A::DCOR_OPNIE_1
    }
}
#[doc = "Field `DCOR_OPNIE` writer - DCO external resistor open circuit fault flag interrupt enable."]
pub type DCOR_OPNIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSIE_SPEC, DCOR_OPNIE_A, O>;
impl<'a, const O: u8> DCOR_OPNIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn dcor_opnie_0(self) -> &'a mut W {
        self.variant(DCOR_OPNIE_A::DCOR_OPNIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn dcor_opnie_1(self) -> &'a mut W {
        self.variant(DCOR_OPNIE_A::DCOR_OPNIE_1)
    }
}
#[doc = "Field `FCNTLFIE` reader - Start fault counter interrupt enable LFXT"]
pub type FCNTLFIE_R = crate::BitReader<FCNTLFIE_A>;
#[doc = "Start fault counter interrupt enable LFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCNTLFIE_A {
    #[doc = "0: Interrupt disabled"]
    FCNTLFIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    FCNTLFIE_1 = 1,
}
impl From<FCNTLFIE_A> for bool {
    #[inline(always)]
    fn from(variant: FCNTLFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl FCNTLFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCNTLFIE_A {
        match self.bits {
            false => FCNTLFIE_A::FCNTLFIE_0,
            true => FCNTLFIE_A::FCNTLFIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FCNTLFIE_0`"]
    #[inline(always)]
    pub fn is_fcntlfie_0(&self) -> bool {
        *self == FCNTLFIE_A::FCNTLFIE_0
    }
    #[doc = "Checks if the value of the field is `FCNTLFIE_1`"]
    #[inline(always)]
    pub fn is_fcntlfie_1(&self) -> bool {
        *self == FCNTLFIE_A::FCNTLFIE_1
    }
}
#[doc = "Field `FCNTLFIE` writer - Start fault counter interrupt enable LFXT"]
pub type FCNTLFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSIE_SPEC, FCNTLFIE_A, O>;
impl<'a, const O: u8> FCNTLFIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn fcntlfie_0(self) -> &'a mut W {
        self.variant(FCNTLFIE_A::FCNTLFIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn fcntlfie_1(self) -> &'a mut W {
        self.variant(FCNTLFIE_A::FCNTLFIE_1)
    }
}
#[doc = "Field `FCNTHFIE` reader - Start fault counter interrupt enable HFXT"]
pub type FCNTHFIE_R = crate::BitReader<FCNTHFIE_A>;
#[doc = "Start fault counter interrupt enable HFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCNTHFIE_A {
    #[doc = "0: Interrupt disabled"]
    FCNTHFIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    FCNTHFIE_1 = 1,
}
impl From<FCNTHFIE_A> for bool {
    #[inline(always)]
    fn from(variant: FCNTHFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl FCNTHFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCNTHFIE_A {
        match self.bits {
            false => FCNTHFIE_A::FCNTHFIE_0,
            true => FCNTHFIE_A::FCNTHFIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FCNTHFIE_0`"]
    #[inline(always)]
    pub fn is_fcnthfie_0(&self) -> bool {
        *self == FCNTHFIE_A::FCNTHFIE_0
    }
    #[doc = "Checks if the value of the field is `FCNTHFIE_1`"]
    #[inline(always)]
    pub fn is_fcnthfie_1(&self) -> bool {
        *self == FCNTHFIE_A::FCNTHFIE_1
    }
}
#[doc = "Field `FCNTHFIE` writer - Start fault counter interrupt enable HFXT"]
pub type FCNTHFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSIE_SPEC, FCNTHFIE_A, O>;
impl<'a, const O: u8> FCNTHFIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn fcnthfie_0(self) -> &'a mut W {
        self.variant(FCNTHFIE_A::FCNTHFIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn fcnthfie_1(self) -> &'a mut W {
        self.variant(FCNTHFIE_A::FCNTHFIE_1)
    }
}
#[doc = "Field `FCNTHF2IE` reader - Start fault counter interrupt enable HFXT2"]
pub type FCNTHF2IE_R = crate::BitReader<FCNTHF2IE_A>;
#[doc = "Start fault counter interrupt enable HFXT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCNTHF2IE_A {
    #[doc = "0: Interrupt disabled"]
    FCNTHF2IE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    FCNTHF2IE_1 = 1,
}
impl From<FCNTHF2IE_A> for bool {
    #[inline(always)]
    fn from(variant: FCNTHF2IE_A) -> Self {
        variant as u8 != 0
    }
}
impl FCNTHF2IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCNTHF2IE_A {
        match self.bits {
            false => FCNTHF2IE_A::FCNTHF2IE_0,
            true => FCNTHF2IE_A::FCNTHF2IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `FCNTHF2IE_0`"]
    #[inline(always)]
    pub fn is_fcnthf2ie_0(&self) -> bool {
        *self == FCNTHF2IE_A::FCNTHF2IE_0
    }
    #[doc = "Checks if the value of the field is `FCNTHF2IE_1`"]
    #[inline(always)]
    pub fn is_fcnthf2ie_1(&self) -> bool {
        *self == FCNTHF2IE_A::FCNTHF2IE_1
    }
}
#[doc = "Field `FCNTHF2IE` writer - Start fault counter interrupt enable HFXT2"]
pub type FCNTHF2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSIE_SPEC, FCNTHF2IE_A, O>;
impl<'a, const O: u8> FCNTHF2IE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn fcnthf2ie_0(self) -> &'a mut W {
        self.variant(FCNTHF2IE_A::FCNTHF2IE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn fcnthf2ie_1(self) -> &'a mut W {
        self.variant(FCNTHF2IE_A::FCNTHF2IE_1)
    }
}
#[doc = "Field `PLLOOLIE` reader - PLL out-of-lock interrupt enable"]
pub type PLLOOLIE_R = crate::BitReader<PLLOOLIE_A>;
#[doc = "PLL out-of-lock interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLOOLIE_A {
    #[doc = "0: Interrupt disabled"]
    PLLOOLIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    PLLOOLIE_1 = 1,
}
impl From<PLLOOLIE_A> for bool {
    #[inline(always)]
    fn from(variant: PLLOOLIE_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLOOLIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLOOLIE_A {
        match self.bits {
            false => PLLOOLIE_A::PLLOOLIE_0,
            true => PLLOOLIE_A::PLLOOLIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PLLOOLIE_0`"]
    #[inline(always)]
    pub fn is_plloolie_0(&self) -> bool {
        *self == PLLOOLIE_A::PLLOOLIE_0
    }
    #[doc = "Checks if the value of the field is `PLLOOLIE_1`"]
    #[inline(always)]
    pub fn is_plloolie_1(&self) -> bool {
        *self == PLLOOLIE_A::PLLOOLIE_1
    }
}
#[doc = "Field `PLLOOLIE` writer - PLL out-of-lock interrupt enable"]
pub type PLLOOLIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSIE_SPEC, PLLOOLIE_A, O>;
impl<'a, const O: u8> PLLOOLIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn plloolie_0(self) -> &'a mut W {
        self.variant(PLLOOLIE_A::PLLOOLIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn plloolie_1(self) -> &'a mut W {
        self.variant(PLLOOLIE_A::PLLOOLIE_1)
    }
}
#[doc = "Field `PLLLOSIE` reader - PLL loss-of-signal interrupt enable"]
pub type PLLLOSIE_R = crate::BitReader<PLLLOSIE_A>;
#[doc = "PLL loss-of-signal interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLLOSIE_A {
    #[doc = "0: Interrupt disabled"]
    PLLLOSIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    PLLLOSIE_1 = 1,
}
impl From<PLLLOSIE_A> for bool {
    #[inline(always)]
    fn from(variant: PLLLOSIE_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLLOSIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLLOSIE_A {
        match self.bits {
            false => PLLLOSIE_A::PLLLOSIE_0,
            true => PLLLOSIE_A::PLLLOSIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PLLLOSIE_0`"]
    #[inline(always)]
    pub fn is_plllosie_0(&self) -> bool {
        *self == PLLLOSIE_A::PLLLOSIE_0
    }
    #[doc = "Checks if the value of the field is `PLLLOSIE_1`"]
    #[inline(always)]
    pub fn is_plllosie_1(&self) -> bool {
        *self == PLLLOSIE_A::PLLLOSIE_1
    }
}
#[doc = "Field `PLLLOSIE` writer - PLL loss-of-signal interrupt enable"]
pub type PLLLOSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSIE_SPEC, PLLLOSIE_A, O>;
impl<'a, const O: u8> PLLLOSIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn plllosie_0(self) -> &'a mut W {
        self.variant(PLLLOSIE_A::PLLLOSIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn plllosie_1(self) -> &'a mut W {
        self.variant(PLLLOSIE_A::PLLLOSIE_1)
    }
}
#[doc = "Field `PLLOORIE` reader - PLL out-of-range interrupt enable"]
pub type PLLOORIE_R = crate::BitReader<PLLOORIE_A>;
#[doc = "PLL out-of-range interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLOORIE_A {
    #[doc = "0: Interrupt disabled"]
    PLLOORIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    PLLOORIE_1 = 1,
}
impl From<PLLOORIE_A> for bool {
    #[inline(always)]
    fn from(variant: PLLOORIE_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLOORIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLOORIE_A {
        match self.bits {
            false => PLLOORIE_A::PLLOORIE_0,
            true => PLLOORIE_A::PLLOORIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `PLLOORIE_0`"]
    #[inline(always)]
    pub fn is_plloorie_0(&self) -> bool {
        *self == PLLOORIE_A::PLLOORIE_0
    }
    #[doc = "Checks if the value of the field is `PLLOORIE_1`"]
    #[inline(always)]
    pub fn is_plloorie_1(&self) -> bool {
        *self == PLLOORIE_A::PLLOORIE_1
    }
}
#[doc = "Field `PLLOORIE` writer - PLL out-of-range interrupt enable"]
pub type PLLOORIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSIE_SPEC, PLLOORIE_A, O>;
impl<'a, const O: u8> PLLOORIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn plloorie_0(self) -> &'a mut W {
        self.variant(PLLOORIE_A::PLLOORIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn plloorie_1(self) -> &'a mut W {
        self.variant(PLLOORIE_A::PLLOORIE_1)
    }
}
#[doc = "Field `CALIE` reader - REFCNT period counter interrupt enable"]
pub type CALIE_R = crate::BitReader<CALIE_A>;
#[doc = "REFCNT period counter interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALIE_A {
    #[doc = "0: Interrupt disabled"]
    CALIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    CALIE_1 = 1,
}
impl From<CALIE_A> for bool {
    #[inline(always)]
    fn from(variant: CALIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CALIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALIE_A {
        match self.bits {
            false => CALIE_A::CALIE_0,
            true => CALIE_A::CALIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CALIE_0`"]
    #[inline(always)]
    pub fn is_calie_0(&self) -> bool {
        *self == CALIE_A::CALIE_0
    }
    #[doc = "Checks if the value of the field is `CALIE_1`"]
    #[inline(always)]
    pub fn is_calie_1(&self) -> bool {
        *self == CALIE_A::CALIE_1
    }
}
#[doc = "Field `CALIE` writer - REFCNT period counter interrupt enable"]
pub type CALIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSIE_SPEC, CALIE_A, O>;
impl<'a, const O: u8> CALIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn calie_0(self) -> &'a mut W {
        self.variant(CALIE_A::CALIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn calie_1(self) -> &'a mut W {
        self.variant(CALIE_A::CALIE_1)
    }
}
impl R {
    #[doc = "Bit 0 - LFXT oscillator fault flag interrupt enable"]
    #[inline(always)]
    pub fn lfxtie(&self) -> LFXTIE_R {
        LFXTIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HFXT oscillator fault flag interrupt enable"]
    #[inline(always)]
    pub fn hfxtie(&self) -> HFXTIE_R {
        HFXTIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HFXT2 oscillator fault flag interrupt enable"]
    #[inline(always)]
    pub fn hfxt2ie(&self) -> HFXT2IE_R {
        HFXT2IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - DCO external resistor open circuit fault flag interrupt enable."]
    #[inline(always)]
    pub fn dcor_opnie(&self) -> DCOR_OPNIE_R {
        DCOR_OPNIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Start fault counter interrupt enable LFXT"]
    #[inline(always)]
    pub fn fcntlfie(&self) -> FCNTLFIE_R {
        FCNTLFIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start fault counter interrupt enable HFXT"]
    #[inline(always)]
    pub fn fcnthfie(&self) -> FCNTHFIE_R {
        FCNTHFIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Start fault counter interrupt enable HFXT2"]
    #[inline(always)]
    pub fn fcnthf2ie(&self) -> FCNTHF2IE_R {
        FCNTHF2IE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - PLL out-of-lock interrupt enable"]
    #[inline(always)]
    pub fn plloolie(&self) -> PLLOOLIE_R {
        PLLOOLIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PLL loss-of-signal interrupt enable"]
    #[inline(always)]
    pub fn plllosie(&self) -> PLLLOSIE_R {
        PLLLOSIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PLL out-of-range interrupt enable"]
    #[inline(always)]
    pub fn plloorie(&self) -> PLLOORIE_R {
        PLLOORIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - REFCNT period counter interrupt enable"]
    #[inline(always)]
    pub fn calie(&self) -> CALIE_R {
        CALIE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LFXT oscillator fault flag interrupt enable"]
    #[inline(always)]
    pub fn lfxtie(&mut self) -> LFXTIE_W<0> {
        LFXTIE_W::new(self)
    }
    #[doc = "Bit 1 - HFXT oscillator fault flag interrupt enable"]
    #[inline(always)]
    pub fn hfxtie(&mut self) -> HFXTIE_W<1> {
        HFXTIE_W::new(self)
    }
    #[doc = "Bit 2 - HFXT2 oscillator fault flag interrupt enable"]
    #[inline(always)]
    pub fn hfxt2ie(&mut self) -> HFXT2IE_W<2> {
        HFXT2IE_W::new(self)
    }
    #[doc = "Bit 6 - DCO external resistor open circuit fault flag interrupt enable."]
    #[inline(always)]
    pub fn dcor_opnie(&mut self) -> DCOR_OPNIE_W<6> {
        DCOR_OPNIE_W::new(self)
    }
    #[doc = "Bit 8 - Start fault counter interrupt enable LFXT"]
    #[inline(always)]
    pub fn fcntlfie(&mut self) -> FCNTLFIE_W<8> {
        FCNTLFIE_W::new(self)
    }
    #[doc = "Bit 9 - Start fault counter interrupt enable HFXT"]
    #[inline(always)]
    pub fn fcnthfie(&mut self) -> FCNTHFIE_W<9> {
        FCNTHFIE_W::new(self)
    }
    #[doc = "Bit 10 - Start fault counter interrupt enable HFXT2"]
    #[inline(always)]
    pub fn fcnthf2ie(&mut self) -> FCNTHF2IE_W<10> {
        FCNTHF2IE_W::new(self)
    }
    #[doc = "Bit 12 - PLL out-of-lock interrupt enable"]
    #[inline(always)]
    pub fn plloolie(&mut self) -> PLLOOLIE_W<12> {
        PLLOOLIE_W::new(self)
    }
    #[doc = "Bit 13 - PLL loss-of-signal interrupt enable"]
    #[inline(always)]
    pub fn plllosie(&mut self) -> PLLLOSIE_W<13> {
        PLLLOSIE_W::new(self)
    }
    #[doc = "Bit 14 - PLL out-of-range interrupt enable"]
    #[inline(always)]
    pub fn plloorie(&mut self) -> PLLOORIE_W<14> {
        PLLOORIE_W::new(self)
    }
    #[doc = "Bit 15 - REFCNT period counter interrupt enable"]
    #[inline(always)]
    pub fn calie(&mut self) -> CALIE_W<15> {
        CALIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csie](index.html) module"]
pub struct CSIE_SPEC;
impl crate::RegisterSpec for CSIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csie::R](R) reader structure"]
impl crate::Readable for CSIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csie::W](W) writer structure"]
impl crate::Writable for CSIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSIE to value 0"]
impl crate::Resettable for CSIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
