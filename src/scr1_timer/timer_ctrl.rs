#[doc = "Register `TIMER_CTRL` reader"]
pub type R = crate::R<TimerCtrlSpec>;
#[doc = "Register `TIMER_CTRL` writer"]
pub type W = crate::W<TimerCtrlSpec>;
#[doc = "Включение таймера\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Таймер выключен"]
    Diasable = 0,
    #[doc = "1: Таймер включен"]
    Enable = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Включение таймера"]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Diasable,
            true => Enable::Enable,
        }
    }
    #[doc = "Таймер выключен"]
    #[inline(always)]
    pub fn is_diasable(&self) -> bool {
        *self == Enable::Diasable
    }
    #[doc = "Таймер включен"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enable::Enable
    }
}
#[doc = "Field `ENABLE` writer - Включение таймера"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Таймер выключен"]
    #[inline(always)]
    pub fn diasable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Diasable)
    }
    #[doc = "Таймер включен"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enable)
    }
}
#[doc = "Источник тактрования системного таймера\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clksrc {
    #[doc = "0: Тактирование ядра (частота шины AHB hclk)"]
    Hclk = 0,
    #[doc = "1: Внешний RTC (источник выбранный в PM.CPU_RTC_CLK_MUX)"]
    ExternalClk = 1,
}
impl From<Clksrc> for bool {
    #[inline(always)]
    fn from(variant: Clksrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSRC` reader - Источник тактрования системного таймера"]
pub type ClksrcR = crate::BitReader<Clksrc>;
impl ClksrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clksrc {
        match self.bits {
            false => Clksrc::Hclk,
            true => Clksrc::ExternalClk,
        }
    }
    #[doc = "Тактирование ядра (частота шины AHB hclk)"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == Clksrc::Hclk
    }
    #[doc = "Внешний RTC (источник выбранный в PM.CPU_RTC_CLK_MUX)"]
    #[inline(always)]
    pub fn is_external_clk(&self) -> bool {
        *self == Clksrc::ExternalClk
    }
}
#[doc = "Field `CLKSRC` writer - Источник тактрования системного таймера"]
pub type ClksrcW<'a, REG> = crate::BitWriter<'a, REG, Clksrc>;
impl<'a, REG> ClksrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование ядра (частота шины AHB hclk)"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksrc::Hclk)
    }
    #[doc = "Внешний RTC (источник выбранный в PM.CPU_RTC_CLK_MUX)"]
    #[inline(always)]
    pub fn external_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksrc::ExternalClk)
    }
}
impl R {
    #[doc = "Bit 0 - Включение таймера"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Источник тактрования системного таймера"]
    #[inline(always)]
    pub fn clksrc(&self) -> ClksrcR {
        ClksrcR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Включение таймера"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<TimerCtrlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Источник тактрования системного таймера"]
    #[inline(always)]
    pub fn clksrc(&mut self) -> ClksrcW<TimerCtrlSpec> {
        ClksrcW::new(self, 1)
    }
}
#[doc = "Регистр конфигурации\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimerCtrlSpec;
impl crate::RegisterSpec for TimerCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_ctrl::R`](R) reader structure"]
impl crate::Readable for TimerCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`timer_ctrl::W`](W) writer structure"]
impl crate::Writable for TimerCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER_CTRL to value 0x01"]
impl crate::Resettable for TimerCtrlSpec {
    const RESET_VALUE: u32 = 0x01;
}
