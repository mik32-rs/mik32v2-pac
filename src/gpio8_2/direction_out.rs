#[doc = "Register `DIRECTION_OUT` reader"]
pub type R = crate::R<DirectionOutSpec>;
#[doc = "Register `DIRECTION_OUT` writer"]
pub type W = crate::W<DirectionOutSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Установка направления выводов как выход При чтении – текущее направление выводов: 0 – выход; 1 – вход. Запись «1» устанавливает соответствующий вывод как «выход»\n\nYou can [`read`](crate::Reg::read) this register and get [`direction_out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`direction_out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DirectionOutSpec;
impl crate::RegisterSpec for DirectionOutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`direction_out::R`](R) reader structure"]
impl crate::Readable for DirectionOutSpec {}
#[doc = "`write(|w| ..)` method takes [`direction_out::W`](W) writer structure"]
impl crate::Writable for DirectionOutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIRECTION_OUT to value 0xff"]
impl crate::Resettable for DirectionOutSpec {
    const RESET_VALUE: u32 = 0xff;
}
