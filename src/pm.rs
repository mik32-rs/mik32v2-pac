#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    div_ahb: DivAhb,
    div_apb_m: DivApbM,
    div_apb_p: DivApbP,
    clk_ahb_set: ClkAhbSet,
    clk_ahb_clear: ClkAhbClear,
    clk_apb_m_set: ClkApbMSet,
    clk_apb_m_clear: ClkApbMClear,
    clk_apb_p_set: ClkApbPSet,
    clk_apb_p_clear: ClkApbPClear,
    ahb_mux: AhbMux,
    wdt_clk_mux: WdtClkMux,
    cpu_rtc_clk_mux: CpuRtcClkMux,
    timer_cfg: TimerCfg,
    freq_mask: FreqMask,
    freq_status: FreqStatus,
    sleep_mode: SleepMode,
}
impl RegisterBlock {
    #[doc = "0x00 - Задает значение делителя шины AHB. Частота шины AHB (FAHB) рассчитывается, как FSYS/( DIV_AHB+1)"]
    #[inline(always)]
    pub const fn div_ahb(&self) -> &DivAhb {
        &self.div_ahb
    }
    #[doc = "0x04 - Задает значение делителя шины APB_M. Частота шины APB_M (FAPM_M) рассчитывается, как FAPB/( Div_APM_M+1)"]
    #[inline(always)]
    pub const fn div_apb_m(&self) -> &DivApbM {
        &self.div_apb_m
    }
    #[doc = "0x08 - Задает значение делителя шины APB_P. Частота шины APB_P (FAPM_P) рассчитывается, как FAPB/( Div_APM_P+1)"]
    #[inline(always)]
    pub const fn div_apb_p(&self) -> &DivApbP {
        &self.div_apb_p
    }
    #[doc = "0x0c - Регистр включения тактированием устройств на шине AHB"]
    #[inline(always)]
    pub const fn clk_ahb_set(&self) -> &ClkAhbSet {
        &self.clk_ahb_set
    }
    #[doc = "0x10 - Регистр выключения тактированием устройств на шине AHB. Каждому биту соответствует устройство, аналогично CLK_AHB_SET"]
    #[inline(always)]
    pub const fn clk_ahb_clear(&self) -> &ClkAhbClear {
        &self.clk_ahb_clear
    }
    #[doc = "0x14 - Регистр включения тактированием устройств на шине APB_M. Каждому биту соответствует устройство"]
    #[inline(always)]
    pub const fn clk_apb_m_set(&self) -> &ClkApbMSet {
        &self.clk_apb_m_set
    }
    #[doc = "0x18 - Регистр выключения тактированием устройств на шине APB_M. Каждому биту соответствует устройство, аналогично Clk_APB_M_Set"]
    #[inline(always)]
    pub const fn clk_apb_m_clear(&self) -> &ClkApbMClear {
        &self.clk_apb_m_clear
    }
    #[doc = "0x1c - Регистр включения тактированием устройств на шине APB_P. Каждому биту соответствует одно устройство"]
    #[inline(always)]
    pub const fn clk_apb_p_set(&self) -> &ClkApbPSet {
        &self.clk_apb_p_set
    }
    #[doc = "0x20 - Регистр выключения тактированием устройств на шине APB_P. Каждому биту соответствует устройство, аналогично CLK_APB_P_SET"]
    #[inline(always)]
    pub const fn clk_apb_p_clear(&self) -> &ClkApbPClear {
        &self.clk_apb_p_clear
    }
    #[doc = "0x24 - Настройка источника тактирования системы"]
    #[inline(always)]
    pub const fn ahb_mux(&self) -> &AhbMux {
        &self.ahb_mux
    }
    #[doc = "0x28 - Выбор источника тактирования сторожевого таймера: 0 – внешний OSC32M; 1 – внутренний HSI32M; 2 – внешний OSC32K; 3 – внутренний LSI32К;"]
    #[inline(always)]
    pub const fn wdt_clk_mux(&self) -> &WdtClkMux {
        &self.wdt_clk_mux
    }
    #[doc = "0x2c - Выбор источника тактирования RTC для системного таймера в составе ядра"]
    #[inline(always)]
    pub const fn cpu_rtc_clk_mux(&self) -> &CpuRtcClkMux {
        &self.cpu_rtc_clk_mux
    }
    #[doc = "0x30 - Выбор источника тактирования для таймеров"]
    #[inline(always)]
    pub const fn timer_cfg(&self) -> &TimerCfg {
        &self.timer_cfg
    }
    #[doc = "0x34 - Настройки прерываний монитора частоты"]
    #[inline(always)]
    pub const fn freq_mask(&self) -> &FreqMask {
        &self.freq_mask
    }
    #[doc = "0x38 - Статус монитора частоты"]
    #[inline(always)]
    pub const fn freq_status(&self) -> &FreqStatus {
        &self.freq_status
    }
    #[doc = "0x3c - Переход в спящий режим осуществляется записью в данный регистр. При записи отключается тактирование ядра. В зависимости от записываемого значения отключается тактирование модулей"]
    #[inline(always)]
    pub const fn sleep_mode(&self) -> &SleepMode {
        &self.sleep_mode
    }
}
#[doc = "DIV_AHB (rw) register accessor: Задает значение делителя шины AHB. Частота шины AHB (FAHB) рассчитывается, как FSYS/( DIV_AHB+1)\n\nYou can [`read`](crate::Reg::read) this register and get [`div_ahb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div_ahb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div_ahb`]
module"]
#[doc(alias = "DIV_AHB")]
pub type DivAhb = crate::Reg<div_ahb::DivAhbSpec>;
#[doc = "Задает значение делителя шины AHB. Частота шины AHB (FAHB) рассчитывается, как FSYS/( DIV_AHB+1)"]
pub mod div_ahb;
#[doc = "DIV_APB_M (rw) register accessor: Задает значение делителя шины APB_M. Частота шины APB_M (FAPM_M) рассчитывается, как FAPB/( Div_APM_M+1)\n\nYou can [`read`](crate::Reg::read) this register and get [`div_apb_m::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div_apb_m::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div_apb_m`]
module"]
#[doc(alias = "DIV_APB_M")]
pub type DivApbM = crate::Reg<div_apb_m::DivApbMSpec>;
#[doc = "Задает значение делителя шины APB_M. Частота шины APB_M (FAPM_M) рассчитывается, как FAPB/( Div_APM_M+1)"]
pub mod div_apb_m;
#[doc = "DIV_APB_P (rw) register accessor: Задает значение делителя шины APB_P. Частота шины APB_P (FAPM_P) рассчитывается, как FAPB/( Div_APM_P+1)\n\nYou can [`read`](crate::Reg::read) this register and get [`div_apb_p::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div_apb_p::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div_apb_p`]
module"]
#[doc(alias = "DIV_APB_P")]
pub type DivApbP = crate::Reg<div_apb_p::DivApbPSpec>;
#[doc = "Задает значение делителя шины APB_P. Частота шины APB_P (FAPM_P) рассчитывается, как FAPB/( Div_APM_P+1)"]
pub mod div_apb_p;
#[doc = "CLK_AHB_SET (rw) register accessor: Регистр включения тактированием устройств на шине AHB\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_ahb_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_ahb_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_ahb_set`]
module"]
#[doc(alias = "CLK_AHB_SET")]
pub type ClkAhbSet = crate::Reg<clk_ahb_set::ClkAhbSetSpec>;
#[doc = "Регистр включения тактированием устройств на шине AHB"]
pub mod clk_ahb_set;
#[doc = "CLK_AHB_CLEAR (rw) register accessor: Регистр выключения тактированием устройств на шине AHB. Каждому биту соответствует устройство, аналогично CLK_AHB_SET\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_ahb_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_ahb_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_ahb_clear`]
module"]
#[doc(alias = "CLK_AHB_CLEAR")]
pub type ClkAhbClear = crate::Reg<clk_ahb_clear::ClkAhbClearSpec>;
#[doc = "Регистр выключения тактированием устройств на шине AHB. Каждому биту соответствует устройство, аналогично CLK_AHB_SET"]
pub mod clk_ahb_clear;
#[doc = "CLK_APB_M_SET (rw) register accessor: Регистр включения тактированием устройств на шине APB_M. Каждому биту соответствует устройство\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_apb_m_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_apb_m_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_apb_m_set`]
module"]
#[doc(alias = "CLK_APB_M_SET")]
pub type ClkApbMSet = crate::Reg<clk_apb_m_set::ClkApbMSetSpec>;
#[doc = "Регистр включения тактированием устройств на шине APB_M. Каждому биту соответствует устройство"]
pub mod clk_apb_m_set;
#[doc = "CLK_APB_M_CLEAR (rw) register accessor: Регистр выключения тактированием устройств на шине APB_M. Каждому биту соответствует устройство, аналогично Clk_APB_M_Set\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_apb_m_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_apb_m_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_apb_m_clear`]
module"]
#[doc(alias = "CLK_APB_M_CLEAR")]
pub type ClkApbMClear = crate::Reg<clk_apb_m_clear::ClkApbMClearSpec>;
#[doc = "Регистр выключения тактированием устройств на шине APB_M. Каждому биту соответствует устройство, аналогично Clk_APB_M_Set"]
pub mod clk_apb_m_clear;
#[doc = "CLK_APB_P_SET (rw) register accessor: Регистр включения тактированием устройств на шине APB_P. Каждому биту соответствует одно устройство\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_apb_p_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_apb_p_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_apb_p_set`]
module"]
#[doc(alias = "CLK_APB_P_SET")]
pub type ClkApbPSet = crate::Reg<clk_apb_p_set::ClkApbPSetSpec>;
#[doc = "Регистр включения тактированием устройств на шине APB_P. Каждому биту соответствует одно устройство"]
pub mod clk_apb_p_set;
#[doc = "CLK_APB_P_CLEAR (rw) register accessor: Регистр выключения тактированием устройств на шине APB_P. Каждому биту соответствует устройство, аналогично CLK_APB_P_SET\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_apb_p_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_apb_p_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_apb_p_clear`]
module"]
#[doc(alias = "CLK_APB_P_CLEAR")]
pub type ClkApbPClear = crate::Reg<clk_apb_p_clear::ClkApbPClearSpec>;
#[doc = "Регистр выключения тактированием устройств на шине APB_P. Каждому биту соответствует устройство, аналогично CLK_APB_P_SET"]
pub mod clk_apb_p_clear;
#[doc = "AHB_MUX (rw) register accessor: Настройка источника тактирования системы\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_mux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_mux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb_mux`]
module"]
#[doc(alias = "AHB_MUX")]
pub type AhbMux = crate::Reg<ahb_mux::AhbMuxSpec>;
#[doc = "Настройка источника тактирования системы"]
pub mod ahb_mux;
#[doc = "WDT_CLK_MUX (rw) register accessor: Выбор источника тактирования сторожевого таймера: 0 – внешний OSC32M; 1 – внутренний HSI32M; 2 – внешний OSC32K; 3 – внутренний LSI32К;\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt_clk_mux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_clk_mux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt_clk_mux`]
module"]
#[doc(alias = "WDT_CLK_MUX")]
pub type WdtClkMux = crate::Reg<wdt_clk_mux::WdtClkMuxSpec>;
#[doc = "Выбор источника тактирования сторожевого таймера: 0 – внешний OSC32M; 1 – внутренний HSI32M; 2 – внешний OSC32K; 3 – внутренний LSI32К;"]
pub mod wdt_clk_mux;
#[doc = "CPU_RTC_CLK_MUX (rw) register accessor: Выбор источника тактирования RTC для системного таймера в составе ядра\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_rtc_clk_mux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_rtc_clk_mux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_rtc_clk_mux`]
module"]
#[doc(alias = "CPU_RTC_CLK_MUX")]
pub type CpuRtcClkMux = crate::Reg<cpu_rtc_clk_mux::CpuRtcClkMuxSpec>;
#[doc = "Выбор источника тактирования RTC для системного таймера в составе ядра"]
pub mod cpu_rtc_clk_mux;
#[doc = "TIMER_CFG (rw) register accessor: Выбор источника тактирования для таймеров\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_cfg`]
module"]
#[doc(alias = "TIMER_CFG")]
pub type TimerCfg = crate::Reg<timer_cfg::TimerCfgSpec>;
#[doc = "Выбор источника тактирования для таймеров"]
pub mod timer_cfg;
#[doc = "FREQ_MASK (rw) register accessor: Настройки прерываний монитора частоты\n\nYou can [`read`](crate::Reg::read) this register and get [`freq_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freq_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freq_mask`]
module"]
#[doc(alias = "FREQ_MASK")]
pub type FreqMask = crate::Reg<freq_mask::FreqMaskSpec>;
#[doc = "Настройки прерываний монитора частоты"]
pub mod freq_mask;
#[doc = "FREQ_STATUS (rw) register accessor: Статус монитора частоты\n\nYou can [`read`](crate::Reg::read) this register and get [`freq_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freq_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@freq_status`]
module"]
#[doc(alias = "FREQ_STATUS")]
pub type FreqStatus = crate::Reg<freq_status::FreqStatusSpec>;
#[doc = "Статус монитора частоты"]
pub mod freq_status;
#[doc = "SLEEP_MODE (rw) register accessor: Переход в спящий режим осуществляется записью в данный регистр. При записи отключается тактирование ядра. В зависимости от записываемого значения отключается тактирование модулей\n\nYou can [`read`](crate::Reg::read) this register and get [`sleep_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleep_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sleep_mode`]
module"]
#[doc(alias = "SLEEP_MODE")]
pub type SleepMode = crate::Reg<sleep_mode::SleepModeSpec>;
#[doc = "Переход в спящий режим осуществляется записью в данный регистр. При записи отключается тактирование ядра. В зависимости от записываемого значения отключается тактирование модулей"]
pub mod sleep_mode;
