#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dac0_cfg: Dac0Cfg,
    dac0_value: Dac0Value,
}
impl RegisterBlock {
    #[doc = "0x00 - Регистр настойки ЦАП0"]
    #[inline(always)]
    pub const fn dac0_cfg(&self) -> &Dac0Cfg {
        &self.dac0_cfg
    }
    #[doc = "0x04 - Входные данные для ЦАП0"]
    #[inline(always)]
    pub const fn dac0_value(&self) -> &Dac0Value {
        &self.dac0_value
    }
}
#[doc = "DAC0_CFG (rw) register accessor: Регистр настойки ЦАП0\n\nYou can [`read`](crate::Reg::read) this register and get [`dac0_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac0_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac0_cfg`]
module"]
#[doc(alias = "DAC0_CFG")]
pub type Dac0Cfg = crate::Reg<dac0_cfg::Dac0CfgSpec>;
#[doc = "Регистр настойки ЦАП0"]
pub mod dac0_cfg;
#[doc = "DAC0_VALUE (rw) register accessor: Входные данные для ЦАП0\n\nYou can [`read`](crate::Reg::read) this register and get [`dac0_value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac0_value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac0_value`]
module"]
#[doc(alias = "DAC0_VALUE")]
pub type Dac0Value = crate::Reg<dac0_value::Dac0ValueSpec>;
#[doc = "Входные данные для ЦАП0"]
pub mod dac0_value;
