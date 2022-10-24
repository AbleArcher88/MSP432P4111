#[doc = "Register `ADC14_CAL_TAG` reader"]
pub struct R(crate::R<ADC14_CAL_TAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC14_CAL_TAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC14_CAL_TAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC14_CAL_TAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "ADC14 Calibration Tag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14_cal_tag](index.html) module"]
pub struct ADC14_CAL_TAG_SPEC;
impl crate::RegisterSpec for ADC14_CAL_TAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc14_cal_tag::R](R) reader structure"]
impl crate::Readable for ADC14_CAL_TAG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADC14_CAL_TAG to value 0x05"]
impl crate::Resettable for ADC14_CAL_TAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x05
    }
}
