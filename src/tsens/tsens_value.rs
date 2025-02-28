#[doc = "Register `TSENS_VALUE` reader"]
pub type R = crate::R<TsensValueSpec>;
#[doc = "Field `VALUE` reader - Последнее измереннное значение сенсором"]
pub type ValueR = crate::FieldReader<u16>;
#[doc = "Field `EOC` reader - Текущее значение выхода окончания преобразования"]
pub type EocR = crate::BitReader;
impl R {
    #[doc = "Bits 0:9 - Последнее измереннное значение сенсором"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Текущее значение выхода окончания преобразования"]
    #[inline(always)]
    pub fn eoc(&self) -> EocR {
        EocR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Регистр данных\n\nYou can [`read`](crate::Reg::read) this register and get [`tsens_value::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsensValueSpec;
impl crate::RegisterSpec for TsensValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsens_value::R`](R) reader structure"]
impl crate::Readable for TsensValueSpec {}
#[doc = "`reset()` method sets TSENS_VALUE to value 0"]
impl crate::Resettable for TsensValueSpec {
    const RESET_VALUE: u32 = 0;
}
