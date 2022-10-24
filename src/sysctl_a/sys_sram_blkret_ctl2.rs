#[doc = "Register `SYS_SRAM_BLKRET_CTL2` reader"]
pub struct R(crate::R<SYS_SRAM_BLKRET_CTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_SRAM_BLKRET_CTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_SRAM_BLKRET_CTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_SRAM_BLKRET_CTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_SRAM_BLKRET_CTL2` writer"]
pub struct W(crate::W<SYS_SRAM_BLKRET_CTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_SRAM_BLKRET_CTL2_SPEC>;
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
impl From<crate::W<SYS_SRAM_BLKRET_CTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_SRAM_BLKRET_CTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLK64_EN` reader - When 1, Block64 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK64_EN_R = crate::BitReader<BLK64_EN_A>;
#[doc = "When 1, Block64 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK64_EN_A {
    #[doc = "0: Block64 of the SRAM is not retained in LPM3 or LPM4"]
    BLK64_EN_0 = 0,
    #[doc = "1: Block64 of the SRAM is retained in LPM3 and LPM4"]
    BLK64_EN_1 = 1,
}
impl From<BLK64_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK64_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK64_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK64_EN_A {
        match self.bits {
            false => BLK64_EN_A::BLK64_EN_0,
            true => BLK64_EN_A::BLK64_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK64_EN_0`"]
    #[inline(always)]
    pub fn is_blk64_en_0(&self) -> bool {
        *self == BLK64_EN_A::BLK64_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK64_EN_1`"]
    #[inline(always)]
    pub fn is_blk64_en_1(&self) -> bool {
        *self == BLK64_EN_A::BLK64_EN_1
    }
}
#[doc = "Field `BLK64_EN` writer - When 1, Block64 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK64_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK64_EN_A, O>;
impl<'a, const O: u8> BLK64_EN_W<'a, O> {
    #[doc = "Block64 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk64_en_0(self) -> &'a mut W {
        self.variant(BLK64_EN_A::BLK64_EN_0)
    }
    #[doc = "Block64 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk64_en_1(self) -> &'a mut W {
        self.variant(BLK64_EN_A::BLK64_EN_1)
    }
}
#[doc = "Field `BLK65_EN` reader - When 1, Block65 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK65_EN_R = crate::BitReader<BLK65_EN_A>;
#[doc = "When 1, Block65 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK65_EN_A {
    #[doc = "0: Block65 of the SRAM is not retained in LPM3 or LPM4"]
    BLK65_EN_0 = 0,
    #[doc = "1: Block65 of the SRAM is retained in LPM3 and LPM4"]
    BLK65_EN_1 = 1,
}
impl From<BLK65_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK65_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK65_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK65_EN_A {
        match self.bits {
            false => BLK65_EN_A::BLK65_EN_0,
            true => BLK65_EN_A::BLK65_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK65_EN_0`"]
    #[inline(always)]
    pub fn is_blk65_en_0(&self) -> bool {
        *self == BLK65_EN_A::BLK65_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK65_EN_1`"]
    #[inline(always)]
    pub fn is_blk65_en_1(&self) -> bool {
        *self == BLK65_EN_A::BLK65_EN_1
    }
}
#[doc = "Field `BLK65_EN` writer - When 1, Block65 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK65_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK65_EN_A, O>;
impl<'a, const O: u8> BLK65_EN_W<'a, O> {
    #[doc = "Block65 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk65_en_0(self) -> &'a mut W {
        self.variant(BLK65_EN_A::BLK65_EN_0)
    }
    #[doc = "Block65 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk65_en_1(self) -> &'a mut W {
        self.variant(BLK65_EN_A::BLK65_EN_1)
    }
}
#[doc = "Field `BLK66_EN` reader - When 1, Block66 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK66_EN_R = crate::BitReader<BLK66_EN_A>;
#[doc = "When 1, Block66 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK66_EN_A {
    #[doc = "0: Block66 of the SRAM is not retained in LPM3 or LPM4"]
    BLK66_EN_0 = 0,
    #[doc = "1: Block66 of the SRAM is retained in LPM3 and LPM4"]
    BLK66_EN_1 = 1,
}
impl From<BLK66_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK66_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK66_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK66_EN_A {
        match self.bits {
            false => BLK66_EN_A::BLK66_EN_0,
            true => BLK66_EN_A::BLK66_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK66_EN_0`"]
    #[inline(always)]
    pub fn is_blk66_en_0(&self) -> bool {
        *self == BLK66_EN_A::BLK66_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK66_EN_1`"]
    #[inline(always)]
    pub fn is_blk66_en_1(&self) -> bool {
        *self == BLK66_EN_A::BLK66_EN_1
    }
}
#[doc = "Field `BLK66_EN` writer - When 1, Block66 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK66_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK66_EN_A, O>;
impl<'a, const O: u8> BLK66_EN_W<'a, O> {
    #[doc = "Block66 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk66_en_0(self) -> &'a mut W {
        self.variant(BLK66_EN_A::BLK66_EN_0)
    }
    #[doc = "Block66 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk66_en_1(self) -> &'a mut W {
        self.variant(BLK66_EN_A::BLK66_EN_1)
    }
}
#[doc = "Field `BLK67_EN` reader - When 1, Block67 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK67_EN_R = crate::BitReader<BLK67_EN_A>;
#[doc = "When 1, Block67 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK67_EN_A {
    #[doc = "0: Block67 of the SRAM is not retained in LPM3 or LPM4"]
    BLK67_EN_0 = 0,
    #[doc = "1: Block67 of the SRAM is retained in LPM3 and LPM4"]
    BLK67_EN_1 = 1,
}
impl From<BLK67_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK67_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK67_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK67_EN_A {
        match self.bits {
            false => BLK67_EN_A::BLK67_EN_0,
            true => BLK67_EN_A::BLK67_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK67_EN_0`"]
    #[inline(always)]
    pub fn is_blk67_en_0(&self) -> bool {
        *self == BLK67_EN_A::BLK67_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK67_EN_1`"]
    #[inline(always)]
    pub fn is_blk67_en_1(&self) -> bool {
        *self == BLK67_EN_A::BLK67_EN_1
    }
}
#[doc = "Field `BLK67_EN` writer - When 1, Block67 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK67_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK67_EN_A, O>;
impl<'a, const O: u8> BLK67_EN_W<'a, O> {
    #[doc = "Block67 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk67_en_0(self) -> &'a mut W {
        self.variant(BLK67_EN_A::BLK67_EN_0)
    }
    #[doc = "Block67 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk67_en_1(self) -> &'a mut W {
        self.variant(BLK67_EN_A::BLK67_EN_1)
    }
}
#[doc = "Field `BLK68_EN` reader - When 1, Block68 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK68_EN_R = crate::BitReader<BLK68_EN_A>;
#[doc = "When 1, Block68 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK68_EN_A {
    #[doc = "0: Block68 of the SRAM is not retained in LPM3 or LPM4"]
    BLK68_EN_0 = 0,
    #[doc = "1: Block68 of the SRAM is retained in LPM3 and LPM4"]
    BLK68_EN_1 = 1,
}
impl From<BLK68_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK68_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK68_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK68_EN_A {
        match self.bits {
            false => BLK68_EN_A::BLK68_EN_0,
            true => BLK68_EN_A::BLK68_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK68_EN_0`"]
    #[inline(always)]
    pub fn is_blk68_en_0(&self) -> bool {
        *self == BLK68_EN_A::BLK68_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK68_EN_1`"]
    #[inline(always)]
    pub fn is_blk68_en_1(&self) -> bool {
        *self == BLK68_EN_A::BLK68_EN_1
    }
}
#[doc = "Field `BLK68_EN` writer - When 1, Block68 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK68_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK68_EN_A, O>;
impl<'a, const O: u8> BLK68_EN_W<'a, O> {
    #[doc = "Block68 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk68_en_0(self) -> &'a mut W {
        self.variant(BLK68_EN_A::BLK68_EN_0)
    }
    #[doc = "Block68 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk68_en_1(self) -> &'a mut W {
        self.variant(BLK68_EN_A::BLK68_EN_1)
    }
}
#[doc = "Field `BLK69_EN` reader - When 1, Block69 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK69_EN_R = crate::BitReader<BLK69_EN_A>;
#[doc = "When 1, Block69 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK69_EN_A {
    #[doc = "0: Block69 of the SRAM is not retained in LPM3 or LPM4"]
    BLK69_EN_0 = 0,
    #[doc = "1: Block69 of the SRAM is retained in LPM3 and LPM4"]
    BLK69_EN_1 = 1,
}
impl From<BLK69_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK69_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK69_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK69_EN_A {
        match self.bits {
            false => BLK69_EN_A::BLK69_EN_0,
            true => BLK69_EN_A::BLK69_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK69_EN_0`"]
    #[inline(always)]
    pub fn is_blk69_en_0(&self) -> bool {
        *self == BLK69_EN_A::BLK69_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK69_EN_1`"]
    #[inline(always)]
    pub fn is_blk69_en_1(&self) -> bool {
        *self == BLK69_EN_A::BLK69_EN_1
    }
}
#[doc = "Field `BLK69_EN` writer - When 1, Block69 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK69_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK69_EN_A, O>;
impl<'a, const O: u8> BLK69_EN_W<'a, O> {
    #[doc = "Block69 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk69_en_0(self) -> &'a mut W {
        self.variant(BLK69_EN_A::BLK69_EN_0)
    }
    #[doc = "Block69 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk69_en_1(self) -> &'a mut W {
        self.variant(BLK69_EN_A::BLK69_EN_1)
    }
}
#[doc = "Field `BLK70_EN` reader - When 1, Block70 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK70_EN_R = crate::BitReader<BLK70_EN_A>;
#[doc = "When 1, Block70 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK70_EN_A {
    #[doc = "0: Block70 of the SRAM is not retained in LPM3 or LPM4"]
    BLK70_EN_0 = 0,
    #[doc = "1: Block70 of the SRAM is retained in LPM3 and LPM4"]
    BLK70_EN_1 = 1,
}
impl From<BLK70_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK70_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK70_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK70_EN_A {
        match self.bits {
            false => BLK70_EN_A::BLK70_EN_0,
            true => BLK70_EN_A::BLK70_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK70_EN_0`"]
    #[inline(always)]
    pub fn is_blk70_en_0(&self) -> bool {
        *self == BLK70_EN_A::BLK70_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK70_EN_1`"]
    #[inline(always)]
    pub fn is_blk70_en_1(&self) -> bool {
        *self == BLK70_EN_A::BLK70_EN_1
    }
}
#[doc = "Field `BLK70_EN` writer - When 1, Block70 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK70_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK70_EN_A, O>;
impl<'a, const O: u8> BLK70_EN_W<'a, O> {
    #[doc = "Block70 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk70_en_0(self) -> &'a mut W {
        self.variant(BLK70_EN_A::BLK70_EN_0)
    }
    #[doc = "Block70 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk70_en_1(self) -> &'a mut W {
        self.variant(BLK70_EN_A::BLK70_EN_1)
    }
}
#[doc = "Field `BLK71_EN` reader - When 1, Block71 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK71_EN_R = crate::BitReader<BLK71_EN_A>;
#[doc = "When 1, Block71 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK71_EN_A {
    #[doc = "0: Block71 of the SRAM is not retained in LPM3 or LPM4"]
    BLK71_EN_0 = 0,
    #[doc = "1: Block71 of the SRAM is retained in LPM3 and LPM4"]
    BLK71_EN_1 = 1,
}
impl From<BLK71_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK71_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK71_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK71_EN_A {
        match self.bits {
            false => BLK71_EN_A::BLK71_EN_0,
            true => BLK71_EN_A::BLK71_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK71_EN_0`"]
    #[inline(always)]
    pub fn is_blk71_en_0(&self) -> bool {
        *self == BLK71_EN_A::BLK71_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK71_EN_1`"]
    #[inline(always)]
    pub fn is_blk71_en_1(&self) -> bool {
        *self == BLK71_EN_A::BLK71_EN_1
    }
}
#[doc = "Field `BLK71_EN` writer - When 1, Block71 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK71_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK71_EN_A, O>;
impl<'a, const O: u8> BLK71_EN_W<'a, O> {
    #[doc = "Block71 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk71_en_0(self) -> &'a mut W {
        self.variant(BLK71_EN_A::BLK71_EN_0)
    }
    #[doc = "Block71 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk71_en_1(self) -> &'a mut W {
        self.variant(BLK71_EN_A::BLK71_EN_1)
    }
}
#[doc = "Field `BLK72_EN` reader - When 1, Block72 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK72_EN_R = crate::BitReader<BLK72_EN_A>;
#[doc = "When 1, Block72 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK72_EN_A {
    #[doc = "0: Block72 of the SRAM is not retained in LPM3 or LPM4"]
    BLK72_EN_0 = 0,
    #[doc = "1: Block72 of the SRAM is retained in LPM3 and LPM4"]
    BLK72_EN_1 = 1,
}
impl From<BLK72_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK72_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK72_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK72_EN_A {
        match self.bits {
            false => BLK72_EN_A::BLK72_EN_0,
            true => BLK72_EN_A::BLK72_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK72_EN_0`"]
    #[inline(always)]
    pub fn is_blk72_en_0(&self) -> bool {
        *self == BLK72_EN_A::BLK72_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK72_EN_1`"]
    #[inline(always)]
    pub fn is_blk72_en_1(&self) -> bool {
        *self == BLK72_EN_A::BLK72_EN_1
    }
}
#[doc = "Field `BLK72_EN` writer - When 1, Block72 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK72_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK72_EN_A, O>;
impl<'a, const O: u8> BLK72_EN_W<'a, O> {
    #[doc = "Block72 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk72_en_0(self) -> &'a mut W {
        self.variant(BLK72_EN_A::BLK72_EN_0)
    }
    #[doc = "Block72 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk72_en_1(self) -> &'a mut W {
        self.variant(BLK72_EN_A::BLK72_EN_1)
    }
}
#[doc = "Field `BLK73_EN` reader - When 1, Block73 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK73_EN_R = crate::BitReader<BLK73_EN_A>;
#[doc = "When 1, Block73 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK73_EN_A {
    #[doc = "0: Block73 of the SRAM is not retained in LPM3 or LPM4"]
    BLK73_EN_0 = 0,
    #[doc = "1: Block73 of the SRAM is retained in LPM3 and LPM4"]
    BLK73_EN_1 = 1,
}
impl From<BLK73_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK73_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK73_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK73_EN_A {
        match self.bits {
            false => BLK73_EN_A::BLK73_EN_0,
            true => BLK73_EN_A::BLK73_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK73_EN_0`"]
    #[inline(always)]
    pub fn is_blk73_en_0(&self) -> bool {
        *self == BLK73_EN_A::BLK73_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK73_EN_1`"]
    #[inline(always)]
    pub fn is_blk73_en_1(&self) -> bool {
        *self == BLK73_EN_A::BLK73_EN_1
    }
}
#[doc = "Field `BLK73_EN` writer - When 1, Block73 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK73_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK73_EN_A, O>;
impl<'a, const O: u8> BLK73_EN_W<'a, O> {
    #[doc = "Block73 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk73_en_0(self) -> &'a mut W {
        self.variant(BLK73_EN_A::BLK73_EN_0)
    }
    #[doc = "Block73 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk73_en_1(self) -> &'a mut W {
        self.variant(BLK73_EN_A::BLK73_EN_1)
    }
}
#[doc = "Field `BLK74_EN` reader - When 1, Block74 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK74_EN_R = crate::BitReader<BLK74_EN_A>;
#[doc = "When 1, Block74 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK74_EN_A {
    #[doc = "0: Block74 of the SRAM is not retained in LPM3 or LPM4"]
    BLK74_EN_0 = 0,
    #[doc = "1: Block74 of the SRAM is retained in LPM3 and LPM4"]
    BLK74_EN_1 = 1,
}
impl From<BLK74_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK74_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK74_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK74_EN_A {
        match self.bits {
            false => BLK74_EN_A::BLK74_EN_0,
            true => BLK74_EN_A::BLK74_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK74_EN_0`"]
    #[inline(always)]
    pub fn is_blk74_en_0(&self) -> bool {
        *self == BLK74_EN_A::BLK74_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK74_EN_1`"]
    #[inline(always)]
    pub fn is_blk74_en_1(&self) -> bool {
        *self == BLK74_EN_A::BLK74_EN_1
    }
}
#[doc = "Field `BLK74_EN` writer - When 1, Block74 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK74_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK74_EN_A, O>;
impl<'a, const O: u8> BLK74_EN_W<'a, O> {
    #[doc = "Block74 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk74_en_0(self) -> &'a mut W {
        self.variant(BLK74_EN_A::BLK74_EN_0)
    }
    #[doc = "Block74 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk74_en_1(self) -> &'a mut W {
        self.variant(BLK74_EN_A::BLK74_EN_1)
    }
}
#[doc = "Field `BLK75_EN` reader - When 1, Block75 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK75_EN_R = crate::BitReader<BLK75_EN_A>;
#[doc = "When 1, Block75 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK75_EN_A {
    #[doc = "0: Block75 of the SRAM is not retained in LPM3 or LPM4"]
    BLK75_EN_0 = 0,
    #[doc = "1: Block75 of the SRAM is retained in LPM3 and LPM4"]
    BLK75_EN_1 = 1,
}
impl From<BLK75_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK75_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK75_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK75_EN_A {
        match self.bits {
            false => BLK75_EN_A::BLK75_EN_0,
            true => BLK75_EN_A::BLK75_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK75_EN_0`"]
    #[inline(always)]
    pub fn is_blk75_en_0(&self) -> bool {
        *self == BLK75_EN_A::BLK75_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK75_EN_1`"]
    #[inline(always)]
    pub fn is_blk75_en_1(&self) -> bool {
        *self == BLK75_EN_A::BLK75_EN_1
    }
}
#[doc = "Field `BLK75_EN` writer - When 1, Block75 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK75_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK75_EN_A, O>;
impl<'a, const O: u8> BLK75_EN_W<'a, O> {
    #[doc = "Block75 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk75_en_0(self) -> &'a mut W {
        self.variant(BLK75_EN_A::BLK75_EN_0)
    }
    #[doc = "Block75 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk75_en_1(self) -> &'a mut W {
        self.variant(BLK75_EN_A::BLK75_EN_1)
    }
}
#[doc = "Field `BLK76_EN` reader - When 1, Block76 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK76_EN_R = crate::BitReader<BLK76_EN_A>;
#[doc = "When 1, Block76 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK76_EN_A {
    #[doc = "0: Block76 of the SRAM is not retained in LPM3 or LPM4"]
    BLK76_EN_0 = 0,
    #[doc = "1: Block76 of the SRAM is retained in LPM3 and LPM4"]
    BLK76_EN_1 = 1,
}
impl From<BLK76_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK76_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK76_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK76_EN_A {
        match self.bits {
            false => BLK76_EN_A::BLK76_EN_0,
            true => BLK76_EN_A::BLK76_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK76_EN_0`"]
    #[inline(always)]
    pub fn is_blk76_en_0(&self) -> bool {
        *self == BLK76_EN_A::BLK76_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK76_EN_1`"]
    #[inline(always)]
    pub fn is_blk76_en_1(&self) -> bool {
        *self == BLK76_EN_A::BLK76_EN_1
    }
}
#[doc = "Field `BLK76_EN` writer - When 1, Block76 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK76_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK76_EN_A, O>;
impl<'a, const O: u8> BLK76_EN_W<'a, O> {
    #[doc = "Block76 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk76_en_0(self) -> &'a mut W {
        self.variant(BLK76_EN_A::BLK76_EN_0)
    }
    #[doc = "Block76 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk76_en_1(self) -> &'a mut W {
        self.variant(BLK76_EN_A::BLK76_EN_1)
    }
}
#[doc = "Field `BLK77_EN` reader - When 1, Block77 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK77_EN_R = crate::BitReader<BLK77_EN_A>;
#[doc = "When 1, Block77 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK77_EN_A {
    #[doc = "0: Block77 of the SRAM is not retained in LPM3 or LPM4"]
    BLK77_EN_0 = 0,
    #[doc = "1: Block77 of the SRAM is retained in LPM3 and LPM4"]
    BLK77_EN_1 = 1,
}
impl From<BLK77_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK77_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK77_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK77_EN_A {
        match self.bits {
            false => BLK77_EN_A::BLK77_EN_0,
            true => BLK77_EN_A::BLK77_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK77_EN_0`"]
    #[inline(always)]
    pub fn is_blk77_en_0(&self) -> bool {
        *self == BLK77_EN_A::BLK77_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK77_EN_1`"]
    #[inline(always)]
    pub fn is_blk77_en_1(&self) -> bool {
        *self == BLK77_EN_A::BLK77_EN_1
    }
}
#[doc = "Field `BLK77_EN` writer - When 1, Block77 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK77_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK77_EN_A, O>;
impl<'a, const O: u8> BLK77_EN_W<'a, O> {
    #[doc = "Block77 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk77_en_0(self) -> &'a mut W {
        self.variant(BLK77_EN_A::BLK77_EN_0)
    }
    #[doc = "Block77 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk77_en_1(self) -> &'a mut W {
        self.variant(BLK77_EN_A::BLK77_EN_1)
    }
}
#[doc = "Field `BLK78_EN` reader - When 1, Block78 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK78_EN_R = crate::BitReader<BLK78_EN_A>;
#[doc = "When 1, Block78 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK78_EN_A {
    #[doc = "0: Block78 of the SRAM is not retained in LPM3 or LPM4"]
    BLK78_EN_0 = 0,
    #[doc = "1: Block78 of the SRAM is retained in LPM3 and LPM4"]
    BLK78_EN_1 = 1,
}
impl From<BLK78_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK78_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK78_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK78_EN_A {
        match self.bits {
            false => BLK78_EN_A::BLK78_EN_0,
            true => BLK78_EN_A::BLK78_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK78_EN_0`"]
    #[inline(always)]
    pub fn is_blk78_en_0(&self) -> bool {
        *self == BLK78_EN_A::BLK78_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK78_EN_1`"]
    #[inline(always)]
    pub fn is_blk78_en_1(&self) -> bool {
        *self == BLK78_EN_A::BLK78_EN_1
    }
}
#[doc = "Field `BLK78_EN` writer - When 1, Block78 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK78_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK78_EN_A, O>;
impl<'a, const O: u8> BLK78_EN_W<'a, O> {
    #[doc = "Block78 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk78_en_0(self) -> &'a mut W {
        self.variant(BLK78_EN_A::BLK78_EN_0)
    }
    #[doc = "Block78 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk78_en_1(self) -> &'a mut W {
        self.variant(BLK78_EN_A::BLK78_EN_1)
    }
}
#[doc = "Field `BLK79_EN` reader - When 1, Block79 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK79_EN_R = crate::BitReader<BLK79_EN_A>;
#[doc = "When 1, Block79 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK79_EN_A {
    #[doc = "0: Block79 of the SRAM is not retained in LPM3 or LPM4"]
    BLK79_EN_0 = 0,
    #[doc = "1: Block79 of the SRAM is retained in LPM3 and LPM4"]
    BLK79_EN_1 = 1,
}
impl From<BLK79_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK79_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK79_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK79_EN_A {
        match self.bits {
            false => BLK79_EN_A::BLK79_EN_0,
            true => BLK79_EN_A::BLK79_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK79_EN_0`"]
    #[inline(always)]
    pub fn is_blk79_en_0(&self) -> bool {
        *self == BLK79_EN_A::BLK79_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK79_EN_1`"]
    #[inline(always)]
    pub fn is_blk79_en_1(&self) -> bool {
        *self == BLK79_EN_A::BLK79_EN_1
    }
}
#[doc = "Field `BLK79_EN` writer - When 1, Block79 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK79_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK79_EN_A, O>;
impl<'a, const O: u8> BLK79_EN_W<'a, O> {
    #[doc = "Block79 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk79_en_0(self) -> &'a mut W {
        self.variant(BLK79_EN_A::BLK79_EN_0)
    }
    #[doc = "Block79 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk79_en_1(self) -> &'a mut W {
        self.variant(BLK79_EN_A::BLK79_EN_1)
    }
}
#[doc = "Field `BLK80_EN` reader - When 1, Block80 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK80_EN_R = crate::BitReader<BLK80_EN_A>;
#[doc = "When 1, Block80 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK80_EN_A {
    #[doc = "0: Block80 of the SRAM is not retained in LPM3 or LPM4"]
    BLK80_EN_0 = 0,
    #[doc = "1: Block80 of the SRAM is retained in LPM3 and LPM4"]
    BLK80_EN_1 = 1,
}
impl From<BLK80_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK80_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK80_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK80_EN_A {
        match self.bits {
            false => BLK80_EN_A::BLK80_EN_0,
            true => BLK80_EN_A::BLK80_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK80_EN_0`"]
    #[inline(always)]
    pub fn is_blk80_en_0(&self) -> bool {
        *self == BLK80_EN_A::BLK80_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK80_EN_1`"]
    #[inline(always)]
    pub fn is_blk80_en_1(&self) -> bool {
        *self == BLK80_EN_A::BLK80_EN_1
    }
}
#[doc = "Field `BLK80_EN` writer - When 1, Block80 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK80_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK80_EN_A, O>;
impl<'a, const O: u8> BLK80_EN_W<'a, O> {
    #[doc = "Block80 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk80_en_0(self) -> &'a mut W {
        self.variant(BLK80_EN_A::BLK80_EN_0)
    }
    #[doc = "Block80 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk80_en_1(self) -> &'a mut W {
        self.variant(BLK80_EN_A::BLK80_EN_1)
    }
}
#[doc = "Field `BLK81_EN` reader - When 1, Block81 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK81_EN_R = crate::BitReader<BLK81_EN_A>;
#[doc = "When 1, Block81 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK81_EN_A {
    #[doc = "0: Block81 of the SRAM is not retained in LPM3 or LPM4"]
    BLK81_EN_0 = 0,
    #[doc = "1: Block81 of the SRAM is retained in LPM3 and LPM4"]
    BLK81_EN_1 = 1,
}
impl From<BLK81_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK81_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK81_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK81_EN_A {
        match self.bits {
            false => BLK81_EN_A::BLK81_EN_0,
            true => BLK81_EN_A::BLK81_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK81_EN_0`"]
    #[inline(always)]
    pub fn is_blk81_en_0(&self) -> bool {
        *self == BLK81_EN_A::BLK81_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK81_EN_1`"]
    #[inline(always)]
    pub fn is_blk81_en_1(&self) -> bool {
        *self == BLK81_EN_A::BLK81_EN_1
    }
}
#[doc = "Field `BLK81_EN` writer - When 1, Block81 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK81_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK81_EN_A, O>;
impl<'a, const O: u8> BLK81_EN_W<'a, O> {
    #[doc = "Block81 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk81_en_0(self) -> &'a mut W {
        self.variant(BLK81_EN_A::BLK81_EN_0)
    }
    #[doc = "Block81 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk81_en_1(self) -> &'a mut W {
        self.variant(BLK81_EN_A::BLK81_EN_1)
    }
}
#[doc = "Field `BLK82_EN` reader - When 1, Block82 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK82_EN_R = crate::BitReader<BLK82_EN_A>;
#[doc = "When 1, Block82 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK82_EN_A {
    #[doc = "0: Block82 of the SRAM is not retained in LPM3 or LPM4"]
    BLK82_EN_0 = 0,
    #[doc = "1: Block82 of the SRAM is retained in LPM3 and LPM4"]
    BLK82_EN_1 = 1,
}
impl From<BLK82_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK82_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK82_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK82_EN_A {
        match self.bits {
            false => BLK82_EN_A::BLK82_EN_0,
            true => BLK82_EN_A::BLK82_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK82_EN_0`"]
    #[inline(always)]
    pub fn is_blk82_en_0(&self) -> bool {
        *self == BLK82_EN_A::BLK82_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK82_EN_1`"]
    #[inline(always)]
    pub fn is_blk82_en_1(&self) -> bool {
        *self == BLK82_EN_A::BLK82_EN_1
    }
}
#[doc = "Field `BLK82_EN` writer - When 1, Block82 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK82_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK82_EN_A, O>;
impl<'a, const O: u8> BLK82_EN_W<'a, O> {
    #[doc = "Block82 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk82_en_0(self) -> &'a mut W {
        self.variant(BLK82_EN_A::BLK82_EN_0)
    }
    #[doc = "Block82 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk82_en_1(self) -> &'a mut W {
        self.variant(BLK82_EN_A::BLK82_EN_1)
    }
}
#[doc = "Field `BLK83_EN` reader - When 1, Block83 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK83_EN_R = crate::BitReader<BLK83_EN_A>;
#[doc = "When 1, Block83 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK83_EN_A {
    #[doc = "0: Block83 of the SRAM is not retained in LPM3 or LPM4"]
    BLK83_EN_0 = 0,
    #[doc = "1: Block83 of the SRAM is retained in LPM3 and LPM4"]
    BLK83_EN_1 = 1,
}
impl From<BLK83_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK83_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK83_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK83_EN_A {
        match self.bits {
            false => BLK83_EN_A::BLK83_EN_0,
            true => BLK83_EN_A::BLK83_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK83_EN_0`"]
    #[inline(always)]
    pub fn is_blk83_en_0(&self) -> bool {
        *self == BLK83_EN_A::BLK83_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK83_EN_1`"]
    #[inline(always)]
    pub fn is_blk83_en_1(&self) -> bool {
        *self == BLK83_EN_A::BLK83_EN_1
    }
}
#[doc = "Field `BLK83_EN` writer - When 1, Block83 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK83_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK83_EN_A, O>;
impl<'a, const O: u8> BLK83_EN_W<'a, O> {
    #[doc = "Block83 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk83_en_0(self) -> &'a mut W {
        self.variant(BLK83_EN_A::BLK83_EN_0)
    }
    #[doc = "Block83 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk83_en_1(self) -> &'a mut W {
        self.variant(BLK83_EN_A::BLK83_EN_1)
    }
}
#[doc = "Field `BLK84_EN` reader - When 1, Block84 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK84_EN_R = crate::BitReader<BLK84_EN_A>;
#[doc = "When 1, Block84 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK84_EN_A {
    #[doc = "0: Block84 of the SRAM is not retained in LPM3 or LPM4"]
    BLK84_EN_0 = 0,
    #[doc = "1: Block84 of the SRAM is retained in LPM3 and LPM4"]
    BLK84_EN_1 = 1,
}
impl From<BLK84_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK84_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK84_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK84_EN_A {
        match self.bits {
            false => BLK84_EN_A::BLK84_EN_0,
            true => BLK84_EN_A::BLK84_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK84_EN_0`"]
    #[inline(always)]
    pub fn is_blk84_en_0(&self) -> bool {
        *self == BLK84_EN_A::BLK84_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK84_EN_1`"]
    #[inline(always)]
    pub fn is_blk84_en_1(&self) -> bool {
        *self == BLK84_EN_A::BLK84_EN_1
    }
}
#[doc = "Field `BLK84_EN` writer - When 1, Block84 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK84_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK84_EN_A, O>;
impl<'a, const O: u8> BLK84_EN_W<'a, O> {
    #[doc = "Block84 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk84_en_0(self) -> &'a mut W {
        self.variant(BLK84_EN_A::BLK84_EN_0)
    }
    #[doc = "Block84 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk84_en_1(self) -> &'a mut W {
        self.variant(BLK84_EN_A::BLK84_EN_1)
    }
}
#[doc = "Field `BLK85_EN` reader - When 1, Block85 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK85_EN_R = crate::BitReader<BLK85_EN_A>;
#[doc = "When 1, Block85 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK85_EN_A {
    #[doc = "0: Block85 of the SRAM is not retained in LPM3 or LPM4"]
    BLK85_EN_0 = 0,
    #[doc = "1: Block85 of the SRAM is retained in LPM3 and LPM4"]
    BLK85_EN_1 = 1,
}
impl From<BLK85_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK85_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK85_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK85_EN_A {
        match self.bits {
            false => BLK85_EN_A::BLK85_EN_0,
            true => BLK85_EN_A::BLK85_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK85_EN_0`"]
    #[inline(always)]
    pub fn is_blk85_en_0(&self) -> bool {
        *self == BLK85_EN_A::BLK85_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK85_EN_1`"]
    #[inline(always)]
    pub fn is_blk85_en_1(&self) -> bool {
        *self == BLK85_EN_A::BLK85_EN_1
    }
}
#[doc = "Field `BLK85_EN` writer - When 1, Block85 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK85_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK85_EN_A, O>;
impl<'a, const O: u8> BLK85_EN_W<'a, O> {
    #[doc = "Block85 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk85_en_0(self) -> &'a mut W {
        self.variant(BLK85_EN_A::BLK85_EN_0)
    }
    #[doc = "Block85 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk85_en_1(self) -> &'a mut W {
        self.variant(BLK85_EN_A::BLK85_EN_1)
    }
}
#[doc = "Field `BLK86_EN` reader - When 1, Block86 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK86_EN_R = crate::BitReader<BLK86_EN_A>;
#[doc = "When 1, Block86 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK86_EN_A {
    #[doc = "0: Block86 of the SRAM is not retained in LPM3 or LPM4"]
    BLK86_EN_0 = 0,
    #[doc = "1: Block86 of the SRAM is retained in LPM3 and LPM4"]
    BLK86_EN_1 = 1,
}
impl From<BLK86_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK86_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK86_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK86_EN_A {
        match self.bits {
            false => BLK86_EN_A::BLK86_EN_0,
            true => BLK86_EN_A::BLK86_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK86_EN_0`"]
    #[inline(always)]
    pub fn is_blk86_en_0(&self) -> bool {
        *self == BLK86_EN_A::BLK86_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK86_EN_1`"]
    #[inline(always)]
    pub fn is_blk86_en_1(&self) -> bool {
        *self == BLK86_EN_A::BLK86_EN_1
    }
}
#[doc = "Field `BLK86_EN` writer - When 1, Block86 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK86_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK86_EN_A, O>;
impl<'a, const O: u8> BLK86_EN_W<'a, O> {
    #[doc = "Block86 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk86_en_0(self) -> &'a mut W {
        self.variant(BLK86_EN_A::BLK86_EN_0)
    }
    #[doc = "Block86 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk86_en_1(self) -> &'a mut W {
        self.variant(BLK86_EN_A::BLK86_EN_1)
    }
}
#[doc = "Field `BLK87_EN` reader - When 1, Block87 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK87_EN_R = crate::BitReader<BLK87_EN_A>;
#[doc = "When 1, Block87 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK87_EN_A {
    #[doc = "0: Block87 of the SRAM is not retained in LPM3 or LPM4"]
    BLK87_EN_0 = 0,
    #[doc = "1: Block87 of the SRAM is retained in LPM3 and LPM4"]
    BLK87_EN_1 = 1,
}
impl From<BLK87_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK87_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK87_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK87_EN_A {
        match self.bits {
            false => BLK87_EN_A::BLK87_EN_0,
            true => BLK87_EN_A::BLK87_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK87_EN_0`"]
    #[inline(always)]
    pub fn is_blk87_en_0(&self) -> bool {
        *self == BLK87_EN_A::BLK87_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK87_EN_1`"]
    #[inline(always)]
    pub fn is_blk87_en_1(&self) -> bool {
        *self == BLK87_EN_A::BLK87_EN_1
    }
}
#[doc = "Field `BLK87_EN` writer - When 1, Block87 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK87_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK87_EN_A, O>;
impl<'a, const O: u8> BLK87_EN_W<'a, O> {
    #[doc = "Block87 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk87_en_0(self) -> &'a mut W {
        self.variant(BLK87_EN_A::BLK87_EN_0)
    }
    #[doc = "Block87 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk87_en_1(self) -> &'a mut W {
        self.variant(BLK87_EN_A::BLK87_EN_1)
    }
}
#[doc = "Field `BLK88_EN` reader - When 1, Block88 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK88_EN_R = crate::BitReader<BLK88_EN_A>;
#[doc = "When 1, Block88 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK88_EN_A {
    #[doc = "0: Block88 of the SRAM is not retained in LPM3 or LPM4"]
    BLK88_EN_0 = 0,
    #[doc = "1: Block88 of the SRAM is retained in LPM3 and LPM4"]
    BLK88_EN_1 = 1,
}
impl From<BLK88_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK88_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK88_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK88_EN_A {
        match self.bits {
            false => BLK88_EN_A::BLK88_EN_0,
            true => BLK88_EN_A::BLK88_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK88_EN_0`"]
    #[inline(always)]
    pub fn is_blk88_en_0(&self) -> bool {
        *self == BLK88_EN_A::BLK88_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK88_EN_1`"]
    #[inline(always)]
    pub fn is_blk88_en_1(&self) -> bool {
        *self == BLK88_EN_A::BLK88_EN_1
    }
}
#[doc = "Field `BLK88_EN` writer - When 1, Block88 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK88_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK88_EN_A, O>;
impl<'a, const O: u8> BLK88_EN_W<'a, O> {
    #[doc = "Block88 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk88_en_0(self) -> &'a mut W {
        self.variant(BLK88_EN_A::BLK88_EN_0)
    }
    #[doc = "Block88 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk88_en_1(self) -> &'a mut W {
        self.variant(BLK88_EN_A::BLK88_EN_1)
    }
}
#[doc = "Field `BLK89_EN` reader - When 1, Block89 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK89_EN_R = crate::BitReader<BLK89_EN_A>;
#[doc = "When 1, Block89 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK89_EN_A {
    #[doc = "0: Block89 of the SRAM is not retained in LPM3 or LPM4"]
    BLK89_EN_0 = 0,
    #[doc = "1: Block89 of the SRAM is retained in LPM3 and LPM4"]
    BLK89_EN_1 = 1,
}
impl From<BLK89_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK89_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK89_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK89_EN_A {
        match self.bits {
            false => BLK89_EN_A::BLK89_EN_0,
            true => BLK89_EN_A::BLK89_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK89_EN_0`"]
    #[inline(always)]
    pub fn is_blk89_en_0(&self) -> bool {
        *self == BLK89_EN_A::BLK89_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK89_EN_1`"]
    #[inline(always)]
    pub fn is_blk89_en_1(&self) -> bool {
        *self == BLK89_EN_A::BLK89_EN_1
    }
}
#[doc = "Field `BLK89_EN` writer - When 1, Block89 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK89_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK89_EN_A, O>;
impl<'a, const O: u8> BLK89_EN_W<'a, O> {
    #[doc = "Block89 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk89_en_0(self) -> &'a mut W {
        self.variant(BLK89_EN_A::BLK89_EN_0)
    }
    #[doc = "Block89 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk89_en_1(self) -> &'a mut W {
        self.variant(BLK89_EN_A::BLK89_EN_1)
    }
}
#[doc = "Field `BLK90_EN` reader - When 1, Block90 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK90_EN_R = crate::BitReader<BLK90_EN_A>;
#[doc = "When 1, Block90 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK90_EN_A {
    #[doc = "0: Block90 of the SRAM is not retained in LPM3 or LPM4"]
    BLK90_EN_0 = 0,
    #[doc = "1: Block90 of the SRAM is retained in LPM3 and LPM4"]
    BLK90_EN_1 = 1,
}
impl From<BLK90_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK90_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK90_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK90_EN_A {
        match self.bits {
            false => BLK90_EN_A::BLK90_EN_0,
            true => BLK90_EN_A::BLK90_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK90_EN_0`"]
    #[inline(always)]
    pub fn is_blk90_en_0(&self) -> bool {
        *self == BLK90_EN_A::BLK90_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK90_EN_1`"]
    #[inline(always)]
    pub fn is_blk90_en_1(&self) -> bool {
        *self == BLK90_EN_A::BLK90_EN_1
    }
}
#[doc = "Field `BLK90_EN` writer - When 1, Block90 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK90_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK90_EN_A, O>;
impl<'a, const O: u8> BLK90_EN_W<'a, O> {
    #[doc = "Block90 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk90_en_0(self) -> &'a mut W {
        self.variant(BLK90_EN_A::BLK90_EN_0)
    }
    #[doc = "Block90 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk90_en_1(self) -> &'a mut W {
        self.variant(BLK90_EN_A::BLK90_EN_1)
    }
}
#[doc = "Field `BLK91_EN` reader - When 1, Block91 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK91_EN_R = crate::BitReader<BLK91_EN_A>;
#[doc = "When 1, Block91 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK91_EN_A {
    #[doc = "0: Block91 of the SRAM is not retained in LPM3 or LPM4"]
    BLK91_EN_0 = 0,
    #[doc = "1: Block91 of the SRAM is retained in LPM3 and LPM4"]
    BLK91_EN_1 = 1,
}
impl From<BLK91_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK91_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK91_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK91_EN_A {
        match self.bits {
            false => BLK91_EN_A::BLK91_EN_0,
            true => BLK91_EN_A::BLK91_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK91_EN_0`"]
    #[inline(always)]
    pub fn is_blk91_en_0(&self) -> bool {
        *self == BLK91_EN_A::BLK91_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK91_EN_1`"]
    #[inline(always)]
    pub fn is_blk91_en_1(&self) -> bool {
        *self == BLK91_EN_A::BLK91_EN_1
    }
}
#[doc = "Field `BLK91_EN` writer - When 1, Block91 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK91_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK91_EN_A, O>;
impl<'a, const O: u8> BLK91_EN_W<'a, O> {
    #[doc = "Block91 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk91_en_0(self) -> &'a mut W {
        self.variant(BLK91_EN_A::BLK91_EN_0)
    }
    #[doc = "Block91 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk91_en_1(self) -> &'a mut W {
        self.variant(BLK91_EN_A::BLK91_EN_1)
    }
}
#[doc = "Field `BLK92_EN` reader - When 1, Block92 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK92_EN_R = crate::BitReader<BLK92_EN_A>;
#[doc = "When 1, Block92 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK92_EN_A {
    #[doc = "0: Block92 of the SRAM is not retained in LPM3 or LPM4"]
    BLK92_EN_0 = 0,
    #[doc = "1: Block92 of the SRAM is retained in LPM3 and LPM4"]
    BLK92_EN_1 = 1,
}
impl From<BLK92_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK92_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK92_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK92_EN_A {
        match self.bits {
            false => BLK92_EN_A::BLK92_EN_0,
            true => BLK92_EN_A::BLK92_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK92_EN_0`"]
    #[inline(always)]
    pub fn is_blk92_en_0(&self) -> bool {
        *self == BLK92_EN_A::BLK92_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK92_EN_1`"]
    #[inline(always)]
    pub fn is_blk92_en_1(&self) -> bool {
        *self == BLK92_EN_A::BLK92_EN_1
    }
}
#[doc = "Field `BLK92_EN` writer - When 1, Block92 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK92_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK92_EN_A, O>;
impl<'a, const O: u8> BLK92_EN_W<'a, O> {
    #[doc = "Block92 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk92_en_0(self) -> &'a mut W {
        self.variant(BLK92_EN_A::BLK92_EN_0)
    }
    #[doc = "Block92 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk92_en_1(self) -> &'a mut W {
        self.variant(BLK92_EN_A::BLK92_EN_1)
    }
}
#[doc = "Field `BLK93_EN` reader - When 1, Block93 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK93_EN_R = crate::BitReader<BLK93_EN_A>;
#[doc = "When 1, Block93 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK93_EN_A {
    #[doc = "0: Block93 of the SRAM is not retained in LPM3 or LPM4"]
    BLK93_EN_0 = 0,
    #[doc = "1: Block93 of the SRAM is retained in LPM3 and LPM4"]
    BLK93_EN_1 = 1,
}
impl From<BLK93_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK93_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK93_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK93_EN_A {
        match self.bits {
            false => BLK93_EN_A::BLK93_EN_0,
            true => BLK93_EN_A::BLK93_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK93_EN_0`"]
    #[inline(always)]
    pub fn is_blk93_en_0(&self) -> bool {
        *self == BLK93_EN_A::BLK93_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK93_EN_1`"]
    #[inline(always)]
    pub fn is_blk93_en_1(&self) -> bool {
        *self == BLK93_EN_A::BLK93_EN_1
    }
}
#[doc = "Field `BLK93_EN` writer - When 1, Block93 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK93_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK93_EN_A, O>;
impl<'a, const O: u8> BLK93_EN_W<'a, O> {
    #[doc = "Block93 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk93_en_0(self) -> &'a mut W {
        self.variant(BLK93_EN_A::BLK93_EN_0)
    }
    #[doc = "Block93 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk93_en_1(self) -> &'a mut W {
        self.variant(BLK93_EN_A::BLK93_EN_1)
    }
}
#[doc = "Field `BLK94_EN` reader - When 1, Block94 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK94_EN_R = crate::BitReader<BLK94_EN_A>;
#[doc = "When 1, Block94 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK94_EN_A {
    #[doc = "0: Block94 of the SRAM is not retained in LPM3 or LPM4"]
    BLK94_EN_0 = 0,
    #[doc = "1: Block94 of the SRAM is retained in LPM3 and LPM4"]
    BLK94_EN_1 = 1,
}
impl From<BLK94_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK94_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK94_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK94_EN_A {
        match self.bits {
            false => BLK94_EN_A::BLK94_EN_0,
            true => BLK94_EN_A::BLK94_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK94_EN_0`"]
    #[inline(always)]
    pub fn is_blk94_en_0(&self) -> bool {
        *self == BLK94_EN_A::BLK94_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK94_EN_1`"]
    #[inline(always)]
    pub fn is_blk94_en_1(&self) -> bool {
        *self == BLK94_EN_A::BLK94_EN_1
    }
}
#[doc = "Field `BLK94_EN` writer - When 1, Block94 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK94_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK94_EN_A, O>;
impl<'a, const O: u8> BLK94_EN_W<'a, O> {
    #[doc = "Block94 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk94_en_0(self) -> &'a mut W {
        self.variant(BLK94_EN_A::BLK94_EN_0)
    }
    #[doc = "Block94 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk94_en_1(self) -> &'a mut W {
        self.variant(BLK94_EN_A::BLK94_EN_1)
    }
}
#[doc = "Field `BLK95_EN` reader - When 1, Block95 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK95_EN_R = crate::BitReader<BLK95_EN_A>;
#[doc = "When 1, Block95 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK95_EN_A {
    #[doc = "0: Block95 of the SRAM is not retained in LPM3 or LPM4"]
    BLK95_EN_0 = 0,
    #[doc = "1: Block95 of the SRAM is retained in LPM3 and LPM4"]
    BLK95_EN_1 = 1,
}
impl From<BLK95_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK95_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK95_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK95_EN_A {
        match self.bits {
            false => BLK95_EN_A::BLK95_EN_0,
            true => BLK95_EN_A::BLK95_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK95_EN_0`"]
    #[inline(always)]
    pub fn is_blk95_en_0(&self) -> bool {
        *self == BLK95_EN_A::BLK95_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK95_EN_1`"]
    #[inline(always)]
    pub fn is_blk95_en_1(&self) -> bool {
        *self == BLK95_EN_A::BLK95_EN_1
    }
}
#[doc = "Field `BLK95_EN` writer - When 1, Block95 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK95_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL2_SPEC, BLK95_EN_A, O>;
impl<'a, const O: u8> BLK95_EN_W<'a, O> {
    #[doc = "Block95 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk95_en_0(self) -> &'a mut W {
        self.variant(BLK95_EN_A::BLK95_EN_0)
    }
    #[doc = "Block95 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk95_en_1(self) -> &'a mut W {
        self.variant(BLK95_EN_A::BLK95_EN_1)
    }
}
impl R {
    #[doc = "Bit 0 - When 1, Block64 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk64_en(&self) -> BLK64_EN_R {
        BLK64_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, Block65 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk65_en(&self) -> BLK65_EN_R {
        BLK65_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When 1, Block66 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk66_en(&self) -> BLK66_EN_R {
        BLK66_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When 1, Block67 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk67_en(&self) -> BLK67_EN_R {
        BLK67_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When 1, Block68 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk68_en(&self) -> BLK68_EN_R {
        BLK68_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When 1, Block69 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk69_en(&self) -> BLK69_EN_R {
        BLK69_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When 1, Block70 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk70_en(&self) -> BLK70_EN_R {
        BLK70_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When 1, Block71 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk71_en(&self) -> BLK71_EN_R {
        BLK71_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - When 1, Block72 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk72_en(&self) -> BLK72_EN_R {
        BLK72_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When 1, Block73 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk73_en(&self) -> BLK73_EN_R {
        BLK73_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When 1, Block74 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk74_en(&self) -> BLK74_EN_R {
        BLK74_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When 1, Block75 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk75_en(&self) -> BLK75_EN_R {
        BLK75_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - When 1, Block76 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk76_en(&self) -> BLK76_EN_R {
        BLK76_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - When 1, Block77 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk77_en(&self) -> BLK77_EN_R {
        BLK77_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - When 1, Block78 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk78_en(&self) -> BLK78_EN_R {
        BLK78_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - When 1, Block79 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk79_en(&self) -> BLK79_EN_R {
        BLK79_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - When 1, Block80 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk80_en(&self) -> BLK80_EN_R {
        BLK80_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When 1, Block81 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk81_en(&self) -> BLK81_EN_R {
        BLK81_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - When 1, Block82 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk82_en(&self) -> BLK82_EN_R {
        BLK82_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - When 1, Block83 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk83_en(&self) -> BLK83_EN_R {
        BLK83_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - When 1, Block84 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk84_en(&self) -> BLK84_EN_R {
        BLK84_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - When 1, Block85 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk85_en(&self) -> BLK85_EN_R {
        BLK85_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - When 1, Block86 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk86_en(&self) -> BLK86_EN_R {
        BLK86_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - When 1, Block87 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk87_en(&self) -> BLK87_EN_R {
        BLK87_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - When 1, Block88 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk88_en(&self) -> BLK88_EN_R {
        BLK88_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - When 1, Block89 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk89_en(&self) -> BLK89_EN_R {
        BLK89_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - When 1, Block90 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk90_en(&self) -> BLK90_EN_R {
        BLK90_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - When 1, Block91 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk91_en(&self) -> BLK91_EN_R {
        BLK91_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - When 1, Block92 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk92_en(&self) -> BLK92_EN_R {
        BLK92_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - When 1, Block93 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk93_en(&self) -> BLK93_EN_R {
        BLK93_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - When 1, Block94 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk94_en(&self) -> BLK94_EN_R {
        BLK94_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When 1, Block95 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk95_en(&self) -> BLK95_EN_R {
        BLK95_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, Block64 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk64_en(&mut self) -> BLK64_EN_W<0> {
        BLK64_EN_W::new(self)
    }
    #[doc = "Bit 1 - When 1, Block65 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk65_en(&mut self) -> BLK65_EN_W<1> {
        BLK65_EN_W::new(self)
    }
    #[doc = "Bit 2 - When 1, Block66 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk66_en(&mut self) -> BLK66_EN_W<2> {
        BLK66_EN_W::new(self)
    }
    #[doc = "Bit 3 - When 1, Block67 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk67_en(&mut self) -> BLK67_EN_W<3> {
        BLK67_EN_W::new(self)
    }
    #[doc = "Bit 4 - When 1, Block68 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk68_en(&mut self) -> BLK68_EN_W<4> {
        BLK68_EN_W::new(self)
    }
    #[doc = "Bit 5 - When 1, Block69 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk69_en(&mut self) -> BLK69_EN_W<5> {
        BLK69_EN_W::new(self)
    }
    #[doc = "Bit 6 - When 1, Block70 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk70_en(&mut self) -> BLK70_EN_W<6> {
        BLK70_EN_W::new(self)
    }
    #[doc = "Bit 7 - When 1, Block71 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk71_en(&mut self) -> BLK71_EN_W<7> {
        BLK71_EN_W::new(self)
    }
    #[doc = "Bit 8 - When 1, Block72 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk72_en(&mut self) -> BLK72_EN_W<8> {
        BLK72_EN_W::new(self)
    }
    #[doc = "Bit 9 - When 1, Block73 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk73_en(&mut self) -> BLK73_EN_W<9> {
        BLK73_EN_W::new(self)
    }
    #[doc = "Bit 10 - When 1, Block74 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk74_en(&mut self) -> BLK74_EN_W<10> {
        BLK74_EN_W::new(self)
    }
    #[doc = "Bit 11 - When 1, Block75 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk75_en(&mut self) -> BLK75_EN_W<11> {
        BLK75_EN_W::new(self)
    }
    #[doc = "Bit 12 - When 1, Block76 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk76_en(&mut self) -> BLK76_EN_W<12> {
        BLK76_EN_W::new(self)
    }
    #[doc = "Bit 13 - When 1, Block77 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk77_en(&mut self) -> BLK77_EN_W<13> {
        BLK77_EN_W::new(self)
    }
    #[doc = "Bit 14 - When 1, Block78 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk78_en(&mut self) -> BLK78_EN_W<14> {
        BLK78_EN_W::new(self)
    }
    #[doc = "Bit 15 - When 1, Block79 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk79_en(&mut self) -> BLK79_EN_W<15> {
        BLK79_EN_W::new(self)
    }
    #[doc = "Bit 16 - When 1, Block80 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk80_en(&mut self) -> BLK80_EN_W<16> {
        BLK80_EN_W::new(self)
    }
    #[doc = "Bit 17 - When 1, Block81 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk81_en(&mut self) -> BLK81_EN_W<17> {
        BLK81_EN_W::new(self)
    }
    #[doc = "Bit 18 - When 1, Block82 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk82_en(&mut self) -> BLK82_EN_W<18> {
        BLK82_EN_W::new(self)
    }
    #[doc = "Bit 19 - When 1, Block83 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk83_en(&mut self) -> BLK83_EN_W<19> {
        BLK83_EN_W::new(self)
    }
    #[doc = "Bit 20 - When 1, Block84 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk84_en(&mut self) -> BLK84_EN_W<20> {
        BLK84_EN_W::new(self)
    }
    #[doc = "Bit 21 - When 1, Block85 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk85_en(&mut self) -> BLK85_EN_W<21> {
        BLK85_EN_W::new(self)
    }
    #[doc = "Bit 22 - When 1, Block86 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk86_en(&mut self) -> BLK86_EN_W<22> {
        BLK86_EN_W::new(self)
    }
    #[doc = "Bit 23 - When 1, Block87 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk87_en(&mut self) -> BLK87_EN_W<23> {
        BLK87_EN_W::new(self)
    }
    #[doc = "Bit 24 - When 1, Block88 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk88_en(&mut self) -> BLK88_EN_W<24> {
        BLK88_EN_W::new(self)
    }
    #[doc = "Bit 25 - When 1, Block89 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk89_en(&mut self) -> BLK89_EN_W<25> {
        BLK89_EN_W::new(self)
    }
    #[doc = "Bit 26 - When 1, Block90 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk90_en(&mut self) -> BLK90_EN_W<26> {
        BLK90_EN_W::new(self)
    }
    #[doc = "Bit 27 - When 1, Block91 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk91_en(&mut self) -> BLK91_EN_W<27> {
        BLK91_EN_W::new(self)
    }
    #[doc = "Bit 28 - When 1, Block92 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk92_en(&mut self) -> BLK92_EN_W<28> {
        BLK92_EN_W::new(self)
    }
    #[doc = "Bit 29 - When 1, Block93 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk93_en(&mut self) -> BLK93_EN_W<29> {
        BLK93_EN_W::new(self)
    }
    #[doc = "Bit 30 - When 1, Block94 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk94_en(&mut self) -> BLK94_EN_W<30> {
        BLK94_EN_W::new(self)
    }
    #[doc = "Bit 31 - When 1, Block95 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk95_en(&mut self) -> BLK95_EN_W<31> {
        BLK95_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM Block Retention Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_sram_blkret_ctl2](index.html) module"]
pub struct SYS_SRAM_BLKRET_CTL2_SPEC;
impl crate::RegisterSpec for SYS_SRAM_BLKRET_CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_sram_blkret_ctl2::R](R) reader structure"]
impl crate::Readable for SYS_SRAM_BLKRET_CTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_sram_blkret_ctl2::W](W) writer structure"]
impl crate::Writable for SYS_SRAM_BLKRET_CTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_SRAM_BLKRET_CTL2 to value 0xffff_ffff"]
impl crate::Resettable for SYS_SRAM_BLKRET_CTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
