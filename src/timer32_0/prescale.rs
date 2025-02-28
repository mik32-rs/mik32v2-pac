#[doc = "Register `PRESCALE` reader"]
pub type R = crate::R<PrescaleSpec>;
#[doc = "Register `PRESCALE` writer"]
pub type W = crate::W<PrescaleSpec>;
#[doc = "Field `TIM_PRESCALE` reader - Значение предварительного делителя. Предделитель вырабатывает тактовый сигнал для счета, частота которого в целое раз меньше входной"]
pub type TimPrescaleR = crate::FieldReader<u32>;
#[doc = "Field `TIM_PRESCALE` writer - Значение предварительного делителя. Предделитель вырабатывает тактовый сигнал для счета, частота которого в целое раз меньше входной"]
pub type TimPrescaleW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Значение предварительного делителя. Предделитель вырабатывает тактовый сигнал для счета, частота которого в целое раз меньше входной"]
    #[inline(always)]
    pub fn tim_prescale(&self) -> TimPrescaleR {
        TimPrescaleR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Значение предварительного делителя. Предделитель вырабатывает тактовый сигнал для счета, частота которого в целое раз меньше входной"]
    #[inline(always)]
    pub fn tim_prescale(&mut self) -> TimPrescaleW<PrescaleSpec> {
        TimPrescaleW::new(self, 0)
    }
}
#[doc = "значение делителя\n\nYou can [`read`](crate::Reg::read) this register and get [`prescale::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prescale::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrescaleSpec;
impl crate::RegisterSpec for PrescaleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prescale::R`](R) reader structure"]
impl crate::Readable for PrescaleSpec {}
#[doc = "`write(|w| ..)` method takes [`prescale::W`](W) writer structure"]
impl crate::Writable for PrescaleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRESCALE to value 0"]
impl crate::Resettable for PrescaleSpec {
    const RESET_VALUE: u32 = 0;
}
