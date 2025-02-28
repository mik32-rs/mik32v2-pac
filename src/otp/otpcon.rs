#[doc = "Register `OTPCON` writer"]
pub type W = crate::W<OtpconSpec>;
#[doc = "Field `APBNWS` writer - Отключение вставки тактов ожидания в процессе обмена по APB при чтении данных (обращении к регистру OTPDAT на чтение). Если такты ожидания отключены (APBNWS=1), то требуется производить опрос флага OTPSTA.BSY после за¬писи адреса в OTPA до тех пор, пока EESTA.BSY не станет равным 0"]
pub type ApbnwsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `man_we_i` writer - Ручное управление сигналом we i блока Hard IP. Используется для тестирования"]
pub type ManWeIW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `man_re_i` writer - Ручное управление сигналом re i блока Hard IP. Используется для тестирования"]
pub type ManReIW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MME` writer - Режим ручного управления включен. В этом режиме автоматизированные операции чтения и записи не выполняются. Используется для тестирования"]
pub type MmeW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Отключение вставки тактов ожидания в процессе обмена по APB при чтении данных (обращении к регистру OTPDAT на чтение). Если такты ожидания отключены (APBNWS=1), то требуется производить опрос флага OTPSTA.BSY после за¬писи адреса в OTPA до тех пор, пока EESTA.BSY не станет равным 0"]
    #[inline(always)]
    pub fn apbnws(&mut self) -> ApbnwsW<OtpconSpec> {
        ApbnwsW::new(self, 0)
    }
    #[doc = "Bit 1 - Ручное управление сигналом we i блока Hard IP. Используется для тестирования"]
    #[inline(always)]
    pub fn man_we_i(&mut self) -> ManWeIW<OtpconSpec> {
        ManWeIW::new(self, 1)
    }
    #[doc = "Bit 2 - Ручное управление сигналом re i блока Hard IP. Используется для тестирования"]
    #[inline(always)]
    pub fn man_re_i(&mut self) -> ManReIW<OtpconSpec> {
        ManReIW::new(self, 2)
    }
    #[doc = "Bit 3 - Режим ручного управления включен. В этом режиме автоматизированные операции чтения и записи не выполняются. Используется для тестирования"]
    #[inline(always)]
    pub fn mme(&mut self) -> MmeW<OtpconSpec> {
        MmeW::new(self, 3)
    }
}
#[doc = "Регистр управления\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otpcon::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpconSpec;
impl crate::RegisterSpec for OtpconSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`otpcon::W`](W) writer structure"]
impl crate::Writable for OtpconSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTPCON to value 0"]
impl crate::Resettable for OtpconSpec {
    const RESET_VALUE: u32 = 0;
}
