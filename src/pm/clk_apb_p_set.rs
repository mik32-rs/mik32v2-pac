#[doc = "Register `CLK_APB_P_SET` reader"]
pub type R = crate::R<ClkApbPSetSpec>;
#[doc = "Register `CLK_APB_P_SET` writer"]
pub type W = crate::W<ClkApbPSetSpec>;
#[doc = "Сторожевой таймер\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdt {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<Wdt> for bool {
    #[inline(always)]
    fn from(variant: Wdt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT` reader - Сторожевой таймер"]
pub type WdtR = crate::BitReader<Wdt>;
impl WdtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdt {
        match self.bits {
            false => Wdt::Disable,
            true => Wdt::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wdt::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wdt::Enable
    }
}
#[doc = "Field `WDT` writer - Сторожевой таймер"]
pub type WdtW<'a, REG> = crate::BitWriter<'a, REG, Wdt>;
impl<'a, REG> WdtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wdt::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wdt::Enable)
    }
}
#[doc = "UART_0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart0 {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<Uart0> for bool {
    #[inline(always)]
    fn from(variant: Uart0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART_0` reader - UART_0"]
pub type Uart0R = crate::BitReader<Uart0>;
impl Uart0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart0 {
        match self.bits {
            false => Uart0::Disable,
            true => Uart0::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Uart0::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Uart0::Enable
    }
}
#[doc = "Field `UART_0` writer - UART_0"]
pub type Uart0W<'a, REG> = crate::BitWriter<'a, REG, Uart0>;
impl<'a, REG> Uart0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Uart0::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Uart0::Enable)
    }
}
#[doc = "UART_1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart1 {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<Uart1> for bool {
    #[inline(always)]
    fn from(variant: Uart1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART_1` reader - UART_1"]
pub type Uart1R = crate::BitReader<Uart1>;
impl Uart1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart1 {
        match self.bits {
            false => Uart1::Disable,
            true => Uart1::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Uart1::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Uart1::Enable
    }
}
#[doc = "Field `UART_1` writer - UART_1"]
pub type Uart1W<'a, REG> = crate::BitWriter<'a, REG, Uart1>;
impl<'a, REG> Uart1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Uart1::Enable)
    }
}
#[doc = "Timer16_0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer16_0 {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<Timer16_0> for bool {
    #[inline(always)]
    fn from(variant: Timer16_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Timer16_0` reader - Timer16_0"]
pub type Timer16_0R = crate::BitReader<Timer16_0>;
impl Timer16_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer16_0 {
        match self.bits {
            false => Timer16_0::Disable,
            true => Timer16_0::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Timer16_0::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Timer16_0::Enable
    }
}
#[doc = "Field `Timer16_0` writer - Timer16_0"]
pub type Timer16_0W<'a, REG> = crate::BitWriter<'a, REG, Timer16_0>;
impl<'a, REG> Timer16_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer16_0::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer16_0::Enable)
    }
}
#[doc = "Timer16_1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer16_1 {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<Timer16_1> for bool {
    #[inline(always)]
    fn from(variant: Timer16_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Timer16_1` reader - Timer16_1"]
pub type Timer16_1R = crate::BitReader<Timer16_1>;
impl Timer16_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer16_1 {
        match self.bits {
            false => Timer16_1::Disable,
            true => Timer16_1::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Timer16_1::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Timer16_1::Enable
    }
}
#[doc = "Field `Timer16_1` writer - Timer16_1"]
pub type Timer16_1W<'a, REG> = crate::BitWriter<'a, REG, Timer16_1>;
impl<'a, REG> Timer16_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer16_1::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer16_1::Enable)
    }
}
#[doc = "Timer16_2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer16_2 {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<Timer16_2> for bool {
    #[inline(always)]
    fn from(variant: Timer16_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Timer16_2` reader - Timer16_2"]
pub type Timer16_2R = crate::BitReader<Timer16_2>;
impl Timer16_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer16_2 {
        match self.bits {
            false => Timer16_2::Disable,
            true => Timer16_2::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Timer16_2::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Timer16_2::Enable
    }
}
#[doc = "Field `Timer16_2` writer - Timer16_2"]
pub type Timer16_2W<'a, REG> = crate::BitWriter<'a, REG, Timer16_2>;
impl<'a, REG> Timer16_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer16_2::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer16_2::Enable)
    }
}
#[doc = "Timer32_1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer32_1 {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<Timer32_1> for bool {
    #[inline(always)]
    fn from(variant: Timer32_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Timer32_1` reader - Timer32_1"]
pub type Timer32_1R = crate::BitReader<Timer32_1>;
impl Timer32_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer32_1 {
        match self.bits {
            false => Timer32_1::Disable,
            true => Timer32_1::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Timer32_1::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Timer32_1::Enable
    }
}
#[doc = "Field `Timer32_1` writer - Timer32_1"]
pub type Timer32_1W<'a, REG> = crate::BitWriter<'a, REG, Timer32_1>;
impl<'a, REG> Timer32_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer32_1::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer32_1::Enable)
    }
}
#[doc = "Timer32_2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer32_2 {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<Timer32_2> for bool {
    #[inline(always)]
    fn from(variant: Timer32_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Timer32_2` reader - Timer32_2"]
pub type Timer32_2R = crate::BitReader<Timer32_2>;
impl Timer32_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer32_2 {
        match self.bits {
            false => Timer32_2::Disable,
            true => Timer32_2::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Timer32_2::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Timer32_2::Enable
    }
}
#[doc = "Field `Timer32_2` writer - Timer32_2"]
pub type Timer32_2W<'a, REG> = crate::BitWriter<'a, REG, Timer32_2>;
impl<'a, REG> Timer32_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer32_2::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer32_2::Enable)
    }
}
#[doc = "SPI_0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi0 {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<Spi0> for bool {
    #[inline(always)]
    fn from(variant: Spi0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI_0` reader - SPI_0"]
pub type Spi0R = crate::BitReader<Spi0>;
impl Spi0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spi0 {
        match self.bits {
            false => Spi0::Disable,
            true => Spi0::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Spi0::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Spi0::Enable
    }
}
#[doc = "Field `SPI_0` writer - SPI_0"]
pub type Spi0W<'a, REG> = crate::BitWriter<'a, REG, Spi0>;
impl<'a, REG> Spi0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Spi0::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Spi0::Enable)
    }
}
#[doc = "SPI_1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi1 {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<Spi1> for bool {
    #[inline(always)]
    fn from(variant: Spi1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI_1` reader - SPI_1"]
pub type Spi1R = crate::BitReader<Spi1>;
impl Spi1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spi1 {
        match self.bits {
            false => Spi1::Disable,
            true => Spi1::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Spi1::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Spi1::Enable
    }
}
#[doc = "Field `SPI_1` writer - SPI_1"]
pub type Spi1W<'a, REG> = crate::BitWriter<'a, REG, Spi1>;
impl<'a, REG> Spi1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Spi1::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Spi1::Enable)
    }
}
#[doc = "I2C_0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c0 {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<I2c0> for bool {
    #[inline(always)]
    fn from(variant: I2c0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_0` reader - I2C_0"]
pub type I2c0R = crate::BitReader<I2c0>;
impl I2c0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c0 {
        match self.bits {
            false => I2c0::Disable,
            true => I2c0::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == I2c0::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == I2c0::Enable
    }
}
#[doc = "Field `I2C_0` writer - I2C_0"]
pub type I2c0W<'a, REG> = crate::BitWriter<'a, REG, I2c0>;
impl<'a, REG> I2c0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(I2c0::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(I2c0::Enable)
    }
}
#[doc = "I2C_1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c1 {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<I2c1> for bool {
    #[inline(always)]
    fn from(variant: I2c1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_1` reader - I2C_1"]
pub type I2c1R = crate::BitReader<I2c1>;
impl I2c1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c1 {
        match self.bits {
            false => I2c1::Disable,
            true => I2c1::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == I2c1::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == I2c1::Enable
    }
}
#[doc = "Field `I2C_1` writer - I2C_1"]
pub type I2c1W<'a, REG> = crate::BitWriter<'a, REG, I2c1>;
impl<'a, REG> I2c1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(I2c1::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(I2c1::Enable)
    }
}
#[doc = "GPIO_0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio0 {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<Gpio0> for bool {
    #[inline(always)]
    fn from(variant: Gpio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_0` reader - GPIO_0"]
pub type Gpio0R = crate::BitReader<Gpio0>;
impl Gpio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio0 {
        match self.bits {
            false => Gpio0::Disable,
            true => Gpio0::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Gpio0::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Gpio0::Enable
    }
}
#[doc = "Field `GPIO_0` writer - GPIO_0"]
pub type Gpio0W<'a, REG> = crate::BitWriter<'a, REG, Gpio0>;
impl<'a, REG> Gpio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0::Enable)
    }
}
#[doc = "GPIO_1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio1 {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<Gpio1> for bool {
    #[inline(always)]
    fn from(variant: Gpio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_1` reader - GPIO_1"]
pub type Gpio1R = crate::BitReader<Gpio1>;
impl Gpio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio1 {
        match self.bits {
            false => Gpio1::Disable,
            true => Gpio1::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Gpio1::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Gpio1::Enable
    }
}
#[doc = "Field `GPIO_1` writer - GPIO_1"]
pub type Gpio1W<'a, REG> = crate::BitWriter<'a, REG, Gpio1>;
impl<'a, REG> Gpio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1::Enable)
    }
}
#[doc = "GPIO_2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio2 {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<Gpio2> for bool {
    #[inline(always)]
    fn from(variant: Gpio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_2` reader - GPIO_2"]
pub type Gpio2R = crate::BitReader<Gpio2>;
impl Gpio2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2 {
        match self.bits {
            false => Gpio2::Disable,
            true => Gpio2::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Gpio2::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Gpio2::Enable
    }
}
#[doc = "Field `GPIO_2` writer - GPIO_2"]
pub type Gpio2W<'a, REG> = crate::BitWriter<'a, REG, Gpio2>;
impl<'a, REG> Gpio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2::Enable)
    }
}
#[doc = "Регистры аналоговых блоков\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnalogRegs {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<AnalogRegs> for bool {
    #[inline(always)]
    fn from(variant: AnalogRegs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Analog_regs` reader - Регистры аналоговых блоков"]
pub type AnalogRegsR = crate::BitReader<AnalogRegs>;
impl AnalogRegsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AnalogRegs {
        match self.bits {
            false => AnalogRegs::Disable,
            true => AnalogRegs::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AnalogRegs::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AnalogRegs::Enable
    }
}
#[doc = "Field `Analog_regs` writer - Регистры аналоговых блоков"]
pub type AnalogRegsW<'a, REG> = crate::BitWriter<'a, REG, AnalogRegs>;
impl<'a, REG> AnalogRegsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(AnalogRegs::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(AnalogRegs::Enable)
    }
}
#[doc = "Схема формирования прерываний GPIO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpioIrq {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<GpioIrq> for bool {
    #[inline(always)]
    fn from(variant: GpioIrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_IRQ` reader - Схема формирования прерываний GPIO"]
pub type GpioIrqR = crate::BitReader<GpioIrq>;
impl GpioIrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpioIrq {
        match self.bits {
            false => GpioIrq::Disable,
            true => GpioIrq::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == GpioIrq::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == GpioIrq::Enable
    }
}
#[doc = "Field `GPIO_IRQ` writer - Схема формирования прерываний GPIO"]
pub type GpioIrqW<'a, REG> = crate::BitWriter<'a, REG, GpioIrq>;
impl<'a, REG> GpioIrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(GpioIrq::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(GpioIrq::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Сторожевой таймер"]
    #[inline(always)]
    pub fn wdt(&self) -> WdtR {
        WdtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART_0"]
    #[inline(always)]
    pub fn uart_0(&self) -> Uart0R {
        Uart0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART_1"]
    #[inline(always)]
    pub fn uart_1(&self) -> Uart1R {
        Uart1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer16_0"]
    #[inline(always)]
    pub fn timer16_0(&self) -> Timer16_0R {
        Timer16_0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer16_1"]
    #[inline(always)]
    pub fn timer16_1(&self) -> Timer16_1R {
        Timer16_1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer16_2"]
    #[inline(always)]
    pub fn timer16_2(&self) -> Timer16_2R {
        Timer16_2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer32_1"]
    #[inline(always)]
    pub fn timer32_1(&self) -> Timer32_1R {
        Timer32_1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer32_2"]
    #[inline(always)]
    pub fn timer32_2(&self) -> Timer32_2R {
        Timer32_2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SPI_0"]
    #[inline(always)]
    pub fn spi_0(&self) -> Spi0R {
        Spi0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SPI_1"]
    #[inline(always)]
    pub fn spi_1(&self) -> Spi1R {
        Spi1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - I2C_0"]
    #[inline(always)]
    pub fn i2c_0(&self) -> I2c0R {
        I2c0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2C_1"]
    #[inline(always)]
    pub fn i2c_1(&self) -> I2c1R {
        I2c1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIO_0"]
    #[inline(always)]
    pub fn gpio_0(&self) -> Gpio0R {
        Gpio0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIO_1"]
    #[inline(always)]
    pub fn gpio_1(&self) -> Gpio1R {
        Gpio1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO_2"]
    #[inline(always)]
    pub fn gpio_2(&self) -> Gpio2R {
        Gpio2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Регистры аналоговых блоков"]
    #[inline(always)]
    pub fn analog_regs(&self) -> AnalogRegsR {
        AnalogRegsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 15 - Схема формирования прерываний GPIO"]
    #[inline(always)]
    pub fn gpio_irq(&self) -> GpioIrqR {
        GpioIrqR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Сторожевой таймер"]
    #[inline(always)]
    pub fn wdt(&mut self) -> WdtW<ClkApbPSetSpec> {
        WdtW::new(self, 0)
    }
    #[doc = "Bit 1 - UART_0"]
    #[inline(always)]
    pub fn uart_0(&mut self) -> Uart0W<ClkApbPSetSpec> {
        Uart0W::new(self, 1)
    }
    #[doc = "Bit 2 - UART_1"]
    #[inline(always)]
    pub fn uart_1(&mut self) -> Uart1W<ClkApbPSetSpec> {
        Uart1W::new(self, 2)
    }
    #[doc = "Bit 3 - Timer16_0"]
    #[inline(always)]
    pub fn timer16_0(&mut self) -> Timer16_0W<ClkApbPSetSpec> {
        Timer16_0W::new(self, 3)
    }
    #[doc = "Bit 4 - Timer16_1"]
    #[inline(always)]
    pub fn timer16_1(&mut self) -> Timer16_1W<ClkApbPSetSpec> {
        Timer16_1W::new(self, 4)
    }
    #[doc = "Bit 5 - Timer16_2"]
    #[inline(always)]
    pub fn timer16_2(&mut self) -> Timer16_2W<ClkApbPSetSpec> {
        Timer16_2W::new(self, 5)
    }
    #[doc = "Bit 6 - Timer32_1"]
    #[inline(always)]
    pub fn timer32_1(&mut self) -> Timer32_1W<ClkApbPSetSpec> {
        Timer32_1W::new(self, 6)
    }
    #[doc = "Bit 7 - Timer32_2"]
    #[inline(always)]
    pub fn timer32_2(&mut self) -> Timer32_2W<ClkApbPSetSpec> {
        Timer32_2W::new(self, 7)
    }
    #[doc = "Bit 8 - SPI_0"]
    #[inline(always)]
    pub fn spi_0(&mut self) -> Spi0W<ClkApbPSetSpec> {
        Spi0W::new(self, 8)
    }
    #[doc = "Bit 9 - SPI_1"]
    #[inline(always)]
    pub fn spi_1(&mut self) -> Spi1W<ClkApbPSetSpec> {
        Spi1W::new(self, 9)
    }
    #[doc = "Bit 10 - I2C_0"]
    #[inline(always)]
    pub fn i2c_0(&mut self) -> I2c0W<ClkApbPSetSpec> {
        I2c0W::new(self, 10)
    }
    #[doc = "Bit 11 - I2C_1"]
    #[inline(always)]
    pub fn i2c_1(&mut self) -> I2c1W<ClkApbPSetSpec> {
        I2c1W::new(self, 11)
    }
    #[doc = "Bit 12 - GPIO_0"]
    #[inline(always)]
    pub fn gpio_0(&mut self) -> Gpio0W<ClkApbPSetSpec> {
        Gpio0W::new(self, 12)
    }
    #[doc = "Bit 13 - GPIO_1"]
    #[inline(always)]
    pub fn gpio_1(&mut self) -> Gpio1W<ClkApbPSetSpec> {
        Gpio1W::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO_2"]
    #[inline(always)]
    pub fn gpio_2(&mut self) -> Gpio2W<ClkApbPSetSpec> {
        Gpio2W::new(self, 14)
    }
    #[doc = "Bit 15 - Регистры аналоговых блоков"]
    #[inline(always)]
    pub fn analog_regs(&mut self) -> AnalogRegsW<ClkApbPSetSpec> {
        AnalogRegsW::new(self, 15)
    }
    #[doc = "Bit 15 - Схема формирования прерываний GPIO"]
    #[inline(always)]
    pub fn gpio_irq(&mut self) -> GpioIrqW<ClkApbPSetSpec> {
        GpioIrqW::new(self, 16)
    }
}
#[doc = "Регистр включения тактированием устройств на шине APB_P. Каждому биту соответствует одно устройство\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_apb_p_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_apb_p_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkApbPSetSpec;
impl crate::RegisterSpec for ClkApbPSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_apb_p_set::R`](R) reader structure"]
impl crate::Readable for ClkApbPSetSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_apb_p_set::W`](W) writer structure"]
impl crate::Writable for ClkApbPSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_APB_P_SET to value 0"]
impl crate::Resettable for ClkApbPSetSpec {
    const RESET_VALUE: u32 = 0;
}
