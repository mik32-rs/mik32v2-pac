#[doc = "Register `CH2_DST` reader"]
pub type R = crate::R<Ch2DstSpec>;
#[doc = "Register `CH2_DST` writer"]
pub type W = crate::W<Ch2DstSpec>;
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
    pub fn dst(&mut self) -> DstW<Ch2DstSpec> {
        DstW::new(self, 0)
    }
}
#[doc = "Регистр адреса назначения канала 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_dst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_dst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch2DstSpec;
impl crate::RegisterSpec for Ch2DstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2_dst::R`](R) reader structure"]
impl crate::Readable for Ch2DstSpec {}
#[doc = "`write(|w| ..)` method takes [`ch2_dst::W`](W) writer structure"]
impl crate::Writable for Ch2DstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH2_DST to value 0"]
impl crate::Resettable for Ch2DstSpec {
    const RESET_VALUE: u32 = 0;
}
