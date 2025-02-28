#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Field `CLEAR_LOCAL_IRQ` writer - Очистка локального прерывания"]
pub type ClearLocalIrqW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLEAR_GLOBAL_IRQ` writer - Очистка глобального прерывания: «1» – снятие запроса на прерывание"]
pub type ClearGlobalIrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_ERROR_IRQ` writer - Очистка прерывания ошибки: «1» – снятие запроса на прерывание"]
pub type ClearErrorIrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Разрешение формирования глобального прерывания\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GlobalIrqEna {
    #[doc = "0: Прерывание запрещено"]
    Disable = 0,
    #[doc = "1: Прерывание разрешено"]
    Enable = 1,
}
impl From<GlobalIrqEna> for bool {
    #[inline(always)]
    fn from(variant: GlobalIrqEna) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GLOBAL_IRQ_ENA` writer - Разрешение формирования глобального прерывания"]
pub type GlobalIrqEnaW<'a, REG> = crate::BitWriter<'a, REG, GlobalIrqEna>;
impl<'a, REG> GlobalIrqEnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание запрещено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(GlobalIrqEna::Disable)
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(GlobalIrqEna::Enable)
    }
}
#[doc = "Разрешение формирования прерывания при ошибке\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorIrqEna {
    #[doc = "0: Прерывание запрещено"]
    Disable = 0,
    #[doc = "1: Прерывание разрешено"]
    Enable = 1,
}
impl From<ErrorIrqEna> for bool {
    #[inline(always)]
    fn from(variant: ErrorIrqEna) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR_IRQ_ENA` writer - Разрешение формирования прерывания при ошибке"]
pub type ErrorIrqEnaW<'a, REG> = crate::BitWriter<'a, REG, ErrorIrqEna>;
impl<'a, REG> ErrorIrqEnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание запрещено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ErrorIrqEna::Disable)
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ErrorIrqEna::Enable)
    }
}
#[doc = "Разрешение чтения текущего статуса канала\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurrentValue {
    #[doc = "0: Разрешено (текущие значения)"]
    Disable = 0,
    #[doc = "1: Запрещено (значения при настройке)"]
    Enable = 1,
}
impl From<CurrentValue> for bool {
    #[inline(always)]
    fn from(variant: CurrentValue) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CURRENT_VALUE` writer - Разрешение чтения текущего статуса канала"]
pub type CurrentValueW<'a, REG> = crate::BitWriter<'a, REG, CurrentValue>;
impl<'a, REG> CurrentValueW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Разрешено (текущие значения)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CurrentValue::Disable)
    }
    #[doc = "Запрещено (значения при настройке)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CurrentValue::Enable)
    }
}
impl W {
    #[doc = "Bits 0:3 - Очистка локального прерывания"]
    #[inline(always)]
    pub fn clear_local_irq(&mut self) -> ClearLocalIrqW<ConfigSpec> {
        ClearLocalIrqW::new(self, 0)
    }
    #[doc = "Bit 4 - Очистка глобального прерывания: «1» – снятие запроса на прерывание"]
    #[inline(always)]
    pub fn clear_global_irq(&mut self) -> ClearGlobalIrqW<ConfigSpec> {
        ClearGlobalIrqW::new(self, 4)
    }
    #[doc = "Bit 5 - Очистка прерывания ошибки: «1» – снятие запроса на прерывание"]
    #[inline(always)]
    pub fn clear_error_irq(&mut self) -> ClearErrorIrqW<ConfigSpec> {
        ClearErrorIrqW::new(self, 5)
    }
    #[doc = "Bit 6 - Разрешение формирования глобального прерывания"]
    #[inline(always)]
    pub fn global_irq_ena(&mut self) -> GlobalIrqEnaW<ConfigSpec> {
        GlobalIrqEnaW::new(self, 6)
    }
    #[doc = "Bit 7 - Разрешение формирования прерывания при ошибке"]
    #[inline(always)]
    pub fn error_irq_ena(&mut self) -> ErrorIrqEnaW<ConfigSpec> {
        ErrorIrqEnaW::new(self, 7)
    }
    #[doc = "Bit 8 - Разрешение чтения текущего статуса канала"]
    #[inline(always)]
    pub fn current_value(&mut self) -> CurrentValueW<ConfigSpec> {
        CurrentValueW::new(self, 8)
    }
}
#[doc = "Регистр прерываний и настройки контроллера\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0;
}
