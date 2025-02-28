#[doc = "Register `INIT` writer"]
pub type W = crate::W<InitSpec>;
impl core::fmt::Debug for crate::generic::Reg<InitSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Буфер данных вектора инициализации\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InitSpec;
impl crate::RegisterSpec for InitSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`init::W`](W) writer structure"]
impl crate::Writable for InitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INIT to value 0"]
impl crate::Resettable for InitSpec {
    const RESET_VALUE: u32 = 0;
}
