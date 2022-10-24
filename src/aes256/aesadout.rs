#[doc = "Register `AESADOUT` writer"]
pub struct W(crate::W<AESADOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESADOUT_SPEC>;
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
impl From<crate::W<AESADOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESADOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AESDOUT0x` writer - AES data out byte n when AESADOUT is read as half-word"]
pub type AESDOUT0X_W<'a, const O: u8> = crate::FieldWriter<'a, u16, AESADOUT_SPEC, u8, u8, 8, O>;
#[doc = "Field `AESDOUT1x` writer - AES data out byte n+1 when AESADOUT is read as half-word"]
pub type AESDOUT1X_W<'a, const O: u8> = crate::FieldWriter<'a, u16, AESADOUT_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - AES data out byte n when AESADOUT is read as half-word"]
    #[inline(always)]
    pub fn aesdout0x(&mut self) -> AESDOUT0X_W<0> {
        AESDOUT0X_W::new(self)
    }
    #[doc = "Bits 8:15 - AES data out byte n+1 when AESADOUT is read as half-word"]
    #[inline(always)]
    pub fn aesdout1x(&mut self) -> AESDOUT1X_W<8> {
        AESDOUT1X_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Accelerator Data Out Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesadout](index.html) module"]
pub struct AESADOUT_SPEC;
impl crate::RegisterSpec for AESADOUT_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [aesadout::W](W) writer structure"]
impl crate::Writable for AESADOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AESADOUT to value 0"]
impl crate::Resettable for AESADOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
