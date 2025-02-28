#[doc = "Register `TSENS_CLEAR_IRQ` writer"]
pub type W = crate::W<TsensClearIrqSpec>;
#[doc = "Field `EOC_CLEAR` writer - Сброс прерывания по окончанию преобразования"]
pub type EocClearW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `HI_CLEAR` writer - Сброс прерывания события «выход сенсора больше порогового значения HI_THRESHOLD»"]
pub type HiClearW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LOW_CLEAR` writer - Сброс прерывания события «выход сенсора меньше порогового значения LOW_ THRESHOLD»"]
pub type LowClearW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "Bit 0 - Сброс прерывания по окончанию преобразования"]
    #[inline(always)]
    pub fn eoc_clear(&mut self) -> EocClearW<TsensClearIrqSpec> {
        EocClearW::new(self, 0)
    }
    #[doc = "Bit 1 - Сброс прерывания события «выход сенсора больше порогового значения HI_THRESHOLD»"]
    #[inline(always)]
    pub fn hi_clear(&mut self) -> HiClearW<TsensClearIrqSpec> {
        HiClearW::new(self, 1)
    }
    #[doc = "Bit 2 - Сброс прерывания события «выход сенсора меньше порогового значения LOW_ THRESHOLD»"]
    #[inline(always)]
    pub fn low_clear(&mut self) -> LowClearW<TsensClearIrqSpec> {
        LowClearW::new(self, 2)
    }
}
#[doc = "Регистр сброса прерываний\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsens_clear_irq::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsensClearIrqSpec;
impl crate::RegisterSpec for TsensClearIrqSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tsens_clear_irq::W`](W) writer structure"]
impl crate::Writable for TsensClearIrqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x07;
}
#[doc = "`reset()` method sets TSENS_CLEAR_IRQ to value 0"]
impl crate::Resettable for TsensClearIrqSpec {
    const RESET_VALUE: u32 = 0;
}
