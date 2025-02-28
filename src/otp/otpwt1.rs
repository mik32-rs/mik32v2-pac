#[doc = "Register `OTPWT1` reader"]
pub type R = crate::R<Otpwt1Spec>;
#[doc = "Register `OTPWT1` writer"]
pub type W = crate::W<Otpwt1Spec>;
#[doc = "Field `N_SU` reader - Время между моментом окончания транзакции на APB и положительным фронтом we i в тактах. Должно использоваться для обеспечения требования к временам предустановки Hard IP. Рекомендуемое значение N_SU = =ceil(40/Pclk), где Pclk – период тактового сигнала в нс, ceil – функция округле-ния до ближайшего большего целого числа. Пример – два такта для частоты Fclk = 33,3 МГц. Для корректной работы схемы значение должно быть больше 0"]
pub type NSuR = crate::FieldReader;
#[doc = "Field `N_SU` writer - Время между моментом окончания транзакции на APB и положительным фронтом we i в тактах. Должно использоваться для обеспечения требования к временам предустановки Hard IP. Рекомендуемое значение N_SU = =ceil(40/Pclk), где Pclk – период тактового сигнала в нс, ceil – функция округле-ния до ближайшего большего целого числа. Пример – два такта для частоты Fclk = 33,3 МГц. Для корректной работы схемы значение должно быть больше 0"]
pub type NSuW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `N_H` reader - Время между задним фронтом we_i и задним фронтом OTPSTA.BSY в тактах. Должно использоваться для обеспече¬ния требования к временам удержания Hard IP. Рекомендуемое значение N_H=ceil(40/Pclk), где Pclk – период тактового сигнала в нс, ceil – функция округления до ближайшего боль¬шего целого числа. Пример – ldf такта для частоты Fclk = 33,3 МГц. Для корректной работы схемы значение должно быть больше 0"]
pub type NHR = crate::FieldReader;
#[doc = "Field `N_H` writer - Время между задним фронтом we_i и задним фронтом OTPSTA.BSY в тактах. Должно использоваться для обеспече¬ния требования к временам удержания Hard IP. Рекомендуемое значение N_H=ceil(40/Pclk), где Pclk – период тактового сигнала в нс, ceil – функция округления до ближайшего боль¬шего целого числа. Пример – ldf такта для частоты Fclk = 33,3 МГц. Для корректной работы схемы значение должно быть больше 0"]
pub type NHW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Время между моментом окончания транзакции на APB и положительным фронтом we i в тактах. Должно использоваться для обеспечения требования к временам предустановки Hard IP. Рекомендуемое значение N_SU = =ceil(40/Pclk), где Pclk – период тактового сигнала в нс, ceil – функция округле-ния до ближайшего большего целого числа. Пример – два такта для частоты Fclk = 33,3 МГц. Для корректной работы схемы значение должно быть больше 0"]
    #[inline(always)]
    pub fn n_su(&self) -> NSuR {
        NSuR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - Время между задним фронтом we_i и задним фронтом OTPSTA.BSY в тактах. Должно использоваться для обеспече¬ния требования к временам удержания Hard IP. Рекомендуемое значение N_H=ceil(40/Pclk), где Pclk – период тактового сигнала в нс, ceil – функция округления до ближайшего боль¬шего целого числа. Пример – ldf такта для частоты Fclk = 33,3 МГц. Для корректной работы схемы значение должно быть больше 0"]
    #[inline(always)]
    pub fn n_h(&self) -> NHR {
        NHR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Время между моментом окончания транзакции на APB и положительным фронтом we i в тактах. Должно использоваться для обеспечения требования к временам предустановки Hard IP. Рекомендуемое значение N_SU = =ceil(40/Pclk), где Pclk – период тактового сигнала в нс, ceil – функция округле-ния до ближайшего большего целого числа. Пример – два такта для частоты Fclk = 33,3 МГц. Для корректной работы схемы значение должно быть больше 0"]
    #[inline(always)]
    pub fn n_su(&mut self) -> NSuW<Otpwt1Spec> {
        NSuW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Время между задним фронтом we_i и задним фронтом OTPSTA.BSY в тактах. Должно использоваться для обеспече¬ния требования к временам удержания Hard IP. Рекомендуемое значение N_H=ceil(40/Pclk), где Pclk – период тактового сигнала в нс, ceil – функция округления до ближайшего боль¬шего целого числа. Пример – ldf такта для частоты Fclk = 33,3 МГц. Для корректной работы схемы значение должно быть больше 0"]
    #[inline(always)]
    pub fn n_h(&mut self) -> NHW<Otpwt1Spec> {
        NHW::new(self, 8)
    }
}
#[doc = "Регистр подстройки длительности процедуры записи 1\n\nYou can [`read`](crate::Reg::read) this register and get [`otpwt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otpwt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Otpwt1Spec;
impl crate::RegisterSpec for Otpwt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otpwt1::R`](R) reader structure"]
impl crate::Readable for Otpwt1Spec {}
#[doc = "`write(|w| ..)` method takes [`otpwt1::W`](W) writer structure"]
impl crate::Writable for Otpwt1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTPWT1 to value 0x0202"]
impl crate::Resettable for Otpwt1Spec {
    const RESET_VALUE: u32 = 0x0202;
}
