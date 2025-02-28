#[doc = "Register `EEA` writer"]
pub type W = crate::W<EeaSpec>;
#[doc = "Field `ADDR` writer - Адрес слова для выполнения стирания/програмирования /чтения. Если необходимо выполнить стирание или программирование, данный адрес должен использоваться в процедуре заполнения буфера записи"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl W {
    #[doc = "Bits 2:12 - Адрес слова для выполнения стирания/програмирования /чтения. Если необходимо выполнить стирание или программирование, данный адрес должен использоваться в процедуре заполнения буфера записи"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<EeaSpec> {
        AddrW::new(self, 2)
    }
}
#[doc = "Регистр адреса страницы\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eea::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EeaSpec;
impl crate::RegisterSpec for EeaSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`eea::W`](W) writer structure"]
impl crate::Writable for EeaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EEA to value 0"]
impl crate::Resettable for EeaSpec {
    const RESET_VALUE: u32 = 0;
}
