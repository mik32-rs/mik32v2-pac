#[doc = "Register `DIVIDER` reader"]
pub type R = crate::R<DividerSpec>;
#[doc = "Register `DIVIDER` writer"]
pub type W = crate::W<DividerSpec>;
#[doc = "Field `BRR` reader - Значение делителя входного тактового сигнала (clk_in). При BRR < 16 USART не активен Бодрейт рассчитывается по следующей формуле: BR = F/BRR, где BR - бодрейт, F - значение частоты входного тактового сигнала Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type BrrR = crate::FieldReader<u16>;
#[doc = "Field `BRR` writer - Значение делителя входного тактового сигнала (clk_in). При BRR < 16 USART не активен Бодрейт рассчитывается по следующей формуле: BR = F/BRR, где BR - бодрейт, F - значение частоты входного тактового сигнала Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type BrrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Значение делителя входного тактового сигнала (clk_in). При BRR < 16 USART не активен Бодрейт рассчитывается по следующей формуле: BR = F/BRR, где BR - бодрейт, F - значение частоты входного тактового сигнала Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn brr(&self) -> BrrR {
        BrrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Значение делителя входного тактового сигнала (clk_in). При BRR < 16 USART не активен Бодрейт рассчитывается по следующей формуле: BR = F/BRR, где BR - бодрейт, F - значение частоты входного тактового сигнала Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn brr(&mut self) -> BrrW<DividerSpec> {
        BrrW::new(self, 0)
    }
}
#[doc = "Регистр настройки делителя\n\nYou can [`read`](crate::Reg::read) this register and get [`divider::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divider::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DividerSpec;
impl crate::RegisterSpec for DividerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divider::R`](R) reader structure"]
impl crate::Readable for DividerSpec {}
#[doc = "`write(|w| ..)` method takes [`divider::W`](W) writer structure"]
impl crate::Writable for DividerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIVIDER to value 0"]
impl crate::Resettable for DividerSpec {
    const RESET_VALUE: u32 = 0;
}
