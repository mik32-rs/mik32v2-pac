#[doc = "Register `RRTC_TALRM` reader"]
pub type R = crate::R<RrtcTalrmSpec>;
#[doc = "Register `RRTC_TALRM` writer"]
pub type W = crate::W<RrtcTalrmSpec>;
#[doc = "Разрешает сравнения секунд, когда установлен\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cs {
    #[doc = "0: Сравнение секунд отключено"]
    Disabled = 0,
    #[doc = "1: Сравнение секунд включено"]
    Enable = 1,
}
impl From<Cs> for bool {
    #[inline(always)]
    fn from(variant: Cs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CS` reader - Разрешает сравнения секунд, когда установлен"]
pub type CsR = crate::BitReader<Cs>;
impl CsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cs {
        match self.bits {
            false => Cs::Disabled,
            true => Cs::Enable,
        }
    }
    #[doc = "Сравнение секунд отключено"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cs::Disabled
    }
    #[doc = "Сравнение секунд включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cs::Enable
    }
}
#[doc = "Field `CS` writer - Разрешает сравнения секунд, когда установлен"]
pub type CsW<'a, REG> = crate::BitWriter<'a, REG, Cs>;
impl<'a, REG> CsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Сравнение секунд отключено"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cs::Disabled)
    }
    #[doc = "Сравнение секунд включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cs::Enable)
    }
}
#[doc = "Разрешает сравнения минут, когда установлен\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cm {
    #[doc = "0: Сравнение минут отключено"]
    Disabled = 0,
    #[doc = "1: Сравнение минут включено"]
    Enable = 1,
}
impl From<Cm> for bool {
    #[inline(always)]
    fn from(variant: Cm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CM` reader - Разрешает сравнения минут, когда установлен"]
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
    #[doc = "Сравнение минут отключено"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cm::Disabled
    }
    #[doc = "Сравнение минут включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cm::Enable
    }
}
#[doc = "Field `CM` writer - Разрешает сравнения минут, когда установлен"]
pub type CmW<'a, REG> = crate::BitWriter<'a, REG, Cm>;
impl<'a, REG> CmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Сравнение минут отключено"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::Disabled)
    }
    #[doc = "Сравнение минут включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::Enable)
    }
}
#[doc = "Разрешает сравнения часов, когда установлен\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch {
    #[doc = "0: Сравнение часов отключено"]
    Disabled = 0,
    #[doc = "1: Сравнение часов включено"]
    Enable = 1,
}
impl From<Ch> for bool {
    #[inline(always)]
    fn from(variant: Ch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH` reader - Разрешает сравнения часов, когда установлен"]
pub type ChR = crate::BitReader<Ch>;
impl ChR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch {
        match self.bits {
            false => Ch::Disabled,
            true => Ch::Enable,
        }
    }
    #[doc = "Сравнение часов отключено"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch::Disabled
    }
    #[doc = "Сравнение часов включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ch::Enable
    }
}
#[doc = "Field `CH` writer - Разрешает сравнения часов, когда установлен"]
pub type ChW<'a, REG> = crate::BitWriter<'a, REG, Ch>;
impl<'a, REG> ChW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Сравнение часов отключено"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch::Disabled)
    }
    #[doc = "Сравнение часов включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ch::Enable)
    }
}
#[doc = "Разрешает сравнения дней недели, когда установлен\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cdow {
    #[doc = "0: Сравнение дней недели отключено"]
    Disabled = 0,
    #[doc = "1: Сравнение дней недели включено"]
    Enable = 1,
}
impl From<Cdow> for bool {
    #[inline(always)]
    fn from(variant: Cdow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDOW` reader - Разрешает сравнения дней недели, когда установлен"]
pub type CdowR = crate::BitReader<Cdow>;
impl CdowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cdow {
        match self.bits {
            false => Cdow::Disabled,
            true => Cdow::Enable,
        }
    }
    #[doc = "Сравнение дней недели отключено"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cdow::Disabled
    }
    #[doc = "Сравнение дней недели включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cdow::Enable
    }
}
#[doc = "Field `CDOW` writer - Разрешает сравнения дней недели, когда установлен"]
pub type CdowW<'a, REG> = crate::BitWriter<'a, REG, Cdow>;
impl<'a, REG> CdowW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Сравнение дней недели отключено"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cdow::Disabled)
    }
    #[doc = "Сравнение дней недели включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cdow::Enable)
    }
}
impl R {
    #[doc = "Bit 28 - Разрешает сравнения секунд, когда установлен"]
    #[inline(always)]
    pub fn cs(&self) -> CsR {
        CsR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Разрешает сравнения минут, когда установлен"]
    #[inline(always)]
    pub fn cm(&self) -> CmR {
        CmR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Разрешает сравнения часов, когда установлен"]
    #[inline(always)]
    pub fn ch(&self) -> ChR {
        ChR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Разрешает сравнения дней недели, когда установлен"]
    #[inline(always)]
    pub fn cdow(&self) -> CdowR {
        CdowR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 28 - Разрешает сравнения секунд, когда установлен"]
    #[inline(always)]
    pub fn cs(&mut self) -> CsW<RrtcTalrmSpec> {
        CsW::new(self, 28)
    }
    #[doc = "Bit 29 - Разрешает сравнения минут, когда установлен"]
    #[inline(always)]
    pub fn cm(&mut self) -> CmW<RrtcTalrmSpec> {
        CmW::new(self, 29)
    }
    #[doc = "Bit 30 - Разрешает сравнения часов, когда установлен"]
    #[inline(always)]
    pub fn ch(&mut self) -> ChW<RrtcTalrmSpec> {
        ChW::new(self, 30)
    }
    #[doc = "Bit 31 - Разрешает сравнения дней недели, когда установлен"]
    #[inline(always)]
    pub fn cdow(&mut self) -> CdowW<RrtcTalrmSpec> {
        CdowW::new(self, 31)
    }
}
#[doc = "регистр хранит время, при совпадении которого со значением регистра RRTC_TIME, будет сгенерировано соответствующее прерывание\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_talrm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_talrm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RrtcTalrmSpec;
impl crate::RegisterSpec for RrtcTalrmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rrtc_talrm::R`](R) reader structure"]
impl crate::Readable for RrtcTalrmSpec {}
#[doc = "`write(|w| ..)` method takes [`rrtc_talrm::W`](W) writer structure"]
impl crate::Writable for RrtcTalrmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RRTC_TALRM to value 0"]
impl crate::Resettable for RrtcTalrmSpec {
    const RESET_VALUE: u32 = 0;
}
