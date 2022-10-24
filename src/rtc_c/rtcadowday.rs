#[doc = "Register `RTCADOWDAY` reader"]
pub struct R(crate::R<RTCADOWDAY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCADOWDAY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCADOWDAY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCADOWDAY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCADOWDAY` writer"]
pub struct W(crate::W<RTCADOWDAY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCADOWDAY_SPEC>;
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
impl From<crate::W<RTCADOWDAY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCADOWDAY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DayofWeek` reader - Day of week (0 to 6)"]
pub type DAYOF_WEEK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DayofWeek` writer - Day of week (0 to 6)"]
pub type DAYOF_WEEK_W<'a, const O: u8> = crate::FieldWriter<'a, u16, RTCADOWDAY_SPEC, u8, u8, 3, O>;
#[doc = "Field `DOWAE` reader - Alarm enable"]
pub type DOWAE_R = crate::BitReader<bool>;
#[doc = "Field `DOWAE` writer - Alarm enable"]
pub type DOWAE_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCADOWDAY_SPEC, bool, O>;
#[doc = "Field `DayofMonth` reader - Day of month (1 to 28, 29, 30, 31)"]
pub type DAYOF_MONTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DayofMonth` writer - Day of month (1 to 28, 29, 30, 31)"]
pub type DAYOF_MONTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, RTCADOWDAY_SPEC, u8, u8, 5, O>;
#[doc = "Field `DAYAE` reader - Alarm enable"]
pub type DAYAE_R = crate::BitReader<bool>;
#[doc = "Field `DAYAE` writer - Alarm enable"]
pub type DAYAE_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCADOWDAY_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Day of week (0 to 6)"]
    #[inline(always)]
    pub fn dayof_week(&self) -> DAYOF_WEEK_R {
        DAYOF_WEEK_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 7 - Alarm enable"]
    #[inline(always)]
    pub fn dowae(&self) -> DOWAE_R {
        DOWAE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Day of month (1 to 28, 29, 30, 31)"]
    #[inline(always)]
    pub fn dayof_month(&self) -> DAYOF_MONTH_R {
        DAYOF_MONTH_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Alarm enable"]
    #[inline(always)]
    pub fn dayae(&self) -> DAYAE_R {
        DAYAE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Day of week (0 to 6)"]
    #[inline(always)]
    pub fn dayof_week(&mut self) -> DAYOF_WEEK_W<0> {
        DAYOF_WEEK_W::new(self)
    }
    #[doc = "Bit 7 - Alarm enable"]
    #[inline(always)]
    pub fn dowae(&mut self) -> DOWAE_W<7> {
        DOWAE_W::new(self)
    }
    #[doc = "Bits 8:12 - Day of month (1 to 28, 29, 30, 31)"]
    #[inline(always)]
    pub fn dayof_month(&mut self) -> DAYOF_MONTH_W<8> {
        DAYOF_MONTH_W::new(self)
    }
    #[doc = "Bit 15 - Alarm enable"]
    #[inline(always)]
    pub fn dayae(&mut self) -> DAYAE_W<15> {
        DAYAE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTCADOWDAY - Hexadecimal Format\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcadowday](index.html) module"]
pub struct RTCADOWDAY_SPEC;
impl crate::RegisterSpec for RTCADOWDAY_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtcadowday::R](R) reader structure"]
impl crate::Readable for RTCADOWDAY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcadowday::W](W) writer structure"]
impl crate::Writable for RTCADOWDAY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCADOWDAY to value 0"]
impl crate::Resettable for RTCADOWDAY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
