#[doc = "Register `MASK_EDGE_CLEAR` reader"]
pub type R = crate::R<MaskEdgeClearSpec>;
#[doc = "Register `MASK_EDGE_CLEAR` writer"]
pub type W = crate::W<MaskEdgeClearSpec>;
#[doc = "Линия прерывания Timer32_0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer32_0 {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<Timer32_0> for bool {
    #[inline(always)]
    fn from(variant: Timer32_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Timer32_0` reader - Линия прерывания Timer32_0"]
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
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Timer32_0::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Timer32_0::Enable
    }
}
#[doc = "Field `Timer32_0` writer - Линия прерывания Timer32_0"]
pub type Timer32_0W<'a, REG> = crate::BitWriter1C<'a, REG, Timer32_0>;
impl<'a, REG> Timer32_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer32_0::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer32_0::Enable)
    }
}
#[doc = "Линия прерывания USART_0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usart0 {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<Usart0> for bool {
    #[inline(always)]
    fn from(variant: Usart0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART_0` reader - Линия прерывания USART_0"]
pub type Usart0R = crate::BitReader<Usart0>;
impl Usart0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart0 {
        match self.bits {
            false => Usart0::Disable,
            true => Usart0::Enable,
        }
    }
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Usart0::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Usart0::Enable
    }
}
#[doc = "Field `USART_0` writer - Линия прерывания USART_0"]
pub type Usart0W<'a, REG> = crate::BitWriter1C<'a, REG, Usart0>;
impl<'a, REG> Usart0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Usart0::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Usart0::Enable)
    }
}
#[doc = "Линия прерывания USART_1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usart1 {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<Usart1> for bool {
    #[inline(always)]
    fn from(variant: Usart1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART_1` reader - Линия прерывания USART_1"]
pub type Usart1R = crate::BitReader<Usart1>;
impl Usart1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usart1 {
        match self.bits {
            false => Usart1::Disable,
            true => Usart1::Enable,
        }
    }
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Usart1::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Usart1::Enable
    }
}
#[doc = "Field `USART_1` writer - Линия прерывания USART_1"]
pub type Usart1W<'a, REG> = crate::BitWriter1C<'a, REG, Usart1>;
impl<'a, REG> Usart1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Usart1::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Usart1::Enable)
    }
}
#[doc = "Линия прерывания SPI_0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi0 {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<Spi0> for bool {
    #[inline(always)]
    fn from(variant: Spi0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI_0` reader - Линия прерывания SPI_0"]
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
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Spi0::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Spi0::Enable
    }
}
#[doc = "Field `SPI_0` writer - Линия прерывания SPI_0"]
pub type Spi0W<'a, REG> = crate::BitWriter1C<'a, REG, Spi0>;
impl<'a, REG> Spi0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Spi0::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Spi0::Enable)
    }
}
#[doc = "Линия прерывания SPI_1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi1 {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<Spi1> for bool {
    #[inline(always)]
    fn from(variant: Spi1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI_1` reader - Линия прерывания SPI_1"]
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
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Spi1::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Spi1::Enable
    }
}
#[doc = "Field `SPI_1` writer - Линия прерывания SPI_1"]
pub type Spi1W<'a, REG> = crate::BitWriter1C<'a, REG, Spi1>;
impl<'a, REG> Spi1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Spi1::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Spi1::Enable)
    }
}
#[doc = "Линия прерывания GPIO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<Gpio> for bool {
    #[inline(always)]
    fn from(variant: Gpio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO` reader - Линия прерывания GPIO"]
pub type GpioR = crate::BitReader<Gpio>;
impl GpioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio {
        match self.bits {
            false => Gpio::Disable,
            true => Gpio::Enable,
        }
    }
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Gpio::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Gpio::Enable
    }
}
#[doc = "Field `GPIO` writer - Линия прерывания GPIO"]
pub type GpioW<'a, REG> = crate::BitWriter1C<'a, REG, Gpio>;
impl<'a, REG> GpioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio::Enable)
    }
}
#[doc = "Линия прерывания I2C_0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c0 {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<I2c0> for bool {
    #[inline(always)]
    fn from(variant: I2c0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_0` reader - Линия прерывания I2C_0"]
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
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == I2c0::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == I2c0::Enable
    }
}
#[doc = "Field `I2C_0` writer - Линия прерывания I2C_0"]
pub type I2c0W<'a, REG> = crate::BitWriter1C<'a, REG, I2c0>;
impl<'a, REG> I2c0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(I2c0::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(I2c0::Enable)
    }
}
#[doc = "Линия прерывания I2C_1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c1 {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<I2c1> for bool {
    #[inline(always)]
    fn from(variant: I2c1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_1` reader - Линия прерывания I2C_1"]
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
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == I2c1::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == I2c1::Enable
    }
}
#[doc = "Field `I2C_1` writer - Линия прерывания I2C_1"]
pub type I2c1W<'a, REG> = crate::BitWriter1C<'a, REG, I2c1>;
impl<'a, REG> I2c1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(I2c1::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(I2c1::Enable)
    }
}
#[doc = "Линия прерывания сторожевого таймера (WDT)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdt {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<Wdt> for bool {
    #[inline(always)]
    fn from(variant: Wdt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT` reader - Линия прерывания сторожевого таймера (WDT)"]
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
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wdt::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wdt::Enable
    }
}
#[doc = "Field `WDT` writer - Линия прерывания сторожевого таймера (WDT)"]
pub type WdtW<'a, REG> = crate::BitWriter1C<'a, REG, Wdt>;
impl<'a, REG> WdtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wdt::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wdt::Enable)
    }
}
#[doc = "Линия прерывания Timer16_0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer16_0 {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<Timer16_0> for bool {
    #[inline(always)]
    fn from(variant: Timer16_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Timer16_0` reader - Линия прерывания Timer16_0"]
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
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Timer16_0::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Timer16_0::Enable
    }
}
#[doc = "Field `Timer16_0` writer - Линия прерывания Timer16_0"]
pub type Timer16_0W<'a, REG> = crate::BitWriter1C<'a, REG, Timer16_0>;
impl<'a, REG> Timer16_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer16_0::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer16_0::Enable)
    }
}
#[doc = "Линия прерывания Timer16_1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer16_1 {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<Timer16_1> for bool {
    #[inline(always)]
    fn from(variant: Timer16_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Timer16_1` reader - Линия прерывания Timer16_1"]
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
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Timer16_1::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Timer16_1::Enable
    }
}
#[doc = "Field `Timer16_1` writer - Линия прерывания Timer16_1"]
pub type Timer16_1W<'a, REG> = crate::BitWriter1C<'a, REG, Timer16_1>;
impl<'a, REG> Timer16_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer16_1::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer16_1::Enable)
    }
}
#[doc = "Линия прерывания Timer16_2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer16_2 {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<Timer16_2> for bool {
    #[inline(always)]
    fn from(variant: Timer16_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Timer16_2` reader - Линия прерывания Timer16_2"]
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
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Timer16_2::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Timer16_2::Enable
    }
}
#[doc = "Field `Timer16_2` writer - Линия прерывания Timer16_2"]
pub type Timer16_2W<'a, REG> = crate::BitWriter1C<'a, REG, Timer16_2>;
impl<'a, REG> Timer16_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer16_2::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer16_2::Enable)
    }
}
#[doc = "Линия прерывания Timer32_1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer32_1 {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<Timer32_1> for bool {
    #[inline(always)]
    fn from(variant: Timer32_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Timer32_1` reader - Линия прерывания Timer32_1"]
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
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Timer32_1::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Timer32_1::Enable
    }
}
#[doc = "Field `Timer32_1` writer - Линия прерывания Timer32_1"]
pub type Timer32_1W<'a, REG> = crate::BitWriter1C<'a, REG, Timer32_1>;
impl<'a, REG> Timer32_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer32_1::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer32_1::Enable)
    }
}
#[doc = "Линия прерывания Timer32_2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer32_2 {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<Timer32_2> for bool {
    #[inline(always)]
    fn from(variant: Timer32_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Timer32_2` reader - Линия прерывания Timer32_2"]
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
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Timer32_2::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Timer32_2::Enable
    }
}
#[doc = "Field `Timer32_2` writer - Линия прерывания Timer32_2"]
pub type Timer32_2W<'a, REG> = crate::BitWriter1C<'a, REG, Timer32_2>;
impl<'a, REG> Timer32_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer32_2::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Timer32_2::Enable)
    }
}
#[doc = "Линия прерывания SPIFI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spifi {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<Spifi> for bool {
    #[inline(always)]
    fn from(variant: Spifi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIFI` reader - Линия прерывания SPIFI"]
pub type SpifiR = crate::BitReader<Spifi>;
impl SpifiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spifi {
        match self.bits {
            false => Spifi::Disable,
            true => Spifi::Enable,
        }
    }
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Spifi::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Spifi::Enable
    }
}
#[doc = "Field `SPIFI` writer - Линия прерывания SPIFI"]
pub type SpifiW<'a, REG> = crate::BitWriter1C<'a, REG, Spifi>;
impl<'a, REG> SpifiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Spifi::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Spifi::Enable)
    }
}
#[doc = "Линия прерывания RTC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtc {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<Rtc> for bool {
    #[inline(always)]
    fn from(variant: Rtc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC` reader - Линия прерывания RTC"]
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
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rtc::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rtc::Enable
    }
}
#[doc = "Field `RTC` writer - Линия прерывания RTC"]
pub type RtcW<'a, REG> = crate::BitWriter1C<'a, REG, Rtc>;
impl<'a, REG> RtcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::Enable)
    }
}
#[doc = "Линия прерывания EEPROM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eeprom {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<Eeprom> for bool {
    #[inline(always)]
    fn from(variant: Eeprom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEPROM` reader - Линия прерывания EEPROM"]
pub type EepromR = crate::BitReader<Eeprom>;
impl EepromR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eeprom {
        match self.bits {
            false => Eeprom::Disable,
            true => Eeprom::Enable,
        }
    }
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Eeprom::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Eeprom::Enable
    }
}
#[doc = "Field `EEPROM` writer - Линия прерывания EEPROM"]
pub type EepromW<'a, REG> = crate::BitWriter1C<'a, REG, Eeprom>;
impl<'a, REG> EepromW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Eeprom::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Eeprom::Enable)
    }
}
#[doc = "Линия прерывания сторожевого таймера шины (периферийные устройства)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WdtBusDom3 {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<WdtBusDom3> for bool {
    #[inline(always)]
    fn from(variant: WdtBusDom3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT_BUS_DOM3` reader - Линия прерывания сторожевого таймера шины (периферийные устройства)"]
pub type WdtBusDom3R = crate::BitReader<WdtBusDom3>;
impl WdtBusDom3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WdtBusDom3 {
        match self.bits {
            false => WdtBusDom3::Disable,
            true => WdtBusDom3::Enable,
        }
    }
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WdtBusDom3::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WdtBusDom3::Enable
    }
}
#[doc = "Field `WDT_BUS_DOM3` writer - Линия прерывания сторожевого таймера шины (периферийные устройства)"]
pub type WdtBusDom3W<'a, REG> = crate::BitWriter1C<'a, REG, WdtBusDom3>;
impl<'a, REG> WdtBusDom3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WdtBusDom3::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WdtBusDom3::Enable)
    }
}
#[doc = "Линия прерывания сторожевого таймера шины (SPIFI)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WdtBusSpifi {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<WdtBusSpifi> for bool {
    #[inline(always)]
    fn from(variant: WdtBusSpifi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT_BUS_SPIFI` reader - Линия прерывания сторожевого таймера шины (SPIFI)"]
pub type WdtBusSpifiR = crate::BitReader<WdtBusSpifi>;
impl WdtBusSpifiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WdtBusSpifi {
        match self.bits {
            false => WdtBusSpifi::Disable,
            true => WdtBusSpifi::Enable,
        }
    }
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WdtBusSpifi::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WdtBusSpifi::Enable
    }
}
#[doc = "Field `WDT_BUS_SPIFI` writer - Линия прерывания сторожевого таймера шины (SPIFI)"]
pub type WdtBusSpifiW<'a, REG> = crate::BitWriter1C<'a, REG, WdtBusSpifi>;
impl<'a, REG> WdtBusSpifiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WdtBusSpifi::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WdtBusSpifi::Enable)
    }
}
#[doc = "Линия прерывания сторожевого таймера шины (EEPROM)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WdtBusEeprom {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<WdtBusEeprom> for bool {
    #[inline(always)]
    fn from(variant: WdtBusEeprom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDT_BUS_EEPROM` reader - Линия прерывания сторожевого таймера шины (EEPROM)"]
pub type WdtBusEepromR = crate::BitReader<WdtBusEeprom>;
impl WdtBusEepromR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WdtBusEeprom {
        match self.bits {
            false => WdtBusEeprom::Disable,
            true => WdtBusEeprom::Enable,
        }
    }
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WdtBusEeprom::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WdtBusEeprom::Enable
    }
}
#[doc = "Field `WDT_BUS_EEPROM` writer - Линия прерывания сторожевого таймера шины (EEPROM)"]
pub type WdtBusEepromW<'a, REG> = crate::BitWriter1C<'a, REG, WdtBusEeprom>;
impl<'a, REG> WdtBusEepromW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WdtBusEeprom::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WdtBusEeprom::Enable)
    }
}
#[doc = "Линия прерывания ПДП\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<Dma> for bool {
    #[inline(always)]
    fn from(variant: Dma) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA` reader - Линия прерывания ПДП"]
pub type DmaR = crate::BitReader<Dma>;
impl DmaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma {
        match self.bits {
            false => Dma::Disable,
            true => Dma::Enable,
        }
    }
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dma::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dma::Enable
    }
}
#[doc = "Field `DMA` writer - Линия прерывания ПДП"]
pub type DmaW<'a, REG> = crate::BitWriter1C<'a, REG, Dma>;
impl<'a, REG> DmaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dma::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dma::Enable)
    }
}
#[doc = "Линия прерывания монитора частоты\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrequencyMonitor {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<FrequencyMonitor> for bool {
    #[inline(always)]
    fn from(variant: FrequencyMonitor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Frequency_monitor` reader - Линия прерывания монитора частоты"]
pub type FrequencyMonitorR = crate::BitReader<FrequencyMonitor>;
impl FrequencyMonitorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FrequencyMonitor {
        match self.bits {
            false => FrequencyMonitor::Disable,
            true => FrequencyMonitor::Enable,
        }
    }
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FrequencyMonitor::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FrequencyMonitor::Enable
    }
}
#[doc = "Field `Frequency_monitor` writer - Линия прерывания монитора частоты"]
pub type FrequencyMonitorW<'a, REG> = crate::BitWriter1C<'a, REG, FrequencyMonitor>;
impl<'a, REG> FrequencyMonitorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FrequencyMonitor::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FrequencyMonitor::Enable)
    }
}
#[doc = "Линия прерывания монитора напряжения AVCC (ниже порога)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PvdAvccUnder {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<PvdAvccUnder> for bool {
    #[inline(always)]
    fn from(variant: PvdAvccUnder) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVD_AVCC_under` reader - Линия прерывания монитора напряжения AVCC (ниже порога)"]
pub type PvdAvccUnderR = crate::BitReader<PvdAvccUnder>;
impl PvdAvccUnderR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PvdAvccUnder {
        match self.bits {
            false => PvdAvccUnder::Disable,
            true => PvdAvccUnder::Enable,
        }
    }
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PvdAvccUnder::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PvdAvccUnder::Enable
    }
}
#[doc = "Field `PVD_AVCC_under` writer - Линия прерывания монитора напряжения AVCC (ниже порога)"]
pub type PvdAvccUnderW<'a, REG> = crate::BitWriter1C<'a, REG, PvdAvccUnder>;
impl<'a, REG> PvdAvccUnderW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PvdAvccUnder::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PvdAvccUnder::Enable)
    }
}
#[doc = "Линия прерывания монитора напряжения AVCC (выше порога)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PvdAvccOver {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<PvdAvccOver> for bool {
    #[inline(always)]
    fn from(variant: PvdAvccOver) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVD_AVCC_over` reader - Линия прерывания монитора напряжения AVCC (выше порога)"]
pub type PvdAvccOverR = crate::BitReader<PvdAvccOver>;
impl PvdAvccOverR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PvdAvccOver {
        match self.bits {
            false => PvdAvccOver::Disable,
            true => PvdAvccOver::Enable,
        }
    }
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PvdAvccOver::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PvdAvccOver::Enable
    }
}
#[doc = "Field `PVD_AVCC_over` writer - Линия прерывания монитора напряжения AVCC (выше порога)"]
pub type PvdAvccOverW<'a, REG> = crate::BitWriter1C<'a, REG, PvdAvccOver>;
impl<'a, REG> PvdAvccOverW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PvdAvccOver::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PvdAvccOver::Enable)
    }
}
#[doc = "Линия прерывания монитора напряжения VCC (ниже порога)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PvdVccUnder {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<PvdVccUnder> for bool {
    #[inline(always)]
    fn from(variant: PvdVccUnder) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVD_VCC_under` reader - Линия прерывания монитора напряжения VCC (ниже порога)"]
pub type PvdVccUnderR = crate::BitReader<PvdVccUnder>;
impl PvdVccUnderR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PvdVccUnder {
        match self.bits {
            false => PvdVccUnder::Disable,
            true => PvdVccUnder::Enable,
        }
    }
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PvdVccUnder::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PvdVccUnder::Enable
    }
}
#[doc = "Field `PVD_VCC_under` writer - Линия прерывания монитора напряжения VCC (ниже порога)"]
pub type PvdVccUnderW<'a, REG> = crate::BitWriter1C<'a, REG, PvdVccUnder>;
impl<'a, REG> PvdVccUnderW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PvdVccUnder::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PvdVccUnder::Enable)
    }
}
#[doc = "Линия прерывания монитора напряжения VCC (выше порога)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PvdVccOver {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<PvdVccOver> for bool {
    #[inline(always)]
    fn from(variant: PvdVccOver) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVD_VCC_over` reader - Линия прерывания монитора напряжения VCC (выше порога)"]
pub type PvdVccOverR = crate::BitReader<PvdVccOver>;
impl PvdVccOverR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PvdVccOver {
        match self.bits {
            false => PvdVccOver::Disable,
            true => PvdVccOver::Enable,
        }
    }
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PvdVccOver::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PvdVccOver::Enable
    }
}
#[doc = "Field `PVD_VCC_over` writer - Линия прерывания монитора напряжения VCC (выше порога)"]
pub type PvdVccOverW<'a, REG> = crate::BitWriter1C<'a, REG, PvdVccOver>;
impl<'a, REG> PvdVccOverW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PvdVccOver::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PvdVccOver::Enable)
    }
}
#[doc = "Линия прерывания недостаточного напряжения батареи\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BatteryNonGood {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<BatteryNonGood> for bool {
    #[inline(always)]
    fn from(variant: BatteryNonGood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BATTERY_NON_GOOD` reader - Линия прерывания недостаточного напряжения батареи"]
pub type BatteryNonGoodR = crate::BitReader<BatteryNonGood>;
impl BatteryNonGoodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BatteryNonGood {
        match self.bits {
            false => BatteryNonGood::Disable,
            true => BatteryNonGood::Enable,
        }
    }
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BatteryNonGood::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BatteryNonGood::Enable
    }
}
#[doc = "Field `BATTERY_NON_GOOD` writer - Линия прерывания недостаточного напряжения батареи"]
pub type BatteryNonGoodW<'a, REG> = crate::BitWriter1C<'a, REG, BatteryNonGood>;
impl<'a, REG> BatteryNonGoodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(BatteryNonGood::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(BatteryNonGood::Enable)
    }
}
#[doc = "Линия прерывания BrouwnOut детектора\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bor {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<Bor> for bool {
    #[inline(always)]
    fn from(variant: Bor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOR` reader - Линия прерывания BrouwnOut детектора"]
pub type BorR = crate::BitReader<Bor>;
impl BorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bor {
        match self.bits {
            false => Bor::Disable,
            true => Bor::Enable,
        }
    }
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Bor::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Bor::Enable
    }
}
#[doc = "Field `BOR` writer - Линия прерывания BrouwnOut детектора"]
pub type BorW<'a, REG> = crate::BitWriter1C<'a, REG, Bor>;
impl<'a, REG> BorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Bor::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Bor::Enable)
    }
}
#[doc = "Линия прерывания монитора температуры\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsens {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<Tsens> for bool {
    #[inline(always)]
    fn from(variant: Tsens) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSENS` reader - Линия прерывания монитора температуры"]
pub type TsensR = crate::BitReader<Tsens>;
impl TsensR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsens {
        match self.bits {
            false => Tsens::Disable,
            true => Tsens::Enable,
        }
    }
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tsens::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tsens::Enable
    }
}
#[doc = "Field `TSENS` writer - Линия прерывания монитора температуры"]
pub type TsensW<'a, REG> = crate::BitWriter1C<'a, REG, Tsens>;
impl<'a, REG> TsensW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tsens::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tsens::Enable)
    }
}
#[doc = "Линия прерывания АЦП\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<Adc> for bool {
    #[inline(always)]
    fn from(variant: Adc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC` reader - Линия прерывания АЦП"]
pub type AdcR = crate::BitReader<Adc>;
impl AdcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc {
        match self.bits {
            false => Adc::Disable,
            true => Adc::Enable,
        }
    }
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Adc::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Adc::Enable
    }
}
#[doc = "Field `ADC` writer - Линия прерывания АЦП"]
pub type AdcW<'a, REG> = crate::BitWriter1C<'a, REG, Adc>;
impl<'a, REG> AdcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Adc::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Adc::Enable)
    }
}
#[doc = "Линия прерывания ЦАП0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dac0 {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<Dac0> for bool {
    #[inline(always)]
    fn from(variant: Dac0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC0` reader - Линия прерывания ЦАП0"]
pub type Dac0R = crate::BitReader<Dac0>;
impl Dac0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dac0 {
        match self.bits {
            false => Dac0::Disable,
            true => Dac0::Enable,
        }
    }
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dac0::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dac0::Enable
    }
}
#[doc = "Field `DAC0` writer - Линия прерывания ЦАП0"]
pub type Dac0W<'a, REG> = crate::BitWriter1C<'a, REG, Dac0>;
impl<'a, REG> Dac0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dac0::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dac0::Enable)
    }
}
#[doc = "Линия прерывания ЦАП1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dac1 {
    #[doc = "0: Прерывание по фронту запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание по фронту разрешено"]
    Enable = 1,
}
impl From<Dac1> for bool {
    #[inline(always)]
    fn from(variant: Dac1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC1` reader - Линия прерывания ЦАП1"]
pub type Dac1R = crate::BitReader<Dac1>;
impl Dac1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dac1 {
        match self.bits {
            false => Dac1::Disable,
            true => Dac1::Enable,
        }
    }
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dac1::Disable
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dac1::Enable
    }
}
#[doc = "Field `DAC1` writer - Линия прерывания ЦАП1"]
pub type Dac1W<'a, REG> = crate::BitWriter1C<'a, REG, Dac1>;
impl<'a, REG> Dac1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по фронту запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dac1::Disable)
    }
    #[doc = "Прерывание по фронту разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dac1::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Линия прерывания Timer32_0"]
    #[inline(always)]
    pub fn timer32_0(&self) -> Timer32_0R {
        Timer32_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Линия прерывания USART_0"]
    #[inline(always)]
    pub fn usart_0(&self) -> Usart0R {
        Usart0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Линия прерывания USART_1"]
    #[inline(always)]
    pub fn usart_1(&self) -> Usart1R {
        Usart1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Линия прерывания SPI_0"]
    #[inline(always)]
    pub fn spi_0(&self) -> Spi0R {
        Spi0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Линия прерывания SPI_1"]
    #[inline(always)]
    pub fn spi_1(&self) -> Spi1R {
        Spi1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Линия прерывания GPIO"]
    #[inline(always)]
    pub fn gpio(&self) -> GpioR {
        GpioR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Линия прерывания I2C_0"]
    #[inline(always)]
    pub fn i2c_0(&self) -> I2c0R {
        I2c0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Линия прерывания I2C_1"]
    #[inline(always)]
    pub fn i2c_1(&self) -> I2c1R {
        I2c1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Линия прерывания сторожевого таймера (WDT)"]
    #[inline(always)]
    pub fn wdt(&self) -> WdtR {
        WdtR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Линия прерывания Timer16_0"]
    #[inline(always)]
    pub fn timer16_0(&self) -> Timer16_0R {
        Timer16_0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Линия прерывания Timer16_1"]
    #[inline(always)]
    pub fn timer16_1(&self) -> Timer16_1R {
        Timer16_1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Линия прерывания Timer16_2"]
    #[inline(always)]
    pub fn timer16_2(&self) -> Timer16_2R {
        Timer16_2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Линия прерывания Timer32_1"]
    #[inline(always)]
    pub fn timer32_1(&self) -> Timer32_1R {
        Timer32_1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Линия прерывания Timer32_2"]
    #[inline(always)]
    pub fn timer32_2(&self) -> Timer32_2R {
        Timer32_2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Линия прерывания SPIFI"]
    #[inline(always)]
    pub fn spifi(&self) -> SpifiR {
        SpifiR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Линия прерывания RTC"]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Линия прерывания EEPROM"]
    #[inline(always)]
    pub fn eeprom(&self) -> EepromR {
        EepromR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Линия прерывания сторожевого таймера шины (периферийные устройства)"]
    #[inline(always)]
    pub fn wdt_bus_dom3(&self) -> WdtBusDom3R {
        WdtBusDom3R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Линия прерывания сторожевого таймера шины (SPIFI)"]
    #[inline(always)]
    pub fn wdt_bus_spifi(&self) -> WdtBusSpifiR {
        WdtBusSpifiR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Линия прерывания сторожевого таймера шины (EEPROM)"]
    #[inline(always)]
    pub fn wdt_bus_eeprom(&self) -> WdtBusEepromR {
        WdtBusEepromR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Линия прерывания ПДП"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Линия прерывания монитора частоты"]
    #[inline(always)]
    pub fn frequency_monitor(&self) -> FrequencyMonitorR {
        FrequencyMonitorR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Линия прерывания монитора напряжения AVCC (ниже порога)"]
    #[inline(always)]
    pub fn pvd_avcc_under(&self) -> PvdAvccUnderR {
        PvdAvccUnderR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Линия прерывания монитора напряжения AVCC (выше порога)"]
    #[inline(always)]
    pub fn pvd_avcc_over(&self) -> PvdAvccOverR {
        PvdAvccOverR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Линия прерывания монитора напряжения VCC (ниже порога)"]
    #[inline(always)]
    pub fn pvd_vcc_under(&self) -> PvdVccUnderR {
        PvdVccUnderR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Линия прерывания монитора напряжения VCC (выше порога)"]
    #[inline(always)]
    pub fn pvd_vcc_over(&self) -> PvdVccOverR {
        PvdVccOverR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Линия прерывания недостаточного напряжения батареи"]
    #[inline(always)]
    pub fn battery_non_good(&self) -> BatteryNonGoodR {
        BatteryNonGoodR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Линия прерывания BrouwnOut детектора"]
    #[inline(always)]
    pub fn bor(&self) -> BorR {
        BorR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Линия прерывания монитора температуры"]
    #[inline(always)]
    pub fn tsens(&self) -> TsensR {
        TsensR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Линия прерывания АЦП"]
    #[inline(always)]
    pub fn adc(&self) -> AdcR {
        AdcR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Линия прерывания ЦАП0"]
    #[inline(always)]
    pub fn dac0(&self) -> Dac0R {
        Dac0R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Линия прерывания ЦАП1"]
    #[inline(always)]
    pub fn dac1(&self) -> Dac1R {
        Dac1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Линия прерывания Timer32_0"]
    #[inline(always)]
    pub fn timer32_0(&mut self) -> Timer32_0W<MaskEdgeClearSpec> {
        Timer32_0W::new(self, 0)
    }
    #[doc = "Bit 1 - Линия прерывания USART_0"]
    #[inline(always)]
    pub fn usart_0(&mut self) -> Usart0W<MaskEdgeClearSpec> {
        Usart0W::new(self, 1)
    }
    #[doc = "Bit 2 - Линия прерывания USART_1"]
    #[inline(always)]
    pub fn usart_1(&mut self) -> Usart1W<MaskEdgeClearSpec> {
        Usart1W::new(self, 2)
    }
    #[doc = "Bit 3 - Линия прерывания SPI_0"]
    #[inline(always)]
    pub fn spi_0(&mut self) -> Spi0W<MaskEdgeClearSpec> {
        Spi0W::new(self, 3)
    }
    #[doc = "Bit 4 - Линия прерывания SPI_1"]
    #[inline(always)]
    pub fn spi_1(&mut self) -> Spi1W<MaskEdgeClearSpec> {
        Spi1W::new(self, 4)
    }
    #[doc = "Bit 5 - Линия прерывания GPIO"]
    #[inline(always)]
    pub fn gpio(&mut self) -> GpioW<MaskEdgeClearSpec> {
        GpioW::new(self, 5)
    }
    #[doc = "Bit 6 - Линия прерывания I2C_0"]
    #[inline(always)]
    pub fn i2c_0(&mut self) -> I2c0W<MaskEdgeClearSpec> {
        I2c0W::new(self, 6)
    }
    #[doc = "Bit 7 - Линия прерывания I2C_1"]
    #[inline(always)]
    pub fn i2c_1(&mut self) -> I2c1W<MaskEdgeClearSpec> {
        I2c1W::new(self, 7)
    }
    #[doc = "Bit 8 - Линия прерывания сторожевого таймера (WDT)"]
    #[inline(always)]
    pub fn wdt(&mut self) -> WdtW<MaskEdgeClearSpec> {
        WdtW::new(self, 8)
    }
    #[doc = "Bit 9 - Линия прерывания Timer16_0"]
    #[inline(always)]
    pub fn timer16_0(&mut self) -> Timer16_0W<MaskEdgeClearSpec> {
        Timer16_0W::new(self, 9)
    }
    #[doc = "Bit 10 - Линия прерывания Timer16_1"]
    #[inline(always)]
    pub fn timer16_1(&mut self) -> Timer16_1W<MaskEdgeClearSpec> {
        Timer16_1W::new(self, 10)
    }
    #[doc = "Bit 11 - Линия прерывания Timer16_2"]
    #[inline(always)]
    pub fn timer16_2(&mut self) -> Timer16_2W<MaskEdgeClearSpec> {
        Timer16_2W::new(self, 11)
    }
    #[doc = "Bit 12 - Линия прерывания Timer32_1"]
    #[inline(always)]
    pub fn timer32_1(&mut self) -> Timer32_1W<MaskEdgeClearSpec> {
        Timer32_1W::new(self, 12)
    }
    #[doc = "Bit 13 - Линия прерывания Timer32_2"]
    #[inline(always)]
    pub fn timer32_2(&mut self) -> Timer32_2W<MaskEdgeClearSpec> {
        Timer32_2W::new(self, 13)
    }
    #[doc = "Bit 14 - Линия прерывания SPIFI"]
    #[inline(always)]
    pub fn spifi(&mut self) -> SpifiW<MaskEdgeClearSpec> {
        SpifiW::new(self, 14)
    }
    #[doc = "Bit 15 - Линия прерывания RTC"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RtcW<MaskEdgeClearSpec> {
        RtcW::new(self, 15)
    }
    #[doc = "Bit 16 - Линия прерывания EEPROM"]
    #[inline(always)]
    pub fn eeprom(&mut self) -> EepromW<MaskEdgeClearSpec> {
        EepromW::new(self, 16)
    }
    #[doc = "Bit 17 - Линия прерывания сторожевого таймера шины (периферийные устройства)"]
    #[inline(always)]
    pub fn wdt_bus_dom3(&mut self) -> WdtBusDom3W<MaskEdgeClearSpec> {
        WdtBusDom3W::new(self, 17)
    }
    #[doc = "Bit 18 - Линия прерывания сторожевого таймера шины (SPIFI)"]
    #[inline(always)]
    pub fn wdt_bus_spifi(&mut self) -> WdtBusSpifiW<MaskEdgeClearSpec> {
        WdtBusSpifiW::new(self, 18)
    }
    #[doc = "Bit 19 - Линия прерывания сторожевого таймера шины (EEPROM)"]
    #[inline(always)]
    pub fn wdt_bus_eeprom(&mut self) -> WdtBusEepromW<MaskEdgeClearSpec> {
        WdtBusEepromW::new(self, 19)
    }
    #[doc = "Bit 20 - Линия прерывания ПДП"]
    #[inline(always)]
    pub fn dma(&mut self) -> DmaW<MaskEdgeClearSpec> {
        DmaW::new(self, 20)
    }
    #[doc = "Bit 21 - Линия прерывания монитора частоты"]
    #[inline(always)]
    pub fn frequency_monitor(&mut self) -> FrequencyMonitorW<MaskEdgeClearSpec> {
        FrequencyMonitorW::new(self, 21)
    }
    #[doc = "Bit 22 - Линия прерывания монитора напряжения AVCC (ниже порога)"]
    #[inline(always)]
    pub fn pvd_avcc_under(&mut self) -> PvdAvccUnderW<MaskEdgeClearSpec> {
        PvdAvccUnderW::new(self, 22)
    }
    #[doc = "Bit 23 - Линия прерывания монитора напряжения AVCC (выше порога)"]
    #[inline(always)]
    pub fn pvd_avcc_over(&mut self) -> PvdAvccOverW<MaskEdgeClearSpec> {
        PvdAvccOverW::new(self, 23)
    }
    #[doc = "Bit 24 - Линия прерывания монитора напряжения VCC (ниже порога)"]
    #[inline(always)]
    pub fn pvd_vcc_under(&mut self) -> PvdVccUnderW<MaskEdgeClearSpec> {
        PvdVccUnderW::new(self, 24)
    }
    #[doc = "Bit 25 - Линия прерывания монитора напряжения VCC (выше порога)"]
    #[inline(always)]
    pub fn pvd_vcc_over(&mut self) -> PvdVccOverW<MaskEdgeClearSpec> {
        PvdVccOverW::new(self, 25)
    }
    #[doc = "Bit 26 - Линия прерывания недостаточного напряжения батареи"]
    #[inline(always)]
    pub fn battery_non_good(&mut self) -> BatteryNonGoodW<MaskEdgeClearSpec> {
        BatteryNonGoodW::new(self, 26)
    }
    #[doc = "Bit 27 - Линия прерывания BrouwnOut детектора"]
    #[inline(always)]
    pub fn bor(&mut self) -> BorW<MaskEdgeClearSpec> {
        BorW::new(self, 27)
    }
    #[doc = "Bit 28 - Линия прерывания монитора температуры"]
    #[inline(always)]
    pub fn tsens(&mut self) -> TsensW<MaskEdgeClearSpec> {
        TsensW::new(self, 28)
    }
    #[doc = "Bit 29 - Линия прерывания АЦП"]
    #[inline(always)]
    pub fn adc(&mut self) -> AdcW<MaskEdgeClearSpec> {
        AdcW::new(self, 29)
    }
    #[doc = "Bit 30 - Линия прерывания ЦАП0"]
    #[inline(always)]
    pub fn dac0(&mut self) -> Dac0W<MaskEdgeClearSpec> {
        Dac0W::new(self, 30)
    }
    #[doc = "Bit 31 - Линия прерывания ЦАП1"]
    #[inline(always)]
    pub fn dac1(&mut self) -> Dac1W<MaskEdgeClearSpec> {
        Dac1W::new(self, 31)
    }
}
#[doc = "Установка маски прерываний по фронту\n\nYou can [`read`](crate::Reg::read) this register and get [`mask_edge_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_edge_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaskEdgeClearSpec;
impl crate::RegisterSpec for MaskEdgeClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mask_edge_clear::R`](R) reader structure"]
impl crate::Readable for MaskEdgeClearSpec {}
#[doc = "`write(|w| ..)` method takes [`mask_edge_clear::W`](W) writer structure"]
impl crate::Writable for MaskEdgeClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets MASK_EDGE_CLEAR to value 0"]
impl crate::Resettable for MaskEdgeClearSpec {
    const RESET_VALUE: u32 = 0;
}
