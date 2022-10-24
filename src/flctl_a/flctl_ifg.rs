#[doc = "Register `FLCTL_IFG` reader"]
pub struct R(crate::R<FLCTL_IFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_IFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_IFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_IFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDBRST` reader - If set to 1, indicates that the Read Burst/Compare operation is complete"]
pub type RDBRST_R = crate::BitReader<bool>;
#[doc = "Field `AVPRE` reader - If set to 1, indicates that the pre-program verify operation has detected an error"]
pub type AVPRE_R = crate::BitReader<bool>;
#[doc = "Field `AVPST` reader - If set to 1, indicates that the post-program verify operation has failed comparison"]
pub type AVPST_R = crate::BitReader<bool>;
#[doc = "Field `PRG` reader - If set to 1, indicates that a word Program operation is complete"]
pub type PRG_R = crate::BitReader<bool>;
#[doc = "Field `PRGB` reader - If set to 1, indicates that the configured Burst Program operation is complete"]
pub type PRGB_R = crate::BitReader<bool>;
#[doc = "Field `ERASE` reader - If set to 1, indicates that the Erase operation is complete"]
pub type ERASE_R = crate::BitReader<bool>;
#[doc = "Field `BMRK` reader - If set to 1, indicates that a Benchmark Compare match occurred"]
pub type BMRK_R = crate::BitReader<bool>;
#[doc = "Field `PRG_ERR` reader - If set to 1, indicates a word composition error in full word write mode (possible data loss due to writes crossing over to a new 128bit boundary before full word has been composed)"]
pub type PRG_ERR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - If set to 1, indicates that the Read Burst/Compare operation is complete"]
    #[inline(always)]
    pub fn rdbrst(&self) -> RDBRST_R {
        RDBRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If set to 1, indicates that the pre-program verify operation has detected an error"]
    #[inline(always)]
    pub fn avpre(&self) -> AVPRE_R {
        AVPRE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If set to 1, indicates that the post-program verify operation has failed comparison"]
    #[inline(always)]
    pub fn avpst(&self) -> AVPST_R {
        AVPST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If set to 1, indicates that a word Program operation is complete"]
    #[inline(always)]
    pub fn prg(&self) -> PRG_R {
        PRG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If set to 1, indicates that the configured Burst Program operation is complete"]
    #[inline(always)]
    pub fn prgb(&self) -> PRGB_R {
        PRGB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - If set to 1, indicates that the Erase operation is complete"]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - If set to 1, indicates that a Benchmark Compare match occurred"]
    #[inline(always)]
    pub fn bmrk(&self) -> BMRK_R {
        BMRK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - If set to 1, indicates a word composition error in full word write mode (possible data loss due to writes crossing over to a new 128bit boundary before full word has been composed)"]
    #[inline(always)]
    pub fn prg_err(&self) -> PRG_ERR_R {
        PRG_ERR_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_ifg](index.html) module"]
pub struct FLCTL_IFG_SPEC;
impl crate::RegisterSpec for FLCTL_IFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_ifg::R](R) reader structure"]
impl crate::Readable for FLCTL_IFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FLCTL_IFG to value 0"]
impl crate::Resettable for FLCTL_IFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
