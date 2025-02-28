#[doc = "Register `DAC1_VALUE` reader"]
pub type R = crate::R<Dac1ValueSpec>;
#[doc = "Register `DAC1_VALUE` writer"]
pub type W = crate::W<Dac1ValueSpec>;
#[doc = "Field `VALUE` reader - Входные данные для прерывания"]
pub type ValueR = crate::FieldReader<u16>;
#[doc = "Field `VALUE` writer - Входные данные для прерывания"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Входные данные для прерывания"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Входные данные для прерывания"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<Dac1ValueSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Входные данные для ЦАП1\n\nYou can [`read`](crate::Reg::read) this register and get [`dac1_value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac1_value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dac1ValueSpec;
impl crate::RegisterSpec for Dac1ValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac1_value::R`](R) reader structure"]
impl crate::Readable for Dac1ValueSpec {}
#[doc = "`write(|w| ..)` method takes [`dac1_value::W`](W) writer structure"]
impl crate::Writable for Dac1ValueSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC1_VALUE to value 0"]
impl crate::Resettable for Dac1ValueSpec {
    const RESET_VALUE: u32 = 0;
}
