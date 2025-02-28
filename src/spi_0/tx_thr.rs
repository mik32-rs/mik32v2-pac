#[doc = "Register `TX_THR` reader"]
pub type R = crate::R<TxThrSpec>;
#[doc = "Register `TX_THR` writer"]
pub type W = crate::W<TxThrSpec>;
#[doc = "Field `Threshold_of_TX_FIFO` reader - Задает уровень, при котором TX_FIFO считается не заполненным и формируется прерывание"]
pub type ThresholdOfTxFifoR = crate::FieldReader<u32>;
#[doc = "Field `Threshold_of_TX_FIFO` writer - Задает уровень, при котором TX_FIFO считается не заполненным и формируется прерывание"]
pub type ThresholdOfTxFifoW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Задает уровень, при котором TX_FIFO считается не заполненным и формируется прерывание"]
    #[inline(always)]
    pub fn threshold_of_tx_fifo(&self) -> ThresholdOfTxFifoR {
        ThresholdOfTxFifoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Задает уровень, при котором TX_FIFO считается не заполненным и формируется прерывание"]
    #[inline(always)]
    pub fn threshold_of_tx_fifo(&mut self) -> ThresholdOfTxFifoW<TxThrSpec> {
        ThresholdOfTxFifoW::new(self, 0)
    }
}
#[doc = "Регистр пороговых значений TX_FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_thr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_thr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxThrSpec;
impl crate::RegisterSpec for TxThrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_thr::R`](R) reader structure"]
impl crate::Readable for TxThrSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_thr::W`](W) writer structure"]
impl crate::Writable for TxThrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX_THR to value 0x01"]
impl crate::Resettable for TxThrSpec {
    const RESET_VALUE: u32 = 0x01;
}
