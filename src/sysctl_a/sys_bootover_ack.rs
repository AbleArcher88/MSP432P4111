#[doc = "Register `SYS_BOOTOVER_ACK` reader"]
pub struct R(crate::R<SYS_BOOTOVER_ACK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_BOOTOVER_ACK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_BOOTOVER_ACK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_BOOTOVER_ACK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_BOOTOVER_ACK` writer"]
pub struct W(crate::W<SYS_BOOTOVER_ACK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_BOOTOVER_ACK_SPEC>;
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
impl From<crate::W<SYS_BOOTOVER_ACK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_BOOTOVER_ACK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGVAL` reader - Value set by CPU, read/clear by the debugger"]
pub type REGVAL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `REGVAL` writer - Value set by CPU, read/clear by the debugger"]
pub type REGVAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYS_BOOTOVER_ACK_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Value set by CPU, read/clear by the debugger"]
    #[inline(always)]
    pub fn regval(&self) -> REGVAL_R {
        REGVAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value set by CPU, read/clear by the debugger"]
    #[inline(always)]
    pub fn regval(&mut self) -> REGVAL_W<0> {
        REGVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Boot Override Acknowledge Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_bootover_ack](index.html) module"]
pub struct SYS_BOOTOVER_ACK_SPEC;
impl crate::RegisterSpec for SYS_BOOTOVER_ACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_bootover_ack::R](R) reader structure"]
impl crate::Readable for SYS_BOOTOVER_ACK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_bootover_ack::W](W) writer structure"]
impl crate::Writable for SYS_BOOTOVER_ACK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_BOOTOVER_ACK to value 0"]
impl crate::Resettable for SYS_BOOTOVER_ACK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
