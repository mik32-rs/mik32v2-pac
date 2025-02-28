#[doc = "Register `CLEAR` writer"]
pub type W = crate::W<ClearSpec>;
impl core::fmt::Debug for crate::generic::Reg<ClearSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Регистр сброса флагов прерываний по событию. Запись «1» – очищает флаг прерывания соответствующего канала\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearSpec;
impl crate::RegisterSpec for ClearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clear::W`](W) writer structure"]
impl crate::Writable for ClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLEAR to value 0"]
impl crate::Resettable for ClearSpec {
    const RESET_VALUE: u32 = 0;
}
