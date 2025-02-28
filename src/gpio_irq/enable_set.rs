#[doc = "Register `ENABLE_SET` reader"]
pub type R = crate::R<EnableSetSpec>;
#[doc = "Register `ENABLE_SET` writer"]
pub type W = crate::W<EnableSetSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Регистр разрешения прерываний При чтении – текущее состояние разрешений прерываний. При записи «1» разрешает прерывание от соответствующего канала.\n\nYou can [`read`](crate::Reg::read) this register and get [`enable_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnableSetSpec;
impl crate::RegisterSpec for EnableSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable_set::R`](R) reader structure"]
impl crate::Readable for EnableSetSpec {}
#[doc = "`write(|w| ..)` method takes [`enable_set::W`](W) writer structure"]
impl crate::Writable for EnableSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENABLE_SET to value 0"]
impl crate::Resettable for EnableSetSpec {
    const RESET_VALUE: u32 = 0;
}
