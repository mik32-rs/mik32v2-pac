#[doc = "Register `RRTC_DALRM` reader"]
pub type R = crate::R<RrtcDalrmSpec>;
#[doc = "Register `RRTC_DALRM` writer"]
pub type W = crate::W<RrtcDalrmSpec>;
#[doc = "Разрешает сравнения дней, когда установлен\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cd {
    #[doc = "0: Сравнение дней отключено"]
    Disabled = 0,
    #[doc = "1: Сравнение дней включено"]
    Enable = 1,
}
impl From<Cd> for bool {
    #[inline(always)]
    fn from(variant: Cd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CD` reader - Разрешает сравнения дней, когда установлен"]
pub type CdR = crate::BitReader<Cd>;
impl CdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cd {
        match self.bits {
            false => Cd::Disabled,
            true => Cd::Enable,
        }
    }
    #[doc = "Сравнение дней отключено"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cd::Disabled
    }
    #[doc = "Сравнение дней включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cd::Enable
    }
}
#[doc = "Field `CD` writer - Разрешает сравнения дней, когда установлен"]
pub type CdW<'a, REG> = crate::BitWriter<'a, REG, Cd>;
impl<'a, REG> CdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Сравнение дней отключено"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cd::Disabled)
    }
    #[doc = "Сравнение дней включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cd::Enable)
    }
}
#[doc = "Разрешает сравнения месяцев, когда установлен\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cm {
    #[doc = "0: Сравнение месяцев отключено"]
    Disabled = 0,
    #[doc = "1: Сравнение месяцев включено"]
    Enable = 1,
}
impl From<Cm> for bool {
    #[inline(always)]
    fn from(variant: Cm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CM` reader - Разрешает сравнения месяцев, когда установлен"]
pub type CmR = crate::BitReader<Cm>;
impl CmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cm {
        match self.bits {
            false => Cm::Disabled,
            true => Cm::Enable,
        }
    }
    #[doc = "Сравнение месяцев отключено"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cm::Disabled
    }
    #[doc = "Сравнение месяцев включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cm::Enable
    }
}
#[doc = "Field `CM` writer - Разрешает сравнения месяцев, когда установлен"]
pub type CmW<'a, REG> = crate::BitWriter<'a, REG, Cm>;
impl<'a, REG> CmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Сравнение месяцев отключено"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::Disabled)
    }
    #[doc = "Сравнение месяцев включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::Enable)
    }
}
#[doc = "Разрешает сравнения годов, когда установлен\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cy {
    #[doc = "0: Сравнение годов отключено"]
    Disabled = 0,
    #[doc = "1: Сравнение годов включено"]
    Enable = 1,
}
impl From<Cy> for bool {
    #[inline(always)]
    fn from(variant: Cy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CY` reader - Разрешает сравнения годов, когда установлен"]
pub type CyR = crate::BitReader<Cy>;
impl CyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cy {
        match self.bits {
            false => Cy::Disabled,
            true => Cy::Enable,
        }
    }
    #[doc = "Сравнение годов отключено"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cy::Disabled
    }
    #[doc = "Сравнение годов включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cy::Enable
    }
}
#[doc = "Field `CY` writer - Разрешает сравнения годов, когда установлен"]
pub type CyW<'a, REG> = crate::BitWriter<'a, REG, Cy>;
impl<'a, REG> CyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Сравнение годов отключено"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cy::Disabled)
    }
    #[doc = "Сравнение годов включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cy::Enable)
    }
}
#[doc = "Рразрешает сравнения веков, когда установлен\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc {
    #[doc = "0: Сравнение веков отключено"]
    Disabled = 0,
    #[doc = "1: Сравнение веков включено"]
    Enable = 1,
}
impl From<Cc> for bool {
    #[inline(always)]
    fn from(variant: Cc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC` reader - Рразрешает сравнения веков, когда установлен"]
pub type CcR = crate::BitReader<Cc>;
impl CcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc {
        match self.bits {
            false => Cc::Disabled,
            true => Cc::Enable,
        }
    }
    #[doc = "Сравнение веков отключено"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cc::Disabled
    }
    #[doc = "Сравнение веков включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cc::Enable
    }
}
#[doc = "Field `CC` writer - Рразрешает сравнения веков, когда установлен"]
pub type CcW<'a, REG> = crate::BitWriter<'a, REG, Cc>;
impl<'a, REG> CcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Сравнение веков отключено"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cc::Disabled)
    }
    #[doc = "Сравнение веков включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cc::Enable)
    }
}
impl R {
    #[doc = "Bit 27 - Разрешает сравнения дней, когда установлен"]
    #[inline(always)]
    pub fn cd(&self) -> CdR {
        CdR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Разрешает сравнения месяцев, когда установлен"]
    #[inline(always)]
    pub fn cm(&self) -> CmR {
        CmR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Разрешает сравнения годов, когда установлен"]
    #[inline(always)]
    pub fn cy(&self) -> CyR {
        CyR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Рразрешает сравнения веков, когда установлен"]
    #[inline(always)]
    pub fn cc(&self) -> CcR {
        CcR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - Разрешает сравнения дней, когда установлен"]
    #[inline(always)]
    pub fn cd(&mut self) -> CdW<RrtcDalrmSpec> {
        CdW::new(self, 27)
    }
    #[doc = "Bit 28 - Разрешает сравнения месяцев, когда установлен"]
    #[inline(always)]
    pub fn cm(&mut self) -> CmW<RrtcDalrmSpec> {
        CmW::new(self, 28)
    }
    #[doc = "Bit 29 - Разрешает сравнения годов, когда установлен"]
    #[inline(always)]
    pub fn cy(&mut self) -> CyW<RrtcDalrmSpec> {
        CyW::new(self, 29)
    }
    #[doc = "Bit 30 - Рразрешает сравнения веков, когда установлен"]
    #[inline(always)]
    pub fn cc(&mut self) -> CcW<RrtcDalrmSpec> {
        CcW::new(self, 30)
    }
}
#[doc = "регистр хранит время, при совпадении которого со значением регистра RRTC_TIME, будет сгенерировано соответствующее прерывание.\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_dalrm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_dalrm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RrtcDalrmSpec;
impl crate::RegisterSpec for RrtcDalrmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rrtc_dalrm::R`](R) reader structure"]
impl crate::Readable for RrtcDalrmSpec {}
#[doc = "`write(|w| ..)` method takes [`rrtc_dalrm::W`](W) writer structure"]
impl crate::Writable for RrtcDalrmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RRTC_DALRM to value 0"]
impl crate::Resettable for RrtcDalrmSpec {
    const RESET_VALUE: u32 = 0;
}
