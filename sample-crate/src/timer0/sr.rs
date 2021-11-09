#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Shows if Timer is running or not\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUN_A {
    #[doc = "0: Timer is not running"]
    STOPPED = 0,
    #[doc = "1: Timer is running"]
    RUNNING = 1,
}
impl From<RUN_A> for bool {
    #[inline(always)]
    fn from(variant: RUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUN` reader - Shows if Timer is running or not"]
pub struct RUN_R(crate::FieldReader<bool, RUN_A>);
impl RUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RUN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUN_A {
        match self.bits {
            false => RUN_A::STOPPED,
            true => RUN_A::RUNNING,
        }
    }
    #[doc = "Checks if the value of the field is `STOPPED`"]
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        **self == RUN_A::STOPPED
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        **self == RUN_A::RUNNING
    }
}
impl core::ops::Deref for RUN_R {
    type Target = crate::FieldReader<bool, RUN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Shows if the MATCH was hit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MATCH_A {
    #[doc = "0: The MATCH condition was not hit"]
    NO_MATCH = 0,
    #[doc = "1: The MATCH condition was hit"]
    MATCH_HIT = 1,
}
impl From<MATCH_A> for bool {
    #[inline(always)]
    fn from(variant: MATCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MATCH` reader - Shows if the MATCH was hit"]
pub struct MATCH_R(crate::FieldReader<bool, MATCH_A>);
impl MATCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MATCH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MATCH_A {
        match self.bits {
            false => MATCH_A::NO_MATCH,
            true => MATCH_A::MATCH_HIT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_MATCH`"]
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        **self == MATCH_A::NO_MATCH
    }
    #[doc = "Checks if the value of the field is `MATCH_HIT`"]
    #[inline(always)]
    pub fn is_match_hit(&self) -> bool {
        **self == MATCH_A::MATCH_HIT
    }
}
impl core::ops::Deref for MATCH_R {
    type Target = crate::FieldReader<bool, MATCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MATCH` writer - Shows if the MATCH was hit"]
pub struct MATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MATCH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The MATCH condition was not hit"]
    #[inline(always)]
    pub fn no_match(self) -> &'a mut W {
        self.variant(MATCH_A::NO_MATCH)
    }
    #[doc = "The MATCH condition was hit"]
    #[inline(always)]
    pub fn match_hit(self) -> &'a mut W {
        self.variant(MATCH_A::MATCH_HIT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u16 & 0x01) << 8);
        self.w
    }
}
#[doc = "Shows if an underflow occured. This flag is sticky\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UN_A {
    #[doc = "0: No underflow occured since last clear"]
    NO_UNDERFLOW = 0,
    #[doc = "1: A minimum of one underflow occured since last clear"]
    UNDERFLOW = 1,
}
impl From<UN_A> for bool {
    #[inline(always)]
    fn from(variant: UN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UN` reader - Shows if an underflow occured. This flag is sticky"]
pub struct UN_R(crate::FieldReader<bool, UN_A>);
impl UN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UN_A {
        match self.bits {
            false => UN_A::NO_UNDERFLOW,
            true => UN_A::UNDERFLOW,
        }
    }
    #[doc = "Checks if the value of the field is `NO_UNDERFLOW`"]
    #[inline(always)]
    pub fn is_no_underflow(&self) -> bool {
        **self == UN_A::NO_UNDERFLOW
    }
    #[doc = "Checks if the value of the field is `UNDERFLOW`"]
    #[inline(always)]
    pub fn is_underflow(&self) -> bool {
        **self == UN_A::UNDERFLOW
    }
}
impl core::ops::Deref for UN_R {
    type Target = crate::FieldReader<bool, UN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UN` writer - Shows if an underflow occured. This flag is sticky"]
pub struct UN_W<'a> {
    w: &'a mut W,
}
impl<'a> UN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No underflow occured since last clear"]
    #[inline(always)]
    pub fn no_underflow(self) -> &'a mut W {
        self.variant(UN_A::NO_UNDERFLOW)
    }
    #[doc = "A minimum of one underflow occured since last clear"]
    #[inline(always)]
    pub fn underflow(self) -> &'a mut W {
        self.variant(UN_A::UNDERFLOW)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u16 & 0x01) << 9);
        self.w
    }
}
#[doc = "Shows if an overflow occured. This flag is sticky\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OV_A {
    #[doc = "0: No overflow occured since last clear"]
    NO_OVERFLOW = 0,
    #[doc = "1: A minimum of one overflow occured since last clear"]
    OVERFLOW_OCCURED = 1,
}
impl From<OV_A> for bool {
    #[inline(always)]
    fn from(variant: OV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OV` reader - Shows if an overflow occured. This flag is sticky"]
pub struct OV_R(crate::FieldReader<bool, OV_A>);
impl OV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OV_A {
        match self.bits {
            false => OV_A::NO_OVERFLOW,
            true => OV_A::OVERFLOW_OCCURED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_OVERFLOW`"]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        **self == OV_A::NO_OVERFLOW
    }
    #[doc = "Checks if the value of the field is `OVERFLOW_OCCURED`"]
    #[inline(always)]
    pub fn is_overflow_occured(&self) -> bool {
        **self == OV_A::OVERFLOW_OCCURED
    }
}
impl core::ops::Deref for OV_R {
    type Target = crate::FieldReader<bool, OV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OV` writer - Shows if an overflow occured. This flag is sticky"]
pub struct OV_W<'a> {
    w: &'a mut W,
}
impl<'a> OV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No overflow occured since last clear"]
    #[inline(always)]
    pub fn no_overflow(self) -> &'a mut W {
        self.variant(OV_A::NO_OVERFLOW)
    }
    #[doc = "A minimum of one overflow occured since last clear"]
    #[inline(always)]
    pub fn overflow_occured(self) -> &'a mut W {
        self.variant(OV_A::OVERFLOW_OCCURED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u16 & 0x01) << 10);
        self.w
    }
}
#[doc = "Shows if Timer is in RESET state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RST_A {
    #[doc = "0: Timer is not in RESET state and can operate"]
    READY = 0,
    #[doc = "1: Timer is in RESET state and can not operate"]
    IN_RESET = 1,
}
impl From<RST_A> for bool {
    #[inline(always)]
    fn from(variant: RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RST` reader - Shows if Timer is in RESET state"]
pub struct RST_R(crate::FieldReader<bool, RST_A>);
impl RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RST_A {
        match self.bits {
            false => RST_A::READY,
            true => RST_A::IN_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == RST_A::READY
    }
    #[doc = "Checks if the value of the field is `IN_RESET`"]
    #[inline(always)]
    pub fn is_in_reset(&self) -> bool {
        **self == RST_A::IN_RESET
    }
}
impl core::ops::Deref for RST_R {
    type Target = crate::FieldReader<bool, RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Shows the currently active RELOAD Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RELOAD_A {
    #[doc = "0: Reload Register number 0 is active"]
    RELOAD0 = 0,
    #[doc = "1: Reload Register number 1 is active"]
    RELOAD1 = 1,
    #[doc = "2: Reload Register number 2 is active"]
    RELOAD2 = 2,
    #[doc = "3: Reload Register number 3 is active"]
    RELOAD3 = 3,
}
impl From<RELOAD_A> for u8 {
    #[inline(always)]
    fn from(variant: RELOAD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RELOAD` reader - Shows the currently active RELOAD Register"]
pub struct RELOAD_R(crate::FieldReader<u8, RELOAD_A>);
impl RELOAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RELOAD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RELOAD_A {
        match self.bits {
            0 => RELOAD_A::RELOAD0,
            1 => RELOAD_A::RELOAD1,
            2 => RELOAD_A::RELOAD2,
            3 => RELOAD_A::RELOAD3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RELOAD0`"]
    #[inline(always)]
    pub fn is_reload0(&self) -> bool {
        **self == RELOAD_A::RELOAD0
    }
    #[doc = "Checks if the value of the field is `RELOAD1`"]
    #[inline(always)]
    pub fn is_reload1(&self) -> bool {
        **self == RELOAD_A::RELOAD1
    }
    #[doc = "Checks if the value of the field is `RELOAD2`"]
    #[inline(always)]
    pub fn is_reload2(&self) -> bool {
        **self == RELOAD_A::RELOAD2
    }
    #[doc = "Checks if the value of the field is `RELOAD3`"]
    #[inline(always)]
    pub fn is_reload3(&self) -> bool {
        **self == RELOAD_A::RELOAD3
    }
}
impl core::ops::Deref for RELOAD_R {
    type Target = crate::FieldReader<u8, RELOAD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Shows if Timer is running or not"]
    #[inline(always)]
    pub fn run(&self) -> RUN_R {
        RUN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Shows if the MATCH was hit"]
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Shows if an underflow occured. This flag is sticky"]
    #[inline(always)]
    pub fn un(&self) -> UN_R {
        UN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Shows if an overflow occured. This flag is sticky"]
    #[inline(always)]
    pub fn ov(&self) -> OV_R {
        OV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Shows if Timer is in RESET state"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Shows the currently active RELOAD Register"]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Shows if the MATCH was hit"]
    #[inline(always)]
    pub fn match_(&mut self) -> MATCH_W {
        MATCH_W { w: self }
    }
    #[doc = "Bit 9 - Shows if an underflow occured. This flag is sticky"]
    #[inline(always)]
    pub fn un(&mut self) -> UN_W {
        UN_W { w: self }
    }
    #[doc = "Bit 10 - Shows if an overflow occured. This flag is sticky"]
    #[inline(always)]
    pub fn ov(&mut self) -> OV_W {
        OV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
