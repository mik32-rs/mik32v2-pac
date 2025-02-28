#[doc = "Register `ENABLE` reader"]
pub type R = crate::R<EnableSpec>;
#[doc = "Register `ENABLE` writer"]
pub type W = crate::W<EnableSpec>;
#[doc = "Запуск/остановка работы счетчика\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimEn {
    #[doc = "0: Счетчик выключен"]
    Disable = 0,
    #[doc = "1: Счетчик работает"]
    Enable = 1,
}
impl From<TimEn> for bool {
    #[inline(always)]
    fn from(variant: TimEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIM_EN` reader - Запуск/остановка работы счетчика"]
pub type TimEnR = crate::BitReader<TimEn>;
impl TimEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TimEn {
        match self.bits {
            false => TimEn::Disable,
            true => TimEn::Enable,
        }
    }
    #[doc = "Счетчик выключен"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TimEn::Disable
    }
    #[doc = "Счетчик работает"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TimEn::Enable
    }
}
#[doc = "Field `TIM_EN` writer - Запуск/остановка работы счетчика"]
pub type TimEnW<'a, REG> = crate::BitWriter<'a, REG, TimEn>;
impl<'a, REG> TimEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Счетчик выключен"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TimEn::Disable)
    }
    #[doc = "Счетчик работает"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TimEn::Enable)
    }
}
#[doc = "Field `TIM_CLR` writer - Сброс (обнуление) текущего значения счетчика при записи «1»"]
pub type TimClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Запуск/остановка работы счетчика"]
    #[inline(always)]
    pub fn tim_en(&self) -> TimEnR {
        TimEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Запуск/остановка работы счетчика"]
    #[inline(always)]
    pub fn tim_en(&mut self) -> TimEnW<EnableSpec> {
        TimEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Сброс (обнуление) текущего значения счетчика при записи «1»"]
    #[inline(always)]
    pub fn tim_clr(&mut self) -> TimClrW<EnableSpec> {
        TimClrW::new(self, 1)
    }
}
#[doc = "Регистр включения таймера\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnableSpec;
impl crate::RegisterSpec for EnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable::R`](R) reader structure"]
impl crate::Readable for EnableSpec {}
#[doc = "`write(|w| ..)` method takes [`enable::W`](W) writer structure"]
impl crate::Writable for EnableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENABLE to value 0"]
impl crate::Resettable for EnableSpec {
    const RESET_VALUE: u32 = 0;
}
