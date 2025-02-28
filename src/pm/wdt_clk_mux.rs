#[doc = "Register `WDT_CLK_MUX` reader"]
pub type R = crate::R<WdtClkMuxSpec>;
#[doc = "Register `WDT_CLK_MUX` writer"]
pub type W = crate::W<WdtClkMuxSpec>;
#[doc = "Выбор источника тактирования сторожевого таймера\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WdtClkMux {
    #[doc = "0: Внешний OSC32M"]
    Osc32m = 0,
    #[doc = "1: Внутренний HSI32M"]
    Hsi32m = 1,
    #[doc = "2: Внешний OSC32K"]
    Osc32k = 2,
    #[doc = "3: Внутренний LSI32К"]
    Lsi32k = 3,
}
impl From<WdtClkMux> for u8 {
    #[inline(always)]
    fn from(variant: WdtClkMux) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WdtClkMux {
    type Ux = u8;
}
impl crate::IsEnum for WdtClkMux {}
#[doc = "Field `WDT_CLK_MUX` reader - Выбор источника тактирования сторожевого таймера"]
pub type WdtClkMuxR = crate::FieldReader<WdtClkMux>;
impl WdtClkMuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WdtClkMux {
        match self.bits {
            0 => WdtClkMux::Osc32m,
            1 => WdtClkMux::Hsi32m,
            2 => WdtClkMux::Osc32k,
            3 => WdtClkMux::Lsi32k,
            _ => unreachable!(),
        }
    }
    #[doc = "Внешний OSC32M"]
    #[inline(always)]
    pub fn is_osc32m(&self) -> bool {
        *self == WdtClkMux::Osc32m
    }
    #[doc = "Внутренний HSI32M"]
    #[inline(always)]
    pub fn is_hsi32m(&self) -> bool {
        *self == WdtClkMux::Hsi32m
    }
    #[doc = "Внешний OSC32K"]
    #[inline(always)]
    pub fn is_osc32k(&self) -> bool {
        *self == WdtClkMux::Osc32k
    }
    #[doc = "Внутренний LSI32К"]
    #[inline(always)]
    pub fn is_lsi32k(&self) -> bool {
        *self == WdtClkMux::Lsi32k
    }
}
#[doc = "Field `WDT_CLK_MUX` writer - Выбор источника тактирования сторожевого таймера"]
pub type WdtClkMuxW<'a, REG> = crate::FieldWriter<'a, REG, 2, WdtClkMux, crate::Safe>;
impl<'a, REG> WdtClkMuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Внешний OSC32M"]
    #[inline(always)]
    pub fn osc32m(self) -> &'a mut crate::W<REG> {
        self.variant(WdtClkMux::Osc32m)
    }
    #[doc = "Внутренний HSI32M"]
    #[inline(always)]
    pub fn hsi32m(self) -> &'a mut crate::W<REG> {
        self.variant(WdtClkMux::Hsi32m)
    }
    #[doc = "Внешний OSC32K"]
    #[inline(always)]
    pub fn osc32k(self) -> &'a mut crate::W<REG> {
        self.variant(WdtClkMux::Osc32k)
    }
    #[doc = "Внутренний LSI32К"]
    #[inline(always)]
    pub fn lsi32k(self) -> &'a mut crate::W<REG> {
        self.variant(WdtClkMux::Lsi32k)
    }
}
impl R {
    #[doc = "Bits 0:1 - Выбор источника тактирования сторожевого таймера"]
    #[inline(always)]
    pub fn wdt_clk_mux(&self) -> WdtClkMuxR {
        WdtClkMuxR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Выбор источника тактирования сторожевого таймера"]
    #[inline(always)]
    pub fn wdt_clk_mux(&mut self) -> WdtClkMuxW<WdtClkMuxSpec> {
        WdtClkMuxW::new(self, 0)
    }
}
#[doc = "Выбор источника тактирования сторожевого таймера: 0 – внешний OSC32M; 1 – внутренний HSI32M; 2 – внешний OSC32K; 3 – внутренний LSI32К;\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt_clk_mux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_clk_mux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtClkMuxSpec;
impl crate::RegisterSpec for WdtClkMuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt_clk_mux::R`](R) reader structure"]
impl crate::Readable for WdtClkMuxSpec {}
#[doc = "`write(|w| ..)` method takes [`wdt_clk_mux::W`](W) writer structure"]
impl crate::Writable for WdtClkMuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDT_CLK_MUX to value 0"]
impl crate::Resettable for WdtClkMuxSpec {
    const RESET_VALUE: u32 = 0;
}
