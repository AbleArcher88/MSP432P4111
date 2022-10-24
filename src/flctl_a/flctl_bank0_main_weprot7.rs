#[doc = "Register `FLCTL_BANK0_MAIN_WEPROT7` reader"]
pub struct R(crate::R<FLCTL_BANK0_MAIN_WEPROT7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_BANK0_MAIN_WEPROT7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_BANK0_MAIN_WEPROT7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_BANK0_MAIN_WEPROT7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLCTL_BANK0_MAIN_WEPROT7` writer"]
pub struct W(crate::W<FLCTL_BANK0_MAIN_WEPROT7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLCTL_BANK0_MAIN_WEPROT7_SPEC>;
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
impl From<crate::W<FLCTL_BANK0_MAIN_WEPROT7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLCTL_BANK0_MAIN_WEPROT7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROT224` reader - Protects Sector 224 from program or erase"]
pub type PROT224_R = crate::BitReader<bool>;
#[doc = "Field `PROT224` writer - Protects Sector 224 from program or erase"]
pub type PROT224_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT225` reader - Protects Sector 225 from program or erase"]
pub type PROT225_R = crate::BitReader<bool>;
#[doc = "Field `PROT225` writer - Protects Sector 225 from program or erase"]
pub type PROT225_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT226` reader - Protects Sector 226 from program or erase"]
pub type PROT226_R = crate::BitReader<bool>;
#[doc = "Field `PROT226` writer - Protects Sector 226 from program or erase"]
pub type PROT226_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT227` reader - Protects Sector 227 from program or erase"]
pub type PROT227_R = crate::BitReader<bool>;
#[doc = "Field `PROT227` writer - Protects Sector 227 from program or erase"]
pub type PROT227_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT228` reader - Protects Sector 228 from program or erase"]
pub type PROT228_R = crate::BitReader<bool>;
#[doc = "Field `PROT228` writer - Protects Sector 228 from program or erase"]
pub type PROT228_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT229` reader - Protects Sector 229 from program or erase"]
pub type PROT229_R = crate::BitReader<bool>;
#[doc = "Field `PROT229` writer - Protects Sector 229 from program or erase"]
pub type PROT229_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT230` reader - Protects Sector 230 from program or erase"]
pub type PROT230_R = crate::BitReader<bool>;
#[doc = "Field `PROT230` writer - Protects Sector 230 from program or erase"]
pub type PROT230_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT231` reader - Protects Sector 231 from program or erase"]
pub type PROT231_R = crate::BitReader<bool>;
#[doc = "Field `PROT231` writer - Protects Sector 231 from program or erase"]
pub type PROT231_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT232` reader - Protects Sector 232 from program or erase"]
pub type PROT232_R = crate::BitReader<bool>;
#[doc = "Field `PROT232` writer - Protects Sector 232 from program or erase"]
pub type PROT232_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT233` reader - Protects Sector 233 from program or erase"]
pub type PROT233_R = crate::BitReader<bool>;
#[doc = "Field `PROT233` writer - Protects Sector 233 from program or erase"]
pub type PROT233_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT234` reader - Protects Sector 234 from program or erase"]
pub type PROT234_R = crate::BitReader<bool>;
#[doc = "Field `PROT234` writer - Protects Sector 234 from program or erase"]
pub type PROT234_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT235` reader - Protects Sector 235 from program or erase"]
pub type PROT235_R = crate::BitReader<bool>;
#[doc = "Field `PROT235` writer - Protects Sector 235 from program or erase"]
pub type PROT235_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT236` reader - Protects Sector 236 from program or erase"]
pub type PROT236_R = crate::BitReader<bool>;
#[doc = "Field `PROT236` writer - Protects Sector 236 from program or erase"]
pub type PROT236_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT237` reader - Protects Sector 237 from program or erase"]
pub type PROT237_R = crate::BitReader<bool>;
#[doc = "Field `PROT237` writer - Protects Sector 237 from program or erase"]
pub type PROT237_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT238` reader - Protects Sector 238 from program or erase"]
pub type PROT238_R = crate::BitReader<bool>;
#[doc = "Field `PROT238` writer - Protects Sector 238 from program or erase"]
pub type PROT238_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT239` reader - Protects Sector 239 from program or erase"]
pub type PROT239_R = crate::BitReader<bool>;
#[doc = "Field `PROT239` writer - Protects Sector 239 from program or erase"]
pub type PROT239_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT240` reader - Protects Sector 240 from program or erase"]
pub type PROT240_R = crate::BitReader<bool>;
#[doc = "Field `PROT240` writer - Protects Sector 240 from program or erase"]
pub type PROT240_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT241` reader - Protects Sector 241 from program or erase"]
pub type PROT241_R = crate::BitReader<bool>;
#[doc = "Field `PROT241` writer - Protects Sector 241 from program or erase"]
pub type PROT241_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT242` reader - Protects Sector 242 from program or erase"]
pub type PROT242_R = crate::BitReader<bool>;
#[doc = "Field `PROT242` writer - Protects Sector 242 from program or erase"]
pub type PROT242_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT243` reader - Protects Sector 243 from program or erase"]
pub type PROT243_R = crate::BitReader<bool>;
#[doc = "Field `PROT243` writer - Protects Sector 243 from program or erase"]
pub type PROT243_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT244` reader - Protects Sector 244 from program or erase"]
pub type PROT244_R = crate::BitReader<bool>;
#[doc = "Field `PROT244` writer - Protects Sector 244 from program or erase"]
pub type PROT244_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT245` reader - Protects Sector 245 from program or erase"]
pub type PROT245_R = crate::BitReader<bool>;
#[doc = "Field `PROT245` writer - Protects Sector 245 from program or erase"]
pub type PROT245_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT246` reader - Protects Sector 246 from program or erase"]
pub type PROT246_R = crate::BitReader<bool>;
#[doc = "Field `PROT246` writer - Protects Sector 246 from program or erase"]
pub type PROT246_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT247` reader - Protects Sector 247 from program or erase"]
pub type PROT247_R = crate::BitReader<bool>;
#[doc = "Field `PROT247` writer - Protects Sector 247 from program or erase"]
pub type PROT247_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT248` reader - Protects Sector 248 from program or erase"]
pub type PROT248_R = crate::BitReader<bool>;
#[doc = "Field `PROT248` writer - Protects Sector 248 from program or erase"]
pub type PROT248_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT249` reader - Protects Sector 249 from program or erase"]
pub type PROT249_R = crate::BitReader<bool>;
#[doc = "Field `PROT249` writer - Protects Sector 249 from program or erase"]
pub type PROT249_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT250` reader - Protects Sector 250 from program or erase"]
pub type PROT250_R = crate::BitReader<bool>;
#[doc = "Field `PROT250` writer - Protects Sector 250 from program or erase"]
pub type PROT250_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT251` reader - Protects Sector 251 from program or erase"]
pub type PROT251_R = crate::BitReader<bool>;
#[doc = "Field `PROT251` writer - Protects Sector 251 from program or erase"]
pub type PROT251_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT252` reader - Protects Sector 252 from program or erase"]
pub type PROT252_R = crate::BitReader<bool>;
#[doc = "Field `PROT252` writer - Protects Sector 252 from program or erase"]
pub type PROT252_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT253` reader - Protects Sector 253 from program or erase"]
pub type PROT253_R = crate::BitReader<bool>;
#[doc = "Field `PROT253` writer - Protects Sector 253 from program or erase"]
pub type PROT253_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT254` reader - Protects Sector 254 from program or erase"]
pub type PROT254_R = crate::BitReader<bool>;
#[doc = "Field `PROT254` writer - Protects Sector 254 from program or erase"]
pub type PROT254_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
#[doc = "Field `PROT255` reader - Protects Sector 255 from program or erase"]
pub type PROT255_R = crate::BitReader<bool>;
#[doc = "Field `PROT255` writer - Protects Sector 255 from program or erase"]
pub type PROT255_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT7_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Protects Sector 224 from program or erase"]
    #[inline(always)]
    pub fn prot224(&self) -> PROT224_R {
        PROT224_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Protects Sector 225 from program or erase"]
    #[inline(always)]
    pub fn prot225(&self) -> PROT225_R {
        PROT225_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Protects Sector 226 from program or erase"]
    #[inline(always)]
    pub fn prot226(&self) -> PROT226_R {
        PROT226_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Protects Sector 227 from program or erase"]
    #[inline(always)]
    pub fn prot227(&self) -> PROT227_R {
        PROT227_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Protects Sector 228 from program or erase"]
    #[inline(always)]
    pub fn prot228(&self) -> PROT228_R {
        PROT228_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Protects Sector 229 from program or erase"]
    #[inline(always)]
    pub fn prot229(&self) -> PROT229_R {
        PROT229_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Protects Sector 230 from program or erase"]
    #[inline(always)]
    pub fn prot230(&self) -> PROT230_R {
        PROT230_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Protects Sector 231 from program or erase"]
    #[inline(always)]
    pub fn prot231(&self) -> PROT231_R {
        PROT231_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Protects Sector 232 from program or erase"]
    #[inline(always)]
    pub fn prot232(&self) -> PROT232_R {
        PROT232_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Protects Sector 233 from program or erase"]
    #[inline(always)]
    pub fn prot233(&self) -> PROT233_R {
        PROT233_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Protects Sector 234 from program or erase"]
    #[inline(always)]
    pub fn prot234(&self) -> PROT234_R {
        PROT234_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Protects Sector 235 from program or erase"]
    #[inline(always)]
    pub fn prot235(&self) -> PROT235_R {
        PROT235_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Protects Sector 236 from program or erase"]
    #[inline(always)]
    pub fn prot236(&self) -> PROT236_R {
        PROT236_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Protects Sector 237 from program or erase"]
    #[inline(always)]
    pub fn prot237(&self) -> PROT237_R {
        PROT237_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Protects Sector 238 from program or erase"]
    #[inline(always)]
    pub fn prot238(&self) -> PROT238_R {
        PROT238_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Protects Sector 239 from program or erase"]
    #[inline(always)]
    pub fn prot239(&self) -> PROT239_R {
        PROT239_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Protects Sector 240 from program or erase"]
    #[inline(always)]
    pub fn prot240(&self) -> PROT240_R {
        PROT240_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Protects Sector 241 from program or erase"]
    #[inline(always)]
    pub fn prot241(&self) -> PROT241_R {
        PROT241_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Protects Sector 242 from program or erase"]
    #[inline(always)]
    pub fn prot242(&self) -> PROT242_R {
        PROT242_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Protects Sector 243 from program or erase"]
    #[inline(always)]
    pub fn prot243(&self) -> PROT243_R {
        PROT243_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Protects Sector 244 from program or erase"]
    #[inline(always)]
    pub fn prot244(&self) -> PROT244_R {
        PROT244_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Protects Sector 245 from program or erase"]
    #[inline(always)]
    pub fn prot245(&self) -> PROT245_R {
        PROT245_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Protects Sector 246 from program or erase"]
    #[inline(always)]
    pub fn prot246(&self) -> PROT246_R {
        PROT246_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Protects Sector 247 from program or erase"]
    #[inline(always)]
    pub fn prot247(&self) -> PROT247_R {
        PROT247_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Protects Sector 248 from program or erase"]
    #[inline(always)]
    pub fn prot248(&self) -> PROT248_R {
        PROT248_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Protects Sector 249 from program or erase"]
    #[inline(always)]
    pub fn prot249(&self) -> PROT249_R {
        PROT249_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Protects Sector 250 from program or erase"]
    #[inline(always)]
    pub fn prot250(&self) -> PROT250_R {
        PROT250_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protects Sector 251 from program or erase"]
    #[inline(always)]
    pub fn prot251(&self) -> PROT251_R {
        PROT251_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protects Sector 252 from program or erase"]
    #[inline(always)]
    pub fn prot252(&self) -> PROT252_R {
        PROT252_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Protects Sector 253 from program or erase"]
    #[inline(always)]
    pub fn prot253(&self) -> PROT253_R {
        PROT253_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Protects Sector 254 from program or erase"]
    #[inline(always)]
    pub fn prot254(&self) -> PROT254_R {
        PROT254_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Protects Sector 255 from program or erase"]
    #[inline(always)]
    pub fn prot255(&self) -> PROT255_R {
        PROT255_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Protects Sector 224 from program or erase"]
    #[inline(always)]
    pub fn prot224(&mut self) -> PROT224_W<0> {
        PROT224_W::new(self)
    }
    #[doc = "Bit 1 - Protects Sector 225 from program or erase"]
    #[inline(always)]
    pub fn prot225(&mut self) -> PROT225_W<1> {
        PROT225_W::new(self)
    }
    #[doc = "Bit 2 - Protects Sector 226 from program or erase"]
    #[inline(always)]
    pub fn prot226(&mut self) -> PROT226_W<2> {
        PROT226_W::new(self)
    }
    #[doc = "Bit 3 - Protects Sector 227 from program or erase"]
    #[inline(always)]
    pub fn prot227(&mut self) -> PROT227_W<3> {
        PROT227_W::new(self)
    }
    #[doc = "Bit 4 - Protects Sector 228 from program or erase"]
    #[inline(always)]
    pub fn prot228(&mut self) -> PROT228_W<4> {
        PROT228_W::new(self)
    }
    #[doc = "Bit 5 - Protects Sector 229 from program or erase"]
    #[inline(always)]
    pub fn prot229(&mut self) -> PROT229_W<5> {
        PROT229_W::new(self)
    }
    #[doc = "Bit 6 - Protects Sector 230 from program or erase"]
    #[inline(always)]
    pub fn prot230(&mut self) -> PROT230_W<6> {
        PROT230_W::new(self)
    }
    #[doc = "Bit 7 - Protects Sector 231 from program or erase"]
    #[inline(always)]
    pub fn prot231(&mut self) -> PROT231_W<7> {
        PROT231_W::new(self)
    }
    #[doc = "Bit 8 - Protects Sector 232 from program or erase"]
    #[inline(always)]
    pub fn prot232(&mut self) -> PROT232_W<8> {
        PROT232_W::new(self)
    }
    #[doc = "Bit 9 - Protects Sector 233 from program or erase"]
    #[inline(always)]
    pub fn prot233(&mut self) -> PROT233_W<9> {
        PROT233_W::new(self)
    }
    #[doc = "Bit 10 - Protects Sector 234 from program or erase"]
    #[inline(always)]
    pub fn prot234(&mut self) -> PROT234_W<10> {
        PROT234_W::new(self)
    }
    #[doc = "Bit 11 - Protects Sector 235 from program or erase"]
    #[inline(always)]
    pub fn prot235(&mut self) -> PROT235_W<11> {
        PROT235_W::new(self)
    }
    #[doc = "Bit 12 - Protects Sector 236 from program or erase"]
    #[inline(always)]
    pub fn prot236(&mut self) -> PROT236_W<12> {
        PROT236_W::new(self)
    }
    #[doc = "Bit 13 - Protects Sector 237 from program or erase"]
    #[inline(always)]
    pub fn prot237(&mut self) -> PROT237_W<13> {
        PROT237_W::new(self)
    }
    #[doc = "Bit 14 - Protects Sector 238 from program or erase"]
    #[inline(always)]
    pub fn prot238(&mut self) -> PROT238_W<14> {
        PROT238_W::new(self)
    }
    #[doc = "Bit 15 - Protects Sector 239 from program or erase"]
    #[inline(always)]
    pub fn prot239(&mut self) -> PROT239_W<15> {
        PROT239_W::new(self)
    }
    #[doc = "Bit 16 - Protects Sector 240 from program or erase"]
    #[inline(always)]
    pub fn prot240(&mut self) -> PROT240_W<16> {
        PROT240_W::new(self)
    }
    #[doc = "Bit 17 - Protects Sector 241 from program or erase"]
    #[inline(always)]
    pub fn prot241(&mut self) -> PROT241_W<17> {
        PROT241_W::new(self)
    }
    #[doc = "Bit 18 - Protects Sector 242 from program or erase"]
    #[inline(always)]
    pub fn prot242(&mut self) -> PROT242_W<18> {
        PROT242_W::new(self)
    }
    #[doc = "Bit 19 - Protects Sector 243 from program or erase"]
    #[inline(always)]
    pub fn prot243(&mut self) -> PROT243_W<19> {
        PROT243_W::new(self)
    }
    #[doc = "Bit 20 - Protects Sector 244 from program or erase"]
    #[inline(always)]
    pub fn prot244(&mut self) -> PROT244_W<20> {
        PROT244_W::new(self)
    }
    #[doc = "Bit 21 - Protects Sector 245 from program or erase"]
    #[inline(always)]
    pub fn prot245(&mut self) -> PROT245_W<21> {
        PROT245_W::new(self)
    }
    #[doc = "Bit 22 - Protects Sector 246 from program or erase"]
    #[inline(always)]
    pub fn prot246(&mut self) -> PROT246_W<22> {
        PROT246_W::new(self)
    }
    #[doc = "Bit 23 - Protects Sector 247 from program or erase"]
    #[inline(always)]
    pub fn prot247(&mut self) -> PROT247_W<23> {
        PROT247_W::new(self)
    }
    #[doc = "Bit 24 - Protects Sector 248 from program or erase"]
    #[inline(always)]
    pub fn prot248(&mut self) -> PROT248_W<24> {
        PROT248_W::new(self)
    }
    #[doc = "Bit 25 - Protects Sector 249 from program or erase"]
    #[inline(always)]
    pub fn prot249(&mut self) -> PROT249_W<25> {
        PROT249_W::new(self)
    }
    #[doc = "Bit 26 - Protects Sector 250 from program or erase"]
    #[inline(always)]
    pub fn prot250(&mut self) -> PROT250_W<26> {
        PROT250_W::new(self)
    }
    #[doc = "Bit 27 - Protects Sector 251 from program or erase"]
    #[inline(always)]
    pub fn prot251(&mut self) -> PROT251_W<27> {
        PROT251_W::new(self)
    }
    #[doc = "Bit 28 - Protects Sector 252 from program or erase"]
    #[inline(always)]
    pub fn prot252(&mut self) -> PROT252_W<28> {
        PROT252_W::new(self)
    }
    #[doc = "Bit 29 - Protects Sector 253 from program or erase"]
    #[inline(always)]
    pub fn prot253(&mut self) -> PROT253_W<29> {
        PROT253_W::new(self)
    }
    #[doc = "Bit 30 - Protects Sector 254 from program or erase"]
    #[inline(always)]
    pub fn prot254(&mut self) -> PROT254_W<30> {
        PROT254_W::new(self)
    }
    #[doc = "Bit 31 - Protects Sector 255 from program or erase"]
    #[inline(always)]
    pub fn prot255(&mut self) -> PROT255_W<31> {
        PROT255_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Memory Bank0 Write/Erase Protection Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_bank0_main_weprot7](index.html) module"]
pub struct FLCTL_BANK0_MAIN_WEPROT7_SPEC;
impl crate::RegisterSpec for FLCTL_BANK0_MAIN_WEPROT7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_bank0_main_weprot7::R](R) reader structure"]
impl crate::Readable for FLCTL_BANK0_MAIN_WEPROT7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flctl_bank0_main_weprot7::W](W) writer structure"]
impl crate::Writable for FLCTL_BANK0_MAIN_WEPROT7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLCTL_BANK0_MAIN_WEPROT7 to value 0xffff_ffff"]
impl crate::Resettable for FLCTL_BANK0_MAIN_WEPROT7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
