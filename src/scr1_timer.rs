#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    timer_ctrl: TimerCtrl,
    timer_div: TimerDiv,
    mtime: Mtime,
    mtimeh: Mtimeh,
    mtimecmp: Mtimecmp,
    mtimecmph: Mtimecmph,
}
impl RegisterBlock {
    #[doc = "0x00 - Регистр конфигурации"]
    #[inline(always)]
    pub const fn timer_ctrl(&self) -> &TimerCtrl {
        &self.timer_ctrl
    }
    #[doc = "0x04 - Делитель частоты. Счет идет каждые DIV+1 такта частоты"]
    #[inline(always)]
    pub const fn timer_div(&self) -> &TimerDiv {
        &self.timer_div
    }
    #[doc = "0x08 - Счетчик таймера, младшее слово"]
    #[inline(always)]
    pub const fn mtime(&self) -> &Mtime {
        &self.mtime
    }
    #[doc = "0x0c - Счетчик таймера, старшее слово"]
    #[inline(always)]
    pub const fn mtimeh(&self) -> &Mtimeh {
        &self.mtimeh
    }
    #[doc = "0x10 - Регистр сравнения, младшее слово"]
    #[inline(always)]
    pub const fn mtimecmp(&self) -> &Mtimecmp {
        &self.mtimecmp
    }
    #[doc = "0x14 - Регистр сравнения, старшее слово"]
    #[inline(always)]
    pub const fn mtimecmph(&self) -> &Mtimecmph {
        &self.mtimecmph
    }
}
#[doc = "TIMER_CTRL (rw) register accessor: Регистр конфигурации\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_ctrl`]
module"]
#[doc(alias = "TIMER_CTRL")]
pub type TimerCtrl = crate::Reg<timer_ctrl::TimerCtrlSpec>;
#[doc = "Регистр конфигурации"]
pub mod timer_ctrl;
#[doc = "TIMER_DIV (rw) register accessor: Делитель частоты. Счет идет каждые DIV+1 такта частоты\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_div`]
module"]
#[doc(alias = "TIMER_DIV")]
pub type TimerDiv = crate::Reg<timer_div::TimerDivSpec>;
#[doc = "Делитель частоты. Счет идет каждые DIV+1 такта частоты"]
pub mod timer_div;
#[doc = "MTIME (rw) register accessor: Счетчик таймера, младшее слово\n\nYou can [`read`](crate::Reg::read) this register and get [`mtime::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtime::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtime`]
module"]
#[doc(alias = "MTIME")]
pub type Mtime = crate::Reg<mtime::MtimeSpec>;
#[doc = "Счетчик таймера, младшее слово"]
pub mod mtime;
#[doc = "MTIMEH (rw) register accessor: Счетчик таймера, старшее слово\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimeh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimeh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimeh`]
module"]
#[doc(alias = "MTIMEH")]
pub type Mtimeh = crate::Reg<mtimeh::MtimehSpec>;
#[doc = "Счетчик таймера, старшее слово"]
pub mod mtimeh;
#[doc = "MTIMECMP (rw) register accessor: Регистр сравнения, младшее слово\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimecmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimecmp`]
module"]
#[doc(alias = "MTIMECMP")]
pub type Mtimecmp = crate::Reg<mtimecmp::MtimecmpSpec>;
#[doc = "Регистр сравнения, младшее слово"]
pub mod mtimecmp;
#[doc = "MTIMECMPH (rw) register accessor: Регистр сравнения, старшее слово\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimecmph::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimecmph::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimecmph`]
module"]
#[doc(alias = "MTIMECMPH")]
pub type Mtimecmph = crate::Reg<mtimecmph::MtimecmphSpec>;
#[doc = "Регистр сравнения, старшее слово"]
pub mod mtimecmph;
