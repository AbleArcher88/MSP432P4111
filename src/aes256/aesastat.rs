#[doc = "Register `AESASTAT` reader"]
pub struct R(crate::R<AESASTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESASTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AESASTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AESASTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESASTAT` writer"]
pub struct W(crate::W<AESASTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESASTAT_SPEC>;
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
impl From<crate::W<AESASTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESASTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AESBUSY` reader - AES accelerator module busy"]
pub type AESBUSY_R = crate::BitReader<AESBUSY_A>;
#[doc = "AES accelerator module busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AESBUSY_A {
    #[doc = "0: Not busy"]
    AESBUSY_0 = 0,
    #[doc = "1: Busy"]
    AESBUSY_1 = 1,
}
impl From<AESBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: AESBUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl AESBUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESBUSY_A {
        match self.bits {
            false => AESBUSY_A::AESBUSY_0,
            true => AESBUSY_A::AESBUSY_1,
        }
    }
    #[doc = "Checks if the value of the field is `AESBUSY_0`"]
    #[inline(always)]
    pub fn is_aesbusy_0(&self) -> bool {
        *self == AESBUSY_A::AESBUSY_0
    }
    #[doc = "Checks if the value of the field is `AESBUSY_1`"]
    #[inline(always)]
    pub fn is_aesbusy_1(&self) -> bool {
        *self == AESBUSY_A::AESBUSY_1
    }
}
#[doc = "Field `AESBUSY` writer - AES accelerator module busy"]
pub type AESBUSY_W<'a, const O: u8> = crate::BitWriter<'a, u16, AESASTAT_SPEC, AESBUSY_A, O>;
impl<'a, const O: u8> AESBUSY_W<'a, O> {
    #[doc = "Not busy"]
    #[inline(always)]
    pub fn aesbusy_0(self) -> &'a mut W {
        self.variant(AESBUSY_A::AESBUSY_0)
    }
    #[doc = "Busy"]
    #[inline(always)]
    pub fn aesbusy_1(self) -> &'a mut W {
        self.variant(AESBUSY_A::AESBUSY_1)
    }
}
#[doc = "Field `AESKEYWR` reader - All 16 bytes written to AESAKEY"]
pub type AESKEYWR_R = crate::BitReader<AESKEYWR_A>;
#[doc = "All 16 bytes written to AESAKEY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AESKEYWR_A {
    #[doc = "0: Not all bytes written"]
    AESKEYWR_0 = 0,
    #[doc = "1: All bytes written"]
    AESKEYWR_1 = 1,
}
impl From<AESKEYWR_A> for bool {
    #[inline(always)]
    fn from(variant: AESKEYWR_A) -> Self {
        variant as u8 != 0
    }
}
impl AESKEYWR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESKEYWR_A {
        match self.bits {
            false => AESKEYWR_A::AESKEYWR_0,
            true => AESKEYWR_A::AESKEYWR_1,
        }
    }
    #[doc = "Checks if the value of the field is `AESKEYWR_0`"]
    #[inline(always)]
    pub fn is_aeskeywr_0(&self) -> bool {
        *self == AESKEYWR_A::AESKEYWR_0
    }
    #[doc = "Checks if the value of the field is `AESKEYWR_1`"]
    #[inline(always)]
    pub fn is_aeskeywr_1(&self) -> bool {
        *self == AESKEYWR_A::AESKEYWR_1
    }
}
#[doc = "Field `AESKEYWR` writer - All 16 bytes written to AESAKEY"]
pub type AESKEYWR_W<'a, const O: u8> = crate::BitWriter<'a, u16, AESASTAT_SPEC, AESKEYWR_A, O>;
impl<'a, const O: u8> AESKEYWR_W<'a, O> {
    #[doc = "Not all bytes written"]
    #[inline(always)]
    pub fn aeskeywr_0(self) -> &'a mut W {
        self.variant(AESKEYWR_A::AESKEYWR_0)
    }
    #[doc = "All bytes written"]
    #[inline(always)]
    pub fn aeskeywr_1(self) -> &'a mut W {
        self.variant(AESKEYWR_A::AESKEYWR_1)
    }
}
#[doc = "Field `AESDINWR` reader - All 16 bytes written to AESADIN, AESAXDIN or AESAXIN"]
pub type AESDINWR_R = crate::BitReader<AESDINWR_A>;
#[doc = "All 16 bytes written to AESADIN, AESAXDIN or AESAXIN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AESDINWR_A {
    #[doc = "0: Not all bytes written"]
    AESDINWR_0 = 0,
    #[doc = "1: All bytes written"]
    AESDINWR_1 = 1,
}
impl From<AESDINWR_A> for bool {
    #[inline(always)]
    fn from(variant: AESDINWR_A) -> Self {
        variant as u8 != 0
    }
}
impl AESDINWR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESDINWR_A {
        match self.bits {
            false => AESDINWR_A::AESDINWR_0,
            true => AESDINWR_A::AESDINWR_1,
        }
    }
    #[doc = "Checks if the value of the field is `AESDINWR_0`"]
    #[inline(always)]
    pub fn is_aesdinwr_0(&self) -> bool {
        *self == AESDINWR_A::AESDINWR_0
    }
    #[doc = "Checks if the value of the field is `AESDINWR_1`"]
    #[inline(always)]
    pub fn is_aesdinwr_1(&self) -> bool {
        *self == AESDINWR_A::AESDINWR_1
    }
}
#[doc = "Field `AESDINWR` writer - All 16 bytes written to AESADIN, AESAXDIN or AESAXIN"]
pub type AESDINWR_W<'a, const O: u8> = crate::BitWriter<'a, u16, AESASTAT_SPEC, AESDINWR_A, O>;
impl<'a, const O: u8> AESDINWR_W<'a, O> {
    #[doc = "Not all bytes written"]
    #[inline(always)]
    pub fn aesdinwr_0(self) -> &'a mut W {
        self.variant(AESDINWR_A::AESDINWR_0)
    }
    #[doc = "All bytes written"]
    #[inline(always)]
    pub fn aesdinwr_1(self) -> &'a mut W {
        self.variant(AESDINWR_A::AESDINWR_1)
    }
}
#[doc = "Field `AESDOUTRD` reader - All 16 bytes read from AESADOUT"]
pub type AESDOUTRD_R = crate::BitReader<AESDOUTRD_ENUM_READ_A>;
#[doc = "All 16 bytes read from AESADOUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AESDOUTRD_ENUM_READ_A {
    #[doc = "0: Not all bytes read"]
    AESDOUTRD_0 = 0,
    #[doc = "1: All bytes read"]
    AESDOUTRD_1 = 1,
}
impl From<AESDOUTRD_ENUM_READ_A> for bool {
    #[inline(always)]
    fn from(variant: AESDOUTRD_ENUM_READ_A) -> Self {
        variant as u8 != 0
    }
}
impl AESDOUTRD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESDOUTRD_ENUM_READ_A {
        match self.bits {
            false => AESDOUTRD_ENUM_READ_A::AESDOUTRD_0,
            true => AESDOUTRD_ENUM_READ_A::AESDOUTRD_1,
        }
    }
    #[doc = "Checks if the value of the field is `AESDOUTRD_0`"]
    #[inline(always)]
    pub fn is_aesdoutrd_0(&self) -> bool {
        *self == AESDOUTRD_ENUM_READ_A::AESDOUTRD_0
    }
    #[doc = "Checks if the value of the field is `AESDOUTRD_1`"]
    #[inline(always)]
    pub fn is_aesdoutrd_1(&self) -> bool {
        *self == AESDOUTRD_ENUM_READ_A::AESDOUTRD_1
    }
}
#[doc = "Field `AESKEYCNTx` reader - Bytes written via AESAKEY for AESKLx=00, half-words written via AESAKEY"]
pub type AESKEYCNTX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AESDINCNTx` reader - Bytes written via AESADIN, AESAXDIN or AESAXIN"]
pub type AESDINCNTX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AESDOUTCNTx` reader - Bytes read via AESADOUT"]
pub type AESDOUTCNTX_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - AES accelerator module busy"]
    #[inline(always)]
    pub fn aesbusy(&self) -> AESBUSY_R {
        AESBUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - All 16 bytes written to AESAKEY"]
    #[inline(always)]
    pub fn aeskeywr(&self) -> AESKEYWR_R {
        AESKEYWR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - All 16 bytes written to AESADIN, AESAXDIN or AESAXIN"]
    #[inline(always)]
    pub fn aesdinwr(&self) -> AESDINWR_R {
        AESDINWR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - All 16 bytes read from AESADOUT"]
    #[inline(always)]
    pub fn aesdoutrd(&self) -> AESDOUTRD_R {
        AESDOUTRD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Bytes written via AESAKEY for AESKLx=00, half-words written via AESAKEY"]
    #[inline(always)]
    pub fn aeskeycntx(&self) -> AESKEYCNTX_R {
        AESKEYCNTX_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Bytes written via AESADIN, AESAXDIN or AESAXIN"]
    #[inline(always)]
    pub fn aesdincntx(&self) -> AESDINCNTX_R {
        AESDINCNTX_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Bytes read via AESADOUT"]
    #[inline(always)]
    pub fn aesdoutcntx(&self) -> AESDOUTCNTX_R {
        AESDOUTCNTX_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AES accelerator module busy"]
    #[inline(always)]
    pub fn aesbusy(&mut self) -> AESBUSY_W<0> {
        AESBUSY_W::new(self)
    }
    #[doc = "Bit 1 - All 16 bytes written to AESAKEY"]
    #[inline(always)]
    pub fn aeskeywr(&mut self) -> AESKEYWR_W<1> {
        AESKEYWR_W::new(self)
    }
    #[doc = "Bit 2 - All 16 bytes written to AESADIN, AESAXDIN or AESAXIN"]
    #[inline(always)]
    pub fn aesdinwr(&mut self) -> AESDINWR_W<2> {
        AESDINWR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Accelerator Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesastat](index.html) module"]
pub struct AESASTAT_SPEC;
impl crate::RegisterSpec for AESASTAT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [aesastat::R](R) reader structure"]
impl crate::Readable for AESASTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aesastat::W](W) writer structure"]
impl crate::Writable for AESASTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AESASTAT to value 0"]
impl crate::Resettable for AESASTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
