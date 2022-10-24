#[doc = "Register `FLCTL_BANK0_MAIN_WEPROT` reader"]
pub struct R(crate::R<FLCTL_BANK0_MAIN_WEPROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_BANK0_MAIN_WEPROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_BANK0_MAIN_WEPROT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_BANK0_MAIN_WEPROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLCTL_BANK0_MAIN_WEPROT` writer"]
pub struct W(crate::W<FLCTL_BANK0_MAIN_WEPROT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLCTL_BANK0_MAIN_WEPROT_SPEC>;
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
impl From<crate::W<FLCTL_BANK0_MAIN_WEPROT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLCTL_BANK0_MAIN_WEPROT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROT0` reader - Protects Sector 0 from program or erase"]
pub type PROT0_R = crate::BitReader<bool>;
#[doc = "Field `PROT0` writer - Protects Sector 0 from program or erase"]
pub type PROT0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT1` reader - Protects Sector 1 from program or erase"]
pub type PROT1_R = crate::BitReader<bool>;
#[doc = "Field `PROT1` writer - Protects Sector 1 from program or erase"]
pub type PROT1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT2` reader - Protects Sector 2 from program or erase"]
pub type PROT2_R = crate::BitReader<bool>;
#[doc = "Field `PROT2` writer - Protects Sector 2 from program or erase"]
pub type PROT2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT3` reader - Protects Sector 3 from program or erase"]
pub type PROT3_R = crate::BitReader<bool>;
#[doc = "Field `PROT3` writer - Protects Sector 3 from program or erase"]
pub type PROT3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT4` reader - Protects Sector 4 from program or erase"]
pub type PROT4_R = crate::BitReader<bool>;
#[doc = "Field `PROT4` writer - Protects Sector 4 from program or erase"]
pub type PROT4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT5` reader - Protects Sector 5 from program or erase"]
pub type PROT5_R = crate::BitReader<bool>;
#[doc = "Field `PROT5` writer - Protects Sector 5 from program or erase"]
pub type PROT5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT6` reader - Protects Sector 6 from program or erase"]
pub type PROT6_R = crate::BitReader<bool>;
#[doc = "Field `PROT6` writer - Protects Sector 6 from program or erase"]
pub type PROT6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT7` reader - Protects Sector 7 from program or erase"]
pub type PROT7_R = crate::BitReader<bool>;
#[doc = "Field `PROT7` writer - Protects Sector 7 from program or erase"]
pub type PROT7_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT8` reader - Protects Sector 8 from program or erase"]
pub type PROT8_R = crate::BitReader<bool>;
#[doc = "Field `PROT8` writer - Protects Sector 8 from program or erase"]
pub type PROT8_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT9` reader - Protects Sector 9 from program or erase"]
pub type PROT9_R = crate::BitReader<bool>;
#[doc = "Field `PROT9` writer - Protects Sector 9 from program or erase"]
pub type PROT9_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT10` reader - Protects Sector 10 from program or erase"]
pub type PROT10_R = crate::BitReader<bool>;
#[doc = "Field `PROT10` writer - Protects Sector 10 from program or erase"]
pub type PROT10_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT11` reader - Protects Sector 11 from program or erase"]
pub type PROT11_R = crate::BitReader<bool>;
#[doc = "Field `PROT11` writer - Protects Sector 11 from program or erase"]
pub type PROT11_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT12` reader - Protects Sector 12 from program or erase"]
pub type PROT12_R = crate::BitReader<bool>;
#[doc = "Field `PROT12` writer - Protects Sector 12 from program or erase"]
pub type PROT12_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT13` reader - Protects Sector 13 from program or erase"]
pub type PROT13_R = crate::BitReader<bool>;
#[doc = "Field `PROT13` writer - Protects Sector 13 from program or erase"]
pub type PROT13_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT14` reader - Protects Sector 14 from program or erase"]
pub type PROT14_R = crate::BitReader<bool>;
#[doc = "Field `PROT14` writer - Protects Sector 14 from program or erase"]
pub type PROT14_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT15` reader - Protects Sector 15 from program or erase"]
pub type PROT15_R = crate::BitReader<bool>;
#[doc = "Field `PROT15` writer - Protects Sector 15 from program or erase"]
pub type PROT15_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT16` reader - Protects Sector 16 from program or erase"]
pub type PROT16_R = crate::BitReader<bool>;
#[doc = "Field `PROT16` writer - Protects Sector 16 from program or erase"]
pub type PROT16_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT17` reader - Protects Sector 17 from program or erase"]
pub type PROT17_R = crate::BitReader<bool>;
#[doc = "Field `PROT17` writer - Protects Sector 17 from program or erase"]
pub type PROT17_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT18` reader - Protects Sector 18 from program or erase"]
pub type PROT18_R = crate::BitReader<bool>;
#[doc = "Field `PROT18` writer - Protects Sector 18 from program or erase"]
pub type PROT18_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT19` reader - Protects Sector 19 from program or erase"]
pub type PROT19_R = crate::BitReader<bool>;
#[doc = "Field `PROT19` writer - Protects Sector 19 from program or erase"]
pub type PROT19_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT20` reader - Protects Sector 20 from program or erase"]
pub type PROT20_R = crate::BitReader<bool>;
#[doc = "Field `PROT20` writer - Protects Sector 20 from program or erase"]
pub type PROT20_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT21` reader - Protects Sector 21 from program or erase"]
pub type PROT21_R = crate::BitReader<bool>;
#[doc = "Field `PROT21` writer - Protects Sector 21 from program or erase"]
pub type PROT21_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT22` reader - Protects Sector 22 from program or erase"]
pub type PROT22_R = crate::BitReader<bool>;
#[doc = "Field `PROT22` writer - Protects Sector 22 from program or erase"]
pub type PROT22_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT23` reader - Protects Sector 23 from program or erase"]
pub type PROT23_R = crate::BitReader<bool>;
#[doc = "Field `PROT23` writer - Protects Sector 23 from program or erase"]
pub type PROT23_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT24` reader - Protects Sector 24 from program or erase"]
pub type PROT24_R = crate::BitReader<bool>;
#[doc = "Field `PROT24` writer - Protects Sector 24 from program or erase"]
pub type PROT24_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT25` reader - Protects Sector 25 from program or erase"]
pub type PROT25_R = crate::BitReader<bool>;
#[doc = "Field `PROT25` writer - Protects Sector 25 from program or erase"]
pub type PROT25_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT26` reader - Protects Sector 26 from program or erase"]
pub type PROT26_R = crate::BitReader<bool>;
#[doc = "Field `PROT26` writer - Protects Sector 26 from program or erase"]
pub type PROT26_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT27` reader - Protects Sector 27 from program or erase"]
pub type PROT27_R = crate::BitReader<bool>;
#[doc = "Field `PROT27` writer - Protects Sector 27 from program or erase"]
pub type PROT27_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT28` reader - Protects Sector 28 from program or erase"]
pub type PROT28_R = crate::BitReader<bool>;
#[doc = "Field `PROT28` writer - Protects Sector 28 from program or erase"]
pub type PROT28_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT29` reader - Protects Sector 29 from program or erase"]
pub type PROT29_R = crate::BitReader<bool>;
#[doc = "Field `PROT29` writer - Protects Sector 29 from program or erase"]
pub type PROT29_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT30` reader - Protects Sector 30 from program or erase"]
pub type PROT30_R = crate::BitReader<bool>;
#[doc = "Field `PROT30` writer - Protects Sector 30 from program or erase"]
pub type PROT30_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
#[doc = "Field `PROT31` reader - Protects Sector 31 from program or erase"]
pub type PROT31_R = crate::BitReader<bool>;
#[doc = "Field `PROT31` writer - Protects Sector 31 from program or erase"]
pub type PROT31_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Protects Sector 0 from program or erase"]
    #[inline(always)]
    pub fn prot0(&self) -> PROT0_R {
        PROT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Protects Sector 1 from program or erase"]
    #[inline(always)]
    pub fn prot1(&self) -> PROT1_R {
        PROT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Protects Sector 2 from program or erase"]
    #[inline(always)]
    pub fn prot2(&self) -> PROT2_R {
        PROT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Protects Sector 3 from program or erase"]
    #[inline(always)]
    pub fn prot3(&self) -> PROT3_R {
        PROT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Protects Sector 4 from program or erase"]
    #[inline(always)]
    pub fn prot4(&self) -> PROT4_R {
        PROT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Protects Sector 5 from program or erase"]
    #[inline(always)]
    pub fn prot5(&self) -> PROT5_R {
        PROT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Protects Sector 6 from program or erase"]
    #[inline(always)]
    pub fn prot6(&self) -> PROT6_R {
        PROT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Protects Sector 7 from program or erase"]
    #[inline(always)]
    pub fn prot7(&self) -> PROT7_R {
        PROT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Protects Sector 8 from program or erase"]
    #[inline(always)]
    pub fn prot8(&self) -> PROT8_R {
        PROT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Protects Sector 9 from program or erase"]
    #[inline(always)]
    pub fn prot9(&self) -> PROT9_R {
        PROT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Protects Sector 10 from program or erase"]
    #[inline(always)]
    pub fn prot10(&self) -> PROT10_R {
        PROT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Protects Sector 11 from program or erase"]
    #[inline(always)]
    pub fn prot11(&self) -> PROT11_R {
        PROT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Protects Sector 12 from program or erase"]
    #[inline(always)]
    pub fn prot12(&self) -> PROT12_R {
        PROT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Protects Sector 13 from program or erase"]
    #[inline(always)]
    pub fn prot13(&self) -> PROT13_R {
        PROT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Protects Sector 14 from program or erase"]
    #[inline(always)]
    pub fn prot14(&self) -> PROT14_R {
        PROT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Protects Sector 15 from program or erase"]
    #[inline(always)]
    pub fn prot15(&self) -> PROT15_R {
        PROT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Protects Sector 16 from program or erase"]
    #[inline(always)]
    pub fn prot16(&self) -> PROT16_R {
        PROT16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Protects Sector 17 from program or erase"]
    #[inline(always)]
    pub fn prot17(&self) -> PROT17_R {
        PROT17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Protects Sector 18 from program or erase"]
    #[inline(always)]
    pub fn prot18(&self) -> PROT18_R {
        PROT18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Protects Sector 19 from program or erase"]
    #[inline(always)]
    pub fn prot19(&self) -> PROT19_R {
        PROT19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Protects Sector 20 from program or erase"]
    #[inline(always)]
    pub fn prot20(&self) -> PROT20_R {
        PROT20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Protects Sector 21 from program or erase"]
    #[inline(always)]
    pub fn prot21(&self) -> PROT21_R {
        PROT21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Protects Sector 22 from program or erase"]
    #[inline(always)]
    pub fn prot22(&self) -> PROT22_R {
        PROT22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Protects Sector 23 from program or erase"]
    #[inline(always)]
    pub fn prot23(&self) -> PROT23_R {
        PROT23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Protects Sector 24 from program or erase"]
    #[inline(always)]
    pub fn prot24(&self) -> PROT24_R {
        PROT24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Protects Sector 25 from program or erase"]
    #[inline(always)]
    pub fn prot25(&self) -> PROT25_R {
        PROT25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Protects Sector 26 from program or erase"]
    #[inline(always)]
    pub fn prot26(&self) -> PROT26_R {
        PROT26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protects Sector 27 from program or erase"]
    #[inline(always)]
    pub fn prot27(&self) -> PROT27_R {
        PROT27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protects Sector 28 from program or erase"]
    #[inline(always)]
    pub fn prot28(&self) -> PROT28_R {
        PROT28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Protects Sector 29 from program or erase"]
    #[inline(always)]
    pub fn prot29(&self) -> PROT29_R {
        PROT29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Protects Sector 30 from program or erase"]
    #[inline(always)]
    pub fn prot30(&self) -> PROT30_R {
        PROT30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Protects Sector 31 from program or erase"]
    #[inline(always)]
    pub fn prot31(&self) -> PROT31_R {
        PROT31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Protects Sector 0 from program or erase"]
    #[inline(always)]
    pub fn prot0(&mut self) -> PROT0_W<0> {
        PROT0_W::new(self)
    }
    #[doc = "Bit 1 - Protects Sector 1 from program or erase"]
    #[inline(always)]
    pub fn prot1(&mut self) -> PROT1_W<1> {
        PROT1_W::new(self)
    }
    #[doc = "Bit 2 - Protects Sector 2 from program or erase"]
    #[inline(always)]
    pub fn prot2(&mut self) -> PROT2_W<2> {
        PROT2_W::new(self)
    }
    #[doc = "Bit 3 - Protects Sector 3 from program or erase"]
    #[inline(always)]
    pub fn prot3(&mut self) -> PROT3_W<3> {
        PROT3_W::new(self)
    }
    #[doc = "Bit 4 - Protects Sector 4 from program or erase"]
    #[inline(always)]
    pub fn prot4(&mut self) -> PROT4_W<4> {
        PROT4_W::new(self)
    }
    #[doc = "Bit 5 - Protects Sector 5 from program or erase"]
    #[inline(always)]
    pub fn prot5(&mut self) -> PROT5_W<5> {
        PROT5_W::new(self)
    }
    #[doc = "Bit 6 - Protects Sector 6 from program or erase"]
    #[inline(always)]
    pub fn prot6(&mut self) -> PROT6_W<6> {
        PROT6_W::new(self)
    }
    #[doc = "Bit 7 - Protects Sector 7 from program or erase"]
    #[inline(always)]
    pub fn prot7(&mut self) -> PROT7_W<7> {
        PROT7_W::new(self)
    }
    #[doc = "Bit 8 - Protects Sector 8 from program or erase"]
    #[inline(always)]
    pub fn prot8(&mut self) -> PROT8_W<8> {
        PROT8_W::new(self)
    }
    #[doc = "Bit 9 - Protects Sector 9 from program or erase"]
    #[inline(always)]
    pub fn prot9(&mut self) -> PROT9_W<9> {
        PROT9_W::new(self)
    }
    #[doc = "Bit 10 - Protects Sector 10 from program or erase"]
    #[inline(always)]
    pub fn prot10(&mut self) -> PROT10_W<10> {
        PROT10_W::new(self)
    }
    #[doc = "Bit 11 - Protects Sector 11 from program or erase"]
    #[inline(always)]
    pub fn prot11(&mut self) -> PROT11_W<11> {
        PROT11_W::new(self)
    }
    #[doc = "Bit 12 - Protects Sector 12 from program or erase"]
    #[inline(always)]
    pub fn prot12(&mut self) -> PROT12_W<12> {
        PROT12_W::new(self)
    }
    #[doc = "Bit 13 - Protects Sector 13 from program or erase"]
    #[inline(always)]
    pub fn prot13(&mut self) -> PROT13_W<13> {
        PROT13_W::new(self)
    }
    #[doc = "Bit 14 - Protects Sector 14 from program or erase"]
    #[inline(always)]
    pub fn prot14(&mut self) -> PROT14_W<14> {
        PROT14_W::new(self)
    }
    #[doc = "Bit 15 - Protects Sector 15 from program or erase"]
    #[inline(always)]
    pub fn prot15(&mut self) -> PROT15_W<15> {
        PROT15_W::new(self)
    }
    #[doc = "Bit 16 - Protects Sector 16 from program or erase"]
    #[inline(always)]
    pub fn prot16(&mut self) -> PROT16_W<16> {
        PROT16_W::new(self)
    }
    #[doc = "Bit 17 - Protects Sector 17 from program or erase"]
    #[inline(always)]
    pub fn prot17(&mut self) -> PROT17_W<17> {
        PROT17_W::new(self)
    }
    #[doc = "Bit 18 - Protects Sector 18 from program or erase"]
    #[inline(always)]
    pub fn prot18(&mut self) -> PROT18_W<18> {
        PROT18_W::new(self)
    }
    #[doc = "Bit 19 - Protects Sector 19 from program or erase"]
    #[inline(always)]
    pub fn prot19(&mut self) -> PROT19_W<19> {
        PROT19_W::new(self)
    }
    #[doc = "Bit 20 - Protects Sector 20 from program or erase"]
    #[inline(always)]
    pub fn prot20(&mut self) -> PROT20_W<20> {
        PROT20_W::new(self)
    }
    #[doc = "Bit 21 - Protects Sector 21 from program or erase"]
    #[inline(always)]
    pub fn prot21(&mut self) -> PROT21_W<21> {
        PROT21_W::new(self)
    }
    #[doc = "Bit 22 - Protects Sector 22 from program or erase"]
    #[inline(always)]
    pub fn prot22(&mut self) -> PROT22_W<22> {
        PROT22_W::new(self)
    }
    #[doc = "Bit 23 - Protects Sector 23 from program or erase"]
    #[inline(always)]
    pub fn prot23(&mut self) -> PROT23_W<23> {
        PROT23_W::new(self)
    }
    #[doc = "Bit 24 - Protects Sector 24 from program or erase"]
    #[inline(always)]
    pub fn prot24(&mut self) -> PROT24_W<24> {
        PROT24_W::new(self)
    }
    #[doc = "Bit 25 - Protects Sector 25 from program or erase"]
    #[inline(always)]
    pub fn prot25(&mut self) -> PROT25_W<25> {
        PROT25_W::new(self)
    }
    #[doc = "Bit 26 - Protects Sector 26 from program or erase"]
    #[inline(always)]
    pub fn prot26(&mut self) -> PROT26_W<26> {
        PROT26_W::new(self)
    }
    #[doc = "Bit 27 - Protects Sector 27 from program or erase"]
    #[inline(always)]
    pub fn prot27(&mut self) -> PROT27_W<27> {
        PROT27_W::new(self)
    }
    #[doc = "Bit 28 - Protects Sector 28 from program or erase"]
    #[inline(always)]
    pub fn prot28(&mut self) -> PROT28_W<28> {
        PROT28_W::new(self)
    }
    #[doc = "Bit 29 - Protects Sector 29 from program or erase"]
    #[inline(always)]
    pub fn prot29(&mut self) -> PROT29_W<29> {
        PROT29_W::new(self)
    }
    #[doc = "Bit 30 - Protects Sector 30 from program or erase"]
    #[inline(always)]
    pub fn prot30(&mut self) -> PROT30_W<30> {
        PROT30_W::new(self)
    }
    #[doc = "Bit 31 - Protects Sector 31 from program or erase"]
    #[inline(always)]
    pub fn prot31(&mut self) -> PROT31_W<31> {
        PROT31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Memory Bank0 Write/Erase Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_bank0_main_weprot](index.html) module"]
pub struct FLCTL_BANK0_MAIN_WEPROT_SPEC;
impl crate::RegisterSpec for FLCTL_BANK0_MAIN_WEPROT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_bank0_main_weprot::R](R) reader structure"]
impl crate::Readable for FLCTL_BANK0_MAIN_WEPROT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flctl_bank0_main_weprot::W](W) writer structure"]
impl crate::Writable for FLCTL_BANK0_MAIN_WEPROT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLCTL_BANK0_MAIN_WEPROT to value 0xffff_ffff"]
impl crate::Resettable for FLCTL_BANK0_MAIN_WEPROT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
