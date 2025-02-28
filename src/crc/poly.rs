#[doc = "Register `POLY` reader"]
pub type R = crate::R<PolySpec>;
#[doc = "Register `POLY` writer"]
pub type W = crate::W<PolySpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Регистр полинома\n\nYou can [`read`](crate::Reg::read) this register and get [`poly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PolySpec;
impl crate::RegisterSpec for PolySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`poly::R`](R) reader structure"]
impl crate::Readable for PolySpec {}
#[doc = "`write(|w| ..)` method takes [`poly::W`](W) writer structure"]
impl crate::Writable for PolySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POLY to value 0"]
impl crate::Resettable for PolySpec {
    const RESET_VALUE: u32 = 0;
}
