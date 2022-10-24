#[doc = "Register `SYS_SRAM_BLKRET_CTL1` reader"]
pub struct R(crate::R<SYS_SRAM_BLKRET_CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_SRAM_BLKRET_CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_SRAM_BLKRET_CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_SRAM_BLKRET_CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_SRAM_BLKRET_CTL1` writer"]
pub struct W(crate::W<SYS_SRAM_BLKRET_CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_SRAM_BLKRET_CTL1_SPEC>;
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
impl From<crate::W<SYS_SRAM_BLKRET_CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_SRAM_BLKRET_CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLK32_EN` reader - When 1, Block32 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK32_EN_R = crate::BitReader<BLK32_EN_A>;
#[doc = "When 1, Block32 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK32_EN_A {
    #[doc = "0: Block32 of the SRAM is not retained in LPM3 or LPM4"]
    BLK32_EN_0 = 0,
    #[doc = "1: Block32 of the SRAM is retained in LPM3 and LPM4"]
    BLK32_EN_1 = 1,
}
impl From<BLK32_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK32_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK32_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK32_EN_A {
        match self.bits {
            false => BLK32_EN_A::BLK32_EN_0,
            true => BLK32_EN_A::BLK32_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK32_EN_0`"]
    #[inline(always)]
    pub fn is_blk32_en_0(&self) -> bool {
        *self == BLK32_EN_A::BLK32_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK32_EN_1`"]
    #[inline(always)]
    pub fn is_blk32_en_1(&self) -> bool {
        *self == BLK32_EN_A::BLK32_EN_1
    }
}
#[doc = "Field `BLK32_EN` writer - When 1, Block32 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK32_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK32_EN_A, O>;
impl<'a, const O: u8> BLK32_EN_W<'a, O> {
    #[doc = "Block32 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk32_en_0(self) -> &'a mut W {
        self.variant(BLK32_EN_A::BLK32_EN_0)
    }
    #[doc = "Block32 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk32_en_1(self) -> &'a mut W {
        self.variant(BLK32_EN_A::BLK32_EN_1)
    }
}
#[doc = "Field `BLK33_EN` reader - When 1, Block33 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK33_EN_R = crate::BitReader<BLK33_EN_A>;
#[doc = "When 1, Block33 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK33_EN_A {
    #[doc = "0: Block33 of the SRAM is not retained in LPM3 or LPM4"]
    BLK33_EN_0 = 0,
    #[doc = "1: Block33 of the SRAM is retained in LPM3 and LPM4"]
    BLK33_EN_1 = 1,
}
impl From<BLK33_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK33_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK33_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK33_EN_A {
        match self.bits {
            false => BLK33_EN_A::BLK33_EN_0,
            true => BLK33_EN_A::BLK33_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK33_EN_0`"]
    #[inline(always)]
    pub fn is_blk33_en_0(&self) -> bool {
        *self == BLK33_EN_A::BLK33_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK33_EN_1`"]
    #[inline(always)]
    pub fn is_blk33_en_1(&self) -> bool {
        *self == BLK33_EN_A::BLK33_EN_1
    }
}
#[doc = "Field `BLK33_EN` writer - When 1, Block33 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK33_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK33_EN_A, O>;
impl<'a, const O: u8> BLK33_EN_W<'a, O> {
    #[doc = "Block33 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk33_en_0(self) -> &'a mut W {
        self.variant(BLK33_EN_A::BLK33_EN_0)
    }
    #[doc = "Block33 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk33_en_1(self) -> &'a mut W {
        self.variant(BLK33_EN_A::BLK33_EN_1)
    }
}
#[doc = "Field `BLK34_EN` reader - When 1, Block34 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK34_EN_R = crate::BitReader<BLK34_EN_A>;
#[doc = "When 1, Block34 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK34_EN_A {
    #[doc = "0: Block34 of the SRAM is not retained in LPM3 or LPM4"]
    BLK34_EN_0 = 0,
    #[doc = "1: Block34 of the SRAM is retained in LPM3 and LPM4"]
    BLK34_EN_1 = 1,
}
impl From<BLK34_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK34_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK34_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK34_EN_A {
        match self.bits {
            false => BLK34_EN_A::BLK34_EN_0,
            true => BLK34_EN_A::BLK34_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK34_EN_0`"]
    #[inline(always)]
    pub fn is_blk34_en_0(&self) -> bool {
        *self == BLK34_EN_A::BLK34_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK34_EN_1`"]
    #[inline(always)]
    pub fn is_blk34_en_1(&self) -> bool {
        *self == BLK34_EN_A::BLK34_EN_1
    }
}
#[doc = "Field `BLK34_EN` writer - When 1, Block34 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK34_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK34_EN_A, O>;
impl<'a, const O: u8> BLK34_EN_W<'a, O> {
    #[doc = "Block34 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk34_en_0(self) -> &'a mut W {
        self.variant(BLK34_EN_A::BLK34_EN_0)
    }
    #[doc = "Block34 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk34_en_1(self) -> &'a mut W {
        self.variant(BLK34_EN_A::BLK34_EN_1)
    }
}
#[doc = "Field `BLK35_EN` reader - When 1, Block35 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK35_EN_R = crate::BitReader<BLK35_EN_A>;
#[doc = "When 1, Block35 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK35_EN_A {
    #[doc = "0: Block35 of the SRAM is not retained in LPM3 or LPM4"]
    BLK35_EN_0 = 0,
    #[doc = "1: Block35 of the SRAM is retained in LPM3 and LPM4"]
    BLK35_EN_1 = 1,
}
impl From<BLK35_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK35_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK35_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK35_EN_A {
        match self.bits {
            false => BLK35_EN_A::BLK35_EN_0,
            true => BLK35_EN_A::BLK35_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK35_EN_0`"]
    #[inline(always)]
    pub fn is_blk35_en_0(&self) -> bool {
        *self == BLK35_EN_A::BLK35_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK35_EN_1`"]
    #[inline(always)]
    pub fn is_blk35_en_1(&self) -> bool {
        *self == BLK35_EN_A::BLK35_EN_1
    }
}
#[doc = "Field `BLK35_EN` writer - When 1, Block35 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK35_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK35_EN_A, O>;
impl<'a, const O: u8> BLK35_EN_W<'a, O> {
    #[doc = "Block35 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk35_en_0(self) -> &'a mut W {
        self.variant(BLK35_EN_A::BLK35_EN_0)
    }
    #[doc = "Block35 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk35_en_1(self) -> &'a mut W {
        self.variant(BLK35_EN_A::BLK35_EN_1)
    }
}
#[doc = "Field `BLK36_EN` reader - When 1, Block36 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK36_EN_R = crate::BitReader<BLK36_EN_A>;
#[doc = "When 1, Block36 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK36_EN_A {
    #[doc = "0: Block36 of the SRAM is not retained in LPM3 or LPM4"]
    BLK36_EN_0 = 0,
    #[doc = "1: Block36 of the SRAM is retained in LPM3 and LPM4"]
    BLK36_EN_1 = 1,
}
impl From<BLK36_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK36_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK36_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK36_EN_A {
        match self.bits {
            false => BLK36_EN_A::BLK36_EN_0,
            true => BLK36_EN_A::BLK36_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK36_EN_0`"]
    #[inline(always)]
    pub fn is_blk36_en_0(&self) -> bool {
        *self == BLK36_EN_A::BLK36_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK36_EN_1`"]
    #[inline(always)]
    pub fn is_blk36_en_1(&self) -> bool {
        *self == BLK36_EN_A::BLK36_EN_1
    }
}
#[doc = "Field `BLK36_EN` writer - When 1, Block36 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK36_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK36_EN_A, O>;
impl<'a, const O: u8> BLK36_EN_W<'a, O> {
    #[doc = "Block36 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk36_en_0(self) -> &'a mut W {
        self.variant(BLK36_EN_A::BLK36_EN_0)
    }
    #[doc = "Block36 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk36_en_1(self) -> &'a mut W {
        self.variant(BLK36_EN_A::BLK36_EN_1)
    }
}
#[doc = "Field `BLK37_EN` reader - When 1, Block37 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK37_EN_R = crate::BitReader<BLK37_EN_A>;
#[doc = "When 1, Block37 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK37_EN_A {
    #[doc = "0: Block37 of the SRAM is not retained in LPM3 or LPM4"]
    BLK37_EN_0 = 0,
    #[doc = "1: Block37 of the SRAM is retained in LPM3 and LPM4"]
    BLK37_EN_1 = 1,
}
impl From<BLK37_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK37_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK37_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK37_EN_A {
        match self.bits {
            false => BLK37_EN_A::BLK37_EN_0,
            true => BLK37_EN_A::BLK37_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK37_EN_0`"]
    #[inline(always)]
    pub fn is_blk37_en_0(&self) -> bool {
        *self == BLK37_EN_A::BLK37_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK37_EN_1`"]
    #[inline(always)]
    pub fn is_blk37_en_1(&self) -> bool {
        *self == BLK37_EN_A::BLK37_EN_1
    }
}
#[doc = "Field `BLK37_EN` writer - When 1, Block37 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK37_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK37_EN_A, O>;
impl<'a, const O: u8> BLK37_EN_W<'a, O> {
    #[doc = "Block37 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk37_en_0(self) -> &'a mut W {
        self.variant(BLK37_EN_A::BLK37_EN_0)
    }
    #[doc = "Block37 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk37_en_1(self) -> &'a mut W {
        self.variant(BLK37_EN_A::BLK37_EN_1)
    }
}
#[doc = "Field `BLK38_EN` reader - When 1, Block38 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK38_EN_R = crate::BitReader<BLK38_EN_A>;
#[doc = "When 1, Block38 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK38_EN_A {
    #[doc = "0: Block38 of the SRAM is not retained in LPM3 or LPM4"]
    BLK38_EN_0 = 0,
    #[doc = "1: Block38 of the SRAM is retained in LPM3 and LPM4"]
    BLK38_EN_1 = 1,
}
impl From<BLK38_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK38_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK38_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK38_EN_A {
        match self.bits {
            false => BLK38_EN_A::BLK38_EN_0,
            true => BLK38_EN_A::BLK38_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK38_EN_0`"]
    #[inline(always)]
    pub fn is_blk38_en_0(&self) -> bool {
        *self == BLK38_EN_A::BLK38_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK38_EN_1`"]
    #[inline(always)]
    pub fn is_blk38_en_1(&self) -> bool {
        *self == BLK38_EN_A::BLK38_EN_1
    }
}
#[doc = "Field `BLK38_EN` writer - When 1, Block38 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK38_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK38_EN_A, O>;
impl<'a, const O: u8> BLK38_EN_W<'a, O> {
    #[doc = "Block38 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk38_en_0(self) -> &'a mut W {
        self.variant(BLK38_EN_A::BLK38_EN_0)
    }
    #[doc = "Block38 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk38_en_1(self) -> &'a mut W {
        self.variant(BLK38_EN_A::BLK38_EN_1)
    }
}
#[doc = "Field `BLK39_EN` reader - When 1, Block39 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK39_EN_R = crate::BitReader<BLK39_EN_A>;
#[doc = "When 1, Block39 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK39_EN_A {
    #[doc = "0: Block39 of the SRAM is not retained in LPM3 or LPM4"]
    BLK39_EN_0 = 0,
    #[doc = "1: Block39 of the SRAM is retained in LPM3 and LPM4"]
    BLK39_EN_1 = 1,
}
impl From<BLK39_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK39_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK39_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK39_EN_A {
        match self.bits {
            false => BLK39_EN_A::BLK39_EN_0,
            true => BLK39_EN_A::BLK39_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK39_EN_0`"]
    #[inline(always)]
    pub fn is_blk39_en_0(&self) -> bool {
        *self == BLK39_EN_A::BLK39_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK39_EN_1`"]
    #[inline(always)]
    pub fn is_blk39_en_1(&self) -> bool {
        *self == BLK39_EN_A::BLK39_EN_1
    }
}
#[doc = "Field `BLK39_EN` writer - When 1, Block39 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK39_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK39_EN_A, O>;
impl<'a, const O: u8> BLK39_EN_W<'a, O> {
    #[doc = "Block39 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk39_en_0(self) -> &'a mut W {
        self.variant(BLK39_EN_A::BLK39_EN_0)
    }
    #[doc = "Block39 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk39_en_1(self) -> &'a mut W {
        self.variant(BLK39_EN_A::BLK39_EN_1)
    }
}
#[doc = "Field `BLK40_EN` reader - When 1, Block40 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK40_EN_R = crate::BitReader<BLK40_EN_A>;
#[doc = "When 1, Block40 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK40_EN_A {
    #[doc = "0: Block40 of the SRAM is not retained in LPM3 or LPM4"]
    BLK40_EN_0 = 0,
    #[doc = "1: Block40 of the SRAM is retained in LPM3 and LPM4"]
    BLK40_EN_1 = 1,
}
impl From<BLK40_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK40_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK40_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK40_EN_A {
        match self.bits {
            false => BLK40_EN_A::BLK40_EN_0,
            true => BLK40_EN_A::BLK40_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK40_EN_0`"]
    #[inline(always)]
    pub fn is_blk40_en_0(&self) -> bool {
        *self == BLK40_EN_A::BLK40_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK40_EN_1`"]
    #[inline(always)]
    pub fn is_blk40_en_1(&self) -> bool {
        *self == BLK40_EN_A::BLK40_EN_1
    }
}
#[doc = "Field `BLK40_EN` writer - When 1, Block40 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK40_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK40_EN_A, O>;
impl<'a, const O: u8> BLK40_EN_W<'a, O> {
    #[doc = "Block40 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk40_en_0(self) -> &'a mut W {
        self.variant(BLK40_EN_A::BLK40_EN_0)
    }
    #[doc = "Block40 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk40_en_1(self) -> &'a mut W {
        self.variant(BLK40_EN_A::BLK40_EN_1)
    }
}
#[doc = "Field `BLK41_EN` reader - When 1, Block41 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK41_EN_R = crate::BitReader<BLK41_EN_A>;
#[doc = "When 1, Block41 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK41_EN_A {
    #[doc = "0: Block41 of the SRAM is not retained in LPM3 or LPM4"]
    BLK41_EN_0 = 0,
    #[doc = "1: Block41 of the SRAM is retained in LPM3 and LPM4"]
    BLK41_EN_1 = 1,
}
impl From<BLK41_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK41_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK41_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK41_EN_A {
        match self.bits {
            false => BLK41_EN_A::BLK41_EN_0,
            true => BLK41_EN_A::BLK41_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK41_EN_0`"]
    #[inline(always)]
    pub fn is_blk41_en_0(&self) -> bool {
        *self == BLK41_EN_A::BLK41_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK41_EN_1`"]
    #[inline(always)]
    pub fn is_blk41_en_1(&self) -> bool {
        *self == BLK41_EN_A::BLK41_EN_1
    }
}
#[doc = "Field `BLK41_EN` writer - When 1, Block41 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK41_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK41_EN_A, O>;
impl<'a, const O: u8> BLK41_EN_W<'a, O> {
    #[doc = "Block41 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk41_en_0(self) -> &'a mut W {
        self.variant(BLK41_EN_A::BLK41_EN_0)
    }
    #[doc = "Block41 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk41_en_1(self) -> &'a mut W {
        self.variant(BLK41_EN_A::BLK41_EN_1)
    }
}
#[doc = "Field `BLK42_EN` reader - When 1, Block42 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK42_EN_R = crate::BitReader<BLK42_EN_A>;
#[doc = "When 1, Block42 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK42_EN_A {
    #[doc = "0: Block42 of the SRAM is not retained in LPM3 or LPM4"]
    BLK42_EN_0 = 0,
    #[doc = "1: Block42 of the SRAM is retained in LPM3 and LPM4"]
    BLK42_EN_1 = 1,
}
impl From<BLK42_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK42_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK42_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK42_EN_A {
        match self.bits {
            false => BLK42_EN_A::BLK42_EN_0,
            true => BLK42_EN_A::BLK42_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK42_EN_0`"]
    #[inline(always)]
    pub fn is_blk42_en_0(&self) -> bool {
        *self == BLK42_EN_A::BLK42_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK42_EN_1`"]
    #[inline(always)]
    pub fn is_blk42_en_1(&self) -> bool {
        *self == BLK42_EN_A::BLK42_EN_1
    }
}
#[doc = "Field `BLK42_EN` writer - When 1, Block42 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK42_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK42_EN_A, O>;
impl<'a, const O: u8> BLK42_EN_W<'a, O> {
    #[doc = "Block42 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk42_en_0(self) -> &'a mut W {
        self.variant(BLK42_EN_A::BLK42_EN_0)
    }
    #[doc = "Block42 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk42_en_1(self) -> &'a mut W {
        self.variant(BLK42_EN_A::BLK42_EN_1)
    }
}
#[doc = "Field `BLK43_EN` reader - When 1, Block43 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK43_EN_R = crate::BitReader<BLK43_EN_A>;
#[doc = "When 1, Block43 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK43_EN_A {
    #[doc = "0: Block43 of the SRAM is not retained in LPM3 or LPM4"]
    BLK43_EN_0 = 0,
    #[doc = "1: Block43 of the SRAM is retained in LPM3 and LPM4"]
    BLK43_EN_1 = 1,
}
impl From<BLK43_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK43_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK43_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK43_EN_A {
        match self.bits {
            false => BLK43_EN_A::BLK43_EN_0,
            true => BLK43_EN_A::BLK43_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK43_EN_0`"]
    #[inline(always)]
    pub fn is_blk43_en_0(&self) -> bool {
        *self == BLK43_EN_A::BLK43_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK43_EN_1`"]
    #[inline(always)]
    pub fn is_blk43_en_1(&self) -> bool {
        *self == BLK43_EN_A::BLK43_EN_1
    }
}
#[doc = "Field `BLK43_EN` writer - When 1, Block43 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK43_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK43_EN_A, O>;
impl<'a, const O: u8> BLK43_EN_W<'a, O> {
    #[doc = "Block43 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk43_en_0(self) -> &'a mut W {
        self.variant(BLK43_EN_A::BLK43_EN_0)
    }
    #[doc = "Block43 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk43_en_1(self) -> &'a mut W {
        self.variant(BLK43_EN_A::BLK43_EN_1)
    }
}
#[doc = "Field `BLK44_EN` reader - When 1, Block44 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK44_EN_R = crate::BitReader<BLK44_EN_A>;
#[doc = "When 1, Block44 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK44_EN_A {
    #[doc = "0: Block44 of the SRAM is not retained in LPM3 or LPM4"]
    BLK44_EN_0 = 0,
    #[doc = "1: Block44 of the SRAM is retained in LPM3 and LPM4"]
    BLK44_EN_1 = 1,
}
impl From<BLK44_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK44_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK44_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK44_EN_A {
        match self.bits {
            false => BLK44_EN_A::BLK44_EN_0,
            true => BLK44_EN_A::BLK44_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK44_EN_0`"]
    #[inline(always)]
    pub fn is_blk44_en_0(&self) -> bool {
        *self == BLK44_EN_A::BLK44_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK44_EN_1`"]
    #[inline(always)]
    pub fn is_blk44_en_1(&self) -> bool {
        *self == BLK44_EN_A::BLK44_EN_1
    }
}
#[doc = "Field `BLK44_EN` writer - When 1, Block44 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK44_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK44_EN_A, O>;
impl<'a, const O: u8> BLK44_EN_W<'a, O> {
    #[doc = "Block44 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk44_en_0(self) -> &'a mut W {
        self.variant(BLK44_EN_A::BLK44_EN_0)
    }
    #[doc = "Block44 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk44_en_1(self) -> &'a mut W {
        self.variant(BLK44_EN_A::BLK44_EN_1)
    }
}
#[doc = "Field `BLK45_EN` reader - When 1, Block45 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK45_EN_R = crate::BitReader<BLK45_EN_A>;
#[doc = "When 1, Block45 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK45_EN_A {
    #[doc = "0: Block45 of the SRAM is not retained in LPM3 or LPM4"]
    BLK45_EN_0 = 0,
    #[doc = "1: Block45 of the SRAM is retained in LPM3 and LPM4"]
    BLK45_EN_1 = 1,
}
impl From<BLK45_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK45_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK45_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK45_EN_A {
        match self.bits {
            false => BLK45_EN_A::BLK45_EN_0,
            true => BLK45_EN_A::BLK45_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK45_EN_0`"]
    #[inline(always)]
    pub fn is_blk45_en_0(&self) -> bool {
        *self == BLK45_EN_A::BLK45_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK45_EN_1`"]
    #[inline(always)]
    pub fn is_blk45_en_1(&self) -> bool {
        *self == BLK45_EN_A::BLK45_EN_1
    }
}
#[doc = "Field `BLK45_EN` writer - When 1, Block45 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK45_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK45_EN_A, O>;
impl<'a, const O: u8> BLK45_EN_W<'a, O> {
    #[doc = "Block45 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk45_en_0(self) -> &'a mut W {
        self.variant(BLK45_EN_A::BLK45_EN_0)
    }
    #[doc = "Block45 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk45_en_1(self) -> &'a mut W {
        self.variant(BLK45_EN_A::BLK45_EN_1)
    }
}
#[doc = "Field `BLK46_EN` reader - When 1, Block46 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK46_EN_R = crate::BitReader<BLK46_EN_A>;
#[doc = "When 1, Block46 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK46_EN_A {
    #[doc = "0: Block46 of the SRAM is not retained in LPM3 or LPM4"]
    BLK46_EN_0 = 0,
    #[doc = "1: Block46 of the SRAM is retained in LPM3 and LPM4"]
    BLK46_EN_1 = 1,
}
impl From<BLK46_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK46_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK46_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK46_EN_A {
        match self.bits {
            false => BLK46_EN_A::BLK46_EN_0,
            true => BLK46_EN_A::BLK46_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK46_EN_0`"]
    #[inline(always)]
    pub fn is_blk46_en_0(&self) -> bool {
        *self == BLK46_EN_A::BLK46_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK46_EN_1`"]
    #[inline(always)]
    pub fn is_blk46_en_1(&self) -> bool {
        *self == BLK46_EN_A::BLK46_EN_1
    }
}
#[doc = "Field `BLK46_EN` writer - When 1, Block46 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK46_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK46_EN_A, O>;
impl<'a, const O: u8> BLK46_EN_W<'a, O> {
    #[doc = "Block46 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk46_en_0(self) -> &'a mut W {
        self.variant(BLK46_EN_A::BLK46_EN_0)
    }
    #[doc = "Block46 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk46_en_1(self) -> &'a mut W {
        self.variant(BLK46_EN_A::BLK46_EN_1)
    }
}
#[doc = "Field `BLK47_EN` reader - When 1, Block47 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK47_EN_R = crate::BitReader<BLK47_EN_A>;
#[doc = "When 1, Block47 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK47_EN_A {
    #[doc = "0: Block47 of the SRAM is not retained in LPM3 or LPM4"]
    BLK47_EN_0 = 0,
    #[doc = "1: Block47 of the SRAM is retained in LPM3 and LPM4"]
    BLK47_EN_1 = 1,
}
impl From<BLK47_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK47_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK47_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK47_EN_A {
        match self.bits {
            false => BLK47_EN_A::BLK47_EN_0,
            true => BLK47_EN_A::BLK47_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK47_EN_0`"]
    #[inline(always)]
    pub fn is_blk47_en_0(&self) -> bool {
        *self == BLK47_EN_A::BLK47_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK47_EN_1`"]
    #[inline(always)]
    pub fn is_blk47_en_1(&self) -> bool {
        *self == BLK47_EN_A::BLK47_EN_1
    }
}
#[doc = "Field `BLK47_EN` writer - When 1, Block47 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK47_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK47_EN_A, O>;
impl<'a, const O: u8> BLK47_EN_W<'a, O> {
    #[doc = "Block47 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk47_en_0(self) -> &'a mut W {
        self.variant(BLK47_EN_A::BLK47_EN_0)
    }
    #[doc = "Block47 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk47_en_1(self) -> &'a mut W {
        self.variant(BLK47_EN_A::BLK47_EN_1)
    }
}
#[doc = "Field `BLK48_EN` reader - When 1, Block48 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK48_EN_R = crate::BitReader<BLK48_EN_A>;
#[doc = "When 1, Block48 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK48_EN_A {
    #[doc = "0: Block48 of the SRAM is not retained in LPM3 or LPM4"]
    BLK48_EN_0 = 0,
    #[doc = "1: Block48 of the SRAM is retained in LPM3 and LPM4"]
    BLK48_EN_1 = 1,
}
impl From<BLK48_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK48_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK48_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK48_EN_A {
        match self.bits {
            false => BLK48_EN_A::BLK48_EN_0,
            true => BLK48_EN_A::BLK48_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK48_EN_0`"]
    #[inline(always)]
    pub fn is_blk48_en_0(&self) -> bool {
        *self == BLK48_EN_A::BLK48_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK48_EN_1`"]
    #[inline(always)]
    pub fn is_blk48_en_1(&self) -> bool {
        *self == BLK48_EN_A::BLK48_EN_1
    }
}
#[doc = "Field `BLK48_EN` writer - When 1, Block48 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK48_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK48_EN_A, O>;
impl<'a, const O: u8> BLK48_EN_W<'a, O> {
    #[doc = "Block48 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk48_en_0(self) -> &'a mut W {
        self.variant(BLK48_EN_A::BLK48_EN_0)
    }
    #[doc = "Block48 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk48_en_1(self) -> &'a mut W {
        self.variant(BLK48_EN_A::BLK48_EN_1)
    }
}
#[doc = "Field `BLK49_EN` reader - When 1, Block49 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK49_EN_R = crate::BitReader<BLK49_EN_A>;
#[doc = "When 1, Block49 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK49_EN_A {
    #[doc = "0: Block49 of the SRAM is not retained in LPM3 or LPM4"]
    BLK49_EN_0 = 0,
    #[doc = "1: Block49 of the SRAM is retained in LPM3 and LPM4"]
    BLK49_EN_1 = 1,
}
impl From<BLK49_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK49_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK49_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK49_EN_A {
        match self.bits {
            false => BLK49_EN_A::BLK49_EN_0,
            true => BLK49_EN_A::BLK49_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK49_EN_0`"]
    #[inline(always)]
    pub fn is_blk49_en_0(&self) -> bool {
        *self == BLK49_EN_A::BLK49_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK49_EN_1`"]
    #[inline(always)]
    pub fn is_blk49_en_1(&self) -> bool {
        *self == BLK49_EN_A::BLK49_EN_1
    }
}
#[doc = "Field `BLK49_EN` writer - When 1, Block49 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK49_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK49_EN_A, O>;
impl<'a, const O: u8> BLK49_EN_W<'a, O> {
    #[doc = "Block49 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk49_en_0(self) -> &'a mut W {
        self.variant(BLK49_EN_A::BLK49_EN_0)
    }
    #[doc = "Block49 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk49_en_1(self) -> &'a mut W {
        self.variant(BLK49_EN_A::BLK49_EN_1)
    }
}
#[doc = "Field `BLK50_EN` reader - When 1, Block50 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK50_EN_R = crate::BitReader<BLK50_EN_A>;
#[doc = "When 1, Block50 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK50_EN_A {
    #[doc = "0: Block50 of the SRAM is not retained in LPM3 or LPM4"]
    BLK50_EN_0 = 0,
    #[doc = "1: Block50 of the SRAM is retained in LPM3 and LPM4"]
    BLK50_EN_1 = 1,
}
impl From<BLK50_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK50_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK50_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK50_EN_A {
        match self.bits {
            false => BLK50_EN_A::BLK50_EN_0,
            true => BLK50_EN_A::BLK50_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK50_EN_0`"]
    #[inline(always)]
    pub fn is_blk50_en_0(&self) -> bool {
        *self == BLK50_EN_A::BLK50_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK50_EN_1`"]
    #[inline(always)]
    pub fn is_blk50_en_1(&self) -> bool {
        *self == BLK50_EN_A::BLK50_EN_1
    }
}
#[doc = "Field `BLK50_EN` writer - When 1, Block50 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK50_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK50_EN_A, O>;
impl<'a, const O: u8> BLK50_EN_W<'a, O> {
    #[doc = "Block50 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk50_en_0(self) -> &'a mut W {
        self.variant(BLK50_EN_A::BLK50_EN_0)
    }
    #[doc = "Block50 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk50_en_1(self) -> &'a mut W {
        self.variant(BLK50_EN_A::BLK50_EN_1)
    }
}
#[doc = "Field `BLK51_EN` reader - When 1, Block51 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK51_EN_R = crate::BitReader<BLK51_EN_A>;
#[doc = "When 1, Block51 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK51_EN_A {
    #[doc = "0: Block51 of the SRAM is not retained in LPM3 or LPM4"]
    BLK51_EN_0 = 0,
    #[doc = "1: Block51 of the SRAM is retained in LPM3 and LPM4"]
    BLK51_EN_1 = 1,
}
impl From<BLK51_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK51_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK51_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK51_EN_A {
        match self.bits {
            false => BLK51_EN_A::BLK51_EN_0,
            true => BLK51_EN_A::BLK51_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK51_EN_0`"]
    #[inline(always)]
    pub fn is_blk51_en_0(&self) -> bool {
        *self == BLK51_EN_A::BLK51_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK51_EN_1`"]
    #[inline(always)]
    pub fn is_blk51_en_1(&self) -> bool {
        *self == BLK51_EN_A::BLK51_EN_1
    }
}
#[doc = "Field `BLK51_EN` writer - When 1, Block51 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK51_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK51_EN_A, O>;
impl<'a, const O: u8> BLK51_EN_W<'a, O> {
    #[doc = "Block51 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk51_en_0(self) -> &'a mut W {
        self.variant(BLK51_EN_A::BLK51_EN_0)
    }
    #[doc = "Block51 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk51_en_1(self) -> &'a mut W {
        self.variant(BLK51_EN_A::BLK51_EN_1)
    }
}
#[doc = "Field `BLK52_EN` reader - When 1, Block52 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK52_EN_R = crate::BitReader<BLK52_EN_A>;
#[doc = "When 1, Block52 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK52_EN_A {
    #[doc = "0: Block52 of the SRAM is not retained in LPM3 or LPM4"]
    BLK52_EN_0 = 0,
    #[doc = "1: Block52 of the SRAM is retained in LPM3 and LPM4"]
    BLK52_EN_1 = 1,
}
impl From<BLK52_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK52_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK52_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK52_EN_A {
        match self.bits {
            false => BLK52_EN_A::BLK52_EN_0,
            true => BLK52_EN_A::BLK52_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK52_EN_0`"]
    #[inline(always)]
    pub fn is_blk52_en_0(&self) -> bool {
        *self == BLK52_EN_A::BLK52_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK52_EN_1`"]
    #[inline(always)]
    pub fn is_blk52_en_1(&self) -> bool {
        *self == BLK52_EN_A::BLK52_EN_1
    }
}
#[doc = "Field `BLK52_EN` writer - When 1, Block52 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK52_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK52_EN_A, O>;
impl<'a, const O: u8> BLK52_EN_W<'a, O> {
    #[doc = "Block52 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk52_en_0(self) -> &'a mut W {
        self.variant(BLK52_EN_A::BLK52_EN_0)
    }
    #[doc = "Block52 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk52_en_1(self) -> &'a mut W {
        self.variant(BLK52_EN_A::BLK52_EN_1)
    }
}
#[doc = "Field `BLK53_EN` reader - When 1, Block53 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK53_EN_R = crate::BitReader<BLK53_EN_A>;
#[doc = "When 1, Block53 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK53_EN_A {
    #[doc = "0: Block53 of the SRAM is not retained in LPM3 or LPM4"]
    BLK53_EN_0 = 0,
    #[doc = "1: Block53 of the SRAM is retained in LPM3 and LPM4"]
    BLK53_EN_1 = 1,
}
impl From<BLK53_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK53_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK53_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK53_EN_A {
        match self.bits {
            false => BLK53_EN_A::BLK53_EN_0,
            true => BLK53_EN_A::BLK53_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK53_EN_0`"]
    #[inline(always)]
    pub fn is_blk53_en_0(&self) -> bool {
        *self == BLK53_EN_A::BLK53_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK53_EN_1`"]
    #[inline(always)]
    pub fn is_blk53_en_1(&self) -> bool {
        *self == BLK53_EN_A::BLK53_EN_1
    }
}
#[doc = "Field `BLK53_EN` writer - When 1, Block53 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK53_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK53_EN_A, O>;
impl<'a, const O: u8> BLK53_EN_W<'a, O> {
    #[doc = "Block53 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk53_en_0(self) -> &'a mut W {
        self.variant(BLK53_EN_A::BLK53_EN_0)
    }
    #[doc = "Block53 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk53_en_1(self) -> &'a mut W {
        self.variant(BLK53_EN_A::BLK53_EN_1)
    }
}
#[doc = "Field `BLK54_EN` reader - When 1, Block54 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK54_EN_R = crate::BitReader<BLK54_EN_A>;
#[doc = "When 1, Block54 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK54_EN_A {
    #[doc = "0: Block54 of the SRAM is not retained in LPM3 or LPM4"]
    BLK54_EN_0 = 0,
    #[doc = "1: Block54 of the SRAM is retained in LPM3 and LPM4"]
    BLK54_EN_1 = 1,
}
impl From<BLK54_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK54_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK54_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK54_EN_A {
        match self.bits {
            false => BLK54_EN_A::BLK54_EN_0,
            true => BLK54_EN_A::BLK54_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK54_EN_0`"]
    #[inline(always)]
    pub fn is_blk54_en_0(&self) -> bool {
        *self == BLK54_EN_A::BLK54_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK54_EN_1`"]
    #[inline(always)]
    pub fn is_blk54_en_1(&self) -> bool {
        *self == BLK54_EN_A::BLK54_EN_1
    }
}
#[doc = "Field `BLK54_EN` writer - When 1, Block54 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK54_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK54_EN_A, O>;
impl<'a, const O: u8> BLK54_EN_W<'a, O> {
    #[doc = "Block54 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk54_en_0(self) -> &'a mut W {
        self.variant(BLK54_EN_A::BLK54_EN_0)
    }
    #[doc = "Block54 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk54_en_1(self) -> &'a mut W {
        self.variant(BLK54_EN_A::BLK54_EN_1)
    }
}
#[doc = "Field `BLK55_EN` reader - When 1, Block55 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK55_EN_R = crate::BitReader<BLK55_EN_A>;
#[doc = "When 1, Block55 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK55_EN_A {
    #[doc = "0: Block55 of the SRAM is not retained in LPM3 or LPM4"]
    BLK55_EN_0 = 0,
    #[doc = "1: Block55 of the SRAM is retained in LPM3 and LPM4"]
    BLK55_EN_1 = 1,
}
impl From<BLK55_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK55_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK55_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK55_EN_A {
        match self.bits {
            false => BLK55_EN_A::BLK55_EN_0,
            true => BLK55_EN_A::BLK55_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK55_EN_0`"]
    #[inline(always)]
    pub fn is_blk55_en_0(&self) -> bool {
        *self == BLK55_EN_A::BLK55_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK55_EN_1`"]
    #[inline(always)]
    pub fn is_blk55_en_1(&self) -> bool {
        *self == BLK55_EN_A::BLK55_EN_1
    }
}
#[doc = "Field `BLK55_EN` writer - When 1, Block55 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK55_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK55_EN_A, O>;
impl<'a, const O: u8> BLK55_EN_W<'a, O> {
    #[doc = "Block55 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk55_en_0(self) -> &'a mut W {
        self.variant(BLK55_EN_A::BLK55_EN_0)
    }
    #[doc = "Block55 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk55_en_1(self) -> &'a mut W {
        self.variant(BLK55_EN_A::BLK55_EN_1)
    }
}
#[doc = "Field `BLK56_EN` reader - When 1, Block56 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK56_EN_R = crate::BitReader<BLK56_EN_A>;
#[doc = "When 1, Block56 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK56_EN_A {
    #[doc = "0: Block56 of the SRAM is not retained in LPM3 or LPM4"]
    BLK56_EN_0 = 0,
    #[doc = "1: Block56 of the SRAM is retained in LPM3 and LPM4"]
    BLK56_EN_1 = 1,
}
impl From<BLK56_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK56_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK56_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK56_EN_A {
        match self.bits {
            false => BLK56_EN_A::BLK56_EN_0,
            true => BLK56_EN_A::BLK56_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK56_EN_0`"]
    #[inline(always)]
    pub fn is_blk56_en_0(&self) -> bool {
        *self == BLK56_EN_A::BLK56_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK56_EN_1`"]
    #[inline(always)]
    pub fn is_blk56_en_1(&self) -> bool {
        *self == BLK56_EN_A::BLK56_EN_1
    }
}
#[doc = "Field `BLK56_EN` writer - When 1, Block56 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK56_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK56_EN_A, O>;
impl<'a, const O: u8> BLK56_EN_W<'a, O> {
    #[doc = "Block56 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk56_en_0(self) -> &'a mut W {
        self.variant(BLK56_EN_A::BLK56_EN_0)
    }
    #[doc = "Block56 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk56_en_1(self) -> &'a mut W {
        self.variant(BLK56_EN_A::BLK56_EN_1)
    }
}
#[doc = "Field `BLK57_EN` reader - When 1, Block57 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK57_EN_R = crate::BitReader<BLK57_EN_A>;
#[doc = "When 1, Block57 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK57_EN_A {
    #[doc = "0: Block57 of the SRAM is not retained in LPM3 or LPM4"]
    BLK57_EN_0 = 0,
    #[doc = "1: Block57 of the SRAM is retained in LPM3 and LPM4"]
    BLK57_EN_1 = 1,
}
impl From<BLK57_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK57_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK57_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK57_EN_A {
        match self.bits {
            false => BLK57_EN_A::BLK57_EN_0,
            true => BLK57_EN_A::BLK57_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK57_EN_0`"]
    #[inline(always)]
    pub fn is_blk57_en_0(&self) -> bool {
        *self == BLK57_EN_A::BLK57_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK57_EN_1`"]
    #[inline(always)]
    pub fn is_blk57_en_1(&self) -> bool {
        *self == BLK57_EN_A::BLK57_EN_1
    }
}
#[doc = "Field `BLK57_EN` writer - When 1, Block57 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK57_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK57_EN_A, O>;
impl<'a, const O: u8> BLK57_EN_W<'a, O> {
    #[doc = "Block57 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk57_en_0(self) -> &'a mut W {
        self.variant(BLK57_EN_A::BLK57_EN_0)
    }
    #[doc = "Block57 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk57_en_1(self) -> &'a mut W {
        self.variant(BLK57_EN_A::BLK57_EN_1)
    }
}
#[doc = "Field `BLK58_EN` reader - When 1, Block58 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK58_EN_R = crate::BitReader<BLK58_EN_A>;
#[doc = "When 1, Block58 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK58_EN_A {
    #[doc = "0: Block58 of the SRAM is not retained in LPM3 or LPM4"]
    BLK58_EN_0 = 0,
    #[doc = "1: Block58 of the SRAM is retained in LPM3 and LPM4"]
    BLK58_EN_1 = 1,
}
impl From<BLK58_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK58_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK58_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK58_EN_A {
        match self.bits {
            false => BLK58_EN_A::BLK58_EN_0,
            true => BLK58_EN_A::BLK58_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK58_EN_0`"]
    #[inline(always)]
    pub fn is_blk58_en_0(&self) -> bool {
        *self == BLK58_EN_A::BLK58_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK58_EN_1`"]
    #[inline(always)]
    pub fn is_blk58_en_1(&self) -> bool {
        *self == BLK58_EN_A::BLK58_EN_1
    }
}
#[doc = "Field `BLK58_EN` writer - When 1, Block58 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK58_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK58_EN_A, O>;
impl<'a, const O: u8> BLK58_EN_W<'a, O> {
    #[doc = "Block58 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk58_en_0(self) -> &'a mut W {
        self.variant(BLK58_EN_A::BLK58_EN_0)
    }
    #[doc = "Block58 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk58_en_1(self) -> &'a mut W {
        self.variant(BLK58_EN_A::BLK58_EN_1)
    }
}
#[doc = "Field `BLK59_EN` reader - When 1, Block59 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK59_EN_R = crate::BitReader<BLK59_EN_A>;
#[doc = "When 1, Block59 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK59_EN_A {
    #[doc = "0: Block59 of the SRAM is not retained in LPM3 or LPM4"]
    BLK59_EN_0 = 0,
    #[doc = "1: Block59 of the SRAM is retained in LPM3 and LPM4"]
    BLK59_EN_1 = 1,
}
impl From<BLK59_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK59_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK59_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK59_EN_A {
        match self.bits {
            false => BLK59_EN_A::BLK59_EN_0,
            true => BLK59_EN_A::BLK59_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK59_EN_0`"]
    #[inline(always)]
    pub fn is_blk59_en_0(&self) -> bool {
        *self == BLK59_EN_A::BLK59_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK59_EN_1`"]
    #[inline(always)]
    pub fn is_blk59_en_1(&self) -> bool {
        *self == BLK59_EN_A::BLK59_EN_1
    }
}
#[doc = "Field `BLK59_EN` writer - When 1, Block59 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK59_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK59_EN_A, O>;
impl<'a, const O: u8> BLK59_EN_W<'a, O> {
    #[doc = "Block59 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk59_en_0(self) -> &'a mut W {
        self.variant(BLK59_EN_A::BLK59_EN_0)
    }
    #[doc = "Block59 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk59_en_1(self) -> &'a mut W {
        self.variant(BLK59_EN_A::BLK59_EN_1)
    }
}
#[doc = "Field `BLK60_EN` reader - When 1, Block60 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK60_EN_R = crate::BitReader<BLK60_EN_A>;
#[doc = "When 1, Block60 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK60_EN_A {
    #[doc = "0: Block60 of the SRAM is not retained in LPM3 or LPM4"]
    BLK60_EN_0 = 0,
    #[doc = "1: Block60 of the SRAM is retained in LPM3 and LPM4"]
    BLK60_EN_1 = 1,
}
impl From<BLK60_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK60_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK60_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK60_EN_A {
        match self.bits {
            false => BLK60_EN_A::BLK60_EN_0,
            true => BLK60_EN_A::BLK60_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK60_EN_0`"]
    #[inline(always)]
    pub fn is_blk60_en_0(&self) -> bool {
        *self == BLK60_EN_A::BLK60_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK60_EN_1`"]
    #[inline(always)]
    pub fn is_blk60_en_1(&self) -> bool {
        *self == BLK60_EN_A::BLK60_EN_1
    }
}
#[doc = "Field `BLK60_EN` writer - When 1, Block60 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK60_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK60_EN_A, O>;
impl<'a, const O: u8> BLK60_EN_W<'a, O> {
    #[doc = "Block60 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk60_en_0(self) -> &'a mut W {
        self.variant(BLK60_EN_A::BLK60_EN_0)
    }
    #[doc = "Block60 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk60_en_1(self) -> &'a mut W {
        self.variant(BLK60_EN_A::BLK60_EN_1)
    }
}
#[doc = "Field `BLK61_EN` reader - When 1, Block61 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK61_EN_R = crate::BitReader<BLK61_EN_A>;
#[doc = "When 1, Block61 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK61_EN_A {
    #[doc = "0: Block61 of the SRAM is not retained in LPM3 or LPM4"]
    BLK61_EN_0 = 0,
    #[doc = "1: Block61 of the SRAM is retained in LPM3 and LPM4"]
    BLK61_EN_1 = 1,
}
impl From<BLK61_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK61_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK61_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK61_EN_A {
        match self.bits {
            false => BLK61_EN_A::BLK61_EN_0,
            true => BLK61_EN_A::BLK61_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK61_EN_0`"]
    #[inline(always)]
    pub fn is_blk61_en_0(&self) -> bool {
        *self == BLK61_EN_A::BLK61_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK61_EN_1`"]
    #[inline(always)]
    pub fn is_blk61_en_1(&self) -> bool {
        *self == BLK61_EN_A::BLK61_EN_1
    }
}
#[doc = "Field `BLK61_EN` writer - When 1, Block61 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK61_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK61_EN_A, O>;
impl<'a, const O: u8> BLK61_EN_W<'a, O> {
    #[doc = "Block61 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk61_en_0(self) -> &'a mut W {
        self.variant(BLK61_EN_A::BLK61_EN_0)
    }
    #[doc = "Block61 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk61_en_1(self) -> &'a mut W {
        self.variant(BLK61_EN_A::BLK61_EN_1)
    }
}
#[doc = "Field `BLK62_EN` reader - When 1, Block62 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK62_EN_R = crate::BitReader<BLK62_EN_A>;
#[doc = "When 1, Block62 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK62_EN_A {
    #[doc = "0: Block62 of the SRAM is not retained in LPM3 or LPM4"]
    BLK62_EN_0 = 0,
    #[doc = "1: Block62 of the SRAM is retained in LPM3 and LPM4"]
    BLK62_EN_1 = 1,
}
impl From<BLK62_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK62_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK62_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK62_EN_A {
        match self.bits {
            false => BLK62_EN_A::BLK62_EN_0,
            true => BLK62_EN_A::BLK62_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK62_EN_0`"]
    #[inline(always)]
    pub fn is_blk62_en_0(&self) -> bool {
        *self == BLK62_EN_A::BLK62_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK62_EN_1`"]
    #[inline(always)]
    pub fn is_blk62_en_1(&self) -> bool {
        *self == BLK62_EN_A::BLK62_EN_1
    }
}
#[doc = "Field `BLK62_EN` writer - When 1, Block62 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK62_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK62_EN_A, O>;
impl<'a, const O: u8> BLK62_EN_W<'a, O> {
    #[doc = "Block62 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk62_en_0(self) -> &'a mut W {
        self.variant(BLK62_EN_A::BLK62_EN_0)
    }
    #[doc = "Block62 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk62_en_1(self) -> &'a mut W {
        self.variant(BLK62_EN_A::BLK62_EN_1)
    }
}
#[doc = "Field `BLK63_EN` reader - When 1, Block63 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK63_EN_R = crate::BitReader<BLK63_EN_A>;
#[doc = "When 1, Block63 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK63_EN_A {
    #[doc = "0: Block63 of the SRAM is not retained in LPM3 or LPM4"]
    BLK63_EN_0 = 0,
    #[doc = "1: Block63 of the SRAM is retained in LPM3 and LPM4"]
    BLK63_EN_1 = 1,
}
impl From<BLK63_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK63_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK63_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK63_EN_A {
        match self.bits {
            false => BLK63_EN_A::BLK63_EN_0,
            true => BLK63_EN_A::BLK63_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK63_EN_0`"]
    #[inline(always)]
    pub fn is_blk63_en_0(&self) -> bool {
        *self == BLK63_EN_A::BLK63_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK63_EN_1`"]
    #[inline(always)]
    pub fn is_blk63_en_1(&self) -> bool {
        *self == BLK63_EN_A::BLK63_EN_1
    }
}
#[doc = "Field `BLK63_EN` writer - When 1, Block63 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK63_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL1_SPEC, BLK63_EN_A, O>;
impl<'a, const O: u8> BLK63_EN_W<'a, O> {
    #[doc = "Block63 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk63_en_0(self) -> &'a mut W {
        self.variant(BLK63_EN_A::BLK63_EN_0)
    }
    #[doc = "Block63 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk63_en_1(self) -> &'a mut W {
        self.variant(BLK63_EN_A::BLK63_EN_1)
    }
}
impl R {
    #[doc = "Bit 0 - When 1, Block32 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk32_en(&self) -> BLK32_EN_R {
        BLK32_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, Block33 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk33_en(&self) -> BLK33_EN_R {
        BLK33_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When 1, Block34 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk34_en(&self) -> BLK34_EN_R {
        BLK34_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When 1, Block35 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk35_en(&self) -> BLK35_EN_R {
        BLK35_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When 1, Block36 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk36_en(&self) -> BLK36_EN_R {
        BLK36_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When 1, Block37 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk37_en(&self) -> BLK37_EN_R {
        BLK37_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When 1, Block38 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk38_en(&self) -> BLK38_EN_R {
        BLK38_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When 1, Block39 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk39_en(&self) -> BLK39_EN_R {
        BLK39_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - When 1, Block40 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk40_en(&self) -> BLK40_EN_R {
        BLK40_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When 1, Block41 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk41_en(&self) -> BLK41_EN_R {
        BLK41_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When 1, Block42 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk42_en(&self) -> BLK42_EN_R {
        BLK42_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When 1, Block43 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk43_en(&self) -> BLK43_EN_R {
        BLK43_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - When 1, Block44 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk44_en(&self) -> BLK44_EN_R {
        BLK44_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - When 1, Block45 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk45_en(&self) -> BLK45_EN_R {
        BLK45_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - When 1, Block46 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk46_en(&self) -> BLK46_EN_R {
        BLK46_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - When 1, Block47 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk47_en(&self) -> BLK47_EN_R {
        BLK47_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - When 1, Block48 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk48_en(&self) -> BLK48_EN_R {
        BLK48_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When 1, Block49 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk49_en(&self) -> BLK49_EN_R {
        BLK49_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - When 1, Block50 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk50_en(&self) -> BLK50_EN_R {
        BLK50_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - When 1, Block51 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk51_en(&self) -> BLK51_EN_R {
        BLK51_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - When 1, Block52 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk52_en(&self) -> BLK52_EN_R {
        BLK52_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - When 1, Block53 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk53_en(&self) -> BLK53_EN_R {
        BLK53_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - When 1, Block54 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk54_en(&self) -> BLK54_EN_R {
        BLK54_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - When 1, Block55 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk55_en(&self) -> BLK55_EN_R {
        BLK55_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - When 1, Block56 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk56_en(&self) -> BLK56_EN_R {
        BLK56_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - When 1, Block57 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk57_en(&self) -> BLK57_EN_R {
        BLK57_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - When 1, Block58 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk58_en(&self) -> BLK58_EN_R {
        BLK58_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - When 1, Block59 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk59_en(&self) -> BLK59_EN_R {
        BLK59_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - When 1, Block60 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk60_en(&self) -> BLK60_EN_R {
        BLK60_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - When 1, Block61 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk61_en(&self) -> BLK61_EN_R {
        BLK61_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - When 1, Block62 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk62_en(&self) -> BLK62_EN_R {
        BLK62_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When 1, Block63 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk63_en(&self) -> BLK63_EN_R {
        BLK63_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, Block32 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk32_en(&mut self) -> BLK32_EN_W<0> {
        BLK32_EN_W::new(self)
    }
    #[doc = "Bit 1 - When 1, Block33 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk33_en(&mut self) -> BLK33_EN_W<1> {
        BLK33_EN_W::new(self)
    }
    #[doc = "Bit 2 - When 1, Block34 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk34_en(&mut self) -> BLK34_EN_W<2> {
        BLK34_EN_W::new(self)
    }
    #[doc = "Bit 3 - When 1, Block35 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk35_en(&mut self) -> BLK35_EN_W<3> {
        BLK35_EN_W::new(self)
    }
    #[doc = "Bit 4 - When 1, Block36 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk36_en(&mut self) -> BLK36_EN_W<4> {
        BLK36_EN_W::new(self)
    }
    #[doc = "Bit 5 - When 1, Block37 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk37_en(&mut self) -> BLK37_EN_W<5> {
        BLK37_EN_W::new(self)
    }
    #[doc = "Bit 6 - When 1, Block38 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk38_en(&mut self) -> BLK38_EN_W<6> {
        BLK38_EN_W::new(self)
    }
    #[doc = "Bit 7 - When 1, Block39 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk39_en(&mut self) -> BLK39_EN_W<7> {
        BLK39_EN_W::new(self)
    }
    #[doc = "Bit 8 - When 1, Block40 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk40_en(&mut self) -> BLK40_EN_W<8> {
        BLK40_EN_W::new(self)
    }
    #[doc = "Bit 9 - When 1, Block41 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk41_en(&mut self) -> BLK41_EN_W<9> {
        BLK41_EN_W::new(self)
    }
    #[doc = "Bit 10 - When 1, Block42 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk42_en(&mut self) -> BLK42_EN_W<10> {
        BLK42_EN_W::new(self)
    }
    #[doc = "Bit 11 - When 1, Block43 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk43_en(&mut self) -> BLK43_EN_W<11> {
        BLK43_EN_W::new(self)
    }
    #[doc = "Bit 12 - When 1, Block44 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk44_en(&mut self) -> BLK44_EN_W<12> {
        BLK44_EN_W::new(self)
    }
    #[doc = "Bit 13 - When 1, Block45 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk45_en(&mut self) -> BLK45_EN_W<13> {
        BLK45_EN_W::new(self)
    }
    #[doc = "Bit 14 - When 1, Block46 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk46_en(&mut self) -> BLK46_EN_W<14> {
        BLK46_EN_W::new(self)
    }
    #[doc = "Bit 15 - When 1, Block47 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk47_en(&mut self) -> BLK47_EN_W<15> {
        BLK47_EN_W::new(self)
    }
    #[doc = "Bit 16 - When 1, Block48 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk48_en(&mut self) -> BLK48_EN_W<16> {
        BLK48_EN_W::new(self)
    }
    #[doc = "Bit 17 - When 1, Block49 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk49_en(&mut self) -> BLK49_EN_W<17> {
        BLK49_EN_W::new(self)
    }
    #[doc = "Bit 18 - When 1, Block50 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk50_en(&mut self) -> BLK50_EN_W<18> {
        BLK50_EN_W::new(self)
    }
    #[doc = "Bit 19 - When 1, Block51 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk51_en(&mut self) -> BLK51_EN_W<19> {
        BLK51_EN_W::new(self)
    }
    #[doc = "Bit 20 - When 1, Block52 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk52_en(&mut self) -> BLK52_EN_W<20> {
        BLK52_EN_W::new(self)
    }
    #[doc = "Bit 21 - When 1, Block53 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk53_en(&mut self) -> BLK53_EN_W<21> {
        BLK53_EN_W::new(self)
    }
    #[doc = "Bit 22 - When 1, Block54 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk54_en(&mut self) -> BLK54_EN_W<22> {
        BLK54_EN_W::new(self)
    }
    #[doc = "Bit 23 - When 1, Block55 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk55_en(&mut self) -> BLK55_EN_W<23> {
        BLK55_EN_W::new(self)
    }
    #[doc = "Bit 24 - When 1, Block56 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk56_en(&mut self) -> BLK56_EN_W<24> {
        BLK56_EN_W::new(self)
    }
    #[doc = "Bit 25 - When 1, Block57 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk57_en(&mut self) -> BLK57_EN_W<25> {
        BLK57_EN_W::new(self)
    }
    #[doc = "Bit 26 - When 1, Block58 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk58_en(&mut self) -> BLK58_EN_W<26> {
        BLK58_EN_W::new(self)
    }
    #[doc = "Bit 27 - When 1, Block59 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk59_en(&mut self) -> BLK59_EN_W<27> {
        BLK59_EN_W::new(self)
    }
    #[doc = "Bit 28 - When 1, Block60 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk60_en(&mut self) -> BLK60_EN_W<28> {
        BLK60_EN_W::new(self)
    }
    #[doc = "Bit 29 - When 1, Block61 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk61_en(&mut self) -> BLK61_EN_W<29> {
        BLK61_EN_W::new(self)
    }
    #[doc = "Bit 30 - When 1, Block62 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk62_en(&mut self) -> BLK62_EN_W<30> {
        BLK62_EN_W::new(self)
    }
    #[doc = "Bit 31 - When 1, Block63 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk63_en(&mut self) -> BLK63_EN_W<31> {
        BLK63_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM Block Retention Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_sram_blkret_ctl1](index.html) module"]
pub struct SYS_SRAM_BLKRET_CTL1_SPEC;
impl crate::RegisterSpec for SYS_SRAM_BLKRET_CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_sram_blkret_ctl1::R](R) reader structure"]
impl crate::Readable for SYS_SRAM_BLKRET_CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_sram_blkret_ctl1::W](W) writer structure"]
impl crate::Writable for SYS_SRAM_BLKRET_CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_SRAM_BLKRET_CTL1 to value 0xffff_ffff"]
impl crate::Resettable for SYS_SRAM_BLKRET_CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
