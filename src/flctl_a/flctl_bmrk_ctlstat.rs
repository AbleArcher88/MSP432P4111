#[doc = "Register `FLCTL_BMRK_CTLSTAT` reader"]
pub struct R(crate::R<FLCTL_BMRK_CTLSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_BMRK_CTLSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_BMRK_CTLSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_BMRK_CTLSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLCTL_BMRK_CTLSTAT` writer"]
pub struct W(crate::W<FLCTL_BMRK_CTLSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLCTL_BMRK_CTLSTAT_SPEC>;
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
impl From<crate::W<FLCTL_BMRK_CTLSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLCTL_BMRK_CTLSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I_BMRK` reader - When 1, increments the Instruction Benchmark count register on each instruction fetch to the Flash"]
pub type I_BMRK_R = crate::BitReader<bool>;
#[doc = "Field `I_BMRK` writer - When 1, increments the Instruction Benchmark count register on each instruction fetch to the Flash"]
pub type I_BMRK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLCTL_BMRK_CTLSTAT_SPEC, bool, O>;
#[doc = "Field `D_BMRK` reader - When 1, increments the Data Benchmark count register on each data read access to the Flash"]
pub type D_BMRK_R = crate::BitReader<bool>;
#[doc = "Field `D_BMRK` writer - When 1, increments the Data Benchmark count register on each data read access to the Flash"]
pub type D_BMRK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLCTL_BMRK_CTLSTAT_SPEC, bool, O>;
#[doc = "Field `CMP_EN` reader - When 1, enables comparison of the Instruction or Data Benchmark Registers against the threshold value"]
pub type CMP_EN_R = crate::BitReader<bool>;
#[doc = "Field `CMP_EN` writer - When 1, enables comparison of the Instruction or Data Benchmark Registers against the threshold value"]
pub type CMP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLCTL_BMRK_CTLSTAT_SPEC, bool, O>;
#[doc = "Field `CMP_SEL` reader - Selects which benchmark register should be compared against the threshold"]
pub type CMP_SEL_R = crate::BitReader<CMP_SEL_A>;
#[doc = "Selects which benchmark register should be compared against the threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMP_SEL_A {
    #[doc = "0: Compares the Instruction Benchmark Register against the threshold value"]
    EN_1_0X0 = 0,
    #[doc = "1: Compares the Data Benchmark Register against the threshold value"]
    EN_2_0X1 = 1,
}
impl From<CMP_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CMP_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CMP_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP_SEL_A {
        match self.bits {
            false => CMP_SEL_A::EN_1_0X0,
            true => CMP_SEL_A::EN_2_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_1_0X0`"]
    #[inline(always)]
    pub fn is_en_1_0x0(&self) -> bool {
        *self == CMP_SEL_A::EN_1_0X0
    }
    #[doc = "Checks if the value of the field is `EN_2_0X1`"]
    #[inline(always)]
    pub fn is_en_2_0x1(&self) -> bool {
        *self == CMP_SEL_A::EN_2_0X1
    }
}
#[doc = "Field `CMP_SEL` writer - Selects which benchmark register should be compared against the threshold"]
pub type CMP_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_BMRK_CTLSTAT_SPEC, CMP_SEL_A, O>;
impl<'a, const O: u8> CMP_SEL_W<'a, O> {
    #[doc = "Compares the Instruction Benchmark Register against the threshold value"]
    #[inline(always)]
    pub fn en_1_0x0(self) -> &'a mut W {
        self.variant(CMP_SEL_A::EN_1_0X0)
    }
    #[doc = "Compares the Data Benchmark Register against the threshold value"]
    #[inline(always)]
    pub fn en_2_0x1(self) -> &'a mut W {
        self.variant(CMP_SEL_A::EN_2_0X1)
    }
}
impl R {
    #[doc = "Bit 0 - When 1, increments the Instruction Benchmark count register on each instruction fetch to the Flash"]
    #[inline(always)]
    pub fn i_bmrk(&self) -> I_BMRK_R {
        I_BMRK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, increments the Data Benchmark count register on each data read access to the Flash"]
    #[inline(always)]
    pub fn d_bmrk(&self) -> D_BMRK_R {
        D_BMRK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When 1, enables comparison of the Instruction or Data Benchmark Registers against the threshold value"]
    #[inline(always)]
    pub fn cmp_en(&self) -> CMP_EN_R {
        CMP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects which benchmark register should be compared against the threshold"]
    #[inline(always)]
    pub fn cmp_sel(&self) -> CMP_SEL_R {
        CMP_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, increments the Instruction Benchmark count register on each instruction fetch to the Flash"]
    #[inline(always)]
    pub fn i_bmrk(&mut self) -> I_BMRK_W<0> {
        I_BMRK_W::new(self)
    }
    #[doc = "Bit 1 - When 1, increments the Data Benchmark count register on each data read access to the Flash"]
    #[inline(always)]
    pub fn d_bmrk(&mut self) -> D_BMRK_W<1> {
        D_BMRK_W::new(self)
    }
    #[doc = "Bit 2 - When 1, enables comparison of the Instruction or Data Benchmark Registers against the threshold value"]
    #[inline(always)]
    pub fn cmp_en(&mut self) -> CMP_EN_W<2> {
        CMP_EN_W::new(self)
    }
    #[doc = "Bit 3 - Selects which benchmark register should be compared against the threshold"]
    #[inline(always)]
    pub fn cmp_sel(&mut self) -> CMP_SEL_W<3> {
        CMP_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Benchmark Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_bmrk_ctlstat](index.html) module"]
pub struct FLCTL_BMRK_CTLSTAT_SPEC;
impl crate::RegisterSpec for FLCTL_BMRK_CTLSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_bmrk_ctlstat::R](R) reader structure"]
impl crate::Readable for FLCTL_BMRK_CTLSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flctl_bmrk_ctlstat::W](W) writer structure"]
impl crate::Writable for FLCTL_BMRK_CTLSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLCTL_BMRK_CTLSTAT to value 0"]
impl crate::Resettable for FLCTL_BMRK_CTLSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
