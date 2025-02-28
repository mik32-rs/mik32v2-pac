#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    value: Value,
    top: Top,
    prescale: Prescale,
    control: Control,
    enable: Enable,
    int_mask: IntMask,
    int_clear: IntClear,
    int_flag: IntFlag,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn value(&self) -> &Value {
        &self.value
    }
    #[doc = "0x04 - максимальное значение счетной последовательности"]
    #[inline(always)]
    pub const fn top(&self) -> &Top {
        &self.top
    }
    #[doc = "0x08 - значение делителя"]
    #[inline(always)]
    pub const fn prescale(&self) -> &Prescale {
        &self.prescale
    }
    #[doc = "0x0c - Конфигурационный регистр основного таймера"]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x10 - Регистр включения таймера"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x14 - Регистр маски прерываний"]
    #[inline(always)]
    pub const fn int_mask(&self) -> &IntMask {
        &self.int_mask
    }
    #[doc = "0x18 - Регистр сброса флагов прерываний"]
    #[inline(always)]
    pub const fn int_clear(&self) -> &IntClear {
        &self.int_clear
    }
    #[doc = "0x1c - Регистр флагов прерываний"]
    #[inline(always)]
    pub const fn int_flag(&self) -> &IntFlag {
        &self.int_flag
    }
}
#[doc = "VALUE (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@value`]
module"]
#[doc(alias = "VALUE")]
pub type Value = crate::Reg<value::ValueSpec>;
#[doc = ""]
pub mod value;
#[doc = "TOP (rw) register accessor: максимальное значение счетной последовательности\n\nYou can [`read`](crate::Reg::read) this register and get [`top::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`top::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@top`]
module"]
#[doc(alias = "TOP")]
pub type Top = crate::Reg<top::TopSpec>;
#[doc = "максимальное значение счетной последовательности"]
pub mod top;
#[doc = "PRESCALE (rw) register accessor: значение делителя\n\nYou can [`read`](crate::Reg::read) this register and get [`prescale::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prescale::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prescale`]
module"]
#[doc(alias = "PRESCALE")]
pub type Prescale = crate::Reg<prescale::PrescaleSpec>;
#[doc = "значение делителя"]
pub mod prescale;
#[doc = "CONTROL (rw) register accessor: Конфигурационный регистр основного таймера\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "Конфигурационный регистр основного таймера"]
pub mod control;
#[doc = "ENABLE (rw) register accessor: Регистр включения таймера\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Регистр включения таймера"]
pub mod enable;
#[doc = "INT_MASK (rw) register accessor: Регистр маски прерываний\n\nYou can [`read`](crate::Reg::read) this register and get [`int_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_mask`]
module"]
#[doc(alias = "INT_MASK")]
pub type IntMask = crate::Reg<int_mask::IntMaskSpec>;
#[doc = "Регистр маски прерываний"]
pub mod int_mask;
#[doc = "INT_CLEAR (rw) register accessor: Регистр сброса флагов прерываний\n\nYou can [`read`](crate::Reg::read) this register and get [`int_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clear`]
module"]
#[doc(alias = "INT_CLEAR")]
pub type IntClear = crate::Reg<int_clear::IntClearSpec>;
#[doc = "Регистр сброса флагов прерываний"]
pub mod int_clear;
#[doc = "INT_FLAG (rw) register accessor: Регистр флагов прерываний\n\nYou can [`read`](crate::Reg::read) this register and get [`int_flag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_flag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_flag`]
module"]
#[doc(alias = "INT_FLAG")]
pub type IntFlag = crate::Reg<int_flag::IntFlagSpec>;
#[doc = "Регистр флагов прерываний"]
pub mod int_flag;
