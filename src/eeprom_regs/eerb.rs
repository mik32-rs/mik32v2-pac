#[doc = "Register `EERB` reader"]
pub type R = crate::R<EerbSpec>;
#[doc = "Field `CORRECT` reader - Содержит 6 бит коррекции последнего считанного слова"]
pub type CorrectR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Содержит 6 бит коррекции последнего считанного слова"]
    #[inline(always)]
    pub fn correct(&self) -> CorrectR {
        CorrectR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Регистр бит коррекции прочитанного слова\n\nYou can [`read`](crate::Reg::read) this register and get [`eerb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EerbSpec;
impl crate::RegisterSpec for EerbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eerb::R`](R) reader structure"]
impl crate::Readable for EerbSpec {}
#[doc = "`reset()` method sets EERB to value 0"]
impl crate::Resettable for EerbSpec {
    const RESET_VALUE: u32 = 0;
}
