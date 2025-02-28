#[doc = "Register `TSENS_IRQ` reader"]
pub type R = crate::R<TsensIrqSpec>;
#[doc = "Register `TSENS_IRQ` writer"]
pub type W = crate::W<TsensIrqSpec>;
#[doc = "Field `EOC_MASK` reader - Маска прерывания по окончанию преобразования"]
pub type EocMaskR = crate::BitReader;
#[doc = "Field `EOC_MASK` writer - Маска прерывания по окончанию преобразования"]
pub type EocMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HI_MASK` reader - Маска прерывания события «выход сенсора больше порогового значения HI_THRESHOLD»"]
pub type HiMaskR = crate::BitReader;
#[doc = "Field `HI_MASK` writer - Маска прерывания события «выход сенсора больше порогового значения HI_THRESHOLD»"]
pub type HiMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOW_MASK` reader - Маска прерывания события «выход сенсора меньше порогового значения LOW_ THRESHOLD»"]
pub type LowMaskR = crate::BitReader;
#[doc = "Field `LOW_MASK` writer - Маска прерывания события «выход сенсора меньше порогового значения LOW_ THRESHOLD»"]
pub type LowMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC_IRQ` reader - Статус прерывания по окончанию преобразования"]
pub type EocIrqR = crate::BitReader;
#[doc = "Field `HI_IRQ` reader - Статус прерывания события «выход сенсора больше порогового значения HI_THRESHOLD»"]
pub type HiIrqR = crate::BitReader;
#[doc = "Field `LOW_IRQ` reader - Статус прерывания события «выход сенсора меньше порогового значения LOW_ THRESHOLD»"]
pub type LowIrqR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Маска прерывания по окончанию преобразования"]
    #[inline(always)]
    pub fn eoc_mask(&self) -> EocMaskR {
        EocMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Маска прерывания события «выход сенсора больше порогового значения HI_THRESHOLD»"]
    #[inline(always)]
    pub fn hi_mask(&self) -> HiMaskR {
        HiMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Маска прерывания события «выход сенсора меньше порогового значения LOW_ THRESHOLD»"]
    #[inline(always)]
    pub fn low_mask(&self) -> LowMaskR {
        LowMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Статус прерывания по окончанию преобразования"]
    #[inline(always)]
    pub fn eoc_irq(&self) -> EocIrqR {
        EocIrqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Статус прерывания события «выход сенсора больше порогового значения HI_THRESHOLD»"]
    #[inline(always)]
    pub fn hi_irq(&self) -> HiIrqR {
        HiIrqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Статус прерывания события «выход сенсора меньше порогового значения LOW_ THRESHOLD»"]
    #[inline(always)]
    pub fn low_irq(&self) -> LowIrqR {
        LowIrqR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Маска прерывания по окончанию преобразования"]
    #[inline(always)]
    pub fn eoc_mask(&mut self) -> EocMaskW<TsensIrqSpec> {
        EocMaskW::new(self, 0)
    }
    #[doc = "Bit 1 - Маска прерывания события «выход сенсора больше порогового значения HI_THRESHOLD»"]
    #[inline(always)]
    pub fn hi_mask(&mut self) -> HiMaskW<TsensIrqSpec> {
        HiMaskW::new(self, 1)
    }
    #[doc = "Bit 2 - Маска прерывания события «выход сенсора меньше порогового значения LOW_ THRESHOLD»"]
    #[inline(always)]
    pub fn low_mask(&mut self) -> LowMaskW<TsensIrqSpec> {
        LowMaskW::new(self, 2)
    }
}
#[doc = "Регистр прерываний\n\nYou can [`read`](crate::Reg::read) this register and get [`tsens_irq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsens_irq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsensIrqSpec;
impl crate::RegisterSpec for TsensIrqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsens_irq::R`](R) reader structure"]
impl crate::Readable for TsensIrqSpec {}
#[doc = "`write(|w| ..)` method takes [`tsens_irq::W`](W) writer structure"]
impl crate::Writable for TsensIrqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSENS_IRQ to value 0"]
impl crate::Resettable for TsensIrqSpec {
    const RESET_VALUE: u32 = 0;
}
