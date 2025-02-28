#[doc = "Register `CNT` reader"]
pub type R = crate::R<CntSpec>;
#[doc = "Register `CNT` writer"]
pub type W = crate::W<CntSpec>;
#[doc = "Field `CNT` reader - Значение счетчика. Когда TIMER16 работает с асинхронными тактовыми сигналами, чтение регистра CNT мо-жет вернуть недостоверные значения. Поэтому в этом случае необходимо выполнить два последовательных до-ступа на чтение и убедиться, что два возвращенных значения идентичны."]
pub type CntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Значение счетчика. Когда TIMER16 работает с асинхронными тактовыми сигналами, чтение регистра CNT мо-жет вернуть недостоверные значения. Поэтому в этом случае необходимо выполнить два последовательных до-ступа на чтение и убедиться, что два возвращенных значения идентичны."]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Регистр счётчика\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntSpec;
impl crate::RegisterSpec for CntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt::R`](R) reader structure"]
impl crate::Readable for CntSpec {}
#[doc = "`write(|w| ..)` method takes [`cnt::W`](W) writer structure"]
impl crate::Writable for CntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::Resettable for CntSpec {
    const RESET_VALUE: u32 = 0;
}
