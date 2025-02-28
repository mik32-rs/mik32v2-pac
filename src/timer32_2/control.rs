#[doc = "Register `CONTROL` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `CONTROL` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Режим счёта таймера\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CountMode {
    #[doc = "0: Прямой режим"]
    Direct = 0,
    #[doc = "1: Обратный режим"]
    Reverse = 1,
    #[doc = "2: Двунаправленный режим"]
    Bidirectional = 2,
}
impl From<CountMode> for u8 {
    #[inline(always)]
    fn from(variant: CountMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CountMode {
    type Ux = u8;
}
impl crate::IsEnum for CountMode {}
#[doc = "Field `COUNT_MODE` reader - Режим счёта таймера"]
pub type CountModeR = crate::FieldReader<CountMode>;
impl CountModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CountMode> {
        match self.bits {
            0 => Some(CountMode::Direct),
            1 => Some(CountMode::Reverse),
            2 => Some(CountMode::Bidirectional),
            _ => None,
        }
    }
    #[doc = "Прямой режим"]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        *self == CountMode::Direct
    }
    #[doc = "Обратный режим"]
    #[inline(always)]
    pub fn is_reverse(&self) -> bool {
        *self == CountMode::Reverse
    }
    #[doc = "Двунаправленный режим"]
    #[inline(always)]
    pub fn is_bidirectional(&self) -> bool {
        *self == CountMode::Bidirectional
    }
}
#[doc = "Field `COUNT_MODE` writer - Режим счёта таймера"]
pub type CountModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, CountMode>;
impl<'a, REG> CountModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Прямой режим"]
    #[inline(always)]
    pub fn direct(self) -> &'a mut crate::W<REG> {
        self.variant(CountMode::Direct)
    }
    #[doc = "Обратный режим"]
    #[inline(always)]
    pub fn reverse(self) -> &'a mut crate::W<REG> {
        self.variant(CountMode::Reverse)
    }
    #[doc = "Двунаправленный режим"]
    #[inline(always)]
    pub fn bidirectional(self) -> &'a mut crate::W<REG> {
        self.variant(CountMode::Bidirectional)
    }
}
#[doc = "Выбор источника тактового сигнала для счета\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sourse {
    #[doc = "0: Вход модуля TIM1"]
    Tim1 = 0,
    #[doc = "2: Вход модуля TIM2"]
    Tim2 = 2,
    #[doc = "3: Выход предделителя"]
    Tim3 = 3,
}
impl From<Sourse> for u8 {
    #[inline(always)]
    fn from(variant: Sourse) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sourse {
    type Ux = u8;
}
impl crate::IsEnum for Sourse {}
#[doc = "Field `SOURSE` reader - Выбор источника тактового сигнала для счета"]
pub type SourseR = crate::FieldReader<Sourse>;
impl SourseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sourse> {
        match self.bits {
            0 => Some(Sourse::Tim1),
            2 => Some(Sourse::Tim2),
            3 => Some(Sourse::Tim3),
            _ => None,
        }
    }
    #[doc = "Вход модуля TIM1"]
    #[inline(always)]
    pub fn is_tim1(&self) -> bool {
        *self == Sourse::Tim1
    }
    #[doc = "Вход модуля TIM2"]
    #[inline(always)]
    pub fn is_tim2(&self) -> bool {
        *self == Sourse::Tim2
    }
    #[doc = "Выход предделителя"]
    #[inline(always)]
    pub fn is_tim3(&self) -> bool {
        *self == Sourse::Tim3
    }
}
#[doc = "Field `SOURSE` writer - Выбор источника тактового сигнала для счета"]
pub type SourseW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sourse>;
impl<'a, REG> SourseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Вход модуля TIM1"]
    #[inline(always)]
    pub fn tim1(self) -> &'a mut crate::W<REG> {
        self.variant(Sourse::Tim1)
    }
    #[doc = "Вход модуля TIM2"]
    #[inline(always)]
    pub fn tim2(self) -> &'a mut crate::W<REG> {
        self.variant(Sourse::Tim2)
    }
    #[doc = "Выход предделителя"]
    #[inline(always)]
    pub fn tim3(self) -> &'a mut crate::W<REG> {
        self.variant(Sourse::Tim3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Режим счёта таймера"]
    #[inline(always)]
    pub fn count_mode(&self) -> CountModeR {
        CountModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Выбор источника тактового сигнала для счета"]
    #[inline(always)]
    pub fn sourse(&self) -> SourseR {
        SourseR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Режим счёта таймера"]
    #[inline(always)]
    pub fn count_mode(&mut self) -> CountModeW<ControlSpec> {
        CountModeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Выбор источника тактового сигнала для счета"]
    #[inline(always)]
    pub fn sourse(&mut self) -> SourseW<ControlSpec> {
        SourseW::new(self, 2)
    }
}
#[doc = "Конфигурационный регистр основного таймера\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for ControlSpec {
    const RESET_VALUE: u32 = 0;
}
