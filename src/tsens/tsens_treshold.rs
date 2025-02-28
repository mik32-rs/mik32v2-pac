#[doc = "Register `TSENS_TRESHOLD` reader"]
pub type R = crate::R<TsensTresholdSpec>;
#[doc = "Register `TSENS_TRESHOLD` writer"]
pub type W = crate::W<TsensTresholdSpec>;
#[doc = "Field `TRESHOLD_HI` reader - При значении выхода сенсора больше HI_THRESHOLD будет формироваться прерывание. Значение по умолчанию соот-ветсвует температуре 125 оС"]
pub type TresholdHiR = crate::FieldReader<u16>;
#[doc = "Field `TRESHOLD_HI` writer - При значении выхода сенсора больше HI_THRESHOLD будет формироваться прерывание. Значение по умолчанию соот-ветсвует температуре 125 оС"]
pub type TresholdHiW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TRESHOLD_LOW` reader - При значении выхода сенсора менее LOW_THRESHOLD будет формироваться прерывание. Значение по умолчанию соответсвует температуре -40 оС"]
pub type TresholdLowR = crate::FieldReader<u16>;
#[doc = "Field `TRESHOLD_LOW` writer - При значении выхода сенсора менее LOW_THRESHOLD будет формироваться прерывание. Значение по умолчанию соответсвует температуре -40 оС"]
pub type TresholdLowW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - При значении выхода сенсора больше HI_THRESHOLD будет формироваться прерывание. Значение по умолчанию соот-ветсвует температуре 125 оС"]
    #[inline(always)]
    pub fn treshold_hi(&self) -> TresholdHiR {
        TresholdHiR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - При значении выхода сенсора менее LOW_THRESHOLD будет формироваться прерывание. Значение по умолчанию соответсвует температуре -40 оС"]
    #[inline(always)]
    pub fn treshold_low(&self) -> TresholdLowR {
        TresholdLowR::new(((self.bits >> 10) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - При значении выхода сенсора больше HI_THRESHOLD будет формироваться прерывание. Значение по умолчанию соот-ветсвует температуре 125 оС"]
    #[inline(always)]
    pub fn treshold_hi(&mut self) -> TresholdHiW<TsensTresholdSpec> {
        TresholdHiW::new(self, 0)
    }
    #[doc = "Bits 10:19 - При значении выхода сенсора менее LOW_THRESHOLD будет формироваться прерывание. Значение по умолчанию соответсвует температуре -40 оС"]
    #[inline(always)]
    pub fn treshold_low(&mut self) -> TresholdLowW<TsensTresholdSpec> {
        TresholdLowW::new(self, 10)
    }
}
#[doc = "Регистр пороговых значений\n\nYou can [`read`](crate::Reg::read) this register and get [`tsens_treshold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsens_treshold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsensTresholdSpec;
impl crate::RegisterSpec for TsensTresholdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsens_treshold::R`](R) reader structure"]
impl crate::Readable for TsensTresholdSpec {}
#[doc = "`write(|w| ..)` method takes [`tsens_treshold::W`](W) writer structure"]
impl crate::Writable for TsensTresholdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSENS_TRESHOLD to value 0x0003_865b"]
impl crate::Resettable for TsensTresholdSpec {
    const RESET_VALUE: u32 = 0x0003_865b;
}
