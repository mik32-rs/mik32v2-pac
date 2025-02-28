#[doc = "Register `DIRECTION_IN` reader"]
pub type R = crate::R<DirectionInSpec>;
#[doc = "Register `DIRECTION_IN` writer"]
pub type W = crate::W<DirectionInSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Установка направления выводов как вход При чтении – текущее направление выводов: 0 – выход; 1 – вход. Запись «1» устанавливает соответствующий вывод как «вход»\n\nYou can [`read`](crate::Reg::read) this register and get [`direction_in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`direction_in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DirectionInSpec;
impl crate::RegisterSpec for DirectionInSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`direction_in::R`](R) reader structure"]
impl crate::Readable for DirectionInSpec {}
#[doc = "`write(|w| ..)` method takes [`direction_in::W`](W) writer structure"]
impl crate::Writable for DirectionInSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIRECTION_IN to value 0xff"]
impl crate::Resettable for DirectionInSpec {
    const RESET_VALUE: u32 = 0xff;
}
