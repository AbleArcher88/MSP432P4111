#[doc = "Register `RSTCTL_SOFTRESET_CLR` reader"]
pub struct R(crate::R<RSTCTL_SOFTRESET_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCTL_SOFTRESET_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCTL_SOFTRESET_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCTL_SOFTRESET_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTCTL_SOFTRESET_CLR` writer"]
pub struct W(crate::W<RSTCTL_SOFTRESET_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTCTL_SOFTRESET_CLR_SPEC>;
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
impl From<crate::W<RSTCTL_SOFTRESET_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTCTL_SOFTRESET_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRC0` writer - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
pub type SRC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTL_SOFTRESET_CLR_SPEC, bool, O>;
#[doc = "Field `SRC1` writer - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
pub type SRC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTL_SOFTRESET_CLR_SPEC, bool, O>;
#[doc = "Field `SRC2` writer - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
pub type SRC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTL_SOFTRESET_CLR_SPEC, bool, O>;
#[doc = "Field `SRC3` writer - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
pub type SRC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTL_SOFTRESET_CLR_SPEC, bool, O>;
#[doc = "Field `SRC4` writer - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
pub type SRC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTL_SOFTRESET_CLR_SPEC, bool, O>;
#[doc = "Field `SRC5` writer - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
pub type SRC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTL_SOFTRESET_CLR_SPEC, bool, O>;
#[doc = "Field `SRC6` writer - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
pub type SRC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTL_SOFTRESET_CLR_SPEC, bool, O>;
#[doc = "Field `SRC7` writer - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
pub type SRC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTL_SOFTRESET_CLR_SPEC, bool, O>;
#[doc = "Field `SRC8` writer - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
pub type SRC8_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTL_SOFTRESET_CLR_SPEC, bool, O>;
#[doc = "Field `SRC9` writer - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
pub type SRC9_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTL_SOFTRESET_CLR_SPEC, bool, O>;
#[doc = "Field `SRC10` writer - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
pub type SRC10_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTL_SOFTRESET_CLR_SPEC, bool, O>;
#[doc = "Field `SRC11` writer - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
pub type SRC11_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTL_SOFTRESET_CLR_SPEC, bool, O>;
#[doc = "Field `SRC12` writer - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
pub type SRC12_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTL_SOFTRESET_CLR_SPEC, bool, O>;
#[doc = "Field `SRC13` writer - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
pub type SRC13_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTL_SOFTRESET_CLR_SPEC, bool, O>;
#[doc = "Field `SRC14` writer - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
pub type SRC14_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTL_SOFTRESET_CLR_SPEC, bool, O>;
#[doc = "Field `SRC15` writer - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
pub type SRC15_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSTCTL_SOFTRESET_CLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
    #[inline(always)]
    pub fn src0(&mut self) -> SRC0_W<0> {
        SRC0_W::new(self)
    }
    #[doc = "Bit 1 - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
    #[inline(always)]
    pub fn src1(&mut self) -> SRC1_W<1> {
        SRC1_W::new(self)
    }
    #[doc = "Bit 2 - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
    #[inline(always)]
    pub fn src2(&mut self) -> SRC2_W<2> {
        SRC2_W::new(self)
    }
    #[doc = "Bit 3 - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
    #[inline(always)]
    pub fn src3(&mut self) -> SRC3_W<3> {
        SRC3_W::new(self)
    }
    #[doc = "Bit 4 - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
    #[inline(always)]
    pub fn src4(&mut self) -> SRC4_W<4> {
        SRC4_W::new(self)
    }
    #[doc = "Bit 5 - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
    #[inline(always)]
    pub fn src5(&mut self) -> SRC5_W<5> {
        SRC5_W::new(self)
    }
    #[doc = "Bit 6 - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
    #[inline(always)]
    pub fn src6(&mut self) -> SRC6_W<6> {
        SRC6_W::new(self)
    }
    #[doc = "Bit 7 - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
    #[inline(always)]
    pub fn src7(&mut self) -> SRC7_W<7> {
        SRC7_W::new(self)
    }
    #[doc = "Bit 8 - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
    #[inline(always)]
    pub fn src8(&mut self) -> SRC8_W<8> {
        SRC8_W::new(self)
    }
    #[doc = "Bit 9 - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
    #[inline(always)]
    pub fn src9(&mut self) -> SRC9_W<9> {
        SRC9_W::new(self)
    }
    #[doc = "Bit 10 - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
    #[inline(always)]
    pub fn src10(&mut self) -> SRC10_W<10> {
        SRC10_W::new(self)
    }
    #[doc = "Bit 11 - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
    #[inline(always)]
    pub fn src11(&mut self) -> SRC11_W<11> {
        SRC11_W::new(self)
    }
    #[doc = "Bit 12 - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
    #[inline(always)]
    pub fn src12(&mut self) -> SRC12_W<12> {
        SRC12_W::new(self)
    }
    #[doc = "Bit 13 - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
    #[inline(always)]
    pub fn src13(&mut self) -> SRC13_W<13> {
        SRC13_W::new(self)
    }
    #[doc = "Bit 14 - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
    #[inline(always)]
    pub fn src14(&mut self) -> SRC14_W<14> {
        SRC14_W::new(self)
    }
    #[doc = "Bit 15 - Write 1 clears the corresponding bit in the RSTCTL_SOFTRESET_STAT"]
    #[inline(always)]
    pub fn src15(&mut self) -> SRC15_W<15> {
        SRC15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Soft Reset Status Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl_softreset_clr](index.html) module"]
pub struct RSTCTL_SOFTRESET_CLR_SPEC;
impl crate::RegisterSpec for RSTCTL_SOFTRESET_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstctl_softreset_clr::R](R) reader structure"]
impl crate::Readable for RSTCTL_SOFTRESET_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstctl_softreset_clr::W](W) writer structure"]
impl crate::Writable for RSTCTL_SOFTRESET_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSTCTL_SOFTRESET_CLR to value 0"]
impl crate::Resettable for RSTCTL_SOFTRESET_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
