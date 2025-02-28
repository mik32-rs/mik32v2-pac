#[doc = "Register `EEDAT` reader"]
pub type R = crate::R<EedatSpec>;
#[doc = "Register `EEDAT` writer"]
pub type W = crate::W<EedatSpec>;
#[doc = "Field `DATA` reader - В процессе чтения/записи данного регистра производится автоматическое инкрементирование адреса (EEA)."]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - В процессе чтения/записи данного регистра производится автоматическое инкрементирование адреса (EEA)."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - В процессе чтения/записи данного регистра производится автоматическое инкрементирование адреса (EEA)."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - В процессе чтения/записи данного регистра производится автоматическое инкрементирование адреса (EEA)."]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<EedatSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Регистр чтения/записи данных\n\nYou can [`read`](crate::Reg::read) this register and get [`eedat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eedat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EedatSpec;
impl crate::RegisterSpec for EedatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eedat::R`](R) reader structure"]
impl crate::Readable for EedatSpec {}
#[doc = "`write(|w| ..)` method takes [`eedat::W`](W) writer structure"]
impl crate::Writable for EedatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EEDAT to value 0"]
impl crate::Resettable for EedatSpec {
    const RESET_VALUE: u32 = 0;
}
