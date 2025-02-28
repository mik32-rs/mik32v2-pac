#[doc = "Register `NCYCEP2` reader"]
pub type R = crate::R<Ncycep2Spec>;
#[doc = "Register `NCYCEP2` writer"]
pub type W = crate::W<Ncycep2Spec>;
#[doc = "Field `N_EP_2` reader - Задержка между задними фронтами BUSY и HvonValid (входы Hard IP) в тактах. Рекомендуемое значение N RA = 15000 нс / Pclk, где Pclk – период тактового сигнала в нс. Пример – 500 тактов для частоты Fclk = 33,3 МГц. Запрещено устанавливать равным 0"]
pub type NEp2R = crate::FieldReader<u16>;
#[doc = "Field `N_EP_2` writer - Задержка между задними фронтами BUSY и HvonValid (входы Hard IP) в тактах. Рекомендуемое значение N RA = 15000 нс / Pclk, где Pclk – период тактового сигнала в нс. Пример – 500 тактов для частоты Fclk = 33,3 МГц. Запрещено устанавливать равным 0"]
pub type NEp2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Задержка между задними фронтами BUSY и HvonValid (входы Hard IP) в тактах. Рекомендуемое значение N RA = 15000 нс / Pclk, где Pclk – период тактового сигнала в нс. Пример – 500 тактов для частоты Fclk = 33,3 МГц. Запрещено устанавливать равным 0"]
    #[inline(always)]
    pub fn n_ep_2(&self) -> NEp2R {
        NEp2R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Задержка между задними фронтами BUSY и HvonValid (входы Hard IP) в тактах. Рекомендуемое значение N RA = 15000 нс / Pclk, где Pclk – период тактового сигнала в нс. Пример – 500 тактов для частоты Fclk = 33,3 МГц. Запрещено устанавливать равным 0"]
    #[inline(always)]
    pub fn n_ep_2(&mut self) -> NEp2W<Ncycep2Spec> {
        NEp2W::new(self, 0)
    }
}
#[doc = "Регистр 2 подстройки длительности процедур стирания и программирования\n\nYou can [`read`](crate::Reg::read) this register and get [`ncycep2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ncycep2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ncycep2Spec;
impl crate::RegisterSpec for Ncycep2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ncycep2::R`](R) reader structure"]
impl crate::Readable for Ncycep2Spec {}
#[doc = "`write(|w| ..)` method takes [`ncycep2::W`](W) writer structure"]
impl crate::Writable for Ncycep2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NCYCEP2 to value 0x32"]
impl crate::Resettable for Ncycep2Spec {
    const RESET_VALUE: u32 = 0x32;
}
