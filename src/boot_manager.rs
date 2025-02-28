#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    boot: Boot,
}
impl RegisterBlock {
    #[doc = "0x00 - Регистр режима загрузки"]
    #[inline(always)]
    pub const fn boot(&self) -> &Boot {
        &self.boot
    }
}
#[doc = "BOOT (rw) register accessor: Регистр режима загрузки\n\nYou can [`read`](crate::Reg::read) this register and get [`boot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boot`]
module"]
#[doc(alias = "BOOT")]
pub type Boot = crate::Reg<boot::BootSpec>;
#[doc = "Регистр режима загрузки"]
pub mod boot;
