#[doc = "Register `SYS_SRAM_BANKEN_CTL2` reader"]
pub struct R(crate::R<SYS_SRAM_BANKEN_CTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_SRAM_BANKEN_CTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_SRAM_BANKEN_CTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_SRAM_BANKEN_CTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_SRAM_BANKEN_CTL2` writer"]
pub struct W(crate::W<SYS_SRAM_BANKEN_CTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_SRAM_BANKEN_CTL2_SPEC>;
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
impl From<crate::W<SYS_SRAM_BANKEN_CTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_SRAM_BANKEN_CTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BNK64_EN` reader - When 1, enables Bank64 of the SRAM"]
pub type BNK64_EN_R = crate::BitReader<BNK64_EN_A>;
#[doc = "When 1, enables Bank64 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK64_EN_A {
    #[doc = "0: Disables Bank64 of the SRAM"]
    BNK64_EN_0 = 0,
    #[doc = "1: Enables Bank64 of the SRAM"]
    BNK64_EN_1 = 1,
}
impl From<BNK64_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK64_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK64_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK64_EN_A {
        match self.bits {
            false => BNK64_EN_A::BNK64_EN_0,
            true => BNK64_EN_A::BNK64_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK64_EN_0`"]
    #[inline(always)]
    pub fn is_bnk64_en_0(&self) -> bool {
        *self == BNK64_EN_A::BNK64_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK64_EN_1`"]
    #[inline(always)]
    pub fn is_bnk64_en_1(&self) -> bool {
        *self == BNK64_EN_A::BNK64_EN_1
    }
}
#[doc = "Field `BNK64_EN` writer - When 1, enables Bank64 of the SRAM"]
pub type BNK64_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK64_EN_A, O>;
impl<'a, const O: u8> BNK64_EN_W<'a, O> {
    #[doc = "Disables Bank64 of the SRAM"]
    #[inline(always)]
    pub fn bnk64_en_0(self) -> &'a mut W {
        self.variant(BNK64_EN_A::BNK64_EN_0)
    }
    #[doc = "Enables Bank64 of the SRAM"]
    #[inline(always)]
    pub fn bnk64_en_1(self) -> &'a mut W {
        self.variant(BNK64_EN_A::BNK64_EN_1)
    }
}
#[doc = "Field `BNK65_EN` reader - When 1, enables Bank65 of the SRAM"]
pub type BNK65_EN_R = crate::BitReader<BNK65_EN_A>;
#[doc = "When 1, enables Bank65 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK65_EN_A {
    #[doc = "0: Disables Bank65 of the SRAM"]
    BNK65_EN_0 = 0,
    #[doc = "1: Enables Bank65 of the SRAM"]
    BNK65_EN_1 = 1,
}
impl From<BNK65_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK65_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK65_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK65_EN_A {
        match self.bits {
            false => BNK65_EN_A::BNK65_EN_0,
            true => BNK65_EN_A::BNK65_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK65_EN_0`"]
    #[inline(always)]
    pub fn is_bnk65_en_0(&self) -> bool {
        *self == BNK65_EN_A::BNK65_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK65_EN_1`"]
    #[inline(always)]
    pub fn is_bnk65_en_1(&self) -> bool {
        *self == BNK65_EN_A::BNK65_EN_1
    }
}
#[doc = "Field `BNK65_EN` writer - When 1, enables Bank65 of the SRAM"]
pub type BNK65_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK65_EN_A, O>;
impl<'a, const O: u8> BNK65_EN_W<'a, O> {
    #[doc = "Disables Bank65 of the SRAM"]
    #[inline(always)]
    pub fn bnk65_en_0(self) -> &'a mut W {
        self.variant(BNK65_EN_A::BNK65_EN_0)
    }
    #[doc = "Enables Bank65 of the SRAM"]
    #[inline(always)]
    pub fn bnk65_en_1(self) -> &'a mut W {
        self.variant(BNK65_EN_A::BNK65_EN_1)
    }
}
#[doc = "Field `BNK66_EN` reader - When 1, enables Bank66 of the SRAM"]
pub type BNK66_EN_R = crate::BitReader<BNK66_EN_A>;
#[doc = "When 1, enables Bank66 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK66_EN_A {
    #[doc = "0: Disables Bank66 of the SRAM"]
    BNK66_EN_0 = 0,
    #[doc = "1: Enables Bank66 of the SRAM"]
    BNK66_EN_1 = 1,
}
impl From<BNK66_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK66_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK66_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK66_EN_A {
        match self.bits {
            false => BNK66_EN_A::BNK66_EN_0,
            true => BNK66_EN_A::BNK66_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK66_EN_0`"]
    #[inline(always)]
    pub fn is_bnk66_en_0(&self) -> bool {
        *self == BNK66_EN_A::BNK66_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK66_EN_1`"]
    #[inline(always)]
    pub fn is_bnk66_en_1(&self) -> bool {
        *self == BNK66_EN_A::BNK66_EN_1
    }
}
#[doc = "Field `BNK66_EN` writer - When 1, enables Bank66 of the SRAM"]
pub type BNK66_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK66_EN_A, O>;
impl<'a, const O: u8> BNK66_EN_W<'a, O> {
    #[doc = "Disables Bank66 of the SRAM"]
    #[inline(always)]
    pub fn bnk66_en_0(self) -> &'a mut W {
        self.variant(BNK66_EN_A::BNK66_EN_0)
    }
    #[doc = "Enables Bank66 of the SRAM"]
    #[inline(always)]
    pub fn bnk66_en_1(self) -> &'a mut W {
        self.variant(BNK66_EN_A::BNK66_EN_1)
    }
}
#[doc = "Field `BNK67_EN` reader - When 1, enables Bank67 of the SRAM"]
pub type BNK67_EN_R = crate::BitReader<BNK67_EN_A>;
#[doc = "When 1, enables Bank67 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK67_EN_A {
    #[doc = "0: Disables Bank67 of the SRAM"]
    BNK67_EN_0 = 0,
    #[doc = "1: Enables Bank67 of the SRAM"]
    BNK67_EN_1 = 1,
}
impl From<BNK67_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK67_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK67_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK67_EN_A {
        match self.bits {
            false => BNK67_EN_A::BNK67_EN_0,
            true => BNK67_EN_A::BNK67_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK67_EN_0`"]
    #[inline(always)]
    pub fn is_bnk67_en_0(&self) -> bool {
        *self == BNK67_EN_A::BNK67_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK67_EN_1`"]
    #[inline(always)]
    pub fn is_bnk67_en_1(&self) -> bool {
        *self == BNK67_EN_A::BNK67_EN_1
    }
}
#[doc = "Field `BNK67_EN` writer - When 1, enables Bank67 of the SRAM"]
pub type BNK67_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK67_EN_A, O>;
impl<'a, const O: u8> BNK67_EN_W<'a, O> {
    #[doc = "Disables Bank67 of the SRAM"]
    #[inline(always)]
    pub fn bnk67_en_0(self) -> &'a mut W {
        self.variant(BNK67_EN_A::BNK67_EN_0)
    }
    #[doc = "Enables Bank67 of the SRAM"]
    #[inline(always)]
    pub fn bnk67_en_1(self) -> &'a mut W {
        self.variant(BNK67_EN_A::BNK67_EN_1)
    }
}
#[doc = "Field `BNK68_EN` reader - When 1, enables Bank68 of the SRAM"]
pub type BNK68_EN_R = crate::BitReader<BNK68_EN_A>;
#[doc = "When 1, enables Bank68 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK68_EN_A {
    #[doc = "0: Disables Bank68 of the SRAM"]
    BNK68_EN_0 = 0,
    #[doc = "1: Enables Bank68 of the SRAM"]
    BNK68_EN_1 = 1,
}
impl From<BNK68_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK68_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK68_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK68_EN_A {
        match self.bits {
            false => BNK68_EN_A::BNK68_EN_0,
            true => BNK68_EN_A::BNK68_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK68_EN_0`"]
    #[inline(always)]
    pub fn is_bnk68_en_0(&self) -> bool {
        *self == BNK68_EN_A::BNK68_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK68_EN_1`"]
    #[inline(always)]
    pub fn is_bnk68_en_1(&self) -> bool {
        *self == BNK68_EN_A::BNK68_EN_1
    }
}
#[doc = "Field `BNK68_EN` writer - When 1, enables Bank68 of the SRAM"]
pub type BNK68_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK68_EN_A, O>;
impl<'a, const O: u8> BNK68_EN_W<'a, O> {
    #[doc = "Disables Bank68 of the SRAM"]
    #[inline(always)]
    pub fn bnk68_en_0(self) -> &'a mut W {
        self.variant(BNK68_EN_A::BNK68_EN_0)
    }
    #[doc = "Enables Bank68 of the SRAM"]
    #[inline(always)]
    pub fn bnk68_en_1(self) -> &'a mut W {
        self.variant(BNK68_EN_A::BNK68_EN_1)
    }
}
#[doc = "Field `BNK69_EN` reader - When 1, enables Bank69 of the SRAM"]
pub type BNK69_EN_R = crate::BitReader<BNK69_EN_A>;
#[doc = "When 1, enables Bank69 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK69_EN_A {
    #[doc = "0: Disables Bank69 of the SRAM"]
    BNK69_EN_0 = 0,
    #[doc = "1: Enables Bank69 of the SRAM"]
    BNK69_EN_1 = 1,
}
impl From<BNK69_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK69_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK69_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK69_EN_A {
        match self.bits {
            false => BNK69_EN_A::BNK69_EN_0,
            true => BNK69_EN_A::BNK69_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK69_EN_0`"]
    #[inline(always)]
    pub fn is_bnk69_en_0(&self) -> bool {
        *self == BNK69_EN_A::BNK69_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK69_EN_1`"]
    #[inline(always)]
    pub fn is_bnk69_en_1(&self) -> bool {
        *self == BNK69_EN_A::BNK69_EN_1
    }
}
#[doc = "Field `BNK69_EN` writer - When 1, enables Bank69 of the SRAM"]
pub type BNK69_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK69_EN_A, O>;
impl<'a, const O: u8> BNK69_EN_W<'a, O> {
    #[doc = "Disables Bank69 of the SRAM"]
    #[inline(always)]
    pub fn bnk69_en_0(self) -> &'a mut W {
        self.variant(BNK69_EN_A::BNK69_EN_0)
    }
    #[doc = "Enables Bank69 of the SRAM"]
    #[inline(always)]
    pub fn bnk69_en_1(self) -> &'a mut W {
        self.variant(BNK69_EN_A::BNK69_EN_1)
    }
}
#[doc = "Field `BNK70_EN` reader - When 1, enables Bank70 of the SRAM"]
pub type BNK70_EN_R = crate::BitReader<BNK70_EN_A>;
#[doc = "When 1, enables Bank70 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK70_EN_A {
    #[doc = "0: Disables Bank70 of the SRAM"]
    BNK70_EN_0 = 0,
    #[doc = "1: Enables Bank70 of the SRAM"]
    BNK70_EN_1 = 1,
}
impl From<BNK70_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK70_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK70_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK70_EN_A {
        match self.bits {
            false => BNK70_EN_A::BNK70_EN_0,
            true => BNK70_EN_A::BNK70_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK70_EN_0`"]
    #[inline(always)]
    pub fn is_bnk70_en_0(&self) -> bool {
        *self == BNK70_EN_A::BNK70_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK70_EN_1`"]
    #[inline(always)]
    pub fn is_bnk70_en_1(&self) -> bool {
        *self == BNK70_EN_A::BNK70_EN_1
    }
}
#[doc = "Field `BNK70_EN` writer - When 1, enables Bank70 of the SRAM"]
pub type BNK70_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK70_EN_A, O>;
impl<'a, const O: u8> BNK70_EN_W<'a, O> {
    #[doc = "Disables Bank70 of the SRAM"]
    #[inline(always)]
    pub fn bnk70_en_0(self) -> &'a mut W {
        self.variant(BNK70_EN_A::BNK70_EN_0)
    }
    #[doc = "Enables Bank70 of the SRAM"]
    #[inline(always)]
    pub fn bnk70_en_1(self) -> &'a mut W {
        self.variant(BNK70_EN_A::BNK70_EN_1)
    }
}
#[doc = "Field `BNK71_EN` reader - When 1, enables Bank71 of the SRAM"]
pub type BNK71_EN_R = crate::BitReader<BNK71_EN_A>;
#[doc = "When 1, enables Bank71 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK71_EN_A {
    #[doc = "0: Disables Bank71 of the SRAM"]
    BNK71_EN_0 = 0,
    #[doc = "1: Enables Bank71 of the SRAM"]
    BNK71_EN_1 = 1,
}
impl From<BNK71_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK71_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK71_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK71_EN_A {
        match self.bits {
            false => BNK71_EN_A::BNK71_EN_0,
            true => BNK71_EN_A::BNK71_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK71_EN_0`"]
    #[inline(always)]
    pub fn is_bnk71_en_0(&self) -> bool {
        *self == BNK71_EN_A::BNK71_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK71_EN_1`"]
    #[inline(always)]
    pub fn is_bnk71_en_1(&self) -> bool {
        *self == BNK71_EN_A::BNK71_EN_1
    }
}
#[doc = "Field `BNK71_EN` writer - When 1, enables Bank71 of the SRAM"]
pub type BNK71_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK71_EN_A, O>;
impl<'a, const O: u8> BNK71_EN_W<'a, O> {
    #[doc = "Disables Bank71 of the SRAM"]
    #[inline(always)]
    pub fn bnk71_en_0(self) -> &'a mut W {
        self.variant(BNK71_EN_A::BNK71_EN_0)
    }
    #[doc = "Enables Bank71 of the SRAM"]
    #[inline(always)]
    pub fn bnk71_en_1(self) -> &'a mut W {
        self.variant(BNK71_EN_A::BNK71_EN_1)
    }
}
#[doc = "Field `BNK72_EN` reader - When 1, enables Bank72 of the SRAM"]
pub type BNK72_EN_R = crate::BitReader<BNK72_EN_A>;
#[doc = "When 1, enables Bank72 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK72_EN_A {
    #[doc = "0: Disables Bank72 of the SRAM"]
    BNK72_EN_0 = 0,
    #[doc = "1: Enables Bank72 of the SRAM"]
    BNK72_EN_1 = 1,
}
impl From<BNK72_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK72_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK72_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK72_EN_A {
        match self.bits {
            false => BNK72_EN_A::BNK72_EN_0,
            true => BNK72_EN_A::BNK72_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK72_EN_0`"]
    #[inline(always)]
    pub fn is_bnk72_en_0(&self) -> bool {
        *self == BNK72_EN_A::BNK72_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK72_EN_1`"]
    #[inline(always)]
    pub fn is_bnk72_en_1(&self) -> bool {
        *self == BNK72_EN_A::BNK72_EN_1
    }
}
#[doc = "Field `BNK72_EN` writer - When 1, enables Bank72 of the SRAM"]
pub type BNK72_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK72_EN_A, O>;
impl<'a, const O: u8> BNK72_EN_W<'a, O> {
    #[doc = "Disables Bank72 of the SRAM"]
    #[inline(always)]
    pub fn bnk72_en_0(self) -> &'a mut W {
        self.variant(BNK72_EN_A::BNK72_EN_0)
    }
    #[doc = "Enables Bank72 of the SRAM"]
    #[inline(always)]
    pub fn bnk72_en_1(self) -> &'a mut W {
        self.variant(BNK72_EN_A::BNK72_EN_1)
    }
}
#[doc = "Field `BNK73_EN` reader - When 1, enables Bank73 of the SRAM"]
pub type BNK73_EN_R = crate::BitReader<BNK73_EN_A>;
#[doc = "When 1, enables Bank73 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK73_EN_A {
    #[doc = "0: Disables Bank73 of the SRAM"]
    BNK73_EN_0 = 0,
    #[doc = "1: Enables Bank73 of the SRAM"]
    BNK73_EN_1 = 1,
}
impl From<BNK73_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK73_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK73_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK73_EN_A {
        match self.bits {
            false => BNK73_EN_A::BNK73_EN_0,
            true => BNK73_EN_A::BNK73_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK73_EN_0`"]
    #[inline(always)]
    pub fn is_bnk73_en_0(&self) -> bool {
        *self == BNK73_EN_A::BNK73_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK73_EN_1`"]
    #[inline(always)]
    pub fn is_bnk73_en_1(&self) -> bool {
        *self == BNK73_EN_A::BNK73_EN_1
    }
}
#[doc = "Field `BNK73_EN` writer - When 1, enables Bank73 of the SRAM"]
pub type BNK73_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK73_EN_A, O>;
impl<'a, const O: u8> BNK73_EN_W<'a, O> {
    #[doc = "Disables Bank73 of the SRAM"]
    #[inline(always)]
    pub fn bnk73_en_0(self) -> &'a mut W {
        self.variant(BNK73_EN_A::BNK73_EN_0)
    }
    #[doc = "Enables Bank73 of the SRAM"]
    #[inline(always)]
    pub fn bnk73_en_1(self) -> &'a mut W {
        self.variant(BNK73_EN_A::BNK73_EN_1)
    }
}
#[doc = "Field `BNK74_EN` reader - When 1, enables Bank74 of the SRAM"]
pub type BNK74_EN_R = crate::BitReader<BNK74_EN_A>;
#[doc = "When 1, enables Bank74 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK74_EN_A {
    #[doc = "0: Disables Bank74 of the SRAM"]
    BNK74_EN_0 = 0,
    #[doc = "1: Enables Bank74 of the SRAM"]
    BNK74_EN_1 = 1,
}
impl From<BNK74_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK74_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK74_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK74_EN_A {
        match self.bits {
            false => BNK74_EN_A::BNK74_EN_0,
            true => BNK74_EN_A::BNK74_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK74_EN_0`"]
    #[inline(always)]
    pub fn is_bnk74_en_0(&self) -> bool {
        *self == BNK74_EN_A::BNK74_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK74_EN_1`"]
    #[inline(always)]
    pub fn is_bnk74_en_1(&self) -> bool {
        *self == BNK74_EN_A::BNK74_EN_1
    }
}
#[doc = "Field `BNK74_EN` writer - When 1, enables Bank74 of the SRAM"]
pub type BNK74_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK74_EN_A, O>;
impl<'a, const O: u8> BNK74_EN_W<'a, O> {
    #[doc = "Disables Bank74 of the SRAM"]
    #[inline(always)]
    pub fn bnk74_en_0(self) -> &'a mut W {
        self.variant(BNK74_EN_A::BNK74_EN_0)
    }
    #[doc = "Enables Bank74 of the SRAM"]
    #[inline(always)]
    pub fn bnk74_en_1(self) -> &'a mut W {
        self.variant(BNK74_EN_A::BNK74_EN_1)
    }
}
#[doc = "Field `BNK75_EN` reader - When 1, enables Bank75 of the SRAM"]
pub type BNK75_EN_R = crate::BitReader<BNK75_EN_A>;
#[doc = "When 1, enables Bank75 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK75_EN_A {
    #[doc = "0: Disables Bank75 of the SRAM"]
    BNK75_EN_0 = 0,
    #[doc = "1: Enables Bank75 of the SRAM"]
    BNK75_EN_1 = 1,
}
impl From<BNK75_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK75_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK75_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK75_EN_A {
        match self.bits {
            false => BNK75_EN_A::BNK75_EN_0,
            true => BNK75_EN_A::BNK75_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK75_EN_0`"]
    #[inline(always)]
    pub fn is_bnk75_en_0(&self) -> bool {
        *self == BNK75_EN_A::BNK75_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK75_EN_1`"]
    #[inline(always)]
    pub fn is_bnk75_en_1(&self) -> bool {
        *self == BNK75_EN_A::BNK75_EN_1
    }
}
#[doc = "Field `BNK75_EN` writer - When 1, enables Bank75 of the SRAM"]
pub type BNK75_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK75_EN_A, O>;
impl<'a, const O: u8> BNK75_EN_W<'a, O> {
    #[doc = "Disables Bank75 of the SRAM"]
    #[inline(always)]
    pub fn bnk75_en_0(self) -> &'a mut W {
        self.variant(BNK75_EN_A::BNK75_EN_0)
    }
    #[doc = "Enables Bank75 of the SRAM"]
    #[inline(always)]
    pub fn bnk75_en_1(self) -> &'a mut W {
        self.variant(BNK75_EN_A::BNK75_EN_1)
    }
}
#[doc = "Field `BNK76_EN` reader - When 1, enables Bank76 of the SRAM"]
pub type BNK76_EN_R = crate::BitReader<BNK76_EN_A>;
#[doc = "When 1, enables Bank76 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK76_EN_A {
    #[doc = "0: Disables Bank76 of the SRAM"]
    BNK76_EN_0 = 0,
    #[doc = "1: Enables Bank76 of the SRAM"]
    BNK76_EN_1 = 1,
}
impl From<BNK76_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK76_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK76_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK76_EN_A {
        match self.bits {
            false => BNK76_EN_A::BNK76_EN_0,
            true => BNK76_EN_A::BNK76_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK76_EN_0`"]
    #[inline(always)]
    pub fn is_bnk76_en_0(&self) -> bool {
        *self == BNK76_EN_A::BNK76_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK76_EN_1`"]
    #[inline(always)]
    pub fn is_bnk76_en_1(&self) -> bool {
        *self == BNK76_EN_A::BNK76_EN_1
    }
}
#[doc = "Field `BNK76_EN` writer - When 1, enables Bank76 of the SRAM"]
pub type BNK76_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK76_EN_A, O>;
impl<'a, const O: u8> BNK76_EN_W<'a, O> {
    #[doc = "Disables Bank76 of the SRAM"]
    #[inline(always)]
    pub fn bnk76_en_0(self) -> &'a mut W {
        self.variant(BNK76_EN_A::BNK76_EN_0)
    }
    #[doc = "Enables Bank76 of the SRAM"]
    #[inline(always)]
    pub fn bnk76_en_1(self) -> &'a mut W {
        self.variant(BNK76_EN_A::BNK76_EN_1)
    }
}
#[doc = "Field `BNK77_EN` reader - When 1, enables Bank77 of the SRAM"]
pub type BNK77_EN_R = crate::BitReader<BNK77_EN_A>;
#[doc = "When 1, enables Bank77 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK77_EN_A {
    #[doc = "0: Disables Bank77 of the SRAM"]
    BNK77_EN_0 = 0,
    #[doc = "1: Enables Bank77 of the SRAM"]
    BNK77_EN_1 = 1,
}
impl From<BNK77_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK77_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK77_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK77_EN_A {
        match self.bits {
            false => BNK77_EN_A::BNK77_EN_0,
            true => BNK77_EN_A::BNK77_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK77_EN_0`"]
    #[inline(always)]
    pub fn is_bnk77_en_0(&self) -> bool {
        *self == BNK77_EN_A::BNK77_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK77_EN_1`"]
    #[inline(always)]
    pub fn is_bnk77_en_1(&self) -> bool {
        *self == BNK77_EN_A::BNK77_EN_1
    }
}
#[doc = "Field `BNK77_EN` writer - When 1, enables Bank77 of the SRAM"]
pub type BNK77_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK77_EN_A, O>;
impl<'a, const O: u8> BNK77_EN_W<'a, O> {
    #[doc = "Disables Bank77 of the SRAM"]
    #[inline(always)]
    pub fn bnk77_en_0(self) -> &'a mut W {
        self.variant(BNK77_EN_A::BNK77_EN_0)
    }
    #[doc = "Enables Bank77 of the SRAM"]
    #[inline(always)]
    pub fn bnk77_en_1(self) -> &'a mut W {
        self.variant(BNK77_EN_A::BNK77_EN_1)
    }
}
#[doc = "Field `BNK78_EN` reader - When 1, enables Bank78 of the SRAM"]
pub type BNK78_EN_R = crate::BitReader<BNK78_EN_A>;
#[doc = "When 1, enables Bank78 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK78_EN_A {
    #[doc = "0: Disables Bank78 of the SRAM"]
    BNK78_EN_0 = 0,
    #[doc = "1: Enables Bank78 of the SRAM"]
    BNK78_EN_1 = 1,
}
impl From<BNK78_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK78_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK78_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK78_EN_A {
        match self.bits {
            false => BNK78_EN_A::BNK78_EN_0,
            true => BNK78_EN_A::BNK78_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK78_EN_0`"]
    #[inline(always)]
    pub fn is_bnk78_en_0(&self) -> bool {
        *self == BNK78_EN_A::BNK78_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK78_EN_1`"]
    #[inline(always)]
    pub fn is_bnk78_en_1(&self) -> bool {
        *self == BNK78_EN_A::BNK78_EN_1
    }
}
#[doc = "Field `BNK78_EN` writer - When 1, enables Bank78 of the SRAM"]
pub type BNK78_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK78_EN_A, O>;
impl<'a, const O: u8> BNK78_EN_W<'a, O> {
    #[doc = "Disables Bank78 of the SRAM"]
    #[inline(always)]
    pub fn bnk78_en_0(self) -> &'a mut W {
        self.variant(BNK78_EN_A::BNK78_EN_0)
    }
    #[doc = "Enables Bank78 of the SRAM"]
    #[inline(always)]
    pub fn bnk78_en_1(self) -> &'a mut W {
        self.variant(BNK78_EN_A::BNK78_EN_1)
    }
}
#[doc = "Field `BNK79_EN` reader - When 1, enables Bank79 of the SRAM"]
pub type BNK79_EN_R = crate::BitReader<BNK79_EN_A>;
#[doc = "When 1, enables Bank79 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK79_EN_A {
    #[doc = "0: Disables Bank79 of the SRAM"]
    BNK79_EN_0 = 0,
    #[doc = "1: Enables Bank79 of the SRAM"]
    BNK79_EN_1 = 1,
}
impl From<BNK79_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK79_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK79_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK79_EN_A {
        match self.bits {
            false => BNK79_EN_A::BNK79_EN_0,
            true => BNK79_EN_A::BNK79_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK79_EN_0`"]
    #[inline(always)]
    pub fn is_bnk79_en_0(&self) -> bool {
        *self == BNK79_EN_A::BNK79_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK79_EN_1`"]
    #[inline(always)]
    pub fn is_bnk79_en_1(&self) -> bool {
        *self == BNK79_EN_A::BNK79_EN_1
    }
}
#[doc = "Field `BNK79_EN` writer - When 1, enables Bank79 of the SRAM"]
pub type BNK79_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK79_EN_A, O>;
impl<'a, const O: u8> BNK79_EN_W<'a, O> {
    #[doc = "Disables Bank79 of the SRAM"]
    #[inline(always)]
    pub fn bnk79_en_0(self) -> &'a mut W {
        self.variant(BNK79_EN_A::BNK79_EN_0)
    }
    #[doc = "Enables Bank79 of the SRAM"]
    #[inline(always)]
    pub fn bnk79_en_1(self) -> &'a mut W {
        self.variant(BNK79_EN_A::BNK79_EN_1)
    }
}
#[doc = "Field `BNK80_EN` reader - When 1, enables Bank80 of the SRAM"]
pub type BNK80_EN_R = crate::BitReader<BNK80_EN_A>;
#[doc = "When 1, enables Bank80 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK80_EN_A {
    #[doc = "0: Disables Bank80 of the SRAM"]
    BNK80_EN_0 = 0,
    #[doc = "1: Enables Bank80 of the SRAM"]
    BNK80_EN_1 = 1,
}
impl From<BNK80_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK80_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK80_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK80_EN_A {
        match self.bits {
            false => BNK80_EN_A::BNK80_EN_0,
            true => BNK80_EN_A::BNK80_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK80_EN_0`"]
    #[inline(always)]
    pub fn is_bnk80_en_0(&self) -> bool {
        *self == BNK80_EN_A::BNK80_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK80_EN_1`"]
    #[inline(always)]
    pub fn is_bnk80_en_1(&self) -> bool {
        *self == BNK80_EN_A::BNK80_EN_1
    }
}
#[doc = "Field `BNK80_EN` writer - When 1, enables Bank80 of the SRAM"]
pub type BNK80_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK80_EN_A, O>;
impl<'a, const O: u8> BNK80_EN_W<'a, O> {
    #[doc = "Disables Bank80 of the SRAM"]
    #[inline(always)]
    pub fn bnk80_en_0(self) -> &'a mut W {
        self.variant(BNK80_EN_A::BNK80_EN_0)
    }
    #[doc = "Enables Bank80 of the SRAM"]
    #[inline(always)]
    pub fn bnk80_en_1(self) -> &'a mut W {
        self.variant(BNK80_EN_A::BNK80_EN_1)
    }
}
#[doc = "Field `BNK81_EN` reader - When 1, enables Bank81 of the SRAM"]
pub type BNK81_EN_R = crate::BitReader<BNK81_EN_A>;
#[doc = "When 1, enables Bank81 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK81_EN_A {
    #[doc = "0: Disables Bank81 of the SRAM"]
    BNK81_EN_0 = 0,
    #[doc = "1: Enables Bank81 of the SRAM"]
    BNK81_EN_1 = 1,
}
impl From<BNK81_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK81_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK81_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK81_EN_A {
        match self.bits {
            false => BNK81_EN_A::BNK81_EN_0,
            true => BNK81_EN_A::BNK81_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK81_EN_0`"]
    #[inline(always)]
    pub fn is_bnk81_en_0(&self) -> bool {
        *self == BNK81_EN_A::BNK81_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK81_EN_1`"]
    #[inline(always)]
    pub fn is_bnk81_en_1(&self) -> bool {
        *self == BNK81_EN_A::BNK81_EN_1
    }
}
#[doc = "Field `BNK81_EN` writer - When 1, enables Bank81 of the SRAM"]
pub type BNK81_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK81_EN_A, O>;
impl<'a, const O: u8> BNK81_EN_W<'a, O> {
    #[doc = "Disables Bank81 of the SRAM"]
    #[inline(always)]
    pub fn bnk81_en_0(self) -> &'a mut W {
        self.variant(BNK81_EN_A::BNK81_EN_0)
    }
    #[doc = "Enables Bank81 of the SRAM"]
    #[inline(always)]
    pub fn bnk81_en_1(self) -> &'a mut W {
        self.variant(BNK81_EN_A::BNK81_EN_1)
    }
}
#[doc = "Field `BNK82_EN` reader - When 1, enables Bank82 of the SRAM"]
pub type BNK82_EN_R = crate::BitReader<BNK82_EN_A>;
#[doc = "When 1, enables Bank82 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK82_EN_A {
    #[doc = "0: Disables Bank82 of the SRAM"]
    BNK82_EN_0 = 0,
    #[doc = "1: Enables Bank82 of the SRAM"]
    BNK82_EN_1 = 1,
}
impl From<BNK82_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK82_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK82_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK82_EN_A {
        match self.bits {
            false => BNK82_EN_A::BNK82_EN_0,
            true => BNK82_EN_A::BNK82_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK82_EN_0`"]
    #[inline(always)]
    pub fn is_bnk82_en_0(&self) -> bool {
        *self == BNK82_EN_A::BNK82_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK82_EN_1`"]
    #[inline(always)]
    pub fn is_bnk82_en_1(&self) -> bool {
        *self == BNK82_EN_A::BNK82_EN_1
    }
}
#[doc = "Field `BNK82_EN` writer - When 1, enables Bank82 of the SRAM"]
pub type BNK82_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK82_EN_A, O>;
impl<'a, const O: u8> BNK82_EN_W<'a, O> {
    #[doc = "Disables Bank82 of the SRAM"]
    #[inline(always)]
    pub fn bnk82_en_0(self) -> &'a mut W {
        self.variant(BNK82_EN_A::BNK82_EN_0)
    }
    #[doc = "Enables Bank82 of the SRAM"]
    #[inline(always)]
    pub fn bnk82_en_1(self) -> &'a mut W {
        self.variant(BNK82_EN_A::BNK82_EN_1)
    }
}
#[doc = "Field `BNK83_EN` reader - When 1, enables Bank83 of the SRAM"]
pub type BNK83_EN_R = crate::BitReader<BNK83_EN_A>;
#[doc = "When 1, enables Bank83 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK83_EN_A {
    #[doc = "0: Disables Bank83 of the SRAM"]
    BNK83_EN_0 = 0,
    #[doc = "1: Enables Bank83 of the SRAM"]
    BNK83_EN_1 = 1,
}
impl From<BNK83_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK83_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK83_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK83_EN_A {
        match self.bits {
            false => BNK83_EN_A::BNK83_EN_0,
            true => BNK83_EN_A::BNK83_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK83_EN_0`"]
    #[inline(always)]
    pub fn is_bnk83_en_0(&self) -> bool {
        *self == BNK83_EN_A::BNK83_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK83_EN_1`"]
    #[inline(always)]
    pub fn is_bnk83_en_1(&self) -> bool {
        *self == BNK83_EN_A::BNK83_EN_1
    }
}
#[doc = "Field `BNK83_EN` writer - When 1, enables Bank83 of the SRAM"]
pub type BNK83_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK83_EN_A, O>;
impl<'a, const O: u8> BNK83_EN_W<'a, O> {
    #[doc = "Disables Bank83 of the SRAM"]
    #[inline(always)]
    pub fn bnk83_en_0(self) -> &'a mut W {
        self.variant(BNK83_EN_A::BNK83_EN_0)
    }
    #[doc = "Enables Bank83 of the SRAM"]
    #[inline(always)]
    pub fn bnk83_en_1(self) -> &'a mut W {
        self.variant(BNK83_EN_A::BNK83_EN_1)
    }
}
#[doc = "Field `BNK84_EN` reader - When 1, enables Bank84 of the SRAM"]
pub type BNK84_EN_R = crate::BitReader<BNK84_EN_A>;
#[doc = "When 1, enables Bank84 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK84_EN_A {
    #[doc = "0: Disables Bank84 of the SRAM"]
    BNK84_EN_0 = 0,
    #[doc = "1: Enables Bank84 of the SRAM"]
    BNK84_EN_1 = 1,
}
impl From<BNK84_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK84_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK84_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK84_EN_A {
        match self.bits {
            false => BNK84_EN_A::BNK84_EN_0,
            true => BNK84_EN_A::BNK84_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK84_EN_0`"]
    #[inline(always)]
    pub fn is_bnk84_en_0(&self) -> bool {
        *self == BNK84_EN_A::BNK84_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK84_EN_1`"]
    #[inline(always)]
    pub fn is_bnk84_en_1(&self) -> bool {
        *self == BNK84_EN_A::BNK84_EN_1
    }
}
#[doc = "Field `BNK84_EN` writer - When 1, enables Bank84 of the SRAM"]
pub type BNK84_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK84_EN_A, O>;
impl<'a, const O: u8> BNK84_EN_W<'a, O> {
    #[doc = "Disables Bank84 of the SRAM"]
    #[inline(always)]
    pub fn bnk84_en_0(self) -> &'a mut W {
        self.variant(BNK84_EN_A::BNK84_EN_0)
    }
    #[doc = "Enables Bank84 of the SRAM"]
    #[inline(always)]
    pub fn bnk84_en_1(self) -> &'a mut W {
        self.variant(BNK84_EN_A::BNK84_EN_1)
    }
}
#[doc = "Field `BNK85_EN` reader - When 1, enables Bank85 of the SRAM"]
pub type BNK85_EN_R = crate::BitReader<BNK85_EN_A>;
#[doc = "When 1, enables Bank85 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK85_EN_A {
    #[doc = "0: Disables Bank85 of the SRAM"]
    BNK85_EN_0 = 0,
    #[doc = "1: Enables Bank85 of the SRAM"]
    BNK85_EN_1 = 1,
}
impl From<BNK85_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK85_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK85_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK85_EN_A {
        match self.bits {
            false => BNK85_EN_A::BNK85_EN_0,
            true => BNK85_EN_A::BNK85_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK85_EN_0`"]
    #[inline(always)]
    pub fn is_bnk85_en_0(&self) -> bool {
        *self == BNK85_EN_A::BNK85_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK85_EN_1`"]
    #[inline(always)]
    pub fn is_bnk85_en_1(&self) -> bool {
        *self == BNK85_EN_A::BNK85_EN_1
    }
}
#[doc = "Field `BNK85_EN` writer - When 1, enables Bank85 of the SRAM"]
pub type BNK85_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK85_EN_A, O>;
impl<'a, const O: u8> BNK85_EN_W<'a, O> {
    #[doc = "Disables Bank85 of the SRAM"]
    #[inline(always)]
    pub fn bnk85_en_0(self) -> &'a mut W {
        self.variant(BNK85_EN_A::BNK85_EN_0)
    }
    #[doc = "Enables Bank85 of the SRAM"]
    #[inline(always)]
    pub fn bnk85_en_1(self) -> &'a mut W {
        self.variant(BNK85_EN_A::BNK85_EN_1)
    }
}
#[doc = "Field `BNK86_EN` reader - When 1, enables Bank86 of the SRAM"]
pub type BNK86_EN_R = crate::BitReader<BNK86_EN_A>;
#[doc = "When 1, enables Bank86 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK86_EN_A {
    #[doc = "0: Disables Bank86 of the SRAM"]
    BNK86_EN_0 = 0,
    #[doc = "1: Enables Bank86 of the SRAM"]
    BNK86_EN_1 = 1,
}
impl From<BNK86_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK86_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK86_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK86_EN_A {
        match self.bits {
            false => BNK86_EN_A::BNK86_EN_0,
            true => BNK86_EN_A::BNK86_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK86_EN_0`"]
    #[inline(always)]
    pub fn is_bnk86_en_0(&self) -> bool {
        *self == BNK86_EN_A::BNK86_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK86_EN_1`"]
    #[inline(always)]
    pub fn is_bnk86_en_1(&self) -> bool {
        *self == BNK86_EN_A::BNK86_EN_1
    }
}
#[doc = "Field `BNK86_EN` writer - When 1, enables Bank86 of the SRAM"]
pub type BNK86_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK86_EN_A, O>;
impl<'a, const O: u8> BNK86_EN_W<'a, O> {
    #[doc = "Disables Bank86 of the SRAM"]
    #[inline(always)]
    pub fn bnk86_en_0(self) -> &'a mut W {
        self.variant(BNK86_EN_A::BNK86_EN_0)
    }
    #[doc = "Enables Bank86 of the SRAM"]
    #[inline(always)]
    pub fn bnk86_en_1(self) -> &'a mut W {
        self.variant(BNK86_EN_A::BNK86_EN_1)
    }
}
#[doc = "Field `BNK87_EN` reader - When 1, enables Bank87 of the SRAM"]
pub type BNK87_EN_R = crate::BitReader<BNK87_EN_A>;
#[doc = "When 1, enables Bank87 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK87_EN_A {
    #[doc = "0: Disables Bank87 of the SRAM"]
    BNK87_EN_0 = 0,
    #[doc = "1: Enables Bank87 of the SRAM"]
    BNK87_EN_1 = 1,
}
impl From<BNK87_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK87_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK87_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK87_EN_A {
        match self.bits {
            false => BNK87_EN_A::BNK87_EN_0,
            true => BNK87_EN_A::BNK87_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK87_EN_0`"]
    #[inline(always)]
    pub fn is_bnk87_en_0(&self) -> bool {
        *self == BNK87_EN_A::BNK87_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK87_EN_1`"]
    #[inline(always)]
    pub fn is_bnk87_en_1(&self) -> bool {
        *self == BNK87_EN_A::BNK87_EN_1
    }
}
#[doc = "Field `BNK87_EN` writer - When 1, enables Bank87 of the SRAM"]
pub type BNK87_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK87_EN_A, O>;
impl<'a, const O: u8> BNK87_EN_W<'a, O> {
    #[doc = "Disables Bank87 of the SRAM"]
    #[inline(always)]
    pub fn bnk87_en_0(self) -> &'a mut W {
        self.variant(BNK87_EN_A::BNK87_EN_0)
    }
    #[doc = "Enables Bank87 of the SRAM"]
    #[inline(always)]
    pub fn bnk87_en_1(self) -> &'a mut W {
        self.variant(BNK87_EN_A::BNK87_EN_1)
    }
}
#[doc = "Field `BNK88_EN` reader - When 1, enables Bank88 of the SRAM"]
pub type BNK88_EN_R = crate::BitReader<BNK88_EN_A>;
#[doc = "When 1, enables Bank88 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK88_EN_A {
    #[doc = "0: Disables Bank88 of the SRAM"]
    BNK88_EN_0 = 0,
    #[doc = "1: Enables Bank88 of the SRAM"]
    BNK88_EN_1 = 1,
}
impl From<BNK88_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK88_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK88_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK88_EN_A {
        match self.bits {
            false => BNK88_EN_A::BNK88_EN_0,
            true => BNK88_EN_A::BNK88_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK88_EN_0`"]
    #[inline(always)]
    pub fn is_bnk88_en_0(&self) -> bool {
        *self == BNK88_EN_A::BNK88_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK88_EN_1`"]
    #[inline(always)]
    pub fn is_bnk88_en_1(&self) -> bool {
        *self == BNK88_EN_A::BNK88_EN_1
    }
}
#[doc = "Field `BNK88_EN` writer - When 1, enables Bank88 of the SRAM"]
pub type BNK88_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK88_EN_A, O>;
impl<'a, const O: u8> BNK88_EN_W<'a, O> {
    #[doc = "Disables Bank88 of the SRAM"]
    #[inline(always)]
    pub fn bnk88_en_0(self) -> &'a mut W {
        self.variant(BNK88_EN_A::BNK88_EN_0)
    }
    #[doc = "Enables Bank88 of the SRAM"]
    #[inline(always)]
    pub fn bnk88_en_1(self) -> &'a mut W {
        self.variant(BNK88_EN_A::BNK88_EN_1)
    }
}
#[doc = "Field `BNK89_EN` reader - When 1, enables Bank89 of the SRAM"]
pub type BNK89_EN_R = crate::BitReader<BNK89_EN_A>;
#[doc = "When 1, enables Bank89 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK89_EN_A {
    #[doc = "0: Disables Bank89 of the SRAM"]
    BNK89_EN_0 = 0,
    #[doc = "1: Enables Bank89 of the SRAM"]
    BNK89_EN_1 = 1,
}
impl From<BNK89_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK89_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK89_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK89_EN_A {
        match self.bits {
            false => BNK89_EN_A::BNK89_EN_0,
            true => BNK89_EN_A::BNK89_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK89_EN_0`"]
    #[inline(always)]
    pub fn is_bnk89_en_0(&self) -> bool {
        *self == BNK89_EN_A::BNK89_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK89_EN_1`"]
    #[inline(always)]
    pub fn is_bnk89_en_1(&self) -> bool {
        *self == BNK89_EN_A::BNK89_EN_1
    }
}
#[doc = "Field `BNK89_EN` writer - When 1, enables Bank89 of the SRAM"]
pub type BNK89_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK89_EN_A, O>;
impl<'a, const O: u8> BNK89_EN_W<'a, O> {
    #[doc = "Disables Bank89 of the SRAM"]
    #[inline(always)]
    pub fn bnk89_en_0(self) -> &'a mut W {
        self.variant(BNK89_EN_A::BNK89_EN_0)
    }
    #[doc = "Enables Bank89 of the SRAM"]
    #[inline(always)]
    pub fn bnk89_en_1(self) -> &'a mut W {
        self.variant(BNK89_EN_A::BNK89_EN_1)
    }
}
#[doc = "Field `BNK90_EN` reader - When 1, enables Bank90 of the SRAM"]
pub type BNK90_EN_R = crate::BitReader<BNK90_EN_A>;
#[doc = "When 1, enables Bank90 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK90_EN_A {
    #[doc = "0: Disables Bank90 of the SRAM"]
    BNK90_EN_0 = 0,
    #[doc = "1: Enables Bank90 of the SRAM"]
    BNK90_EN_1 = 1,
}
impl From<BNK90_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK90_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK90_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK90_EN_A {
        match self.bits {
            false => BNK90_EN_A::BNK90_EN_0,
            true => BNK90_EN_A::BNK90_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK90_EN_0`"]
    #[inline(always)]
    pub fn is_bnk90_en_0(&self) -> bool {
        *self == BNK90_EN_A::BNK90_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK90_EN_1`"]
    #[inline(always)]
    pub fn is_bnk90_en_1(&self) -> bool {
        *self == BNK90_EN_A::BNK90_EN_1
    }
}
#[doc = "Field `BNK90_EN` writer - When 1, enables Bank90 of the SRAM"]
pub type BNK90_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK90_EN_A, O>;
impl<'a, const O: u8> BNK90_EN_W<'a, O> {
    #[doc = "Disables Bank90 of the SRAM"]
    #[inline(always)]
    pub fn bnk90_en_0(self) -> &'a mut W {
        self.variant(BNK90_EN_A::BNK90_EN_0)
    }
    #[doc = "Enables Bank90 of the SRAM"]
    #[inline(always)]
    pub fn bnk90_en_1(self) -> &'a mut W {
        self.variant(BNK90_EN_A::BNK90_EN_1)
    }
}
#[doc = "Field `BNK91_EN` reader - When 1, enables Bank91 of the SRAM"]
pub type BNK91_EN_R = crate::BitReader<BNK91_EN_A>;
#[doc = "When 1, enables Bank91 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK91_EN_A {
    #[doc = "0: Disables Bank91 of the SRAM"]
    BNK91_EN_0 = 0,
    #[doc = "1: Enables Bank91 of the SRAM"]
    BNK91_EN_1 = 1,
}
impl From<BNK91_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK91_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK91_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK91_EN_A {
        match self.bits {
            false => BNK91_EN_A::BNK91_EN_0,
            true => BNK91_EN_A::BNK91_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK91_EN_0`"]
    #[inline(always)]
    pub fn is_bnk91_en_0(&self) -> bool {
        *self == BNK91_EN_A::BNK91_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK91_EN_1`"]
    #[inline(always)]
    pub fn is_bnk91_en_1(&self) -> bool {
        *self == BNK91_EN_A::BNK91_EN_1
    }
}
#[doc = "Field `BNK91_EN` writer - When 1, enables Bank91 of the SRAM"]
pub type BNK91_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK91_EN_A, O>;
impl<'a, const O: u8> BNK91_EN_W<'a, O> {
    #[doc = "Disables Bank91 of the SRAM"]
    #[inline(always)]
    pub fn bnk91_en_0(self) -> &'a mut W {
        self.variant(BNK91_EN_A::BNK91_EN_0)
    }
    #[doc = "Enables Bank91 of the SRAM"]
    #[inline(always)]
    pub fn bnk91_en_1(self) -> &'a mut W {
        self.variant(BNK91_EN_A::BNK91_EN_1)
    }
}
#[doc = "Field `BNK92_EN` reader - When 1, enables Bank92 of the SRAM"]
pub type BNK92_EN_R = crate::BitReader<BNK92_EN_A>;
#[doc = "When 1, enables Bank92 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK92_EN_A {
    #[doc = "0: Disables Bank92 of the SRAM"]
    BNK92_EN_0 = 0,
    #[doc = "1: Enables Bank92 of the SRAM"]
    BNK92_EN_1 = 1,
}
impl From<BNK92_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK92_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK92_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK92_EN_A {
        match self.bits {
            false => BNK92_EN_A::BNK92_EN_0,
            true => BNK92_EN_A::BNK92_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK92_EN_0`"]
    #[inline(always)]
    pub fn is_bnk92_en_0(&self) -> bool {
        *self == BNK92_EN_A::BNK92_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK92_EN_1`"]
    #[inline(always)]
    pub fn is_bnk92_en_1(&self) -> bool {
        *self == BNK92_EN_A::BNK92_EN_1
    }
}
#[doc = "Field `BNK92_EN` writer - When 1, enables Bank92 of the SRAM"]
pub type BNK92_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK92_EN_A, O>;
impl<'a, const O: u8> BNK92_EN_W<'a, O> {
    #[doc = "Disables Bank92 of the SRAM"]
    #[inline(always)]
    pub fn bnk92_en_0(self) -> &'a mut W {
        self.variant(BNK92_EN_A::BNK92_EN_0)
    }
    #[doc = "Enables Bank92 of the SRAM"]
    #[inline(always)]
    pub fn bnk92_en_1(self) -> &'a mut W {
        self.variant(BNK92_EN_A::BNK92_EN_1)
    }
}
#[doc = "Field `BNK93_EN` reader - When 1, enables Bank93 of the SRAM"]
pub type BNK93_EN_R = crate::BitReader<BNK93_EN_A>;
#[doc = "When 1, enables Bank93 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK93_EN_A {
    #[doc = "0: Disables Bank93 of the SRAM"]
    BNK93_EN_0 = 0,
    #[doc = "1: Enables Bank93 of the SRAM"]
    BNK93_EN_1 = 1,
}
impl From<BNK93_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK93_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK93_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK93_EN_A {
        match self.bits {
            false => BNK93_EN_A::BNK93_EN_0,
            true => BNK93_EN_A::BNK93_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK93_EN_0`"]
    #[inline(always)]
    pub fn is_bnk93_en_0(&self) -> bool {
        *self == BNK93_EN_A::BNK93_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK93_EN_1`"]
    #[inline(always)]
    pub fn is_bnk93_en_1(&self) -> bool {
        *self == BNK93_EN_A::BNK93_EN_1
    }
}
#[doc = "Field `BNK93_EN` writer - When 1, enables Bank93 of the SRAM"]
pub type BNK93_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK93_EN_A, O>;
impl<'a, const O: u8> BNK93_EN_W<'a, O> {
    #[doc = "Disables Bank93 of the SRAM"]
    #[inline(always)]
    pub fn bnk93_en_0(self) -> &'a mut W {
        self.variant(BNK93_EN_A::BNK93_EN_0)
    }
    #[doc = "Enables Bank93 of the SRAM"]
    #[inline(always)]
    pub fn bnk93_en_1(self) -> &'a mut W {
        self.variant(BNK93_EN_A::BNK93_EN_1)
    }
}
#[doc = "Field `BNK94_EN` reader - When 1, enables Bank94 of the SRAM"]
pub type BNK94_EN_R = crate::BitReader<BNK94_EN_A>;
#[doc = "When 1, enables Bank94 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK94_EN_A {
    #[doc = "0: Disables Bank94 of the SRAM"]
    BNK94_EN_0 = 0,
    #[doc = "1: Enables Bank94 of the SRAM"]
    BNK94_EN_1 = 1,
}
impl From<BNK94_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK94_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK94_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK94_EN_A {
        match self.bits {
            false => BNK94_EN_A::BNK94_EN_0,
            true => BNK94_EN_A::BNK94_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK94_EN_0`"]
    #[inline(always)]
    pub fn is_bnk94_en_0(&self) -> bool {
        *self == BNK94_EN_A::BNK94_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK94_EN_1`"]
    #[inline(always)]
    pub fn is_bnk94_en_1(&self) -> bool {
        *self == BNK94_EN_A::BNK94_EN_1
    }
}
#[doc = "Field `BNK94_EN` writer - When 1, enables Bank94 of the SRAM"]
pub type BNK94_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK94_EN_A, O>;
impl<'a, const O: u8> BNK94_EN_W<'a, O> {
    #[doc = "Disables Bank94 of the SRAM"]
    #[inline(always)]
    pub fn bnk94_en_0(self) -> &'a mut W {
        self.variant(BNK94_EN_A::BNK94_EN_0)
    }
    #[doc = "Enables Bank94 of the SRAM"]
    #[inline(always)]
    pub fn bnk94_en_1(self) -> &'a mut W {
        self.variant(BNK94_EN_A::BNK94_EN_1)
    }
}
#[doc = "Field `BNK95_EN` reader - When 1, enables Bank95 of the SRAM"]
pub type BNK95_EN_R = crate::BitReader<BNK95_EN_A>;
#[doc = "When 1, enables Bank95 of the SRAM\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BNK95_EN_A {
    #[doc = "0: Disables Bank95 of the SRAM"]
    BNK95_EN_0 = 0,
    #[doc = "1: Enables Bank95 of the SRAM"]
    BNK95_EN_1 = 1,
}
impl From<BNK95_EN_A> for bool {
    #[inline(always)]
    fn from(variant: BNK95_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl BNK95_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK95_EN_A {
        match self.bits {
            false => BNK95_EN_A::BNK95_EN_0,
            true => BNK95_EN_A::BNK95_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK95_EN_0`"]
    #[inline(always)]
    pub fn is_bnk95_en_0(&self) -> bool {
        *self == BNK95_EN_A::BNK95_EN_0
    }
    #[doc = "Checks if the value of the field is `BNK95_EN_1`"]
    #[inline(always)]
    pub fn is_bnk95_en_1(&self) -> bool {
        *self == BNK95_EN_A::BNK95_EN_1
    }
}
#[doc = "Field `BNK95_EN` writer - When 1, enables Bank95 of the SRAM"]
pub type BNK95_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYS_SRAM_BANKEN_CTL2_SPEC, BNK95_EN_A, O>;
impl<'a, const O: u8> BNK95_EN_W<'a, O> {
    #[doc = "Disables Bank95 of the SRAM"]
    #[inline(always)]
    pub fn bnk95_en_0(self) -> &'a mut W {
        self.variant(BNK95_EN_A::BNK95_EN_0)
    }
    #[doc = "Enables Bank95 of the SRAM"]
    #[inline(always)]
    pub fn bnk95_en_1(self) -> &'a mut W {
        self.variant(BNK95_EN_A::BNK95_EN_1)
    }
}
impl R {
    #[doc = "Bit 0 - When 1, enables Bank64 of the SRAM"]
    #[inline(always)]
    pub fn bnk64_en(&self) -> BNK64_EN_R {
        BNK64_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, enables Bank65 of the SRAM"]
    #[inline(always)]
    pub fn bnk65_en(&self) -> BNK65_EN_R {
        BNK65_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When 1, enables Bank66 of the SRAM"]
    #[inline(always)]
    pub fn bnk66_en(&self) -> BNK66_EN_R {
        BNK66_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When 1, enables Bank67 of the SRAM"]
    #[inline(always)]
    pub fn bnk67_en(&self) -> BNK67_EN_R {
        BNK67_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When 1, enables Bank68 of the SRAM"]
    #[inline(always)]
    pub fn bnk68_en(&self) -> BNK68_EN_R {
        BNK68_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When 1, enables Bank69 of the SRAM"]
    #[inline(always)]
    pub fn bnk69_en(&self) -> BNK69_EN_R {
        BNK69_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When 1, enables Bank70 of the SRAM"]
    #[inline(always)]
    pub fn bnk70_en(&self) -> BNK70_EN_R {
        BNK70_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When 1, enables Bank71 of the SRAM"]
    #[inline(always)]
    pub fn bnk71_en(&self) -> BNK71_EN_R {
        BNK71_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - When 1, enables Bank72 of the SRAM"]
    #[inline(always)]
    pub fn bnk72_en(&self) -> BNK72_EN_R {
        BNK72_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When 1, enables Bank73 of the SRAM"]
    #[inline(always)]
    pub fn bnk73_en(&self) -> BNK73_EN_R {
        BNK73_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When 1, enables Bank74 of the SRAM"]
    #[inline(always)]
    pub fn bnk74_en(&self) -> BNK74_EN_R {
        BNK74_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When 1, enables Bank75 of the SRAM"]
    #[inline(always)]
    pub fn bnk75_en(&self) -> BNK75_EN_R {
        BNK75_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - When 1, enables Bank76 of the SRAM"]
    #[inline(always)]
    pub fn bnk76_en(&self) -> BNK76_EN_R {
        BNK76_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - When 1, enables Bank77 of the SRAM"]
    #[inline(always)]
    pub fn bnk77_en(&self) -> BNK77_EN_R {
        BNK77_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - When 1, enables Bank78 of the SRAM"]
    #[inline(always)]
    pub fn bnk78_en(&self) -> BNK78_EN_R {
        BNK78_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - When 1, enables Bank79 of the SRAM"]
    #[inline(always)]
    pub fn bnk79_en(&self) -> BNK79_EN_R {
        BNK79_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - When 1, enables Bank80 of the SRAM"]
    #[inline(always)]
    pub fn bnk80_en(&self) -> BNK80_EN_R {
        BNK80_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When 1, enables Bank81 of the SRAM"]
    #[inline(always)]
    pub fn bnk81_en(&self) -> BNK81_EN_R {
        BNK81_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - When 1, enables Bank82 of the SRAM"]
    #[inline(always)]
    pub fn bnk82_en(&self) -> BNK82_EN_R {
        BNK82_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - When 1, enables Bank83 of the SRAM"]
    #[inline(always)]
    pub fn bnk83_en(&self) -> BNK83_EN_R {
        BNK83_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - When 1, enables Bank84 of the SRAM"]
    #[inline(always)]
    pub fn bnk84_en(&self) -> BNK84_EN_R {
        BNK84_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - When 1, enables Bank85 of the SRAM"]
    #[inline(always)]
    pub fn bnk85_en(&self) -> BNK85_EN_R {
        BNK85_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - When 1, enables Bank86 of the SRAM"]
    #[inline(always)]
    pub fn bnk86_en(&self) -> BNK86_EN_R {
        BNK86_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - When 1, enables Bank87 of the SRAM"]
    #[inline(always)]
    pub fn bnk87_en(&self) -> BNK87_EN_R {
        BNK87_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - When 1, enables Bank88 of the SRAM"]
    #[inline(always)]
    pub fn bnk88_en(&self) -> BNK88_EN_R {
        BNK88_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - When 1, enables Bank89 of the SRAM"]
    #[inline(always)]
    pub fn bnk89_en(&self) -> BNK89_EN_R {
        BNK89_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - When 1, enables Bank90 of the SRAM"]
    #[inline(always)]
    pub fn bnk90_en(&self) -> BNK90_EN_R {
        BNK90_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - When 1, enables Bank91 of the SRAM"]
    #[inline(always)]
    pub fn bnk91_en(&self) -> BNK91_EN_R {
        BNK91_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - When 1, enables Bank92 of the SRAM"]
    #[inline(always)]
    pub fn bnk92_en(&self) -> BNK92_EN_R {
        BNK92_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - When 1, enables Bank93 of the SRAM"]
    #[inline(always)]
    pub fn bnk93_en(&self) -> BNK93_EN_R {
        BNK93_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - When 1, enables Bank94 of the SRAM"]
    #[inline(always)]
    pub fn bnk94_en(&self) -> BNK94_EN_R {
        BNK94_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When 1, enables Bank95 of the SRAM"]
    #[inline(always)]
    pub fn bnk95_en(&self) -> BNK95_EN_R {
        BNK95_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, enables Bank64 of the SRAM"]
    #[inline(always)]
    pub fn bnk64_en(&mut self) -> BNK64_EN_W<0> {
        BNK64_EN_W::new(self)
    }
    #[doc = "Bit 1 - When 1, enables Bank65 of the SRAM"]
    #[inline(always)]
    pub fn bnk65_en(&mut self) -> BNK65_EN_W<1> {
        BNK65_EN_W::new(self)
    }
    #[doc = "Bit 2 - When 1, enables Bank66 of the SRAM"]
    #[inline(always)]
    pub fn bnk66_en(&mut self) -> BNK66_EN_W<2> {
        BNK66_EN_W::new(self)
    }
    #[doc = "Bit 3 - When 1, enables Bank67 of the SRAM"]
    #[inline(always)]
    pub fn bnk67_en(&mut self) -> BNK67_EN_W<3> {
        BNK67_EN_W::new(self)
    }
    #[doc = "Bit 4 - When 1, enables Bank68 of the SRAM"]
    #[inline(always)]
    pub fn bnk68_en(&mut self) -> BNK68_EN_W<4> {
        BNK68_EN_W::new(self)
    }
    #[doc = "Bit 5 - When 1, enables Bank69 of the SRAM"]
    #[inline(always)]
    pub fn bnk69_en(&mut self) -> BNK69_EN_W<5> {
        BNK69_EN_W::new(self)
    }
    #[doc = "Bit 6 - When 1, enables Bank70 of the SRAM"]
    #[inline(always)]
    pub fn bnk70_en(&mut self) -> BNK70_EN_W<6> {
        BNK70_EN_W::new(self)
    }
    #[doc = "Bit 7 - When 1, enables Bank71 of the SRAM"]
    #[inline(always)]
    pub fn bnk71_en(&mut self) -> BNK71_EN_W<7> {
        BNK71_EN_W::new(self)
    }
    #[doc = "Bit 8 - When 1, enables Bank72 of the SRAM"]
    #[inline(always)]
    pub fn bnk72_en(&mut self) -> BNK72_EN_W<8> {
        BNK72_EN_W::new(self)
    }
    #[doc = "Bit 9 - When 1, enables Bank73 of the SRAM"]
    #[inline(always)]
    pub fn bnk73_en(&mut self) -> BNK73_EN_W<9> {
        BNK73_EN_W::new(self)
    }
    #[doc = "Bit 10 - When 1, enables Bank74 of the SRAM"]
    #[inline(always)]
    pub fn bnk74_en(&mut self) -> BNK74_EN_W<10> {
        BNK74_EN_W::new(self)
    }
    #[doc = "Bit 11 - When 1, enables Bank75 of the SRAM"]
    #[inline(always)]
    pub fn bnk75_en(&mut self) -> BNK75_EN_W<11> {
        BNK75_EN_W::new(self)
    }
    #[doc = "Bit 12 - When 1, enables Bank76 of the SRAM"]
    #[inline(always)]
    pub fn bnk76_en(&mut self) -> BNK76_EN_W<12> {
        BNK76_EN_W::new(self)
    }
    #[doc = "Bit 13 - When 1, enables Bank77 of the SRAM"]
    #[inline(always)]
    pub fn bnk77_en(&mut self) -> BNK77_EN_W<13> {
        BNK77_EN_W::new(self)
    }
    #[doc = "Bit 14 - When 1, enables Bank78 of the SRAM"]
    #[inline(always)]
    pub fn bnk78_en(&mut self) -> BNK78_EN_W<14> {
        BNK78_EN_W::new(self)
    }
    #[doc = "Bit 15 - When 1, enables Bank79 of the SRAM"]
    #[inline(always)]
    pub fn bnk79_en(&mut self) -> BNK79_EN_W<15> {
        BNK79_EN_W::new(self)
    }
    #[doc = "Bit 16 - When 1, enables Bank80 of the SRAM"]
    #[inline(always)]
    pub fn bnk80_en(&mut self) -> BNK80_EN_W<16> {
        BNK80_EN_W::new(self)
    }
    #[doc = "Bit 17 - When 1, enables Bank81 of the SRAM"]
    #[inline(always)]
    pub fn bnk81_en(&mut self) -> BNK81_EN_W<17> {
        BNK81_EN_W::new(self)
    }
    #[doc = "Bit 18 - When 1, enables Bank82 of the SRAM"]
    #[inline(always)]
    pub fn bnk82_en(&mut self) -> BNK82_EN_W<18> {
        BNK82_EN_W::new(self)
    }
    #[doc = "Bit 19 - When 1, enables Bank83 of the SRAM"]
    #[inline(always)]
    pub fn bnk83_en(&mut self) -> BNK83_EN_W<19> {
        BNK83_EN_W::new(self)
    }
    #[doc = "Bit 20 - When 1, enables Bank84 of the SRAM"]
    #[inline(always)]
    pub fn bnk84_en(&mut self) -> BNK84_EN_W<20> {
        BNK84_EN_W::new(self)
    }
    #[doc = "Bit 21 - When 1, enables Bank85 of the SRAM"]
    #[inline(always)]
    pub fn bnk85_en(&mut self) -> BNK85_EN_W<21> {
        BNK85_EN_W::new(self)
    }
    #[doc = "Bit 22 - When 1, enables Bank86 of the SRAM"]
    #[inline(always)]
    pub fn bnk86_en(&mut self) -> BNK86_EN_W<22> {
        BNK86_EN_W::new(self)
    }
    #[doc = "Bit 23 - When 1, enables Bank87 of the SRAM"]
    #[inline(always)]
    pub fn bnk87_en(&mut self) -> BNK87_EN_W<23> {
        BNK87_EN_W::new(self)
    }
    #[doc = "Bit 24 - When 1, enables Bank88 of the SRAM"]
    #[inline(always)]
    pub fn bnk88_en(&mut self) -> BNK88_EN_W<24> {
        BNK88_EN_W::new(self)
    }
    #[doc = "Bit 25 - When 1, enables Bank89 of the SRAM"]
    #[inline(always)]
    pub fn bnk89_en(&mut self) -> BNK89_EN_W<25> {
        BNK89_EN_W::new(self)
    }
    #[doc = "Bit 26 - When 1, enables Bank90 of the SRAM"]
    #[inline(always)]
    pub fn bnk90_en(&mut self) -> BNK90_EN_W<26> {
        BNK90_EN_W::new(self)
    }
    #[doc = "Bit 27 - When 1, enables Bank91 of the SRAM"]
    #[inline(always)]
    pub fn bnk91_en(&mut self) -> BNK91_EN_W<27> {
        BNK91_EN_W::new(self)
    }
    #[doc = "Bit 28 - When 1, enables Bank92 of the SRAM"]
    #[inline(always)]
    pub fn bnk92_en(&mut self) -> BNK92_EN_W<28> {
        BNK92_EN_W::new(self)
    }
    #[doc = "Bit 29 - When 1, enables Bank93 of the SRAM"]
    #[inline(always)]
    pub fn bnk93_en(&mut self) -> BNK93_EN_W<29> {
        BNK93_EN_W::new(self)
    }
    #[doc = "Bit 30 - When 1, enables Bank94 of the SRAM"]
    #[inline(always)]
    pub fn bnk94_en(&mut self) -> BNK94_EN_W<30> {
        BNK94_EN_W::new(self)
    }
    #[doc = "Bit 31 - When 1, enables Bank95 of the SRAM"]
    #[inline(always)]
    pub fn bnk95_en(&mut self) -> BNK95_EN_W<31> {
        BNK95_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM Bank Enable Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_sram_banken_ctl2](index.html) module"]
pub struct SYS_SRAM_BANKEN_CTL2_SPEC;
impl crate::RegisterSpec for SYS_SRAM_BANKEN_CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_sram_banken_ctl2::R](R) reader structure"]
impl crate::Readable for SYS_SRAM_BANKEN_CTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_sram_banken_ctl2::W](W) writer structure"]
impl crate::Writable for SYS_SRAM_BANKEN_CTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_SRAM_BANKEN_CTL2 to value 0xffff_ffff"]
impl crate::Resettable for SYS_SRAM_BANKEN_CTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
