#[doc = "Register `MVFR0` reader"]
pub struct R(crate::R<MVFR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MVFR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MVFR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MVFR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `A_SIMD_REGISTERS` reader - Indicates the size of the FP register bank. The value of this field is: 0b0001 - supported, 16 x 64-bit registers."]
pub type A_SIMD_REGISTERS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SINGLE_PRECISION` reader - Indicates the hardware support for FP single-precision operations. The value of this field is: 0b0010 - supported."]
pub type SINGLE_PRECISION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DOUBLE_PRECISION` reader - Indicates the hardware support for FP double-precision operations. The value of this field is: 0b0000 - not supported in ARMv7-M."]
pub type DOUBLE_PRECISION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FP_EXCEPTION_TRAPPING` reader - Indicates whether the FP hardware implementation supports exception trapping. The value of this field is: 0b0000 - not supported in ARMv7-M."]
pub type FP_EXCEPTION_TRAPPING_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIVIDE` reader - Indicates the hardware support for FP divide operations. The value of this field is: 0b0001 - supported."]
pub type DIVIDE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQUARE_ROOT` reader - Indicates the hardware support for FP square root operations. The value of this field is: 0b0001 - supported."]
pub type SQUARE_ROOT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHORT_VECTORS` reader - Indicates the hardware support for FP short vectors. The value of this field is: 0b0000 - not supported in ARMv7-M."]
pub type SHORT_VECTORS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FP_ROUNDING_MODES` reader - Indicates the rounding modes supported by the FP floating-point hardware. The value of this field is: 0b0001 - all rounding modes supported."]
pub type FP_ROUNDING_MODES_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Indicates the size of the FP register bank. The value of this field is: 0b0001 - supported, 16 x 64-bit registers."]
    #[inline(always)]
    pub fn a_simd_registers(&self) -> A_SIMD_REGISTERS_R {
        A_SIMD_REGISTERS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Indicates the hardware support for FP single-precision operations. The value of this field is: 0b0010 - supported."]
    #[inline(always)]
    pub fn single_precision(&self) -> SINGLE_PRECISION_R {
        SINGLE_PRECISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Indicates the hardware support for FP double-precision operations. The value of this field is: 0b0000 - not supported in ARMv7-M."]
    #[inline(always)]
    pub fn double_precision(&self) -> DOUBLE_PRECISION_R {
        DOUBLE_PRECISION_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Indicates whether the FP hardware implementation supports exception trapping. The value of this field is: 0b0000 - not supported in ARMv7-M."]
    #[inline(always)]
    pub fn fp_exception_trapping(&self) -> FP_EXCEPTION_TRAPPING_R {
        FP_EXCEPTION_TRAPPING_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Indicates the hardware support for FP divide operations. The value of this field is: 0b0001 - supported."]
    #[inline(always)]
    pub fn divide(&self) -> DIVIDE_R {
        DIVIDE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Indicates the hardware support for FP square root operations. The value of this field is: 0b0001 - supported."]
    #[inline(always)]
    pub fn square_root(&self) -> SQUARE_ROOT_R {
        SQUARE_ROOT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Indicates the hardware support for FP short vectors. The value of this field is: 0b0000 - not supported in ARMv7-M."]
    #[inline(always)]
    pub fn short_vectors(&self) -> SHORT_VECTORS_R {
        SHORT_VECTORS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Indicates the rounding modes supported by the FP floating-point hardware. The value of this field is: 0b0001 - all rounding modes supported."]
    #[inline(always)]
    pub fn fp_rounding_modes(&self) -> FP_ROUNDING_MODES_R {
        FP_ROUNDING_MODES_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Media and FP Feature Register 0 (MVFR0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mvfr0](index.html) module"]
pub struct MVFR0_SPEC;
impl crate::RegisterSpec for MVFR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mvfr0::R](R) reader structure"]
impl crate::Readable for MVFR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MVFR0 to value 0x1011_0021"]
impl crate::Resettable for MVFR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1011_0021
    }
}
