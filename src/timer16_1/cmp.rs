#[doc = "Register `CMP` reader"]
pub type R = crate::R<CmpSpec>;
#[doc = "Register `CMP` writer"]
pub type W = crate::W<CmpSpec>;
#[doc = "Field `CMP` reader - Сравниваемое значение. CMP - это значение сравнения, используемое TIMER16. Примечание: Регистр CMP может быть изменен только тогда, когда TIMER16 включен (бит ENABLE установлен в '1')."]
pub type CmpR = crate::FieldReader<u16>;
#[doc = "Field `CMP` writer - Сравниваемое значение. CMP - это значение сравнения, используемое TIMER16. Примечание: Регистр CMP может быть изменен только тогда, когда TIMER16 включен (бит ENABLE установлен в '1')."]
pub type CmpW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Сравниваемое значение. CMP - это значение сравнения, используемое TIMER16. Примечание: Регистр CMP может быть изменен только тогда, когда TIMER16 включен (бит ENABLE установлен в '1')."]
    #[inline(always)]
    pub fn cmp(&self) -> CmpR {
        CmpR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Сравниваемое значение. CMP - это значение сравнения, используемое TIMER16. Примечание: Регистр CMP может быть изменен только тогда, когда TIMER16 включен (бит ENABLE установлен в '1')."]
    #[inline(always)]
    pub fn cmp(&mut self) -> CmpW<CmpSpec> {
        CmpW::new(self, 0)
    }
}
#[doc = "Регистр сравнения\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpSpec;
impl crate::RegisterSpec for CmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp::R`](R) reader structure"]
impl crate::Readable for CmpSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp::W`](W) writer structure"]
impl crate::Writable for CmpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP to value 0"]
impl crate::Resettable for CmpSpec {
    const RESET_VALUE: u32 = 0;
}
