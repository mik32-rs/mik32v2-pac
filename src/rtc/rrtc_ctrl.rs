#[doc = "Register `RRTC_CTRL` reader"]
pub type R = crate::R<RrtcCtrlSpec>;
#[doc = "Register `RRTC_CTRL` writer"]
pub type W = crate::W<RrtcCtrlSpec>;
#[doc = "Запись “1” – сброс признака установки активного уровня ALARM_PAD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlrmPad {
    #[doc = "1: Cброс признака установки активного уровня ALARM_PAD"]
    AlarmReset = 1,
}
impl From<AlrmPad> for bool {
    #[inline(always)]
    fn from(variant: AlrmPad) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRM_PAD` writer - Запись “1” – сброс признака установки активного уровня ALARM_PAD"]
pub type AlrmPadW<'a, REG> = crate::BitWriter<'a, REG, AlrmPad>;
impl<'a, REG> AlrmPadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cброс признака установки активного уровня ALARM_PAD"]
    #[inline(always)]
    pub fn alarm_reset(self) -> &'a mut crate::W<REG> {
        self.variant(AlrmPad::AlarmReset)
    }
}
#[doc = "Сигнал проведения синхронизации между тактовыми доменами. После записи в любой регистр и пока данный бит читается равным «1», запрещено выполнять любую новую запись\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flag {
    #[doc = "0: Запись в регистры разрешена"]
    Ready = 0,
    #[doc = "1: Выполняется синхронизация, запись в регистры запрещена"]
    Synchronization = 1,
}
impl From<Flag> for bool {
    #[inline(always)]
    fn from(variant: Flag) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLAG` reader - Сигнал проведения синхронизации между тактовыми доменами. После записи в любой регистр и пока данный бит читается равным «1», запрещено выполнять любую новую запись"]
pub type FlagR = crate::BitReader<Flag>;
impl FlagR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flag {
        match self.bits {
            false => Flag::Ready,
            true => Flag::Synchronization,
        }
    }
    #[doc = "Запись в регистры разрешена"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Flag::Ready
    }
    #[doc = "Выполняется синхронизация, запись в регистры запрещена"]
    #[inline(always)]
    pub fn is_synchronization(&self) -> bool {
        *self == Flag::Synchronization
    }
}
#[doc = "Бит разрешения прерывания на выходах irq и irq_async. Прерывания появляются при наличии установленного бита ALRM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inte {
    #[doc = "0: Прерывания блока запрещены"]
    Disabled = 0,
    #[doc = "1: Прерывания блока разрешены"]
    Enable = 1,
}
impl From<Inte> for bool {
    #[inline(always)]
    fn from(variant: Inte) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTE` reader - Бит разрешения прерывания на выходах irq и irq_async. Прерывания появляются при наличии установленного бита ALRM"]
pub type InteR = crate::BitReader<Inte>;
impl InteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inte {
        match self.bits {
            false => Inte::Disabled,
            true => Inte::Enable,
        }
    }
    #[doc = "Прерывания блока запрещены"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Inte::Disabled
    }
    #[doc = "Прерывания блока разрешены"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Inte::Enable
    }
}
#[doc = "Field `INTE` writer - Бит разрешения прерывания на выходах irq и irq_async. Прерывания появляются при наличии установленного бита ALRM"]
pub type InteW<'a, REG> = crate::BitWriter<'a, REG, Inte>;
impl<'a, REG> InteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывания блока запрещены"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Inte::Disabled)
    }
    #[doc = "Прерывания блока разрешены"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Inte::Enable)
    }
}
#[doc = "Установленный бит свидетельствует о совпадении одного разрешенного или всех разрешённых полей будильника. Бит необходимо сбрасывать при помощи управляющего ПО\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alrm {
    #[doc = "0: Будильник не сработал"]
    Inactive = 0,
    #[doc = "1: Будильник сработал"]
    Active = 1,
}
impl From<Alrm> for bool {
    #[inline(always)]
    fn from(variant: Alrm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALRM` reader - Установленный бит свидетельствует о совпадении одного разрешенного или всех разрешённых полей будильника. Бит необходимо сбрасывать при помощи управляющего ПО"]
pub type AlrmR = crate::BitReader<Alrm>;
impl AlrmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alrm {
        match self.bits {
            false => Alrm::Inactive,
            true => Alrm::Active,
        }
    }
    #[doc = "Будильник не сработал"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Alrm::Inactive
    }
    #[doc = "Будильник сработал"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Alrm::Active
    }
}
#[doc = "Field `ALRM` writer - Установленный бит свидетельствует о совпадении одного разрешенного или всех разрешённых полей будильника. Бит необходимо сбрасывать при помощи управляющего ПО"]
pub type AlrmW<'a, REG> = crate::BitWriter<'a, REG, Alrm>;
impl<'a, REG> AlrmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Будильник не сработал"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Alrm::Inactive)
    }
    #[doc = "Будильник сработал"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(Alrm::Active)
    }
}
#[doc = "Модуль включён и производит отсчёт времени, когда установлен\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Модуль отключен"]
    Disabled = 0,
    #[doc = "1: Модуль включен"]
    Enable = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Модуль включён и производит отсчёт времени, когда установлен"]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::Disabled,
            true => En::Enable,
        }
    }
    #[doc = "Модуль отключен"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == En::Disabled
    }
    #[doc = "Модуль включен"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == En::Enable
    }
}
#[doc = "Field `EN` writer - Модуль включён и производит отсчёт времени, когда установлен"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Модуль отключен"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(En::Disabled)
    }
    #[doc = "Модуль включен"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(En::Enable)
    }
}
impl R {
    #[doc = "Bit 28 - Сигнал проведения синхронизации между тактовыми доменами. После записи в любой регистр и пока данный бит читается равным «1», запрещено выполнять любую новую запись"]
    #[inline(always)]
    pub fn flag(&self) -> FlagR {
        FlagR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Бит разрешения прерывания на выходах irq и irq_async. Прерывания появляются при наличии установленного бита ALRM"]
    #[inline(always)]
    pub fn inte(&self) -> InteR {
        InteR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Установленный бит свидетельствует о совпадении одного разрешенного или всех разрешённых полей будильника. Бит необходимо сбрасывать при помощи управляющего ПО"]
    #[inline(always)]
    pub fn alrm(&self) -> AlrmR {
        AlrmR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Модуль включён и производит отсчёт времени, когда установлен"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - Запись “1” – сброс признака установки активного уровня ALARM_PAD"]
    #[inline(always)]
    pub fn alrm_pad(&mut self) -> AlrmPadW<RrtcCtrlSpec> {
        AlrmPadW::new(self, 27)
    }
    #[doc = "Bit 29 - Бит разрешения прерывания на выходах irq и irq_async. Прерывания появляются при наличии установленного бита ALRM"]
    #[inline(always)]
    pub fn inte(&mut self) -> InteW<RrtcCtrlSpec> {
        InteW::new(self, 29)
    }
    #[doc = "Bit 30 - Установленный бит свидетельствует о совпадении одного разрешенного или всех разрешённых полей будильника. Бит необходимо сбрасывать при помощи управляющего ПО"]
    #[inline(always)]
    pub fn alrm(&mut self) -> AlrmW<RrtcCtrlSpec> {
        AlrmW::new(self, 30)
    }
    #[doc = "Bit 31 - Модуль включён и производит отсчёт времени, когда установлен"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<RrtcCtrlSpec> {
        EnW::new(self, 31)
    }
}
#[doc = "регистр управления модулем\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RrtcCtrlSpec;
impl crate::RegisterSpec for RrtcCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rrtc_ctrl::R`](R) reader structure"]
impl crate::Readable for RrtcCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rrtc_ctrl::W`](W) writer structure"]
impl crate::Writable for RrtcCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RRTC_CTRL to value 0"]
impl crate::Resettable for RrtcCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
