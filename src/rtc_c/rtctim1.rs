#[doc = "Register `RTCTIM1` reader"]
pub struct R(crate::R<RTCTIM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCTIM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCTIM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCTIM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCTIM1` writer"]
pub struct W(crate::W<RTCTIM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCTIM1_SPEC>;
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
impl From<crate::W<RTCTIM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCTIM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Hours` reader - Hours (0 to 23)"]
pub type HOURS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Hours` writer - Hours (0 to 23)"]
pub type HOURS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, RTCTIM1_SPEC, u8, u8, 5, O>;
#[doc = "Field `DayofWeek` reader - Day of week (0 to 6)"]
pub type DAYOF_WEEK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DayofWeek` writer - Day of week (0 to 6)"]
pub type DAYOF_WEEK_W<'a, const O: u8> = crate::FieldWriter<'a, u16, RTCTIM1_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:4 - Hours (0 to 23)"]
    #[inline(always)]
    pub fn hours(&self) -> HOURS_R {
        HOURS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - Day of week (0 to 6)"]
    #[inline(always)]
    pub fn dayof_week(&self) -> DAYOF_WEEK_R {
        DAYOF_WEEK_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Hours (0 to 23)"]
    #[inline(always)]
    pub fn hours(&mut self) -> HOURS_W<0> {
        HOURS_W::new(self)
    }
    #[doc = "Bits 8:10 - Day of week (0 to 6)"]
    #[inline(always)]
    pub fn dayof_week(&mut self) -> DAYOF_WEEK_W<8> {
        DAYOF_WEEK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Real-Time Clock Hour, Day of Week\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtctim1](index.html) module"]
pub struct RTCTIM1_SPEC;
impl crate::RegisterSpec for RTCTIM1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtctim1::R](R) reader structure"]
impl crate::Readable for RTCTIM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtctim1::W](W) writer structure"]
impl crate::Writable for RTCTIM1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCTIM1 to value 0"]
impl crate::Resettable for RTCTIM1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
