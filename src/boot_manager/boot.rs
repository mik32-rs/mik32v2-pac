#[doc = "Register `BOOT` reader"]
pub type R = crate::R<BootSpec>;
#[doc = "Register `BOOT` writer"]
pub type W = crate::W<BootSpec>;
#[doc = "Регистр режима загрузки. При чтении, возвращает режим, который будет использоваться для следующего сброса системного домена. При записи – установка нового режима загрузки.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BootMode {
    #[doc = "0: Старт из встроенной памяти EEPROM"]
    Eeprom = 0,
    #[doc = "1: Старт из системного ОЗУ"]
    Ram = 1,
    #[doc = "2: Старт из внешней памяти с использованием контроллера SPIFI"]
    Spifi = 2,
}
impl From<BootMode> for u8 {
    #[inline(always)]
    fn from(variant: BootMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BootMode {
    type Ux = u8;
}
impl crate::IsEnum for BootMode {}
#[doc = "Field `BOOT_MODE` reader - Регистр режима загрузки. При чтении, возвращает режим, который будет использоваться для следующего сброса системного домена. При записи – установка нового режима загрузки."]
pub type BootModeR = crate::FieldReader<BootMode>;
impl BootModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BootMode> {
        match self.bits {
            0 => Some(BootMode::Eeprom),
            1 => Some(BootMode::Ram),
            2 => Some(BootMode::Spifi),
            _ => None,
        }
    }
    #[doc = "Старт из встроенной памяти EEPROM"]
    #[inline(always)]
    pub fn is_eeprom(&self) -> bool {
        *self == BootMode::Eeprom
    }
    #[doc = "Старт из системного ОЗУ"]
    #[inline(always)]
    pub fn is_ram(&self) -> bool {
        *self == BootMode::Ram
    }
    #[doc = "Старт из внешней памяти с использованием контроллера SPIFI"]
    #[inline(always)]
    pub fn is_spifi(&self) -> bool {
        *self == BootMode::Spifi
    }
}
#[doc = "Field `BOOT_MODE` writer - Регистр режима загрузки. При чтении, возвращает режим, который будет использоваться для следующего сброса системного домена. При записи – установка нового режима загрузки."]
pub type BootModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, BootMode>;
impl<'a, REG> BootModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Старт из встроенной памяти EEPROM"]
    #[inline(always)]
    pub fn eeprom(self) -> &'a mut crate::W<REG> {
        self.variant(BootMode::Eeprom)
    }
    #[doc = "Старт из системного ОЗУ"]
    #[inline(always)]
    pub fn ram(self) -> &'a mut crate::W<REG> {
        self.variant(BootMode::Ram)
    }
    #[doc = "Старт из внешней памяти с использованием контроллера SPIFI"]
    #[inline(always)]
    pub fn spifi(self) -> &'a mut crate::W<REG> {
        self.variant(BootMode::Spifi)
    }
}
impl R {
    #[doc = "Bits 0:1 - Регистр режима загрузки. При чтении, возвращает режим, который будет использоваться для следующего сброса системного домена. При записи – установка нового режима загрузки."]
    #[inline(always)]
    pub fn boot_mode(&self) -> BootModeR {
        BootModeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Регистр режима загрузки. При чтении, возвращает режим, который будет использоваться для следующего сброса системного домена. При записи – установка нового режима загрузки."]
    #[inline(always)]
    pub fn boot_mode(&mut self) -> BootModeW<BootSpec> {
        BootModeW::new(self, 0)
    }
}
#[doc = "Регистр режима загрузки\n\nYou can [`read`](crate::Reg::read) this register and get [`boot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BootSpec;
impl crate::RegisterSpec for BootSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot::R`](R) reader structure"]
impl crate::Readable for BootSpec {}
#[doc = "`write(|w| ..)` method takes [`boot::W`](W) writer structure"]
impl crate::Writable for BootSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOT to value 0"]
impl crate::Resettable for BootSpec {
    const RESET_VALUE: u32 = 0;
}
