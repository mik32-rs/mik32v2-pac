#[doc = "Register `INT_ENABLE` writer"]
pub type W = crate::W<IntEnableSpec>;
#[doc = "Field `RX_OVERFLOW` writer - (IXR_RXOVR) Запись «1» устанавливает маску прерывания (разрешает прерывание) при переполнении при приеме"]
pub type RxOverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE_FAIL` writer - (IXR_MODF) Запись «1» устанавливает маску прерывания (разрешает прерывание) при нарушении режима"]
pub type ModeFailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_NOT_FULL` writer - (IXR_TXOW) Запись «1» устанавливает маску прерывания (разрешает прерывание) когда TX_FIFO не заполнен"]
pub type TxFifoNotFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_FULL` writer - (IXR_TXFULL) Запись «1» устанавливает маску прерывания (разрешает прерывание), когда TX_FIFO заполнен"]
pub type TxFifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_NOT_EMPTY` writer - (IXR_RXNEMPTY) Запись «1» устанавливает маску прерывания (разрешает прерывание), когда RX_FIFO не пустой"]
pub type RxFifoNotEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PX_FIFO_FULL` writer - (IXR_RXFULL) Запись «1» устанавливает маску прерывания (разрешает прерывание) при заполнении RX_FIFO"]
pub type PxFifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FIFO_UNDERFLOW` writer - (IXR_TXUF) Запись «1» устанавливает маску прерывания (разрешает прерывание) при опустошении TX_FIFO"]
pub type TxFifoUnderflowW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - (IXR_RXOVR) Запись «1» устанавливает маску прерывания (разрешает прерывание) при переполнении при приеме"]
    #[inline(always)]
    pub fn rx_overflow(&mut self) -> RxOverflowW<IntEnableSpec> {
        RxOverflowW::new(self, 0)
    }
    #[doc = "Bit 1 - (IXR_MODF) Запись «1» устанавливает маску прерывания (разрешает прерывание) при нарушении режима"]
    #[inline(always)]
    pub fn mode_fail(&mut self) -> ModeFailW<IntEnableSpec> {
        ModeFailW::new(self, 1)
    }
    #[doc = "Bit 2 - (IXR_TXOW) Запись «1» устанавливает маску прерывания (разрешает прерывание) когда TX_FIFO не заполнен"]
    #[inline(always)]
    pub fn tx_fifo_not_full(&mut self) -> TxFifoNotFullW<IntEnableSpec> {
        TxFifoNotFullW::new(self, 2)
    }
    #[doc = "Bit 3 - (IXR_TXFULL) Запись «1» устанавливает маску прерывания (разрешает прерывание), когда TX_FIFO заполнен"]
    #[inline(always)]
    pub fn tx_fifo_full(&mut self) -> TxFifoFullW<IntEnableSpec> {
        TxFifoFullW::new(self, 3)
    }
    #[doc = "Bit 4 - (IXR_RXNEMPTY) Запись «1» устанавливает маску прерывания (разрешает прерывание), когда RX_FIFO не пустой"]
    #[inline(always)]
    pub fn rx_fifo_not_empty(&mut self) -> RxFifoNotEmptyW<IntEnableSpec> {
        RxFifoNotEmptyW::new(self, 4)
    }
    #[doc = "Bit 5 - (IXR_RXFULL) Запись «1» устанавливает маску прерывания (разрешает прерывание) при заполнении RX_FIFO"]
    #[inline(always)]
    pub fn px_fifo_full(&mut self) -> PxFifoFullW<IntEnableSpec> {
        PxFifoFullW::new(self, 5)
    }
    #[doc = "Bit 6 - (IXR_TXUF) Запись «1» устанавливает маску прерывания (разрешает прерывание) при опустошении TX_FIFO"]
    #[inline(always)]
    pub fn tx_fifo_underflow(&mut self) -> TxFifoUnderflowW<IntEnableSpec> {
        TxFifoUnderflowW::new(self, 6)
    }
}
#[doc = "Регистр разрешения прерываний\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_enable::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEnableSpec;
impl crate::RegisterSpec for IntEnableSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_enable::W`](W) writer structure"]
impl crate::Writable for IntEnableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENABLE to value 0"]
impl crate::Resettable for IntEnableSpec {
    const RESET_VALUE: u32 = 0;
}
