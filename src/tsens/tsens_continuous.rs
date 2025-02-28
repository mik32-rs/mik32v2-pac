#[doc = "Register `TSENS_CONTINUOUS` writer"]
pub type W = crate::W<TsensContinuousSpec>;
#[doc = "Field `CONTINUOUS` writer - Запуск / остановка непрерывного измерения температуры"]
pub type ContinuousW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Запуск / остановка непрерывного измерения температуры"]
    #[inline(always)]
    pub fn continuous(&mut self) -> ContinuousW<TsensContinuousSpec> {
        ContinuousW::new(self, 0)
    }
}
#[doc = "Регистр запуска непрерываного измерения\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsens_continuous::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsensContinuousSpec;
impl crate::RegisterSpec for TsensContinuousSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tsens_continuous::W`](W) writer structure"]
impl crate::Writable for TsensContinuousSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSENS_CONTINUOUS to value 0"]
impl crate::Resettable for TsensContinuousSpec {
    const RESET_VALUE: u32 = 0;
}
