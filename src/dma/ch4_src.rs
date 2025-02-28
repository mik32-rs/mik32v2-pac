#[doc = "Register `CH4_SRC` reader"]
pub type R = crate::R<Ch4SrcSpec>;
#[doc = "Register `CH4_SRC` writer"]
pub type W = crate::W<Ch4SrcSpec>;
#[doc = "Field `Src` reader - Адрес источника В режиме чтения текущего статуса (Current_valuе=1) возвращает текущую подзадачу контроллера канала. Указатель на адрес блока, который обрабатывается вычисляется как: Current_ch_read_task - 2^READ_BURST_SIZE В случае ошибки записи содержит указатель на текущий адрес мастер-интерфейса, вернувшего HRESP=1"]
pub type SrcR = crate::FieldReader<u32>;
#[doc = "Field `Src` writer - Адрес источника В режиме чтения текущего статуса (Current_valuе=1) возвращает текущую подзадачу контроллера канала. Указатель на адрес блока, который обрабатывается вычисляется как: Current_ch_read_task - 2^READ_BURST_SIZE В случае ошибки записи содержит указатель на текущий адрес мастер-интерфейса, вернувшего HRESP=1"]
pub type SrcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Адрес источника В режиме чтения текущего статуса (Current_valuе=1) возвращает текущую подзадачу контроллера канала. Указатель на адрес блока, который обрабатывается вычисляется как: Current_ch_read_task - 2^READ_BURST_SIZE В случае ошибки записи содержит указатель на текущий адрес мастер-интерфейса, вернувшего HRESP=1"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Адрес источника В режиме чтения текущего статуса (Current_valuе=1) возвращает текущую подзадачу контроллера канала. Указатель на адрес блока, который обрабатывается вычисляется как: Current_ch_read_task - 2^READ_BURST_SIZE В случае ошибки записи содержит указатель на текущий адрес мастер-интерфейса, вернувшего HRESP=1"]
    #[inline(always)]
    pub fn src(&mut self) -> SrcW<Ch4SrcSpec> {
        SrcW::new(self, 0)
    }
}
#[doc = "Регистр адреса источника канала 4\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_src::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_src::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch4SrcSpec;
impl crate::RegisterSpec for Ch4SrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4_src::R`](R) reader structure"]
impl crate::Readable for Ch4SrcSpec {}
#[doc = "`write(|w| ..)` method takes [`ch4_src::W`](W) writer structure"]
impl crate::Writable for Ch4SrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH4_SRC to value 0"]
impl crate::Resettable for Ch4SrcSpec {
    const RESET_VALUE: u32 = 0;
}
