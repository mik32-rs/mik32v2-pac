#[doc = "Register `TXDATA` reader"]
pub type R = crate::R<TxdataSpec>;
#[doc = "Register `TXDATA` writer"]
pub type W = crate::W<TxdataSpec>;
#[doc = "Field `TDR` reader - Передаваемые данные"]
pub type TdrR = crate::FieldReader<u16>;
#[doc = "Field `TDR` writer - Передаваемые данные"]
pub type TdrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Передаваемые данные"]
    #[inline(always)]
    pub fn tdr(&self) -> TdrR {
        TdrR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Передаваемые данные"]
    #[inline(always)]
    pub fn tdr(&mut self) -> TdrW<TxdataSpec> {
        TdrW::new(self, 0)
    }
}
#[doc = "Регистр передаваемых данных\n\nYou can [`read`](crate::Reg::read) this register and get [`txdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdataSpec;
impl crate::RegisterSpec for TxdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdata::R`](R) reader structure"]
impl crate::Readable for TxdataSpec {}
#[doc = "`write(|w| ..)` method takes [`txdata::W`](W) writer structure"]
impl crate::Writable for TxdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDATA to value 0"]
impl crate::Resettable for TxdataSpec {
    const RESET_VALUE: u32 = 0;
}
