#[doc = "Register `DMA_INT0_CLRFLG` writer"]
pub struct W(crate::W<DMA_INT0_CLRFLG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_INT0_CLRFLG_SPEC>;
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
impl From<crate::W<DMA_INT0_CLRFLG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_INT0_CLRFLG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH1` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH2` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH3` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH4` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH5` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH6` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH7` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH8` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH8_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH9` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH9_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH10` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH10_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH11` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH11_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH12` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH12_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH13` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH13_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH14` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH14_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH15` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH15_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH16` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH16_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH17` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH17_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH18` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH18_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH19` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH19_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH20` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH20_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH21` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH21_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH22` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH22_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH23` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH23_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH24` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH24_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH25` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH25_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH26` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH26_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH27` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH27_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH28` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH28_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH29` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH29_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH30` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH30_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
#[doc = "Field `CH31` writer - Clear corresponding DMA_INT0_SRCFLG_REG"]
pub type CH31_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT0_CLRFLG_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W<0> {
        CH0_W::new(self)
    }
    #[doc = "Bit 1 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH1_W<1> {
        CH1_W::new(self)
    }
    #[doc = "Bit 2 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch2(&mut self) -> CH2_W<2> {
        CH2_W::new(self)
    }
    #[doc = "Bit 3 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch3(&mut self) -> CH3_W<3> {
        CH3_W::new(self)
    }
    #[doc = "Bit 4 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch4(&mut self) -> CH4_W<4> {
        CH4_W::new(self)
    }
    #[doc = "Bit 5 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch5(&mut self) -> CH5_W<5> {
        CH5_W::new(self)
    }
    #[doc = "Bit 6 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch6(&mut self) -> CH6_W<6> {
        CH6_W::new(self)
    }
    #[doc = "Bit 7 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch7(&mut self) -> CH7_W<7> {
        CH7_W::new(self)
    }
    #[doc = "Bit 8 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch8(&mut self) -> CH8_W<8> {
        CH8_W::new(self)
    }
    #[doc = "Bit 9 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch9(&mut self) -> CH9_W<9> {
        CH9_W::new(self)
    }
    #[doc = "Bit 10 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch10(&mut self) -> CH10_W<10> {
        CH10_W::new(self)
    }
    #[doc = "Bit 11 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch11(&mut self) -> CH11_W<11> {
        CH11_W::new(self)
    }
    #[doc = "Bit 12 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch12(&mut self) -> CH12_W<12> {
        CH12_W::new(self)
    }
    #[doc = "Bit 13 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch13(&mut self) -> CH13_W<13> {
        CH13_W::new(self)
    }
    #[doc = "Bit 14 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch14(&mut self) -> CH14_W<14> {
        CH14_W::new(self)
    }
    #[doc = "Bit 15 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch15(&mut self) -> CH15_W<15> {
        CH15_W::new(self)
    }
    #[doc = "Bit 16 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch16(&mut self) -> CH16_W<16> {
        CH16_W::new(self)
    }
    #[doc = "Bit 17 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch17(&mut self) -> CH17_W<17> {
        CH17_W::new(self)
    }
    #[doc = "Bit 18 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch18(&mut self) -> CH18_W<18> {
        CH18_W::new(self)
    }
    #[doc = "Bit 19 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch19(&mut self) -> CH19_W<19> {
        CH19_W::new(self)
    }
    #[doc = "Bit 20 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch20(&mut self) -> CH20_W<20> {
        CH20_W::new(self)
    }
    #[doc = "Bit 21 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch21(&mut self) -> CH21_W<21> {
        CH21_W::new(self)
    }
    #[doc = "Bit 22 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch22(&mut self) -> CH22_W<22> {
        CH22_W::new(self)
    }
    #[doc = "Bit 23 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch23(&mut self) -> CH23_W<23> {
        CH23_W::new(self)
    }
    #[doc = "Bit 24 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch24(&mut self) -> CH24_W<24> {
        CH24_W::new(self)
    }
    #[doc = "Bit 25 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch25(&mut self) -> CH25_W<25> {
        CH25_W::new(self)
    }
    #[doc = "Bit 26 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch26(&mut self) -> CH26_W<26> {
        CH26_W::new(self)
    }
    #[doc = "Bit 27 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch27(&mut self) -> CH27_W<27> {
        CH27_W::new(self)
    }
    #[doc = "Bit 28 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch28(&mut self) -> CH28_W<28> {
        CH28_W::new(self)
    }
    #[doc = "Bit 29 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch29(&mut self) -> CH29_W<29> {
        CH29_W::new(self)
    }
    #[doc = "Bit 30 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch30(&mut self) -> CH30_W<30> {
        CH30_W::new(self)
    }
    #[doc = "Bit 31 - Clear corresponding DMA_INT0_SRCFLG_REG"]
    #[inline(always)]
    pub fn ch31(&mut self) -> CH31_W<31> {
        CH31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt 0 Source Channel Clear Flag Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int0_clrflg](index.html) module"]
pub struct DMA_INT0_CLRFLG_SPEC;
impl crate::RegisterSpec for DMA_INT0_CLRFLG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dma_int0_clrflg::W](W) writer structure"]
impl crate::Writable for DMA_INT0_CLRFLG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_INT0_CLRFLG to value 0"]
impl crate::Resettable for DMA_INT0_CLRFLG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
