#[doc = "Register `REFCTL0` reader"]
pub struct R(crate::R<REFCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REFCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REFCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REFCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REFCTL0` writer"]
pub struct W(crate::W<REFCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REFCTL0_SPEC>;
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
impl From<crate::W<REFCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REFCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REFON` reader - Reference enable"]
pub type REFON_R = crate::BitReader<REFON_A>;
#[doc = "Reference enable"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFON_A {
    #[doc = "0: Disables reference if no other reference requests are pending"]
    REFON_0 = 0,
    #[doc = "1: Enables reference in static mode"]
    REFON_1 = 1,
}
impl From<REFON_A> for bool {
    #[inline(always)]
    fn from(variant: REFON_A) -> Self {
        variant as u8 != 0
    }
}
impl REFON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFON_A {
        match self.bits {
            false => REFON_A::REFON_0,
            true => REFON_A::REFON_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFON_0`"]
    #[inline(always)]
    pub fn is_refon_0(&self) -> bool {
        *self == REFON_A::REFON_0
    }
    #[doc = "Checks if the value of the field is `REFON_1`"]
    #[inline(always)]
    pub fn is_refon_1(&self) -> bool {
        *self == REFON_A::REFON_1
    }
}
#[doc = "Field `REFON` writer - Reference enable"]
pub type REFON_W<'a, const O: u8> = crate::BitWriter<'a, u16, REFCTL0_SPEC, REFON_A, O>;
impl<'a, const O: u8> REFON_W<'a, O> {
    #[doc = "Disables reference if no other reference requests are pending"]
    #[inline(always)]
    pub fn refon_0(self) -> &'a mut W {
        self.variant(REFON_A::REFON_0)
    }
    #[doc = "Enables reference in static mode"]
    #[inline(always)]
    pub fn refon_1(self) -> &'a mut W {
        self.variant(REFON_A::REFON_1)
    }
}
#[doc = "Field `REFOUT` reader - Reference output buffer"]
pub type REFOUT_R = crate::BitReader<REFOUT_A>;
#[doc = "Reference output buffer"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFOUT_A {
    #[doc = "0: Reference output not available externally"]
    REFOUT_0 = 0,
    #[doc = "1: Reference output available externally. If ADC14REFBURST = 0, output is available continuously. If ADC14REFBURST = 1, output is available only during an ADC14 conversion."]
    REFOUT_1 = 1,
}
impl From<REFOUT_A> for bool {
    #[inline(always)]
    fn from(variant: REFOUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REFOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFOUT_A {
        match self.bits {
            false => REFOUT_A::REFOUT_0,
            true => REFOUT_A::REFOUT_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFOUT_0`"]
    #[inline(always)]
    pub fn is_refout_0(&self) -> bool {
        *self == REFOUT_A::REFOUT_0
    }
    #[doc = "Checks if the value of the field is `REFOUT_1`"]
    #[inline(always)]
    pub fn is_refout_1(&self) -> bool {
        *self == REFOUT_A::REFOUT_1
    }
}
#[doc = "Field `REFOUT` writer - Reference output buffer"]
pub type REFOUT_W<'a, const O: u8> = crate::BitWriter<'a, u16, REFCTL0_SPEC, REFOUT_A, O>;
impl<'a, const O: u8> REFOUT_W<'a, O> {
    #[doc = "Reference output not available externally"]
    #[inline(always)]
    pub fn refout_0(self) -> &'a mut W {
        self.variant(REFOUT_A::REFOUT_0)
    }
    #[doc = "Reference output available externally. If ADC14REFBURST = 0, output is available continuously. If ADC14REFBURST = 1, output is available only during an ADC14 conversion."]
    #[inline(always)]
    pub fn refout_1(self) -> &'a mut W {
        self.variant(REFOUT_A::REFOUT_1)
    }
}
#[doc = "Field `REFTCOFF` reader - Temperature sensor disabled"]
pub type REFTCOFF_R = crate::BitReader<REFTCOFF_A>;
#[doc = "Temperature sensor disabled"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFTCOFF_A {
    #[doc = "0: Temperature sensor enabled"]
    REFTCOFF_0 = 0,
    #[doc = "1: Temperature sensor disabled to save power"]
    REFTCOFF_1 = 1,
}
impl From<REFTCOFF_A> for bool {
    #[inline(always)]
    fn from(variant: REFTCOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl REFTCOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFTCOFF_A {
        match self.bits {
            false => REFTCOFF_A::REFTCOFF_0,
            true => REFTCOFF_A::REFTCOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFTCOFF_0`"]
    #[inline(always)]
    pub fn is_reftcoff_0(&self) -> bool {
        *self == REFTCOFF_A::REFTCOFF_0
    }
    #[doc = "Checks if the value of the field is `REFTCOFF_1`"]
    #[inline(always)]
    pub fn is_reftcoff_1(&self) -> bool {
        *self == REFTCOFF_A::REFTCOFF_1
    }
}
#[doc = "Field `REFTCOFF` writer - Temperature sensor disabled"]
pub type REFTCOFF_W<'a, const O: u8> = crate::BitWriter<'a, u16, REFCTL0_SPEC, REFTCOFF_A, O>;
impl<'a, const O: u8> REFTCOFF_W<'a, O> {
    #[doc = "Temperature sensor enabled"]
    #[inline(always)]
    pub fn reftcoff_0(self) -> &'a mut W {
        self.variant(REFTCOFF_A::REFTCOFF_0)
    }
    #[doc = "Temperature sensor disabled to save power"]
    #[inline(always)]
    pub fn reftcoff_1(self) -> &'a mut W {
        self.variant(REFTCOFF_A::REFTCOFF_1)
    }
}
#[doc = "Field `REFVSEL` reader - Reference voltage level select"]
pub type REFVSEL_R = crate::FieldReader<u8, REFVSEL_A>;
#[doc = "Reference voltage level select"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFVSEL_A {
    #[doc = "0: 1.2 V available when reference requested or REFON = 1"]
    REFVSEL_0 = 0,
    #[doc = "1: 1.45 V available when reference requested or REFON = 1"]
    REFVSEL_1 = 1,
    #[doc = "3: 2.5 V available when reference requested or REFON = 1"]
    REFVSEL_3 = 3,
}
impl From<REFVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFVSEL_A) -> Self {
        variant as _
    }
}
impl REFVSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REFVSEL_A> {
        match self.bits {
            0 => Some(REFVSEL_A::REFVSEL_0),
            1 => Some(REFVSEL_A::REFVSEL_1),
            3 => Some(REFVSEL_A::REFVSEL_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REFVSEL_0`"]
    #[inline(always)]
    pub fn is_refvsel_0(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_0
    }
    #[doc = "Checks if the value of the field is `REFVSEL_1`"]
    #[inline(always)]
    pub fn is_refvsel_1(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_1
    }
    #[doc = "Checks if the value of the field is `REFVSEL_3`"]
    #[inline(always)]
    pub fn is_refvsel_3(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_3
    }
}
#[doc = "Field `REFVSEL` writer - Reference voltage level select"]
pub type REFVSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, REFCTL0_SPEC, u8, REFVSEL_A, 2, O>;
impl<'a, const O: u8> REFVSEL_W<'a, O> {
    #[doc = "1.2 V available when reference requested or REFON = 1"]
    #[inline(always)]
    pub fn refvsel_0(self) -> &'a mut W {
        self.variant(REFVSEL_A::REFVSEL_0)
    }
    #[doc = "1.45 V available when reference requested or REFON = 1"]
    #[inline(always)]
    pub fn refvsel_1(self) -> &'a mut W {
        self.variant(REFVSEL_A::REFVSEL_1)
    }
    #[doc = "2.5 V available when reference requested or REFON = 1"]
    #[inline(always)]
    pub fn refvsel_3(self) -> &'a mut W {
        self.variant(REFVSEL_A::REFVSEL_3)
    }
}
#[doc = "Field `REFGENOT` reader - Reference generator one-time trigger"]
pub type REFGENOT_R = crate::BitReader<REFGENOT_A>;
#[doc = "Reference generator one-time trigger"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFGENOT_A {
    #[doc = "0: No trigger"]
    REFGENOT_0 = 0,
    #[doc = "1: Generation of the reference voltage is started by writing 1 or by a hardware trigger"]
    REFGENOT_1 = 1,
}
impl From<REFGENOT_A> for bool {
    #[inline(always)]
    fn from(variant: REFGENOT_A) -> Self {
        variant as u8 != 0
    }
}
impl REFGENOT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFGENOT_A {
        match self.bits {
            false => REFGENOT_A::REFGENOT_0,
            true => REFGENOT_A::REFGENOT_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFGENOT_0`"]
    #[inline(always)]
    pub fn is_refgenot_0(&self) -> bool {
        *self == REFGENOT_A::REFGENOT_0
    }
    #[doc = "Checks if the value of the field is `REFGENOT_1`"]
    #[inline(always)]
    pub fn is_refgenot_1(&self) -> bool {
        *self == REFGENOT_A::REFGENOT_1
    }
}
#[doc = "Field `REFGENOT` writer - Reference generator one-time trigger"]
pub type REFGENOT_W<'a, const O: u8> = crate::BitWriter<'a, u16, REFCTL0_SPEC, REFGENOT_A, O>;
impl<'a, const O: u8> REFGENOT_W<'a, O> {
    #[doc = "No trigger"]
    #[inline(always)]
    pub fn refgenot_0(self) -> &'a mut W {
        self.variant(REFGENOT_A::REFGENOT_0)
    }
    #[doc = "Generation of the reference voltage is started by writing 1 or by a hardware trigger"]
    #[inline(always)]
    pub fn refgenot_1(self) -> &'a mut W {
        self.variant(REFGENOT_A::REFGENOT_1)
    }
}
#[doc = "Field `REFBGOT` reader - Bandgap and bandgap buffer one-time trigger"]
pub type REFBGOT_R = crate::BitReader<REFBGOT_A>;
#[doc = "Bandgap and bandgap buffer one-time trigger"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFBGOT_A {
    #[doc = "0: No trigger"]
    REFBGOT_0 = 0,
    #[doc = "1: Generation of the bandgap voltage is started by writing 1 or by a hardware trigger"]
    REFBGOT_1 = 1,
}
impl From<REFBGOT_A> for bool {
    #[inline(always)]
    fn from(variant: REFBGOT_A) -> Self {
        variant as u8 != 0
    }
}
impl REFBGOT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFBGOT_A {
        match self.bits {
            false => REFBGOT_A::REFBGOT_0,
            true => REFBGOT_A::REFBGOT_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFBGOT_0`"]
    #[inline(always)]
    pub fn is_refbgot_0(&self) -> bool {
        *self == REFBGOT_A::REFBGOT_0
    }
    #[doc = "Checks if the value of the field is `REFBGOT_1`"]
    #[inline(always)]
    pub fn is_refbgot_1(&self) -> bool {
        *self == REFBGOT_A::REFBGOT_1
    }
}
#[doc = "Field `REFBGOT` writer - Bandgap and bandgap buffer one-time trigger"]
pub type REFBGOT_W<'a, const O: u8> = crate::BitWriter<'a, u16, REFCTL0_SPEC, REFBGOT_A, O>;
impl<'a, const O: u8> REFBGOT_W<'a, O> {
    #[doc = "No trigger"]
    #[inline(always)]
    pub fn refbgot_0(self) -> &'a mut W {
        self.variant(REFBGOT_A::REFBGOT_0)
    }
    #[doc = "Generation of the bandgap voltage is started by writing 1 or by a hardware trigger"]
    #[inline(always)]
    pub fn refbgot_1(self) -> &'a mut W {
        self.variant(REFBGOT_A::REFBGOT_1)
    }
}
#[doc = "Field `REFGENACT` reader - Reference generator active"]
pub type REFGENACT_R = crate::BitReader<REFGENACT_ENUM_READ_A>;
#[doc = "Reference generator active"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFGENACT_ENUM_READ_A {
    #[doc = "0: Reference generator not active"]
    REFGENACT_0 = 0,
    #[doc = "1: Reference generator active"]
    REFGENACT_1 = 1,
}
impl From<REFGENACT_ENUM_READ_A> for bool {
    #[inline(always)]
    fn from(variant: REFGENACT_ENUM_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl REFGENACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFGENACT_ENUM_READ_A {
        match self.bits {
            false => REFGENACT_ENUM_READ_A::REFGENACT_0,
            true => REFGENACT_ENUM_READ_A::REFGENACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFGENACT_0`"]
    #[inline(always)]
    pub fn is_refgenact_0(&self) -> bool {
        *self == REFGENACT_ENUM_READ_A::REFGENACT_0
    }
    #[doc = "Checks if the value of the field is `REFGENACT_1`"]
    #[inline(always)]
    pub fn is_refgenact_1(&self) -> bool {
        *self == REFGENACT_ENUM_READ_A::REFGENACT_1
    }
}
#[doc = "Field `REFBGACT` reader - Reference bandgap active"]
pub type REFBGACT_R = crate::BitReader<REFBGACT_ENUM_READ_A>;
#[doc = "Reference bandgap active"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFBGACT_ENUM_READ_A {
    #[doc = "0: Reference bandgap buffer not active"]
    REFBGACT_0 = 0,
    #[doc = "1: Reference bandgap buffer active"]
    REFBGACT_1 = 1,
}
impl From<REFBGACT_ENUM_READ_A> for bool {
    #[inline(always)]
    fn from(variant: REFBGACT_ENUM_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl REFBGACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFBGACT_ENUM_READ_A {
        match self.bits {
            false => REFBGACT_ENUM_READ_A::REFBGACT_0,
            true => REFBGACT_ENUM_READ_A::REFBGACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFBGACT_0`"]
    #[inline(always)]
    pub fn is_refbgact_0(&self) -> bool {
        *self == REFBGACT_ENUM_READ_A::REFBGACT_0
    }
    #[doc = "Checks if the value of the field is `REFBGACT_1`"]
    #[inline(always)]
    pub fn is_refbgact_1(&self) -> bool {
        *self == REFBGACT_ENUM_READ_A::REFBGACT_1
    }
}
#[doc = "Field `REFGENBUSY` reader - Reference generator busy"]
pub type REFGENBUSY_R = crate::BitReader<REFGENBUSY_ENUM_READ_A>;
#[doc = "Reference generator busy"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFGENBUSY_ENUM_READ_A {
    #[doc = "0: Reference generator not busy"]
    REFGENBUSY_0 = 0,
    #[doc = "1: Reference generator busy"]
    REFGENBUSY_1 = 1,
}
impl From<REFGENBUSY_ENUM_READ_A> for bool {
    #[inline(always)]
    fn from(variant: REFGENBUSY_ENUM_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl REFGENBUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFGENBUSY_ENUM_READ_A {
        match self.bits {
            false => REFGENBUSY_ENUM_READ_A::REFGENBUSY_0,
            true => REFGENBUSY_ENUM_READ_A::REFGENBUSY_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFGENBUSY_0`"]
    #[inline(always)]
    pub fn is_refgenbusy_0(&self) -> bool {
        *self == REFGENBUSY_ENUM_READ_A::REFGENBUSY_0
    }
    #[doc = "Checks if the value of the field is `REFGENBUSY_1`"]
    #[inline(always)]
    pub fn is_refgenbusy_1(&self) -> bool {
        *self == REFGENBUSY_ENUM_READ_A::REFGENBUSY_1
    }
}
#[doc = "Field `BGMODE` reader - Bandgap mode"]
pub type BGMODE_R = crate::BitReader<BGMODE_ENUM_READ_A>;
#[doc = "Bandgap mode"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BGMODE_ENUM_READ_A {
    #[doc = "0: Static mode"]
    BGMODE_0 = 0,
    #[doc = "1: Sampled mode"]
    BGMODE_1 = 1,
}
impl From<BGMODE_ENUM_READ_A> for bool {
    #[inline(always)]
    fn from(variant: BGMODE_ENUM_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl BGMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BGMODE_ENUM_READ_A {
        match self.bits {
            false => BGMODE_ENUM_READ_A::BGMODE_0,
            true => BGMODE_ENUM_READ_A::BGMODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `BGMODE_0`"]
    #[inline(always)]
    pub fn is_bgmode_0(&self) -> bool {
        *self == BGMODE_ENUM_READ_A::BGMODE_0
    }
    #[doc = "Checks if the value of the field is `BGMODE_1`"]
    #[inline(always)]
    pub fn is_bgmode_1(&self) -> bool {
        *self == BGMODE_ENUM_READ_A::BGMODE_1
    }
}
#[doc = "Field `REFGENRDY` reader - Variable reference voltage ready status"]
pub type REFGENRDY_R = crate::BitReader<REFGENRDY_ENUM_READ_A>;
#[doc = "Variable reference voltage ready status"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFGENRDY_ENUM_READ_A {
    #[doc = "0: Reference voltage output is not ready to be used"]
    REFGENRDY_0 = 0,
    #[doc = "1: Reference voltage output is ready to be used"]
    REFGENRDY_1 = 1,
}
impl From<REFGENRDY_ENUM_READ_A> for bool {
    #[inline(always)]
    fn from(variant: REFGENRDY_ENUM_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl REFGENRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFGENRDY_ENUM_READ_A {
        match self.bits {
            false => REFGENRDY_ENUM_READ_A::REFGENRDY_0,
            true => REFGENRDY_ENUM_READ_A::REFGENRDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFGENRDY_0`"]
    #[inline(always)]
    pub fn is_refgenrdy_0(&self) -> bool {
        *self == REFGENRDY_ENUM_READ_A::REFGENRDY_0
    }
    #[doc = "Checks if the value of the field is `REFGENRDY_1`"]
    #[inline(always)]
    pub fn is_refgenrdy_1(&self) -> bool {
        *self == REFGENRDY_ENUM_READ_A::REFGENRDY_1
    }
}
#[doc = "Field `REFBGRDY` reader - Buffered bandgap voltage ready status"]
pub type REFBGRDY_R = crate::BitReader<REFBGRDY_ENUM_READ_A>;
#[doc = "Buffered bandgap voltage ready status"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFBGRDY_ENUM_READ_A {
    #[doc = "0: Buffered bandgap voltage is not ready to be used"]
    REFBGRDY_0 = 0,
    #[doc = "1: Buffered bandgap voltage is ready to be used"]
    REFBGRDY_1 = 1,
}
impl From<REFBGRDY_ENUM_READ_A> for bool {
    #[inline(always)]
    fn from(variant: REFBGRDY_ENUM_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl REFBGRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFBGRDY_ENUM_READ_A {
        match self.bits {
            false => REFBGRDY_ENUM_READ_A::REFBGRDY_0,
            true => REFBGRDY_ENUM_READ_A::REFBGRDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFBGRDY_0`"]
    #[inline(always)]
    pub fn is_refbgrdy_0(&self) -> bool {
        *self == REFBGRDY_ENUM_READ_A::REFBGRDY_0
    }
    #[doc = "Checks if the value of the field is `REFBGRDY_1`"]
    #[inline(always)]
    pub fn is_refbgrdy_1(&self) -> bool {
        *self == REFBGRDY_ENUM_READ_A::REFBGRDY_1
    }
}
impl R {
    #[doc = "Bit 0 - Reference enable"]
    #[inline(always)]
    pub fn refon(&self) -> REFON_R {
        REFON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reference output buffer"]
    #[inline(always)]
    pub fn refout(&self) -> REFOUT_R {
        REFOUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Temperature sensor disabled"]
    #[inline(always)]
    pub fn reftcoff(&self) -> REFTCOFF_R {
        REFTCOFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Reference voltage level select"]
    #[inline(always)]
    pub fn refvsel(&self) -> REFVSEL_R {
        REFVSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Reference generator one-time trigger"]
    #[inline(always)]
    pub fn refgenot(&self) -> REFGENOT_R {
        REFGENOT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bandgap and bandgap buffer one-time trigger"]
    #[inline(always)]
    pub fn refbgot(&self) -> REFBGOT_R {
        REFBGOT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reference generator active"]
    #[inline(always)]
    pub fn refgenact(&self) -> REFGENACT_R {
        REFGENACT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reference bandgap active"]
    #[inline(always)]
    pub fn refbgact(&self) -> REFBGACT_R {
        REFBGACT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reference generator busy"]
    #[inline(always)]
    pub fn refgenbusy(&self) -> REFGENBUSY_R {
        REFGENBUSY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bandgap mode"]
    #[inline(always)]
    pub fn bgmode(&self) -> BGMODE_R {
        BGMODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Variable reference voltage ready status"]
    #[inline(always)]
    pub fn refgenrdy(&self) -> REFGENRDY_R {
        REFGENRDY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Buffered bandgap voltage ready status"]
    #[inline(always)]
    pub fn refbgrdy(&self) -> REFBGRDY_R {
        REFBGRDY_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reference enable"]
    #[inline(always)]
    pub fn refon(&mut self) -> REFON_W<0> {
        REFON_W::new(self)
    }
    #[doc = "Bit 1 - Reference output buffer"]
    #[inline(always)]
    pub fn refout(&mut self) -> REFOUT_W<1> {
        REFOUT_W::new(self)
    }
    #[doc = "Bit 3 - Temperature sensor disabled"]
    #[inline(always)]
    pub fn reftcoff(&mut self) -> REFTCOFF_W<3> {
        REFTCOFF_W::new(self)
    }
    #[doc = "Bits 4:5 - Reference voltage level select"]
    #[inline(always)]
    pub fn refvsel(&mut self) -> REFVSEL_W<4> {
        REFVSEL_W::new(self)
    }
    #[doc = "Bit 6 - Reference generator one-time trigger"]
    #[inline(always)]
    pub fn refgenot(&mut self) -> REFGENOT_W<6> {
        REFGENOT_W::new(self)
    }
    #[doc = "Bit 7 - Bandgap and bandgap buffer one-time trigger"]
    #[inline(always)]
    pub fn refbgot(&mut self) -> REFBGOT_W<7> {
        REFBGOT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "REF Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refctl0](index.html) module"]
pub struct REFCTL0_SPEC;
impl crate::RegisterSpec for REFCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [refctl0::R](R) reader structure"]
impl crate::Readable for REFCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [refctl0::W](W) writer structure"]
impl crate::Writable for REFCTL0_SPEC {
    type Writer = W;
}
