#[doc = "Register `DAC0_CFG` reader"]
pub type R = crate::R<Dac0CfgSpec>;
#[doc = "Register `DAC0_CFG` writer"]
pub type W = crate::W<Dac0CfgSpec>;
#[doc = "Управление питанием ЦАП\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Выключен"]
    Disable = 0,
    #[doc = "1: Включен"]
    Enable = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Управление питанием ЦАП"]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::Disable,
            true => En::Enable,
        }
    }
    #[doc = "Выключен"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == En::Disable
    }
    #[doc = "Включен"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == En::Enable
    }
}
#[doc = "Field `EN` writer - Управление питанием ЦАП"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Выключен"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(En::Disable)
    }
    #[doc = "Включен"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(En::Enable)
    }
}
#[doc = "Field `RN` reader - Управление сбросом ЦАП, активный уровень «0»"]
pub type RnR = crate::BitReader;
#[doc = "Field `RN` writer - Управление сбросом ЦАП, активный уровень «0»"]
pub type RnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIV` reader - Значение делителя тактового сигнала. Частота определяется как FЦАП=FIN/(DIV+1)"]
pub type DivR = crate::FieldReader;
#[doc = "Field `DIV` writer - Значение делителя тактового сигнала. Частота определяется как FЦАП=FIN/(DIV+1)"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Выбор источника опорного напряжения\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exten {
    #[doc = "0: Встроенный"]
    Internal = 0,
    #[doc = "1: Внешний"]
    External = 1,
}
impl From<Exten> for bool {
    #[inline(always)]
    fn from(variant: Exten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTEN` reader - Выбор источника опорного напряжения"]
pub type ExtenR = crate::BitReader<Exten>;
impl ExtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exten {
        match self.bits {
            false => Exten::Internal,
            true => Exten::External,
        }
    }
    #[doc = "Встроенный"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == Exten::Internal
    }
    #[doc = "Внешний"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == Exten::External
    }
}
#[doc = "Field `EXTEN` writer - Выбор источника опорного напряжения"]
pub type ExtenW<'a, REG> = crate::BitWriter<'a, REG, Exten>;
impl<'a, REG> ExtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Встроенный"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(Exten::Internal)
    }
    #[doc = "Внешний"]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(Exten::External)
    }
}
#[doc = "Выбор источника внешнего опорного напряжения\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extpad {
    #[doc = "0: Настраиваемый ИОН"]
    CalibratedVoltageReference = 0,
    #[doc = "1: Внешний вывод REF_ADC_DAC (PORT1_11)"]
    RefDacPin = 1,
}
impl From<Extpad> for bool {
    #[inline(always)]
    fn from(variant: Extpad) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTPAD` reader - Выбор источника внешнего опорного напряжения"]
pub type ExtpadR = crate::BitReader<Extpad>;
impl ExtpadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extpad {
        match self.bits {
            false => Extpad::CalibratedVoltageReference,
            true => Extpad::RefDacPin,
        }
    }
    #[doc = "Настраиваемый ИОН"]
    #[inline(always)]
    pub fn is_calibrated_voltage_reference(&self) -> bool {
        *self == Extpad::CalibratedVoltageReference
    }
    #[doc = "Внешний вывод REF_ADC_DAC (PORT1_11)"]
    #[inline(always)]
    pub fn is_ref_dac_pin(&self) -> bool {
        *self == Extpad::RefDacPin
    }
}
#[doc = "Field `EXTPAD` writer - Выбор источника внешнего опорного напряжения"]
pub type ExtpadW<'a, REG> = crate::BitWriter<'a, REG, Extpad>;
impl<'a, REG> ExtpadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Настраиваемый ИОН"]
    #[inline(always)]
    pub fn calibrated_voltage_reference(self) -> &'a mut crate::W<REG> {
        self.variant(Extpad::CalibratedVoltageReference)
    }
    #[doc = "Внешний вывод REF_ADC_DAC (PORT1_11)"]
    #[inline(always)]
    pub fn ref_dac_pin(self) -> &'a mut crate::W<REG> {
        self.variant(Extpad::RefDacPin)
    }
}
#[doc = "Признак заполненности регистра DAC_VALUE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmptyRead {
    #[doc = "0: Значение, хранящееся в DAC_VALUE было сдвинуто в ЦАП, возможна запись следующего значения"]
    Full = 0,
    #[doc = "1: В регистре DAC_Value находится необработанное значение. Автоматически сбрасывается при записи в DAC_Value"]
    Empty = 1,
}
impl From<EmptyRead> for bool {
    #[inline(always)]
    fn from(variant: EmptyRead) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMPTY_READ` reader - Признак заполненности регистра DAC_VALUE"]
pub type EmptyReadR = crate::BitReader<EmptyRead>;
impl EmptyReadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EmptyRead {
        match self.bits {
            false => EmptyRead::Full,
            true => EmptyRead::Empty,
        }
    }
    #[doc = "Значение, хранящееся в DAC_VALUE было сдвинуто в ЦАП, возможна запись следующего значения"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == EmptyRead::Full
    }
    #[doc = "В регистре DAC_Value находится необработанное значение. Автоматически сбрасывается при записи в DAC_Value"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == EmptyRead::Empty
    }
}
impl R {
    #[doc = "Bit 0 - Управление питанием ЦАП"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Управление сбросом ЦАП, активный уровень «0»"]
    #[inline(always)]
    pub fn rn(&self) -> RnR {
        RnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:9 - Значение делителя тактового сигнала. Частота определяется как FЦАП=FIN/(DIV+1)"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bit 10 - Выбор источника опорного напряжения"]
    #[inline(always)]
    pub fn exten(&self) -> ExtenR {
        ExtenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Выбор источника внешнего опорного напряжения"]
    #[inline(always)]
    pub fn extpad(&self) -> ExtpadR {
        ExtpadR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Признак заполненности регистра DAC_VALUE"]
    #[inline(always)]
    pub fn empty_read(&self) -> EmptyReadR {
        EmptyReadR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Управление питанием ЦАП"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<Dac0CfgSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Управление сбросом ЦАП, активный уровень «0»"]
    #[inline(always)]
    pub fn rn(&mut self) -> RnW<Dac0CfgSpec> {
        RnW::new(self, 1)
    }
    #[doc = "Bits 2:9 - Значение делителя тактового сигнала. Частота определяется как FЦАП=FIN/(DIV+1)"]
    #[inline(always)]
    pub fn div(&mut self) -> DivW<Dac0CfgSpec> {
        DivW::new(self, 2)
    }
    #[doc = "Bit 10 - Выбор источника опорного напряжения"]
    #[inline(always)]
    pub fn exten(&mut self) -> ExtenW<Dac0CfgSpec> {
        ExtenW::new(self, 10)
    }
    #[doc = "Bit 11 - Выбор источника внешнего опорного напряжения"]
    #[inline(always)]
    pub fn extpad(&mut self) -> ExtpadW<Dac0CfgSpec> {
        ExtpadW::new(self, 11)
    }
}
#[doc = "Регистр настойки ЦАП0\n\nYou can [`read`](crate::Reg::read) this register and get [`dac0_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac0_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dac0CfgSpec;
impl crate::RegisterSpec for Dac0CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac0_cfg::R`](R) reader structure"]
impl crate::Readable for Dac0CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dac0_cfg::W`](W) writer structure"]
impl crate::Writable for Dac0CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC0_CFG to value 0"]
impl crate::Resettable for Dac0CfgSpec {
    const RESET_VALUE: u32 = 0;
}
