#[doc = "Register `FLCTL_ERASE_CTLSTAT` reader"]
pub struct R(crate::R<FLCTL_ERASE_CTLSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_ERASE_CTLSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_ERASE_CTLSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_ERASE_CTLSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLCTL_ERASE_CTLSTAT` writer"]
pub struct W(crate::W<FLCTL_ERASE_CTLSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLCTL_ERASE_CTLSTAT_SPEC>;
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
impl From<crate::W<FLCTL_ERASE_CTLSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLCTL_ERASE_CTLSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` writer - Start of Erase operation"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLCTL_ERASE_CTLSTAT_SPEC, bool, O>;
#[doc = "Field `MODE` reader - Erase mode selected by application"]
pub type MODE_R = crate::BitReader<MODE_A>;
#[doc = "Erase mode selected by application\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE_A {
    #[doc = "0: Sector Erase (controlled by FLTCTL_ERASE_SECTADDR)"]
    MODE_0 = 0,
    #[doc = "1: Mass Erase (includes all Main and Information memory sectors that don't have corresponding WE bits set)"]
    MODE_1 = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::MODE_0,
            true => MODE_A::MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `MODE_0`"]
    #[inline(always)]
    pub fn is_mode_0(&self) -> bool {
        *self == MODE_A::MODE_0
    }
    #[doc = "Checks if the value of the field is `MODE_1`"]
    #[inline(always)]
    pub fn is_mode_1(&self) -> bool {
        *self == MODE_A::MODE_1
    }
}
#[doc = "Field `MODE` writer - Erase mode selected by application"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLCTL_ERASE_CTLSTAT_SPEC, MODE_A, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Sector Erase (controlled by FLTCTL_ERASE_SECTADDR)"]
    #[inline(always)]
    pub fn mode_0(self) -> &'a mut W {
        self.variant(MODE_A::MODE_0)
    }
    #[doc = "Mass Erase (includes all Main and Information memory sectors that don't have corresponding WE bits set)"]
    #[inline(always)]
    pub fn mode_1(self) -> &'a mut W {
        self.variant(MODE_A::MODE_1)
    }
}
#[doc = "Field `TYPE` reader - Type of memory that erase operation is carried out on"]
pub type TYPE_R = crate::FieldReader<u8, TYPE_A>;
#[doc = "Type of memory that erase operation is carried out on\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TYPE_A {
    #[doc = "0: Main Memory"]
    TYPE_0 = 0,
    #[doc = "1: Information Memory"]
    TYPE_1 = 1,
    #[doc = "3: Engineering Memory"]
    TYPE_3 = 3,
}
impl From<TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: TYPE_A) -> Self {
        variant as _
    }
}
impl TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TYPE_A> {
        match self.bits {
            0 => Some(TYPE_A::TYPE_0),
            1 => Some(TYPE_A::TYPE_1),
            3 => Some(TYPE_A::TYPE_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TYPE_0`"]
    #[inline(always)]
    pub fn is_type_0(&self) -> bool {
        *self == TYPE_A::TYPE_0
    }
    #[doc = "Checks if the value of the field is `TYPE_1`"]
    #[inline(always)]
    pub fn is_type_1(&self) -> bool {
        *self == TYPE_A::TYPE_1
    }
    #[doc = "Checks if the value of the field is `TYPE_3`"]
    #[inline(always)]
    pub fn is_type_3(&self) -> bool {
        *self == TYPE_A::TYPE_3
    }
}
#[doc = "Field `TYPE` writer - Type of memory that erase operation is carried out on"]
pub type TYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLCTL_ERASE_CTLSTAT_SPEC, u8, TYPE_A, 2, O>;
impl<'a, const O: u8> TYPE_W<'a, O> {
    #[doc = "Main Memory"]
    #[inline(always)]
    pub fn type_0(self) -> &'a mut W {
        self.variant(TYPE_A::TYPE_0)
    }
    #[doc = "Information Memory"]
    #[inline(always)]
    pub fn type_1(self) -> &'a mut W {
        self.variant(TYPE_A::TYPE_1)
    }
    #[doc = "Engineering Memory"]
    #[inline(always)]
    pub fn type_3(self) -> &'a mut W {
        self.variant(TYPE_A::TYPE_3)
    }
}
#[doc = "Field `STATUS` reader - Status of erase operations in the Flash memory"]
pub type STATUS_R = crate::FieldReader<u8, STATUS_A>;
#[doc = "Status of erase operations in the Flash memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STATUS_A {
    #[doc = "0: Idle (no program operation currently active)"]
    STATUS_0 = 0,
    #[doc = "1: Erase operation triggered to START but pending"]
    STATUS_1 = 1,
    #[doc = "2: Erase operation in progress"]
    STATUS_2 = 2,
    #[doc = "3: Erase operation completed (status of completed erase remains in this state unless explicitly cleared by SW)"]
    STATUS_3 = 3,
}
impl From<STATUS_A> for u8 {
    #[inline(always)]
    fn from(variant: STATUS_A) -> Self {
        variant as _
    }
}
impl STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATUS_A {
        match self.bits {
            0 => STATUS_A::STATUS_0,
            1 => STATUS_A::STATUS_1,
            2 => STATUS_A::STATUS_2,
            3 => STATUS_A::STATUS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STATUS_0`"]
    #[inline(always)]
    pub fn is_status_0(&self) -> bool {
        *self == STATUS_A::STATUS_0
    }
    #[doc = "Checks if the value of the field is `STATUS_1`"]
    #[inline(always)]
    pub fn is_status_1(&self) -> bool {
        *self == STATUS_A::STATUS_1
    }
    #[doc = "Checks if the value of the field is `STATUS_2`"]
    #[inline(always)]
    pub fn is_status_2(&self) -> bool {
        *self == STATUS_A::STATUS_2
    }
    #[doc = "Checks if the value of the field is `STATUS_3`"]
    #[inline(always)]
    pub fn is_status_3(&self) -> bool {
        *self == STATUS_A::STATUS_3
    }
}
#[doc = "Field `ADDR_ERR` reader - Erase Operation was terminated due to attempted erase of reserved memory address"]
pub type ADDR_ERR_R = crate::BitReader<bool>;
#[doc = "Field `CLR_STAT` writer - Clear status bits 18-16 of this register"]
pub type CLR_STAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLCTL_ERASE_CTLSTAT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Erase mode selected by application"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Type of memory that erase operation is carried out on"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Status of erase operations in the Flash memory"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Erase Operation was terminated due to attempted erase of reserved memory address"]
    #[inline(always)]
    pub fn addr_err(&self) -> ADDR_ERR_R {
        ADDR_ERR_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start of Erase operation"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bit 1 - Erase mode selected by application"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<1> {
        MODE_W::new(self)
    }
    #[doc = "Bits 2:3 - Type of memory that erase operation is carried out on"]
    #[inline(always)]
    pub fn type_(&mut self) -> TYPE_W<2> {
        TYPE_W::new(self)
    }
    #[doc = "Bit 19 - Clear status bits 18-16 of this register"]
    #[inline(always)]
    pub fn clr_stat(&mut self) -> CLR_STAT_W<19> {
        CLR_STAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Erase Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_erase_ctlstat](index.html) module"]
pub struct FLCTL_ERASE_CTLSTAT_SPEC;
impl crate::RegisterSpec for FLCTL_ERASE_CTLSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_erase_ctlstat::R](R) reader structure"]
impl crate::Readable for FLCTL_ERASE_CTLSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flctl_erase_ctlstat::W](W) writer structure"]
impl crate::Writable for FLCTL_ERASE_CTLSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLCTL_ERASE_CTLSTAT to value 0"]
impl crate::Resettable for FLCTL_ERASE_CTLSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
