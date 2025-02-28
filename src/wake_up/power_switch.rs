#[doc = "Register `POWER_SWITCH` reader"]
pub type R = crate::R<PowerSwitchSpec>;
#[doc = "Register `POWER_SWITCH` writer"]
pub type W = crate::W<PowerSwitchSpec>;
#[doc = "Разрешение принудительного переключения на один из источников питания\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Автоматическое переключение на один из источников питания"]
    Automatic = 0,
    #[doc = "1: Принудительное переключение на один из источников питания"]
    Forced = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Разрешение принудительного переключения на один из источников питания"]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::Automatic,
            true => En::Forced,
        }
    }
    #[doc = "Автоматическое переключение на один из источников питания"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == En::Automatic
    }
    #[doc = "Принудительное переключение на один из источников питания"]
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == En::Forced
    }
}
#[doc = "Field `EN` writer - Разрешение принудительного переключения на один из источников питания"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Автоматическое переключение на один из источников питания"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut crate::W<REG> {
        self.variant(En::Automatic)
    }
    #[doc = "Принудительное переключение на один из источников питания"]
    #[inline(always)]
    pub fn forced(self) -> &'a mut crate::W<REG> {
        self.variant(En::Forced)
    }
}
#[doc = "Выбор источника напряжения VCC_BU при принудительного переключения (при En = \"1\")\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Control {
    #[doc = "0: VCC - источник напряжения VCC_BU"]
    Vcc = 0,
    #[doc = "1: VCC_BAT - источник напряжения VCC_BU"]
    VccBat = 1,
}
impl From<Control> for bool {
    #[inline(always)]
    fn from(variant: Control) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONTROL` reader - Выбор источника напряжения VCC_BU при принудительного переключения (при En = \"1\")"]
pub type ControlR = crate::BitReader<Control>;
impl ControlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Control {
        match self.bits {
            false => Control::Vcc,
            true => Control::VccBat,
        }
    }
    #[doc = "VCC - источник напряжения VCC_BU"]
    #[inline(always)]
    pub fn is_vcc(&self) -> bool {
        *self == Control::Vcc
    }
    #[doc = "VCC_BAT - источник напряжения VCC_BU"]
    #[inline(always)]
    pub fn is_vcc_bat(&self) -> bool {
        *self == Control::VccBat
    }
}
#[doc = "Field `CONTROL` writer - Выбор источника напряжения VCC_BU при принудительного переключения (при En = \"1\")"]
pub type ControlW<'a, REG> = crate::BitWriter<'a, REG, Control>;
impl<'a, REG> ControlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VCC - источник напряжения VCC_BU"]
    #[inline(always)]
    pub fn vcc(self) -> &'a mut crate::W<REG> {
        self.variant(Control::Vcc)
    }
    #[doc = "VCC_BAT - источник напряжения VCC_BU"]
    #[inline(always)]
    pub fn vcc_bat(self) -> &'a mut crate::W<REG> {
        self.variant(Control::VccBat)
    }
}
#[doc = "Field `BATT_GOOD` reader - Флаг состояние резервного (батарейного) источника питания"]
pub type BattGoodR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Разрешение принудительного переключения на один из источников питания"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Выбор источника напряжения VCC_BU при принудительного переключения (при En = \"1\")"]
    #[inline(always)]
    pub fn control(&self) -> ControlR {
        ControlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Флаг состояние резервного (батарейного) источника питания"]
    #[inline(always)]
    pub fn batt_good(&self) -> BattGoodR {
        BattGoodR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Разрешение принудительного переключения на один из источников питания"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<PowerSwitchSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Выбор источника напряжения VCC_BU при принудительного переключения (при En = \"1\")"]
    #[inline(always)]
    pub fn control(&mut self) -> ControlW<PowerSwitchSpec> {
        ControlW::new(self, 1)
    }
}
#[doc = "Регистр управления и статуса схемы слежения за питанием\n\nYou can [`read`](crate::Reg::read) this register and get [`power_switch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_switch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerSwitchSpec;
impl crate::RegisterSpec for PowerSwitchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_switch::R`](R) reader structure"]
impl crate::Readable for PowerSwitchSpec {}
#[doc = "`write(|w| ..)` method takes [`power_switch::W`](W) writer structure"]
impl crate::Writable for PowerSwitchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POWER_SWITCH to value 0"]
impl crate::Resettable for PowerSwitchSpec {
    const RESET_VALUE: u32 = 0;
}
