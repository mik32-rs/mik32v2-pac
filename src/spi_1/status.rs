#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "(IXR_RXOVR) Прерывание при переполнении RX_FIFO, значение сбрасывается при чтении\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxOverflow {
    #[doc = "0: Нет переполнения"]
    Ok = 0,
    #[doc = "1: Переполнение RX_FIFO"]
    Overflow = 1,
}
impl From<RxOverflow> for bool {
    #[inline(always)]
    fn from(variant: RxOverflow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_OVERFLOW` reader - (IXR_RXOVR) Прерывание при переполнении RX_FIFO, значение сбрасывается при чтении"]
pub type RxOverflowR = crate::BitReader<RxOverflow>;
impl RxOverflowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxOverflow {
        match self.bits {
            false => RxOverflow::Ok,
            true => RxOverflow::Overflow,
        }
    }
    #[doc = "Нет переполнения"]
    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        *self == RxOverflow::Ok
    }
    #[doc = "Переполнение RX_FIFO"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == RxOverflow::Overflow
    }
}
#[doc = "(IXR_MODF) Напряжение на выводе n_ss_in не соответствую режиму работы SPI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModeFail {
    #[doc = "0: Корректная работа"]
    Ok = 0,
    #[doc = "1: n_ss_in имеет низкий уровень в режиме ведомого устройства"]
    Fail = 1,
}
impl From<ModeFail> for bool {
    #[inline(always)]
    fn from(variant: ModeFail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE_FAIL` reader - (IXR_MODF) Напряжение на выводе n_ss_in не соответствую режиму работы SPI"]
pub type ModeFailR = crate::BitReader<ModeFail>;
impl ModeFailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ModeFail {
        match self.bits {
            false => ModeFail::Ok,
            true => ModeFail::Fail,
        }
    }
    #[doc = "Корректная работа"]
    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        *self == ModeFail::Ok
    }
    #[doc = "n_ss_in имеет низкий уровень в режиме ведомого устройства"]
    #[inline(always)]
    pub fn is_fail(&self) -> bool {
        *self == ModeFail::Fail
    }
}
#[doc = "(IXR_TXOW) Регистр TX_FIFO не заполнен\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxFifoNotFull {
    #[doc = "0: Регистр заполнен до значение THRESHOLD или больше"]
    LessThanTheThreshold = 0,
    #[doc = "1: Регистр заполнен меньше чем THRESHOLD"]
    MoreThanTheThreshold = 1,
}
impl From<TxFifoNotFull> for bool {
    #[inline(always)]
    fn from(variant: TxFifoNotFull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_FIFO_NOT_FULL` reader - (IXR_TXOW) Регистр TX_FIFO не заполнен"]
pub type TxFifoNotFullR = crate::BitReader<TxFifoNotFull>;
impl TxFifoNotFullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxFifoNotFull {
        match self.bits {
            false => TxFifoNotFull::LessThanTheThreshold,
            true => TxFifoNotFull::MoreThanTheThreshold,
        }
    }
    #[doc = "Регистр заполнен до значение THRESHOLD или больше"]
    #[inline(always)]
    pub fn is_less_than_the_threshold(&self) -> bool {
        *self == TxFifoNotFull::LessThanTheThreshold
    }
    #[doc = "Регистр заполнен меньше чем THRESHOLD"]
    #[inline(always)]
    pub fn is_more_than_the_threshold(&self) -> bool {
        *self == TxFifoNotFull::MoreThanTheThreshold
    }
}
#[doc = "(IXR_TXFULL) Регистр TX_FIFO заполнен\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxFifoFull {
    #[doc = "0: FIFO не заполнен"]
    NotFull = 0,
    #[doc = "1: FIFO заполнен"]
    Full = 1,
}
impl From<TxFifoFull> for bool {
    #[inline(always)]
    fn from(variant: TxFifoFull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_FIFO_FULL` reader - (IXR_TXFULL) Регистр TX_FIFO заполнен"]
pub type TxFifoFullR = crate::BitReader<TxFifoFull>;
impl TxFifoFullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxFifoFull {
        match self.bits {
            false => TxFifoFull::NotFull,
            true => TxFifoFull::Full,
        }
    }
    #[doc = "FIFO не заполнен"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == TxFifoFull::NotFull
    }
    #[doc = "FIFO заполнен"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == TxFifoFull::Full
    }
}
#[doc = "(IXR_RXNEMPTY) Регистр RX_FIFO не пустой\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxFifoNotEmpty {
    #[doc = "0: FIFO пустой"]
    Empty = 0,
    #[doc = "1: В FIFO есть хотя бы один байт"]
    NotEmpty = 1,
}
impl From<RxFifoNotEmpty> for bool {
    #[inline(always)]
    fn from(variant: RxFifoNotEmpty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_FIFO_NOT_EMPTY` reader - (IXR_RXNEMPTY) Регистр RX_FIFO не пустой"]
pub type RxFifoNotEmptyR = crate::BitReader<RxFifoNotEmpty>;
impl RxFifoNotEmptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxFifoNotEmpty {
        match self.bits {
            false => RxFifoNotEmpty::Empty,
            true => RxFifoNotEmpty::NotEmpty,
        }
    }
    #[doc = "FIFO пустой"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RxFifoNotEmpty::Empty
    }
    #[doc = "В FIFO есть хотя бы один байт"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RxFifoNotEmpty::NotEmpty
    }
}
#[doc = "(IXR_RXFULL) Регистр RX_FIFO заполнен\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxFifoFull {
    #[doc = "0: FIFO не заполнен"]
    NotFull = 0,
    #[doc = "1: FIFO заполнен"]
    Full = 1,
}
impl From<RxFifoFull> for bool {
    #[inline(always)]
    fn from(variant: RxFifoFull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_FIFO_FULL` reader - (IXR_RXFULL) Регистр RX_FIFO заполнен"]
pub type RxFifoFullR = crate::BitReader<RxFifoFull>;
impl RxFifoFullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxFifoFull {
        match self.bits {
            false => RxFifoFull::NotFull,
            true => RxFifoFull::Full,
        }
    }
    #[doc = "FIFO не заполнен"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == RxFifoFull::NotFull
    }
    #[doc = "FIFO заполнен"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RxFifoFull::Full
    }
}
#[doc = "(IXR_TXUF) Регистр TX FIFO опустошен. Устанавливается в режиме ведомого в случае, если к началу обмена в TX_FIFO нет данных\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxFifoUnderflow {
    #[doc = "0: Опустошение не детектируется"]
    NotEmpty = 0,
    #[doc = "1: Определение опустошения"]
    Underflow = 1,
}
impl From<TxFifoUnderflow> for bool {
    #[inline(always)]
    fn from(variant: TxFifoUnderflow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_FIFO_UNDERFLOW` reader - (IXR_TXUF) Регистр TX FIFO опустошен. Устанавливается в режиме ведомого в случае, если к началу обмена в TX_FIFO нет данных"]
pub type TxFifoUnderflowR = crate::BitReader<TxFifoUnderflow>;
impl TxFifoUnderflowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxFifoUnderflow {
        match self.bits {
            false => TxFifoUnderflow::NotEmpty,
            true => TxFifoUnderflow::Underflow,
        }
    }
    #[doc = "Опустошение не детектируется"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TxFifoUnderflow::NotEmpty
    }
    #[doc = "Определение опустошения"]
    #[inline(always)]
    pub fn is_underflow(&self) -> bool {
        *self == TxFifoUnderflow::Underflow
    }
}
#[doc = "Статус сеанса передачи\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpiActive {
    #[doc = "0: Контроллер SPI в состоянии ожидания обмена"]
    Ready = 0,
    #[doc = "1: Контроллер SPI в процессе обмена"]
    Busy = 1,
}
impl From<SpiActive> for bool {
    #[inline(always)]
    fn from(variant: SpiActive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI_ACTIVE` reader - Статус сеанса передачи"]
pub type SpiActiveR = crate::BitReader<SpiActive>;
impl SpiActiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpiActive {
        match self.bits {
            false => SpiActive::Ready,
            true => SpiActive::Busy,
        }
    }
    #[doc = "Контроллер SPI в состоянии ожидания обмена"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == SpiActive::Ready
    }
    #[doc = "Контроллер SPI в процессе обмена"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == SpiActive::Busy
    }
}
impl R {
    #[doc = "Bit 0 - (IXR_RXOVR) Прерывание при переполнении RX_FIFO, значение сбрасывается при чтении"]
    #[inline(always)]
    pub fn rx_overflow(&self) -> RxOverflowR {
        RxOverflowR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - (IXR_MODF) Напряжение на выводе n_ss_in не соответствую режиму работы SPI"]
    #[inline(always)]
    pub fn mode_fail(&self) -> ModeFailR {
        ModeFailR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - (IXR_TXOW) Регистр TX_FIFO не заполнен"]
    #[inline(always)]
    pub fn tx_fifo_not_full(&self) -> TxFifoNotFullR {
        TxFifoNotFullR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - (IXR_TXFULL) Регистр TX_FIFO заполнен"]
    #[inline(always)]
    pub fn tx_fifo_full(&self) -> TxFifoFullR {
        TxFifoFullR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - (IXR_RXNEMPTY) Регистр RX_FIFO не пустой"]
    #[inline(always)]
    pub fn rx_fifo_not_empty(&self) -> RxFifoNotEmptyR {
        RxFifoNotEmptyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - (IXR_RXFULL) Регистр RX_FIFO заполнен"]
    #[inline(always)]
    pub fn rx_fifo_full(&self) -> RxFifoFullR {
        RxFifoFullR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - (IXR_TXUF) Регистр TX FIFO опустошен. Устанавливается в режиме ведомого в случае, если к началу обмена в TX_FIFO нет данных"]
    #[inline(always)]
    pub fn tx_fifo_underflow(&self) -> TxFifoUnderflowR {
        TxFifoUnderflowR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 15 - Статус сеанса передачи"]
    #[inline(always)]
    pub fn spi_active(&self) -> SpiActiveR {
        SpiActiveR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {}
#[doc = "Регистр статуса. Примечание: биты регистра \\[6:0\\]
устанавливаются в «1», если произошло событие вызывающее прерывание.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0x04"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0x04;
}
