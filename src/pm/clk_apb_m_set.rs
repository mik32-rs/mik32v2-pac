#[doc = "Register `CLK_APB_M_SET` reader"]
pub type R = crate::R<ClkApbMSetSpec>;
#[doc = "Register `CLK_APB_M_SET` writer"]
pub type W = crate::W<ClkApbMSetSpec>;
#[doc = "Блок управления питанием (PM - Power Monitor)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pm {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<Pm> for bool {
    #[inline(always)]
    fn from(variant: Pm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PM` reader - Блок управления питанием (PM - Power Monitor)"]
pub type PmR = crate::BitReader<Pm>;
impl PmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pm {
        match self.bits {
            false => Pm::Disable,
            true => Pm::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pm::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pm::Enable
    }
}
#[doc = "Field `PM` writer - Блок управления питанием (PM - Power Monitor)"]
pub type PmW<'a, REG> = crate::BitWriter<'a, REG, Pm>;
impl<'a, REG> PmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pm::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pm::Enable)
    }
}
#[doc = "Контроллер прерываний\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Epic {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<Epic> for bool {
    #[inline(always)]
    fn from(variant: Epic) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPIC` reader - Контроллер прерываний"]
pub type EpicR = crate::BitReader<Epic>;
impl EpicR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Epic {
        match self.bits {
            false => Epic::Disable,
            true => Epic::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Epic::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Epic::Enable
    }
}
#[doc = "Field `EPIC` writer - Контроллер прерываний"]
pub type EpicW<'a, REG> = crate::BitWriter<'a, REG, Epic>;
impl<'a, REG> EpicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Epic::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Epic::Enable)
    }
}
#[doc = "Таймер32_0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer32_0 {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<Timer32_0> for bool {
    #[inline(always)]
    fn from(variant: Timer32_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Timer32_0` reader - Таймер32_0"]
pub type Timer32_0R = crate::BitReader<Timer32_0>;
impl Timer32_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer32_0 {
        match self.bits {
            false => Timer32_0::Disable,
            true => Timer32_0::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Timer32_0::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Timer32_0::Enable
    }
}
#[doc = "Field `Timer32_0` writer - Таймер32_0"]
pub type Timer32_0W<'a, REG> = crate::BitWriter<'a, REG, Timer32_0>;
impl<'a, REG> Timer32_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer32_0::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer32_0::Enable)
    }
}
#[doc = "Контроллер выводов\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PadConfig {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<PadConfig> for bool {
    #[inline(always)]
    fn from(variant: PadConfig) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Pad_config` reader - Контроллер выводов"]
pub type PadConfigR = crate::BitReader<PadConfig>;
impl PadConfigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PadConfig {
        match self.bits {
            false => PadConfig::Disable,
            true => PadConfig::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PadConfig::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PadConfig::Enable
    }
}
#[doc = "Field `Pad_config` writer - Контроллер выводов"]
pub type PadConfigW<'a, REG> = crate::BitWriter<'a, REG, PadConfig>;
impl<'a, REG> PadConfigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PadConfig::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PadConfig::Enable)
    }
}
#[doc = "Сторожевой таймер шины\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WdtBus {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<WdtBus> for bool {
    #[inline(always)]
    fn from(variant: WdtBus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT_BUS` reader - Сторожевой таймер шины"]
pub type WdtBusR = crate::BitReader<WdtBus>;
impl WdtBusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WdtBus {
        match self.bits {
            false => WdtBus::Disable,
            true => WdtBus::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WdtBus::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WdtBus::Enable
    }
}
#[doc = "Field `WDT_BUS` writer - Сторожевой таймер шины"]
pub type WdtBusW<'a, REG> = crate::BitWriter<'a, REG, WdtBus>;
impl<'a, REG> WdtBusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WdtBus::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WdtBus::Enable)
    }
}
#[doc = "OTP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Otp {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<Otp> for bool {
    #[inline(always)]
    fn from(variant: Otp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTP` reader - OTP"]
pub type OtpR = crate::BitReader<Otp>;
impl OtpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Otp {
        match self.bits {
            false => Otp::Disable,
            true => Otp::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Otp::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Otp::Enable
    }
}
#[doc = "Field `OTP` writer - OTP"]
pub type OtpW<'a, REG> = crate::BitWriter<'a, REG, Otp>;
impl<'a, REG> OtpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Otp::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Otp::Enable)
    }
}
#[doc = "Монитор питания системного домена\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pvd {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<Pvd> for bool {
    #[inline(always)]
    fn from(variant: Pvd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVD` reader - Монитор питания системного домена"]
pub type PvdR = crate::BitReader<Pvd>;
impl PvdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pvd {
        match self.bits {
            false => Pvd::Disable,
            true => Pvd::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pvd::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pvd::Enable
    }
}
#[doc = "Field `PVD` writer - Монитор питания системного домена"]
pub type PvdW<'a, REG> = crate::BitWriter<'a, REG, Pvd>;
impl<'a, REG> PvdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pvd::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pvd::Enable)
    }
}
#[doc = "Блок WakeUp (WU) батарейного домена\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wu {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<Wu> for bool {
    #[inline(always)]
    fn from(variant: Wu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WU` reader - Блок WakeUp (WU) батарейного домена"]
pub type WuR = crate::BitReader<Wu>;
impl WuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wu {
        match self.bits {
            false => Wu::Disable,
            true => Wu::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wu::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wu::Enable
    }
}
#[doc = "Field `WU` writer - Блок WakeUp (WU) батарейного домена"]
pub type WuW<'a, REG> = crate::BitWriter<'a, REG, Wu>;
impl<'a, REG> WuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wu::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wu::Enable)
    }
}
#[doc = "RTC батарейного домена\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtc {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<Rtc> for bool {
    #[inline(always)]
    fn from(variant: Rtc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC` reader - RTC батарейного домена"]
pub type RtcR = crate::BitReader<Rtc>;
impl RtcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtc {
        match self.bits {
            false => Rtc::Disable,
            true => Rtc::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rtc::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rtc::Enable
    }
}
#[doc = "Field `RTC` writer - RTC батарейного домена"]
pub type RtcW<'a, REG> = crate::BitWriter<'a, REG, Rtc>;
impl<'a, REG> RtcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Блок управления питанием (PM - Power Monitor)"]
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Контроллер прерываний"]
    #[inline(always)]
    pub fn epic(&self) -> EpicR {
        EpicR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Таймер32_0"]
    #[inline(always)]
    pub fn timer32_0(&self) -> Timer32_0R {
        Timer32_0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Контроллер выводов"]
    #[inline(always)]
    pub fn pad_config(&self) -> PadConfigR {
        PadConfigR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Сторожевой таймер шины"]
    #[inline(always)]
    pub fn wdt_bus(&self) -> WdtBusR {
        WdtBusR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OTP"]
    #[inline(always)]
    pub fn otp(&self) -> OtpR {
        OtpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Монитор питания системного домена"]
    #[inline(always)]
    pub fn pvd(&self) -> PvdR {
        PvdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Блок WakeUp (WU) батарейного домена"]
    #[inline(always)]
    pub fn wu(&self) -> WuR {
        WuR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTC батарейного домена"]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Блок управления питанием (PM - Power Monitor)"]
    #[inline(always)]
    pub fn pm(&mut self) -> PmW<ClkApbMSetSpec> {
        PmW::new(self, 0)
    }
    #[doc = "Bit 1 - Контроллер прерываний"]
    #[inline(always)]
    pub fn epic(&mut self) -> EpicW<ClkApbMSetSpec> {
        EpicW::new(self, 1)
    }
    #[doc = "Bit 2 - Таймер32_0"]
    #[inline(always)]
    pub fn timer32_0(&mut self) -> Timer32_0W<ClkApbMSetSpec> {
        Timer32_0W::new(self, 2)
    }
    #[doc = "Bit 3 - Контроллер выводов"]
    #[inline(always)]
    pub fn pad_config(&mut self) -> PadConfigW<ClkApbMSetSpec> {
        PadConfigW::new(self, 3)
    }
    #[doc = "Bit 4 - Сторожевой таймер шины"]
    #[inline(always)]
    pub fn wdt_bus(&mut self) -> WdtBusW<ClkApbMSetSpec> {
        WdtBusW::new(self, 4)
    }
    #[doc = "Bit 5 - OTP"]
    #[inline(always)]
    pub fn otp(&mut self) -> OtpW<ClkApbMSetSpec> {
        OtpW::new(self, 5)
    }
    #[doc = "Bit 6 - Монитор питания системного домена"]
    #[inline(always)]
    pub fn pvd(&mut self) -> PvdW<ClkApbMSetSpec> {
        PvdW::new(self, 6)
    }
    #[doc = "Bit 7 - Блок WakeUp (WU) батарейного домена"]
    #[inline(always)]
    pub fn wu(&mut self) -> WuW<ClkApbMSetSpec> {
        WuW::new(self, 7)
    }
    #[doc = "Bit 8 - RTC батарейного домена"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RtcW<ClkApbMSetSpec> {
        RtcW::new(self, 8)
    }
}
#[doc = "Регистр включения тактированием устройств на шине APB_M. Каждому биту соответствует устройство\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_apb_m_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_apb_m_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkApbMSetSpec;
impl crate::RegisterSpec for ClkApbMSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_apb_m_set::R`](R) reader structure"]
impl crate::Readable for ClkApbMSetSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_apb_m_set::W`](W) writer structure"]
impl crate::Writable for ClkApbMSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_APB_M_SET to value 0x89"]
impl crate::Resettable for ClkApbMSetSpec {
    const RESET_VALUE: u32 = 0x89;
}
