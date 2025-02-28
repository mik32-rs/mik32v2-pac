#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    timeout: Timeout,
    irq_clear: IrqClear,
    enable: Enable,
}
impl RegisterBlock {
    #[doc = "0x00 - Определеяет количество циклов ожидания до формирования ошибки. Значения от 0 до 15. Количество циклов ожидания вычисляется как 2^TIMEOUT."]
    #[inline(always)]
    pub const fn timeout(&self) -> &Timeout {
        &self.timeout
    }
    #[doc = "0x04 - Сброс прерываний"]
    #[inline(always)]
    pub const fn irq_clear(&self) -> &IrqClear {
        &self.irq_clear
    }
    #[doc = "0x08 - Запуск/отключение мониторов шины"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
}
#[doc = "TIMEOUT (rw) register accessor: Определеяет количество циклов ожидания до формирования ошибки. Значения от 0 до 15. Количество циклов ожидания вычисляется как 2^TIMEOUT.\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeout`]
module"]
#[doc(alias = "TIMEOUT")]
pub type Timeout = crate::Reg<timeout::TimeoutSpec>;
#[doc = "Определеяет количество циклов ожидания до формирования ошибки. Значения от 0 до 15. Количество циклов ожидания вычисляется как 2^TIMEOUT."]
pub mod timeout;
#[doc = "IRQ_CLEAR (w) register accessor: Сброс прерываний\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_clear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_clear`]
module"]
#[doc(alias = "IRQ_CLEAR")]
pub type IrqClear = crate::Reg<irq_clear::IrqClearSpec>;
#[doc = "Сброс прерываний"]
pub mod irq_clear;
#[doc = "ENABLE (rw) register accessor: Запуск/отключение мониторов шины\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Запуск/отключение мониторов шины"]
pub mod enable;
