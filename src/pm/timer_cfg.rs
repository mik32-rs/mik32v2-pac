#[doc = "Register `TIMER_CFG` reader"]
pub type R = crate::R<TimerCfgSpec>;
#[doc = "Register `TIMER_CFG` writer"]
pub type W = crate::W<TimerCfgSpec>;
#[doc = "Выбор синхронного тактового сигнала для входа TIM1 Timer32_0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MuxTim32_0Tim1 {
    #[doc = "0: Системная частота (sys_clk)"]
    SysClk = 0,
    #[doc = "1: Частота шины AHB (hclk)"]
    Hclk = 1,
}
impl From<MuxTim32_0Tim1> for bool {
    #[inline(always)]
    fn from(variant: MuxTim32_0Tim1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUX_TIM32_0_TIM1` reader - Выбор синхронного тактового сигнала для входа TIM1 Timer32_0"]
pub type MuxTim32_0Tim1R = crate::BitReader<MuxTim32_0Tim1>;
impl MuxTim32_0Tim1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MuxTim32_0Tim1 {
        match self.bits {
            false => MuxTim32_0Tim1::SysClk,
            true => MuxTim32_0Tim1::Hclk,
        }
    }
    #[doc = "Системная частота (sys_clk)"]
    #[inline(always)]
    pub fn is_sys_clk(&self) -> bool {
        *self == MuxTim32_0Tim1::SysClk
    }
    #[doc = "Частота шины AHB (hclk)"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == MuxTim32_0Tim1::Hclk
    }
}
#[doc = "Field `MUX_TIM32_0_TIM1` writer - Выбор синхронного тактового сигнала для входа TIM1 Timer32_0"]
pub type MuxTim32_0Tim1W<'a, REG> = crate::BitWriter<'a, REG, MuxTim32_0Tim1>;
impl<'a, REG> MuxTim32_0Tim1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Системная частота (sys_clk)"]
    #[inline(always)]
    pub fn sys_clk(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim32_0Tim1::SysClk)
    }
    #[doc = "Частота шины AHB (hclk)"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim32_0Tim1::Hclk)
    }
}
#[doc = "Выбор асинхронного тактового сигнала для входа TIM2 Timer32_0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MuxTim32_0Tim2 {
    #[doc = "0: OSC32K"]
    Osc32k = 0,
    #[doc = "1: LSI32K"]
    Lsi32k = 1,
}
impl From<MuxTim32_0Tim2> for bool {
    #[inline(always)]
    fn from(variant: MuxTim32_0Tim2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUX_TIM32_0_TIM2` reader - Выбор асинхронного тактового сигнала для входа TIM2 Timer32_0"]
pub type MuxTim32_0Tim2R = crate::BitReader<MuxTim32_0Tim2>;
impl MuxTim32_0Tim2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MuxTim32_0Tim2 {
        match self.bits {
            false => MuxTim32_0Tim2::Osc32k,
            true => MuxTim32_0Tim2::Lsi32k,
        }
    }
    #[doc = "OSC32K"]
    #[inline(always)]
    pub fn is_osc32k(&self) -> bool {
        *self == MuxTim32_0Tim2::Osc32k
    }
    #[doc = "LSI32K"]
    #[inline(always)]
    pub fn is_lsi32k(&self) -> bool {
        *self == MuxTim32_0Tim2::Lsi32k
    }
}
#[doc = "Field `MUX_TIM32_0_TIM2` writer - Выбор асинхронного тактового сигнала для входа TIM2 Timer32_0"]
pub type MuxTim32_0Tim2W<'a, REG> = crate::BitWriter<'a, REG, MuxTim32_0Tim2>;
impl<'a, REG> MuxTim32_0Tim2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OSC32K"]
    #[inline(always)]
    pub fn osc32k(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim32_0Tim2::Osc32k)
    }
    #[doc = "LSI32K"]
    #[inline(always)]
    pub fn lsi32k(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim32_0Tim2::Lsi32k)
    }
}
#[doc = "Выбор синхронного тактового сигнала для входа TIM1 Timer32_1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MuxTim32_1Tim1 {
    #[doc = "0: Системная частота (sys_clk)"]
    SysClk = 0,
    #[doc = "1: Частота шины AHB (hclk)"]
    Hclk = 1,
}
impl From<MuxTim32_1Tim1> for bool {
    #[inline(always)]
    fn from(variant: MuxTim32_1Tim1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUX_TIM32_1_TIM1` reader - Выбор синхронного тактового сигнала для входа TIM1 Timer32_1"]
pub type MuxTim32_1Tim1R = crate::BitReader<MuxTim32_1Tim1>;
impl MuxTim32_1Tim1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MuxTim32_1Tim1 {
        match self.bits {
            false => MuxTim32_1Tim1::SysClk,
            true => MuxTim32_1Tim1::Hclk,
        }
    }
    #[doc = "Системная частота (sys_clk)"]
    #[inline(always)]
    pub fn is_sys_clk(&self) -> bool {
        *self == MuxTim32_1Tim1::SysClk
    }
    #[doc = "Частота шины AHB (hclk)"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == MuxTim32_1Tim1::Hclk
    }
}
#[doc = "Field `MUX_TIM32_1_TIM1` writer - Выбор синхронного тактового сигнала для входа TIM1 Timer32_1"]
pub type MuxTim32_1Tim1W<'a, REG> = crate::BitWriter<'a, REG, MuxTim32_1Tim1>;
impl<'a, REG> MuxTim32_1Tim1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Системная частота (sys_clk)"]
    #[inline(always)]
    pub fn sys_clk(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim32_1Tim1::SysClk)
    }
    #[doc = "Частота шины AHB (hclk)"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim32_1Tim1::Hclk)
    }
}
#[doc = "Выбор асинхронного тактового сигнала для входа TIM2 Timer32_1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MuxTim32_1Tim2 {
    #[doc = "0: OSC32K"]
    Osc32k = 0,
    #[doc = "1: LSI32K"]
    Lsi32k = 1,
}
impl From<MuxTim32_1Tim2> for bool {
    #[inline(always)]
    fn from(variant: MuxTim32_1Tim2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUX_TIM32_1_TIM2` reader - Выбор асинхронного тактового сигнала для входа TIM2 Timer32_1"]
pub type MuxTim32_1Tim2R = crate::BitReader<MuxTim32_1Tim2>;
impl MuxTim32_1Tim2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MuxTim32_1Tim2 {
        match self.bits {
            false => MuxTim32_1Tim2::Osc32k,
            true => MuxTim32_1Tim2::Lsi32k,
        }
    }
    #[doc = "OSC32K"]
    #[inline(always)]
    pub fn is_osc32k(&self) -> bool {
        *self == MuxTim32_1Tim2::Osc32k
    }
    #[doc = "LSI32K"]
    #[inline(always)]
    pub fn is_lsi32k(&self) -> bool {
        *self == MuxTim32_1Tim2::Lsi32k
    }
}
#[doc = "Field `MUX_TIM32_1_TIM2` writer - Выбор асинхронного тактового сигнала для входа TIM2 Timer32_1"]
pub type MuxTim32_1Tim2W<'a, REG> = crate::BitWriter<'a, REG, MuxTim32_1Tim2>;
impl<'a, REG> MuxTim32_1Tim2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OSC32K"]
    #[inline(always)]
    pub fn osc32k(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim32_1Tim2::Osc32k)
    }
    #[doc = "LSI32K"]
    #[inline(always)]
    pub fn lsi32k(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim32_1Tim2::Lsi32k)
    }
}
#[doc = "Выбор синхронного тактового сигнала для входа TIM1 Timer32_2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MuxTim32_2Tim1 {
    #[doc = "0: Системная частота (sys_clk)"]
    SysClk = 0,
    #[doc = "1: Частота шины AHB (hclk)"]
    Hclk = 1,
}
impl From<MuxTim32_2Tim1> for bool {
    #[inline(always)]
    fn from(variant: MuxTim32_2Tim1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUX_TIM32_2_TIM1` reader - Выбор синхронного тактового сигнала для входа TIM1 Timer32_2"]
pub type MuxTim32_2Tim1R = crate::BitReader<MuxTim32_2Tim1>;
impl MuxTim32_2Tim1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MuxTim32_2Tim1 {
        match self.bits {
            false => MuxTim32_2Tim1::SysClk,
            true => MuxTim32_2Tim1::Hclk,
        }
    }
    #[doc = "Системная частота (sys_clk)"]
    #[inline(always)]
    pub fn is_sys_clk(&self) -> bool {
        *self == MuxTim32_2Tim1::SysClk
    }
    #[doc = "Частота шины AHB (hclk)"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == MuxTim32_2Tim1::Hclk
    }
}
#[doc = "Field `MUX_TIM32_2_TIM1` writer - Выбор синхронного тактового сигнала для входа TIM1 Timer32_2"]
pub type MuxTim32_2Tim1W<'a, REG> = crate::BitWriter<'a, REG, MuxTim32_2Tim1>;
impl<'a, REG> MuxTim32_2Tim1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Системная частота (sys_clk)"]
    #[inline(always)]
    pub fn sys_clk(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim32_2Tim1::SysClk)
    }
    #[doc = "Частота шины AHB (hclk)"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim32_2Tim1::Hclk)
    }
}
#[doc = "Выбор асинхронного тактового сигнала для входа TIM2 Timer32_2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MuxTim32_2Tim2 {
    #[doc = "0: OSC32K"]
    Osc32k = 0,
    #[doc = "1: LSI32K"]
    Lsi32k = 1,
}
impl From<MuxTim32_2Tim2> for bool {
    #[inline(always)]
    fn from(variant: MuxTim32_2Tim2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUX_TIM32_2_TIM2` reader - Выбор асинхронного тактового сигнала для входа TIM2 Timer32_2"]
pub type MuxTim32_2Tim2R = crate::BitReader<MuxTim32_2Tim2>;
impl MuxTim32_2Tim2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MuxTim32_2Tim2 {
        match self.bits {
            false => MuxTim32_2Tim2::Osc32k,
            true => MuxTim32_2Tim2::Lsi32k,
        }
    }
    #[doc = "OSC32K"]
    #[inline(always)]
    pub fn is_osc32k(&self) -> bool {
        *self == MuxTim32_2Tim2::Osc32k
    }
    #[doc = "LSI32K"]
    #[inline(always)]
    pub fn is_lsi32k(&self) -> bool {
        *self == MuxTim32_2Tim2::Lsi32k
    }
}
#[doc = "Field `MUX_TIM32_2_TIM2` writer - Выбор асинхронного тактового сигнала для входа TIM2 Timer32_2"]
pub type MuxTim32_2Tim2W<'a, REG> = crate::BitWriter<'a, REG, MuxTim32_2Tim2>;
impl<'a, REG> MuxTim32_2Tim2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OSC32K"]
    #[inline(always)]
    pub fn osc32k(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim32_2Tim2::Osc32k)
    }
    #[doc = "LSI32K"]
    #[inline(always)]
    pub fn lsi32k(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim32_2Tim2::Lsi32k)
    }
}
#[doc = "Выбор тактового сигнала для Timer16_0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MuxTim16_0 {
    #[doc = "0: Системная частота (sys_clk)"]
    SysClk = 0,
    #[doc = "1: Частота шины AHB (hclk)"]
    Hclk = 1,
    #[doc = "2: Частота внешнего осциллятора OSC32М"]
    Osc32m = 2,
    #[doc = "3: Частота встроенного осциллятора HSI32M"]
    Hsi32m = 3,
    #[doc = "4: частота внешнего осциллятора OSC32К"]
    Osc32k = 4,
    #[doc = "5: Частота встроенного осциллятора LSI32K"]
    Lsi32k = 5,
}
impl From<MuxTim16_0> for u8 {
    #[inline(always)]
    fn from(variant: MuxTim16_0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MuxTim16_0 {
    type Ux = u8;
}
impl crate::IsEnum for MuxTim16_0 {}
#[doc = "Field `MUX_TIM16_0` reader - Выбор тактового сигнала для Timer16_0"]
pub type MuxTim16_0R = crate::FieldReader<MuxTim16_0>;
impl MuxTim16_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MuxTim16_0> {
        match self.bits {
            0 => Some(MuxTim16_0::SysClk),
            1 => Some(MuxTim16_0::Hclk),
            2 => Some(MuxTim16_0::Osc32m),
            3 => Some(MuxTim16_0::Hsi32m),
            4 => Some(MuxTim16_0::Osc32k),
            5 => Some(MuxTim16_0::Lsi32k),
            _ => None,
        }
    }
    #[doc = "Системная частота (sys_clk)"]
    #[inline(always)]
    pub fn is_sys_clk(&self) -> bool {
        *self == MuxTim16_0::SysClk
    }
    #[doc = "Частота шины AHB (hclk)"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == MuxTim16_0::Hclk
    }
    #[doc = "Частота внешнего осциллятора OSC32М"]
    #[inline(always)]
    pub fn is_osc32m(&self) -> bool {
        *self == MuxTim16_0::Osc32m
    }
    #[doc = "Частота встроенного осциллятора HSI32M"]
    #[inline(always)]
    pub fn is_hsi32m(&self) -> bool {
        *self == MuxTim16_0::Hsi32m
    }
    #[doc = "частота внешнего осциллятора OSC32К"]
    #[inline(always)]
    pub fn is_osc32k(&self) -> bool {
        *self == MuxTim16_0::Osc32k
    }
    #[doc = "Частота встроенного осциллятора LSI32K"]
    #[inline(always)]
    pub fn is_lsi32k(&self) -> bool {
        *self == MuxTim16_0::Lsi32k
    }
}
#[doc = "Field `MUX_TIM16_0` writer - Выбор тактового сигнала для Timer16_0"]
pub type MuxTim16_0W<'a, REG> = crate::FieldWriter<'a, REG, 3, MuxTim16_0>;
impl<'a, REG> MuxTim16_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Системная частота (sys_clk)"]
    #[inline(always)]
    pub fn sys_clk(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim16_0::SysClk)
    }
    #[doc = "Частота шины AHB (hclk)"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim16_0::Hclk)
    }
    #[doc = "Частота внешнего осциллятора OSC32М"]
    #[inline(always)]
    pub fn osc32m(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim16_0::Osc32m)
    }
    #[doc = "Частота встроенного осциллятора HSI32M"]
    #[inline(always)]
    pub fn hsi32m(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim16_0::Hsi32m)
    }
    #[doc = "частота внешнего осциллятора OSC32К"]
    #[inline(always)]
    pub fn osc32k(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim16_0::Osc32k)
    }
    #[doc = "Частота встроенного осциллятора LSI32K"]
    #[inline(always)]
    pub fn lsi32k(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim16_0::Lsi32k)
    }
}
#[doc = "Выбор тактового сигнала для Timer16_1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MuxTim16_1 {
    #[doc = "0: Системная частота (sys_clk)"]
    SysClk = 0,
    #[doc = "1: Частота шины AHB (hclk)"]
    Hclk = 1,
    #[doc = "2: Частота внешнего осциллятора OSC32М"]
    Osc32m = 2,
    #[doc = "3: Частота встроенного осциллятора HSI32M"]
    Hsi32m = 3,
    #[doc = "4: частота внешнего осциллятора OSC32К"]
    Osc32k = 4,
    #[doc = "5: Частота встроенного осциллятора LSI32K"]
    Lsi32k = 5,
}
impl From<MuxTim16_1> for u8 {
    #[inline(always)]
    fn from(variant: MuxTim16_1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MuxTim16_1 {
    type Ux = u8;
}
impl crate::IsEnum for MuxTim16_1 {}
#[doc = "Field `MUX_TIM16_1` reader - Выбор тактового сигнала для Timer16_1"]
pub type MuxTim16_1R = crate::FieldReader<MuxTim16_1>;
impl MuxTim16_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MuxTim16_1> {
        match self.bits {
            0 => Some(MuxTim16_1::SysClk),
            1 => Some(MuxTim16_1::Hclk),
            2 => Some(MuxTim16_1::Osc32m),
            3 => Some(MuxTim16_1::Hsi32m),
            4 => Some(MuxTim16_1::Osc32k),
            5 => Some(MuxTim16_1::Lsi32k),
            _ => None,
        }
    }
    #[doc = "Системная частота (sys_clk)"]
    #[inline(always)]
    pub fn is_sys_clk(&self) -> bool {
        *self == MuxTim16_1::SysClk
    }
    #[doc = "Частота шины AHB (hclk)"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == MuxTim16_1::Hclk
    }
    #[doc = "Частота внешнего осциллятора OSC32М"]
    #[inline(always)]
    pub fn is_osc32m(&self) -> bool {
        *self == MuxTim16_1::Osc32m
    }
    #[doc = "Частота встроенного осциллятора HSI32M"]
    #[inline(always)]
    pub fn is_hsi32m(&self) -> bool {
        *self == MuxTim16_1::Hsi32m
    }
    #[doc = "частота внешнего осциллятора OSC32К"]
    #[inline(always)]
    pub fn is_osc32k(&self) -> bool {
        *self == MuxTim16_1::Osc32k
    }
    #[doc = "Частота встроенного осциллятора LSI32K"]
    #[inline(always)]
    pub fn is_lsi32k(&self) -> bool {
        *self == MuxTim16_1::Lsi32k
    }
}
#[doc = "Field `MUX_TIM16_1` writer - Выбор тактового сигнала для Timer16_1"]
pub type MuxTim16_1W<'a, REG> = crate::FieldWriter<'a, REG, 3, MuxTim16_1>;
impl<'a, REG> MuxTim16_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Системная частота (sys_clk)"]
    #[inline(always)]
    pub fn sys_clk(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim16_1::SysClk)
    }
    #[doc = "Частота шины AHB (hclk)"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim16_1::Hclk)
    }
    #[doc = "Частота внешнего осциллятора OSC32М"]
    #[inline(always)]
    pub fn osc32m(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim16_1::Osc32m)
    }
    #[doc = "Частота встроенного осциллятора HSI32M"]
    #[inline(always)]
    pub fn hsi32m(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim16_1::Hsi32m)
    }
    #[doc = "частота внешнего осциллятора OSC32К"]
    #[inline(always)]
    pub fn osc32k(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim16_1::Osc32k)
    }
    #[doc = "Частота встроенного осциллятора LSI32K"]
    #[inline(always)]
    pub fn lsi32k(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim16_1::Lsi32k)
    }
}
#[doc = "Выбор тактового сигнала для Timer16_2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MuxTim16_2 {
    #[doc = "0: Системная частота (sys_clk)"]
    SysClk = 0,
    #[doc = "1: Частота шины AHB (hclk)"]
    Hclk = 1,
    #[doc = "2: Частота внешнего осциллятора OSC32М"]
    Osc32m = 2,
    #[doc = "3: Частота встроенного осциллятора HSI32M"]
    Hsi32m = 3,
    #[doc = "4: частота внешнего осциллятора OSC32К"]
    Osc32k = 4,
    #[doc = "5: Частота встроенного осциллятора LSI32K"]
    Lsi32k = 5,
}
impl From<MuxTim16_2> for u8 {
    #[inline(always)]
    fn from(variant: MuxTim16_2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MuxTim16_2 {
    type Ux = u8;
}
impl crate::IsEnum for MuxTim16_2 {}
#[doc = "Field `MUX_TIM16_2` reader - Выбор тактового сигнала для Timer16_2"]
pub type MuxTim16_2R = crate::FieldReader<MuxTim16_2>;
impl MuxTim16_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MuxTim16_2> {
        match self.bits {
            0 => Some(MuxTim16_2::SysClk),
            1 => Some(MuxTim16_2::Hclk),
            2 => Some(MuxTim16_2::Osc32m),
            3 => Some(MuxTim16_2::Hsi32m),
            4 => Some(MuxTim16_2::Osc32k),
            5 => Some(MuxTim16_2::Lsi32k),
            _ => None,
        }
    }
    #[doc = "Системная частота (sys_clk)"]
    #[inline(always)]
    pub fn is_sys_clk(&self) -> bool {
        *self == MuxTim16_2::SysClk
    }
    #[doc = "Частота шины AHB (hclk)"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == MuxTim16_2::Hclk
    }
    #[doc = "Частота внешнего осциллятора OSC32М"]
    #[inline(always)]
    pub fn is_osc32m(&self) -> bool {
        *self == MuxTim16_2::Osc32m
    }
    #[doc = "Частота встроенного осциллятора HSI32M"]
    #[inline(always)]
    pub fn is_hsi32m(&self) -> bool {
        *self == MuxTim16_2::Hsi32m
    }
    #[doc = "частота внешнего осциллятора OSC32К"]
    #[inline(always)]
    pub fn is_osc32k(&self) -> bool {
        *self == MuxTim16_2::Osc32k
    }
    #[doc = "Частота встроенного осциллятора LSI32K"]
    #[inline(always)]
    pub fn is_lsi32k(&self) -> bool {
        *self == MuxTim16_2::Lsi32k
    }
}
#[doc = "Field `MUX_TIM16_2` writer - Выбор тактового сигнала для Timer16_2"]
pub type MuxTim16_2W<'a, REG> = crate::FieldWriter<'a, REG, 3, MuxTim16_2>;
impl<'a, REG> MuxTim16_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Системная частота (sys_clk)"]
    #[inline(always)]
    pub fn sys_clk(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim16_2::SysClk)
    }
    #[doc = "Частота шины AHB (hclk)"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim16_2::Hclk)
    }
    #[doc = "Частота внешнего осциллятора OSC32М"]
    #[inline(always)]
    pub fn osc32m(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim16_2::Osc32m)
    }
    #[doc = "Частота встроенного осциллятора HSI32M"]
    #[inline(always)]
    pub fn hsi32m(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim16_2::Hsi32m)
    }
    #[doc = "частота внешнего осциллятора OSC32К"]
    #[inline(always)]
    pub fn osc32k(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim16_2::Osc32k)
    }
    #[doc = "Частота встроенного осциллятора LSI32K"]
    #[inline(always)]
    pub fn lsi32k(self) -> &'a mut crate::W<REG> {
        self.variant(MuxTim16_2::Lsi32k)
    }
}
impl R {
    #[doc = "Bit 0 - Выбор синхронного тактового сигнала для входа TIM1 Timer32_0"]
    #[inline(always)]
    pub fn mux_tim32_0_tim1(&self) -> MuxTim32_0Tim1R {
        MuxTim32_0Tim1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Выбор асинхронного тактового сигнала для входа TIM2 Timer32_0"]
    #[inline(always)]
    pub fn mux_tim32_0_tim2(&self) -> MuxTim32_0Tim2R {
        MuxTim32_0Tim2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Выбор синхронного тактового сигнала для входа TIM1 Timer32_1"]
    #[inline(always)]
    pub fn mux_tim32_1_tim1(&self) -> MuxTim32_1Tim1R {
        MuxTim32_1Tim1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Выбор асинхронного тактового сигнала для входа TIM2 Timer32_1"]
    #[inline(always)]
    pub fn mux_tim32_1_tim2(&self) -> MuxTim32_1Tim2R {
        MuxTim32_1Tim2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Выбор синхронного тактового сигнала для входа TIM1 Timer32_2"]
    #[inline(always)]
    pub fn mux_tim32_2_tim1(&self) -> MuxTim32_2Tim1R {
        MuxTim32_2Tim1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Выбор асинхронного тактового сигнала для входа TIM2 Timer32_2"]
    #[inline(always)]
    pub fn mux_tim32_2_tim2(&self) -> MuxTim32_2Tim2R {
        MuxTim32_2Tim2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Выбор тактового сигнала для Timer16_0"]
    #[inline(always)]
    pub fn mux_tim16_0(&self) -> MuxTim16_0R {
        MuxTim16_0R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Выбор тактового сигнала для Timer16_1"]
    #[inline(always)]
    pub fn mux_tim16_1(&self) -> MuxTim16_1R {
        MuxTim16_1R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Выбор тактового сигнала для Timer16_2"]
    #[inline(always)]
    pub fn mux_tim16_2(&self) -> MuxTim16_2R {
        MuxTim16_2R::new(((self.bits >> 15) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Выбор синхронного тактового сигнала для входа TIM1 Timer32_0"]
    #[inline(always)]
    pub fn mux_tim32_0_tim1(&mut self) -> MuxTim32_0Tim1W<TimerCfgSpec> {
        MuxTim32_0Tim1W::new(self, 0)
    }
    #[doc = "Bit 1 - Выбор асинхронного тактового сигнала для входа TIM2 Timer32_0"]
    #[inline(always)]
    pub fn mux_tim32_0_tim2(&mut self) -> MuxTim32_0Tim2W<TimerCfgSpec> {
        MuxTim32_0Tim2W::new(self, 1)
    }
    #[doc = "Bit 3 - Выбор синхронного тактового сигнала для входа TIM1 Timer32_1"]
    #[inline(always)]
    pub fn mux_tim32_1_tim1(&mut self) -> MuxTim32_1Tim1W<TimerCfgSpec> {
        MuxTim32_1Tim1W::new(self, 3)
    }
    #[doc = "Bit 4 - Выбор асинхронного тактового сигнала для входа TIM2 Timer32_1"]
    #[inline(always)]
    pub fn mux_tim32_1_tim2(&mut self) -> MuxTim32_1Tim2W<TimerCfgSpec> {
        MuxTim32_1Tim2W::new(self, 4)
    }
    #[doc = "Bit 6 - Выбор синхронного тактового сигнала для входа TIM1 Timer32_2"]
    #[inline(always)]
    pub fn mux_tim32_2_tim1(&mut self) -> MuxTim32_2Tim1W<TimerCfgSpec> {
        MuxTim32_2Tim1W::new(self, 6)
    }
    #[doc = "Bit 7 - Выбор асинхронного тактового сигнала для входа TIM2 Timer32_2"]
    #[inline(always)]
    pub fn mux_tim32_2_tim2(&mut self) -> MuxTim32_2Tim2W<TimerCfgSpec> {
        MuxTim32_2Tim2W::new(self, 7)
    }
    #[doc = "Bits 9:11 - Выбор тактового сигнала для Timer16_0"]
    #[inline(always)]
    pub fn mux_tim16_0(&mut self) -> MuxTim16_0W<TimerCfgSpec> {
        MuxTim16_0W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Выбор тактового сигнала для Timer16_1"]
    #[inline(always)]
    pub fn mux_tim16_1(&mut self) -> MuxTim16_1W<TimerCfgSpec> {
        MuxTim16_1W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Выбор тактового сигнала для Timer16_2"]
    #[inline(always)]
    pub fn mux_tim16_2(&mut self) -> MuxTim16_2W<TimerCfgSpec> {
        MuxTim16_2W::new(self, 15)
    }
}
#[doc = "Выбор источника тактирования для таймеров\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimerCfgSpec;
impl crate::RegisterSpec for TimerCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_cfg::R`](R) reader structure"]
impl crate::Readable for TimerCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`timer_cfg::W`](W) writer structure"]
impl crate::Writable for TimerCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER_CFG to value 0"]
impl crate::Resettable for TimerCfgSpec {
    const RESET_VALUE: u32 = 0;
}
