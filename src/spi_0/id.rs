#[doc = "Register `ID` reader"]
pub type R = crate::R<IdSpec>;
#[doc = "Field `Threshold_of_TX_FIFO` reader - Идентификационный номер модуля"]
pub type ThresholdOfTxFifoR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Идентификационный номер модуля"]
    #[inline(always)]
    pub fn threshold_of_tx_fifo(&self) -> ThresholdOfTxFifoR {
        ThresholdOfTxFifoR::new(self.bits)
    }
}
#[doc = "Идентификационный номер модуля\n\nYou can [`read`](crate::Reg::read) this register and get [`id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdSpec;
impl crate::RegisterSpec for IdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id::R`](R) reader structure"]
impl crate::Readable for IdSpec {}
#[doc = "`reset()` method sets ID to value 0x0109_0100"]
impl crate::Resettable for IdSpec {
    const RESET_VALUE: u32 = 0x0109_0100;
}
