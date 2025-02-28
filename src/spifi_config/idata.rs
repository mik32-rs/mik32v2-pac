#[doc = "Register `IDATA` reader"]
pub type R = crate::R<IdataSpec>;
#[doc = "Register `IDATA` writer"]
pub type W = crate::W<IdataSpec>;
#[doc = "Field `IDATA` reader - Промежуточные данные"]
pub type IdataR = crate::FieldReader<u32>;
#[doc = "Field `IDATA` writer - Промежуточные данные"]
pub type IdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Промежуточные данные"]
    #[inline(always)]
    pub fn idata(&self) -> IdataR {
        IdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Промежуточные данные"]
    #[inline(always)]
    pub fn idata(&mut self) -> IdataW<IdataSpec> {
        IdataW::new(self, 0)
    }
}
#[doc = "SPIFI регистр промежуточных данных. Данный регистр должен быть настроен перед инициализацией команды, в которой используются промежуточные данные. При выполнении команды первым выводится младший бит слова\n\nYou can [`read`](crate::Reg::read) this register and get [`idata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdataSpec;
impl crate::RegisterSpec for IdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idata::R`](R) reader structure"]
impl crate::Readable for IdataSpec {}
#[doc = "`write(|w| ..)` method takes [`idata::W`](W) writer structure"]
impl crate::Writable for IdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDATA to value 0"]
impl crate::Resettable for IdataSpec {
    const RESET_VALUE: u32 = 0;
}
