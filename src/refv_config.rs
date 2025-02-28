#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ref_clb: RefClb,
}
impl RegisterBlock {
    #[doc = "0x00 - Управление калибруемыми источниками тока и напряжения"]
    #[inline(always)]
    pub const fn ref_clb(&self) -> &RefClb {
        &self.ref_clb
    }
}
#[doc = "REF_CLB (rw) register accessor: Управление калибруемыми источниками тока и напряжения\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_clb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_clb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ref_clb`]
module"]
#[doc(alias = "REF_CLB")]
pub type RefClb = crate::Reg<ref_clb::RefClbSpec>;
#[doc = "Управление калибруемыми источниками тока и напряжения"]
pub mod ref_clb;
