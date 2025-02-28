#[doc = "Register `CH3_DST` reader"]
pub type R = crate::R<Ch3DstSpec>;
#[doc = "Register `CH3_DST` writer"]
pub type W = crate::W<Ch3DstSpec>;
#[doc = "Field `Dst` reader - Адрес назначения. В режиме чтения текущего статуса (Current_valuе=1) возвращает последнюю переданную подзадачу контроллера канала. В случае ошибки записи содержит указатель на текущий адрес мастер-интерфейса"]
pub type DstR = crate::FieldReader<u32>;
#[doc = "Field `Dst` writer - Адрес назначения. В режиме чтения текущего статуса (Current_valuе=1) возвращает последнюю переданную подзадачу контроллера канала. В случае ошибки записи содержит указатель на текущий адрес мастер-интерфейса"]
pub type DstW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Адрес назначения. В режиме чтения текущего статуса (Current_valuе=1) возвращает последнюю переданную подзадачу контроллера канала. В случае ошибки записи содержит указатель на текущий адрес мастер-интерфейса"]
    #[inline(always)]
    pub fn dst(&self) -> DstR {
        DstR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Адрес назначения. В режиме чтения текущего статуса (Current_valuе=1) возвращает последнюю переданную подзадачу контроллера канала. В случае ошибки записи содержит указатель на текущий адрес мастер-интерфейса"]
    #[inline(always)]
    pub fn dst(&mut self) -> DstW<Ch3DstSpec> {
        DstW::new(self, 0)
    }
}
#[doc = "Регистр адреса назначения канала 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_dst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_dst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch3DstSpec;
impl crate::RegisterSpec for Ch3DstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3_dst::R`](R) reader structure"]
impl crate::Readable for Ch3DstSpec {}
#[doc = "`write(|w| ..)` method takes [`ch3_dst::W`](W) writer structure"]
impl crate::Writable for Ch3DstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH3_DST to value 0"]
impl crate::Resettable for Ch3DstSpec {
    const RESET_VALUE: u32 = 0;
}
