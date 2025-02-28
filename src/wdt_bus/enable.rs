#[doc = "Register `ENABLE` reader"]
pub type R = crate::R<EnableSpec>;
#[doc = "Register `ENABLE` writer"]
pub type W = crate::W<EnableSpec>;
#[doc = "Управление монитором шины периферийных устройств\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dom3 {
    #[doc = "0: Монитор шины выключен"]
    Disable = 0,
    #[doc = "1: Монитор шины включен"]
    Enable = 1,
}
impl From<Dom3> for bool {
    #[inline(always)]
    fn from(variant: Dom3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOM3` reader - Управление монитором шины периферийных устройств"]
pub type Dom3R = crate::BitReader<Dom3>;
impl Dom3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dom3 {
        match self.bits {
            false => Dom3::Disable,
            true => Dom3::Enable,
        }
    }
    #[doc = "Монитор шины выключен"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dom3::Disable
    }
    #[doc = "Монитор шины включен"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dom3::Enable
    }
}
#[doc = "Field `DOM3` writer - Управление монитором шины периферийных устройств"]
pub type Dom3W<'a, REG> = crate::BitWriter<'a, REG, Dom3>;
impl<'a, REG> Dom3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Монитор шины выключен"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dom3::Disable)
    }
    #[doc = "Монитор шины включен"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dom3::Enable)
    }
}
#[doc = "Управление монитором шины SPIFI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spifi {
    #[doc = "0: Монитор шины выключен"]
    Disable = 0,
    #[doc = "1: Монитор шины включен"]
    Enable = 1,
}
impl From<Spifi> for bool {
    #[inline(always)]
    fn from(variant: Spifi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIFI` reader - Управление монитором шины SPIFI"]
pub type SpifiR = crate::BitReader<Spifi>;
impl SpifiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spifi {
        match self.bits {
            false => Spifi::Disable,
            true => Spifi::Enable,
        }
    }
    #[doc = "Монитор шины выключен"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Spifi::Disable
    }
    #[doc = "Монитор шины включен"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Spifi::Enable
    }
}
#[doc = "Field `SPIFI` writer - Управление монитором шины SPIFI"]
pub type SpifiW<'a, REG> = crate::BitWriter<'a, REG, Spifi>;
impl<'a, REG> SpifiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Монитор шины выключен"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Spifi::Disable)
    }
    #[doc = "Монитор шины включен"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Spifi::Enable)
    }
}
#[doc = "Управление монитором шины EEPROM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eeprom {
    #[doc = "0: Монитор шины выключен"]
    Disable = 0,
    #[doc = "1: Монитор шины включен"]
    Enable = 1,
}
impl From<Eeprom> for bool {
    #[inline(always)]
    fn from(variant: Eeprom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEPROM` reader - Управление монитором шины EEPROM"]
pub type EepromR = crate::BitReader<Eeprom>;
impl EepromR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eeprom {
        match self.bits {
            false => Eeprom::Disable,
            true => Eeprom::Enable,
        }
    }
    #[doc = "Монитор шины выключен"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Eeprom::Disable
    }
    #[doc = "Монитор шины включен"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Eeprom::Enable
    }
}
#[doc = "Field `EEPROM` writer - Управление монитором шины EEPROM"]
pub type EepromW<'a, REG> = crate::BitWriter<'a, REG, Eeprom>;
impl<'a, REG> EepromW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Монитор шины выключен"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Eeprom::Disable)
    }
    #[doc = "Монитор шины включен"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Eeprom::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Управление монитором шины периферийных устройств"]
    #[inline(always)]
    pub fn dom3(&self) -> Dom3R {
        Dom3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Управление монитором шины SPIFI"]
    #[inline(always)]
    pub fn spifi(&self) -> SpifiR {
        SpifiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Управление монитором шины EEPROM"]
    #[inline(always)]
    pub fn eeprom(&self) -> EepromR {
        EepromR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Управление монитором шины периферийных устройств"]
    #[inline(always)]
    pub fn dom3(&mut self) -> Dom3W<EnableSpec> {
        Dom3W::new(self, 0)
    }
    #[doc = "Bit 1 - Управление монитором шины SPIFI"]
    #[inline(always)]
    pub fn spifi(&mut self) -> SpifiW<EnableSpec> {
        SpifiW::new(self, 1)
    }
    #[doc = "Bit 2 - Управление монитором шины EEPROM"]
    #[inline(always)]
    pub fn eeprom(&mut self) -> EepromW<EnableSpec> {
        EepromW::new(self, 2)
    }
}
#[doc = "Запуск/отключение мониторов шины\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
