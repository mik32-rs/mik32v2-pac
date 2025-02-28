#[doc = "Register `SIC` reader"]
pub type R = crate::R<SicSpec>;
#[doc = "Register `SIC` writer"]
pub type W = crate::W<SicSpec>;
#[doc = "Field `Slave_Idle_coun` reader - Модуль SPI в режиме ведомого устройства начинает передачу только когда тактовый сигнал sclk_in (внешнего ведущего устройства) не изменяется в течение количества периодов опорного тактового сигнала SPI заданного в этом поле или когда модуль SPI не активен"]
pub type SlaveIdleCounR = crate::FieldReader;
#[doc = "Field `Slave_Idle_coun` writer - Модуль SPI в режиме ведомого устройства начинает передачу только когда тактовый сигнал sclk_in (внешнего ведущего устройства) не изменяется в течение количества периодов опорного тактового сигнала SPI заданного в этом поле или когда модуль SPI не активен"]
pub type SlaveIdleCounW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Модуль SPI в режиме ведомого устройства начинает передачу только когда тактовый сигнал sclk_in (внешнего ведущего устройства) не изменяется в течение количества периодов опорного тактового сигнала SPI заданного в этом поле или когда модуль SPI не активен"]
    #[inline(always)]
    pub fn slave_idle_coun(&self) -> SlaveIdleCounR {
        SlaveIdleCounR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Модуль SPI в режиме ведомого устройства начинает передачу только когда тактовый сигнал sclk_in (внешнего ведущего устройства) не изменяется в течение количества периодов опорного тактового сигнала SPI заданного в этом поле или когда модуль SPI не активен"]
    #[inline(always)]
    pub fn slave_idle_coun(&mut self) -> SlaveIdleCounW<SicSpec> {
        SlaveIdleCounW::new(self, 0)
    }
}
#[doc = "Регистр счетчика останова ведомого устройства\n\nYou can [`read`](crate::Reg::read) this register and get [`sic::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sic::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SicSpec;
impl crate::RegisterSpec for SicSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sic::R`](R) reader structure"]
impl crate::Readable for SicSpec {}
#[doc = "`write(|w| ..)` method takes [`sic::W`](W) writer structure"]
impl crate::Writable for SicSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIC to value 0xff"]
impl crate::Resettable for SicSpec {
    const RESET_VALUE: u32 = 0xff;
}
