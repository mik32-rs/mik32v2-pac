#[doc = "Register `CON` reader"]
pub type R = crate::R<ConSpec>;
#[doc = "Register `CON` writer"]
pub type W = crate::W<ConSpec>;
#[doc = "Field `PRELOAD` reader - Начальное значение таймера при запуске или перезапуске (таймер считает в сторону увеличения значений)"]
pub type PreloadR = crate::FieldReader<u16>;
#[doc = "Field `PRELOAD` writer - Начальное значение таймера при запуске или перезапуске (таймер считает в сторону увеличения значений)"]
pub type PreloadW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Делитель входной частоты (Fclk) для таймера\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prescale {
    #[doc = "0: Fclk / 1"]
    _1 = 0,
    #[doc = "1: Fclk / 2"]
    _2 = 1,
    #[doc = "2: Fclk / 4"]
    _4 = 2,
    #[doc = "3: Fclk / 16"]
    _16 = 3,
    #[doc = "4: Fclk / 64"]
    _64 = 4,
    #[doc = "5: Fclk / 256"]
    _256 = 5,
    #[doc = "6: Fclk / 1024"]
    _1024 = 6,
    #[doc = "7: Fclk / 4096"]
    _4096 = 7,
}
impl From<Prescale> for u8 {
    #[inline(always)]
    fn from(variant: Prescale) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prescale {
    type Ux = u8;
}
impl crate::IsEnum for Prescale {}
#[doc = "Field `PRESCALE` reader - Делитель входной частоты (Fclk) для таймера"]
pub type PrescaleR = crate::FieldReader<Prescale>;
impl PrescaleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prescale {
        match self.bits {
            0 => Prescale::_1,
            1 => Prescale::_2,
            2 => Prescale::_4,
            3 => Prescale::_16,
            4 => Prescale::_64,
            5 => Prescale::_256,
            6 => Prescale::_1024,
            7 => Prescale::_4096,
            _ => unreachable!(),
        }
    }
    #[doc = "Fclk / 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Prescale::_1
    }
    #[doc = "Fclk / 2"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Prescale::_2
    }
    #[doc = "Fclk / 4"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Prescale::_4
    }
    #[doc = "Fclk / 16"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Prescale::_16
    }
    #[doc = "Fclk / 64"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == Prescale::_64
    }
    #[doc = "Fclk / 256"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == Prescale::_256
    }
    #[doc = "Fclk / 1024"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == Prescale::_1024
    }
    #[doc = "Fclk / 4096"]
    #[inline(always)]
    pub fn is_4096(&self) -> bool {
        *self == Prescale::_4096
    }
}
#[doc = "Field `PRESCALE` writer - Делитель входной частоты (Fclk) для таймера"]
pub type PrescaleW<'a, REG> = crate::FieldWriter<'a, REG, 3, Prescale, crate::Safe>;
impl<'a, REG> PrescaleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Fclk / 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::_1)
    }
    #[doc = "Fclk / 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::_2)
    }
    #[doc = "Fclk / 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::_4)
    }
    #[doc = "Fclk / 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::_16)
    }
    #[doc = "Fclk / 64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::_64)
    }
    #[doc = "Fclk / 256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::_256)
    }
    #[doc = "Fclk / 1024"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::_1024)
    }
    #[doc = "Fclk / 4096"]
    #[inline(always)]
    pub fn _4096(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::_4096)
    }
}
impl R {
    #[doc = "Bits 0:11 - Начальное значение таймера при запуске или перезапуске (таймер считает в сторону увеличения значений)"]
    #[inline(always)]
    pub fn preload(&self) -> PreloadR {
        PreloadR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14 - Делитель входной частоты (Fclk) для таймера"]
    #[inline(always)]
    pub fn prescale(&self) -> PrescaleR {
        PrescaleR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Начальное значение таймера при запуске или перезапуске (таймер считает в сторону увеличения значений)"]
    #[inline(always)]
    pub fn preload(&mut self) -> PreloadW<ConSpec> {
        PreloadW::new(self, 0)
    }
    #[doc = "Bits 12:14 - Делитель входной частоты (Fclk) для таймера"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PrescaleW<ConSpec> {
        PrescaleW::new(self, 12)
    }
}
#[doc = "Конфигурация\n\nYou can [`read`](crate::Reg::read) this register and get [`con::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`con::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConSpec;
impl crate::RegisterSpec for ConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`con::R`](R) reader structure"]
impl crate::Readable for ConSpec {}
#[doc = "`write(|w| ..)` method takes [`con::W`](W) writer structure"]
impl crate::Writable for ConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CON to value 0"]
impl crate::Resettable for ConSpec {
    const RESET_VALUE: u32 = 0;
}
