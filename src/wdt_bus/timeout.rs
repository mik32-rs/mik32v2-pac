#[doc = "Register `TIMEOUT` reader"]
pub type R = crate::R<TimeoutSpec>;
#[doc = "Register `TIMEOUT` writer"]
pub type W = crate::W<TimeoutSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Определеяет количество циклов ожидания до формирования ошибки. Значения от 0 до 15. Количество циклов ожидания вычисляется как 2^TIMEOUT.\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimeoutSpec;
impl crate::RegisterSpec for TimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timeout::R`](R) reader structure"]
impl crate::Readable for TimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`timeout::W`](W) writer structure"]
impl crate::Writable for TimeoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMEOUT to value 0"]
impl crate::Resettable for TimeoutSpec {
    const RESET_VALUE: u32 = 0;
}
