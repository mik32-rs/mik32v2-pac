#[doc = "Register `OTPDEC` reader"]
pub type R = crate::R<OtpdecSpec>;
#[doc = "Field `DECO` reader - Выход дешифратора строк блока OTP. Используется для тестирования. Проверка функционирования дешифратора строк осуществляется путем смены адреса тестового столбца, страницы и контроля выходов дешифратора"]
pub type DecoR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8 - Выход дешифратора строк блока OTP. Используется для тестирования. Проверка функционирования дешифратора строк осуществляется путем смены адреса тестового столбца, страницы и контроля выходов дешифратора"]
    #[inline(always)]
    pub fn deco(&self) -> DecoR {
        DecoR::new((self.bits & 0x01ff) as u16)
    }
}
#[doc = "Регистр дешифратора строк\n\nYou can [`read`](crate::Reg::read) this register and get [`otpdec::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpdecSpec;
impl crate::RegisterSpec for OtpdecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otpdec::R`](R) reader structure"]
impl crate::Readable for OtpdecSpec {}
#[doc = "`reset()` method sets OTPDEC to value 0"]
impl crate::Resettable for OtpdecSpec {
    const RESET_VALUE: u32 = 0;
}
