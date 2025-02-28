#[doc = "Register `FREQ_STATUS` reader"]
pub type R = crate::R<FreqStatusSpec>;
#[doc = "Register `FREQ_STATUS` writer"]
pub type W = crate::W<FreqStatusSpec>;
#[doc = "Field `MASK_LSI32K` reader - Статус частоты LSI32K"]
pub type MaskLsi32kR = crate::BitReader;
#[doc = "Field `MASK_OSC32K` reader - Статус частоты внешнего осциллятора 32 кГц OSC32K"]
pub type MaskOsc32kR = crate::BitReader;
#[doc = "Field `MASK_HSI32M` reader - Статус частоты HSI32M"]
pub type MaskHsi32mR = crate::BitReader;
#[doc = "Field `MASK_OSC32M` reader - Статус частоты внешнего осциллятора 32 МГц OSC32M"]
pub type MaskOsc32mR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Статус частоты LSI32K"]
    #[inline(always)]
    pub fn mask_lsi32k(&self) -> MaskLsi32kR {
        MaskLsi32kR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Статус частоты внешнего осциллятора 32 кГц OSC32K"]
    #[inline(always)]
    pub fn mask_osc32k(&self) -> MaskOsc32kR {
        MaskOsc32kR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Статус частоты HSI32M"]
    #[inline(always)]
    pub fn mask_hsi32m(&self) -> MaskHsi32mR {
        MaskHsi32mR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Статус частоты внешнего осциллятора 32 МГц OSC32M"]
    #[inline(always)]
    pub fn mask_osc32m(&self) -> MaskOsc32mR {
        MaskOsc32mR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {}
#[doc = "Статус монитора частоты\n\nYou can [`read`](crate::Reg::read) this register and get [`freq_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freq_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FreqStatusSpec;
impl crate::RegisterSpec for FreqStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`freq_status::R`](R) reader structure"]
impl crate::Readable for FreqStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`freq_status::W`](W) writer structure"]
impl crate::Writable for FreqStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FREQ_STATUS to value 0"]
impl crate::Resettable for FreqStatusSpec {
    const RESET_VALUE: u32 = 0;
}
