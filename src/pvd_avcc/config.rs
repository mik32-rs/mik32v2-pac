#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Управление питанием монитора\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pd {
    #[doc = "0: Монитор включен"]
    Enable = 0,
    #[doc = "1: Монитор выключен"]
    Disable = 1,
}
impl From<Pd> for bool {
    #[inline(always)]
    fn from(variant: Pd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD` reader - Управление питанием монитора"]
pub type PdR = crate::BitReader<Pd>;
impl PdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pd {
        match self.bits {
            false => Pd::Enable,
            true => Pd::Disable,
        }
    }
    #[doc = "Монитор включен"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pd::Enable
    }
    #[doc = "Монитор выключен"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pd::Disable
    }
}
#[doc = "Field `PD` writer - Управление питанием монитора"]
pub type PdW<'a, REG> = crate::BitWriter<'a, REG, Pd>;
impl<'a, REG> PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Монитор включен"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pd::Enable)
    }
    #[doc = "Монитор выключен"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pd::Disable)
    }
}
#[doc = "Отключение (сброс) детектирования нижнего порога\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nresetu {
    #[doc = "0: Детектирование не выполняется, схема в состоянии сброса"]
    Reset = 0,
    #[doc = "1: Нормальная работа"]
    Normal = 1,
}
impl From<Nresetu> for bool {
    #[inline(always)]
    fn from(variant: Nresetu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NRESETU` reader - Отключение (сброс) детектирования нижнего порога"]
pub type NresetuR = crate::BitReader<Nresetu>;
impl NresetuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nresetu {
        match self.bits {
            false => Nresetu::Reset,
            true => Nresetu::Normal,
        }
    }
    #[doc = "Детектирование не выполняется, схема в состоянии сброса"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Nresetu::Reset
    }
    #[doc = "Нормальная работа"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Nresetu::Normal
    }
}
#[doc = "Field `NRESETU` writer - Отключение (сброс) детектирования нижнего порога"]
pub type NresetuW<'a, REG> = crate::BitWriter<'a, REG, Nresetu>;
impl<'a, REG> NresetuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Детектирование не выполняется, схема в состоянии сброса"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Nresetu::Reset)
    }
    #[doc = "Нормальная работа"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Nresetu::Normal)
    }
}
#[doc = "Отключение (сброс) детектирования вехнего порога\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nreseto {
    #[doc = "0: Детектирование не выполняется, схема в состоянии сброса"]
    Reset = 0,
    #[doc = "1: Нормальная работа"]
    Normal = 1,
}
impl From<Nreseto> for bool {
    #[inline(always)]
    fn from(variant: Nreseto) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NRESETO` reader - Отключение (сброс) детектирования вехнего порога"]
pub type NresetoR = crate::BitReader<Nreseto>;
impl NresetoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nreseto {
        match self.bits {
            false => Nreseto::Reset,
            true => Nreseto::Normal,
        }
    }
    #[doc = "Детектирование не выполняется, схема в состоянии сброса"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Nreseto::Reset
    }
    #[doc = "Нормальная работа"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Nreseto::Normal
    }
}
#[doc = "Field `NRESETO` writer - Отключение (сброс) детектирования вехнего порога"]
pub type NresetoW<'a, REG> = crate::BitWriter<'a, REG, Nreseto>;
impl<'a, REG> NresetoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Детектирование не выполняется, схема в состоянии сброса"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Nreseto::Reset)
    }
    #[doc = "Нормальная работа"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Nreseto::Normal)
    }
}
#[doc = "Field `TESTMODE` reader - Переход в тестовый режим"]
pub type TestmodeR = crate::BitReader;
#[doc = "Field `TESTMODE` writer - Переход в тестовый режим"]
pub type TestmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDER_THRESH` reader - Нижний порог срабатывания монитора"]
pub type UnderThreshR = crate::FieldReader;
#[doc = "Field `UNDER_THRESH` writer - Нижний порог срабатывания монитора"]
pub type UnderThreshW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OVER_THRESH` reader - Верхний порог срабатывания монитора"]
pub type OverThreshR = crate::FieldReader;
#[doc = "Field `OVER_THRESH` writer - Верхний порог срабатывания монитора"]
pub type OverThreshW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EN_VREFCLB` reader - Использование в качестве опорного источника напряжения каллибруемый ОИН"]
pub type EnVrefclbR = crate::BitReader;
#[doc = "Field `EN_VREFCLB` writer - Использование в качестве опорного источника напряжения каллибруемый ОИН"]
pub type EnVrefclbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Управление питанием монитора"]
    #[inline(always)]
    pub fn pd(&self) -> PdR {
        PdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Отключение (сброс) детектирования нижнего порога"]
    #[inline(always)]
    pub fn nresetu(&self) -> NresetuR {
        NresetuR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Отключение (сброс) детектирования вехнего порога"]
    #[inline(always)]
    pub fn nreseto(&self) -> NresetoR {
        NresetoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Переход в тестовый режим"]
    #[inline(always)]
    pub fn testmode(&self) -> TestmodeR {
        TestmodeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Нижний порог срабатывания монитора"]
    #[inline(always)]
    pub fn under_thresh(&self) -> UnderThreshR {
        UnderThreshR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Верхний порог срабатывания монитора"]
    #[inline(always)]
    pub fn over_thresh(&self) -> OverThreshR {
        OverThreshR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Использование в качестве опорного источника напряжения каллибруемый ОИН"]
    #[inline(always)]
    pub fn en_vrefclb(&self) -> EnVrefclbR {
        EnVrefclbR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Управление питанием монитора"]
    #[inline(always)]
    pub fn pd(&mut self) -> PdW<ConfigSpec> {
        PdW::new(self, 0)
    }
    #[doc = "Bit 1 - Отключение (сброс) детектирования нижнего порога"]
    #[inline(always)]
    pub fn nresetu(&mut self) -> NresetuW<ConfigSpec> {
        NresetuW::new(self, 1)
    }
    #[doc = "Bit 2 - Отключение (сброс) детектирования вехнего порога"]
    #[inline(always)]
    pub fn nreseto(&mut self) -> NresetoW<ConfigSpec> {
        NresetoW::new(self, 2)
    }
    #[doc = "Bit 3 - Переход в тестовый режим"]
    #[inline(always)]
    pub fn testmode(&mut self) -> TestmodeW<ConfigSpec> {
        TestmodeW::new(self, 3)
    }
    #[doc = "Bits 4:7 - Нижний порог срабатывания монитора"]
    #[inline(always)]
    pub fn under_thresh(&mut self) -> UnderThreshW<ConfigSpec> {
        UnderThreshW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Верхний порог срабатывания монитора"]
    #[inline(always)]
    pub fn over_thresh(&mut self) -> OverThreshW<ConfigSpec> {
        OverThreshW::new(self, 8)
    }
    #[doc = "Bit 12 - Использование в качестве опорного источника напряжения каллибруемый ОИН"]
    #[inline(always)]
    pub fn en_vrefclb(&mut self) -> EnVrefclbW<ConfigSpec> {
        EnVrefclbW::new(self, 12)
    }
}
#[doc = "Регистр настроек\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0x01"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0x01;
}
