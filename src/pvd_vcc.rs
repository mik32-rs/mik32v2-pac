#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dpf_value: DpfValue,
    config: Config,
    status: Status,
}
impl RegisterBlock {
    #[doc = "0x00 - Настройка цифрового фильтра"]
    #[inline(always)]
    pub const fn dpf_value(&self) -> &DpfValue {
        &self.dpf_value
    }
    #[doc = "0x04 - Регистр настроек"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x08 - Регистр статуса"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
}
#[doc = "DPF_VALUE (rw) register accessor: Настройка цифрового фильтра\n\nYou can [`read`](crate::Reg::read) this register and get [`dpf_value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpf_value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpf_value`]
module"]
#[doc(alias = "DPF_VALUE")]
pub type DpfValue = crate::Reg<dpf_value::DpfValueSpec>;
#[doc = "Настройка цифрового фильтра"]
pub mod dpf_value;
#[doc = "CONFIG (rw) register accessor: Регистр настроек\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Регистр настроек"]
pub mod config;
#[doc = "STATUS (r) register accessor: Регистр статуса\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Регистр статуса"]
pub mod status;
