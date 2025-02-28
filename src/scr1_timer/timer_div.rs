#[doc = "Register `TIMER_DIV` reader"]
pub type R = crate::R<TimerDivSpec>;
#[doc = "Register `TIMER_DIV` writer"]
pub type W = crate::W<TimerDivSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Делитель частоты. Счет идет каждые DIV+1 такта частоты\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimerDivSpec;
impl crate::RegisterSpec for TimerDivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_div::R`](R) reader structure"]
impl crate::Readable for TimerDivSpec {}
#[doc = "`write(|w| ..)` method takes [`timer_div::W`](W) writer structure"]
impl crate::Writable for TimerDivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER_DIV to value 0"]
impl crate::Resettable for TimerDivSpec {
    const RESET_VALUE: u32 = 0;
}
