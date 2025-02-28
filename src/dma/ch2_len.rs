#[doc = "Register `CH2_LEN` reader"]
pub type R = crate::R<Ch2LenSpec>;
#[doc = "Register `CH2_LEN` writer"]
pub type W = crate::W<Ch2LenSpec>;
#[doc = "Field `Data_Len` reader - Количество байт пересылки рассчитывается как LEN +1 В режиме чтения текущего статуса (Current_valuе=1) возвращает счётчик байт подзадач контроллера. Счётчик байт подзадач обновляется только по подзадачам записи. Кол-во байт в блоках, которые удалось записать вычисляется как: If (state==write): LEN - 2^WRITE_BURST_SIZE If (state==read): Current byte len"]
pub type DataLenR = crate::FieldReader<u32>;
#[doc = "Field `Data_Len` writer - Количество байт пересылки рассчитывается как LEN +1 В режиме чтения текущего статуса (Current_valuе=1) возвращает счётчик байт подзадач контроллера. Счётчик байт подзадач обновляется только по подзадачам записи. Кол-во байт в блоках, которые удалось записать вычисляется как: If (state==write): LEN - 2^WRITE_BURST_SIZE If (state==read): Current byte len"]
pub type DataLenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Количество байт пересылки рассчитывается как LEN +1 В режиме чтения текущего статуса (Current_valuе=1) возвращает счётчик байт подзадач контроллера. Счётчик байт подзадач обновляется только по подзадачам записи. Кол-во байт в блоках, которые удалось записать вычисляется как: If (state==write): LEN - 2^WRITE_BURST_SIZE If (state==read): Current byte len"]
    #[inline(always)]
    pub fn data_len(&self) -> DataLenR {
        DataLenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Количество байт пересылки рассчитывается как LEN +1 В режиме чтения текущего статуса (Current_valuе=1) возвращает счётчик байт подзадач контроллера. Счётчик байт подзадач обновляется только по подзадачам записи. Кол-во байт в блоках, которые удалось записать вычисляется как: If (state==write): LEN - 2^WRITE_BURST_SIZE If (state==read): Current byte len"]
    #[inline(always)]
    pub fn data_len(&mut self) -> DataLenW<Ch2LenSpec> {
        DataLenW::new(self, 0)
    }
}
#[doc = "Регистр размера передаваемых данных канала 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch2LenSpec;
impl crate::RegisterSpec for Ch2LenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2_len::R`](R) reader structure"]
impl crate::Readable for Ch2LenSpec {}
#[doc = "`write(|w| ..)` method takes [`ch2_len::W`](W) writer structure"]
impl crate::Writable for Ch2LenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH2_LEN to value 0"]
impl crate::Resettable for Ch2LenSpec {
    const RESET_VALUE: u32 = 0;
}
