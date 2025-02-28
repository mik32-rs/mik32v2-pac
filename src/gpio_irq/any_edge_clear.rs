#[doc = "Register `ANY_EDGE_CLEAR` reader"]
pub type R = crate::R<AnyEdgeClearSpec>;
#[doc = "Register `ANY_EDGE_CLEAR` writer"]
pub type W = crate::W<AnyEdgeClearSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Регистр прерываний по любому событию. Запись «1» – прерывание не формируется по любому изменению соответствующего канала\n\nYou can [`read`](crate::Reg::read) this register and get [`any_edge_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`any_edge_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnyEdgeClearSpec;
impl crate::RegisterSpec for AnyEdgeClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`any_edge_clear::R`](R) reader structure"]
impl crate::Readable for AnyEdgeClearSpec {}
#[doc = "`write(|w| ..)` method takes [`any_edge_clear::W`](W) writer structure"]
impl crate::Writable for AnyEdgeClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANY_EDGE_CLEAR to value 0"]
impl crate::Resettable for AnyEdgeClearSpec {
    const RESET_VALUE: u32 = 0;
}
