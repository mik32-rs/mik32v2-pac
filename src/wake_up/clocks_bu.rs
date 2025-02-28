#[doc = "Register `CLOCKS_BU` reader"]
pub type R = crate::R<ClocksBuSpec>;
#[doc = "Register `CLOCKS_BU` writer"]
pub type W = crate::W<ClocksBuSpec>;
#[doc = "Включение/отключение внешнего осциллятора на 32 KГц\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Osc32kEn {
    #[doc = "0: Включение OSC32K"]
    Enable = 0,
    #[doc = "1: Отключение OSC32K"]
    Disable = 1,
}
impl From<Osc32kEn> for bool {
    #[inline(always)]
    fn from(variant: Osc32kEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSC32K_EN` reader - Включение/отключение внешнего осциллятора на 32 KГц"]
pub type Osc32kEnR = crate::BitReader<Osc32kEn>;
impl Osc32kEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Osc32kEn {
        match self.bits {
            false => Osc32kEn::Enable,
            true => Osc32kEn::Disable,
        }
    }
    #[doc = "Включение OSC32K"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Osc32kEn::Enable
    }
    #[doc = "Отключение OSC32K"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Osc32kEn::Disable
    }
}
#[doc = "Field `OSC32K_EN` writer - Включение/отключение внешнего осциллятора на 32 KГц"]
pub type Osc32kEnW<'a, REG> = crate::BitWriter<'a, REG, Osc32kEn>;
impl<'a, REG> Osc32kEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Включение OSC32K"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Osc32kEn::Enable)
    }
    #[doc = "Отключение OSC32K"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Osc32kEn::Disable)
    }
}
#[doc = "Включение/отключение LSI32К\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsi32kEn {
    #[doc = "0: Включение LSI32К"]
    Enable = 0,
    #[doc = "1: Отключение LSI32К"]
    Disable = 1,
}
impl From<Lsi32kEn> for bool {
    #[inline(always)]
    fn from(variant: Lsi32kEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSI32K_EN` reader - Включение/отключение LSI32К"]
pub type Lsi32kEnR = crate::BitReader<Lsi32kEn>;
impl Lsi32kEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsi32kEn {
        match self.bits {
            false => Lsi32kEn::Enable,
            true => Lsi32kEn::Disable,
        }
    }
    #[doc = "Включение LSI32К"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Lsi32kEn::Enable
    }
    #[doc = "Отключение LSI32К"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Lsi32kEn::Disable
    }
}
#[doc = "Field `LSI32K_EN` writer - Включение/отключение LSI32К"]
pub type Lsi32kEnW<'a, REG> = crate::BitWriter<'a, REG, Lsi32kEn>;
impl<'a, REG> Lsi32kEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Включение LSI32К"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lsi32kEn::Enable)
    }
    #[doc = "Отключение LSI32К"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Lsi32kEn::Disable)
    }
}
#[doc = "Field `ADJ_LSI32K` reader - Поправочные коэффициенты LSI32К"]
pub type AdjLsi32kR = crate::FieldReader;
#[doc = "Field `ADJ_LSI32K` writer - Поправочные коэффициенты LSI32К"]
pub type AdjLsi32kW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Выбор приоритетного источника тактирования часов реального времени: 0x0 – автоматический выбор. При наличии обоих источников 32кГц выбирается внутренний LSI32K\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RtcClkMux {
    #[doc = "0: Автоматический выбор. При наличии обоих источников 32кГц выбирается внутренний LSI32K"]
    Automatic = 0,
    #[doc = "1: Внутренний LSI32K"]
    Lsi32k = 1,
    #[doc = "2: Внешний осциллятор OSC32K"]
    Osc32k = 2,
}
impl From<RtcClkMux> for u8 {
    #[inline(always)]
    fn from(variant: RtcClkMux) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RtcClkMux {
    type Ux = u8;
}
impl crate::IsEnum for RtcClkMux {}
#[doc = "Field `RTC_CLK_MUX` reader - Выбор приоритетного источника тактирования часов реального времени: 0x0 – автоматический выбор. При наличии обоих источников 32кГц выбирается внутренний LSI32K"]
pub type RtcClkMuxR = crate::FieldReader<RtcClkMux>;
impl RtcClkMuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RtcClkMux> {
        match self.bits {
            0 => Some(RtcClkMux::Automatic),
            1 => Some(RtcClkMux::Lsi32k),
            2 => Some(RtcClkMux::Osc32k),
            _ => None,
        }
    }
    #[doc = "Автоматический выбор. При наличии обоих источников 32кГц выбирается внутренний LSI32K"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == RtcClkMux::Automatic
    }
    #[doc = "Внутренний LSI32K"]
    #[inline(always)]
    pub fn is_lsi32k(&self) -> bool {
        *self == RtcClkMux::Lsi32k
    }
    #[doc = "Внешний осциллятор OSC32K"]
    #[inline(always)]
    pub fn is_osc32k(&self) -> bool {
        *self == RtcClkMux::Osc32k
    }
}
#[doc = "Field `RTC_CLK_MUX` writer - Выбор приоритетного источника тактирования часов реального времени: 0x0 – автоматический выбор. При наличии обоих источников 32кГц выбирается внутренний LSI32K"]
pub type RtcClkMuxW<'a, REG> = crate::FieldWriter<'a, REG, 2, RtcClkMux>;
impl<'a, REG> RtcClkMuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Автоматический выбор. При наличии обоих источников 32кГц выбирается внутренний LSI32K"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut crate::W<REG> {
        self.variant(RtcClkMux::Automatic)
    }
    #[doc = "Внутренний LSI32K"]
    #[inline(always)]
    pub fn lsi32k(self) -> &'a mut crate::W<REG> {
        self.variant(RtcClkMux::Lsi32k)
    }
    #[doc = "Внешний осциллятор OSC32K"]
    #[inline(always)]
    pub fn osc32k(self) -> &'a mut crate::W<REG> {
        self.variant(RtcClkMux::Osc32k)
    }
}
#[doc = "Field `OSC32K_SM` reader - Режим повышенного потребления, активный уровень “0” для OSC32K"]
pub type Osc32kSmR = crate::BitReader;
#[doc = "Field `OSC32K_SM` writer - Режим повышенного потребления, активный уровень “0” для OSC32K"]
pub type Osc32kSmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Включение/отключение внешнего осциллятора на 32 KГц"]
    #[inline(always)]
    pub fn osc32k_en(&self) -> Osc32kEnR {
        Osc32kEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Включение/отключение LSI32К"]
    #[inline(always)]
    pub fn lsi32k_en(&self) -> Lsi32kEnR {
        Lsi32kEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 6:9 - Поправочные коэффициенты LSI32К"]
    #[inline(always)]
    pub fn adj_lsi32k(&self) -> AdjLsi32kR {
        AdjLsi32kR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - Выбор приоритетного источника тактирования часов реального времени: 0x0 – автоматический выбор. При наличии обоих источников 32кГц выбирается внутренний LSI32K"]
    #[inline(always)]
    pub fn rtc_clk_mux(&self) -> RtcClkMuxR {
        RtcClkMuxR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 14 - Режим повышенного потребления, активный уровень “0” для OSC32K"]
    #[inline(always)]
    pub fn osc32k_sm(&self) -> Osc32kSmR {
        Osc32kSmR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Включение/отключение внешнего осциллятора на 32 KГц"]
    #[inline(always)]
    pub fn osc32k_en(&mut self) -> Osc32kEnW<ClocksBuSpec> {
        Osc32kEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Включение/отключение LSI32К"]
    #[inline(always)]
    pub fn lsi32k_en(&mut self) -> Lsi32kEnW<ClocksBuSpec> {
        Lsi32kEnW::new(self, 1)
    }
    #[doc = "Bits 6:9 - Поправочные коэффициенты LSI32К"]
    #[inline(always)]
    pub fn adj_lsi32k(&mut self) -> AdjLsi32kW<ClocksBuSpec> {
        AdjLsi32kW::new(self, 6)
    }
    #[doc = "Bits 10:11 - Выбор приоритетного источника тактирования часов реального времени: 0x0 – автоматический выбор. При наличии обоих источников 32кГц выбирается внутренний LSI32K"]
    #[inline(always)]
    pub fn rtc_clk_mux(&mut self) -> RtcClkMuxW<ClocksBuSpec> {
        RtcClkMuxW::new(self, 10)
    }
    #[doc = "Bit 14 - Режим повышенного потребления, активный уровень “0” для OSC32K"]
    #[inline(always)]
    pub fn osc32k_sm(&mut self) -> Osc32kSmW<ClocksBuSpec> {
        Osc32kSmW::new(self, 14)
    }
}
#[doc = "Регистр управления тактированием батарейного домена\n\nYou can [`read`](crate::Reg::read) this register and get [`clocks_bu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clocks_bu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClocksBuSpec;
impl crate::RegisterSpec for ClocksBuSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clocks_bu::R`](R) reader structure"]
impl crate::Readable for ClocksBuSpec {}
#[doc = "`write(|w| ..)` method takes [`clocks_bu::W`](W) writer structure"]
impl crate::Writable for ClocksBuSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLOCKS_BU to value 0x0200"]
impl crate::Resettable for ClocksBuSpec {
    const RESET_VALUE: u32 = 0x0200;
}
