#[doc = "Register `ADC14_REF1P45V_TS85C` reader"]
pub struct R(crate::R<ADC14_REF1P45V_TS85C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC14_REF1P45V_TS85C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC14_REF1P45V_TS85C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC14_REF1P45V_TS85C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "ADC14 1.45V Reference Temp. Sensor 85C\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14_ref1p45v_ts85c](index.html) module"]
pub struct ADC14_REF1P45V_TS85C_SPEC;
impl crate::RegisterSpec for ADC14_REF1P45V_TS85C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc14_ref1p45v_ts85c::R](R) reader structure"]
impl crate::Readable for ADC14_REF1P45V_TS85C_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADC14_REF1P45V_TS85C to value 0"]
impl crate::Resettable for ADC14_REF1P45V_TS85C_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
