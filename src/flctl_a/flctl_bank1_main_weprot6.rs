#[doc = "Register `FLCTL_BANK1_MAIN_WEPROT6` reader"]
pub struct R(crate::R<FLCTL_BANK1_MAIN_WEPROT6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_BANK1_MAIN_WEPROT6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_BANK1_MAIN_WEPROT6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_BANK1_MAIN_WEPROT6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLCTL_BANK1_MAIN_WEPROT6` writer"]
pub struct W(crate::W<FLCTL_BANK1_MAIN_WEPROT6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLCTL_BANK1_MAIN_WEPROT6_SPEC>;
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
impl From<crate::W<FLCTL_BANK1_MAIN_WEPROT6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLCTL_BANK1_MAIN_WEPROT6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROT192` reader - Protects Sector 192 from program or erase"]
pub type PROT192_R = crate::BitReader<bool>;
#[doc = "Field `PROT192` writer - Protects Sector 192 from program or erase"]
pub type PROT192_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT193` reader - Protects Sector 193 from program or erase"]
pub type PROT193_R = crate::BitReader<bool>;
#[doc = "Field `PROT193` writer - Protects Sector 193 from program or erase"]
pub type PROT193_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT194` reader - Protects Sector 194 from program or erase"]
pub type PROT194_R = crate::BitReader<bool>;
#[doc = "Field `PROT194` writer - Protects Sector 194 from program or erase"]
pub type PROT194_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT195` reader - Protects Sector 195 from program or erase"]
pub type PROT195_R = crate::BitReader<bool>;
#[doc = "Field `PROT195` writer - Protects Sector 195 from program or erase"]
pub type PROT195_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT196` reader - Protects Sector 196 from program or erase"]
pub type PROT196_R = crate::BitReader<bool>;
#[doc = "Field `PROT196` writer - Protects Sector 196 from program or erase"]
pub type PROT196_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT197` reader - Protects Sector 197 from program or erase"]
pub type PROT197_R = crate::BitReader<bool>;
#[doc = "Field `PROT197` writer - Protects Sector 197 from program or erase"]
pub type PROT197_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT198` reader - Protects Sector 198 from program or erase"]
pub type PROT198_R = crate::BitReader<bool>;
#[doc = "Field `PROT198` writer - Protects Sector 198 from program or erase"]
pub type PROT198_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT199` reader - Protects Sector 199 from program or erase"]
pub type PROT199_R = crate::BitReader<bool>;
#[doc = "Field `PROT199` writer - Protects Sector 199 from program or erase"]
pub type PROT199_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT200` reader - Protects Sector 200 from program or erase"]
pub type PROT200_R = crate::BitReader<bool>;
#[doc = "Field `PROT200` writer - Protects Sector 200 from program or erase"]
pub type PROT200_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT201` reader - Protects Sector 201 from program or erase"]
pub type PROT201_R = crate::BitReader<bool>;
#[doc = "Field `PROT201` writer - Protects Sector 201 from program or erase"]
pub type PROT201_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT202` reader - Protects Sector 202 from program or erase"]
pub type PROT202_R = crate::BitReader<bool>;
#[doc = "Field `PROT202` writer - Protects Sector 202 from program or erase"]
pub type PROT202_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT203` reader - Protects Sector 203 from program or erase"]
pub type PROT203_R = crate::BitReader<bool>;
#[doc = "Field `PROT203` writer - Protects Sector 203 from program or erase"]
pub type PROT203_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT204` reader - Protects Sector 204 from program or erase"]
pub type PROT204_R = crate::BitReader<bool>;
#[doc = "Field `PROT204` writer - Protects Sector 204 from program or erase"]
pub type PROT204_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT205` reader - Protects Sector 205 from program or erase"]
pub type PROT205_R = crate::BitReader<bool>;
#[doc = "Field `PROT205` writer - Protects Sector 205 from program or erase"]
pub type PROT205_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT206` reader - Protects Sector 206 from program or erase"]
pub type PROT206_R = crate::BitReader<bool>;
#[doc = "Field `PROT206` writer - Protects Sector 206 from program or erase"]
pub type PROT206_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT207` reader - Protects Sector 207 from program or erase"]
pub type PROT207_R = crate::BitReader<bool>;
#[doc = "Field `PROT207` writer - Protects Sector 207 from program or erase"]
pub type PROT207_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT208` reader - Protects Sector 208 from program or erase"]
pub type PROT208_R = crate::BitReader<bool>;
#[doc = "Field `PROT208` writer - Protects Sector 208 from program or erase"]
pub type PROT208_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT209` reader - Protects Sector 209 from program or erase"]
pub type PROT209_R = crate::BitReader<bool>;
#[doc = "Field `PROT209` writer - Protects Sector 209 from program or erase"]
pub type PROT209_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT210` reader - Protects Sector 210 from program or erase"]
pub type PROT210_R = crate::BitReader<bool>;
#[doc = "Field `PROT210` writer - Protects Sector 210 from program or erase"]
pub type PROT210_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT211` reader - Protects Sector 211 from program or erase"]
pub type PROT211_R = crate::BitReader<bool>;
#[doc = "Field `PROT211` writer - Protects Sector 211 from program or erase"]
pub type PROT211_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT212` reader - Protects Sector 212 from program or erase"]
pub type PROT212_R = crate::BitReader<bool>;
#[doc = "Field `PROT212` writer - Protects Sector 212 from program or erase"]
pub type PROT212_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT213` reader - Protects Sector 213 from program or erase"]
pub type PROT213_R = crate::BitReader<bool>;
#[doc = "Field `PROT213` writer - Protects Sector 213 from program or erase"]
pub type PROT213_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT214` reader - Protects Sector 214 from program or erase"]
pub type PROT214_R = crate::BitReader<bool>;
#[doc = "Field `PROT214` writer - Protects Sector 214 from program or erase"]
pub type PROT214_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT215` reader - Protects Sector 215 from program or erase"]
pub type PROT215_R = crate::BitReader<bool>;
#[doc = "Field `PROT215` writer - Protects Sector 215 from program or erase"]
pub type PROT215_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT216` reader - Protects Sector 216 from program or erase"]
pub type PROT216_R = crate::BitReader<bool>;
#[doc = "Field `PROT216` writer - Protects Sector 216 from program or erase"]
pub type PROT216_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT217` reader - Protects Sector 217 from program or erase"]
pub type PROT217_R = crate::BitReader<bool>;
#[doc = "Field `PROT217` writer - Protects Sector 217 from program or erase"]
pub type PROT217_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT218` reader - Protects Sector 218 from program or erase"]
pub type PROT218_R = crate::BitReader<bool>;
#[doc = "Field `PROT218` writer - Protects Sector 218 from program or erase"]
pub type PROT218_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT219` reader - Protects Sector 219 from program or erase"]
pub type PROT219_R = crate::BitReader<bool>;
#[doc = "Field `PROT219` writer - Protects Sector 219 from program or erase"]
pub type PROT219_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT220` reader - Protects Sector 220 from program or erase"]
pub type PROT220_R = crate::BitReader<bool>;
#[doc = "Field `PROT220` writer - Protects Sector 220 from program or erase"]
pub type PROT220_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT221` reader - Protects Sector 221 from program or erase"]
pub type PROT221_R = crate::BitReader<bool>;
#[doc = "Field `PROT221` writer - Protects Sector 221 from program or erase"]
pub type PROT221_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT222` reader - Protects Sector 222 from program or erase"]
pub type PROT222_R = crate::BitReader<bool>;
#[doc = "Field `PROT222` writer - Protects Sector 222 from program or erase"]
pub type PROT222_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
#[doc = "Field `PROT223` reader - Protects Sector 223 from program or erase"]
pub type PROT223_R = crate::BitReader<bool>;
#[doc = "Field `PROT223` writer - Protects Sector 223 from program or erase"]
pub type PROT223_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK1_MAIN_WEPROT6_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Protects Sector 192 from program or erase"]
    #[inline(always)]
    pub fn prot192(&self) -> PROT192_R {
        PROT192_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Protects Sector 193 from program or erase"]
    #[inline(always)]
    pub fn prot193(&self) -> PROT193_R {
        PROT193_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Protects Sector 194 from program or erase"]
    #[inline(always)]
    pub fn prot194(&self) -> PROT194_R {
        PROT194_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Protects Sector 195 from program or erase"]
    #[inline(always)]
    pub fn prot195(&self) -> PROT195_R {
        PROT195_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Protects Sector 196 from program or erase"]
    #[inline(always)]
    pub fn prot196(&self) -> PROT196_R {
        PROT196_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Protects Sector 197 from program or erase"]
    #[inline(always)]
    pub fn prot197(&self) -> PROT197_R {
        PROT197_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Protects Sector 198 from program or erase"]
    #[inline(always)]
    pub fn prot198(&self) -> PROT198_R {
        PROT198_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Protects Sector 199 from program or erase"]
    #[inline(always)]
    pub fn prot199(&self) -> PROT199_R {
        PROT199_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Protects Sector 200 from program or erase"]
    #[inline(always)]
    pub fn prot200(&self) -> PROT200_R {
        PROT200_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Protects Sector 201 from program or erase"]
    #[inline(always)]
    pub fn prot201(&self) -> PROT201_R {
        PROT201_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Protects Sector 202 from program or erase"]
    #[inline(always)]
    pub fn prot202(&self) -> PROT202_R {
        PROT202_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Protects Sector 203 from program or erase"]
    #[inline(always)]
    pub fn prot203(&self) -> PROT203_R {
        PROT203_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Protects Sector 204 from program or erase"]
    #[inline(always)]
    pub fn prot204(&self) -> PROT204_R {
        PROT204_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Protects Sector 205 from program or erase"]
    #[inline(always)]
    pub fn prot205(&self) -> PROT205_R {
        PROT205_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Protects Sector 206 from program or erase"]
    #[inline(always)]
    pub fn prot206(&self) -> PROT206_R {
        PROT206_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Protects Sector 207 from program or erase"]
    #[inline(always)]
    pub fn prot207(&self) -> PROT207_R {
        PROT207_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Protects Sector 208 from program or erase"]
    #[inline(always)]
    pub fn prot208(&self) -> PROT208_R {
        PROT208_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Protects Sector 209 from program or erase"]
    #[inline(always)]
    pub fn prot209(&self) -> PROT209_R {
        PROT209_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Protects Sector 210 from program or erase"]
    #[inline(always)]
    pub fn prot210(&self) -> PROT210_R {
        PROT210_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Protects Sector 211 from program or erase"]
    #[inline(always)]
    pub fn prot211(&self) -> PROT211_R {
        PROT211_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Protects Sector 212 from program or erase"]
    #[inline(always)]
    pub fn prot212(&self) -> PROT212_R {
        PROT212_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Protects Sector 213 from program or erase"]
    #[inline(always)]
    pub fn prot213(&self) -> PROT213_R {
        PROT213_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Protects Sector 214 from program or erase"]
    #[inline(always)]
    pub fn prot214(&self) -> PROT214_R {
        PROT214_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Protects Sector 215 from program or erase"]
    #[inline(always)]
    pub fn prot215(&self) -> PROT215_R {
        PROT215_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Protects Sector 216 from program or erase"]
    #[inline(always)]
    pub fn prot216(&self) -> PROT216_R {
        PROT216_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Protects Sector 217 from program or erase"]
    #[inline(always)]
    pub fn prot217(&self) -> PROT217_R {
        PROT217_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Protects Sector 218 from program or erase"]
    #[inline(always)]
    pub fn prot218(&self) -> PROT218_R {
        PROT218_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protects Sector 219 from program or erase"]
    #[inline(always)]
    pub fn prot219(&self) -> PROT219_R {
        PROT219_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protects Sector 220 from program or erase"]
    #[inline(always)]
    pub fn prot220(&self) -> PROT220_R {
        PROT220_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Protects Sector 221 from program or erase"]
    #[inline(always)]
    pub fn prot221(&self) -> PROT221_R {
        PROT221_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Protects Sector 222 from program or erase"]
    #[inline(always)]
    pub fn prot222(&self) -> PROT222_R {
        PROT222_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Protects Sector 223 from program or erase"]
    #[inline(always)]
    pub fn prot223(&self) -> PROT223_R {
        PROT223_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Protects Sector 192 from program or erase"]
    #[inline(always)]
    pub fn prot192(&mut self) -> PROT192_W<0> {
        PROT192_W::new(self)
    }
    #[doc = "Bit 1 - Protects Sector 193 from program or erase"]
    #[inline(always)]
    pub fn prot193(&mut self) -> PROT193_W<1> {
        PROT193_W::new(self)
    }
    #[doc = "Bit 2 - Protects Sector 194 from program or erase"]
    #[inline(always)]
    pub fn prot194(&mut self) -> PROT194_W<2> {
        PROT194_W::new(self)
    }
    #[doc = "Bit 3 - Protects Sector 195 from program or erase"]
    #[inline(always)]
    pub fn prot195(&mut self) -> PROT195_W<3> {
        PROT195_W::new(self)
    }
    #[doc = "Bit 4 - Protects Sector 196 from program or erase"]
    #[inline(always)]
    pub fn prot196(&mut self) -> PROT196_W<4> {
        PROT196_W::new(self)
    }
    #[doc = "Bit 5 - Protects Sector 197 from program or erase"]
    #[inline(always)]
    pub fn prot197(&mut self) -> PROT197_W<5> {
        PROT197_W::new(self)
    }
    #[doc = "Bit 6 - Protects Sector 198 from program or erase"]
    #[inline(always)]
    pub fn prot198(&mut self) -> PROT198_W<6> {
        PROT198_W::new(self)
    }
    #[doc = "Bit 7 - Protects Sector 199 from program or erase"]
    #[inline(always)]
    pub fn prot199(&mut self) -> PROT199_W<7> {
        PROT199_W::new(self)
    }
    #[doc = "Bit 8 - Protects Sector 200 from program or erase"]
    #[inline(always)]
    pub fn prot200(&mut self) -> PROT200_W<8> {
        PROT200_W::new(self)
    }
    #[doc = "Bit 9 - Protects Sector 201 from program or erase"]
    #[inline(always)]
    pub fn prot201(&mut self) -> PROT201_W<9> {
        PROT201_W::new(self)
    }
    #[doc = "Bit 10 - Protects Sector 202 from program or erase"]
    #[inline(always)]
    pub fn prot202(&mut self) -> PROT202_W<10> {
        PROT202_W::new(self)
    }
    #[doc = "Bit 11 - Protects Sector 203 from program or erase"]
    #[inline(always)]
    pub fn prot203(&mut self) -> PROT203_W<11> {
        PROT203_W::new(self)
    }
    #[doc = "Bit 12 - Protects Sector 204 from program or erase"]
    #[inline(always)]
    pub fn prot204(&mut self) -> PROT204_W<12> {
        PROT204_W::new(self)
    }
    #[doc = "Bit 13 - Protects Sector 205 from program or erase"]
    #[inline(always)]
    pub fn prot205(&mut self) -> PROT205_W<13> {
        PROT205_W::new(self)
    }
    #[doc = "Bit 14 - Protects Sector 206 from program or erase"]
    #[inline(always)]
    pub fn prot206(&mut self) -> PROT206_W<14> {
        PROT206_W::new(self)
    }
    #[doc = "Bit 15 - Protects Sector 207 from program or erase"]
    #[inline(always)]
    pub fn prot207(&mut self) -> PROT207_W<15> {
        PROT207_W::new(self)
    }
    #[doc = "Bit 16 - Protects Sector 208 from program or erase"]
    #[inline(always)]
    pub fn prot208(&mut self) -> PROT208_W<16> {
        PROT208_W::new(self)
    }
    #[doc = "Bit 17 - Protects Sector 209 from program or erase"]
    #[inline(always)]
    pub fn prot209(&mut self) -> PROT209_W<17> {
        PROT209_W::new(self)
    }
    #[doc = "Bit 18 - Protects Sector 210 from program or erase"]
    #[inline(always)]
    pub fn prot210(&mut self) -> PROT210_W<18> {
        PROT210_W::new(self)
    }
    #[doc = "Bit 19 - Protects Sector 211 from program or erase"]
    #[inline(always)]
    pub fn prot211(&mut self) -> PROT211_W<19> {
        PROT211_W::new(self)
    }
    #[doc = "Bit 20 - Protects Sector 212 from program or erase"]
    #[inline(always)]
    pub fn prot212(&mut self) -> PROT212_W<20> {
        PROT212_W::new(self)
    }
    #[doc = "Bit 21 - Protects Sector 213 from program or erase"]
    #[inline(always)]
    pub fn prot213(&mut self) -> PROT213_W<21> {
        PROT213_W::new(self)
    }
    #[doc = "Bit 22 - Protects Sector 214 from program or erase"]
    #[inline(always)]
    pub fn prot214(&mut self) -> PROT214_W<22> {
        PROT214_W::new(self)
    }
    #[doc = "Bit 23 - Protects Sector 215 from program or erase"]
    #[inline(always)]
    pub fn prot215(&mut self) -> PROT215_W<23> {
        PROT215_W::new(self)
    }
    #[doc = "Bit 24 - Protects Sector 216 from program or erase"]
    #[inline(always)]
    pub fn prot216(&mut self) -> PROT216_W<24> {
        PROT216_W::new(self)
    }
    #[doc = "Bit 25 - Protects Sector 217 from program or erase"]
    #[inline(always)]
    pub fn prot217(&mut self) -> PROT217_W<25> {
        PROT217_W::new(self)
    }
    #[doc = "Bit 26 - Protects Sector 218 from program or erase"]
    #[inline(always)]
    pub fn prot218(&mut self) -> PROT218_W<26> {
        PROT218_W::new(self)
    }
    #[doc = "Bit 27 - Protects Sector 219 from program or erase"]
    #[inline(always)]
    pub fn prot219(&mut self) -> PROT219_W<27> {
        PROT219_W::new(self)
    }
    #[doc = "Bit 28 - Protects Sector 220 from program or erase"]
    #[inline(always)]
    pub fn prot220(&mut self) -> PROT220_W<28> {
        PROT220_W::new(self)
    }
    #[doc = "Bit 29 - Protects Sector 221 from program or erase"]
    #[inline(always)]
    pub fn prot221(&mut self) -> PROT221_W<29> {
        PROT221_W::new(self)
    }
    #[doc = "Bit 30 - Protects Sector 222 from program or erase"]
    #[inline(always)]
    pub fn prot222(&mut self) -> PROT222_W<30> {
        PROT222_W::new(self)
    }
    #[doc = "Bit 31 - Protects Sector 223 from program or erase"]
    #[inline(always)]
    pub fn prot223(&mut self) -> PROT223_W<31> {
        PROT223_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Memory Bank1 Write/Erase Protection Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_bank1_main_weprot6](index.html) module"]
pub struct FLCTL_BANK1_MAIN_WEPROT6_SPEC;
impl crate::RegisterSpec for FLCTL_BANK1_MAIN_WEPROT6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_bank1_main_weprot6::R](R) reader structure"]
impl crate::Readable for FLCTL_BANK1_MAIN_WEPROT6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flctl_bank1_main_weprot6::W](W) writer structure"]
impl crate::Writable for FLCTL_BANK1_MAIN_WEPROT6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLCTL_BANK1_MAIN_WEPROT6 to value 0xffff_ffff"]
impl crate::Resettable for FLCTL_BANK1_MAIN_WEPROT6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
