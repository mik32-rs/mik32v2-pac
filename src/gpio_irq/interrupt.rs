#[doc = "Register `INTERRUPT` reader"]
pub type R = crate::R<InterruptSpec>;
#[doc = "Register `INTERRUPT` writer"]
pub type W = crate::W<InterruptSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Текущее состояние прерываний. Номер бита соответсвует номеру канала.\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InterruptSpec;
impl crate::RegisterSpec for InterruptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt::R`](R) reader structure"]
impl crate::Readable for InterruptSpec {}
#[doc = "`write(|w| ..)` method takes [`interrupt::W`](W) writer structure"]
impl crate::Writable for InterruptSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTERRUPT to value 0"]
impl crate::Resettable for InterruptSpec {
    const RESET_VALUE: u32 = 0;
}
