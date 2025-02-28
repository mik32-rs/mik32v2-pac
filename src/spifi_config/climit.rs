#[doc = "Register `CLIMIT` reader"]
pub type R = crate::R<ClimitSpec>;
#[doc = "Register `CLIMIT` writer"]
pub type W = crate::W<ClimitSpec>;
#[doc = "Field `CLIMIT` reader - Верхний предел кэшируемой памяти"]
pub type ClimitR = crate::FieldReader<u32>;
#[doc = "Field `CLIMIT` writer - Верхний предел кэшируемой памяти"]
pub type ClimitW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Верхний предел кэшируемой памяти"]
    #[inline(always)]
    pub fn climit(&self) -> ClimitR {
        ClimitR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Верхний предел кэшируемой памяти"]
    #[inline(always)]
    pub fn climit(&mut self) -> ClimitW<ClimitSpec> {
        ClimitW::new(self, 0)
    }
}
#[doc = "SPIFI регистр верхней границы адреса кеширования\n\nYou can [`read`](crate::Reg::read) this register and get [`climit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`climit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClimitSpec;
impl crate::RegisterSpec for ClimitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`climit::R`](R) reader structure"]
impl crate::Readable for ClimitSpec {}
#[doc = "`write(|w| ..)` method takes [`climit::W`](W) writer structure"]
impl crate::Writable for ClimitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLIMIT to value 0"]
impl crate::Resettable for ClimitSpec {
    const RESET_VALUE: u32 = 0;
}
