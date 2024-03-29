#[doc = "Register `PRESCALE_RD` reader"]
pub struct R(crate::R<PRESCALE_RD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESCALE_RD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESCALE_RD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESCALE_RD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "The Prescale Register stores the Value for the prescaler. The cont event gets divided by this value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prescale_rd](index.html) module"]
pub struct PRESCALE_RD_SPEC;
impl crate::RegisterSpec for PRESCALE_RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prescale_rd::R](R) reader structure"]
impl crate::Readable for PRESCALE_RD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRESCALE_RD to value 0"]
impl crate::Resettable for PRESCALE_RD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
