#[doc = "Register `CPU_RTC_CLK_MUX` reader"]
pub type R = crate::R<CpuRtcClkMuxSpec>;
#[doc = "Register `CPU_RTC_CLK_MUX` writer"]
pub type W = crate::W<CpuRtcClkMuxSpec>;
#[doc = "Выбор источника тактирования сторожевого таймера\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CpuRtcClkMux {
    #[doc = "0: Внешний OSC32K"]
    Osc32k = 0,
    #[doc = "1: Внутренний LSI32К"]
    Lsi32k = 1,
}
impl From<CpuRtcClkMux> for bool {
    #[inline(always)]
    fn from(variant: CpuRtcClkMux) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU_RTC_CLK_MUX` reader - Выбор источника тактирования сторожевого таймера"]
pub type CpuRtcClkMuxR = crate::BitReader<CpuRtcClkMux>;
impl CpuRtcClkMuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CpuRtcClkMux {
        match self.bits {
            false => CpuRtcClkMux::Osc32k,
            true => CpuRtcClkMux::Lsi32k,
        }
    }
    #[doc = "Внешний OSC32K"]
    #[inline(always)]
    pub fn is_osc32k(&self) -> bool {
        *self == CpuRtcClkMux::Osc32k
    }
    #[doc = "Внутренний LSI32К"]
    #[inline(always)]
    pub fn is_lsi32k(&self) -> bool {
        *self == CpuRtcClkMux::Lsi32k
    }
}
#[doc = "Field `CPU_RTC_CLK_MUX` writer - Выбор источника тактирования сторожевого таймера"]
pub type CpuRtcClkMuxW<'a, REG> = crate::BitWriter<'a, REG, CpuRtcClkMux>;
impl<'a, REG> CpuRtcClkMuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Внешний OSC32K"]
    #[inline(always)]
    pub fn osc32k(self) -> &'a mut crate::W<REG> {
        self.variant(CpuRtcClkMux::Osc32k)
    }
    #[doc = "Внутренний LSI32К"]
    #[inline(always)]
    pub fn lsi32k(self) -> &'a mut crate::W<REG> {
        self.variant(CpuRtcClkMux::Lsi32k)
    }
}
impl R {
    #[doc = "Bit 0 - Выбор источника тактирования сторожевого таймера"]
    #[inline(always)]
    pub fn cpu_rtc_clk_mux(&self) -> CpuRtcClkMuxR {
        CpuRtcClkMuxR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Выбор источника тактирования сторожевого таймера"]
    #[inline(always)]
    pub fn cpu_rtc_clk_mux(&mut self) -> CpuRtcClkMuxW<CpuRtcClkMuxSpec> {
        CpuRtcClkMuxW::new(self, 0)
    }
}
#[doc = "Выбор источника тактирования RTC для системного таймера в составе ядра\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_rtc_clk_mux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_rtc_clk_mux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuRtcClkMuxSpec;
impl crate::RegisterSpec for CpuRtcClkMuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_rtc_clk_mux::R`](R) reader structure"]
impl crate::Readable for CpuRtcClkMuxSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu_rtc_clk_mux::W`](W) writer structure"]
impl crate::Writable for CpuRtcClkMuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPU_RTC_CLK_MUX to value 0"]
impl crate::Resettable for CpuRtcClkMuxSpec {
    const RESET_VALUE: u32 = 0;
}
