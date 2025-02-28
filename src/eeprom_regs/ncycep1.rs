#[doc = "Register `NCYCEP1` reader"]
pub type R = crate::R<Ncycep1Spec>;
#[doc = "Register `NCYCEP1` writer"]
pub type W = crate::W<Ncycep1Spec>;
#[doc = "Field `N_EP_1` reader - Длительность высокого уровня сигнала BUSY (вход Hard IP) в тактах. Рекомендуемое значение N RA = 2000000 нс / Pclk, где Pclk – период тактового сигнала в нс. Пример – 66667 так¬тов для частоты Fclk = 33,3 МГц. Запрещено устанавливать равным 0"]
pub type NEp1R = crate::FieldReader<u32>;
#[doc = "Field `N_EP_1` writer - Длительность высокого уровня сигнала BUSY (вход Hard IP) в тактах. Рекомендуемое значение N RA = 2000000 нс / Pclk, где Pclk – период тактового сигнала в нс. Пример – 66667 так¬тов для частоты Fclk = 33,3 МГц. Запрещено устанавливать равным 0"]
pub type NEp1W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - Длительность высокого уровня сигнала BUSY (вход Hard IP) в тактах. Рекомендуемое значение N RA = 2000000 нс / Pclk, где Pclk – период тактового сигнала в нс. Пример – 66667 так¬тов для частоты Fclk = 33,3 МГц. Запрещено устанавливать равным 0"]
    #[inline(always)]
    pub fn n_ep_1(&self) -> NEp1R {
        NEp1R::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - Длительность высокого уровня сигнала BUSY (вход Hard IP) в тактах. Рекомендуемое значение N RA = 2000000 нс / Pclk, где Pclk – период тактового сигнала в нс. Пример – 66667 так¬тов для частоты Fclk = 33,3 МГц. Запрещено устанавливать равным 0"]
    #[inline(always)]
    pub fn n_ep_1(&mut self) -> NEp1W<Ncycep1Spec> {
        NEp1W::new(self, 0)
    }
}
#[doc = "Регистр 1 подстройки длительности процедур стирания и программирования\n\nYou can [`read`](crate::Reg::read) this register and get [`ncycep1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ncycep1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ncycep1Spec;
impl crate::RegisterSpec for Ncycep1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ncycep1::R`](R) reader structure"]
impl crate::Readable for Ncycep1Spec {}
#[doc = "`write(|w| ..)` method takes [`ncycep1::W`](W) writer structure"]
impl crate::Writable for Ncycep1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NCYCEP1 to value 0x0001_046b"]
impl crate::Resettable for Ncycep1Spec {
    const RESET_VALUE: u32 = 0x0001_046b;
}
