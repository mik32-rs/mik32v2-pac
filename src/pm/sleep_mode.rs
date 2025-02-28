#[doc = "Register `SLEEP_MODE` reader"]
pub type R = crate::R<SleepModeSpec>;
#[doc = "Register `SLEEP_MODE` writer"]
pub type W = crate::W<SleepModeSpec>;
#[doc = "Field `EEPROM` reader - Отключение тактирования EEPROM"]
pub type EepromR = crate::BitReader;
#[doc = "Field `RAM` reader - Отключение тактирования ОЗУ"]
pub type RamR = crate::BitReader;
#[doc = "Field `SPIFI` reader - Отключение тактирования контроллера SPIFI"]
pub type SpifiR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Отключение тактирования EEPROM"]
    #[inline(always)]
    pub fn eeprom(&self) -> EepromR {
        EepromR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Отключение тактирования ОЗУ"]
    #[inline(always)]
    pub fn ram(&self) -> RamR {
        RamR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Отключение тактирования контроллера SPIFI"]
    #[inline(always)]
    pub fn spifi(&self) -> SpifiR {
        SpifiR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {}
#[doc = "Переход в спящий режим осуществляется записью в данный регистр. При записи отключается тактирование ядра. В зависимости от записываемого значения отключается тактирование модулей\n\nYou can [`read`](crate::Reg::read) this register and get [`sleep_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleep_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SleepModeSpec;
impl crate::RegisterSpec for SleepModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sleep_mode::R`](R) reader structure"]
impl crate::Readable for SleepModeSpec {}
#[doc = "`write(|w| ..)` method takes [`sleep_mode::W`](W) writer structure"]
impl crate::Writable for SleepModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLEEP_MODE to value 0"]
impl crate::Resettable for SleepModeSpec {
    const RESET_VALUE: u32 = 0;
}
