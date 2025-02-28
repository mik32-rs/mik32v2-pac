#[doc = "Register `ENABLE_CLEAR` reader"]
pub type R = crate::R<EnableClearSpec>;
#[doc = "Register `ENABLE_CLEAR` writer"]
pub type W = crate::W<EnableClearSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Регистр запрета прерываний При чтении – текущее состояние разрешений прерываний. При записи «1» запрещает прерывание от соответствующего канала\n\nYou can [`read`](crate::Reg::read) this register and get [`enable_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnableClearSpec;
impl crate::RegisterSpec for EnableClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable_clear::R`](R) reader structure"]
impl crate::Readable for EnableClearSpec {}
#[doc = "`write(|w| ..)` method takes [`enable_clear::W`](W) writer structure"]
impl crate::Writable for EnableClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENABLE_CLEAR to value 0"]
impl crate::Resettable for EnableClearSpec {
    const RESET_VALUE: u32 = 0;
}
