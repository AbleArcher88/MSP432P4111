#[doc = "Register `FLCTL_PRGBRST_CTLSTAT` reader"]
pub struct R(crate::R<FLCTL_PRGBRST_CTLSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_PRGBRST_CTLSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_PRGBRST_CTLSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_PRGBRST_CTLSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLCTL_PRGBRST_CTLSTAT` writer"]
pub struct W(crate::W<FLCTL_PRGBRST_CTLSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLCTL_PRGBRST_CTLSTAT_SPEC>;
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
impl From<crate::W<FLCTL_PRGBRST_CTLSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLCTL_PRGBRST_CTLSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` writer - Trigger start of burst program operation"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLCTL_PRGBRST_CTLSTAT_SPEC, bool, O>;
#[doc = "Field `TYPE` reader - Type of memory that burst program is carried out on"]
pub type TYPE_R = crate::FieldReader<u8, TYPE_A>;
#[doc = "Type of memory that burst program is carried out on\n\nValue on reset: 0"]
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
#[doc = "Field `TYPE` writer - Type of memory that burst program is carried out on"]
pub type TYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLCTL_PRGBRST_CTLSTAT_SPEC, u8, TYPE_A, 2, O>;
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
#[doc = "Field `LEN` reader - Length of burst"]
pub type LEN_R = crate::FieldReader<u8, LEN_A>;
#[doc = "Length of burst\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LEN_A {
    #[doc = "0: No burst operation"]
    LEN_0 = 0,
    #[doc = "1: 1 word burst of 128 bits, starting with address in the FLCTL_PRGBRST_STARTADDR Register"]
    LEN_1 = 1,
    #[doc = "2: 2*128 bits burst write, starting with address in the FLCTL_PRGBRST_STARTADDR Register"]
    LEN_2 = 2,
    #[doc = "3: 3*128 bits burst write, starting with address in the FLCTL_PRGBRST_STARTADDR Register"]
    LEN_3 = 3,
    #[doc = "4: 4*128 bits burst write, starting with address in the FLCTL_PRGBRST_STARTADDR Register"]
    LEN_4 = 4,
}
impl From<LEN_A> for u8 {
    #[inline(always)]
    fn from(variant: LEN_A) -> Self {
        variant as _
    }
}
impl LEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LEN_A> {
        match self.bits {
            0 => Some(LEN_A::LEN_0),
            1 => Some(LEN_A::LEN_1),
            2 => Some(LEN_A::LEN_2),
            3 => Some(LEN_A::LEN_3),
            4 => Some(LEN_A::LEN_4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LEN_0`"]
    #[inline(always)]
    pub fn is_len_0(&self) -> bool {
        *self == LEN_A::LEN_0
    }
    #[doc = "Checks if the value of the field is `LEN_1`"]
    #[inline(always)]
    pub fn is_len_1(&self) -> bool {
        *self == LEN_A::LEN_1
    }
    #[doc = "Checks if the value of the field is `LEN_2`"]
    #[inline(always)]
    pub fn is_len_2(&self) -> bool {
        *self == LEN_A::LEN_2
    }
    #[doc = "Checks if the value of the field is `LEN_3`"]
    #[inline(always)]
    pub fn is_len_3(&self) -> bool {
        *self == LEN_A::LEN_3
    }
    #[doc = "Checks if the value of the field is `LEN_4`"]
    #[inline(always)]
    pub fn is_len_4(&self) -> bool {
        *self == LEN_A::LEN_4
    }
}
#[doc = "Field `LEN` writer - Length of burst"]
pub type LEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLCTL_PRGBRST_CTLSTAT_SPEC, u8, LEN_A, 3, O>;
impl<'a, const O: u8> LEN_W<'a, O> {
    #[doc = "No burst operation"]
    #[inline(always)]
    pub fn len_0(self) -> &'a mut W {
        self.variant(LEN_A::LEN_0)
    }
    #[doc = "1 word burst of 128 bits, starting with address in the FLCTL_PRGBRST_STARTADDR Register"]
    #[inline(always)]
    pub fn len_1(self) -> &'a mut W {
        self.variant(LEN_A::LEN_1)
    }
    #[doc = "2*128 bits burst write, starting with address in the FLCTL_PRGBRST_STARTADDR Register"]
    #[inline(always)]
    pub fn len_2(self) -> &'a mut W {
        self.variant(LEN_A::LEN_2)
    }
    #[doc = "3*128 bits burst write, starting with address in the FLCTL_PRGBRST_STARTADDR Register"]
    #[inline(always)]
    pub fn len_3(self) -> &'a mut W {
        self.variant(LEN_A::LEN_3)
    }
    #[doc = "4*128 bits burst write, starting with address in the FLCTL_PRGBRST_STARTADDR Register"]
    #[inline(always)]
    pub fn len_4(self) -> &'a mut W {
        self.variant(LEN_A::LEN_4)
    }
}
#[doc = "Field `AUTO_PRE` reader - Auto-Verify operation before the Burst Program"]
pub type AUTO_PRE_R = crate::BitReader<AUTO_PRE_A>;
#[doc = "Auto-Verify operation before the Burst Program\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTO_PRE_A {
    #[doc = "0: No program verify operations carried out"]
    AUTO_PRE_0 = 0,
    #[doc = "1: Causes an automatic Burst Program Verify after the Burst Program Operation"]
    AUTO_PRE_1 = 1,
}
impl From<AUTO_PRE_A> for bool {
    #[inline(always)]
    fn from(variant: AUTO_PRE_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTO_PRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTO_PRE_A {
        match self.bits {
            false => AUTO_PRE_A::AUTO_PRE_0,
            true => AUTO_PRE_A::AUTO_PRE_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUTO_PRE_0`"]
    #[inline(always)]
    pub fn is_auto_pre_0(&self) -> bool {
        *self == AUTO_PRE_A::AUTO_PRE_0
    }
    #[doc = "Checks if the value of the field is `AUTO_PRE_1`"]
    #[inline(always)]
    pub fn is_auto_pre_1(&self) -> bool {
        *self == AUTO_PRE_A::AUTO_PRE_1
    }
}
#[doc = "Field `AUTO_PRE` writer - Auto-Verify operation before the Burst Program"]
pub type AUTO_PRE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_PRGBRST_CTLSTAT_SPEC, AUTO_PRE_A, O>;
impl<'a, const O: u8> AUTO_PRE_W<'a, O> {
    #[doc = "No program verify operations carried out"]
    #[inline(always)]
    pub fn auto_pre_0(self) -> &'a mut W {
        self.variant(AUTO_PRE_A::AUTO_PRE_0)
    }
    #[doc = "Causes an automatic Burst Program Verify after the Burst Program Operation"]
    #[inline(always)]
    pub fn auto_pre_1(self) -> &'a mut W {
        self.variant(AUTO_PRE_A::AUTO_PRE_1)
    }
}
#[doc = "Field `AUTO_PST` reader - Auto-Verify operation after the Burst Program"]
pub type AUTO_PST_R = crate::BitReader<AUTO_PST_A>;
#[doc = "Auto-Verify operation after the Burst Program\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTO_PST_A {
    #[doc = "0: No program verify operations carried out"]
    AUTO_PST_0 = 0,
    #[doc = "1: Causes an automatic Burst Program Verify before the Burst Program Operation"]
    AUTO_PST_1 = 1,
}
impl From<AUTO_PST_A> for bool {
    #[inline(always)]
    fn from(variant: AUTO_PST_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTO_PST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTO_PST_A {
        match self.bits {
            false => AUTO_PST_A::AUTO_PST_0,
            true => AUTO_PST_A::AUTO_PST_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUTO_PST_0`"]
    #[inline(always)]
    pub fn is_auto_pst_0(&self) -> bool {
        *self == AUTO_PST_A::AUTO_PST_0
    }
    #[doc = "Checks if the value of the field is `AUTO_PST_1`"]
    #[inline(always)]
    pub fn is_auto_pst_1(&self) -> bool {
        *self == AUTO_PST_A::AUTO_PST_1
    }
}
#[doc = "Field `AUTO_PST` writer - Auto-Verify operation after the Burst Program"]
pub type AUTO_PST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_PRGBRST_CTLSTAT_SPEC, AUTO_PST_A, O>;
impl<'a, const O: u8> AUTO_PST_W<'a, O> {
    #[doc = "No program verify operations carried out"]
    #[inline(always)]
    pub fn auto_pst_0(self) -> &'a mut W {
        self.variant(AUTO_PST_A::AUTO_PST_0)
    }
    #[doc = "Causes an automatic Burst Program Verify before the Burst Program Operation"]
    #[inline(always)]
    pub fn auto_pst_1(self) -> &'a mut W {
        self.variant(AUTO_PST_A::AUTO_PST_1)
    }
}
#[doc = "Field `BURST_STATUS` reader - Status of a Burst Operation"]
pub type BURST_STATUS_R = crate::FieldReader<u8, BURST_STATUS_A>;
#[doc = "Status of a Burst Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BURST_STATUS_A {
    #[doc = "0: Idle (Burst not active)"]
    BURST_STATUS_0 = 0,
    #[doc = "1: Burst program started but pending"]
    BURST_STATUS_1 = 1,
    #[doc = "2: Burst active, with 1st 128 bit word being written into Flash"]
    BURST_STATUS_2 = 2,
    #[doc = "3: Burst active, with 2nd 128 bit word being written into Flash"]
    BURST_STATUS_3 = 3,
    #[doc = "4: Burst active, with 3rd 128 bit word being written into Flash"]
    BURST_STATUS_4 = 4,
    #[doc = "5: Burst active, with 4th 128 bit word being written into Flash"]
    BURST_STATUS_5 = 5,
    #[doc = "7: Burst Complete (status of completed burst remains in this state unless explicitly cleared by SW)"]
    BURST_STATUS_7 = 7,
}
impl From<BURST_STATUS_A> for u8 {
    #[inline(always)]
    fn from(variant: BURST_STATUS_A) -> Self {
        variant as _
    }
}
impl BURST_STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BURST_STATUS_A> {
        match self.bits {
            0 => Some(BURST_STATUS_A::BURST_STATUS_0),
            1 => Some(BURST_STATUS_A::BURST_STATUS_1),
            2 => Some(BURST_STATUS_A::BURST_STATUS_2),
            3 => Some(BURST_STATUS_A::BURST_STATUS_3),
            4 => Some(BURST_STATUS_A::BURST_STATUS_4),
            5 => Some(BURST_STATUS_A::BURST_STATUS_5),
            7 => Some(BURST_STATUS_A::BURST_STATUS_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BURST_STATUS_0`"]
    #[inline(always)]
    pub fn is_burst_status_0(&self) -> bool {
        *self == BURST_STATUS_A::BURST_STATUS_0
    }
    #[doc = "Checks if the value of the field is `BURST_STATUS_1`"]
    #[inline(always)]
    pub fn is_burst_status_1(&self) -> bool {
        *self == BURST_STATUS_A::BURST_STATUS_1
    }
    #[doc = "Checks if the value of the field is `BURST_STATUS_2`"]
    #[inline(always)]
    pub fn is_burst_status_2(&self) -> bool {
        *self == BURST_STATUS_A::BURST_STATUS_2
    }
    #[doc = "Checks if the value of the field is `BURST_STATUS_3`"]
    #[inline(always)]
    pub fn is_burst_status_3(&self) -> bool {
        *self == BURST_STATUS_A::BURST_STATUS_3
    }
    #[doc = "Checks if the value of the field is `BURST_STATUS_4`"]
    #[inline(always)]
    pub fn is_burst_status_4(&self) -> bool {
        *self == BURST_STATUS_A::BURST_STATUS_4
    }
    #[doc = "Checks if the value of the field is `BURST_STATUS_5`"]
    #[inline(always)]
    pub fn is_burst_status_5(&self) -> bool {
        *self == BURST_STATUS_A::BURST_STATUS_5
    }
    #[doc = "Checks if the value of the field is `BURST_STATUS_7`"]
    #[inline(always)]
    pub fn is_burst_status_7(&self) -> bool {
        *self == BURST_STATUS_A::BURST_STATUS_7
    }
}
#[doc = "Field `PRE_ERR` reader - Burst Operation encountered preprogram auto-verify errors"]
pub type PRE_ERR_R = crate::BitReader<bool>;
#[doc = "Field `PST_ERR` reader - Burst Operation encountered postprogram auto-verify errors"]
pub type PST_ERR_R = crate::BitReader<bool>;
#[doc = "Field `ADDR_ERR` reader - Burst Operation was terminated due to attempted program of reserved memory"]
pub type ADDR_ERR_R = crate::BitReader<bool>;
#[doc = "Field `CLR_STAT` writer - Clear status bits 21-16 of this register"]
pub type CLR_STAT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FLCTL_PRGBRST_CTLSTAT_SPEC, bool, O>;
impl R {
    #[doc = "Bits 1:2 - Type of memory that burst program is carried out on"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:5 - Length of burst"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - Auto-Verify operation before the Burst Program"]
    #[inline(always)]
    pub fn auto_pre(&self) -> AUTO_PRE_R {
        AUTO_PRE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Auto-Verify operation after the Burst Program"]
    #[inline(always)]
    pub fn auto_pst(&self) -> AUTO_PST_R {
        AUTO_PST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Status of a Burst Operation"]
    #[inline(always)]
    pub fn burst_status(&self) -> BURST_STATUS_R {
        BURST_STATUS_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Burst Operation encountered preprogram auto-verify errors"]
    #[inline(always)]
    pub fn pre_err(&self) -> PRE_ERR_R {
        PRE_ERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Burst Operation encountered postprogram auto-verify errors"]
    #[inline(always)]
    pub fn pst_err(&self) -> PST_ERR_R {
        PST_ERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Burst Operation was terminated due to attempted program of reserved memory"]
    #[inline(always)]
    pub fn addr_err(&self) -> ADDR_ERR_R {
        ADDR_ERR_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger start of burst program operation"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bits 1:2 - Type of memory that burst program is carried out on"]
    #[inline(always)]
    pub fn type_(&mut self) -> TYPE_W<1> {
        TYPE_W::new(self)
    }
    #[doc = "Bits 3:5 - Length of burst"]
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W<3> {
        LEN_W::new(self)
    }
    #[doc = "Bit 6 - Auto-Verify operation before the Burst Program"]
    #[inline(always)]
    pub fn auto_pre(&mut self) -> AUTO_PRE_W<6> {
        AUTO_PRE_W::new(self)
    }
    #[doc = "Bit 7 - Auto-Verify operation after the Burst Program"]
    #[inline(always)]
    pub fn auto_pst(&mut self) -> AUTO_PST_W<7> {
        AUTO_PST_W::new(self)
    }
    #[doc = "Bit 23 - Clear status bits 21-16 of this register"]
    #[inline(always)]
    pub fn clr_stat(&mut self) -> CLR_STAT_W<23> {
        CLR_STAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Program Burst Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_prgbrst_ctlstat](index.html) module"]
pub struct FLCTL_PRGBRST_CTLSTAT_SPEC;
impl crate::RegisterSpec for FLCTL_PRGBRST_CTLSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_prgbrst_ctlstat::R](R) reader structure"]
impl crate::Readable for FLCTL_PRGBRST_CTLSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flctl_prgbrst_ctlstat::W](W) writer structure"]
impl crate::Writable for FLCTL_PRGBRST_CTLSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLCTL_PRGBRST_CTLSTAT to value 0xc0"]
impl crate::Resettable for FLCTL_PRGBRST_CTLSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc0
    }
}
