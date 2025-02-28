#[doc = "Register `ADC_CONTINUOUS` reader"]
pub type R = crate::R<AdcContinuousSpec>;
#[doc = "Register `ADC_CONTINUOUS` writer"]
pub type W = crate::W<AdcContinuousSpec>;
#[doc = "Field `CONTINUOUS` reader - Запуск / остановка непрерывного измерения"]
pub type ContinuousR = crate::BitReader;
#[doc = "Field `CONTINUOUS` writer - Запуск / остановка непрерывного измерения"]
pub type ContinuousW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Запуск / остановка непрерывного измерения"]
    #[inline(always)]
    pub fn continuous(&self) -> ContinuousR {
        ContinuousR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Запуск / остановка непрерывного измерения"]
    #[inline(always)]
    pub fn continuous(&mut self) -> ContinuousW<AdcContinuousSpec> {
        ContinuousW::new(self, 0)
    }
}
#[doc = "Управление непрерывным измерением\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_continuous::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_continuous::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcContinuousSpec;
impl crate::RegisterSpec for AdcContinuousSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_continuous::R`](R) reader structure"]
impl crate::Readable for AdcContinuousSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_continuous::W`](W) writer structure"]
impl crate::Writable for AdcContinuousSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_CONTINUOUS to value 0"]
impl crate::Resettable for AdcContinuousSpec {
    const RESET_VALUE: u32 = 0;
}
