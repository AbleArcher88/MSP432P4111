#[doc = "Register `AESACTL0` reader"]
pub struct R(crate::R<AESACTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESACTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AESACTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AESACTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESACTL0` writer"]
pub struct W(crate::W<AESACTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESACTL0_SPEC>;
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
impl From<crate::W<AESACTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESACTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AESOPx` reader - AES operation"]
pub type AESOPX_R = crate::FieldReader<u8, AESOPX_A>;
#[doc = "AES operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AESOPX_A {
    #[doc = "0: Encryption"]
    AESOPX_0 = 0,
    #[doc = "1: Decryption. The provided key is the same key used for encryption"]
    AESOPX_1 = 1,
    #[doc = "2: Generate first round key required for decryption"]
    AESOPX_2 = 2,
    #[doc = "3: Decryption. The provided key is the first round key required for decryption"]
    AESOPX_3 = 3,
}
impl From<AESOPX_A> for u8 {
    #[inline(always)]
    fn from(variant: AESOPX_A) -> Self {
        variant as _
    }
}
impl AESOPX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESOPX_A {
        match self.bits {
            0 => AESOPX_A::AESOPX_0,
            1 => AESOPX_A::AESOPX_1,
            2 => AESOPX_A::AESOPX_2,
            3 => AESOPX_A::AESOPX_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AESOPX_0`"]
    #[inline(always)]
    pub fn is_aesopx_0(&self) -> bool {
        *self == AESOPX_A::AESOPX_0
    }
    #[doc = "Checks if the value of the field is `AESOPX_1`"]
    #[inline(always)]
    pub fn is_aesopx_1(&self) -> bool {
        *self == AESOPX_A::AESOPX_1
    }
    #[doc = "Checks if the value of the field is `AESOPX_2`"]
    #[inline(always)]
    pub fn is_aesopx_2(&self) -> bool {
        *self == AESOPX_A::AESOPX_2
    }
    #[doc = "Checks if the value of the field is `AESOPX_3`"]
    #[inline(always)]
    pub fn is_aesopx_3(&self) -> bool {
        *self == AESOPX_A::AESOPX_3
    }
}
#[doc = "Field `AESOPx` writer - AES operation"]
pub type AESOPX_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, AESACTL0_SPEC, u8, AESOPX_A, 2, O>;
impl<'a, const O: u8> AESOPX_W<'a, O> {
    #[doc = "Encryption"]
    #[inline(always)]
    pub fn aesopx_0(self) -> &'a mut W {
        self.variant(AESOPX_A::AESOPX_0)
    }
    #[doc = "Decryption. The provided key is the same key used for encryption"]
    #[inline(always)]
    pub fn aesopx_1(self) -> &'a mut W {
        self.variant(AESOPX_A::AESOPX_1)
    }
    #[doc = "Generate first round key required for decryption"]
    #[inline(always)]
    pub fn aesopx_2(self) -> &'a mut W {
        self.variant(AESOPX_A::AESOPX_2)
    }
    #[doc = "Decryption. The provided key is the first round key required for decryption"]
    #[inline(always)]
    pub fn aesopx_3(self) -> &'a mut W {
        self.variant(AESOPX_A::AESOPX_3)
    }
}
#[doc = "Field `AESKLx` reader - AES key length"]
pub type AESKLX_R = crate::FieldReader<u8, AESKLX_A>;
#[doc = "AES key length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AESKLX_A {
    #[doc = "0: AES128. The key size is 128 bit"]
    AESKLX_0 = 0,
    #[doc = "1: AES192. The key size is 192 bit."]
    AESKLX_1 = 1,
    #[doc = "2: AES256. The key size is 256 bit"]
    AESKLX_2 = 2,
}
impl From<AESKLX_A> for u8 {
    #[inline(always)]
    fn from(variant: AESKLX_A) -> Self {
        variant as _
    }
}
impl AESKLX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AESKLX_A> {
        match self.bits {
            0 => Some(AESKLX_A::AESKLX_0),
            1 => Some(AESKLX_A::AESKLX_1),
            2 => Some(AESKLX_A::AESKLX_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AESKLX_0`"]
    #[inline(always)]
    pub fn is_aesklx_0(&self) -> bool {
        *self == AESKLX_A::AESKLX_0
    }
    #[doc = "Checks if the value of the field is `AESKLX_1`"]
    #[inline(always)]
    pub fn is_aesklx_1(&self) -> bool {
        *self == AESKLX_A::AESKLX_1
    }
    #[doc = "Checks if the value of the field is `AESKLX_2`"]
    #[inline(always)]
    pub fn is_aesklx_2(&self) -> bool {
        *self == AESKLX_A::AESKLX_2
    }
}
#[doc = "Field `AESKLx` writer - AES key length"]
pub type AESKLX_W<'a, const O: u8> = crate::FieldWriter<'a, u16, AESACTL0_SPEC, u8, AESKLX_A, 2, O>;
impl<'a, const O: u8> AESKLX_W<'a, O> {
    #[doc = "AES128. The key size is 128 bit"]
    #[inline(always)]
    pub fn aesklx_0(self) -> &'a mut W {
        self.variant(AESKLX_A::AESKLX_0)
    }
    #[doc = "AES192. The key size is 192 bit."]
    #[inline(always)]
    pub fn aesklx_1(self) -> &'a mut W {
        self.variant(AESKLX_A::AESKLX_1)
    }
    #[doc = "AES256. The key size is 256 bit"]
    #[inline(always)]
    pub fn aesklx_2(self) -> &'a mut W {
        self.variant(AESKLX_A::AESKLX_2)
    }
}
#[doc = "Field `AESCMx` reader - AES cipher mode select"]
pub type AESCMX_R = crate::FieldReader<u8, AESCMX_A>;
#[doc = "AES cipher mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AESCMX_A {
    #[doc = "0: ECB"]
    AESCMX_0 = 0,
    #[doc = "1: CBC"]
    AESCMX_1 = 1,
    #[doc = "2: OFB"]
    AESCMX_2 = 2,
    #[doc = "3: CFB"]
    AESCMX_3 = 3,
}
impl From<AESCMX_A> for u8 {
    #[inline(always)]
    fn from(variant: AESCMX_A) -> Self {
        variant as _
    }
}
impl AESCMX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESCMX_A {
        match self.bits {
            0 => AESCMX_A::AESCMX_0,
            1 => AESCMX_A::AESCMX_1,
            2 => AESCMX_A::AESCMX_2,
            3 => AESCMX_A::AESCMX_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AESCMX_0`"]
    #[inline(always)]
    pub fn is_aescmx_0(&self) -> bool {
        *self == AESCMX_A::AESCMX_0
    }
    #[doc = "Checks if the value of the field is `AESCMX_1`"]
    #[inline(always)]
    pub fn is_aescmx_1(&self) -> bool {
        *self == AESCMX_A::AESCMX_1
    }
    #[doc = "Checks if the value of the field is `AESCMX_2`"]
    #[inline(always)]
    pub fn is_aescmx_2(&self) -> bool {
        *self == AESCMX_A::AESCMX_2
    }
    #[doc = "Checks if the value of the field is `AESCMX_3`"]
    #[inline(always)]
    pub fn is_aescmx_3(&self) -> bool {
        *self == AESCMX_A::AESCMX_3
    }
}
#[doc = "Field `AESCMx` writer - AES cipher mode select"]
pub type AESCMX_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, AESACTL0_SPEC, u8, AESCMX_A, 2, O>;
impl<'a, const O: u8> AESCMX_W<'a, O> {
    #[doc = "ECB"]
    #[inline(always)]
    pub fn aescmx_0(self) -> &'a mut W {
        self.variant(AESCMX_A::AESCMX_0)
    }
    #[doc = "CBC"]
    #[inline(always)]
    pub fn aescmx_1(self) -> &'a mut W {
        self.variant(AESCMX_A::AESCMX_1)
    }
    #[doc = "OFB"]
    #[inline(always)]
    pub fn aescmx_2(self) -> &'a mut W {
        self.variant(AESCMX_A::AESCMX_2)
    }
    #[doc = "CFB"]
    #[inline(always)]
    pub fn aescmx_3(self) -> &'a mut W {
        self.variant(AESCMX_A::AESCMX_3)
    }
}
#[doc = "Field `AESSWRST` reader - AES software reset"]
pub type AESSWRST_R = crate::BitReader<AESSWRST_A>;
#[doc = "AES software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AESSWRST_A {
    #[doc = "0: No reset"]
    AESSWRST_0 = 0,
    #[doc = "1: Reset AES accelerator module"]
    AESSWRST_1 = 1,
}
impl From<AESSWRST_A> for bool {
    #[inline(always)]
    fn from(variant: AESSWRST_A) -> Self {
        variant as u8 != 0
    }
}
impl AESSWRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESSWRST_A {
        match self.bits {
            false => AESSWRST_A::AESSWRST_0,
            true => AESSWRST_A::AESSWRST_1,
        }
    }
    #[doc = "Checks if the value of the field is `AESSWRST_0`"]
    #[inline(always)]
    pub fn is_aesswrst_0(&self) -> bool {
        *self == AESSWRST_A::AESSWRST_0
    }
    #[doc = "Checks if the value of the field is `AESSWRST_1`"]
    #[inline(always)]
    pub fn is_aesswrst_1(&self) -> bool {
        *self == AESSWRST_A::AESSWRST_1
    }
}
#[doc = "Field `AESSWRST` writer - AES software reset"]
pub type AESSWRST_W<'a, const O: u8> = crate::BitWriter<'a, u16, AESACTL0_SPEC, AESSWRST_A, O>;
impl<'a, const O: u8> AESSWRST_W<'a, O> {
    #[doc = "No reset"]
    #[inline(always)]
    pub fn aesswrst_0(self) -> &'a mut W {
        self.variant(AESSWRST_A::AESSWRST_0)
    }
    #[doc = "Reset AES accelerator module"]
    #[inline(always)]
    pub fn aesswrst_1(self) -> &'a mut W {
        self.variant(AESSWRST_A::AESSWRST_1)
    }
}
#[doc = "Field `AESRDYIFG` reader - AES ready interrupt flag"]
pub type AESRDYIFG_R = crate::BitReader<AESRDYIFG_A>;
#[doc = "AES ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AESRDYIFG_A {
    #[doc = "0: No interrupt pending"]
    AESRDYIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    AESRDYIFG_1 = 1,
}
impl From<AESRDYIFG_A> for bool {
    #[inline(always)]
    fn from(variant: AESRDYIFG_A) -> Self {
        variant as u8 != 0
    }
}
impl AESRDYIFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESRDYIFG_A {
        match self.bits {
            false => AESRDYIFG_A::AESRDYIFG_0,
            true => AESRDYIFG_A::AESRDYIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `AESRDYIFG_0`"]
    #[inline(always)]
    pub fn is_aesrdyifg_0(&self) -> bool {
        *self == AESRDYIFG_A::AESRDYIFG_0
    }
    #[doc = "Checks if the value of the field is `AESRDYIFG_1`"]
    #[inline(always)]
    pub fn is_aesrdyifg_1(&self) -> bool {
        *self == AESRDYIFG_A::AESRDYIFG_1
    }
}
#[doc = "Field `AESRDYIFG` writer - AES ready interrupt flag"]
pub type AESRDYIFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, AESACTL0_SPEC, AESRDYIFG_A, O>;
impl<'a, const O: u8> AESRDYIFG_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn aesrdyifg_0(self) -> &'a mut W {
        self.variant(AESRDYIFG_A::AESRDYIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn aesrdyifg_1(self) -> &'a mut W {
        self.variant(AESRDYIFG_A::AESRDYIFG_1)
    }
}
#[doc = "Field `AESERRFG` reader - AES error flag"]
pub type AESERRFG_R = crate::BitReader<AESERRFG_A>;
#[doc = "AES error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AESERRFG_A {
    #[doc = "0: No error"]
    AESERRFG_0 = 0,
    #[doc = "1: Error occurred"]
    AESERRFG_1 = 1,
}
impl From<AESERRFG_A> for bool {
    #[inline(always)]
    fn from(variant: AESERRFG_A) -> Self {
        variant as u8 != 0
    }
}
impl AESERRFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESERRFG_A {
        match self.bits {
            false => AESERRFG_A::AESERRFG_0,
            true => AESERRFG_A::AESERRFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `AESERRFG_0`"]
    #[inline(always)]
    pub fn is_aeserrfg_0(&self) -> bool {
        *self == AESERRFG_A::AESERRFG_0
    }
    #[doc = "Checks if the value of the field is `AESERRFG_1`"]
    #[inline(always)]
    pub fn is_aeserrfg_1(&self) -> bool {
        *self == AESERRFG_A::AESERRFG_1
    }
}
#[doc = "Field `AESERRFG` writer - AES error flag"]
pub type AESERRFG_W<'a, const O: u8> = crate::BitWriter<'a, u16, AESACTL0_SPEC, AESERRFG_A, O>;
impl<'a, const O: u8> AESERRFG_W<'a, O> {
    #[doc = "No error"]
    #[inline(always)]
    pub fn aeserrfg_0(self) -> &'a mut W {
        self.variant(AESERRFG_A::AESERRFG_0)
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn aeserrfg_1(self) -> &'a mut W {
        self.variant(AESERRFG_A::AESERRFG_1)
    }
}
#[doc = "Field `AESRDYIE` reader - AES ready interrupt enable"]
pub type AESRDYIE_R = crate::BitReader<AESRDYIE_A>;
#[doc = "AES ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AESRDYIE_A {
    #[doc = "0: Interrupt disabled"]
    AESRDYIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    AESRDYIE_1 = 1,
}
impl From<AESRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: AESRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
impl AESRDYIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESRDYIE_A {
        match self.bits {
            false => AESRDYIE_A::AESRDYIE_0,
            true => AESRDYIE_A::AESRDYIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `AESRDYIE_0`"]
    #[inline(always)]
    pub fn is_aesrdyie_0(&self) -> bool {
        *self == AESRDYIE_A::AESRDYIE_0
    }
    #[doc = "Checks if the value of the field is `AESRDYIE_1`"]
    #[inline(always)]
    pub fn is_aesrdyie_1(&self) -> bool {
        *self == AESRDYIE_A::AESRDYIE_1
    }
}
#[doc = "Field `AESRDYIE` writer - AES ready interrupt enable"]
pub type AESRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, AESACTL0_SPEC, AESRDYIE_A, O>;
impl<'a, const O: u8> AESRDYIE_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn aesrdyie_0(self) -> &'a mut W {
        self.variant(AESRDYIE_A::AESRDYIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn aesrdyie_1(self) -> &'a mut W {
        self.variant(AESRDYIE_A::AESRDYIE_1)
    }
}
#[doc = "Field `AESCMEN` reader - AES cipher mode enable"]
pub type AESCMEN_R = crate::BitReader<AESCMEN_A>;
#[doc = "AES cipher mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AESCMEN_A {
    #[doc = "0: No DMA triggers are generated"]
    AESCMEN_0 = 0,
    #[doc = "1: DMA ciphermode support operation is enabled and the corresponding DMA triggers are generated"]
    AESCMEN_1 = 1,
}
impl From<AESCMEN_A> for bool {
    #[inline(always)]
    fn from(variant: AESCMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AESCMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESCMEN_A {
        match self.bits {
            false => AESCMEN_A::AESCMEN_0,
            true => AESCMEN_A::AESCMEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `AESCMEN_0`"]
    #[inline(always)]
    pub fn is_aescmen_0(&self) -> bool {
        *self == AESCMEN_A::AESCMEN_0
    }
    #[doc = "Checks if the value of the field is `AESCMEN_1`"]
    #[inline(always)]
    pub fn is_aescmen_1(&self) -> bool {
        *self == AESCMEN_A::AESCMEN_1
    }
}
#[doc = "Field `AESCMEN` writer - AES cipher mode enable"]
pub type AESCMEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, AESACTL0_SPEC, AESCMEN_A, O>;
impl<'a, const O: u8> AESCMEN_W<'a, O> {
    #[doc = "No DMA triggers are generated"]
    #[inline(always)]
    pub fn aescmen_0(self) -> &'a mut W {
        self.variant(AESCMEN_A::AESCMEN_0)
    }
    #[doc = "DMA ciphermode support operation is enabled and the corresponding DMA triggers are generated"]
    #[inline(always)]
    pub fn aescmen_1(self) -> &'a mut W {
        self.variant(AESCMEN_A::AESCMEN_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - AES operation"]
    #[inline(always)]
    pub fn aesopx(&self) -> AESOPX_R {
        AESOPX_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - AES key length"]
    #[inline(always)]
    pub fn aesklx(&self) -> AESKLX_R {
        AESKLX_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 5:6 - AES cipher mode select"]
    #[inline(always)]
    pub fn aescmx(&self) -> AESCMX_R {
        AESCMX_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - AES software reset"]
    #[inline(always)]
    pub fn aesswrst(&self) -> AESSWRST_R {
        AESSWRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AES ready interrupt flag"]
    #[inline(always)]
    pub fn aesrdyifg(&self) -> AESRDYIFG_R {
        AESRDYIFG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - AES error flag"]
    #[inline(always)]
    pub fn aeserrfg(&self) -> AESERRFG_R {
        AESERRFG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AES ready interrupt enable"]
    #[inline(always)]
    pub fn aesrdyie(&self) -> AESRDYIE_R {
        AESRDYIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - AES cipher mode enable"]
    #[inline(always)]
    pub fn aescmen(&self) -> AESCMEN_R {
        AESCMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - AES operation"]
    #[inline(always)]
    pub fn aesopx(&mut self) -> AESOPX_W<0> {
        AESOPX_W::new(self)
    }
    #[doc = "Bits 2:3 - AES key length"]
    #[inline(always)]
    pub fn aesklx(&mut self) -> AESKLX_W<2> {
        AESKLX_W::new(self)
    }
    #[doc = "Bits 5:6 - AES cipher mode select"]
    #[inline(always)]
    pub fn aescmx(&mut self) -> AESCMX_W<5> {
        AESCMX_W::new(self)
    }
    #[doc = "Bit 7 - AES software reset"]
    #[inline(always)]
    pub fn aesswrst(&mut self) -> AESSWRST_W<7> {
        AESSWRST_W::new(self)
    }
    #[doc = "Bit 8 - AES ready interrupt flag"]
    #[inline(always)]
    pub fn aesrdyifg(&mut self) -> AESRDYIFG_W<8> {
        AESRDYIFG_W::new(self)
    }
    #[doc = "Bit 11 - AES error flag"]
    #[inline(always)]
    pub fn aeserrfg(&mut self) -> AESERRFG_W<11> {
        AESERRFG_W::new(self)
    }
    #[doc = "Bit 12 - AES ready interrupt enable"]
    #[inline(always)]
    pub fn aesrdyie(&mut self) -> AESRDYIE_W<12> {
        AESRDYIE_W::new(self)
    }
    #[doc = "Bit 15 - AES cipher mode enable"]
    #[inline(always)]
    pub fn aescmen(&mut self) -> AESCMEN_W<15> {
        AESCMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Accelerator Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesactl0](index.html) module"]
pub struct AESACTL0_SPEC;
impl crate::RegisterSpec for AESACTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [aesactl0::R](R) reader structure"]
impl crate::Readable for AESACTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aesactl0::W](W) writer structure"]
impl crate::Writable for AESACTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AESACTL0 to value 0"]
impl crate::Resettable for AESACTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
