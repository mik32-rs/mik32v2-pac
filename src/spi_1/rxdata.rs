#[doc = "Register `RXDATA` reader"]
pub type R = crate::R<RxdataSpec>;
#[doc = "Field `RX_FIFO_data` reader - Данные из RX_FIFO."]
pub type RxFifoDataR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Данные из RX_FIFO."]
    #[inline(always)]
    pub fn rx_fifo_data(&self) -> RxFifoDataR {
        RxFifoDataR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Регистр принимаемых данных\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdataSpec;
impl crate::RegisterSpec for RxdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdata::R`](R) reader structure"]
impl crate::Readable for RxdataSpec {}
#[doc = "`reset()` method sets RXDATA to value 0"]
impl crate::Resettable for RxdataSpec {
    const RESET_VALUE: u32 = 0;
}
