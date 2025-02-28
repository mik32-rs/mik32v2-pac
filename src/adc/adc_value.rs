#[doc = "Register `ADC_VALUE` reader"]
pub type R = crate::R<AdcValueSpec>;
#[doc = "Field `VALUE` reader - Результат преобразования"]
pub type ValueR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Результат преобразования"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Регистр данных\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_value::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcValueSpec;
impl crate::RegisterSpec for AdcValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_value::R`](R) reader structure"]
impl crate::Readable for AdcValueSpec {}
#[doc = "`reset()` method sets ADC_VALUE to value 0"]
impl crate::Resettable for AdcValueSpec {
    const RESET_VALUE: u32 = 0;
}
