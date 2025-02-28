#[doc = "Register `DPF_VALUE` reader"]
pub type R = crate::R<DpfValueSpec>;
#[doc = "Register `DPF_VALUE` writer"]
pub type W = crate::W<DpfValueSpec>;
#[doc = "Field `DPF` reader - Коэффициент, определяющий длительность отфильтровываемых импульсов (на системной частоте)"]
pub type DpfR = crate::FieldReader<u16>;
#[doc = "Field `DPF` writer - Коэффициент, определяющий длительность отфильтровываемых импульсов (на системной частоте)"]
pub type DpfW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Коэффициент, определяющий длительность отфильтровываемых импульсов (на системной частоте)"]
    #[inline(always)]
    pub fn dpf(&self) -> DpfR {
        DpfR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Коэффициент, определяющий длительность отфильтровываемых импульсов (на системной частоте)"]
    #[inline(always)]
    pub fn dpf(&mut self) -> DpfW<DpfValueSpec> {
        DpfW::new(self, 0)
    }
}
#[doc = "Настройка цифрового фильтра\n\nYou can [`read`](crate::Reg::read) this register and get [`dpf_value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpf_value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpfValueSpec;
impl crate::RegisterSpec for DpfValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpf_value::R`](R) reader structure"]
impl crate::Readable for DpfValueSpec {}
#[doc = "`write(|w| ..)` method takes [`dpf_value::W`](W) writer structure"]
impl crate::Writable for DpfValueSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPF_VALUE to value 0x05dc"]
impl crate::Resettable for DpfValueSpec {
    const RESET_VALUE: u32 = 0x05dc;
}
