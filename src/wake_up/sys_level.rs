#[doc = "Register `SYS_LEVEL` reader"]
pub type R = crate::R<SysLevelSpec>;
#[doc = "Register `SYS_LEVEL` writer"]
pub type W = crate::W<SysLevelSpec>;
#[doc = "Активный уровень срабатывания будильника RTC\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LvlRtc {
    #[doc = "0: Активный уровень \"0\""]
    Low = 0,
    #[doc = "1: Активный уровень \"1\""]
    High = 1,
}
impl From<LvlRtc> for bool {
    #[inline(always)]
    fn from(variant: LvlRtc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVL_RTC` reader - Активный уровень срабатывания будильника RTC"]
pub type LvlRtcR = crate::BitReader<LvlRtc>;
impl LvlRtcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LvlRtc {
        match self.bits {
            false => LvlRtc::Low,
            true => LvlRtc::High,
        }
    }
    #[doc = "Активный уровень \"0\""]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == LvlRtc::Low
    }
    #[doc = "Активный уровень \"1\""]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == LvlRtc::High
    }
}
#[doc = "Field `LVL_RTC` writer - Активный уровень срабатывания будильника RTC"]
pub type LvlRtcW<'a, REG> = crate::BitWriter<'a, REG, LvlRtc>;
impl<'a, REG> LvlRtcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Активный уровень \"0\""]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(LvlRtc::Low)
    }
    #[doc = "Активный уровень \"1\""]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(LvlRtc::High)
    }
}
#[doc = "Активный уровень внешнего вывода ext_wu\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LvlWu {
    #[doc = "0: Активный уровень \"0\""]
    Low = 0,
    #[doc = "1: Активный уровень \"1\""]
    High = 1,
}
impl From<LvlWu> for bool {
    #[inline(always)]
    fn from(variant: LvlWu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVL_WU` reader - Активный уровень внешнего вывода ext_wu"]
pub type LvlWuR = crate::BitReader<LvlWu>;
impl LvlWuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LvlWu {
        match self.bits {
            false => LvlWu::Low,
            true => LvlWu::High,
        }
    }
    #[doc = "Активный уровень \"0\""]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == LvlWu::Low
    }
    #[doc = "Активный уровень \"1\""]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == LvlWu::High
    }
}
#[doc = "Field `LVL_WU` writer - Активный уровень внешнего вывода ext_wu"]
pub type LvlWuW<'a, REG> = crate::BitWriter<'a, REG, LvlWu>;
impl<'a, REG> LvlWuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Активный уровень \"0\""]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(LvlWu::Low)
    }
    #[doc = "Активный уровень \"1\""]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(LvlWu::High)
    }
}
impl R {
    #[doc = "Bit 0 - Активный уровень срабатывания будильника RTC"]
    #[inline(always)]
    pub fn lvl_rtc(&self) -> LvlRtcR {
        LvlRtcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Активный уровень внешнего вывода ext_wu"]
    #[inline(always)]
    pub fn lvl_wu(&self) -> LvlWuR {
        LvlWuR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Активный уровень срабатывания будильника RTC"]
    #[inline(always)]
    pub fn lvl_rtc(&mut self) -> LvlRtcW<SysLevelSpec> {
        LvlRtcW::new(self, 0)
    }
    #[doc = "Bit 1 - Активный уровень внешнего вывода ext_wu"]
    #[inline(always)]
    pub fn lvl_wu(&mut self) -> LvlWuW<SysLevelSpec> {
        LvlWuW::new(self, 1)
    }
}
#[doc = "Активные уровни событий для включения и сброса системного домена\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_level::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_level::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysLevelSpec;
impl crate::RegisterSpec for SysLevelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_level::R`](R) reader structure"]
impl crate::Readable for SysLevelSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_level::W`](W) writer structure"]
impl crate::Writable for SysLevelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYS_LEVEL to value 0x03"]
impl crate::Resettable for SysLevelSpec {
    const RESET_VALUE: u32 = 0x03;
}
