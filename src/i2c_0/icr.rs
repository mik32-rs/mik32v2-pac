#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `ADDRCF` writer - Сброс флага соответствия адреса"]
pub type AddrcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACKCF` writer - Сброс флага «не получено подтверждение» (NACK)"]
pub type NackcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPCF` writer - Сброс флага детектирования STOP на шине"]
pub type StopcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERRCF` writer - Сброс флага ошибки шины"]
pub type BerrcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARLOCF` writer - Сброс флага проигрыша арбитража"]
pub type ArlocfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRCF` writer - Сброс флага прерывания переполнения/недозагрузки"]
pub type OvrcfW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 3 - Сброс флага соответствия адреса"]
    #[inline(always)]
    pub fn addrcf(&mut self) -> AddrcfW<IcrSpec> {
        AddrcfW::new(self, 3)
    }
    #[doc = "Bit 4 - Сброс флага «не получено подтверждение» (NACK)"]
    #[inline(always)]
    pub fn nackcf(&mut self) -> NackcfW<IcrSpec> {
        NackcfW::new(self, 4)
    }
    #[doc = "Bit 5 - Сброс флага детектирования STOP на шине"]
    #[inline(always)]
    pub fn stopcf(&mut self) -> StopcfW<IcrSpec> {
        StopcfW::new(self, 5)
    }
    #[doc = "Bit 8 - Сброс флага ошибки шины"]
    #[inline(always)]
    pub fn berrcf(&mut self) -> BerrcfW<IcrSpec> {
        BerrcfW::new(self, 8)
    }
    #[doc = "Bit 9 - Сброс флага проигрыша арбитража"]
    #[inline(always)]
    pub fn arlocf(&mut self) -> ArlocfW<IcrSpec> {
        ArlocfW::new(self, 9)
    }
    #[doc = "Bit 10 - Сброс флага прерывания переполнения/недозагрузки"]
    #[inline(always)]
    pub fn ovrcf(&mut self) -> OvrcfW<IcrSpec> {
        OvrcfW::new(self, 10)
    }
}
#[doc = "Регистр сроса флагов прерываний\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {
    const RESET_VALUE: u32 = 0;
}
