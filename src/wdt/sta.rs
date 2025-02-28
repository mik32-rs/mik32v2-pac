#[doc = "Register `STA` reader"]
pub type R = crate::R<StaSpec>;
#[doc = "Бит активности таймера\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timerenabled {
    #[doc = "0: Таймер остановлен"]
    Disable = 0,
    #[doc = "1: Таймер запущен"]
    Enable = 1,
}
impl From<Timerenabled> for bool {
    #[inline(always)]
    fn from(variant: Timerenabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMERENABLED` reader - Бит активности таймера"]
pub type TimerenabledR = crate::BitReader<Timerenabled>;
impl TimerenabledR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timerenabled {
        match self.bits {
            false => Timerenabled::Disable,
            true => Timerenabled::Enable,
        }
    }
    #[doc = "Таймер остановлен"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Timerenabled::Disable
    }
    #[doc = "Таймер запущен"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Timerenabled::Enable
    }
}
#[doc = "Бит перезагрузки значения\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timerloading {
    #[doc = "0: Перезагрузка значения таймера в данный момент не выполняется (завершена)"]
    ReloadingComplete = 0,
    #[doc = "1: Выполняется перезагрузка значения в таймере"]
    Reloading = 1,
}
impl From<Timerloading> for bool {
    #[inline(always)]
    fn from(variant: Timerloading) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMERLOADING` reader - Бит перезагрузки значения"]
pub type TimerloadingR = crate::BitReader<Timerloading>;
impl TimerloadingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timerloading {
        match self.bits {
            false => Timerloading::ReloadingComplete,
            true => Timerloading::Reloading,
        }
    }
    #[doc = "Перезагрузка значения таймера в данный момент не выполняется (завершена)"]
    #[inline(always)]
    pub fn is_reloading_complete(&self) -> bool {
        *self == Timerloading::ReloadingComplete
    }
    #[doc = "Выполняется перезагрузка значения в таймере"]
    #[inline(always)]
    pub fn is_reloading(&self) -> bool {
        *self == Timerloading::Reloading
    }
}
#[doc = "Field `WDT_RST_FLAG` reader - Флаг генерации сброса сторожевым таймером. Сбрасывается в 0 только при снятии и последующей подаче питания"]
pub type WdtRstFlagR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Бит активности таймера"]
    #[inline(always)]
    pub fn timerenabled(&self) -> TimerenabledR {
        TimerenabledR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Бит перезагрузки значения"]
    #[inline(always)]
    pub fn timerloading(&self) -> TimerloadingR {
        TimerloadingR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Флаг генерации сброса сторожевым таймером. Сбрасывается в 0 только при снятии и последующей подаче питания"]
    #[inline(always)]
    pub fn wdt_rst_flag(&self) -> WdtRstFlagR {
        WdtRstFlagR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Регистр состояния\n\nYou can [`read`](crate::Reg::read) this register and get [`sta::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StaSpec;
impl crate::RegisterSpec for StaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sta::R`](R) reader structure"]
impl crate::Readable for StaSpec {}
#[doc = "`reset()` method sets STA to value 0"]
impl crate::Resettable for StaSpec {
    const RESET_VALUE: u32 = 0;
}
