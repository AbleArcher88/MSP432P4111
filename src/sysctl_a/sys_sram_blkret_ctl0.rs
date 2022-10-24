#[doc = "Register `SYS_SRAM_BLKRET_CTL0` reader"]
pub struct R(crate::R<SYS_SRAM_BLKRET_CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_SRAM_BLKRET_CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_SRAM_BLKRET_CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_SRAM_BLKRET_CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_SRAM_BLKRET_CTL0` writer"]
pub struct W(crate::W<SYS_SRAM_BLKRET_CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_SRAM_BLKRET_CTL0_SPEC>;
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
impl From<crate::W<SYS_SRAM_BLKRET_CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_SRAM_BLKRET_CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLK0_EN` reader - Block0 is always retained in LPM3, LPM4 and LPM3.5 modes of operation"]
pub type BLK0_EN_R = crate::BitReader<bool>;
#[doc = "Field `BLK1_EN` reader - When 1, Block1 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK1_EN_R = crate::BitReader<BLK1_EN_A>;
#[doc = "When 1, Block1 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK1_EN_A {
    #[doc = "0: Block1 of the SRAM is not retained in LPM3 or LPM4"]
    BLK1_EN_0 = 0,
    #[doc = "1: Block1 of the SRAM is retained in LPM3 and LPM4"]
    BLK1_EN_1 = 1,
}
impl From<BLK1_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK1_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK1_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK1_EN_A {
        match self.bits {
            false => BLK1_EN_A::BLK1_EN_0,
            true => BLK1_EN_A::BLK1_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK1_EN_0`"]
    #[inline(always)]
    pub fn is_blk1_en_0(&self) -> bool {
        *self == BLK1_EN_A::BLK1_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK1_EN_1`"]
    #[inline(always)]
    pub fn is_blk1_en_1(&self) -> bool {
        *self == BLK1_EN_A::BLK1_EN_1
    }
}
#[doc = "Field `BLK1_EN` writer - When 1, Block1 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK1_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK1_EN_A, O>;
impl<'a, const O: u8> BLK1_EN_W<'a, O> {
    #[doc = "Block1 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk1_en_0(self) -> &'a mut W {
        self.variant(BLK1_EN_A::BLK1_EN_0)
    }
    #[doc = "Block1 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk1_en_1(self) -> &'a mut W {
        self.variant(BLK1_EN_A::BLK1_EN_1)
    }
}
#[doc = "Field `BLK2_EN` reader - When 1, Block2 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK2_EN_R = crate::BitReader<BLK2_EN_A>;
#[doc = "When 1, Block2 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK2_EN_A {
    #[doc = "0: Block2 of the SRAM is not retained in LPM3 or LPM4"]
    BLK2_EN_0 = 0,
    #[doc = "1: Block2 of the SRAM is retained in LPM3 and LPM4"]
    BLK2_EN_1 = 1,
}
impl From<BLK2_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK2_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK2_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK2_EN_A {
        match self.bits {
            false => BLK2_EN_A::BLK2_EN_0,
            true => BLK2_EN_A::BLK2_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK2_EN_0`"]
    #[inline(always)]
    pub fn is_blk2_en_0(&self) -> bool {
        *self == BLK2_EN_A::BLK2_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK2_EN_1`"]
    #[inline(always)]
    pub fn is_blk2_en_1(&self) -> bool {
        *self == BLK2_EN_A::BLK2_EN_1
    }
}
#[doc = "Field `BLK2_EN` writer - When 1, Block2 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK2_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK2_EN_A, O>;
impl<'a, const O: u8> BLK2_EN_W<'a, O> {
    #[doc = "Block2 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk2_en_0(self) -> &'a mut W {
        self.variant(BLK2_EN_A::BLK2_EN_0)
    }
    #[doc = "Block2 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk2_en_1(self) -> &'a mut W {
        self.variant(BLK2_EN_A::BLK2_EN_1)
    }
}
#[doc = "Field `BLK3_EN` reader - When 1, Block3 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK3_EN_R = crate::BitReader<BLK3_EN_A>;
#[doc = "When 1, Block3 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK3_EN_A {
    #[doc = "0: Block3 of the SRAM is not retained in LPM3 or LPM4"]
    BLK3_EN_0 = 0,
    #[doc = "1: Block3 of the SRAM is retained in LPM3 and LPM4"]
    BLK3_EN_1 = 1,
}
impl From<BLK3_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK3_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK3_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK3_EN_A {
        match self.bits {
            false => BLK3_EN_A::BLK3_EN_0,
            true => BLK3_EN_A::BLK3_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK3_EN_0`"]
    #[inline(always)]
    pub fn is_blk3_en_0(&self) -> bool {
        *self == BLK3_EN_A::BLK3_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK3_EN_1`"]
    #[inline(always)]
    pub fn is_blk3_en_1(&self) -> bool {
        *self == BLK3_EN_A::BLK3_EN_1
    }
}
#[doc = "Field `BLK3_EN` writer - When 1, Block3 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK3_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK3_EN_A, O>;
impl<'a, const O: u8> BLK3_EN_W<'a, O> {
    #[doc = "Block3 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk3_en_0(self) -> &'a mut W {
        self.variant(BLK3_EN_A::BLK3_EN_0)
    }
    #[doc = "Block3 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk3_en_1(self) -> &'a mut W {
        self.variant(BLK3_EN_A::BLK3_EN_1)
    }
}
#[doc = "Field `BLK4_EN` reader - When 1, Block4 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK4_EN_R = crate::BitReader<BLK4_EN_A>;
#[doc = "When 1, Block4 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK4_EN_A {
    #[doc = "0: Block4 of the SRAM is not retained in LPM3 or LPM4"]
    BLK4_EN_0 = 0,
    #[doc = "1: Block4 of the SRAM is retained in LPM3 and LPM4"]
    BLK4_EN_1 = 1,
}
impl From<BLK4_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK4_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK4_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK4_EN_A {
        match self.bits {
            false => BLK4_EN_A::BLK4_EN_0,
            true => BLK4_EN_A::BLK4_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK4_EN_0`"]
    #[inline(always)]
    pub fn is_blk4_en_0(&self) -> bool {
        *self == BLK4_EN_A::BLK4_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK4_EN_1`"]
    #[inline(always)]
    pub fn is_blk4_en_1(&self) -> bool {
        *self == BLK4_EN_A::BLK4_EN_1
    }
}
#[doc = "Field `BLK4_EN` writer - When 1, Block4 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK4_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK4_EN_A, O>;
impl<'a, const O: u8> BLK4_EN_W<'a, O> {
    #[doc = "Block4 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk4_en_0(self) -> &'a mut W {
        self.variant(BLK4_EN_A::BLK4_EN_0)
    }
    #[doc = "Block4 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk4_en_1(self) -> &'a mut W {
        self.variant(BLK4_EN_A::BLK4_EN_1)
    }
}
#[doc = "Field `BLK5_EN` reader - When 1, Block5 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK5_EN_R = crate::BitReader<BLK5_EN_A>;
#[doc = "When 1, Block5 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK5_EN_A {
    #[doc = "0: Block5 of the SRAM is not retained in LPM3 or LPM4"]
    BLK5_EN_0 = 0,
    #[doc = "1: Block5 of the SRAM is retained in LPM3 and LPM4"]
    BLK5_EN_1 = 1,
}
impl From<BLK5_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK5_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK5_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK5_EN_A {
        match self.bits {
            false => BLK5_EN_A::BLK5_EN_0,
            true => BLK5_EN_A::BLK5_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK5_EN_0`"]
    #[inline(always)]
    pub fn is_blk5_en_0(&self) -> bool {
        *self == BLK5_EN_A::BLK5_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK5_EN_1`"]
    #[inline(always)]
    pub fn is_blk5_en_1(&self) -> bool {
        *self == BLK5_EN_A::BLK5_EN_1
    }
}
#[doc = "Field `BLK5_EN` writer - When 1, Block5 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK5_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK5_EN_A, O>;
impl<'a, const O: u8> BLK5_EN_W<'a, O> {
    #[doc = "Block5 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk5_en_0(self) -> &'a mut W {
        self.variant(BLK5_EN_A::BLK5_EN_0)
    }
    #[doc = "Block5 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk5_en_1(self) -> &'a mut W {
        self.variant(BLK5_EN_A::BLK5_EN_1)
    }
}
#[doc = "Field `BLK6_EN` reader - When 1, Block6 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK6_EN_R = crate::BitReader<BLK6_EN_A>;
#[doc = "When 1, Block6 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK6_EN_A {
    #[doc = "0: Block6 of the SRAM is not retained in LPM3 or LPM4"]
    BLK6_EN_0 = 0,
    #[doc = "1: Block6 of the SRAM is retained in LPM3 and LPM4"]
    BLK6_EN_1 = 1,
}
impl From<BLK6_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK6_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK6_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK6_EN_A {
        match self.bits {
            false => BLK6_EN_A::BLK6_EN_0,
            true => BLK6_EN_A::BLK6_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK6_EN_0`"]
    #[inline(always)]
    pub fn is_blk6_en_0(&self) -> bool {
        *self == BLK6_EN_A::BLK6_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK6_EN_1`"]
    #[inline(always)]
    pub fn is_blk6_en_1(&self) -> bool {
        *self == BLK6_EN_A::BLK6_EN_1
    }
}
#[doc = "Field `BLK6_EN` writer - When 1, Block6 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK6_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK6_EN_A, O>;
impl<'a, const O: u8> BLK6_EN_W<'a, O> {
    #[doc = "Block6 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk6_en_0(self) -> &'a mut W {
        self.variant(BLK6_EN_A::BLK6_EN_0)
    }
    #[doc = "Block6 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk6_en_1(self) -> &'a mut W {
        self.variant(BLK6_EN_A::BLK6_EN_1)
    }
}
#[doc = "Field `BLK7_EN` reader - When 1, Block7 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK7_EN_R = crate::BitReader<BLK7_EN_A>;
#[doc = "When 1, Block7 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK7_EN_A {
    #[doc = "0: Block7 of the SRAM is not retained in LPM3 or LPM4"]
    BLK7_EN_0 = 0,
    #[doc = "1: Block7 of the SRAM is retained in LPM3 and LPM4"]
    BLK7_EN_1 = 1,
}
impl From<BLK7_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK7_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK7_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK7_EN_A {
        match self.bits {
            false => BLK7_EN_A::BLK7_EN_0,
            true => BLK7_EN_A::BLK7_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK7_EN_0`"]
    #[inline(always)]
    pub fn is_blk7_en_0(&self) -> bool {
        *self == BLK7_EN_A::BLK7_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK7_EN_1`"]
    #[inline(always)]
    pub fn is_blk7_en_1(&self) -> bool {
        *self == BLK7_EN_A::BLK7_EN_1
    }
}
#[doc = "Field `BLK7_EN` writer - When 1, Block7 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK7_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK7_EN_A, O>;
impl<'a, const O: u8> BLK7_EN_W<'a, O> {
    #[doc = "Block7 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk7_en_0(self) -> &'a mut W {
        self.variant(BLK7_EN_A::BLK7_EN_0)
    }
    #[doc = "Block7 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk7_en_1(self) -> &'a mut W {
        self.variant(BLK7_EN_A::BLK7_EN_1)
    }
}
#[doc = "Field `BLK8_EN` reader - When 1, Block8 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK8_EN_R = crate::BitReader<BLK8_EN_A>;
#[doc = "When 1, Block8 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK8_EN_A {
    #[doc = "0: Block8 of the SRAM is not retained in LPM3 or LPM4"]
    BLK8_EN_0 = 0,
    #[doc = "1: Block8 of the SRAM is retained in LPM3 and LPM4"]
    BLK8_EN_1 = 1,
}
impl From<BLK8_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK8_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK8_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK8_EN_A {
        match self.bits {
            false => BLK8_EN_A::BLK8_EN_0,
            true => BLK8_EN_A::BLK8_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK8_EN_0`"]
    #[inline(always)]
    pub fn is_blk8_en_0(&self) -> bool {
        *self == BLK8_EN_A::BLK8_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK8_EN_1`"]
    #[inline(always)]
    pub fn is_blk8_en_1(&self) -> bool {
        *self == BLK8_EN_A::BLK8_EN_1
    }
}
#[doc = "Field `BLK8_EN` writer - When 1, Block8 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK8_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK8_EN_A, O>;
impl<'a, const O: u8> BLK8_EN_W<'a, O> {
    #[doc = "Block8 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk8_en_0(self) -> &'a mut W {
        self.variant(BLK8_EN_A::BLK8_EN_0)
    }
    #[doc = "Block8 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk8_en_1(self) -> &'a mut W {
        self.variant(BLK8_EN_A::BLK8_EN_1)
    }
}
#[doc = "Field `BLK9_EN` reader - When 1, Block9 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK9_EN_R = crate::BitReader<BLK9_EN_A>;
#[doc = "When 1, Block9 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK9_EN_A {
    #[doc = "0: Block9 of the SRAM is not retained in LPM3 or LPM4"]
    BLK9_EN_0 = 0,
    #[doc = "1: Block9 of the SRAM is retained in LPM3 and LPM4"]
    BLK9_EN_1 = 1,
}
impl From<BLK9_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK9_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK9_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK9_EN_A {
        match self.bits {
            false => BLK9_EN_A::BLK9_EN_0,
            true => BLK9_EN_A::BLK9_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK9_EN_0`"]
    #[inline(always)]
    pub fn is_blk9_en_0(&self) -> bool {
        *self == BLK9_EN_A::BLK9_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK9_EN_1`"]
    #[inline(always)]
    pub fn is_blk9_en_1(&self) -> bool {
        *self == BLK9_EN_A::BLK9_EN_1
    }
}
#[doc = "Field `BLK9_EN` writer - When 1, Block9 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK9_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK9_EN_A, O>;
impl<'a, const O: u8> BLK9_EN_W<'a, O> {
    #[doc = "Block9 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk9_en_0(self) -> &'a mut W {
        self.variant(BLK9_EN_A::BLK9_EN_0)
    }
    #[doc = "Block9 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk9_en_1(self) -> &'a mut W {
        self.variant(BLK9_EN_A::BLK9_EN_1)
    }
}
#[doc = "Field `BLK10_EN` reader - When 1, Block10 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK10_EN_R = crate::BitReader<BLK10_EN_A>;
#[doc = "When 1, Block10 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK10_EN_A {
    #[doc = "0: Block10 of the SRAM is not retained in LPM3 or LPM4"]
    BLK10_EN_0 = 0,
    #[doc = "1: Block10 of the SRAM is retained in LPM3 and LPM4"]
    BLK10_EN_1 = 1,
}
impl From<BLK10_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK10_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK10_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK10_EN_A {
        match self.bits {
            false => BLK10_EN_A::BLK10_EN_0,
            true => BLK10_EN_A::BLK10_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK10_EN_0`"]
    #[inline(always)]
    pub fn is_blk10_en_0(&self) -> bool {
        *self == BLK10_EN_A::BLK10_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK10_EN_1`"]
    #[inline(always)]
    pub fn is_blk10_en_1(&self) -> bool {
        *self == BLK10_EN_A::BLK10_EN_1
    }
}
#[doc = "Field `BLK10_EN` writer - When 1, Block10 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK10_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK10_EN_A, O>;
impl<'a, const O: u8> BLK10_EN_W<'a, O> {
    #[doc = "Block10 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk10_en_0(self) -> &'a mut W {
        self.variant(BLK10_EN_A::BLK10_EN_0)
    }
    #[doc = "Block10 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk10_en_1(self) -> &'a mut W {
        self.variant(BLK10_EN_A::BLK10_EN_1)
    }
}
#[doc = "Field `BLK11_EN` reader - When 1, Block11 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK11_EN_R = crate::BitReader<BLK11_EN_A>;
#[doc = "When 1, Block11 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK11_EN_A {
    #[doc = "0: Block11 of the SRAM is not retained in LPM3 or LPM4"]
    BLK11_EN_0 = 0,
    #[doc = "1: Block11 of the SRAM is retained in LPM3 and LPM4"]
    BLK11_EN_1 = 1,
}
impl From<BLK11_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK11_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK11_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK11_EN_A {
        match self.bits {
            false => BLK11_EN_A::BLK11_EN_0,
            true => BLK11_EN_A::BLK11_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK11_EN_0`"]
    #[inline(always)]
    pub fn is_blk11_en_0(&self) -> bool {
        *self == BLK11_EN_A::BLK11_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK11_EN_1`"]
    #[inline(always)]
    pub fn is_blk11_en_1(&self) -> bool {
        *self == BLK11_EN_A::BLK11_EN_1
    }
}
#[doc = "Field `BLK11_EN` writer - When 1, Block11 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK11_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK11_EN_A, O>;
impl<'a, const O: u8> BLK11_EN_W<'a, O> {
    #[doc = "Block11 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk11_en_0(self) -> &'a mut W {
        self.variant(BLK11_EN_A::BLK11_EN_0)
    }
    #[doc = "Block11 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk11_en_1(self) -> &'a mut W {
        self.variant(BLK11_EN_A::BLK11_EN_1)
    }
}
#[doc = "Field `BLK12_EN` reader - When 1, Block12 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK12_EN_R = crate::BitReader<BLK12_EN_A>;
#[doc = "When 1, Block12 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK12_EN_A {
    #[doc = "0: Block12 of the SRAM is not retained in LPM3 or LPM4"]
    BLK12_EN_0 = 0,
    #[doc = "1: Block12 of the SRAM is retained in LPM3 and LPM4"]
    BLK12_EN_1 = 1,
}
impl From<BLK12_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK12_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK12_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK12_EN_A {
        match self.bits {
            false => BLK12_EN_A::BLK12_EN_0,
            true => BLK12_EN_A::BLK12_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK12_EN_0`"]
    #[inline(always)]
    pub fn is_blk12_en_0(&self) -> bool {
        *self == BLK12_EN_A::BLK12_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK12_EN_1`"]
    #[inline(always)]
    pub fn is_blk12_en_1(&self) -> bool {
        *self == BLK12_EN_A::BLK12_EN_1
    }
}
#[doc = "Field `BLK12_EN` writer - When 1, Block12 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK12_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK12_EN_A, O>;
impl<'a, const O: u8> BLK12_EN_W<'a, O> {
    #[doc = "Block12 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk12_en_0(self) -> &'a mut W {
        self.variant(BLK12_EN_A::BLK12_EN_0)
    }
    #[doc = "Block12 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk12_en_1(self) -> &'a mut W {
        self.variant(BLK12_EN_A::BLK12_EN_1)
    }
}
#[doc = "Field `BLK13_EN` reader - When 1, Block13 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK13_EN_R = crate::BitReader<BLK13_EN_A>;
#[doc = "When 1, Block13 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK13_EN_A {
    #[doc = "0: Block13 of the SRAM is not retained in LPM3 or LPM4"]
    BLK13_EN_0 = 0,
    #[doc = "1: Block13 of the SRAM is retained in LPM3 and LPM4"]
    BLK13_EN_1 = 1,
}
impl From<BLK13_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK13_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK13_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK13_EN_A {
        match self.bits {
            false => BLK13_EN_A::BLK13_EN_0,
            true => BLK13_EN_A::BLK13_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK13_EN_0`"]
    #[inline(always)]
    pub fn is_blk13_en_0(&self) -> bool {
        *self == BLK13_EN_A::BLK13_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK13_EN_1`"]
    #[inline(always)]
    pub fn is_blk13_en_1(&self) -> bool {
        *self == BLK13_EN_A::BLK13_EN_1
    }
}
#[doc = "Field `BLK13_EN` writer - When 1, Block13 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK13_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK13_EN_A, O>;
impl<'a, const O: u8> BLK13_EN_W<'a, O> {
    #[doc = "Block13 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk13_en_0(self) -> &'a mut W {
        self.variant(BLK13_EN_A::BLK13_EN_0)
    }
    #[doc = "Block13 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk13_en_1(self) -> &'a mut W {
        self.variant(BLK13_EN_A::BLK13_EN_1)
    }
}
#[doc = "Field `BLK14_EN` reader - When 1, Block14 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK14_EN_R = crate::BitReader<BLK14_EN_A>;
#[doc = "When 1, Block14 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK14_EN_A {
    #[doc = "0: Block14 of the SRAM is not retained in LPM3 or LPM4"]
    BLK14_EN_0 = 0,
    #[doc = "1: Block14 of the SRAM is retained in LPM3 and LPM4"]
    BLK14_EN_1 = 1,
}
impl From<BLK14_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK14_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK14_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK14_EN_A {
        match self.bits {
            false => BLK14_EN_A::BLK14_EN_0,
            true => BLK14_EN_A::BLK14_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK14_EN_0`"]
    #[inline(always)]
    pub fn is_blk14_en_0(&self) -> bool {
        *self == BLK14_EN_A::BLK14_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK14_EN_1`"]
    #[inline(always)]
    pub fn is_blk14_en_1(&self) -> bool {
        *self == BLK14_EN_A::BLK14_EN_1
    }
}
#[doc = "Field `BLK14_EN` writer - When 1, Block14 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK14_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK14_EN_A, O>;
impl<'a, const O: u8> BLK14_EN_W<'a, O> {
    #[doc = "Block14 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk14_en_0(self) -> &'a mut W {
        self.variant(BLK14_EN_A::BLK14_EN_0)
    }
    #[doc = "Block14 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk14_en_1(self) -> &'a mut W {
        self.variant(BLK14_EN_A::BLK14_EN_1)
    }
}
#[doc = "Field `BLK15_EN` reader - When 1, Block15 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK15_EN_R = crate::BitReader<BLK15_EN_A>;
#[doc = "When 1, Block15 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK15_EN_A {
    #[doc = "0: Block15 of the SRAM is not retained in LPM3 or LPM4"]
    BLK15_EN_0 = 0,
    #[doc = "1: Block15 of the SRAM is retained in LPM3 and LPM4"]
    BLK15_EN_1 = 1,
}
impl From<BLK15_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK15_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK15_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK15_EN_A {
        match self.bits {
            false => BLK15_EN_A::BLK15_EN_0,
            true => BLK15_EN_A::BLK15_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK15_EN_0`"]
    #[inline(always)]
    pub fn is_blk15_en_0(&self) -> bool {
        *self == BLK15_EN_A::BLK15_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK15_EN_1`"]
    #[inline(always)]
    pub fn is_blk15_en_1(&self) -> bool {
        *self == BLK15_EN_A::BLK15_EN_1
    }
}
#[doc = "Field `BLK15_EN` writer - When 1, Block15 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK15_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK15_EN_A, O>;
impl<'a, const O: u8> BLK15_EN_W<'a, O> {
    #[doc = "Block15 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk15_en_0(self) -> &'a mut W {
        self.variant(BLK15_EN_A::BLK15_EN_0)
    }
    #[doc = "Block15 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk15_en_1(self) -> &'a mut W {
        self.variant(BLK15_EN_A::BLK15_EN_1)
    }
}
#[doc = "Field `BLK16_EN` reader - When 1, Block16 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK16_EN_R = crate::BitReader<BLK16_EN_A>;
#[doc = "When 1, Block16 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK16_EN_A {
    #[doc = "0: Block16 of the SRAM is not retained in LPM3 or LPM4"]
    BLK16_EN_0 = 0,
    #[doc = "1: Block16 of the SRAM is retained in LPM3 and LPM4"]
    BLK16_EN_1 = 1,
}
impl From<BLK16_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK16_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK16_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK16_EN_A {
        match self.bits {
            false => BLK16_EN_A::BLK16_EN_0,
            true => BLK16_EN_A::BLK16_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK16_EN_0`"]
    #[inline(always)]
    pub fn is_blk16_en_0(&self) -> bool {
        *self == BLK16_EN_A::BLK16_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK16_EN_1`"]
    #[inline(always)]
    pub fn is_blk16_en_1(&self) -> bool {
        *self == BLK16_EN_A::BLK16_EN_1
    }
}
#[doc = "Field `BLK16_EN` writer - When 1, Block16 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK16_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK16_EN_A, O>;
impl<'a, const O: u8> BLK16_EN_W<'a, O> {
    #[doc = "Block16 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk16_en_0(self) -> &'a mut W {
        self.variant(BLK16_EN_A::BLK16_EN_0)
    }
    #[doc = "Block16 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk16_en_1(self) -> &'a mut W {
        self.variant(BLK16_EN_A::BLK16_EN_1)
    }
}
#[doc = "Field `BLK17_EN` reader - When 1, Block17 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK17_EN_R = crate::BitReader<BLK17_EN_A>;
#[doc = "When 1, Block17 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK17_EN_A {
    #[doc = "0: Block17 of the SRAM is not retained in LPM3 or LPM4"]
    BLK17_EN_0 = 0,
    #[doc = "1: Block17 of the SRAM is retained in LPM3 and LPM4"]
    BLK17_EN_1 = 1,
}
impl From<BLK17_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK17_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK17_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK17_EN_A {
        match self.bits {
            false => BLK17_EN_A::BLK17_EN_0,
            true => BLK17_EN_A::BLK17_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK17_EN_0`"]
    #[inline(always)]
    pub fn is_blk17_en_0(&self) -> bool {
        *self == BLK17_EN_A::BLK17_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK17_EN_1`"]
    #[inline(always)]
    pub fn is_blk17_en_1(&self) -> bool {
        *self == BLK17_EN_A::BLK17_EN_1
    }
}
#[doc = "Field `BLK17_EN` writer - When 1, Block17 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK17_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK17_EN_A, O>;
impl<'a, const O: u8> BLK17_EN_W<'a, O> {
    #[doc = "Block17 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk17_en_0(self) -> &'a mut W {
        self.variant(BLK17_EN_A::BLK17_EN_0)
    }
    #[doc = "Block17 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk17_en_1(self) -> &'a mut W {
        self.variant(BLK17_EN_A::BLK17_EN_1)
    }
}
#[doc = "Field `BLK18_EN` reader - When 1, Block18 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK18_EN_R = crate::BitReader<BLK18_EN_A>;
#[doc = "When 1, Block18 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK18_EN_A {
    #[doc = "0: Block18 of the SRAM is not retained in LPM3 or LPM4"]
    BLK18_EN_0 = 0,
    #[doc = "1: Block18 of the SRAM is retained in LPM3 and LPM4"]
    BLK18_EN_1 = 1,
}
impl From<BLK18_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK18_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK18_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK18_EN_A {
        match self.bits {
            false => BLK18_EN_A::BLK18_EN_0,
            true => BLK18_EN_A::BLK18_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK18_EN_0`"]
    #[inline(always)]
    pub fn is_blk18_en_0(&self) -> bool {
        *self == BLK18_EN_A::BLK18_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK18_EN_1`"]
    #[inline(always)]
    pub fn is_blk18_en_1(&self) -> bool {
        *self == BLK18_EN_A::BLK18_EN_1
    }
}
#[doc = "Field `BLK18_EN` writer - When 1, Block18 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK18_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK18_EN_A, O>;
impl<'a, const O: u8> BLK18_EN_W<'a, O> {
    #[doc = "Block18 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk18_en_0(self) -> &'a mut W {
        self.variant(BLK18_EN_A::BLK18_EN_0)
    }
    #[doc = "Block18 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk18_en_1(self) -> &'a mut W {
        self.variant(BLK18_EN_A::BLK18_EN_1)
    }
}
#[doc = "Field `BLK19_EN` reader - When 1, Block19 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK19_EN_R = crate::BitReader<BLK19_EN_A>;
#[doc = "When 1, Block19 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK19_EN_A {
    #[doc = "0: Block19 of the SRAM is not retained in LPM3 or LPM4"]
    BLK19_EN_0 = 0,
    #[doc = "1: Block19 of the SRAM is retained in LPM3 and LPM4"]
    BLK19_EN_1 = 1,
}
impl From<BLK19_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK19_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK19_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK19_EN_A {
        match self.bits {
            false => BLK19_EN_A::BLK19_EN_0,
            true => BLK19_EN_A::BLK19_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK19_EN_0`"]
    #[inline(always)]
    pub fn is_blk19_en_0(&self) -> bool {
        *self == BLK19_EN_A::BLK19_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK19_EN_1`"]
    #[inline(always)]
    pub fn is_blk19_en_1(&self) -> bool {
        *self == BLK19_EN_A::BLK19_EN_1
    }
}
#[doc = "Field `BLK19_EN` writer - When 1, Block19 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK19_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK19_EN_A, O>;
impl<'a, const O: u8> BLK19_EN_W<'a, O> {
    #[doc = "Block19 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk19_en_0(self) -> &'a mut W {
        self.variant(BLK19_EN_A::BLK19_EN_0)
    }
    #[doc = "Block19 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk19_en_1(self) -> &'a mut W {
        self.variant(BLK19_EN_A::BLK19_EN_1)
    }
}
#[doc = "Field `BLK20_EN` reader - When 1, Block20 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK20_EN_R = crate::BitReader<BLK20_EN_A>;
#[doc = "When 1, Block20 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK20_EN_A {
    #[doc = "0: Block20 of the SRAM is not retained in LPM3 or LPM4"]
    BLK20_EN_0 = 0,
    #[doc = "1: Block20 of the SRAM is retained in LPM3 and LPM4"]
    BLK20_EN_1 = 1,
}
impl From<BLK20_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK20_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK20_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK20_EN_A {
        match self.bits {
            false => BLK20_EN_A::BLK20_EN_0,
            true => BLK20_EN_A::BLK20_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK20_EN_0`"]
    #[inline(always)]
    pub fn is_blk20_en_0(&self) -> bool {
        *self == BLK20_EN_A::BLK20_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK20_EN_1`"]
    #[inline(always)]
    pub fn is_blk20_en_1(&self) -> bool {
        *self == BLK20_EN_A::BLK20_EN_1
    }
}
#[doc = "Field `BLK20_EN` writer - When 1, Block20 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK20_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK20_EN_A, O>;
impl<'a, const O: u8> BLK20_EN_W<'a, O> {
    #[doc = "Block20 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk20_en_0(self) -> &'a mut W {
        self.variant(BLK20_EN_A::BLK20_EN_0)
    }
    #[doc = "Block20 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk20_en_1(self) -> &'a mut W {
        self.variant(BLK20_EN_A::BLK20_EN_1)
    }
}
#[doc = "Field `BLK21_EN` reader - When 1, Block21 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK21_EN_R = crate::BitReader<BLK21_EN_A>;
#[doc = "When 1, Block21 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK21_EN_A {
    #[doc = "0: Block21 of the SRAM is not retained in LPM3 or LPM4"]
    BLK21_EN_0 = 0,
    #[doc = "1: Block21 of the SRAM is retained in LPM3 and LPM4"]
    BLK21_EN_1 = 1,
}
impl From<BLK21_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK21_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK21_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK21_EN_A {
        match self.bits {
            false => BLK21_EN_A::BLK21_EN_0,
            true => BLK21_EN_A::BLK21_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK21_EN_0`"]
    #[inline(always)]
    pub fn is_blk21_en_0(&self) -> bool {
        *self == BLK21_EN_A::BLK21_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK21_EN_1`"]
    #[inline(always)]
    pub fn is_blk21_en_1(&self) -> bool {
        *self == BLK21_EN_A::BLK21_EN_1
    }
}
#[doc = "Field `BLK21_EN` writer - When 1, Block21 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK21_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK21_EN_A, O>;
impl<'a, const O: u8> BLK21_EN_W<'a, O> {
    #[doc = "Block21 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk21_en_0(self) -> &'a mut W {
        self.variant(BLK21_EN_A::BLK21_EN_0)
    }
    #[doc = "Block21 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk21_en_1(self) -> &'a mut W {
        self.variant(BLK21_EN_A::BLK21_EN_1)
    }
}
#[doc = "Field `BLK22_EN` reader - When 1, Block22 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK22_EN_R = crate::BitReader<BLK22_EN_A>;
#[doc = "When 1, Block22 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK22_EN_A {
    #[doc = "0: Block22 of the SRAM is not retained in LPM3 or LPM4"]
    BLK22_EN_0 = 0,
    #[doc = "1: Block22 of the SRAM is retained in LPM3 and LPM4"]
    BLK22_EN_1 = 1,
}
impl From<BLK22_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK22_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK22_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK22_EN_A {
        match self.bits {
            false => BLK22_EN_A::BLK22_EN_0,
            true => BLK22_EN_A::BLK22_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK22_EN_0`"]
    #[inline(always)]
    pub fn is_blk22_en_0(&self) -> bool {
        *self == BLK22_EN_A::BLK22_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK22_EN_1`"]
    #[inline(always)]
    pub fn is_blk22_en_1(&self) -> bool {
        *self == BLK22_EN_A::BLK22_EN_1
    }
}
#[doc = "Field `BLK22_EN` writer - When 1, Block22 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK22_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK22_EN_A, O>;
impl<'a, const O: u8> BLK22_EN_W<'a, O> {
    #[doc = "Block22 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk22_en_0(self) -> &'a mut W {
        self.variant(BLK22_EN_A::BLK22_EN_0)
    }
    #[doc = "Block22 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk22_en_1(self) -> &'a mut W {
        self.variant(BLK22_EN_A::BLK22_EN_1)
    }
}
#[doc = "Field `BLK23_EN` reader - When 1, Block23 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK23_EN_R = crate::BitReader<BLK23_EN_A>;
#[doc = "When 1, Block23 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK23_EN_A {
    #[doc = "0: Block23 of the SRAM is not retained in LPM3 or LPM4"]
    BLK23_EN_0 = 0,
    #[doc = "1: Block23 of the SRAM is retained in LPM3 and LPM4"]
    BLK23_EN_1 = 1,
}
impl From<BLK23_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK23_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK23_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK23_EN_A {
        match self.bits {
            false => BLK23_EN_A::BLK23_EN_0,
            true => BLK23_EN_A::BLK23_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK23_EN_0`"]
    #[inline(always)]
    pub fn is_blk23_en_0(&self) -> bool {
        *self == BLK23_EN_A::BLK23_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK23_EN_1`"]
    #[inline(always)]
    pub fn is_blk23_en_1(&self) -> bool {
        *self == BLK23_EN_A::BLK23_EN_1
    }
}
#[doc = "Field `BLK23_EN` writer - When 1, Block23 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK23_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK23_EN_A, O>;
impl<'a, const O: u8> BLK23_EN_W<'a, O> {
    #[doc = "Block23 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk23_en_0(self) -> &'a mut W {
        self.variant(BLK23_EN_A::BLK23_EN_0)
    }
    #[doc = "Block23 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk23_en_1(self) -> &'a mut W {
        self.variant(BLK23_EN_A::BLK23_EN_1)
    }
}
#[doc = "Field `BLK24_EN` reader - When 1, Block24 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK24_EN_R = crate::BitReader<BLK24_EN_A>;
#[doc = "When 1, Block24 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK24_EN_A {
    #[doc = "0: Block24 of the SRAM is not retained in LPM3 or LPM4"]
    BLK24_EN_0 = 0,
    #[doc = "1: Block24 of the SRAM is retained in LPM3 and LPM4"]
    BLK24_EN_1 = 1,
}
impl From<BLK24_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK24_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK24_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK24_EN_A {
        match self.bits {
            false => BLK24_EN_A::BLK24_EN_0,
            true => BLK24_EN_A::BLK24_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK24_EN_0`"]
    #[inline(always)]
    pub fn is_blk24_en_0(&self) -> bool {
        *self == BLK24_EN_A::BLK24_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK24_EN_1`"]
    #[inline(always)]
    pub fn is_blk24_en_1(&self) -> bool {
        *self == BLK24_EN_A::BLK24_EN_1
    }
}
#[doc = "Field `BLK24_EN` writer - When 1, Block24 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK24_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK24_EN_A, O>;
impl<'a, const O: u8> BLK24_EN_W<'a, O> {
    #[doc = "Block24 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk24_en_0(self) -> &'a mut W {
        self.variant(BLK24_EN_A::BLK24_EN_0)
    }
    #[doc = "Block24 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk24_en_1(self) -> &'a mut W {
        self.variant(BLK24_EN_A::BLK24_EN_1)
    }
}
#[doc = "Field `BLK25_EN` reader - When 1, Block25 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK25_EN_R = crate::BitReader<BLK25_EN_A>;
#[doc = "When 1, Block25 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK25_EN_A {
    #[doc = "0: Block25 of the SRAM is not retained in LPM3 or LPM4"]
    BLK25_EN_0 = 0,
    #[doc = "1: Block25 of the SRAM is retained in LPM3 and LPM4"]
    BLK25_EN_1 = 1,
}
impl From<BLK25_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK25_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK25_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK25_EN_A {
        match self.bits {
            false => BLK25_EN_A::BLK25_EN_0,
            true => BLK25_EN_A::BLK25_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK25_EN_0`"]
    #[inline(always)]
    pub fn is_blk25_en_0(&self) -> bool {
        *self == BLK25_EN_A::BLK25_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK25_EN_1`"]
    #[inline(always)]
    pub fn is_blk25_en_1(&self) -> bool {
        *self == BLK25_EN_A::BLK25_EN_1
    }
}
#[doc = "Field `BLK25_EN` writer - When 1, Block25 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK25_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK25_EN_A, O>;
impl<'a, const O: u8> BLK25_EN_W<'a, O> {
    #[doc = "Block25 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk25_en_0(self) -> &'a mut W {
        self.variant(BLK25_EN_A::BLK25_EN_0)
    }
    #[doc = "Block25 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk25_en_1(self) -> &'a mut W {
        self.variant(BLK25_EN_A::BLK25_EN_1)
    }
}
#[doc = "Field `BLK26_EN` reader - When 1, Block26 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK26_EN_R = crate::BitReader<BLK26_EN_A>;
#[doc = "When 1, Block26 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK26_EN_A {
    #[doc = "0: Block26 of the SRAM is not retained in LPM3 or LPM4"]
    BLK26_EN_0 = 0,
    #[doc = "1: Block26 of the SRAM is retained in LPM3 and LPM4"]
    BLK26_EN_1 = 1,
}
impl From<BLK26_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK26_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK26_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK26_EN_A {
        match self.bits {
            false => BLK26_EN_A::BLK26_EN_0,
            true => BLK26_EN_A::BLK26_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK26_EN_0`"]
    #[inline(always)]
    pub fn is_blk26_en_0(&self) -> bool {
        *self == BLK26_EN_A::BLK26_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK26_EN_1`"]
    #[inline(always)]
    pub fn is_blk26_en_1(&self) -> bool {
        *self == BLK26_EN_A::BLK26_EN_1
    }
}
#[doc = "Field `BLK26_EN` writer - When 1, Block26 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK26_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK26_EN_A, O>;
impl<'a, const O: u8> BLK26_EN_W<'a, O> {
    #[doc = "Block26 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk26_en_0(self) -> &'a mut W {
        self.variant(BLK26_EN_A::BLK26_EN_0)
    }
    #[doc = "Block26 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk26_en_1(self) -> &'a mut W {
        self.variant(BLK26_EN_A::BLK26_EN_1)
    }
}
#[doc = "Field `BLK27_EN` reader - When 1, Block27 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK27_EN_R = crate::BitReader<BLK27_EN_A>;
#[doc = "When 1, Block27 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK27_EN_A {
    #[doc = "0: Block27 of the SRAM is not retained in LPM3 or LPM4"]
    BLK27_EN_0 = 0,
    #[doc = "1: Block27 of the SRAM is retained in LPM3 and LPM4"]
    BLK27_EN_1 = 1,
}
impl From<BLK27_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK27_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK27_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK27_EN_A {
        match self.bits {
            false => BLK27_EN_A::BLK27_EN_0,
            true => BLK27_EN_A::BLK27_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK27_EN_0`"]
    #[inline(always)]
    pub fn is_blk27_en_0(&self) -> bool {
        *self == BLK27_EN_A::BLK27_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK27_EN_1`"]
    #[inline(always)]
    pub fn is_blk27_en_1(&self) -> bool {
        *self == BLK27_EN_A::BLK27_EN_1
    }
}
#[doc = "Field `BLK27_EN` writer - When 1, Block27 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK27_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK27_EN_A, O>;
impl<'a, const O: u8> BLK27_EN_W<'a, O> {
    #[doc = "Block27 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk27_en_0(self) -> &'a mut W {
        self.variant(BLK27_EN_A::BLK27_EN_0)
    }
    #[doc = "Block27 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk27_en_1(self) -> &'a mut W {
        self.variant(BLK27_EN_A::BLK27_EN_1)
    }
}
#[doc = "Field `BLK28_EN` reader - When 1, Block28 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK28_EN_R = crate::BitReader<BLK28_EN_A>;
#[doc = "When 1, Block28 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK28_EN_A {
    #[doc = "0: Block28 of the SRAM is not retained in LPM3 or LPM4"]
    BLK28_EN_0 = 0,
    #[doc = "1: Block28 of the SRAM is retained in LPM3 and LPM4"]
    BLK28_EN_1 = 1,
}
impl From<BLK28_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK28_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK28_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK28_EN_A {
        match self.bits {
            false => BLK28_EN_A::BLK28_EN_0,
            true => BLK28_EN_A::BLK28_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK28_EN_0`"]
    #[inline(always)]
    pub fn is_blk28_en_0(&self) -> bool {
        *self == BLK28_EN_A::BLK28_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK28_EN_1`"]
    #[inline(always)]
    pub fn is_blk28_en_1(&self) -> bool {
        *self == BLK28_EN_A::BLK28_EN_1
    }
}
#[doc = "Field `BLK28_EN` writer - When 1, Block28 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK28_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK28_EN_A, O>;
impl<'a, const O: u8> BLK28_EN_W<'a, O> {
    #[doc = "Block28 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk28_en_0(self) -> &'a mut W {
        self.variant(BLK28_EN_A::BLK28_EN_0)
    }
    #[doc = "Block28 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk28_en_1(self) -> &'a mut W {
        self.variant(BLK28_EN_A::BLK28_EN_1)
    }
}
#[doc = "Field `BLK29_EN` reader - When 1, Block29 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK29_EN_R = crate::BitReader<BLK29_EN_A>;
#[doc = "When 1, Block29 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK29_EN_A {
    #[doc = "0: Block29 of the SRAM is not retained in LPM3 or LPM4"]
    BLK29_EN_0 = 0,
    #[doc = "1: Block29 of the SRAM is retained in LPM3 and LPM4"]
    BLK29_EN_1 = 1,
}
impl From<BLK29_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK29_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK29_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK29_EN_A {
        match self.bits {
            false => BLK29_EN_A::BLK29_EN_0,
            true => BLK29_EN_A::BLK29_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK29_EN_0`"]
    #[inline(always)]
    pub fn is_blk29_en_0(&self) -> bool {
        *self == BLK29_EN_A::BLK29_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK29_EN_1`"]
    #[inline(always)]
    pub fn is_blk29_en_1(&self) -> bool {
        *self == BLK29_EN_A::BLK29_EN_1
    }
}
#[doc = "Field `BLK29_EN` writer - When 1, Block29 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK29_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK29_EN_A, O>;
impl<'a, const O: u8> BLK29_EN_W<'a, O> {
    #[doc = "Block29 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk29_en_0(self) -> &'a mut W {
        self.variant(BLK29_EN_A::BLK29_EN_0)
    }
    #[doc = "Block29 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk29_en_1(self) -> &'a mut W {
        self.variant(BLK29_EN_A::BLK29_EN_1)
    }
}
#[doc = "Field `BLK30_EN` reader - When 1, Block30 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK30_EN_R = crate::BitReader<BLK30_EN_A>;
#[doc = "When 1, Block30 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK30_EN_A {
    #[doc = "0: Block30 of the SRAM is not retained in LPM3 or LPM4"]
    BLK30_EN_0 = 0,
    #[doc = "1: Block30 of the SRAM is retained in LPM3 and LPM4"]
    BLK30_EN_1 = 1,
}
impl From<BLK30_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK30_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK30_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK30_EN_A {
        match self.bits {
            false => BLK30_EN_A::BLK30_EN_0,
            true => BLK30_EN_A::BLK30_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK30_EN_0`"]
    #[inline(always)]
    pub fn is_blk30_en_0(&self) -> bool {
        *self == BLK30_EN_A::BLK30_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK30_EN_1`"]
    #[inline(always)]
    pub fn is_blk30_en_1(&self) -> bool {
        *self == BLK30_EN_A::BLK30_EN_1
    }
}
#[doc = "Field `BLK30_EN` writer - When 1, Block30 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK30_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK30_EN_A, O>;
impl<'a, const O: u8> BLK30_EN_W<'a, O> {
    #[doc = "Block30 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk30_en_0(self) -> &'a mut W {
        self.variant(BLK30_EN_A::BLK30_EN_0)
    }
    #[doc = "Block30 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk30_en_1(self) -> &'a mut W {
        self.variant(BLK30_EN_A::BLK30_EN_1)
    }
}
#[doc = "Field `BLK31_EN` reader - When 1, Block31 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK31_EN_R = crate::BitReader<BLK31_EN_A>;
#[doc = "When 1, Block31 of the SRAM is retained in LPM3 and LPM4\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLK31_EN_A {
    #[doc = "0: Block31 of the SRAM is not retained in LPM3 or LPM4"]
    BLK31_EN_0 = 0,
    #[doc = "1: Block31 of the SRAM is retained in LPM3 and LPM4"]
    BLK31_EN_1 = 1,
}
impl From<BLK31_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BLK31_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BLK31_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLK31_EN_A {
        match self.bits {
            false => BLK31_EN_A::BLK31_EN_0,
            true => BLK31_EN_A::BLK31_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BLK31_EN_0`"]
    #[inline(always)]
    pub fn is_blk31_en_0(&self) -> bool {
        *self == BLK31_EN_A::BLK31_EN_0
    }
    #[doc = "Checks if the value of the field is `BLK31_EN_1`"]
    #[inline(always)]
    pub fn is_blk31_en_1(&self) -> bool {
        *self == BLK31_EN_A::BLK31_EN_1
    }
}
#[doc = "Field `BLK31_EN` writer - When 1, Block31 of the SRAM is retained in LPM3 and LPM4"]
pub type BLK31_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BLKRET_CTL0_SPEC, BLK31_EN_A, O>;
impl<'a, const O: u8> BLK31_EN_W<'a, O> {
    #[doc = "Block31 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn blk31_en_0(self) -> &'a mut W {
        self.variant(BLK31_EN_A::BLK31_EN_0)
    }
    #[doc = "Block31 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk31_en_1(self) -> &'a mut W {
        self.variant(BLK31_EN_A::BLK31_EN_1)
    }
}
impl R {
    #[doc = "Bit 0 - Block0 is always retained in LPM3, LPM4 and LPM3.5 modes of operation"]
    #[inline(always)]
    pub fn blk0_en(&self) -> BLK0_EN_R {
        BLK0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, Block1 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk1_en(&self) -> BLK1_EN_R {
        BLK1_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When 1, Block2 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk2_en(&self) -> BLK2_EN_R {
        BLK2_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When 1, Block3 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk3_en(&self) -> BLK3_EN_R {
        BLK3_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When 1, Block4 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk4_en(&self) -> BLK4_EN_R {
        BLK4_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When 1, Block5 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk5_en(&self) -> BLK5_EN_R {
        BLK5_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When 1, Block6 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk6_en(&self) -> BLK6_EN_R {
        BLK6_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When 1, Block7 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk7_en(&self) -> BLK7_EN_R {
        BLK7_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - When 1, Block8 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk8_en(&self) -> BLK8_EN_R {
        BLK8_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When 1, Block9 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk9_en(&self) -> BLK9_EN_R {
        BLK9_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When 1, Block10 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk10_en(&self) -> BLK10_EN_R {
        BLK10_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When 1, Block11 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk11_en(&self) -> BLK11_EN_R {
        BLK11_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - When 1, Block12 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk12_en(&self) -> BLK12_EN_R {
        BLK12_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - When 1, Block13 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk13_en(&self) -> BLK13_EN_R {
        BLK13_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - When 1, Block14 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk14_en(&self) -> BLK14_EN_R {
        BLK14_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - When 1, Block15 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk15_en(&self) -> BLK15_EN_R {
        BLK15_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - When 1, Block16 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk16_en(&self) -> BLK16_EN_R {
        BLK16_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When 1, Block17 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk17_en(&self) -> BLK17_EN_R {
        BLK17_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - When 1, Block18 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk18_en(&self) -> BLK18_EN_R {
        BLK18_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - When 1, Block19 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk19_en(&self) -> BLK19_EN_R {
        BLK19_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - When 1, Block20 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk20_en(&self) -> BLK20_EN_R {
        BLK20_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - When 1, Block21 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk21_en(&self) -> BLK21_EN_R {
        BLK21_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - When 1, Block22 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk22_en(&self) -> BLK22_EN_R {
        BLK22_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - When 1, Block23 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk23_en(&self) -> BLK23_EN_R {
        BLK23_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - When 1, Block24 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk24_en(&self) -> BLK24_EN_R {
        BLK24_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - When 1, Block25 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk25_en(&self) -> BLK25_EN_R {
        BLK25_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - When 1, Block26 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk26_en(&self) -> BLK26_EN_R {
        BLK26_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - When 1, Block27 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk27_en(&self) -> BLK27_EN_R {
        BLK27_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - When 1, Block28 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk28_en(&self) -> BLK28_EN_R {
        BLK28_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - When 1, Block29 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk29_en(&self) -> BLK29_EN_R {
        BLK29_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - When 1, Block30 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk30_en(&self) -> BLK30_EN_R {
        BLK30_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When 1, Block31 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk31_en(&self) -> BLK31_EN_R {
        BLK31_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - When 1, Block1 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk1_en(&mut self) -> BLK1_EN_W<1> {
        BLK1_EN_W::new(self)
    }
    #[doc = "Bit 2 - When 1, Block2 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk2_en(&mut self) -> BLK2_EN_W<2> {
        BLK2_EN_W::new(self)
    }
    #[doc = "Bit 3 - When 1, Block3 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk3_en(&mut self) -> BLK3_EN_W<3> {
        BLK3_EN_W::new(self)
    }
    #[doc = "Bit 4 - When 1, Block4 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk4_en(&mut self) -> BLK4_EN_W<4> {
        BLK4_EN_W::new(self)
    }
    #[doc = "Bit 5 - When 1, Block5 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk5_en(&mut self) -> BLK5_EN_W<5> {
        BLK5_EN_W::new(self)
    }
    #[doc = "Bit 6 - When 1, Block6 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk6_en(&mut self) -> BLK6_EN_W<6> {
        BLK6_EN_W::new(self)
    }
    #[doc = "Bit 7 - When 1, Block7 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk7_en(&mut self) -> BLK7_EN_W<7> {
        BLK7_EN_W::new(self)
    }
    #[doc = "Bit 8 - When 1, Block8 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk8_en(&mut self) -> BLK8_EN_W<8> {
        BLK8_EN_W::new(self)
    }
    #[doc = "Bit 9 - When 1, Block9 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk9_en(&mut self) -> BLK9_EN_W<9> {
        BLK9_EN_W::new(self)
    }
    #[doc = "Bit 10 - When 1, Block10 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk10_en(&mut self) -> BLK10_EN_W<10> {
        BLK10_EN_W::new(self)
    }
    #[doc = "Bit 11 - When 1, Block11 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk11_en(&mut self) -> BLK11_EN_W<11> {
        BLK11_EN_W::new(self)
    }
    #[doc = "Bit 12 - When 1, Block12 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk12_en(&mut self) -> BLK12_EN_W<12> {
        BLK12_EN_W::new(self)
    }
    #[doc = "Bit 13 - When 1, Block13 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk13_en(&mut self) -> BLK13_EN_W<13> {
        BLK13_EN_W::new(self)
    }
    #[doc = "Bit 14 - When 1, Block14 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk14_en(&mut self) -> BLK14_EN_W<14> {
        BLK14_EN_W::new(self)
    }
    #[doc = "Bit 15 - When 1, Block15 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk15_en(&mut self) -> BLK15_EN_W<15> {
        BLK15_EN_W::new(self)
    }
    #[doc = "Bit 16 - When 1, Block16 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk16_en(&mut self) -> BLK16_EN_W<16> {
        BLK16_EN_W::new(self)
    }
    #[doc = "Bit 17 - When 1, Block17 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk17_en(&mut self) -> BLK17_EN_W<17> {
        BLK17_EN_W::new(self)
    }
    #[doc = "Bit 18 - When 1, Block18 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk18_en(&mut self) -> BLK18_EN_W<18> {
        BLK18_EN_W::new(self)
    }
    #[doc = "Bit 19 - When 1, Block19 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk19_en(&mut self) -> BLK19_EN_W<19> {
        BLK19_EN_W::new(self)
    }
    #[doc = "Bit 20 - When 1, Block20 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk20_en(&mut self) -> BLK20_EN_W<20> {
        BLK20_EN_W::new(self)
    }
    #[doc = "Bit 21 - When 1, Block21 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk21_en(&mut self) -> BLK21_EN_W<21> {
        BLK21_EN_W::new(self)
    }
    #[doc = "Bit 22 - When 1, Block22 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk22_en(&mut self) -> BLK22_EN_W<22> {
        BLK22_EN_W::new(self)
    }
    #[doc = "Bit 23 - When 1, Block23 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk23_en(&mut self) -> BLK23_EN_W<23> {
        BLK23_EN_W::new(self)
    }
    #[doc = "Bit 24 - When 1, Block24 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk24_en(&mut self) -> BLK24_EN_W<24> {
        BLK24_EN_W::new(self)
    }
    #[doc = "Bit 25 - When 1, Block25 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk25_en(&mut self) -> BLK25_EN_W<25> {
        BLK25_EN_W::new(self)
    }
    #[doc = "Bit 26 - When 1, Block26 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk26_en(&mut self) -> BLK26_EN_W<26> {
        BLK26_EN_W::new(self)
    }
    #[doc = "Bit 27 - When 1, Block27 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk27_en(&mut self) -> BLK27_EN_W<27> {
        BLK27_EN_W::new(self)
    }
    #[doc = "Bit 28 - When 1, Block28 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk28_en(&mut self) -> BLK28_EN_W<28> {
        BLK28_EN_W::new(self)
    }
    #[doc = "Bit 29 - When 1, Block29 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk29_en(&mut self) -> BLK29_EN_W<29> {
        BLK29_EN_W::new(self)
    }
    #[doc = "Bit 30 - When 1, Block30 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk30_en(&mut self) -> BLK30_EN_W<30> {
        BLK30_EN_W::new(self)
    }
    #[doc = "Bit 31 - When 1, Block31 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn blk31_en(&mut self) -> BLK31_EN_W<31> {
        BLK31_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM Block Retention Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_sram_blkret_ctl0](index.html) module"]
pub struct SYS_SRAM_BLKRET_CTL0_SPEC;
impl crate::RegisterSpec for SYS_SRAM_BLKRET_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_sram_blkret_ctl0::R](R) reader structure"]
impl crate::Readable for SYS_SRAM_BLKRET_CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_sram_blkret_ctl0::W](W) writer structure"]
impl crate::Writable for SYS_SRAM_BLKRET_CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_SRAM_BLKRET_CTL0 to value 0xffff_ffff"]
impl crate::Resettable for SYS_SRAM_BLKRET_CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
