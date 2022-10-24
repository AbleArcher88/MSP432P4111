#[doc = "Register `UCBxSTATW` reader"]
pub struct R(crate::R<UCBX_STATW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCBX_STATW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCBX_STATW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCBX_STATW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCBxSTATW` writer"]
pub struct W(crate::W<UCBX_STATW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCBX_STATW_SPEC>;
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
impl From<crate::W<UCBX_STATW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCBX_STATW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCBBUSY` reader - Bus busy"]
pub type UCBBUSY_R = crate::BitReader<UCBBUSY_ENUM_READ_A>;
#[doc = "Bus busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCBBUSY_ENUM_READ_A {
    #[doc = "0: Bus inactive"]
    UCBBUSY_0 = 0,
    #[doc = "1: Bus busy"]
    UCBBUSY_1 = 1,
}
impl From<UCBBUSY_ENUM_READ_A> for bool {
    #[inline(always)]
    fn from(variant: UCBBUSY_ENUM_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl UCBBUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCBBUSY_ENUM_READ_A {
        match self.bits {
            false => UCBBUSY_ENUM_READ_A::UCBBUSY_0,
            true => UCBBUSY_ENUM_READ_A::UCBBUSY_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCBBUSY_0`"]
    #[inline(always)]
    pub fn is_ucbbusy_0(&self) -> bool {
        *self == UCBBUSY_ENUM_READ_A::UCBBUSY_0
    }
    #[doc = "Checks if the value of the field is `UCBBUSY_1`"]
    #[inline(always)]
    pub fn is_ucbbusy_1(&self) -> bool {
        *self == UCBBUSY_ENUM_READ_A::UCBBUSY_1
    }
}
#[doc = "Field `UCGC` reader - General call address received"]
pub type UCGC_R = crate::BitReader<UCGC_ENUM_READ_A>;
#[doc = "General call address received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCGC_ENUM_READ_A {
    #[doc = "0: No general call address received"]
    UCGC_0 = 0,
    #[doc = "1: General call address received"]
    UCGC_1 = 1,
}
impl From<UCGC_ENUM_READ_A> for bool {
    #[inline(always)]
    fn from(variant: UCGC_ENUM_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl UCGC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCGC_ENUM_READ_A {
        match self.bits {
            false => UCGC_ENUM_READ_A::UCGC_0,
            true => UCGC_ENUM_READ_A::UCGC_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCGC_0`"]
    #[inline(always)]
    pub fn is_ucgc_0(&self) -> bool {
        *self == UCGC_ENUM_READ_A::UCGC_0
    }
    #[doc = "Checks if the value of the field is `UCGC_1`"]
    #[inline(always)]
    pub fn is_ucgc_1(&self) -> bool {
        *self == UCGC_ENUM_READ_A::UCGC_1
    }
}
#[doc = "Field `UCSCLLOW` reader - SCL low"]
pub type UCSCLLOW_R = crate::BitReader<UCSCLLOW_ENUM_READ_A>;
#[doc = "SCL low\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UCSCLLOW_ENUM_READ_A {
    #[doc = "0: SCL is not held low"]
    UCSCLLOW_0 = 0,
    #[doc = "1: SCL is held low"]
    UCSCLLOW_1 = 1,
}
impl From<UCSCLLOW_ENUM_READ_A> for bool {
    #[inline(always)]
    fn from(variant: UCSCLLOW_ENUM_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl UCSCLLOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSCLLOW_ENUM_READ_A {
        match self.bits {
            false => UCSCLLOW_ENUM_READ_A::UCSCLLOW_0,
            true => UCSCLLOW_ENUM_READ_A::UCSCLLOW_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCSCLLOW_0`"]
    #[inline(always)]
    pub fn is_ucscllow_0(&self) -> bool {
        *self == UCSCLLOW_ENUM_READ_A::UCSCLLOW_0
    }
    #[doc = "Checks if the value of the field is `UCSCLLOW_1`"]
    #[inline(always)]
    pub fn is_ucscllow_1(&self) -> bool {
        *self == UCSCLLOW_ENUM_READ_A::UCSCLLOW_1
    }
}
#[doc = "Field `UCBCNT` reader - Hardware byte counter value"]
pub type UCBCNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 4 - Bus busy"]
    #[inline(always)]
    pub fn ucbbusy(&self) -> UCBBUSY_R {
        UCBBUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - General call address received"]
    #[inline(always)]
    pub fn ucgc(&self) -> UCGC_R {
        UCGC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCL low"]
    #[inline(always)]
    pub fn ucscllow(&self) -> UCSCLLOW_R {
        UCSCLLOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Hardware byte counter value"]
    #[inline(always)]
    pub fn ucbcnt(&self) -> UCBCNT_R {
        UCBCNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Bx Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_statw](index.html) module"]
pub struct UCBX_STATW_SPEC;
impl crate::RegisterSpec for UCBX_STATW_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucbx_statw::R](R) reader structure"]
impl crate::Readable for UCBX_STATW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucbx_statw::W](W) writer structure"]
impl crate::Writable for UCBX_STATW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCBxSTATW to value 0"]
impl crate::Resettable for UCBX_STATW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
