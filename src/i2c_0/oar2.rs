#[doc = "Register `OAR2` reader"]
pub type R = crate::R<Oar2Spec>;
#[doc = "Register `OAR2` writer"]
pub type W = crate::W<Oar2Spec>;
#[doc = "Field `OA2` reader - Собственный 7-битный адрес 2 Изменение битов допускается при OA2EN=0"]
pub type Oa2R = crate::FieldReader;
#[doc = "Field `OA2` writer - Собственный 7-битный адрес 2 Изменение битов допускается при OA2EN=0"]
pub type Oa2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Маска адреса OA2. Если OA2MSK ≠ 0, зарезервированные адреса I2C (0b0000xxx, 0b1111xxx) не подтверждаются, даже если адреса совпадают. Изменение битов допускается при OA2EN=0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Oa2msk {
    #[doc = "0: Нет маски"]
    NoMask = 0,
    #[doc = "1: Сравниваются только OA2\\[7:2\\]"]
    _1_1Masked = 1,
    #[doc = "2: Сравниваются только OA2\\[7:3\\]"]
    _2_1Masked = 2,
    #[doc = "3: Сравниваются только OA2\\[7:4\\]"]
    _3_1Masked = 3,
    #[doc = "4: Сравниваются только OA2\\[7:5\\]"]
    _4_1Masked = 4,
    #[doc = "5: Сравниваются только OA2\\[7:6\\]"]
    _5_1Masked = 5,
    #[doc = "6: Сравниваются только OA2\\[7\\]"]
    _6_1Masked = 6,
    #[doc = "7: OA2\\[7:1\\]
маскируются, подтверждаются (ACK) все 7-битные ад-реса (кроме зарезервированных)"]
    _7_1Masked = 7,
}
impl From<Oa2msk> for u8 {
    #[inline(always)]
    fn from(variant: Oa2msk) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Oa2msk {
    type Ux = u8;
}
impl crate::IsEnum for Oa2msk {}
#[doc = "Field `OA2MSK` reader - Маска адреса OA2. Если OA2MSK ≠ 0, зарезервированные адреса I2C (0b0000xxx, 0b1111xxx) не подтверждаются, даже если адреса совпадают. Изменение битов допускается при OA2EN=0"]
pub type Oa2mskR = crate::FieldReader<Oa2msk>;
impl Oa2mskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oa2msk {
        match self.bits {
            0 => Oa2msk::NoMask,
            1 => Oa2msk::_1_1Masked,
            2 => Oa2msk::_2_1Masked,
            3 => Oa2msk::_3_1Masked,
            4 => Oa2msk::_4_1Masked,
            5 => Oa2msk::_5_1Masked,
            6 => Oa2msk::_6_1Masked,
            7 => Oa2msk::_7_1Masked,
            _ => unreachable!(),
        }
    }
    #[doc = "Нет маски"]
    #[inline(always)]
    pub fn is_no_mask(&self) -> bool {
        *self == Oa2msk::NoMask
    }
    #[doc = "Сравниваются только OA2\\[7:2\\]"]
    #[inline(always)]
    pub fn is_1_1_masked(&self) -> bool {
        *self == Oa2msk::_1_1Masked
    }
    #[doc = "Сравниваются только OA2\\[7:3\\]"]
    #[inline(always)]
    pub fn is_2_1_masked(&self) -> bool {
        *self == Oa2msk::_2_1Masked
    }
    #[doc = "Сравниваются только OA2\\[7:4\\]"]
    #[inline(always)]
    pub fn is_3_1_masked(&self) -> bool {
        *self == Oa2msk::_3_1Masked
    }
    #[doc = "Сравниваются только OA2\\[7:5\\]"]
    #[inline(always)]
    pub fn is_4_1_masked(&self) -> bool {
        *self == Oa2msk::_4_1Masked
    }
    #[doc = "Сравниваются только OA2\\[7:6\\]"]
    #[inline(always)]
    pub fn is_5_1_masked(&self) -> bool {
        *self == Oa2msk::_5_1Masked
    }
    #[doc = "Сравниваются только OA2\\[7\\]"]
    #[inline(always)]
    pub fn is_6_1_masked(&self) -> bool {
        *self == Oa2msk::_6_1Masked
    }
    #[doc = "OA2\\[7:1\\]
маскируются, подтверждаются (ACK) все 7-битные ад-реса (кроме зарезервированных)"]
    #[inline(always)]
    pub fn is_7_1_masked(&self) -> bool {
        *self == Oa2msk::_7_1Masked
    }
}
#[doc = "Field `OA2MSK` writer - Маска адреса OA2. Если OA2MSK ≠ 0, зарезервированные адреса I2C (0b0000xxx, 0b1111xxx) не подтверждаются, даже если адреса совпадают. Изменение битов допускается при OA2EN=0"]
pub type Oa2mskW<'a, REG> = crate::FieldWriter<'a, REG, 3, Oa2msk, crate::Safe>;
impl<'a, REG> Oa2mskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Нет маски"]
    #[inline(always)]
    pub fn no_mask(self) -> &'a mut crate::W<REG> {
        self.variant(Oa2msk::NoMask)
    }
    #[doc = "Сравниваются только OA2\\[7:2\\]"]
    #[inline(always)]
    pub fn _1_1_masked(self) -> &'a mut crate::W<REG> {
        self.variant(Oa2msk::_1_1Masked)
    }
    #[doc = "Сравниваются только OA2\\[7:3\\]"]
    #[inline(always)]
    pub fn _2_1_masked(self) -> &'a mut crate::W<REG> {
        self.variant(Oa2msk::_2_1Masked)
    }
    #[doc = "Сравниваются только OA2\\[7:4\\]"]
    #[inline(always)]
    pub fn _3_1_masked(self) -> &'a mut crate::W<REG> {
        self.variant(Oa2msk::_3_1Masked)
    }
    #[doc = "Сравниваются только OA2\\[7:5\\]"]
    #[inline(always)]
    pub fn _4_1_masked(self) -> &'a mut crate::W<REG> {
        self.variant(Oa2msk::_4_1Masked)
    }
    #[doc = "Сравниваются только OA2\\[7:6\\]"]
    #[inline(always)]
    pub fn _5_1_masked(self) -> &'a mut crate::W<REG> {
        self.variant(Oa2msk::_5_1Masked)
    }
    #[doc = "Сравниваются только OA2\\[7\\]"]
    #[inline(always)]
    pub fn _6_1_masked(self) -> &'a mut crate::W<REG> {
        self.variant(Oa2msk::_6_1Masked)
    }
    #[doc = "OA2\\[7:1\\]
маскируются, подтверждаются (ACK) все 7-битные ад-реса (кроме зарезервированных)"]
    #[inline(always)]
    pub fn _7_1_masked(self) -> &'a mut crate::W<REG> {
        self.variant(Oa2msk::_7_1Masked)
    }
}
#[doc = "Использование собственно-го адреса OA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oa2en {
    #[doc = "0: При получении адреса OA2 формируется NACK"]
    Nack = 0,
    #[doc = "1: При получении адреса OA2 формируется ACK"]
    Ack = 1,
}
impl From<Oa2en> for bool {
    #[inline(always)]
    fn from(variant: Oa2en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OA2EN` reader - Использование собственно-го адреса OA2"]
pub type Oa2enR = crate::BitReader<Oa2en>;
impl Oa2enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oa2en {
        match self.bits {
            false => Oa2en::Nack,
            true => Oa2en::Ack,
        }
    }
    #[doc = "При получении адреса OA2 формируется NACK"]
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == Oa2en::Nack
    }
    #[doc = "При получении адреса OA2 формируется ACK"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == Oa2en::Ack
    }
}
#[doc = "Field `OA2EN` writer - Использование собственно-го адреса OA2"]
pub type Oa2enW<'a, REG> = crate::BitWriter<'a, REG, Oa2en>;
impl<'a, REG> Oa2enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "При получении адреса OA2 формируется NACK"]
    #[inline(always)]
    pub fn nack(self) -> &'a mut crate::W<REG> {
        self.variant(Oa2en::Nack)
    }
    #[doc = "При получении адреса OA2 формируется ACK"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut crate::W<REG> {
        self.variant(Oa2en::Ack)
    }
}
impl R {
    #[doc = "Bits 1:7 - Собственный 7-битный адрес 2 Изменение битов допускается при OA2EN=0"]
    #[inline(always)]
    pub fn oa2(&self) -> Oa2R {
        Oa2R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:10 - Маска адреса OA2. Если OA2MSK ≠ 0, зарезервированные адреса I2C (0b0000xxx, 0b1111xxx) не подтверждаются, даже если адреса совпадают. Изменение битов допускается при OA2EN=0"]
    #[inline(always)]
    pub fn oa2msk(&self) -> Oa2mskR {
        Oa2mskR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Использование собственно-го адреса OA2"]
    #[inline(always)]
    pub fn oa2en(&self) -> Oa2enR {
        Oa2enR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:7 - Собственный 7-битный адрес 2 Изменение битов допускается при OA2EN=0"]
    #[inline(always)]
    pub fn oa2(&mut self) -> Oa2W<Oar2Spec> {
        Oa2W::new(self, 1)
    }
    #[doc = "Bits 8:10 - Маска адреса OA2. Если OA2MSK ≠ 0, зарезервированные адреса I2C (0b0000xxx, 0b1111xxx) не подтверждаются, даже если адреса совпадают. Изменение битов допускается при OA2EN=0"]
    #[inline(always)]
    pub fn oa2msk(&mut self) -> Oa2mskW<Oar2Spec> {
        Oa2mskW::new(self, 8)
    }
    #[doc = "Bit 15 - Использование собственно-го адреса OA2"]
    #[inline(always)]
    pub fn oa2en(&mut self) -> Oa2enW<Oar2Spec> {
        Oa2enW::new(self, 15)
    }
}
#[doc = "Регистр адреса 2\n\nYou can [`read`](crate::Reg::read) this register and get [`oar2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Oar2Spec;
impl crate::RegisterSpec for Oar2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oar2::R`](R) reader structure"]
impl crate::Readable for Oar2Spec {}
#[doc = "`write(|w| ..)` method takes [`oar2::W`](W) writer structure"]
impl crate::Writable for Oar2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OAR2 to value 0"]
impl crate::Resettable for Oar2Spec {
    const RESET_VALUE: u32 = 0;
}
