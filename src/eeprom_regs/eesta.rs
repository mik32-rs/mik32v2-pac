#[doc = "Register `EESTA` reader"]
pub type R = crate::R<EestaSpec>;
#[doc = "Register `EESTA` writer"]
pub type W = crate::W<EestaSpec>;
#[doc = "Блок занят (выполняется запрошенная операция)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bsy {
    #[doc = "0: Блок готов"]
    Ready = 0,
    #[doc = "1: Блок занят (выполняется запрошенная операция)"]
    Busy = 1,
}
impl From<Bsy> for bool {
    #[inline(always)]
    fn from(variant: Bsy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSY` reader - Блок занят (выполняется запрошенная операция)"]
pub type BsyR = crate::BitReader<Bsy>;
impl BsyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bsy {
        match self.bits {
            false => Bsy::Ready,
            true => Bsy::Busy,
        }
    }
    #[doc = "Блок готов"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Bsy::Ready
    }
    #[doc = "Блок занят (выполняется запрошенная операция)"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Bsy::Busy
    }
}
#[doc = "Field `SERR` writer - Индикатор одиночной (исправимой) ошибки. Аппаратно может быть только поднят, т.е. если после слова с ошибкой считано слово без ошибки, флаг не очистится. Может быть очищен программно путем записи 0"]
pub type SerrW<'a, REG> = crate::BitWriter0C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Блок занят (выполняется запрошенная операция)"]
    #[inline(always)]
    pub fn bsy(&self) -> BsyR {
        BsyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Индикатор одиночной (исправимой) ошибки. Аппаратно может быть только поднят, т.е. если после слова с ошибкой считано слово без ошибки, флаг не очистится. Может быть очищен программно путем записи 0"]
    #[inline(always)]
    pub fn serr(&mut self) -> SerrW<EestaSpec> {
        SerrW::new(self, 1)
    }
}
#[doc = "Регистр статуса\n\nYou can [`read`](crate::Reg::read) this register and get [`eesta::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eesta::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EestaSpec;
impl crate::RegisterSpec for EestaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eesta::R`](R) reader structure"]
impl crate::Readable for EestaSpec {}
#[doc = "`write(|w| ..)` method takes [`eesta::W`](W) writer structure"]
impl crate::Writable for EestaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x02;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EESTA to value 0"]
impl crate::Resettable for EestaSpec {
    const RESET_VALUE: u32 = 0;
}
