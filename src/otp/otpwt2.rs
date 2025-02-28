#[doc = "Register `OTPWT2` reader"]
pub type R = crate::R<Otpwt2Spec>;
#[doc = "Register `OTPWT2` writer"]
pub type W = crate::W<Otpwt2Spec>;
#[doc = "Field `N_W` reader - Длительность высокого уровня сигнала we i (вход Hard IP) в тактах. Рекомендуемое значение N_W = 50000000 нс/Pclk, где Pclk – период тактового сигнала в нс. Пример– 1666667 тактов для частоты Fclk = 33,3 МГц"]
pub type NWR = crate::FieldReader<u32>;
#[doc = "Field `N_W` writer - Длительность высокого уровня сигнала we i (вход Hard IP) в тактах. Рекомендуемое значение N_W = 50000000 нс/Pclk, где Pclk – период тактового сигнала в нс. Пример– 1666667 тактов для частоты Fclk = 33,3 МГц"]
pub type NWW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Длительность высокого уровня сигнала we i (вход Hard IP) в тактах. Рекомендуемое значение N_W = 50000000 нс/Pclk, где Pclk – период тактового сигнала в нс. Пример– 1666667 тактов для частоты Fclk = 33,3 МГц"]
    #[inline(always)]
    pub fn n_w(&self) -> NWR {
        NWR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Длительность высокого уровня сигнала we i (вход Hard IP) в тактах. Рекомендуемое значение N_W = 50000000 нс/Pclk, где Pclk – период тактового сигнала в нс. Пример– 1666667 тактов для частоты Fclk = 33,3 МГц"]
    #[inline(always)]
    pub fn n_w(&mut self) -> NWW<Otpwt2Spec> {
        NWW::new(self, 0)
    }
}
#[doc = "Регистр подстройки длительности процедуры записи 2\n\nYou can [`read`](crate::Reg::read) this register and get [`otpwt2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otpwt2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Otpwt2Spec;
impl crate::RegisterSpec for Otpwt2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otpwt2::R`](R) reader structure"]
impl crate::Readable for Otpwt2Spec {}
#[doc = "`write(|w| ..)` method takes [`otpwt2::W`](W) writer structure"]
impl crate::Writable for Otpwt2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTPWT2 to value 0x0019_6e6b"]
impl crate::Resettable for Otpwt2Spec {
    const RESET_VALUE: u32 = 0x0019_6e6b;
}
