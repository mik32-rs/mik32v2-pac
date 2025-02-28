#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dac1_cfg: Dac1Cfg,
    dac1_value: Dac1Value,
}
impl RegisterBlock {
    #[doc = "0x00 - Регистр настойки ЦАП1"]
    #[inline(always)]
    pub const fn dac1_cfg(&self) -> &Dac1Cfg {
        &self.dac1_cfg
    }
    #[doc = "0x04 - Входные данные для ЦАП1"]
    #[inline(always)]
    pub const fn dac1_value(&self) -> &Dac1Value {
        &self.dac1_value
    }
}
#[doc = "DAC1_CFG (rw) register accessor: Регистр настойки ЦАП1\n\nYou can [`read`](crate::Reg::read) this register and get [`dac1_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac1_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac1_cfg`]
module"]
#[doc(alias = "DAC1_CFG")]
pub type Dac1Cfg = crate::Reg<dac1_cfg::Dac1CfgSpec>;
#[doc = "Регистр настойки ЦАП1"]
pub mod dac1_cfg;
#[doc = "DAC1_VALUE (rw) register accessor: Входные данные для ЦАП1\n\nYou can [`read`](crate::Reg::read) this register and get [`dac1_value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac1_value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac1_value`]
module"]
#[doc(alias = "DAC1_VALUE")]
pub type Dac1Value = crate::Reg<dac1_value::Dac1ValueSpec>;
#[doc = "Входные данные для ЦАП1"]
pub mod dac1_value;
