#[doc = "Register `ADC_CONFIG` reader"]
pub type R = crate::R<AdcConfigSpec>;
#[doc = "Register `ADC_CONFIG` writer"]
pub type W = crate::W<AdcConfigSpec>;
#[doc = "Управление питанием АЦП\n\nValue on reset: 0"]
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
#[doc = "Field `EN` reader - Управление питанием АЦП"]
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
#[doc = "Field `EN` writer - Управление питанием АЦП"]
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
#[doc = "Field `RESETN` reader - Управление сбросом АЦП, активный уровень «0»"]
pub type ResetnR = crate::BitReader;
#[doc = "Field `RESETN` writer - Управление сбросом АЦП, активный уровень «0»"]
pub type ResetnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Выбор источника опорного напряжения\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extref {
    #[doc = "0: Встроенный"]
    Internal = 0,
    #[doc = "1: Внешний"]
    External = 1,
}
impl From<Extref> for bool {
    #[inline(always)]
    fn from(variant: Extref) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTREF` reader - Выбор источника опорного напряжения"]
pub type ExtrefR = crate::BitReader<Extref>;
impl ExtrefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extref {
        match self.bits {
            false => Extref::Internal,
            true => Extref::External,
        }
    }
    #[doc = "Встроенный"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == Extref::Internal
    }
    #[doc = "Внешний"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == Extref::External
    }
}
#[doc = "Field `EXTREF` writer - Выбор источника опорного напряжения"]
pub type ExtrefW<'a, REG> = crate::BitWriter<'a, REG, Extref>;
impl<'a, REG> ExtrefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Встроенный"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(Extref::Internal)
    }
    #[doc = "Внешний"]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(Extref::External)
    }
}
#[doc = "Выбор источника внешнего опорного напряжения\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExtpadEn {
    #[doc = "0: Настраиваемый ИОН"]
    CalibratedVoltageReference = 0,
    #[doc = "1: Внешний вывод REF_ADC_DAC (PORT1_11)"]
    RefDacPin = 1,
}
impl From<ExtpadEn> for bool {
    #[inline(always)]
    fn from(variant: ExtpadEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTPAD_EN` reader - Выбор источника внешнего опорного напряжения"]
pub type ExtpadEnR = crate::BitReader<ExtpadEn>;
impl ExtpadEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ExtpadEn {
        match self.bits {
            false => ExtpadEn::CalibratedVoltageReference,
            true => ExtpadEn::RefDacPin,
        }
    }
    #[doc = "Настраиваемый ИОН"]
    #[inline(always)]
    pub fn is_calibrated_voltage_reference(&self) -> bool {
        *self == ExtpadEn::CalibratedVoltageReference
    }
    #[doc = "Внешний вывод REF_ADC_DAC (PORT1_11)"]
    #[inline(always)]
    pub fn is_ref_dac_pin(&self) -> bool {
        *self == ExtpadEn::RefDacPin
    }
}
#[doc = "Field `EXTPAD_EN` writer - Выбор источника внешнего опорного напряжения"]
pub type ExtpadEnW<'a, REG> = crate::BitWriter<'a, REG, ExtpadEn>;
impl<'a, REG> ExtpadEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Настраиваемый ИОН"]
    #[inline(always)]
    pub fn calibrated_voltage_reference(self) -> &'a mut crate::W<REG> {
        self.variant(ExtpadEn::CalibratedVoltageReference)
    }
    #[doc = "Внешний вывод REF_ADC_DAC (PORT1_11)"]
    #[inline(always)]
    pub fn ref_dac_pin(self) -> &'a mut crate::W<REG> {
        self.variant(ExtpadEn::RefDacPin)
    }
}
#[doc = "Field `SEL` reader - Выбор канала АЦП"]
pub type SelR = crate::FieldReader;
#[doc = "Field `SEL` writer - Выбор канала АЦП"]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SAH_TIME` reader - Время выборки очередного отсчета в тактах АЦП"]
pub type SahTimeR = crate::FieldReader;
#[doc = "Field `SAH_TIME` writer - Время выборки очередного отсчета в тактах АЦП"]
pub type SahTimeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Управление питанием АЦП"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Управление сбросом АЦП, активный уровень «0»"]
    #[inline(always)]
    pub fn resetn(&self) -> ResetnR {
        ResetnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Выбор источника опорного напряжения"]
    #[inline(always)]
    pub fn extref(&self) -> ExtrefR {
        ExtrefR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Выбор источника внешнего опорного напряжения"]
    #[inline(always)]
    pub fn extpad_en(&self) -> ExtpadEnR {
        ExtpadEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Выбор канала АЦП"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:13 - Время выборки очередного отсчета в тактах АЦП"]
    #[inline(always)]
    pub fn sah_time(&self) -> SahTimeR {
        SahTimeR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Управление питанием АЦП"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<AdcConfigSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Управление сбросом АЦП, активный уровень «0»"]
    #[inline(always)]
    pub fn resetn(&mut self) -> ResetnW<AdcConfigSpec> {
        ResetnW::new(self, 1)
    }
    #[doc = "Bit 2 - Выбор источника опорного напряжения"]
    #[inline(always)]
    pub fn extref(&mut self) -> ExtrefW<AdcConfigSpec> {
        ExtrefW::new(self, 2)
    }
    #[doc = "Bit 3 - Выбор источника внешнего опорного напряжения"]
    #[inline(always)]
    pub fn extpad_en(&mut self) -> ExtpadEnW<AdcConfigSpec> {
        ExtpadEnW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Выбор канала АЦП"]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<AdcConfigSpec> {
        SelW::new(self, 4)
    }
    #[doc = "Bits 8:13 - Время выборки очередного отсчета в тактах АЦП"]
    #[inline(always)]
    pub fn sah_time(&mut self) -> SahTimeW<AdcConfigSpec> {
        SahTimeW::new(self, 8)
    }
}
#[doc = "Регистр настойки\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcConfigSpec;
impl crate::RegisterSpec for AdcConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_config::R`](R) reader structure"]
impl crate::Readable for AdcConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_config::W`](W) writer structure"]
impl crate::Writable for AdcConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_CONFIG to value 0"]
impl crate::Resettable for AdcConfigSpec {
    const RESET_VALUE: u32 = 0;
}
