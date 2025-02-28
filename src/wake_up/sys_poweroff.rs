#[doc = "Register `SYS_POWEROFF` writer"]
pub type W = crate::W<SysPoweroffSpec>;
impl core::fmt::Debug for crate::generic::Reg<SysPoweroffSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Запись в регистр «1» отключает питание системного домена\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_poweroff::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysPoweroffSpec;
impl crate::RegisterSpec for SysPoweroffSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sys_poweroff::W`](W) writer structure"]
impl crate::Writable for SysPoweroffSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYS_POWEROFF to value 0"]
impl crate::Resettable for SysPoweroffSpec {
    const RESET_VALUE: u32 = 0;
}
