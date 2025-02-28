#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `CMPMCF` writer - Запись «1» в этот бит снимает флаг CMPM в регистре LPT_ISR"]
pub type CmpmcfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ARRMCF` writer - Запись «1» в этот бит снимает флаг ARRM в регистре LPT_ISR"]
pub type ArrmcfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EXTTRIGCF` writer - Запись «1» в этот бит снимает флаг EXTTRIG в регистре LPT_ISR"]
pub type ExttrigcfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ARRROCF` writer - Запись «1» в этот бит снимает флаг ARROK в регистре LPT_ISR"]
pub type ArrrocfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `UPCF` writer - Запись «1» в этот бит снимает флаг UP в регистре LPT_ISR"]
pub type UpcfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DOWNCF` writer - Запись «1» в этот бит снимает флаг DOWN в регистре LPT_ISR"]
pub type DowncfW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "Bit 0 - Запись «1» в этот бит снимает флаг CMPM в регистре LPT_ISR"]
    #[inline(always)]
    pub fn cmpmcf(&mut self) -> CmpmcfW<IcrSpec> {
        CmpmcfW::new(self, 0)
    }
    #[doc = "Bit 1 - Запись «1» в этот бит снимает флаг ARRM в регистре LPT_ISR"]
    #[inline(always)]
    pub fn arrmcf(&mut self) -> ArrmcfW<IcrSpec> {
        ArrmcfW::new(self, 1)
    }
    #[doc = "Bit 2 - Запись «1» в этот бит снимает флаг EXTTRIG в регистре LPT_ISR"]
    #[inline(always)]
    pub fn exttrigcf(&mut self) -> ExttrigcfW<IcrSpec> {
        ExttrigcfW::new(self, 2)
    }
    #[doc = "Bit 4 - Запись «1» в этот бит снимает флаг ARROK в регистре LPT_ISR"]
    #[inline(always)]
    pub fn arrrocf(&mut self) -> ArrrocfW<IcrSpec> {
        ArrrocfW::new(self, 4)
    }
    #[doc = "Bit 5 - Запись «1» в этот бит снимает флаг UP в регистре LPT_ISR"]
    #[inline(always)]
    pub fn upcf(&mut self) -> UpcfW<IcrSpec> {
        UpcfW::new(self, 5)
    }
    #[doc = "Bit 6 - Запись «1» в этот бит снимает флаг DOWN в регистре LPT_ISR"]
    #[inline(always)]
    pub fn downcf(&mut self) -> DowncfW<IcrSpec> {
        DowncfW::new(self, 6)
    }
}
#[doc = "Регистр сброса флагов прерываний\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x77;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {
    const RESET_VALUE: u32 = 0;
}
