#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_set: [u8; 0x04],
    clear: Clear,
    direction_out: DirectionOut,
    direction_in: DirectionIn,
    output: Output,
    control: Control,
}
impl RegisterBlock {
    #[doc = "0x00 - Текущее состояние выводов"]
    #[inline(always)]
    pub const fn state(&self) -> &State {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - Установка выводов в «1»"]
    #[inline(always)]
    pub const fn set(&self) -> &Set {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x04 - Текущее состояние прерываний / Установка «0» При чтении – текущее состояние запросов прерываний. При записи бита «1» соответствующий биту вывод устанавливается в «0»"]
    #[inline(always)]
    pub const fn clear(&self) -> &Clear {
        &self.clear
    }
    #[doc = "0x08 - Установка направления выводов как выход При чтении – текущее направление выводов: 0 – выход; 1 – вход. Запись «1» устанавливает соответствующий вывод как «выход»"]
    #[inline(always)]
    pub const fn direction_out(&self) -> &DirectionOut {
        &self.direction_out
    }
    #[doc = "0x0c - Установка направления выводов как вход При чтении – текущее направление выводов: 0 – выход; 1 – вход. Запись «1» устанавливает соответствующий вывод как «вход»"]
    #[inline(always)]
    pub const fn direction_in(&self) -> &DirectionIn {
        &self.direction_in
    }
    #[doc = "0x10 - Выходной регистр Чтение возвращает содержимое выходного регистра независимо от текущего направления выводов Запись устанавливает значения всех битов выходного регистра"]
    #[inline(always)]
    pub const fn output(&self) -> &Output {
        &self.output
    }
    #[doc = "0x14 - Тестовый режим Запись «1» в \\[0:0\\]
разряд – включение тестового режима (выходы замкнуты на входы внутри модуля) Запись «0» в \\[0:0\\]
разряд – нормальный режим"]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
}
#[doc = "SET (w) register accessor: Установка выводов в «1»\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@set`]
module"]
#[doc(alias = "SET")]
pub type Set = crate::Reg<set::SetSpec>;
#[doc = "Установка выводов в «1»"]
pub mod set;
#[doc = "STATE (r) register accessor: Текущее состояние выводов\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`]
module"]
#[doc(alias = "STATE")]
pub type State = crate::Reg<state::StateSpec>;
#[doc = "Текущее состояние выводов"]
pub mod state;
#[doc = "CLEAR (w) register accessor: Текущее состояние прерываний / Установка «0» При чтении – текущее состояние запросов прерываний. При записи бита «1» соответствующий биту вывод устанавливается в «0»\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear`]
module"]
#[doc(alias = "CLEAR")]
pub type Clear = crate::Reg<clear::ClearSpec>;
#[doc = "Текущее состояние прерываний / Установка «0» При чтении – текущее состояние запросов прерываний. При записи бита «1» соответствующий биту вывод устанавливается в «0»"]
pub mod clear;
#[doc = "DIRECTION_OUT (rw) register accessor: Установка направления выводов как выход При чтении – текущее направление выводов: 0 – выход; 1 – вход. Запись «1» устанавливает соответствующий вывод как «выход»\n\nYou can [`read`](crate::Reg::read) this register and get [`direction_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`direction_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@direction_out`]
module"]
#[doc(alias = "DIRECTION_OUT")]
pub type DirectionOut = crate::Reg<direction_out::DirectionOutSpec>;
#[doc = "Установка направления выводов как выход При чтении – текущее направление выводов: 0 – выход; 1 – вход. Запись «1» устанавливает соответствующий вывод как «выход»"]
pub mod direction_out;
#[doc = "DIRECTION_IN (rw) register accessor: Установка направления выводов как вход При чтении – текущее направление выводов: 0 – выход; 1 – вход. Запись «1» устанавливает соответствующий вывод как «вход»\n\nYou can [`read`](crate::Reg::read) this register and get [`direction_in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`direction_in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@direction_in`]
module"]
#[doc(alias = "DIRECTION_IN")]
pub type DirectionIn = crate::Reg<direction_in::DirectionInSpec>;
#[doc = "Установка направления выводов как вход При чтении – текущее направление выводов: 0 – выход; 1 – вход. Запись «1» устанавливает соответствующий вывод как «вход»"]
pub mod direction_in;
#[doc = "OUTPUT (rw) register accessor: Выходной регистр Чтение возвращает содержимое выходного регистра независимо от текущего направления выводов Запись устанавливает значения всех битов выходного регистра\n\nYou can [`read`](crate::Reg::read) this register and get [`output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@output`]
module"]
#[doc(alias = "OUTPUT")]
pub type Output = crate::Reg<output::OutputSpec>;
#[doc = "Выходной регистр Чтение возвращает содержимое выходного регистра независимо от текущего направления выводов Запись устанавливает значения всех битов выходного регистра"]
pub mod output;
#[doc = "CONTROL (rw) register accessor: Тестовый режим Запись «1» в \\[0:0\\]
разряд – включение тестового режима (выходы замкнуты на входы внутри модуля) Запись «0» в \\[0:0\\]
разряд – нормальный режим\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "Тестовый режим Запись «1» в \\[0:0\\]
разряд – включение тестового режима (выходы замкнуты на входы внутри модуля) Запись «0» в \\[0:0\\]
разряд – нормальный режим"]
pub mod control;
