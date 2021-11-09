#[doc = "Register `PRESCALE_WR` writer"]
pub struct W(crate::W<PRESCALE_WR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESCALE_WR_SPEC>;
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
impl From<crate::W<PRESCALE_WR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESCALE_WR_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The Prescale Register stores the Value for the prescaler. The cont event gets divided by this value\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prescale_wr](index.html) module"]
pub struct PRESCALE_WR_SPEC;
impl crate::RegisterSpec for PRESCALE_WR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [prescale_wr::W](W) writer structure"]
impl crate::Writable for PRESCALE_WR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRESCALE_WR to value 0"]
impl crate::Resettable for PRESCALE_WR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
