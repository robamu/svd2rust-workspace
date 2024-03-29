#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "0: Timer is disabled and does not operate"]
    DISABLE = 0,
    #[doc = "1: Timer is enabled and can operate"]
    ENABLE = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Enable"]
pub struct EN_R(crate::FieldReader<bool, EN_A>);
impl EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::DISABLE,
            true => EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == EN_A::ENABLE
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - Enable"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer is disabled and does not operate"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EN_A::DISABLE)
    }
    #[doc = "Timer is enabled and can operate"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Reset Timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RST_AW {
    #[doc = "0: Write as ZERO if necessary"]
    NO_ACTION = 0,
    #[doc = "1: Reset the Timer"]
    RESET_TIMER = 1,
}
impl From<RST_AW> for bool {
    #[inline(always)]
    fn from(variant: RST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RST` writer - Reset Timer"]
pub struct RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RST_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write as ZERO if necessary"]
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(RST_AW::NO_ACTION)
    }
    #[doc = "Reset the Timer"]
    #[inline(always)]
    pub fn reset_timer(self) -> &'a mut W {
        self.variant(RST_AW::RESET_TIMER)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Counting direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CNT_A {
    #[doc = "0: Timer Counts UO and wraps, if no STOP condition is set"]
    COUNT_UP = 0,
    #[doc = "1: Timer Counts DOWN and wraps, if no STOP condition is set"]
    COUNT_DOWN = 1,
    #[doc = "2: Timer Counts up to MAX, then DOWN to ZERO, if no STOP condition is set"]
    TOGGLE = 2,
}
impl From<CNT_A> for u8 {
    #[inline(always)]
    fn from(variant: CNT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CNT` reader - Counting direction"]
pub struct CNT_R(crate::FieldReader<u8, CNT_A>);
impl CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CNT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CNT_A> {
        match self.bits {
            0 => Some(CNT_A::COUNT_UP),
            1 => Some(CNT_A::COUNT_DOWN),
            2 => Some(CNT_A::TOGGLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `COUNT_UP`"]
    #[inline(always)]
    pub fn is_count_up(&self) -> bool {
        **self == CNT_A::COUNT_UP
    }
    #[doc = "Checks if the value of the field is `COUNT_DOWN`"]
    #[inline(always)]
    pub fn is_count_down(&self) -> bool {
        **self == CNT_A::COUNT_DOWN
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        **self == CNT_A::TOGGLE
    }
}
impl core::ops::Deref for CNT_R {
    type Target = crate::FieldReader<u8, CNT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT` writer - Counting direction"]
pub struct CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Timer Counts UO and wraps, if no STOP condition is set"]
    #[inline(always)]
    pub fn count_up(self) -> &'a mut W {
        self.variant(CNT_A::COUNT_UP)
    }
    #[doc = "Timer Counts DOWN and wraps, if no STOP condition is set"]
    #[inline(always)]
    pub fn count_down(self) -> &'a mut W {
        self.variant(CNT_A::COUNT_DOWN)
    }
    #[doc = "Timer Counts up to MAX, then DOWN to ZERO, if no STOP condition is set"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(CNT_A::TOGGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Operation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Timer runs continously"]
    CONTINOUS = 0,
    #[doc = "1: Timer counts to 0x00 or 0xFFFFFFFF (depending on CNT) and stops"]
    SINGLE_ZERO_MAX = 1,
    #[doc = "2: Timer counts to the Value of MATCH Register and stops"]
    SINGLE_MATCH = 2,
    #[doc = "3: Timer counts to 0x00 or 0xFFFFFFFF (depending on CNT), loads the RELOAD Value and continues"]
    RELOAD_ZERO_MAX = 3,
    #[doc = "4: Timer counts to the Value of MATCH Register, loads the RELOAD Value and continues"]
    RELOAD_MATCH = 4,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - Operation Mode"]
pub struct MODE_R(crate::FieldReader<u8, MODE_A>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::CONTINOUS),
            1 => Some(MODE_A::SINGLE_ZERO_MAX),
            2 => Some(MODE_A::SINGLE_MATCH),
            3 => Some(MODE_A::RELOAD_ZERO_MAX),
            4 => Some(MODE_A::RELOAD_MATCH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CONTINOUS`"]
    #[inline(always)]
    pub fn is_continous(&self) -> bool {
        **self == MODE_A::CONTINOUS
    }
    #[doc = "Checks if the value of the field is `SINGLE_ZERO_MAX`"]
    #[inline(always)]
    pub fn is_single_zero_max(&self) -> bool {
        **self == MODE_A::SINGLE_ZERO_MAX
    }
    #[doc = "Checks if the value of the field is `SINGLE_MATCH`"]
    #[inline(always)]
    pub fn is_single_match(&self) -> bool {
        **self == MODE_A::SINGLE_MATCH
    }
    #[doc = "Checks if the value of the field is `RELOAD_ZERO_MAX`"]
    #[inline(always)]
    pub fn is_reload_zero_max(&self) -> bool {
        **self == MODE_A::RELOAD_ZERO_MAX
    }
    #[doc = "Checks if the value of the field is `RELOAD_MATCH`"]
    #[inline(always)]
    pub fn is_reload_match(&self) -> bool {
        **self == MODE_A::RELOAD_MATCH
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Operation Mode"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Timer runs continously"]
    #[inline(always)]
    pub fn continous(self) -> &'a mut W {
        self.variant(MODE_A::CONTINOUS)
    }
    #[doc = "Timer counts to 0x00 or 0xFFFFFFFF (depending on CNT) and stops"]
    #[inline(always)]
    pub fn single_zero_max(self) -> &'a mut W {
        self.variant(MODE_A::SINGLE_ZERO_MAX)
    }
    #[doc = "Timer counts to the Value of MATCH Register and stops"]
    #[inline(always)]
    pub fn single_match(self) -> &'a mut W {
        self.variant(MODE_A::SINGLE_MATCH)
    }
    #[doc = "Timer counts to 0x00 or 0xFFFFFFFF (depending on CNT), loads the RELOAD Value and continues"]
    #[inline(always)]
    pub fn reload_zero_max(self) -> &'a mut W {
        self.variant(MODE_A::RELOAD_ZERO_MAX)
    }
    #[doc = "Timer counts to the Value of MATCH Register, loads the RELOAD Value and continues"]
    #[inline(always)]
    pub fn reload_match(self) -> &'a mut W {
        self.variant(MODE_A::RELOAD_MATCH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Use Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSC_A {
    #[doc = "0: Prescaler is not used"]
    DISABLED = 0,
    #[doc = "1: Prescaler is used as divider"]
    ENABLED = 1,
}
impl From<PSC_A> for bool {
    #[inline(always)]
    fn from(variant: PSC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSC` reader - Use Prescaler"]
pub struct PSC_R(crate::FieldReader<bool, PSC_A>);
impl PSC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PSC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSC_A {
        match self.bits {
            false => PSC_A::DISABLED,
            true => PSC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PSC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PSC_A::ENABLED
    }
}
impl core::ops::Deref for PSC_R {
    type Target = crate::FieldReader<bool, PSC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSC` writer - Use Prescaler"]
pub struct PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Prescaler is not used"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PSC_A::DISABLED)
    }
    #[doc = "Prescaler is used as divider"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PSC_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Timer / Counter Source Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CNTSRC_A {
    #[doc = "0: Capture Source is used directly"]
    CAP_SRC = 0,
    #[doc = "1: Capture Source is divided by 2"]
    CAP_SRC_DIV2 = 1,
    #[doc = "2: Capture Source is divided by 4"]
    CAP_SRC_DIV4 = 2,
    #[doc = "3: Capture Source is divided by 8"]
    CAP_SRC_DIV8 = 3,
    #[doc = "4: Capture Source is divided by 16"]
    CAP_SRC_DIV16 = 4,
    #[doc = "5: Capture Source is divided by 32"]
    CAP_SRC_DIV32 = 5,
    #[doc = "6: Capture Source is divided by 64"]
    CAP_SRC_DIV64 = 6,
    #[doc = "7: Capture Source is divided by 128"]
    CAP_SRC_DIV128 = 7,
    #[doc = "8: Capture Source is divided by 256"]
    CAP_SRC_DIV256 = 8,
}
impl From<CNTSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: CNTSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CNTSRC` reader - Timer / Counter Source Divider"]
pub struct CNTSRC_R(crate::FieldReader<u8, CNTSRC_A>);
impl CNTSRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CNTSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CNTSRC_A> {
        match self.bits {
            0 => Some(CNTSRC_A::CAP_SRC),
            1 => Some(CNTSRC_A::CAP_SRC_DIV2),
            2 => Some(CNTSRC_A::CAP_SRC_DIV4),
            3 => Some(CNTSRC_A::CAP_SRC_DIV8),
            4 => Some(CNTSRC_A::CAP_SRC_DIV16),
            5 => Some(CNTSRC_A::CAP_SRC_DIV32),
            6 => Some(CNTSRC_A::CAP_SRC_DIV64),
            7 => Some(CNTSRC_A::CAP_SRC_DIV128),
            8 => Some(CNTSRC_A::CAP_SRC_DIV256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CAP_SRC`"]
    #[inline(always)]
    pub fn is_cap_src(&self) -> bool {
        **self == CNTSRC_A::CAP_SRC
    }
    #[doc = "Checks if the value of the field is `CAP_SRC_DIV2`"]
    #[inline(always)]
    pub fn is_cap_src_div2(&self) -> bool {
        **self == CNTSRC_A::CAP_SRC_DIV2
    }
    #[doc = "Checks if the value of the field is `CAP_SRC_DIV4`"]
    #[inline(always)]
    pub fn is_cap_src_div4(&self) -> bool {
        **self == CNTSRC_A::CAP_SRC_DIV4
    }
    #[doc = "Checks if the value of the field is `CAP_SRC_DIV8`"]
    #[inline(always)]
    pub fn is_cap_src_div8(&self) -> bool {
        **self == CNTSRC_A::CAP_SRC_DIV8
    }
    #[doc = "Checks if the value of the field is `CAP_SRC_DIV16`"]
    #[inline(always)]
    pub fn is_cap_src_div16(&self) -> bool {
        **self == CNTSRC_A::CAP_SRC_DIV16
    }
    #[doc = "Checks if the value of the field is `CAP_SRC_DIV32`"]
    #[inline(always)]
    pub fn is_cap_src_div32(&self) -> bool {
        **self == CNTSRC_A::CAP_SRC_DIV32
    }
    #[doc = "Checks if the value of the field is `CAP_SRC_DIV64`"]
    #[inline(always)]
    pub fn is_cap_src_div64(&self) -> bool {
        **self == CNTSRC_A::CAP_SRC_DIV64
    }
    #[doc = "Checks if the value of the field is `CAP_SRC_DIV128`"]
    #[inline(always)]
    pub fn is_cap_src_div128(&self) -> bool {
        **self == CNTSRC_A::CAP_SRC_DIV128
    }
    #[doc = "Checks if the value of the field is `CAP_SRC_DIV256`"]
    #[inline(always)]
    pub fn is_cap_src_div256(&self) -> bool {
        **self == CNTSRC_A::CAP_SRC_DIV256
    }
}
impl core::ops::Deref for CNTSRC_R {
    type Target = crate::FieldReader<u8, CNTSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTSRC` writer - Timer / Counter Source Divider"]
pub struct CNTSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CNTSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Capture Source is used directly"]
    #[inline(always)]
    pub fn cap_src(self) -> &'a mut W {
        self.variant(CNTSRC_A::CAP_SRC)
    }
    #[doc = "Capture Source is divided by 2"]
    #[inline(always)]
    pub fn cap_src_div2(self) -> &'a mut W {
        self.variant(CNTSRC_A::CAP_SRC_DIV2)
    }
    #[doc = "Capture Source is divided by 4"]
    #[inline(always)]
    pub fn cap_src_div4(self) -> &'a mut W {
        self.variant(CNTSRC_A::CAP_SRC_DIV4)
    }
    #[doc = "Capture Source is divided by 8"]
    #[inline(always)]
    pub fn cap_src_div8(self) -> &'a mut W {
        self.variant(CNTSRC_A::CAP_SRC_DIV8)
    }
    #[doc = "Capture Source is divided by 16"]
    #[inline(always)]
    pub fn cap_src_div16(self) -> &'a mut W {
        self.variant(CNTSRC_A::CAP_SRC_DIV16)
    }
    #[doc = "Capture Source is divided by 32"]
    #[inline(always)]
    pub fn cap_src_div32(self) -> &'a mut W {
        self.variant(CNTSRC_A::CAP_SRC_DIV32)
    }
    #[doc = "Capture Source is divided by 64"]
    #[inline(always)]
    pub fn cap_src_div64(self) -> &'a mut W {
        self.variant(CNTSRC_A::CAP_SRC_DIV64)
    }
    #[doc = "Capture Source is divided by 128"]
    #[inline(always)]
    pub fn cap_src_div128(self) -> &'a mut W {
        self.variant(CNTSRC_A::CAP_SRC_DIV128)
    }
    #[doc = "Capture Source is divided by 256"]
    #[inline(always)]
    pub fn cap_src_div256(self) -> &'a mut W {
        self.variant(CNTSRC_A::CAP_SRC_DIV256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Timer / Counter Capture Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CAPSRC_A {
    #[doc = "0: Core Clock"]
    CCLK = 0,
    #[doc = "1: GPIO A, PIN 0"]
    GPIOA_0 = 1,
    #[doc = "2: GPIO A, PIN 1"]
    GPIOA_1 = 2,
    #[doc = "3: GPIO A, PIN 2"]
    GPIOA_2 = 3,
    #[doc = "4: GPIO A, PIN 3"]
    GPIOA_3 = 4,
    #[doc = "5: GPIO A, PIN 4"]
    GPIOA_4 = 5,
    #[doc = "6: GPIO A, PIN 5"]
    GPIOA_5 = 6,
    #[doc = "7: GPIO A, PIN 6"]
    GPIOA_6 = 7,
    #[doc = "8: GPIO A, PIN 7"]
    GPIOA_7 = 8,
    #[doc = "9: GPIO B, PIN 0"]
    GPIOB_0 = 9,
    #[doc = "10: GPIO B, PIN 1"]
    GPIOB_1 = 10,
    #[doc = "11: GPIO B, PIN 2"]
    GPIOB_2 = 11,
    #[doc = "12: GPIO B, PIN 3"]
    GPIOB_3 = 12,
    #[doc = "13: GPIO C, PIN 0"]
    GPIOC_0 = 13,
    #[doc = "14: GPIO C, PIN 1"]
    GPIOC_5 = 14,
    #[doc = "15: GPIO C, PIN 2"]
    GPIOC_6 = 15,
}
impl From<CAPSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CAPSRC` reader - Timer / Counter Capture Source"]
pub struct CAPSRC_R(crate::FieldReader<u8, CAPSRC_A>);
impl CAPSRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CAPSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPSRC_A {
        match self.bits {
            0 => CAPSRC_A::CCLK,
            1 => CAPSRC_A::GPIOA_0,
            2 => CAPSRC_A::GPIOA_1,
            3 => CAPSRC_A::GPIOA_2,
            4 => CAPSRC_A::GPIOA_3,
            5 => CAPSRC_A::GPIOA_4,
            6 => CAPSRC_A::GPIOA_5,
            7 => CAPSRC_A::GPIOA_6,
            8 => CAPSRC_A::GPIOA_7,
            9 => CAPSRC_A::GPIOB_0,
            10 => CAPSRC_A::GPIOB_1,
            11 => CAPSRC_A::GPIOB_2,
            12 => CAPSRC_A::GPIOB_3,
            13 => CAPSRC_A::GPIOC_0,
            14 => CAPSRC_A::GPIOC_5,
            15 => CAPSRC_A::GPIOC_6,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CCLK`"]
    #[inline(always)]
    pub fn is_cclk(&self) -> bool {
        **self == CAPSRC_A::CCLK
    }
    #[doc = "Checks if the value of the field is `GPIOA_0`"]
    #[inline(always)]
    pub fn is_gpioa_0(&self) -> bool {
        **self == CAPSRC_A::GPIOA_0
    }
    #[doc = "Checks if the value of the field is `GPIOA_1`"]
    #[inline(always)]
    pub fn is_gpioa_1(&self) -> bool {
        **self == CAPSRC_A::GPIOA_1
    }
    #[doc = "Checks if the value of the field is `GPIOA_2`"]
    #[inline(always)]
    pub fn is_gpioa_2(&self) -> bool {
        **self == CAPSRC_A::GPIOA_2
    }
    #[doc = "Checks if the value of the field is `GPIOA_3`"]
    #[inline(always)]
    pub fn is_gpioa_3(&self) -> bool {
        **self == CAPSRC_A::GPIOA_3
    }
    #[doc = "Checks if the value of the field is `GPIOA_4`"]
    #[inline(always)]
    pub fn is_gpioa_4(&self) -> bool {
        **self == CAPSRC_A::GPIOA_4
    }
    #[doc = "Checks if the value of the field is `GPIOA_5`"]
    #[inline(always)]
    pub fn is_gpioa_5(&self) -> bool {
        **self == CAPSRC_A::GPIOA_5
    }
    #[doc = "Checks if the value of the field is `GPIOA_6`"]
    #[inline(always)]
    pub fn is_gpioa_6(&self) -> bool {
        **self == CAPSRC_A::GPIOA_6
    }
    #[doc = "Checks if the value of the field is `GPIOA_7`"]
    #[inline(always)]
    pub fn is_gpioa_7(&self) -> bool {
        **self == CAPSRC_A::GPIOA_7
    }
    #[doc = "Checks if the value of the field is `GPIOB_0`"]
    #[inline(always)]
    pub fn is_gpiob_0(&self) -> bool {
        **self == CAPSRC_A::GPIOB_0
    }
    #[doc = "Checks if the value of the field is `GPIOB_1`"]
    #[inline(always)]
    pub fn is_gpiob_1(&self) -> bool {
        **self == CAPSRC_A::GPIOB_1
    }
    #[doc = "Checks if the value of the field is `GPIOB_2`"]
    #[inline(always)]
    pub fn is_gpiob_2(&self) -> bool {
        **self == CAPSRC_A::GPIOB_2
    }
    #[doc = "Checks if the value of the field is `GPIOB_3`"]
    #[inline(always)]
    pub fn is_gpiob_3(&self) -> bool {
        **self == CAPSRC_A::GPIOB_3
    }
    #[doc = "Checks if the value of the field is `GPIOC_0`"]
    #[inline(always)]
    pub fn is_gpioc_0(&self) -> bool {
        **self == CAPSRC_A::GPIOC_0
    }
    #[doc = "Checks if the value of the field is `GPIOC_5`"]
    #[inline(always)]
    pub fn is_gpioc_5(&self) -> bool {
        **self == CAPSRC_A::GPIOC_5
    }
    #[doc = "Checks if the value of the field is `GPIOC_6`"]
    #[inline(always)]
    pub fn is_gpioc_6(&self) -> bool {
        **self == CAPSRC_A::GPIOC_6
    }
}
impl core::ops::Deref for CAPSRC_R {
    type Target = crate::FieldReader<u8, CAPSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPSRC` writer - Timer / Counter Capture Source"]
pub struct CAPSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPSRC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Core Clock"]
    #[inline(always)]
    pub fn cclk(self) -> &'a mut W {
        self.variant(CAPSRC_A::CCLK)
    }
    #[doc = "GPIO A, PIN 0"]
    #[inline(always)]
    pub fn gpioa_0(self) -> &'a mut W {
        self.variant(CAPSRC_A::GPIOA_0)
    }
    #[doc = "GPIO A, PIN 1"]
    #[inline(always)]
    pub fn gpioa_1(self) -> &'a mut W {
        self.variant(CAPSRC_A::GPIOA_1)
    }
    #[doc = "GPIO A, PIN 2"]
    #[inline(always)]
    pub fn gpioa_2(self) -> &'a mut W {
        self.variant(CAPSRC_A::GPIOA_2)
    }
    #[doc = "GPIO A, PIN 3"]
    #[inline(always)]
    pub fn gpioa_3(self) -> &'a mut W {
        self.variant(CAPSRC_A::GPIOA_3)
    }
    #[doc = "GPIO A, PIN 4"]
    #[inline(always)]
    pub fn gpioa_4(self) -> &'a mut W {
        self.variant(CAPSRC_A::GPIOA_4)
    }
    #[doc = "GPIO A, PIN 5"]
    #[inline(always)]
    pub fn gpioa_5(self) -> &'a mut W {
        self.variant(CAPSRC_A::GPIOA_5)
    }
    #[doc = "GPIO A, PIN 6"]
    #[inline(always)]
    pub fn gpioa_6(self) -> &'a mut W {
        self.variant(CAPSRC_A::GPIOA_6)
    }
    #[doc = "GPIO A, PIN 7"]
    #[inline(always)]
    pub fn gpioa_7(self) -> &'a mut W {
        self.variant(CAPSRC_A::GPIOA_7)
    }
    #[doc = "GPIO B, PIN 0"]
    #[inline(always)]
    pub fn gpiob_0(self) -> &'a mut W {
        self.variant(CAPSRC_A::GPIOB_0)
    }
    #[doc = "GPIO B, PIN 1"]
    #[inline(always)]
    pub fn gpiob_1(self) -> &'a mut W {
        self.variant(CAPSRC_A::GPIOB_1)
    }
    #[doc = "GPIO B, PIN 2"]
    #[inline(always)]
    pub fn gpiob_2(self) -> &'a mut W {
        self.variant(CAPSRC_A::GPIOB_2)
    }
    #[doc = "GPIO B, PIN 3"]
    #[inline(always)]
    pub fn gpiob_3(self) -> &'a mut W {
        self.variant(CAPSRC_A::GPIOB_3)
    }
    #[doc = "GPIO C, PIN 0"]
    #[inline(always)]
    pub fn gpioc_0(self) -> &'a mut W {
        self.variant(CAPSRC_A::GPIOC_0)
    }
    #[doc = "GPIO C, PIN 1"]
    #[inline(always)]
    pub fn gpioc_5(self) -> &'a mut W {
        self.variant(CAPSRC_A::GPIOC_5)
    }
    #[doc = "GPIO C, PIN 2"]
    #[inline(always)]
    pub fn gpioc_6(self) -> &'a mut W {
        self.variant(CAPSRC_A::GPIOC_6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Capture Edge, select which Edge should result in a counter increment or decrement\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CAPEDGE_A {
    #[doc = "0: Only rising edges result in a counter increment or decrement"]
    RISING = 0,
    #[doc = "1: Only falling edges  result in a counter increment or decrement"]
    FALLING = 1,
    #[doc = "2: Rising and falling edges result in a counter increment or decrement"]
    BOTH = 2,
}
impl From<CAPEDGE_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPEDGE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CAPEDGE` reader - Capture Edge, select which Edge should result in a counter increment or decrement"]
pub struct CAPEDGE_R(crate::FieldReader<u8, CAPEDGE_A>);
impl CAPEDGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CAPEDGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CAPEDGE_A> {
        match self.bits {
            0 => Some(CAPEDGE_A::RISING),
            1 => Some(CAPEDGE_A::FALLING),
            2 => Some(CAPEDGE_A::BOTH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        **self == CAPEDGE_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        **self == CAPEDGE_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        **self == CAPEDGE_A::BOTH
    }
}
impl core::ops::Deref for CAPEDGE_R {
    type Target = crate::FieldReader<u8, CAPEDGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPEDGE` writer - Capture Edge, select which Edge should result in a counter increment or decrement"]
pub struct CAPEDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPEDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPEDGE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Only rising edges result in a counter increment or decrement"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(CAPEDGE_A::RISING)
    }
    #[doc = "Only falling edges result in a counter increment or decrement"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(CAPEDGE_A::FALLING)
    }
    #[doc = "Rising and falling edges result in a counter increment or decrement"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(CAPEDGE_A::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Triggers an other Peripheral\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRGEXT_A {
    #[doc = "0: No Trigger is emitted"]
    NONE = 0,
    #[doc = "1: DMA Controller 1 is triggered, dependant on MODE"]
    DMA1 = 1,
    #[doc = "2: DMA Controller 2 is triggered, dependant on MODE"]
    DMA2 = 2,
    #[doc = "3: UART is triggered, dependant on MODE"]
    UART = 3,
}
impl From<TRGEXT_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGEXT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRGEXT` reader - Triggers an other Peripheral"]
pub struct TRGEXT_R(crate::FieldReader<u8, TRGEXT_A>);
impl TRGEXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRGEXT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGEXT_A {
        match self.bits {
            0 => TRGEXT_A::NONE,
            1 => TRGEXT_A::DMA1,
            2 => TRGEXT_A::DMA2,
            3 => TRGEXT_A::UART,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == TRGEXT_A::NONE
    }
    #[doc = "Checks if the value of the field is `DMA1`"]
    #[inline(always)]
    pub fn is_dma1(&self) -> bool {
        **self == TRGEXT_A::DMA1
    }
    #[doc = "Checks if the value of the field is `DMA2`"]
    #[inline(always)]
    pub fn is_dma2(&self) -> bool {
        **self == TRGEXT_A::DMA2
    }
    #[doc = "Checks if the value of the field is `UART`"]
    #[inline(always)]
    pub fn is_uart(&self) -> bool {
        **self == TRGEXT_A::UART
    }
}
impl core::ops::Deref for TRGEXT_R {
    type Target = crate::FieldReader<u8, TRGEXT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGEXT` writer - Triggers an other Peripheral"]
pub struct TRGEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGEXT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGEXT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No Trigger is emitted"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(TRGEXT_A::NONE)
    }
    #[doc = "DMA Controller 1 is triggered, dependant on MODE"]
    #[inline(always)]
    pub fn dma1(self) -> &'a mut W {
        self.variant(TRGEXT_A::DMA1)
    }
    #[doc = "DMA Controller 2 is triggered, dependant on MODE"]
    #[inline(always)]
    pub fn dma2(self) -> &'a mut W {
        self.variant(TRGEXT_A::DMA2)
    }
    #[doc = "UART is triggered, dependant on MODE"]
    #[inline(always)]
    pub fn uart(self) -> &'a mut W {
        self.variant(TRGEXT_A::UART)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Select RELOAD Register n to reload Timer on condition\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RELOAD_A {
    #[doc = "0: Selects Reload Register number 0"]
    RELOAD0 = 0,
    #[doc = "1: Selects Reload Register number 1"]
    RELOAD1 = 1,
    #[doc = "2: Selects Reload Register number 2"]
    RELOAD2 = 2,
    #[doc = "3: Selects Reload Register number 3"]
    RELOAD3 = 3,
}
impl From<RELOAD_A> for u8 {
    #[inline(always)]
    fn from(variant: RELOAD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RELOAD` reader - Select RELOAD Register n to reload Timer on condition"]
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
#[doc = "Field `RELOAD` writer - Select RELOAD Register n to reload Timer on condition"]
pub struct RELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RELOAD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Selects Reload Register number 0"]
    #[inline(always)]
    pub fn reload0(self) -> &'a mut W {
        self.variant(RELOAD_A::RELOAD0)
    }
    #[doc = "Selects Reload Register number 1"]
    #[inline(always)]
    pub fn reload1(self) -> &'a mut W {
        self.variant(RELOAD_A::RELOAD1)
    }
    #[doc = "Selects Reload Register number 2"]
    #[inline(always)]
    pub fn reload2(self) -> &'a mut W {
        self.variant(RELOAD_A::RELOAD2)
    }
    #[doc = "Selects Reload Register number 3"]
    #[inline(always)]
    pub fn reload3(self) -> &'a mut W {
        self.variant(RELOAD_A::RELOAD3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Selects, if Reload Register number is incremented, decremented or not modified\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IDR_A {
    #[doc = "0: Reload Register number does not change automatically"]
    KEEP = 0,
    #[doc = "1: Reload Register number is incremented on each match"]
    INCREMENT = 1,
    #[doc = "2: Reload Register number is decremented on each match"]
    DECREMENT = 2,
}
impl From<IDR_A> for u8 {
    #[inline(always)]
    fn from(variant: IDR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IDR` reader - Selects, if Reload Register number is incremented, decremented or not modified"]
pub struct IDR_R(crate::FieldReader<u8, IDR_A>);
impl IDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IDR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IDR_A> {
        match self.bits {
            0 => Some(IDR_A::KEEP),
            1 => Some(IDR_A::INCREMENT),
            2 => Some(IDR_A::DECREMENT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `KEEP`"]
    #[inline(always)]
    pub fn is_keep(&self) -> bool {
        **self == IDR_A::KEEP
    }
    #[doc = "Checks if the value of the field is `INCREMENT`"]
    #[inline(always)]
    pub fn is_increment(&self) -> bool {
        **self == IDR_A::INCREMENT
    }
    #[doc = "Checks if the value of the field is `DECREMENT`"]
    #[inline(always)]
    pub fn is_decrement(&self) -> bool {
        **self == IDR_A::DECREMENT
    }
}
impl core::ops::Deref for IDR_R {
    type Target = crate::FieldReader<u8, IDR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDR` writer - Selects, if Reload Register number is incremented, decremented or not modified"]
pub struct IDR_W<'a> {
    w: &'a mut W,
}
impl<'a> IDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Reload Register number does not change automatically"]
    #[inline(always)]
    pub fn keep(self) -> &'a mut W {
        self.variant(IDR_A::KEEP)
    }
    #[doc = "Reload Register number is incremented on each match"]
    #[inline(always)]
    pub fn increment(self) -> &'a mut W {
        self.variant(IDR_A::INCREMENT)
    }
    #[doc = "Reload Register number is decremented on each match"]
    #[inline(always)]
    pub fn decrement(self) -> &'a mut W {
        self.variant(IDR_A::DECREMENT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Starts and Stops the Timer / Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S_A {
    #[doc = "0: Timer / Counter is stopped"]
    STOP = 0,
    #[doc = "1: Timer / Counter is started"]
    START = 1,
}
impl From<S_A> for bool {
    #[inline(always)]
    fn from(variant: S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S` reader - Starts and Stops the Timer / Counter"]
pub struct S_R(crate::FieldReader<bool, S_A>);
impl S_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        S_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S_A {
        match self.bits {
            false => S_A::STOP,
            true => S_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == S_A::STOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == S_A::START
    }
}
impl core::ops::Deref for S_R {
    type Target = crate::FieldReader<bool, S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S` writer - Starts and Stops the Timer / Counter"]
pub struct S_W<'a> {
    w: &'a mut W,
}
impl<'a> S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer / Counter is stopped"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(S_A::STOP)
    }
    #[doc = "Timer / Counter is started"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(S_A::START)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Counting direction"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Operation Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Use Prescaler"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Timer / Counter Source Divider"]
    #[inline(always)]
    pub fn cntsrc(&self) -> CNTSRC_R {
        CNTSRC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Timer / Counter Capture Source"]
    #[inline(always)]
    pub fn capsrc(&self) -> CAPSRC_R {
        CAPSRC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Capture Edge, select which Edge should result in a counter increment or decrement"]
    #[inline(always)]
    pub fn capedge(&self) -> CAPEDGE_R {
        CAPEDGE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Triggers an other Peripheral"]
    #[inline(always)]
    pub fn trgext(&self) -> TRGEXT_R {
        TRGEXT_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Select RELOAD Register n to reload Timer on condition"]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Selects, if Reload Register number is incremented, decremented or not modified"]
    #[inline(always)]
    pub fn idr(&self) -> IDR_R {
        IDR_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bit 31 - Starts and Stops the Timer / Counter"]
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - Reset Timer"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W {
        RST_W { w: self }
    }
    #[doc = "Bits 2:3 - Counting direction"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W {
        CNT_W { w: self }
    }
    #[doc = "Bits 4:6 - Operation Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 7 - Use Prescaler"]
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W {
        PSC_W { w: self }
    }
    #[doc = "Bits 8:11 - Timer / Counter Source Divider"]
    #[inline(always)]
    pub fn cntsrc(&mut self) -> CNTSRC_W {
        CNTSRC_W { w: self }
    }
    #[doc = "Bits 12:15 - Timer / Counter Capture Source"]
    #[inline(always)]
    pub fn capsrc(&mut self) -> CAPSRC_W {
        CAPSRC_W { w: self }
    }
    #[doc = "Bits 16:17 - Capture Edge, select which Edge should result in a counter increment or decrement"]
    #[inline(always)]
    pub fn capedge(&mut self) -> CAPEDGE_W {
        CAPEDGE_W { w: self }
    }
    #[doc = "Bits 20:21 - Triggers an other Peripheral"]
    #[inline(always)]
    pub fn trgext(&mut self) -> TRGEXT_W {
        TRGEXT_W { w: self }
    }
    #[doc = "Bits 24:25 - Select RELOAD Register n to reload Timer on condition"]
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W {
        RELOAD_W { w: self }
    }
    #[doc = "Bits 26:27 - Selects, if Reload Register number is incremented, decremented or not modified"]
    #[inline(always)]
    pub fn idr(&mut self) -> IDR_W {
        IDR_W { w: self }
    }
    #[doc = "Bit 31 - Starts and Stops the Timer / Counter"]
    #[inline(always)]
    pub fn s(&mut self) -> S_W {
        S_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
