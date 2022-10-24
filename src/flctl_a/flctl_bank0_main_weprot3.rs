#[doc = "Register `FLCTL_BANK0_MAIN_WEPROT3` reader"]
pub struct R(crate::R<FLCTL_BANK0_MAIN_WEPROT3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_BANK0_MAIN_WEPROT3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_BANK0_MAIN_WEPROT3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_BANK0_MAIN_WEPROT3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLCTL_BANK0_MAIN_WEPROT3` writer"]
pub struct W(crate::W<FLCTL_BANK0_MAIN_WEPROT3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLCTL_BANK0_MAIN_WEPROT3_SPEC>;
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
impl From<crate::W<FLCTL_BANK0_MAIN_WEPROT3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLCTL_BANK0_MAIN_WEPROT3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROT96` reader - Protects Sector 96 from program or erase"]
pub type PROT96_R = crate::BitReader<bool>;
#[doc = "Field `PROT96` writer - Protects Sector 96 from program or erase"]
pub type PROT96_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT97` reader - Protects Sector 97 from program or erase"]
pub type PROT97_R = crate::BitReader<bool>;
#[doc = "Field `PROT97` writer - Protects Sector 97 from program or erase"]
pub type PROT97_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT98` reader - Protects Sector 98 from program or erase"]
pub type PROT98_R = crate::BitReader<bool>;
#[doc = "Field `PROT98` writer - Protects Sector 98 from program or erase"]
pub type PROT98_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT99` reader - Protects Sector 99 from program or erase"]
pub type PROT99_R = crate::BitReader<bool>;
#[doc = "Field `PROT99` writer - Protects Sector 99 from program or erase"]
pub type PROT99_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT100` reader - Protects Sector 100 from program or erase"]
pub type PROT100_R = crate::BitReader<bool>;
#[doc = "Field `PROT100` writer - Protects Sector 100 from program or erase"]
pub type PROT100_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT101` reader - Protects Sector 101 from program or erase"]
pub type PROT101_R = crate::BitReader<bool>;
#[doc = "Field `PROT101` writer - Protects Sector 101 from program or erase"]
pub type PROT101_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT102` reader - Protects Sector 102 from program or erase"]
pub type PROT102_R = crate::BitReader<bool>;
#[doc = "Field `PROT102` writer - Protects Sector 102 from program or erase"]
pub type PROT102_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT103` reader - Protects Sector 103 from program or erase"]
pub type PROT103_R = crate::BitReader<bool>;
#[doc = "Field `PROT103` writer - Protects Sector 103 from program or erase"]
pub type PROT103_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT104` reader - Protects Sector 104 from program or erase"]
pub type PROT104_R = crate::BitReader<bool>;
#[doc = "Field `PROT104` writer - Protects Sector 104 from program or erase"]
pub type PROT104_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT105` reader - Protects Sector 105 from program or erase"]
pub type PROT105_R = crate::BitReader<bool>;
#[doc = "Field `PROT105` writer - Protects Sector 105 from program or erase"]
pub type PROT105_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT106` reader - Protects Sector 106 from program or erase"]
pub type PROT106_R = crate::BitReader<bool>;
#[doc = "Field `PROT106` writer - Protects Sector 106 from program or erase"]
pub type PROT106_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT107` reader - Protects Sector 107 from program or erase"]
pub type PROT107_R = crate::BitReader<bool>;
#[doc = "Field `PROT107` writer - Protects Sector 107 from program or erase"]
pub type PROT107_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT108` reader - Protects Sector 108 from program or erase"]
pub type PROT108_R = crate::BitReader<bool>;
#[doc = "Field `PROT108` writer - Protects Sector 108 from program or erase"]
pub type PROT108_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT109` reader - Protects Sector 109 from program or erase"]
pub type PROT109_R = crate::BitReader<bool>;
#[doc = "Field `PROT109` writer - Protects Sector 109 from program or erase"]
pub type PROT109_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT110` reader - Protects Sector 110 from program or erase"]
pub type PROT110_R = crate::BitReader<bool>;
#[doc = "Field `PROT110` writer - Protects Sector 110 from program or erase"]
pub type PROT110_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT111` reader - Protects Sector 111 from program or erase"]
pub type PROT111_R = crate::BitReader<bool>;
#[doc = "Field `PROT111` writer - Protects Sector 111 from program or erase"]
pub type PROT111_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT112` reader - Protects Sector 112 from program or erase"]
pub type PROT112_R = crate::BitReader<bool>;
#[doc = "Field `PROT112` writer - Protects Sector 112 from program or erase"]
pub type PROT112_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT113` reader - Protects Sector 113 from program or erase"]
pub type PROT113_R = crate::BitReader<bool>;
#[doc = "Field `PROT113` writer - Protects Sector 113 from program or erase"]
pub type PROT113_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT114` reader - Protects Sector 114 from program or erase"]
pub type PROT114_R = crate::BitReader<bool>;
#[doc = "Field `PROT114` writer - Protects Sector 114 from program or erase"]
pub type PROT114_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT115` reader - Protects Sector 115 from program or erase"]
pub type PROT115_R = crate::BitReader<bool>;
#[doc = "Field `PROT115` writer - Protects Sector 115 from program or erase"]
pub type PROT115_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT116` reader - Protects Sector 116 from program or erase"]
pub type PROT116_R = crate::BitReader<bool>;
#[doc = "Field `PROT116` writer - Protects Sector 116 from program or erase"]
pub type PROT116_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT117` reader - Protects Sector 117 from program or erase"]
pub type PROT117_R = crate::BitReader<bool>;
#[doc = "Field `PROT117` writer - Protects Sector 117 from program or erase"]
pub type PROT117_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT118` reader - Protects Sector 118 from program or erase"]
pub type PROT118_R = crate::BitReader<bool>;
#[doc = "Field `PROT118` writer - Protects Sector 118 from program or erase"]
pub type PROT118_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT119` reader - Protects Sector 119 from program or erase"]
pub type PROT119_R = crate::BitReader<bool>;
#[doc = "Field `PROT119` writer - Protects Sector 119 from program or erase"]
pub type PROT119_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT120` reader - Protects Sector 120 from program or erase"]
pub type PROT120_R = crate::BitReader<bool>;
#[doc = "Field `PROT120` writer - Protects Sector 120 from program or erase"]
pub type PROT120_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT121` reader - Protects Sector 121 from program or erase"]
pub type PROT121_R = crate::BitReader<bool>;
#[doc = "Field `PROT121` writer - Protects Sector 121 from program or erase"]
pub type PROT121_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT122` reader - Protects Sector 122 from program or erase"]
pub type PROT122_R = crate::BitReader<bool>;
#[doc = "Field `PROT122` writer - Protects Sector 122 from program or erase"]
pub type PROT122_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT123` reader - Protects Sector 123 from program or erase"]
pub type PROT123_R = crate::BitReader<bool>;
#[doc = "Field `PROT123` writer - Protects Sector 123 from program or erase"]
pub type PROT123_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT124` reader - Protects Sector 124 from program or erase"]
pub type PROT124_R = crate::BitReader<bool>;
#[doc = "Field `PROT124` writer - Protects Sector 124 from program or erase"]
pub type PROT124_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT125` reader - Protects Sector 125 from program or erase"]
pub type PROT125_R = crate::BitReader<bool>;
#[doc = "Field `PROT125` writer - Protects Sector 125 from program or erase"]
pub type PROT125_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT126` reader - Protects Sector 126 from program or erase"]
pub type PROT126_R = crate::BitReader<bool>;
#[doc = "Field `PROT126` writer - Protects Sector 126 from program or erase"]
pub type PROT126_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
#[doc = "Field `PROT127` reader - Protects Sector 127 from program or erase"]
pub type PROT127_R = crate::BitReader<bool>;
#[doc = "Field `PROT127` writer - Protects Sector 127 from program or erase"]
pub type PROT127_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BANK0_MAIN_WEPROT3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Protects Sector 96 from program or erase"]
    #[inline(always)]
    pub fn prot96(&self) -> PROT96_R {
        PROT96_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Protects Sector 97 from program or erase"]
    #[inline(always)]
    pub fn prot97(&self) -> PROT97_R {
        PROT97_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Protects Sector 98 from program or erase"]
    #[inline(always)]
    pub fn prot98(&self) -> PROT98_R {
        PROT98_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Protects Sector 99 from program or erase"]
    #[inline(always)]
    pub fn prot99(&self) -> PROT99_R {
        PROT99_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Protects Sector 100 from program or erase"]
    #[inline(always)]
    pub fn prot100(&self) -> PROT100_R {
        PROT100_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Protects Sector 101 from program or erase"]
    #[inline(always)]
    pub fn prot101(&self) -> PROT101_R {
        PROT101_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Protects Sector 102 from program or erase"]
    #[inline(always)]
    pub fn prot102(&self) -> PROT102_R {
        PROT102_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Protects Sector 103 from program or erase"]
    #[inline(always)]
    pub fn prot103(&self) -> PROT103_R {
        PROT103_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Protects Sector 104 from program or erase"]
    #[inline(always)]
    pub fn prot104(&self) -> PROT104_R {
        PROT104_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Protects Sector 105 from program or erase"]
    #[inline(always)]
    pub fn prot105(&self) -> PROT105_R {
        PROT105_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Protects Sector 106 from program or erase"]
    #[inline(always)]
    pub fn prot106(&self) -> PROT106_R {
        PROT106_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Protects Sector 107 from program or erase"]
    #[inline(always)]
    pub fn prot107(&self) -> PROT107_R {
        PROT107_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Protects Sector 108 from program or erase"]
    #[inline(always)]
    pub fn prot108(&self) -> PROT108_R {
        PROT108_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Protects Sector 109 from program or erase"]
    #[inline(always)]
    pub fn prot109(&self) -> PROT109_R {
        PROT109_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Protects Sector 110 from program or erase"]
    #[inline(always)]
    pub fn prot110(&self) -> PROT110_R {
        PROT110_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Protects Sector 111 from program or erase"]
    #[inline(always)]
    pub fn prot111(&self) -> PROT111_R {
        PROT111_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Protects Sector 112 from program or erase"]
    #[inline(always)]
    pub fn prot112(&self) -> PROT112_R {
        PROT112_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Protects Sector 113 from program or erase"]
    #[inline(always)]
    pub fn prot113(&self) -> PROT113_R {
        PROT113_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Protects Sector 114 from program or erase"]
    #[inline(always)]
    pub fn prot114(&self) -> PROT114_R {
        PROT114_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Protects Sector 115 from program or erase"]
    #[inline(always)]
    pub fn prot115(&self) -> PROT115_R {
        PROT115_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Protects Sector 116 from program or erase"]
    #[inline(always)]
    pub fn prot116(&self) -> PROT116_R {
        PROT116_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Protects Sector 117 from program or erase"]
    #[inline(always)]
    pub fn prot117(&self) -> PROT117_R {
        PROT117_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Protects Sector 118 from program or erase"]
    #[inline(always)]
    pub fn prot118(&self) -> PROT118_R {
        PROT118_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Protects Sector 119 from program or erase"]
    #[inline(always)]
    pub fn prot119(&self) -> PROT119_R {
        PROT119_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Protects Sector 120 from program or erase"]
    #[inline(always)]
    pub fn prot120(&self) -> PROT120_R {
        PROT120_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Protects Sector 121 from program or erase"]
    #[inline(always)]
    pub fn prot121(&self) -> PROT121_R {
        PROT121_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Protects Sector 122 from program or erase"]
    #[inline(always)]
    pub fn prot122(&self) -> PROT122_R {
        PROT122_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protects Sector 123 from program or erase"]
    #[inline(always)]
    pub fn prot123(&self) -> PROT123_R {
        PROT123_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protects Sector 124 from program or erase"]
    #[inline(always)]
    pub fn prot124(&self) -> PROT124_R {
        PROT124_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Protects Sector 125 from program or erase"]
    #[inline(always)]
    pub fn prot125(&self) -> PROT125_R {
        PROT125_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Protects Sector 126 from program or erase"]
    #[inline(always)]
    pub fn prot126(&self) -> PROT126_R {
        PROT126_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Protects Sector 127 from program or erase"]
    #[inline(always)]
    pub fn prot127(&self) -> PROT127_R {
        PROT127_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Protects Sector 96 from program or erase"]
    #[inline(always)]
    pub fn prot96(&mut self) -> PROT96_W<0> {
        PROT96_W::new(self)
    }
    #[doc = "Bit 1 - Protects Sector 97 from program or erase"]
    #[inline(always)]
    pub fn prot97(&mut self) -> PROT97_W<1> {
        PROT97_W::new(self)
    }
    #[doc = "Bit 2 - Protects Sector 98 from program or erase"]
    #[inline(always)]
    pub fn prot98(&mut self) -> PROT98_W<2> {
        PROT98_W::new(self)
    }
    #[doc = "Bit 3 - Protects Sector 99 from program or erase"]
    #[inline(always)]
    pub fn prot99(&mut self) -> PROT99_W<3> {
        PROT99_W::new(self)
    }
    #[doc = "Bit 4 - Protects Sector 100 from program or erase"]
    #[inline(always)]
    pub fn prot100(&mut self) -> PROT100_W<4> {
        PROT100_W::new(self)
    }
    #[doc = "Bit 5 - Protects Sector 101 from program or erase"]
    #[inline(always)]
    pub fn prot101(&mut self) -> PROT101_W<5> {
        PROT101_W::new(self)
    }
    #[doc = "Bit 6 - Protects Sector 102 from program or erase"]
    #[inline(always)]
    pub fn prot102(&mut self) -> PROT102_W<6> {
        PROT102_W::new(self)
    }
    #[doc = "Bit 7 - Protects Sector 103 from program or erase"]
    #[inline(always)]
    pub fn prot103(&mut self) -> PROT103_W<7> {
        PROT103_W::new(self)
    }
    #[doc = "Bit 8 - Protects Sector 104 from program or erase"]
    #[inline(always)]
    pub fn prot104(&mut self) -> PROT104_W<8> {
        PROT104_W::new(self)
    }
    #[doc = "Bit 9 - Protects Sector 105 from program or erase"]
    #[inline(always)]
    pub fn prot105(&mut self) -> PROT105_W<9> {
        PROT105_W::new(self)
    }
    #[doc = "Bit 10 - Protects Sector 106 from program or erase"]
    #[inline(always)]
    pub fn prot106(&mut self) -> PROT106_W<10> {
        PROT106_W::new(self)
    }
    #[doc = "Bit 11 - Protects Sector 107 from program or erase"]
    #[inline(always)]
    pub fn prot107(&mut self) -> PROT107_W<11> {
        PROT107_W::new(self)
    }
    #[doc = "Bit 12 - Protects Sector 108 from program or erase"]
    #[inline(always)]
    pub fn prot108(&mut self) -> PROT108_W<12> {
        PROT108_W::new(self)
    }
    #[doc = "Bit 13 - Protects Sector 109 from program or erase"]
    #[inline(always)]
    pub fn prot109(&mut self) -> PROT109_W<13> {
        PROT109_W::new(self)
    }
    #[doc = "Bit 14 - Protects Sector 110 from program or erase"]
    #[inline(always)]
    pub fn prot110(&mut self) -> PROT110_W<14> {
        PROT110_W::new(self)
    }
    #[doc = "Bit 15 - Protects Sector 111 from program or erase"]
    #[inline(always)]
    pub fn prot111(&mut self) -> PROT111_W<15> {
        PROT111_W::new(self)
    }
    #[doc = "Bit 16 - Protects Sector 112 from program or erase"]
    #[inline(always)]
    pub fn prot112(&mut self) -> PROT112_W<16> {
        PROT112_W::new(self)
    }
    #[doc = "Bit 17 - Protects Sector 113 from program or erase"]
    #[inline(always)]
    pub fn prot113(&mut self) -> PROT113_W<17> {
        PROT113_W::new(self)
    }
    #[doc = "Bit 18 - Protects Sector 114 from program or erase"]
    #[inline(always)]
    pub fn prot114(&mut self) -> PROT114_W<18> {
        PROT114_W::new(self)
    }
    #[doc = "Bit 19 - Protects Sector 115 from program or erase"]
    #[inline(always)]
    pub fn prot115(&mut self) -> PROT115_W<19> {
        PROT115_W::new(self)
    }
    #[doc = "Bit 20 - Protects Sector 116 from program or erase"]
    #[inline(always)]
    pub fn prot116(&mut self) -> PROT116_W<20> {
        PROT116_W::new(self)
    }
    #[doc = "Bit 21 - Protects Sector 117 from program or erase"]
    #[inline(always)]
    pub fn prot117(&mut self) -> PROT117_W<21> {
        PROT117_W::new(self)
    }
    #[doc = "Bit 22 - Protects Sector 118 from program or erase"]
    #[inline(always)]
    pub fn prot118(&mut self) -> PROT118_W<22> {
        PROT118_W::new(self)
    }
    #[doc = "Bit 23 - Protects Sector 119 from program or erase"]
    #[inline(always)]
    pub fn prot119(&mut self) -> PROT119_W<23> {
        PROT119_W::new(self)
    }
    #[doc = "Bit 24 - Protects Sector 120 from program or erase"]
    #[inline(always)]
    pub fn prot120(&mut self) -> PROT120_W<24> {
        PROT120_W::new(self)
    }
    #[doc = "Bit 25 - Protects Sector 121 from program or erase"]
    #[inline(always)]
    pub fn prot121(&mut self) -> PROT121_W<25> {
        PROT121_W::new(self)
    }
    #[doc = "Bit 26 - Protects Sector 122 from program or erase"]
    #[inline(always)]
    pub fn prot122(&mut self) -> PROT122_W<26> {
        PROT122_W::new(self)
    }
    #[doc = "Bit 27 - Protects Sector 123 from program or erase"]
    #[inline(always)]
    pub fn prot123(&mut self) -> PROT123_W<27> {
        PROT123_W::new(self)
    }
    #[doc = "Bit 28 - Protects Sector 124 from program or erase"]
    #[inline(always)]
    pub fn prot124(&mut self) -> PROT124_W<28> {
        PROT124_W::new(self)
    }
    #[doc = "Bit 29 - Protects Sector 125 from program or erase"]
    #[inline(always)]
    pub fn prot125(&mut self) -> PROT125_W<29> {
        PROT125_W::new(self)
    }
    #[doc = "Bit 30 - Protects Sector 126 from program or erase"]
    #[inline(always)]
    pub fn prot126(&mut self) -> PROT126_W<30> {
        PROT126_W::new(self)
    }
    #[doc = "Bit 31 - Protects Sector 127 from program or erase"]
    #[inline(always)]
    pub fn prot127(&mut self) -> PROT127_W<31> {
        PROT127_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Memory Bank0 Write/Erase Protection Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_bank0_main_weprot3](index.html) module"]
pub struct FLCTL_BANK0_MAIN_WEPROT3_SPEC;
impl crate::RegisterSpec for FLCTL_BANK0_MAIN_WEPROT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_bank0_main_weprot3::R](R) reader structure"]
impl crate::Readable for FLCTL_BANK0_MAIN_WEPROT3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flctl_bank0_main_weprot3::W](W) writer structure"]
impl crate::Writable for FLCTL_BANK0_MAIN_WEPROT3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLCTL_BANK0_MAIN_WEPROT3 to value 0xffff_ffff"]
impl crate::Resettable for FLCTL_BANK0_MAIN_WEPROT3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
