#[doc = "Register `INT_MASK` reader"]
pub type R = crate::R<IntMaskSpec>;
#[doc = "(IXR_RXOVR) Текущее состояние маски прерывания при переполнении при приеме\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxOverflow {
    #[doc = "0: Прерывание запрещено"]
    Disable = 0,
    #[doc = "1: Прерывание разрешено"]
    Enable = 1,
}
impl From<RxOverflow> for bool {
    #[inline(always)]
    fn from(variant: RxOverflow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_OVERFLOW` reader - (IXR_RXOVR) Текущее состояние маски прерывания при переполнении при приеме"]
pub type RxOverflowR = crate::BitReader<RxOverflow>;
impl RxOverflowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxOverflow {
        match self.bits {
            false => RxOverflow::Disable,
            true => RxOverflow::Enable,
        }
    }
    #[doc = "Прерывание запрещено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RxOverflow::Disable
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RxOverflow::Enable
    }
}
#[doc = "(IXR_MODF) Текущее состояние маски прерывания при нарушении режима\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModeFail {
    #[doc = "0: Прерывание запрещено"]
    Disable = 0,
    #[doc = "1: Прерывание разрешено"]
    Enable = 1,
}
impl From<ModeFail> for bool {
    #[inline(always)]
    fn from(variant: ModeFail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE_FAIL` reader - (IXR_MODF) Текущее состояние маски прерывания при нарушении режима"]
pub type ModeFailR = crate::BitReader<ModeFail>;
impl ModeFailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ModeFail {
        match self.bits {
            false => ModeFail::Disable,
            true => ModeFail::Enable,
        }
    }
    #[doc = "Прерывание запрещено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ModeFail::Disable
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ModeFail::Enable
    }
}
#[doc = "(IXR_TXOW) Текущее состояние маски прерывания, когда TX_FIFO не заполнен\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxFifoNotFull {
    #[doc = "0: Прерывание запрещено"]
    Disable = 0,
    #[doc = "1: Прерывание разрешено"]
    Enable = 1,
}
impl From<TxFifoNotFull> for bool {
    #[inline(always)]
    fn from(variant: TxFifoNotFull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_FIFO_NOT_FULL` reader - (IXR_TXOW) Текущее состояние маски прерывания, когда TX_FIFO не заполнен"]
pub type TxFifoNotFullR = crate::BitReader<TxFifoNotFull>;
impl TxFifoNotFullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxFifoNotFull {
        match self.bits {
            false => TxFifoNotFull::Disable,
            true => TxFifoNotFull::Enable,
        }
    }
    #[doc = "Прерывание запрещено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TxFifoNotFull::Disable
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TxFifoNotFull::Enable
    }
}
#[doc = "(IXR_TXFULL) Текущее состояние маски прерывания, когда TX_FIFO заполнен\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxFifoFull {
    #[doc = "0: Прерывание запрещено"]
    Disable = 0,
    #[doc = "1: Прерывание разрешено"]
    Enable = 1,
}
impl From<TxFifoFull> for bool {
    #[inline(always)]
    fn from(variant: TxFifoFull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_FIFO_FULL` reader - (IXR_TXFULL) Текущее состояние маски прерывания, когда TX_FIFO заполнен"]
pub type TxFifoFullR = crate::BitReader<TxFifoFull>;
impl TxFifoFullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxFifoFull {
        match self.bits {
            false => TxFifoFull::Disable,
            true => TxFifoFull::Enable,
        }
    }
    #[doc = "Прерывание запрещено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TxFifoFull::Disable
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TxFifoFull::Enable
    }
}
#[doc = "(IXR_RXNEMPTY) Текущее состояние маски прерывания, когда RX_FIFO не пустой\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxFifoNotEmpty {
    #[doc = "0: Прерывание запрещено"]
    Disable = 0,
    #[doc = "1: Прерывание разрешено"]
    Enable = 1,
}
impl From<RxFifoNotEmpty> for bool {
    #[inline(always)]
    fn from(variant: RxFifoNotEmpty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX_FIFO_NOT_EMPTY` reader - (IXR_RXNEMPTY) Текущее состояние маски прерывания, когда RX_FIFO не пустой"]
pub type RxFifoNotEmptyR = crate::BitReader<RxFifoNotEmpty>;
impl RxFifoNotEmptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxFifoNotEmpty {
        match self.bits {
            false => RxFifoNotEmpty::Disable,
            true => RxFifoNotEmpty::Enable,
        }
    }
    #[doc = "Прерывание запрещено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RxFifoNotEmpty::Disable
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RxFifoNotEmpty::Enable
    }
}
#[doc = "(IXR_RXFULL) Текущее состояние маски прерывания при заполнении RX_FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PxFifoFull {
    #[doc = "0: Прерывание запрещено"]
    Disable = 0,
    #[doc = "1: Прерывание разрешено"]
    Enable = 1,
}
impl From<PxFifoFull> for bool {
    #[inline(always)]
    fn from(variant: PxFifoFull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PX_FIFO_FULL` reader - (IXR_RXFULL) Текущее состояние маски прерывания при заполнении RX_FIFO"]
pub type PxFifoFullR = crate::BitReader<PxFifoFull>;
impl PxFifoFullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PxFifoFull {
        match self.bits {
            false => PxFifoFull::Disable,
            true => PxFifoFull::Enable,
        }
    }
    #[doc = "Прерывание запрещено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PxFifoFull::Disable
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PxFifoFull::Enable
    }
}
#[doc = "(IXR_TXUF) Текущее состояние маски прерывания при опустошении\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxFifoUnderflow {
    #[doc = "0: Прерывание запрещено"]
    Disable = 0,
    #[doc = "1: Прерывание разрешено"]
    Enable = 1,
}
impl From<TxFifoUnderflow> for bool {
    #[inline(always)]
    fn from(variant: TxFifoUnderflow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX_FIFO_UNDERFLOW` reader - (IXR_TXUF) Текущее состояние маски прерывания при опустошении"]
pub type TxFifoUnderflowR = crate::BitReader<TxFifoUnderflow>;
impl TxFifoUnderflowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxFifoUnderflow {
        match self.bits {
            false => TxFifoUnderflow::Disable,
            true => TxFifoUnderflow::Enable,
        }
    }
    #[doc = "Прерывание запрещено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TxFifoUnderflow::Disable
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TxFifoUnderflow::Enable
    }
}
impl R {
    #[doc = "Bit 0 - (IXR_RXOVR) Текущее состояние маски прерывания при переполнении при приеме"]
    #[inline(always)]
    pub fn rx_overflow(&self) -> RxOverflowR {
        RxOverflowR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - (IXR_MODF) Текущее состояние маски прерывания при нарушении режима"]
    #[inline(always)]
    pub fn mode_fail(&self) -> ModeFailR {
        ModeFailR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - (IXR_TXOW) Текущее состояние маски прерывания, когда TX_FIFO не заполнен"]
    #[inline(always)]
    pub fn tx_fifo_not_full(&self) -> TxFifoNotFullR {
        TxFifoNotFullR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - (IXR_TXFULL) Текущее состояние маски прерывания, когда TX_FIFO заполнен"]
    #[inline(always)]
    pub fn tx_fifo_full(&self) -> TxFifoFullR {
        TxFifoFullR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - (IXR_RXNEMPTY) Текущее состояние маски прерывания, когда RX_FIFO не пустой"]
    #[inline(always)]
    pub fn rx_fifo_not_empty(&self) -> RxFifoNotEmptyR {
        RxFifoNotEmptyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - (IXR_RXFULL) Текущее состояние маски прерывания при заполнении RX_FIFO"]
    #[inline(always)]
    pub fn px_fifo_full(&self) -> PxFifoFullR {
        PxFifoFullR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - (IXR_TXUF) Текущее состояние маски прерывания при опустошении"]
    #[inline(always)]
    pub fn tx_fifo_underflow(&self) -> TxFifoUnderflowR {
        TxFifoUnderflowR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Регистр текущих масок прерываний\n\nYou can [`read`](crate::Reg::read) this register and get [`int_mask::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntMaskSpec;
impl crate::RegisterSpec for IntMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_mask::R`](R) reader structure"]
impl crate::Readable for IntMaskSpec {}
#[doc = "`reset()` method sets INT_MASK to value 0"]
impl crate::Resettable for IntMaskSpec {
    const RESET_VALUE: u32 = 0;
}
