#[doc = "Register `IRQ_CLEAR` writer"]
pub type W = crate::W<IrqClearSpec>;
#[doc = "Field `DOM3` writer - Сброс прерываний для монитора шины периферийных устройств"]
pub type Dom3W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SPIFI` writer - Сброс прерываний для монитора шины SPIFI"]
pub type SpifiW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EEPROM` writer - Сброс прерываний для монитора шины EEPROM"]
pub type EepromW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl W {
    #[doc = "Bit 0 - Сброс прерываний для монитора шины периферийных устройств"]
    #[inline(always)]
    pub fn dom3(&mut self) -> Dom3W<IrqClearSpec> {
        Dom3W::new(self, 0)
    }
    #[doc = "Bit 1 - Сброс прерываний для монитора шины SPIFI"]
    #[inline(always)]
    pub fn spifi(&mut self) -> SpifiW<IrqClearSpec> {
        SpifiW::new(self, 1)
    }
    #[doc = "Bit 2 - Сброс прерываний для монитора шины EEPROM"]
    #[inline(always)]
    pub fn eeprom(&mut self) -> EepromW<IrqClearSpec> {
        EepromW::new(self, 2)
    }
}
#[doc = "Сброс прерываний\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_clear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqClearSpec;
impl crate::RegisterSpec for IrqClearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`irq_clear::W`](W) writer structure"]
impl crate::Writable for IrqClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x07;
}
#[doc = "`reset()` method sets IRQ_CLEAR to value 0"]
impl crate::Resettable for IrqClearSpec {
    const RESET_VALUE: u32 = 0;
}
