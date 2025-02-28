#[doc = "Register `OTPDAT` reader"]
pub type R = crate::R<OtpdatSpec>;
#[doc = "Register `OTPDAT` writer"]
pub type W = crate::W<OtpdatSpec>;
#[doc = "Field `DATA` reader - Сразу после выполнения записи в этот регистр будет запущена внутренняя процедура записи загруженных данных в массив antifuse OTP. Временные параметры данной операции могут быть настроены с помощью других регистров. При чтении из этого регистра считываются данные, полученные из OTP"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Сразу после выполнения записи в этот регистр будет запущена внутренняя процедура записи загруженных данных в массив antifuse OTP. Временные параметры данной операции могут быть настроены с помощью других регистров. При чтении из этого регистра считываются данные, полученные из OTP"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Сразу после выполнения записи в этот регистр будет запущена внутренняя процедура записи загруженных данных в массив antifuse OTP. Временные параметры данной операции могут быть настроены с помощью других регистров. При чтении из этого регистра считываются данные, полученные из OTP"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Сразу после выполнения записи в этот регистр будет запущена внутренняя процедура записи загруженных данных в массив antifuse OTP. Временные параметры данной операции могут быть настроены с помощью других регистров. При чтении из этого регистра считываются данные, полученные из OTP"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<OtpdatSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Регистр чтения/записи данных\n\nYou can [`read`](crate::Reg::read) this register and get [`otpdat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otpdat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpdatSpec;
impl crate::RegisterSpec for OtpdatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otpdat::R`](R) reader structure"]
impl crate::Readable for OtpdatSpec {}
#[doc = "`write(|w| ..)` method takes [`otpdat::W`](W) writer structure"]
impl crate::Writable for OtpdatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTPDAT to value 0"]
impl crate::Resettable for OtpdatSpec {
    const RESET_VALUE: u32 = 0;
}
