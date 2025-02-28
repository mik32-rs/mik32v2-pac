#[doc = "Register `LEVEL_SET` reader"]
pub type R = crate::R<LevelSetSpec>;
#[doc = "Register `LEVEL_SET` writer"]
pub type W = crate::W<LevelSetSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Регистр выбора уровня / события прерываний. При чтении: «0» – прерывания формируются по спаду или уровню логического «0»; «1» – прерывания формируются по нарастающему фронту или уровню логической «1» Запись «1» – прерывание формируется по нарастающему фронту или уровню логической «1» для соответствующего канала\n\nYou can [`read`](crate::Reg::read) this register and get [`level_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`level_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LevelSetSpec;
impl crate::RegisterSpec for LevelSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`level_set::R`](R) reader structure"]
impl crate::Readable for LevelSetSpec {}
#[doc = "`write(|w| ..)` method takes [`level_set::W`](W) writer structure"]
impl crate::Writable for LevelSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LEVEL_SET to value 0"]
impl crate::Resettable for LevelSetSpec {
    const RESET_VALUE: u32 = 0;
}
