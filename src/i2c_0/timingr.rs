#[doc = "Register `TIMINGR` reader"]
pub type R = crate::R<TimingrSpec>;
#[doc = "Register `TIMINGR` writer"]
pub type W = crate::W<TimingrSpec>;
#[doc = "Field `SCLL` reader - Длительность удержания SCL в состоянии логиче-ского «0» в режиме «ведущий» t_SCLL = (SCLL+1) x t_PRESC Также используется для генерации задержек t_BUF и t_SU:STA. Примечание: Регистр TIMING должен конфигурироваться, пока интерфейс заблокирован (PE=0)."]
pub type ScllR = crate::FieldReader;
#[doc = "Field `SCLL` writer - Длительность удержания SCL в состоянии логиче-ского «0» в режиме «ведущий» t_SCLL = (SCLL+1) x t_PRESC Также используется для генерации задержек t_BUF и t_SU:STA. Примечание: Регистр TIMING должен конфигурироваться, пока интерфейс заблокирован (PE=0)."]
pub type ScllW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCLH` reader - Длительность удержания SCL в состоянии логической «1» в режиме «ведущий» t_SCLH = (SCLH+1) x t_PRESC Также используется для генерации задержек t_HD:STA и t_SU:STO. Примечание: Регистр TIMING должен конфигурироваться, пока интерфейс заблокирован (PE=0)."]
pub type SclhR = crate::FieldReader;
#[doc = "Field `SCLH` writer - Длительность удержания SCL в состоянии логической «1» в режиме «ведущий» t_SCLH = (SCLH+1) x t_PRESC Также используется для генерации задержек t_HD:STA и t_SU:STO. Примечание: Регистр TIMING должен конфигурироваться, пока интерфейс заблокирован (PE=0)."]
pub type SclhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SDADEL` reader - Длительность предустановки данных t_HD:DAT. Задержка между спадом SCL и изменением SDA в режиме ведущего и ведомого при NOSTRETCH = 0 t_SCADEL = (SCADEL+1) x t_PRESC Используется для генера-ции задержек t_HD:DAT. Примечание: Регистр TIMING должен конфигурироваться, пока интерфейс заблокирован (PE=0)."]
pub type SdadelR = crate::FieldReader;
#[doc = "Field `SDADEL` writer - Длительность предустановки данных t_HD:DAT. Задержка между спадом SCL и изменением SDA в режиме ведущего и ведомого при NOSTRETCH = 0 t_SCADEL = (SCADEL+1) x t_PRESC Используется для генера-ции задержек t_HD:DAT. Примечание: Регистр TIMING должен конфигурироваться, пока интерфейс заблокирован (PE=0)."]
pub type SdadelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCLDEL` reader - Длительность предустанов-ки данных t_SU:DAT. Задержка между изменени-ем SDA и фронтом SCL. t_SCLDEL = (SCLDEL+1) x t_PRESC Используется для генерации задержек t_SU:DAT. Примечание: Регистр TIMING должен конфигурироваться, пока интерфейс заблокирован (PE=0)."]
pub type ScldelR = crate::FieldReader;
#[doc = "Field `SCLDEL` writer - Длительность предустанов-ки данных t_SU:DAT. Задержка между изменени-ем SDA и фронтом SCL. t_SCLDEL = (SCLDEL+1) x t_PRESC Используется для генерации задержек t_SU:DAT. Примечание: Регистр TIMING должен конфигурироваться, пока интерфейс заблокирован (PE=0)."]
pub type ScldelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRESC` reader - Предварительный делитель частоты I2CCLK. Использу-ется для вычисления значения t_PRESC используемого счетчиками предустановки, удержания, низкого и вы-сокого уровней. t_PRESC = (PRESC+1) x t_I2CCLK. Примечание: Регистр TIMING должен конфигурироваться, пока интерфейс заблокирован (PE=0)."]
pub type PrescR = crate::FieldReader;
#[doc = "Field `PRESC` writer - Предварительный делитель частоты I2CCLK. Использу-ется для вычисления значения t_PRESC используемого счетчиками предустановки, удержания, низкого и вы-сокого уровней. t_PRESC = (PRESC+1) x t_I2CCLK. Примечание: Регистр TIMING должен конфигурироваться, пока интерфейс заблокирован (PE=0)."]
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - Длительность удержания SCL в состоянии логиче-ского «0» в режиме «ведущий» t_SCLL = (SCLL+1) x t_PRESC Также используется для генерации задержек t_BUF и t_SU:STA. Примечание: Регистр TIMING должен конфигурироваться, пока интерфейс заблокирован (PE=0)."]
    #[inline(always)]
    pub fn scll(&self) -> ScllR {
        ScllR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Длительность удержания SCL в состоянии логической «1» в режиме «ведущий» t_SCLH = (SCLH+1) x t_PRESC Также используется для генерации задержек t_HD:STA и t_SU:STO. Примечание: Регистр TIMING должен конфигурироваться, пока интерфейс заблокирован (PE=0)."]
    #[inline(always)]
    pub fn sclh(&self) -> SclhR {
        SclhR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Длительность предустановки данных t_HD:DAT. Задержка между спадом SCL и изменением SDA в режиме ведущего и ведомого при NOSTRETCH = 0 t_SCADEL = (SCADEL+1) x t_PRESC Используется для генера-ции задержек t_HD:DAT. Примечание: Регистр TIMING должен конфигурироваться, пока интерфейс заблокирован (PE=0)."]
    #[inline(always)]
    pub fn sdadel(&self) -> SdadelR {
        SdadelR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Длительность предустанов-ки данных t_SU:DAT. Задержка между изменени-ем SDA и фронтом SCL. t_SCLDEL = (SCLDEL+1) x t_PRESC Используется для генерации задержек t_SU:DAT. Примечание: Регистр TIMING должен конфигурироваться, пока интерфейс заблокирован (PE=0)."]
    #[inline(always)]
    pub fn scldel(&self) -> ScldelR {
        ScldelR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Предварительный делитель частоты I2CCLK. Использу-ется для вычисления значения t_PRESC используемого счетчиками предустановки, удержания, низкого и вы-сокого уровней. t_PRESC = (PRESC+1) x t_I2CCLK. Примечание: Регистр TIMING должен конфигурироваться, пока интерфейс заблокирован (PE=0)."]
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Длительность удержания SCL в состоянии логиче-ского «0» в режиме «ведущий» t_SCLL = (SCLL+1) x t_PRESC Также используется для генерации задержек t_BUF и t_SU:STA. Примечание: Регистр TIMING должен конфигурироваться, пока интерфейс заблокирован (PE=0)."]
    #[inline(always)]
    pub fn scll(&mut self) -> ScllW<TimingrSpec> {
        ScllW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Длительность удержания SCL в состоянии логической «1» в режиме «ведущий» t_SCLH = (SCLH+1) x t_PRESC Также используется для генерации задержек t_HD:STA и t_SU:STO. Примечание: Регистр TIMING должен конфигурироваться, пока интерфейс заблокирован (PE=0)."]
    #[inline(always)]
    pub fn sclh(&mut self) -> SclhW<TimingrSpec> {
        SclhW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Длительность предустановки данных t_HD:DAT. Задержка между спадом SCL и изменением SDA в режиме ведущего и ведомого при NOSTRETCH = 0 t_SCADEL = (SCADEL+1) x t_PRESC Используется для генера-ции задержек t_HD:DAT. Примечание: Регистр TIMING должен конфигурироваться, пока интерфейс заблокирован (PE=0)."]
    #[inline(always)]
    pub fn sdadel(&mut self) -> SdadelW<TimingrSpec> {
        SdadelW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Длительность предустанов-ки данных t_SU:DAT. Задержка между изменени-ем SDA и фронтом SCL. t_SCLDEL = (SCLDEL+1) x t_PRESC Используется для генерации задержек t_SU:DAT. Примечание: Регистр TIMING должен конфигурироваться, пока интерфейс заблокирован (PE=0)."]
    #[inline(always)]
    pub fn scldel(&mut self) -> ScldelW<TimingrSpec> {
        ScldelW::new(self, 20)
    }
    #[doc = "Bits 28:31 - Предварительный делитель частоты I2CCLK. Использу-ется для вычисления значения t_PRESC используемого счетчиками предустановки, удержания, низкого и вы-сокого уровней. t_PRESC = (PRESC+1) x t_I2CCLK. Примечание: Регистр TIMING должен конфигурироваться, пока интерфейс заблокирован (PE=0)."]
    #[inline(always)]
    pub fn presc(&mut self) -> PrescW<TimingrSpec> {
        PrescW::new(self, 28)
    }
}
#[doc = "Регистр настройки временных ограничений. Регистр должен конфигурироваться, пока интерфейс заблокирован (PE=0).\n\nYou can [`read`](crate::Reg::read) this register and get [`timingr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timingr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimingrSpec;
impl crate::RegisterSpec for TimingrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timingr::R`](R) reader structure"]
impl crate::Readable for TimingrSpec {}
#[doc = "`write(|w| ..)` method takes [`timingr::W`](W) writer structure"]
impl crate::Writable for TimingrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMINGR to value 0"]
impl crate::Resettable for TimingrSpec {
    const RESET_VALUE: u32 = 0;
}
