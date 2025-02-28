#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tsens_cfg: TsensCfg,
    tsens_treshold: TsensTreshold,
    tsens_irq: TsensIrq,
    tsens_clear_irq: TsensClearIrq,
    tsens_value: TsensValue,
    tsens_single: TsensSingle,
    _reserved6: [u8; 0x1c],
    tsens_continuous: TsensContinuous,
}
impl RegisterBlock {
    #[doc = "0x00 - Регистр настроек"]
    #[inline(always)]
    pub const fn tsens_cfg(&self) -> &TsensCfg {
        &self.tsens_cfg
    }
    #[doc = "0x04 - Регистр пороговых значений"]
    #[inline(always)]
    pub const fn tsens_treshold(&self) -> &TsensTreshold {
        &self.tsens_treshold
    }
    #[doc = "0x08 - Регистр прерываний"]
    #[inline(always)]
    pub const fn tsens_irq(&self) -> &TsensIrq {
        &self.tsens_irq
    }
    #[doc = "0x0c - Регистр сброса прерываний"]
    #[inline(always)]
    pub const fn tsens_clear_irq(&self) -> &TsensClearIrq {
        &self.tsens_clear_irq
    }
    #[doc = "0x10 - Регистр данных"]
    #[inline(always)]
    pub const fn tsens_value(&self) -> &TsensValue {
        &self.tsens_value
    }
    #[doc = "0x14 - Регистр запуска однократного измерения"]
    #[inline(always)]
    pub const fn tsens_single(&self) -> &TsensSingle {
        &self.tsens_single
    }
    #[doc = "0x34 - Регистр запуска непрерываного измерения"]
    #[inline(always)]
    pub const fn tsens_continuous(&self) -> &TsensContinuous {
        &self.tsens_continuous
    }
}
#[doc = "TSENS_CFG (rw) register accessor: Регистр настроек\n\nYou can [`read`](crate::Reg::read) this register and get [`tsens_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsens_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsens_cfg`]
module"]
#[doc(alias = "TSENS_CFG")]
pub type TsensCfg = crate::Reg<tsens_cfg::TsensCfgSpec>;
#[doc = "Регистр настроек"]
pub mod tsens_cfg;
#[doc = "TSENS_TRESHOLD (rw) register accessor: Регистр пороговых значений\n\nYou can [`read`](crate::Reg::read) this register and get [`tsens_treshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsens_treshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsens_treshold`]
module"]
#[doc(alias = "TSENS_TRESHOLD")]
pub type TsensTreshold = crate::Reg<tsens_treshold::TsensTresholdSpec>;
#[doc = "Регистр пороговых значений"]
pub mod tsens_treshold;
#[doc = "TSENS_IRQ (rw) register accessor: Регистр прерываний\n\nYou can [`read`](crate::Reg::read) this register and get [`tsens_irq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsens_irq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsens_irq`]
module"]
#[doc(alias = "TSENS_IRQ")]
pub type TsensIrq = crate::Reg<tsens_irq::TsensIrqSpec>;
#[doc = "Регистр прерываний"]
pub mod tsens_irq;
#[doc = "TSENS_CLEAR_IRQ (w) register accessor: Регистр сброса прерываний\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsens_clear_irq::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsens_clear_irq`]
module"]
#[doc(alias = "TSENS_CLEAR_IRQ")]
pub type TsensClearIrq = crate::Reg<tsens_clear_irq::TsensClearIrqSpec>;
#[doc = "Регистр сброса прерываний"]
pub mod tsens_clear_irq;
#[doc = "TSENS_VALUE (r) register accessor: Регистр данных\n\nYou can [`read`](crate::Reg::read) this register and get [`tsens_value::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsens_value`]
module"]
#[doc(alias = "TSENS_VALUE")]
pub type TsensValue = crate::Reg<tsens_value::TsensValueSpec>;
#[doc = "Регистр данных"]
pub mod tsens_value;
#[doc = "TSENS_SINGLE (w) register accessor: Регистр запуска однократного измерения\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsens_single::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsens_single`]
module"]
#[doc(alias = "TSENS_SINGLE")]
pub type TsensSingle = crate::Reg<tsens_single::TsensSingleSpec>;
#[doc = "Регистр запуска однократного измерения"]
pub mod tsens_single;
#[doc = "TSENS_CONTINUOUS (w) register accessor: Регистр запуска непрерываного измерения\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsens_continuous::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsens_continuous`]
module"]
#[doc(alias = "TSENS_CONTINUOUS")]
pub type TsensContinuous = crate::Reg<tsens_continuous::TsensContinuousSpec>;
#[doc = "Регистр запуска непрерываного измерения"]
pub mod tsens_continuous;
