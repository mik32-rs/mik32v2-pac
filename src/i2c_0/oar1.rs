#[doc = "Register `OAR1` reader"]
pub type R = crate::R<Oar1Spec>;
#[doc = "Register `OAR1` writer"]
pub type W = crate::W<Oar1Spec>;
#[doc = "Field `OA1_10bit` reader - Собственный адрес 1 (10-битный режим). Изменение бита допускается при OA1EN=0."]
pub type Oa1_10bitR = crate::FieldReader<u16>;
#[doc = "Field `OA1_10bit` writer - Собственный адрес 1 (10-битный режим). Изменение бита допускается при OA1EN=0."]
pub type Oa1_10bitW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `OA1_7bit` reader - Собственный адрес 1 (7-битный режим). Изменение бита допускается при OA1EN=0."]
pub type Oa1_7bitR = crate::FieldReader;
#[doc = "Field `OA1_7bit` writer - Собственный адрес 1 (7-битный режим). Изменение бита допускается при OA1EN=0."]
pub type Oa1_7bitW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Режим 10-битного адреса OA1. Изменение бита допускается при OA1EN=0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oa1mode {
    #[doc = "0: OA1 – 7-битный адрес"]
    _7bit = 0,
    #[doc = "1: OA1 – 10-битный адрес"]
    _10bit = 1,
}
impl From<Oa1mode> for bool {
    #[inline(always)]
    fn from(variant: Oa1mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OA1MODE` reader - Режим 10-битного адреса OA1. Изменение бита допускается при OA1EN=0"]
pub type Oa1modeR = crate::BitReader<Oa1mode>;
impl Oa1modeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oa1mode {
        match self.bits {
            false => Oa1mode::_7bit,
            true => Oa1mode::_10bit,
        }
    }
    #[doc = "OA1 – 7-битный адрес"]
    #[inline(always)]
    pub fn is_7bit(&self) -> bool {
        *self == Oa1mode::_7bit
    }
    #[doc = "OA1 – 10-битный адрес"]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == Oa1mode::_10bit
    }
}
#[doc = "Field `OA1MODE` writer - Режим 10-битного адреса OA1. Изменение бита допускается при OA1EN=0"]
pub type Oa1modeW<'a, REG> = crate::BitWriter<'a, REG, Oa1mode>;
impl<'a, REG> Oa1modeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OA1 – 7-битный адрес"]
    #[inline(always)]
    pub fn _7bit(self) -> &'a mut crate::W<REG> {
        self.variant(Oa1mode::_7bit)
    }
    #[doc = "OA1 – 10-битный адрес"]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut crate::W<REG> {
        self.variant(Oa1mode::_10bit)
    }
}
#[doc = "Использование собственно-го адреса OA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oa1en {
    #[doc = "0: При получении адреса OA1 формируется NACK"]
    Nack = 0,
    #[doc = "1: При получении адреса OA1 формируется ACK"]
    Ack = 1,
}
impl From<Oa1en> for bool {
    #[inline(always)]
    fn from(variant: Oa1en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OA1EN` reader - Использование собственно-го адреса OA1"]
pub type Oa1enR = crate::BitReader<Oa1en>;
impl Oa1enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oa1en {
        match self.bits {
            false => Oa1en::Nack,
            true => Oa1en::Ack,
        }
    }
    #[doc = "При получении адреса OA1 формируется NACK"]
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == Oa1en::Nack
    }
    #[doc = "При получении адреса OA1 формируется ACK"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == Oa1en::Ack
    }
}
#[doc = "Field `OA1EN` writer - Использование собственно-го адреса OA1"]
pub type Oa1enW<'a, REG> = crate::BitWriter<'a, REG, Oa1en>;
impl<'a, REG> Oa1enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "При получении адреса OA1 формируется NACK"]
    #[inline(always)]
    pub fn nack(self) -> &'a mut crate::W<REG> {
        self.variant(Oa1en::Nack)
    }
    #[doc = "При получении адреса OA1 формируется ACK"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut crate::W<REG> {
        self.variant(Oa1en::Ack)
    }
}
impl R {
    #[doc = "Bits 0:9 - Собственный адрес 1 (10-битный режим). Изменение бита допускается при OA1EN=0."]
    #[inline(always)]
    pub fn oa1_10bit(&self) -> Oa1_10bitR {
        Oa1_10bitR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 1:7 - Собственный адрес 1 (7-битный режим). Изменение бита допускается при OA1EN=0."]
    #[inline(always)]
    pub fn oa1_7bit(&self) -> Oa1_7bitR {
        Oa1_7bitR::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 10 - Режим 10-битного адреса OA1. Изменение бита допускается при OA1EN=0"]
    #[inline(always)]
    pub fn oa1mode(&self) -> Oa1modeR {
        Oa1modeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Использование собственно-го адреса OA1"]
    #[inline(always)]
    pub fn oa1en(&self) -> Oa1enR {
        Oa1enR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Собственный адрес 1 (10-битный режим). Изменение бита допускается при OA1EN=0."]
    #[inline(always)]
    pub fn oa1_10bit(&mut self) -> Oa1_10bitW<Oar1Spec> {
        Oa1_10bitW::new(self, 0)
    }
    #[doc = "Bits 1:7 - Собственный адрес 1 (7-битный режим). Изменение бита допускается при OA1EN=0."]
    #[inline(always)]
    pub fn oa1_7bit(&mut self) -> Oa1_7bitW<Oar1Spec> {
        Oa1_7bitW::new(self, 1)
    }
    #[doc = "Bit 10 - Режим 10-битного адреса OA1. Изменение бита допускается при OA1EN=0"]
    #[inline(always)]
    pub fn oa1mode(&mut self) -> Oa1modeW<Oar1Spec> {
        Oa1modeW::new(self, 10)
    }
    #[doc = "Bit 15 - Использование собственно-го адреса OA1"]
    #[inline(always)]
    pub fn oa1en(&mut self) -> Oa1enW<Oar1Spec> {
        Oa1enW::new(self, 15)
    }
}
#[doc = "Регистр адреса 1\n\nYou can [`read`](crate::Reg::read) this register and get [`oar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Oar1Spec;
impl crate::RegisterSpec for Oar1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oar1::R`](R) reader structure"]
impl crate::Readable for Oar1Spec {}
#[doc = "`write(|w| ..)` method takes [`oar1::W`](W) writer structure"]
impl crate::Writable for Oar1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OAR1 to value 0"]
impl crate::Resettable for Oar1Spec {
    const RESET_VALUE: u32 = 0;
}
