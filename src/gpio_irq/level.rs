#[doc = "Register `LEVEL` reader"]
pub type R = crate::R<LevelSpec>;
#[doc = "Register `LEVEL` writer"]
pub type W = crate::W<LevelSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Регистр типа прерываний. При чтении – текущий тип прерывания (инвертированный): «0» – по событию; «1» – по уровню. Запись «1» – прерывание формируется по уровню для соответствующего канала Запись «0» - не влияет\n\nYou can [`read`](crate::Reg::read) this register and get [`level::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`level::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LevelSpec;
impl crate::RegisterSpec for LevelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`level::R`](R) reader structure"]
impl crate::Readable for LevelSpec {}
#[doc = "`write(|w| ..)` method takes [`level::W`](W) writer structure"]
impl crate::Writable for LevelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LEVEL to value 0xff"]
impl crate::Resettable for LevelSpec {
    const RESET_VALUE: u32 = 0xff;
}
