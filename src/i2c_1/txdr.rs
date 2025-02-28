#[doc = "Register `TXDR` reader"]
pub type R = crate::R<TxdrSpec>;
#[doc = "Register `TXDR` writer"]
pub type W = crate::W<TxdrSpec>;
#[doc = "Field `TXDATA` reader - Буфер передаваемых данных"]
pub type TxdataR = crate::FieldReader;
#[doc = "Field `TXDATA` writer - Буфер передаваемых данных"]
pub type TxdataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Буфер передаваемых данных"]
    #[inline(always)]
    pub fn txdata(&self) -> TxdataR {
        TxdataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Буфер передаваемых данных"]
    #[inline(always)]
    pub fn txdata(&mut self) -> TxdataW<TxdrSpec> {
        TxdataW::new(self, 0)
    }
}
#[doc = "Регистр передаваемых данных\n\nYou can [`read`](crate::Reg::read) this register and get [`txdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdrSpec;
impl crate::RegisterSpec for TxdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdr::R`](R) reader structure"]
impl crate::Readable for TxdrSpec {}
#[doc = "`write(|w| ..)` method takes [`txdr::W`](W) writer structure"]
impl crate::Writable for TxdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDR to value 0"]
impl crate::Resettable for TxdrSpec {
    const RESET_VALUE: u32 = 0;
}
