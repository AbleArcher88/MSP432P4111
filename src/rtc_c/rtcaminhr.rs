#[doc = "Register `RTCAMINHR` reader"]
pub struct R(crate::R<RTCAMINHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCAMINHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCAMINHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCAMINHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCAMINHR` writer"]
pub struct W(crate::W<RTCAMINHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCAMINHR_SPEC>;
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
impl From<crate::W<RTCAMINHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCAMINHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Minutes` reader - Minutes (0 to 59)"]
pub type MINUTES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Minutes` writer - Minutes (0 to 59)"]
pub type MINUTES_W<'a, const O: u8> = crate::FieldWriter<'a, u16, RTCAMINHR_SPEC, u8, u8, 6, O>;
#[doc = "Field `MINAE` reader - Alarm enable"]
pub type MINAE_R = crate::BitReader<bool>;
#[doc = "Field `MINAE` writer - Alarm enable"]
pub type MINAE_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCAMINHR_SPEC, bool, O>;
#[doc = "Field `Hours` reader - Hours (0 to 23)"]
pub type HOURS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Hours` writer - Hours (0 to 23)"]
pub type HOURS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, RTCAMINHR_SPEC, u8, u8, 5, O>;
#[doc = "Field `HOURAE` reader - Alarm enable"]
pub type HOURAE_R = crate::BitReader<bool>;
#[doc = "Field `HOURAE` writer - Alarm enable"]
pub type HOURAE_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCAMINHR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - Minutes (0 to 59)"]
    #[inline(always)]
    pub fn minutes(&self) -> MINUTES_R {
        MINUTES_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Alarm enable"]
    #[inline(always)]
    pub fn minae(&self) -> MINAE_R {
        MINAE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Hours (0 to 23)"]
    #[inline(always)]
    pub fn hours(&self) -> HOURS_R {
        HOURS_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Alarm enable"]
    #[inline(always)]
    pub fn hourae(&self) -> HOURAE_R {
        HOURAE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Minutes (0 to 59)"]
    #[inline(always)]
    pub fn minutes(&mut self) -> MINUTES_W<0> {
        MINUTES_W::new(self)
    }
    #[doc = "Bit 7 - Alarm enable"]
    #[inline(always)]
    pub fn minae(&mut self) -> MINAE_W<7> {
        MINAE_W::new(self)
    }
    #[doc = "Bits 8:12 - Hours (0 to 23)"]
    #[inline(always)]
    pub fn hours(&mut self) -> HOURS_W<8> {
        HOURS_W::new(self)
    }
    #[doc = "Bit 15 - Alarm enable"]
    #[inline(always)]
    pub fn hourae(&mut self) -> HOURAE_W<15> {
        HOURAE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTCMINHR - Hexadecimal Format\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcaminhr](index.html) module"]
pub struct RTCAMINHR_SPEC;
impl crate::RegisterSpec for RTCAMINHR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtcaminhr::R](R) reader structure"]
impl crate::Readable for RTCAMINHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcaminhr::W](W) writer structure"]
impl crate::Writable for RTCAMINHR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCAMINHR to value 0"]
impl crate::Resettable for RTCAMINHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
