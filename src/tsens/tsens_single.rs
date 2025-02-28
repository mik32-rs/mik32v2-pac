#[doc = "Register `TSENS_SINGLE` writer"]
pub type W = crate::W<TsensSingleSpec>;
#[doc = "Field `SINGLE` writer - Запуск одного измерения"]
pub type SingleW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Запуск одного измерения"]
    #[inline(always)]
    pub fn single(&mut self) -> SingleW<TsensSingleSpec> {
        SingleW::new(self, 0)
    }
}
#[doc = "Регистр запуска однократного измерения\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsens_single::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsensSingleSpec;
impl crate::RegisterSpec for TsensSingleSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tsens_single::W`](W) writer structure"]
impl crate::Writable for TsensSingleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSENS_SINGLE to value 0"]
impl crate::Resettable for TsensSingleSpec {
    const RESET_VALUE: u32 = 0;
}
