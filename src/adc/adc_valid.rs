#[doc = "Register `ADC_VALID` reader"]
pub type R = crate::R<AdcValidSpec>;
#[doc = "Field `VALID` reader - Признак наличия актуальных данных"]
pub type ValidR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Признак наличия актуальных данных"]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new((self.bits & 1) != 0)
    }
}
#[doc = "Регистр статуса\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_valid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcValidSpec;
impl crate::RegisterSpec for AdcValidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_valid::R`](R) reader structure"]
impl crate::Readable for AdcValidSpec {}
#[doc = "`reset()` method sets ADC_VALID to value 0"]
impl crate::Resettable for AdcValidSpec {
    const RESET_VALUE: u32 = 0;
}
