#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sys_mask: SysMask,
    sys_level: SysLevel,
    sys_poweroff: SysPoweroff,
    power_switch: PowerSwitch,
    clocks_bu: ClocksBu,
    clocks_sys: ClocksSys,
    rtc_control: RtcControl,
    stop: Stop,
}
impl RegisterBlock {
    #[doc = "0x00 - Маски событий для включения и сброса системного домена"]
    #[inline(always)]
    pub const fn sys_mask(&self) -> &SysMask {
        &self.sys_mask
    }
    #[doc = "0x04 - Активные уровни событий для включения и сброса системного домена"]
    #[inline(always)]
    pub const fn sys_level(&self) -> &SysLevel {
        &self.sys_level
    }
    #[doc = "0x08 - Запись в регистр «1» отключает питание системного домена"]
    #[inline(always)]
    pub const fn sys_poweroff(&self) -> &SysPoweroff {
        &self.sys_poweroff
    }
    #[doc = "0x0c - Регистр управления и статуса схемы слежения за питанием"]
    #[inline(always)]
    pub const fn power_switch(&self) -> &PowerSwitch {
        &self.power_switch
    }
    #[doc = "0x10 - Регистр управления тактированием батарейного домена"]
    #[inline(always)]
    pub const fn clocks_bu(&self) -> &ClocksBu {
        &self.clocks_bu
    }
    #[doc = "0x14 - Регистр управления тактированием системного домена"]
    #[inline(always)]
    pub const fn clocks_sys(&self) -> &ClocksSys {
        &self.clocks_sys
    }
    #[doc = "0x18 - Сброс RTC происходит при записи “1”"]
    #[inline(always)]
    pub const fn rtc_control(&self) -> &RtcControl {
        &self.rtc_control
    }
    #[doc = "0x1c - Переход в режим “Стоп”. Осуществляется записью “1” Отключает тактирования системной шины."]
    #[inline(always)]
    pub const fn stop(&self) -> &Stop {
        &self.stop
    }
}
#[doc = "SYS_MASK (rw) register accessor: Маски событий для включения и сброса системного домена\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_mask`]
module"]
#[doc(alias = "SYS_MASK")]
pub type SysMask = crate::Reg<sys_mask::SysMaskSpec>;
#[doc = "Маски событий для включения и сброса системного домена"]
pub mod sys_mask;
#[doc = "SYS_LEVEL (rw) register accessor: Активные уровни событий для включения и сброса системного домена\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_level::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_level::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_level`]
module"]
#[doc(alias = "SYS_LEVEL")]
pub type SysLevel = crate::Reg<sys_level::SysLevelSpec>;
#[doc = "Активные уровни событий для включения и сброса системного домена"]
pub mod sys_level;
#[doc = "SYS_POWEROFF (w) register accessor: Запись в регистр «1» отключает питание системного домена\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_poweroff::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_poweroff`]
module"]
#[doc(alias = "SYS_POWEROFF")]
pub type SysPoweroff = crate::Reg<sys_poweroff::SysPoweroffSpec>;
#[doc = "Запись в регистр «1» отключает питание системного домена"]
pub mod sys_poweroff;
#[doc = "POWER_SWITCH (rw) register accessor: Регистр управления и статуса схемы слежения за питанием\n\nYou can [`read`](crate::Reg::read) this register and get [`power_switch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_switch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_switch`]
module"]
#[doc(alias = "POWER_SWITCH")]
pub type PowerSwitch = crate::Reg<power_switch::PowerSwitchSpec>;
#[doc = "Регистр управления и статуса схемы слежения за питанием"]
pub mod power_switch;
#[doc = "CLOCKS_BU (rw) register accessor: Регистр управления тактированием батарейного домена\n\nYou can [`read`](crate::Reg::read) this register and get [`clocks_bu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clocks_bu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clocks_bu`]
module"]
#[doc(alias = "CLOCKS_BU")]
pub type ClocksBu = crate::Reg<clocks_bu::ClocksBuSpec>;
#[doc = "Регистр управления тактированием батарейного домена"]
pub mod clocks_bu;
#[doc = "CLOCKS_SYS (rw) register accessor: Регистр управления тактированием системного домена\n\nYou can [`read`](crate::Reg::read) this register and get [`clocks_sys::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clocks_sys::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clocks_sys`]
module"]
#[doc(alias = "CLOCKS_SYS")]
pub type ClocksSys = crate::Reg<clocks_sys::ClocksSysSpec>;
#[doc = "Регистр управления тактированием системного домена"]
pub mod clocks_sys;
#[doc = "RTC_CONTROL (w) register accessor: Сброс RTC происходит при записи “1”\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_control::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_control`]
module"]
#[doc(alias = "RTC_CONTROL")]
pub type RtcControl = crate::Reg<rtc_control::RtcControlSpec>;
#[doc = "Сброс RTC происходит при записи “1”"]
pub mod rtc_control;
#[doc = "STOP (w) register accessor: Переход в режим “Стоп”. Осуществляется записью “1” Отключает тактирования системной шины.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stop`]
module"]
#[doc(alias = "STOP")]
pub type Stop = crate::Reg<stop::StopSpec>;
#[doc = "Переход в режим “Стоп”. Осуществляется записью “1” Отключает тактирования системной шины."]
pub mod stop;
