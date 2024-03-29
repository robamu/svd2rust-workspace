#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    _reserved2: [u8; 0x0a],
    #[doc = "0x10 - Interrupt Register"]
    pub int: crate::Reg<int::INT_SPEC>,
    _reserved3: [u8; 0x0e],
    #[doc = "0x20 - The Counter Register reflects the actual Value of the Timer/Counter"]
    pub count: crate::Reg<count::COUNT_SPEC>,
    #[doc = "0x24 - The Match Register stores the compare Value for the MATCH condition"]
    pub match_: crate::Reg<match_::MATCH_SPEC>,
    _reserved_5_prescale: [u8; 0x04],
    _reserved6: [u8; 0x24],
    #[doc = "0x50..0x60 - The Reload Register stores the Value the COUNT Register gets reloaded on a when a condition was met."]
    pub reload: [crate::Reg<reload::RELOAD_SPEC>; 4],
}
impl RegisterBlock {
    #[doc = "0x28 - The Prescale Register stores the Value for the prescaler. The cont event gets divided by this value"]
    #[inline(always)]
    pub fn prescale_wr(&self) -> &crate::Reg<prescale_wr::PRESCALE_WR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize)
                as *const crate::Reg<prescale_wr::PRESCALE_WR_SPEC>)
        }
    }
    #[doc = "0x28 - The Prescale Register stores the Value for the prescaler. The cont event gets divided by this value"]
    #[inline(always)]
    pub fn prescale_rd(&self) -> &crate::Reg<prescale_rd::PRESCALE_RD_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize)
                as *const crate::Reg<prescale_rd::PRESCALE_RD_SPEC>)
        }
    }
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "INT register accessor: an alias for `Reg<INT_SPEC>`"]
pub type INT = crate::Reg<int::INT_SPEC>;
#[doc = "Interrupt Register"]
pub mod int;
#[doc = "COUNT register accessor: an alias for `Reg<COUNT_SPEC>`"]
pub type COUNT = crate::Reg<count::COUNT_SPEC>;
#[doc = "The Counter Register reflects the actual Value of the Timer/Counter"]
pub mod count;
#[doc = "MATCH register accessor: an alias for `Reg<MATCH_SPEC>`"]
pub type MATCH = crate::Reg<match_::MATCH_SPEC>;
#[doc = "The Match Register stores the compare Value for the MATCH condition"]
pub mod match_;
#[doc = "PRESCALE_RD register accessor: an alias for `Reg<PRESCALE_RD_SPEC>`"]
pub type PRESCALE_RD = crate::Reg<prescale_rd::PRESCALE_RD_SPEC>;
#[doc = "The Prescale Register stores the Value for the prescaler. The cont event gets divided by this value"]
pub mod prescale_rd;
#[doc = "PRESCALE_WR register accessor: an alias for `Reg<PRESCALE_WR_SPEC>`"]
pub type PRESCALE_WR = crate::Reg<prescale_wr::PRESCALE_WR_SPEC>;
#[doc = "The Prescale Register stores the Value for the prescaler. The cont event gets divided by this value"]
pub mod prescale_wr;
#[doc = "RELOAD register accessor: an alias for `Reg<RELOAD_SPEC>`"]
pub type RELOAD = crate::Reg<reload::RELOAD_SPEC>;
#[doc = "The Reload Register stores the Value the COUNT Register gets reloaded on a when a condition was met."]
pub mod reload;
