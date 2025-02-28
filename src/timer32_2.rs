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
    _reserved8: [u8; 0x60],
    ch1_cntr: Ch1Cntr,
    ch1_ocr: Ch1Ocr,
    ch1_icr: Ch1Icr,
    _reserved11: [u8; 0x04],
    _reserved_11_ch2_cntr: [u8; 0x04],
    ch2_ocr: Ch2Ocr,
    ch2_icr: Ch2Icr,
    _reserved14: [u8; 0x08],
    ch3_ocr: Ch3Ocr,
    ch3_icr: Ch3Icr,
    _reserved16: [u8; 0x04],
    ch4_cntr: Ch4Cntr,
    ch4_ocr: Ch4Ocr,
    ch4_icr: Ch4Icr,
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
    #[doc = "0x80 - Конфигурационный регистр 1 канала"]
    #[inline(always)]
    pub const fn ch1_cntr(&self) -> &Ch1Cntr {
        &self.ch1_cntr
    }
    #[doc = "0x84 - Значение сравнения 1 канала"]
    #[inline(always)]
    pub const fn ch1_ocr(&self) -> &Ch1Ocr {
        &self.ch1_ocr
    }
    #[doc = "0x88 - Значение захвата 1 канала"]
    #[inline(always)]
    pub const fn ch1_icr(&self) -> &Ch1Icr {
        &self.ch1_icr
    }
    #[doc = "0x90 - Конфигурационный регистр 3 канала"]
    #[inline(always)]
    pub const fn ch3_cntr(&self) -> &Ch3Cntr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(144).cast() }
    }
    #[doc = "0x90 - Конфигурационный регистр 2 канала"]
    #[inline(always)]
    pub const fn ch2_cntr(&self) -> &Ch2Cntr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(144).cast() }
    }
    #[doc = "0x94 - Значение сравнения 2 канала"]
    #[inline(always)]
    pub const fn ch2_ocr(&self) -> &Ch2Ocr {
        &self.ch2_ocr
    }
    #[doc = "0x98 - Значение захвата 2 канала"]
    #[inline(always)]
    pub const fn ch2_icr(&self) -> &Ch2Icr {
        &self.ch2_icr
    }
    #[doc = "0xa4 - Значение сравнения 3 канала"]
    #[inline(always)]
    pub const fn ch3_ocr(&self) -> &Ch3Ocr {
        &self.ch3_ocr
    }
    #[doc = "0xa8 - Значение захвата 3 канала"]
    #[inline(always)]
    pub const fn ch3_icr(&self) -> &Ch3Icr {
        &self.ch3_icr
    }
    #[doc = "0xb0 - Конфигурационный регистр 4 канала"]
    #[inline(always)]
    pub const fn ch4_cntr(&self) -> &Ch4Cntr {
        &self.ch4_cntr
    }
    #[doc = "0xb4 - Значение сравнения 4 канала"]
    #[inline(always)]
    pub const fn ch4_ocr(&self) -> &Ch4Ocr {
        &self.ch4_ocr
    }
    #[doc = "0xb8 - Значение захвата 4 канала"]
    #[inline(always)]
    pub const fn ch4_icr(&self) -> &Ch4Icr {
        &self.ch4_icr
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
#[doc = "CH1_CNTR (rw) register accessor: Конфигурационный регистр 1 канала\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_cntr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_cntr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_cntr`]
module"]
#[doc(alias = "CH1_CNTR")]
pub type Ch1Cntr = crate::Reg<ch1_cntr::Ch1CntrSpec>;
#[doc = "Конфигурационный регистр 1 канала"]
pub mod ch1_cntr;
#[doc = "CH1_OCR (rw) register accessor: Значение сравнения 1 канала\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_ocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_ocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_ocr`]
module"]
#[doc(alias = "CH1_OCR")]
pub type Ch1Ocr = crate::Reg<ch1_ocr::Ch1OcrSpec>;
#[doc = "Значение сравнения 1 канала"]
pub mod ch1_ocr;
#[doc = "CH1_ICR (rw) register accessor: Значение захвата 1 канала\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_icr`]
module"]
#[doc(alias = "CH1_ICR")]
pub type Ch1Icr = crate::Reg<ch1_icr::Ch1IcrSpec>;
#[doc = "Значение захвата 1 канала"]
pub mod ch1_icr;
#[doc = "CH2_CNTR (rw) register accessor: Конфигурационный регистр 2 канала\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_cntr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_cntr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_cntr`]
module"]
#[doc(alias = "CH2_CNTR")]
pub type Ch2Cntr = crate::Reg<ch2_cntr::Ch2CntrSpec>;
#[doc = "Конфигурационный регистр 2 канала"]
pub mod ch2_cntr;
#[doc = "CH2_OCR (rw) register accessor: Значение сравнения 2 канала\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_ocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_ocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_ocr`]
module"]
#[doc(alias = "CH2_OCR")]
pub type Ch2Ocr = crate::Reg<ch2_ocr::Ch2OcrSpec>;
#[doc = "Значение сравнения 2 канала"]
pub mod ch2_ocr;
#[doc = "CH2_ICR (rw) register accessor: Значение захвата 2 канала\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_icr`]
module"]
#[doc(alias = "CH2_ICR")]
pub type Ch2Icr = crate::Reg<ch2_icr::Ch2IcrSpec>;
#[doc = "Значение захвата 2 канала"]
pub mod ch2_icr;
#[doc = "CH3_CNTR (rw) register accessor: Конфигурационный регистр 3 канала\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_cntr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_cntr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_cntr`]
module"]
#[doc(alias = "CH3_CNTR")]
pub type Ch3Cntr = crate::Reg<ch3_cntr::Ch3CntrSpec>;
#[doc = "Конфигурационный регистр 3 канала"]
pub mod ch3_cntr;
#[doc = "CH3_OCR (rw) register accessor: Значение сравнения 3 канала\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_ocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_ocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_ocr`]
module"]
#[doc(alias = "CH3_OCR")]
pub type Ch3Ocr = crate::Reg<ch3_ocr::Ch3OcrSpec>;
#[doc = "Значение сравнения 3 канала"]
pub mod ch3_ocr;
#[doc = "CH3_ICR (rw) register accessor: Значение захвата 3 канала\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_icr`]
module"]
#[doc(alias = "CH3_ICR")]
pub type Ch3Icr = crate::Reg<ch3_icr::Ch3IcrSpec>;
#[doc = "Значение захвата 3 канала"]
pub mod ch3_icr;
#[doc = "CH4_CNTR (rw) register accessor: Конфигурационный регистр 4 канала\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_cntr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_cntr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_cntr`]
module"]
#[doc(alias = "CH4_CNTR")]
pub type Ch4Cntr = crate::Reg<ch4_cntr::Ch4CntrSpec>;
#[doc = "Конфигурационный регистр 4 канала"]
pub mod ch4_cntr;
#[doc = "CH4_OCR (rw) register accessor: Значение сравнения 4 канала\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_ocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_ocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_ocr`]
module"]
#[doc(alias = "CH4_OCR")]
pub type Ch4Ocr = crate::Reg<ch4_ocr::Ch4OcrSpec>;
#[doc = "Значение сравнения 4 канала"]
pub mod ch4_ocr;
#[doc = "CH4_ICR (rw) register accessor: Значение захвата 4 канала\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_icr`]
module"]
#[doc(alias = "CH4_ICR")]
pub type Ch4Icr = crate::Reg<ch4_icr::Ch4IcrSpec>;
#[doc = "Значение захвата 4 канала"]
pub mod ch4_icr;
