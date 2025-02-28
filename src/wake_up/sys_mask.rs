#[doc = "Register `SYS_MASK` reader"]
pub type R = crate::R<SysMaskSpec>;
#[doc = "Register `SYS_MASK` writer"]
pub type W = crate::W<SysMaskSpec>;
#[doc = "Разрешение включение системного домена (из режима СТОП) при срабатывании будильника\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysUpRtc {
    #[doc = "0: Запрещено"]
    Disable = 0,
    #[doc = "1: Разрешено"]
    Enable = 1,
}
impl From<SysUpRtc> for bool {
    #[inline(always)]
    fn from(variant: SysUpRtc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYS_UP_RTC` reader - Разрешение включение системного домена (из режима СТОП) при срабатывании будильника"]
pub type SysUpRtcR = crate::BitReader<SysUpRtc>;
impl SysUpRtcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysUpRtc {
        match self.bits {
            false => SysUpRtc::Disable,
            true => SysUpRtc::Enable,
        }
    }
    #[doc = "Запрещено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SysUpRtc::Disable
    }
    #[doc = "Разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SysUpRtc::Enable
    }
}
#[doc = "Field `SYS_UP_RTC` writer - Разрешение включение системного домена (из режима СТОП) при срабатывании будильника"]
pub type SysUpRtcW<'a, REG> = crate::BitWriter<'a, REG, SysUpRtc>;
impl<'a, REG> SysUpRtcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Запрещено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SysUpRtc::Disable)
    }
    #[doc = "Разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SysUpRtc::Enable)
    }
}
#[doc = "Разрешение включение системного домена (из режима СТОП) при активном уровне внешнего вывода ext_wu\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysUpWu {
    #[doc = "0: Запрещено"]
    Disable = 0,
    #[doc = "1: Разрешено"]
    Enable = 1,
}
impl From<SysUpWu> for bool {
    #[inline(always)]
    fn from(variant: SysUpWu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYS_UP_WU` reader - Разрешение включение системного домена (из режима СТОП) при активном уровне внешнего вывода ext_wu"]
pub type SysUpWuR = crate::BitReader<SysUpWu>;
impl SysUpWuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysUpWu {
        match self.bits {
            false => SysUpWu::Disable,
            true => SysUpWu::Enable,
        }
    }
    #[doc = "Запрещено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SysUpWu::Disable
    }
    #[doc = "Разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SysUpWu::Enable
    }
}
#[doc = "Field `SYS_UP_WU` writer - Разрешение включение системного домена (из режима СТОП) при активном уровне внешнего вывода ext_wu"]
pub type SysUpWuW<'a, REG> = crate::BitWriter<'a, REG, SysUpWu>;
impl<'a, REG> SysUpWuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Запрещено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SysUpWu::Disable)
    }
    #[doc = "Разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SysUpWu::Enable)
    }
}
#[doc = "Запрещение формирования сброса системного домена при снижении питания (индикация от LDO_SYS)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysRstLdo {
    #[doc = "1: Запрещено"]
    Disable = 1,
    #[doc = "0: Разрешено"]
    Enable = 0,
}
impl From<SysRstLdo> for bool {
    #[inline(always)]
    fn from(variant: SysRstLdo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYS_RST_LDO` reader - Запрещение формирования сброса системного домена при снижении питания (индикация от LDO_SYS)"]
pub type SysRstLdoR = crate::BitReader<SysRstLdo>;
impl SysRstLdoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysRstLdo {
        match self.bits {
            true => SysRstLdo::Disable,
            false => SysRstLdo::Enable,
        }
    }
    #[doc = "Запрещено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SysRstLdo::Disable
    }
    #[doc = "Разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SysRstLdo::Enable
    }
}
#[doc = "Field `SYS_RST_LDO` writer - Запрещение формирования сброса системного домена при снижении питания (индикация от LDO_SYS)"]
pub type SysRstLdoW<'a, REG> = crate::BitWriter<'a, REG, SysRstLdo>;
impl<'a, REG> SysRstLdoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Запрещено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SysRstLdo::Disable)
    }
    #[doc = "Разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SysRstLdo::Enable)
    }
}
#[doc = "Запрещение формирования сброса системного домена при снижении питания (индикация от схемы слежения)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysRstPs {
    #[doc = "1: Запрещено"]
    Disable = 1,
    #[doc = "0: Разрешено"]
    Enable = 0,
}
impl From<SysRstPs> for bool {
    #[inline(always)]
    fn from(variant: SysRstPs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYS_RST_PS` reader - Запрещение формирования сброса системного домена при снижении питания (индикация от схемы слежения)"]
pub type SysRstPsR = crate::BitReader<SysRstPs>;
impl SysRstPsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysRstPs {
        match self.bits {
            true => SysRstPs::Disable,
            false => SysRstPs::Enable,
        }
    }
    #[doc = "Запрещено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SysRstPs::Disable
    }
    #[doc = "Разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SysRstPs::Enable
    }
}
#[doc = "Field `SYS_RST_PS` writer - Запрещение формирования сброса системного домена при снижении питания (индикация от схемы слежения)"]
pub type SysRstPsW<'a, REG> = crate::BitWriter<'a, REG, SysRstPs>;
impl<'a, REG> SysRstPsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Запрещено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SysRstPs::Disable)
    }
    #[doc = "Разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SysRstPs::Enable)
    }
}
#[doc = "Запрещение формирования сброса системного домена при появлении сигнала BOR\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysRstBor {
    #[doc = "1: Запрещено"]
    Disable = 1,
    #[doc = "0: Разрешено"]
    Enable = 0,
}
impl From<SysRstBor> for bool {
    #[inline(always)]
    fn from(variant: SysRstBor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYS_RST_BOR` reader - Запрещение формирования сброса системного домена при появлении сигнала BOR"]
pub type SysRstBorR = crate::BitReader<SysRstBor>;
impl SysRstBorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysRstBor {
        match self.bits {
            true => SysRstBor::Disable,
            false => SysRstBor::Enable,
        }
    }
    #[doc = "Запрещено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SysRstBor::Disable
    }
    #[doc = "Разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SysRstBor::Enable
    }
}
#[doc = "Field `SYS_RST_BOR` writer - Запрещение формирования сброса системного домена при появлении сигнала BOR"]
pub type SysRstBorW<'a, REG> = crate::BitWriter<'a, REG, SysRstBor>;
impl<'a, REG> SysRstBorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Запрещено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SysRstBor::Disable)
    }
    #[doc = "Разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SysRstBor::Enable)
    }
}
#[doc = "Запрещение формирования сброса батарейного домена при появлении сигнала BOR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuRstBor {
    #[doc = "1: Запрещено"]
    Disable = 1,
    #[doc = "0: Разрешено"]
    Enable = 0,
}
impl From<BuRstBor> for bool {
    #[inline(always)]
    fn from(variant: BuRstBor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BU_RST_BOR` reader - Запрещение формирования сброса батарейного домена при появлении сигнала BOR"]
pub type BuRstBorR = crate::BitReader<BuRstBor>;
impl BuRstBorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BuRstBor {
        match self.bits {
            true => BuRstBor::Disable,
            false => BuRstBor::Enable,
        }
    }
    #[doc = "Запрещено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BuRstBor::Disable
    }
    #[doc = "Разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BuRstBor::Enable
    }
}
#[doc = "Field `BU_RST_BOR` writer - Запрещение формирования сброса батарейного домена при появлении сигнала BOR"]
pub type BuRstBorW<'a, REG> = crate::BitWriter<'a, REG, BuRstBor>;
impl<'a, REG> BuRstBorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Запрещено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(BuRstBor::Disable)
    }
    #[doc = "Разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(BuRstBor::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Разрешение включение системного домена (из режима СТОП) при срабатывании будильника"]
    #[inline(always)]
    pub fn sys_up_rtc(&self) -> SysUpRtcR {
        SysUpRtcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Разрешение включение системного домена (из режима СТОП) при активном уровне внешнего вывода ext_wu"]
    #[inline(always)]
    pub fn sys_up_wu(&self) -> SysUpWuR {
        SysUpWuR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Запрещение формирования сброса системного домена при снижении питания (индикация от LDO_SYS)"]
    #[inline(always)]
    pub fn sys_rst_ldo(&self) -> SysRstLdoR {
        SysRstLdoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Запрещение формирования сброса системного домена при снижении питания (индикация от схемы слежения)"]
    #[inline(always)]
    pub fn sys_rst_ps(&self) -> SysRstPsR {
        SysRstPsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Запрещение формирования сброса системного домена при появлении сигнала BOR"]
    #[inline(always)]
    pub fn sys_rst_bor(&self) -> SysRstBorR {
        SysRstBorR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Запрещение формирования сброса батарейного домена при появлении сигнала BOR"]
    #[inline(always)]
    pub fn bu_rst_bor(&self) -> BuRstBorR {
        BuRstBorR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Разрешение включение системного домена (из режима СТОП) при срабатывании будильника"]
    #[inline(always)]
    pub fn sys_up_rtc(&mut self) -> SysUpRtcW<SysMaskSpec> {
        SysUpRtcW::new(self, 0)
    }
    #[doc = "Bit 1 - Разрешение включение системного домена (из режима СТОП) при активном уровне внешнего вывода ext_wu"]
    #[inline(always)]
    pub fn sys_up_wu(&mut self) -> SysUpWuW<SysMaskSpec> {
        SysUpWuW::new(self, 1)
    }
    #[doc = "Bit 2 - Запрещение формирования сброса системного домена при снижении питания (индикация от LDO_SYS)"]
    #[inline(always)]
    pub fn sys_rst_ldo(&mut self) -> SysRstLdoW<SysMaskSpec> {
        SysRstLdoW::new(self, 2)
    }
    #[doc = "Bit 3 - Запрещение формирования сброса системного домена при снижении питания (индикация от схемы слежения)"]
    #[inline(always)]
    pub fn sys_rst_ps(&mut self) -> SysRstPsW<SysMaskSpec> {
        SysRstPsW::new(self, 3)
    }
    #[doc = "Bit 4 - Запрещение формирования сброса системного домена при появлении сигнала BOR"]
    #[inline(always)]
    pub fn sys_rst_bor(&mut self) -> SysRstBorW<SysMaskSpec> {
        SysRstBorW::new(self, 4)
    }
    #[doc = "Bit 5 - Запрещение формирования сброса батарейного домена при появлении сигнала BOR"]
    #[inline(always)]
    pub fn bu_rst_bor(&mut self) -> BuRstBorW<SysMaskSpec> {
        BuRstBorW::new(self, 5)
    }
}
#[doc = "Маски событий для включения и сброса системного домена\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysMaskSpec;
impl crate::RegisterSpec for SysMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_mask::R`](R) reader structure"]
impl crate::Readable for SysMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`sys_mask::W`](W) writer structure"]
impl crate::Writable for SysMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYS_MASK to value 0x10"]
impl crate::Resettable for SysMaskSpec {
    const RESET_VALUE: u32 = 0x10;
}
