#[doc = "Register `RTC_CONTROL` writer"]
pub type W = crate::W<RtcControlSpec>;
impl core::fmt::Debug for crate::generic::Reg<RtcControlSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "Сброс RTC происходит при записи “1”\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_control::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcControlSpec;
impl crate::RegisterSpec for RtcControlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rtc_control::W`](W) writer structure"]
impl crate::Writable for RtcControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_CONTROL to value 0"]
impl crate::Resettable for RtcControlSpec {
    const RESET_VALUE: u32 = 0;
}
