#[doc = "Register `MTIMEH` reader"]
pub type R = crate::R<MtimehSpec>;
#[doc = "Register `MTIMEH` writer"]
pub type W = crate::W<MtimehSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Счетчик таймера, старшее слово\n\nYou can [`read`](crate::Reg::read) this register and get [`mtimeh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtimeh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtimehSpec;
impl crate::RegisterSpec for MtimehSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtimeh::R`](R) reader structure"]
impl crate::Readable for MtimehSpec {}
#[doc = "`write(|w| ..)` method takes [`mtimeh::W`](W) writer structure"]
impl crate::Writable for MtimehSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTIMEH to value 0"]
impl crate::Resettable for MtimehSpec {
    const RESET_VALUE: u32 = 0;
}
