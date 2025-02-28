#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Селектор тактовых импульсов. Бит CKSEL выбирает, какой источник тактовых импульсов будет использо-вать TIMER16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cksel {
    #[doc = "0: TIMER16 тактируется внутренним источником тактового сигнала (APB тактовый генератор или любой из встроенных генераторов)"]
    Internal = 0,
    #[doc = "1: TIMER16 тактируется внешним источником тактового сигнала через внешний lnput1 TIMER16"]
    External = 1,
}
impl From<Cksel> for bool {
    #[inline(always)]
    fn from(variant: Cksel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKSEL` reader - Селектор тактовых импульсов. Бит CKSEL выбирает, какой источник тактовых импульсов будет использо-вать TIMER16"]
pub type CkselR = crate::BitReader<Cksel>;
impl CkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cksel {
        match self.bits {
            false => Cksel::Internal,
            true => Cksel::External,
        }
    }
    #[doc = "TIMER16 тактируется внутренним источником тактового сигнала (APB тактовый генератор или любой из встроенных генераторов)"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == Cksel::Internal
    }
    #[doc = "TIMER16 тактируется внешним источником тактового сигнала через внешний lnput1 TIMER16"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == Cksel::External
    }
}
#[doc = "Field `CKSEL` writer - Селектор тактовых импульсов. Бит CKSEL выбирает, какой источник тактовых импульсов будет использо-вать TIMER16"]
pub type CkselW<'a, REG> = crate::BitWriter<'a, REG, Cksel>;
impl<'a, REG> CkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIMER16 тактируется внутренним источником тактового сигнала (APB тактовый генератор или любой из встроенных генераторов)"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::Internal)
    }
    #[doc = "TIMER16 тактируется внешним источником тактового сигнала через внешний lnput1 TIMER16"]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(Cksel::External)
    }
}
#[doc = "Полярность синхронизации. Когда TIMER16 тактируется внешним ис-точником тактового сигнала, биты CKPOL используются для настройки активного фронта или фронтов, ис-пользуемых счетчиком\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckpol {
    #[doc = "0: Нарастающий фронт является активным фронтом, используемым для подсчета"]
    Rising = 0,
    #[doc = "1: Спадающий фронт является активным фронтом, используемым для подсчета"]
    Folling = 1,
    #[doc = "2: Оба фронта являются активными фронтами. Когда оба фронта внешнего тактового сигнала считаются активными, TIMER16 должен также тактироваться внутренним источником тактового сигнала с частотой, по крайней мере в четыре раза превышающей частоту внешнего тактового сигнала."]
    BothEdge = 2,
}
impl From<Ckpol> for u8 {
    #[inline(always)]
    fn from(variant: Ckpol) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ckpol {
    type Ux = u8;
}
impl crate::IsEnum for Ckpol {}
#[doc = "Field `CKPOL` reader - Полярность синхронизации. Когда TIMER16 тактируется внешним ис-точником тактового сигнала, биты CKPOL используются для настройки активного фронта или фронтов, ис-пользуемых счетчиком"]
pub type CkpolR = crate::FieldReader<Ckpol>;
impl CkpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ckpol> {
        match self.bits {
            0 => Some(Ckpol::Rising),
            1 => Some(Ckpol::Folling),
            2 => Some(Ckpol::BothEdge),
            _ => None,
        }
    }
    #[doc = "Нарастающий фронт является активным фронтом, используемым для подсчета"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Ckpol::Rising
    }
    #[doc = "Спадающий фронт является активным фронтом, используемым для подсчета"]
    #[inline(always)]
    pub fn is_folling(&self) -> bool {
        *self == Ckpol::Folling
    }
    #[doc = "Оба фронта являются активными фронтами. Когда оба фронта внешнего тактового сигнала считаются активными, TIMER16 должен также тактироваться внутренним источником тактового сигнала с частотой, по крайней мере в четыре раза превышающей частоту внешнего тактового сигнала."]
    #[inline(always)]
    pub fn is_both_edge(&self) -> bool {
        *self == Ckpol::BothEdge
    }
}
#[doc = "Field `CKPOL` writer - Полярность синхронизации. Когда TIMER16 тактируется внешним ис-точником тактового сигнала, биты CKPOL используются для настройки активного фронта или фронтов, ис-пользуемых счетчиком"]
pub type CkpolW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ckpol>;
impl<'a, REG> CkpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Нарастающий фронт является активным фронтом, используемым для подсчета"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Ckpol::Rising)
    }
    #[doc = "Спадающий фронт является активным фронтом, используемым для подсчета"]
    #[inline(always)]
    pub fn folling(self) -> &'a mut crate::W<REG> {
        self.variant(Ckpol::Folling)
    }
    #[doc = "Оба фронта являются активными фронтами. Когда оба фронта внешнего тактового сигнала считаются активными, TIMER16 должен также тактироваться внутренним источником тактового сигнала с частотой, по крайней мере в четыре раза превышающей частоту внешнего тактового сигнала."]
    #[inline(always)]
    pub fn both_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Ckpol::BothEdge)
    }
}
#[doc = "Конфигурируемый цифровой фильтр для внешнего тактового генератора. Значение CKFLT устанавливает количество последовательных одинаковых выборок, которые должны быть обнаружены при изменении уровня внешнего тактового сигнала, прежде чем это будет считаться действительным переходом уровня. Для ис-пользования этой функции необходимо наличие внутреннего источника тактового сигнала.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ckflt {
    #[doc = "0: Любое изменение активного уровня триггера читается действительным триггером"]
    Disable = 0,
    #[doc = "1: Изменение активного уровня триггера должно быть стабильным в течение не менее 2 тактовых периодов, прежде чем он будет считаться действительным триггером."]
    _2clock = 1,
    #[doc = "2: Изменение активного уровня триггера должно быть стабильным в течение как минимум 4 тактовых периодов, прежде чем он будет считаться действительным триггером"]
    _4clock = 2,
    #[doc = "3: Изменение активного уровня триггера должно быть стабильным в течение не менее 8 тактовых периодов, прежде чем он будет считаться действительным триггером"]
    _8clock = 3,
}
impl From<Ckflt> for u8 {
    #[inline(always)]
    fn from(variant: Ckflt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ckflt {
    type Ux = u8;
}
impl crate::IsEnum for Ckflt {}
#[doc = "Field `CKFLT` reader - Конфигурируемый цифровой фильтр для внешнего тактового генератора. Значение CKFLT устанавливает количество последовательных одинаковых выборок, которые должны быть обнаружены при изменении уровня внешнего тактового сигнала, прежде чем это будет считаться действительным переходом уровня. Для ис-пользования этой функции необходимо наличие внутреннего источника тактового сигнала."]
pub type CkfltR = crate::FieldReader<Ckflt>;
impl CkfltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ckflt {
        match self.bits {
            0 => Ckflt::Disable,
            1 => Ckflt::_2clock,
            2 => Ckflt::_4clock,
            3 => Ckflt::_8clock,
            _ => unreachable!(),
        }
    }
    #[doc = "Любое изменение активного уровня триггера читается действительным триггером"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ckflt::Disable
    }
    #[doc = "Изменение активного уровня триггера должно быть стабильным в течение не менее 2 тактовых периодов, прежде чем он будет считаться действительным триггером."]
    #[inline(always)]
    pub fn is_2clock(&self) -> bool {
        *self == Ckflt::_2clock
    }
    #[doc = "Изменение активного уровня триггера должно быть стабильным в течение как минимум 4 тактовых периодов, прежде чем он будет считаться действительным триггером"]
    #[inline(always)]
    pub fn is_4clock(&self) -> bool {
        *self == Ckflt::_4clock
    }
    #[doc = "Изменение активного уровня триггера должно быть стабильным в течение не менее 8 тактовых периодов, прежде чем он будет считаться действительным триггером"]
    #[inline(always)]
    pub fn is_8clock(&self) -> bool {
        *self == Ckflt::_8clock
    }
}
#[doc = "Field `CKFLT` writer - Конфигурируемый цифровой фильтр для внешнего тактового генератора. Значение CKFLT устанавливает количество последовательных одинаковых выборок, которые должны быть обнаружены при изменении уровня внешнего тактового сигнала, прежде чем это будет считаться действительным переходом уровня. Для ис-пользования этой функции необходимо наличие внутреннего источника тактового сигнала."]
pub type CkfltW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ckflt, crate::Safe>;
impl<'a, REG> CkfltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Любое изменение активного уровня триггера читается действительным триггером"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ckflt::Disable)
    }
    #[doc = "Изменение активного уровня триггера должно быть стабильным в течение не менее 2 тактовых периодов, прежде чем он будет считаться действительным триггером."]
    #[inline(always)]
    pub fn _2clock(self) -> &'a mut crate::W<REG> {
        self.variant(Ckflt::_2clock)
    }
    #[doc = "Изменение активного уровня триггера должно быть стабильным в течение как минимум 4 тактовых периодов, прежде чем он будет считаться действительным триггером"]
    #[inline(always)]
    pub fn _4clock(self) -> &'a mut crate::W<REG> {
        self.variant(Ckflt::_4clock)
    }
    #[doc = "Изменение активного уровня триггера должно быть стабильным в течение не менее 8 тактовых периодов, прежде чем он будет считаться действительным триггером"]
    #[inline(always)]
    pub fn _8clock(self) -> &'a mut crate::W<REG> {
        self.variant(Ckflt::_8clock)
    }
}
#[doc = "Конфигурируемый цифровой фильтр для триггера. Значение TRGFLT устанавливает количество последовательных одинаковых выборок, которые должны быть обнаружены при изменении уровня на внутреннем триггере, прежде чем это будет считаться действительным переходом уровня. Для использования этой функции необходимо наличие источника внутреннего тактового сигнала.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgflt {
    #[doc = "0: Любое изменение активного уровня триггера читается действительным триггером"]
    Disable = 0,
    #[doc = "1: Изменение активного уровня триггера должно быть стабильным в течение не менее 2 тактовых периодов, прежде чем он будет считаться действительным триггером."]
    _2clock = 1,
    #[doc = "2: Изменение активного уровня триггера должно быть стабильным в течение как минимум 4 тактовых периодов, прежде чем он будет считаться действительным триггером"]
    _4clock = 2,
    #[doc = "3: Изменение активного уровня триггера должно быть стабильным в течение не менее 8 тактовых периодов, прежде чем он будет считаться действительным триггером"]
    _8clock = 3,
}
impl From<Trgflt> for u8 {
    #[inline(always)]
    fn from(variant: Trgflt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgflt {
    type Ux = u8;
}
impl crate::IsEnum for Trgflt {}
#[doc = "Field `TRGFLT` reader - Конфигурируемый цифровой фильтр для триггера. Значение TRGFLT устанавливает количество последовательных одинаковых выборок, которые должны быть обнаружены при изменении уровня на внутреннем триггере, прежде чем это будет считаться действительным переходом уровня. Для использования этой функции необходимо наличие источника внутреннего тактового сигнала."]
pub type TrgfltR = crate::FieldReader<Trgflt>;
impl TrgfltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgflt {
        match self.bits {
            0 => Trgflt::Disable,
            1 => Trgflt::_2clock,
            2 => Trgflt::_4clock,
            3 => Trgflt::_8clock,
            _ => unreachable!(),
        }
    }
    #[doc = "Любое изменение активного уровня триггера читается действительным триггером"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Trgflt::Disable
    }
    #[doc = "Изменение активного уровня триггера должно быть стабильным в течение не менее 2 тактовых периодов, прежде чем он будет считаться действительным триггером."]
    #[inline(always)]
    pub fn is_2clock(&self) -> bool {
        *self == Trgflt::_2clock
    }
    #[doc = "Изменение активного уровня триггера должно быть стабильным в течение как минимум 4 тактовых периодов, прежде чем он будет считаться действительным триггером"]
    #[inline(always)]
    pub fn is_4clock(&self) -> bool {
        *self == Trgflt::_4clock
    }
    #[doc = "Изменение активного уровня триггера должно быть стабильным в течение не менее 8 тактовых периодов, прежде чем он будет считаться действительным триггером"]
    #[inline(always)]
    pub fn is_8clock(&self) -> bool {
        *self == Trgflt::_8clock
    }
}
#[doc = "Field `TRGFLT` writer - Конфигурируемый цифровой фильтр для триггера. Значение TRGFLT устанавливает количество последовательных одинаковых выборок, которые должны быть обнаружены при изменении уровня на внутреннем триггере, прежде чем это будет считаться действительным переходом уровня. Для использования этой функции необходимо наличие источника внутреннего тактового сигнала."]
pub type TrgfltW<'a, REG> = crate::FieldWriter<'a, REG, 2, Trgflt, crate::Safe>;
impl<'a, REG> TrgfltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Любое изменение активного уровня триггера читается действительным триггером"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Trgflt::Disable)
    }
    #[doc = "Изменение активного уровня триггера должно быть стабильным в течение не менее 2 тактовых периодов, прежде чем он будет считаться действительным триггером."]
    #[inline(always)]
    pub fn _2clock(self) -> &'a mut crate::W<REG> {
        self.variant(Trgflt::_2clock)
    }
    #[doc = "Изменение активного уровня триггера должно быть стабильным в течение как минимум 4 тактовых периодов, прежде чем он будет считаться действительным триггером"]
    #[inline(always)]
    pub fn _4clock(self) -> &'a mut crate::W<REG> {
        self.variant(Trgflt::_4clock)
    }
    #[doc = "Изменение активного уровня триггера должно быть стабильным в течение не менее 8 тактовых периодов, прежде чем он будет считаться действительным триггером"]
    #[inline(always)]
    pub fn _8clock(self) -> &'a mut crate::W<REG> {
        self.variant(Trgflt::_8clock)
    }
}
#[doc = "Делитель частоты. Биты PRESC задают коэффициент деления делителя. /n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Presc {
    #[doc = "0: Делитель 1"]
    _1 = 0,
    #[doc = "1: Делитель 2"]
    _2 = 1,
    #[doc = "2: Делитель 4"]
    _4 = 2,
    #[doc = "3: Делитель 8"]
    _8 = 3,
    #[doc = "4: Делитель 16"]
    _16 = 4,
    #[doc = "5: Делитель 32"]
    _32 = 5,
    #[doc = "6: Делитель 64"]
    _64 = 6,
    #[doc = "7: Делитель 128"]
    _128 = 7,
}
impl From<Presc> for u8 {
    #[inline(always)]
    fn from(variant: Presc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Presc {
    type Ux = u8;
}
impl crate::IsEnum for Presc {}
#[doc = "Field `PRESC` reader - Делитель частоты. Биты PRESC задают коэффициент деления делителя. /n"]
pub type PrescR = crate::FieldReader<Presc>;
impl PrescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Presc {
        match self.bits {
            0 => Presc::_1,
            1 => Presc::_2,
            2 => Presc::_4,
            3 => Presc::_8,
            4 => Presc::_16,
            5 => Presc::_32,
            6 => Presc::_64,
            7 => Presc::_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Делитель 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Presc::_1
    }
    #[doc = "Делитель 2"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Presc::_2
    }
    #[doc = "Делитель 4"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Presc::_4
    }
    #[doc = "Делитель 8"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Presc::_8
    }
    #[doc = "Делитель 16"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == Presc::_16
    }
    #[doc = "Делитель 32"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == Presc::_32
    }
    #[doc = "Делитель 64"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == Presc::_64
    }
    #[doc = "Делитель 128"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == Presc::_128
    }
}
#[doc = "Field `PRESC` writer - Делитель частоты. Биты PRESC задают коэффициент деления делителя. /n"]
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 3, Presc, crate::Safe>;
impl<'a, REG> PrescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Делитель 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::_1)
    }
    #[doc = "Делитель 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::_2)
    }
    #[doc = "Делитель 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::_4)
    }
    #[doc = "Делитель 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::_8)
    }
    #[doc = "Делитель 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::_16)
    }
    #[doc = "Делитель 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::_32)
    }
    #[doc = "Делитель 64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::_64)
    }
    #[doc = "Делитель 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::_128)
    }
}
#[doc = "Селектор триггера. Биты TRIGSEL выбирают источник триггера, который будет служить событием запуска для TIMER16, из 8 доступных источников.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trigsel {
    #[doc = "0: Источник триггера GPIO2_3"]
    Gpio2_3 = 0,
    #[doc = "1: Источник триггера GPIO2_2"]
    Gpio2_2 = 1,
    #[doc = "2: Источник триггера GPIO2_1"]
    Gpio2_1 = 2,
    #[doc = "3: Источник триггера GPIO2_0"]
    Gpio2_0 = 3,
    #[doc = "4: Источник триггера - окончание преобразования термосенсора"]
    Tsens = 4,
    #[doc = "5: окончание преобразования АЦП"]
    Adc = 5,
    #[doc = "6: Источник триггера - прерывание RTC"]
    RtcIrq = 6,
    #[doc = "7: Будильник"]
    Alarm = 7,
}
impl From<Trigsel> for u8 {
    #[inline(always)]
    fn from(variant: Trigsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trigsel {
    type Ux = u8;
}
impl crate::IsEnum for Trigsel {}
#[doc = "Field `TRIGSEL` reader - Селектор триггера. Биты TRIGSEL выбирают источник триггера, который будет служить событием запуска для TIMER16, из 8 доступных источников."]
pub type TrigselR = crate::FieldReader<Trigsel>;
impl TrigselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trigsel {
        match self.bits {
            0 => Trigsel::Gpio2_3,
            1 => Trigsel::Gpio2_2,
            2 => Trigsel::Gpio2_1,
            3 => Trigsel::Gpio2_0,
            4 => Trigsel::Tsens,
            5 => Trigsel::Adc,
            6 => Trigsel::RtcIrq,
            7 => Trigsel::Alarm,
            _ => unreachable!(),
        }
    }
    #[doc = "Источник триггера GPIO2_3"]
    #[inline(always)]
    pub fn is_gpio2_3(&self) -> bool {
        *self == Trigsel::Gpio2_3
    }
    #[doc = "Источник триггера GPIO2_2"]
    #[inline(always)]
    pub fn is_gpio2_2(&self) -> bool {
        *self == Trigsel::Gpio2_2
    }
    #[doc = "Источник триггера GPIO2_1"]
    #[inline(always)]
    pub fn is_gpio2_1(&self) -> bool {
        *self == Trigsel::Gpio2_1
    }
    #[doc = "Источник триггера GPIO2_0"]
    #[inline(always)]
    pub fn is_gpio2_0(&self) -> bool {
        *self == Trigsel::Gpio2_0
    }
    #[doc = "Источник триггера - окончание преобразования термосенсора"]
    #[inline(always)]
    pub fn is_tsens(&self) -> bool {
        *self == Trigsel::Tsens
    }
    #[doc = "окончание преобразования АЦП"]
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        *self == Trigsel::Adc
    }
    #[doc = "Источник триггера - прерывание RTC"]
    #[inline(always)]
    pub fn is_rtc_irq(&self) -> bool {
        *self == Trigsel::RtcIrq
    }
    #[doc = "Будильник"]
    #[inline(always)]
    pub fn is_alarm(&self) -> bool {
        *self == Trigsel::Alarm
    }
}
#[doc = "Field `TRIGSEL` writer - Селектор триггера. Биты TRIGSEL выбирают источник триггера, который будет служить событием запуска для TIMER16, из 8 доступных источников."]
pub type TrigselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Trigsel, crate::Safe>;
impl<'a, REG> TrigselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Источник триггера GPIO2_3"]
    #[inline(always)]
    pub fn gpio2_3(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Gpio2_3)
    }
    #[doc = "Источник триггера GPIO2_2"]
    #[inline(always)]
    pub fn gpio2_2(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Gpio2_2)
    }
    #[doc = "Источник триггера GPIO2_1"]
    #[inline(always)]
    pub fn gpio2_1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Gpio2_1)
    }
    #[doc = "Источник триггера GPIO2_0"]
    #[inline(always)]
    pub fn gpio2_0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Gpio2_0)
    }
    #[doc = "Источник триггера - окончание преобразования термосенсора"]
    #[inline(always)]
    pub fn tsens(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Tsens)
    }
    #[doc = "окончание преобразования АЦП"]
    #[inline(always)]
    pub fn adc(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Adc)
    }
    #[doc = "Источник триггера - прерывание RTC"]
    #[inline(always)]
    pub fn rtc_irq(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::RtcIrq)
    }
    #[doc = "Будильник"]
    #[inline(always)]
    pub fn alarm(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsel::Alarm)
    }
}
#[doc = "Разрешение и полярность триггера.Бит TRIGEN управляет тем, запускается ли счетчик TIMER16 внешним триггером или нет. Если выбрана опция внешнего запуска, возможны три конфигурации активного фронта триггера\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trigen {
    #[doc = "0: Программный триггер (начало отсчета инициируется программно)"]
    Software = 0,
    #[doc = "1: Нарастающий фронт является активным фронтом"]
    Rising = 1,
    #[doc = "2: Падающий фронт является активным фронтом"]
    Folling = 2,
    #[doc = "3: Оба фронта являются активными фронтами энергопотреблением"]
    BothEdge = 3,
}
impl From<Trigen> for u8 {
    #[inline(always)]
    fn from(variant: Trigen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trigen {
    type Ux = u8;
}
impl crate::IsEnum for Trigen {}
#[doc = "Field `TRIGEN` reader - Разрешение и полярность триггера.Бит TRIGEN управляет тем, запускается ли счетчик TIMER16 внешним триггером или нет. Если выбрана опция внешнего запуска, возможны три конфигурации активного фронта триггера"]
pub type TrigenR = crate::FieldReader<Trigen>;
impl TrigenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trigen {
        match self.bits {
            0 => Trigen::Software,
            1 => Trigen::Rising,
            2 => Trigen::Folling,
            3 => Trigen::BothEdge,
            _ => unreachable!(),
        }
    }
    #[doc = "Программный триггер (начало отсчета инициируется программно)"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == Trigen::Software
    }
    #[doc = "Нарастающий фронт является активным фронтом"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Trigen::Rising
    }
    #[doc = "Падающий фронт является активным фронтом"]
    #[inline(always)]
    pub fn is_folling(&self) -> bool {
        *self == Trigen::Folling
    }
    #[doc = "Оба фронта являются активными фронтами энергопотреблением"]
    #[inline(always)]
    pub fn is_both_edge(&self) -> bool {
        *self == Trigen::BothEdge
    }
}
#[doc = "Field `TRIGEN` writer - Разрешение и полярность триггера.Бит TRIGEN управляет тем, запускается ли счетчик TIMER16 внешним триггером или нет. Если выбрана опция внешнего запуска, возможны три конфигурации активного фронта триггера"]
pub type TrigenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Trigen, crate::Safe>;
impl<'a, REG> TrigenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Программный триггер (начало отсчета инициируется программно)"]
    #[inline(always)]
    pub fn software(self) -> &'a mut crate::W<REG> {
        self.variant(Trigen::Software)
    }
    #[doc = "Нарастающий фронт является активным фронтом"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Trigen::Rising)
    }
    #[doc = "Падающий фронт является активным фронтом"]
    #[inline(always)]
    pub fn folling(self) -> &'a mut crate::W<REG> {
        self.variant(Trigen::Folling)
    }
    #[doc = "Оба фронта являются активными фронтами энергопотреблением"]
    #[inline(always)]
    pub fn both_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Trigen::BothEdge)
    }
}
#[doc = "Разрешение тайм-аута. Бит TIMOUT управляет функцией тайм-аута\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timeout {
    #[doc = "0: Триггерное событие, поступающее, когда таймер уже запущен, будет проигнорировано"]
    TriggerEventIgnor = 0,
    #[doc = "1: Триггерное событие, поступающее, когда таймер уже запущен, сбросит и перезапустит счетчик"]
    TriggerEventRestart = 1,
}
impl From<Timeout> for bool {
    #[inline(always)]
    fn from(variant: Timeout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUT` reader - Разрешение тайм-аута. Бит TIMOUT управляет функцией тайм-аута"]
pub type TimeoutR = crate::BitReader<Timeout>;
impl TimeoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timeout {
        match self.bits {
            false => Timeout::TriggerEventIgnor,
            true => Timeout::TriggerEventRestart,
        }
    }
    #[doc = "Триггерное событие, поступающее, когда таймер уже запущен, будет проигнорировано"]
    #[inline(always)]
    pub fn is_trigger_event_ignor(&self) -> bool {
        *self == Timeout::TriggerEventIgnor
    }
    #[doc = "Триггерное событие, поступающее, когда таймер уже запущен, сбросит и перезапустит счетчик"]
    #[inline(always)]
    pub fn is_trigger_event_restart(&self) -> bool {
        *self == Timeout::TriggerEventRestart
    }
}
#[doc = "Field `TIMEOUT` writer - Разрешение тайм-аута. Бит TIMOUT управляет функцией тайм-аута"]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG, Timeout>;
impl<'a, REG> TimeoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Триггерное событие, поступающее, когда таймер уже запущен, будет проигнорировано"]
    #[inline(always)]
    pub fn trigger_event_ignor(self) -> &'a mut crate::W<REG> {
        self.variant(Timeout::TriggerEventIgnor)
    }
    #[doc = "Триггерное событие, поступающее, когда таймер уже запущен, сбросит и перезапустит счетчик"]
    #[inline(always)]
    pub fn trigger_event_restart(self) -> &'a mut crate::W<REG> {
        self.variant(Timeout::TriggerEventRestart)
    }
}
#[doc = "Форма волны. Бит WAVE управляет формой выходного сигнала\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wave {
    #[doc = "0: Деактивировать режим Set-once, форма волны ШИМ (PWM) / один импульс (One shot)"]
    PwmOrOneShot = 0,
    #[doc = "1: Активировать режим Set-once"]
    SetOnce = 1,
}
impl From<Wave> for bool {
    #[inline(always)]
    fn from(variant: Wave) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAVE` reader - Форма волны. Бит WAVE управляет формой выходного сигнала"]
pub type WaveR = crate::BitReader<Wave>;
impl WaveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wave {
        match self.bits {
            false => Wave::PwmOrOneShot,
            true => Wave::SetOnce,
        }
    }
    #[doc = "Деактивировать режим Set-once, форма волны ШИМ (PWM) / один импульс (One shot)"]
    #[inline(always)]
    pub fn is_pwm_or_one_shot(&self) -> bool {
        *self == Wave::PwmOrOneShot
    }
    #[doc = "Активировать режим Set-once"]
    #[inline(always)]
    pub fn is_set_once(&self) -> bool {
        *self == Wave::SetOnce
    }
}
#[doc = "Field `WAVE` writer - Форма волны. Бит WAVE управляет формой выходного сигнала"]
pub type WaveW<'a, REG> = crate::BitWriter<'a, REG, Wave>;
impl<'a, REG> WaveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Деактивировать режим Set-once, форма волны ШИМ (PWM) / один импульс (One shot)"]
    #[inline(always)]
    pub fn pwm_or_one_shot(self) -> &'a mut crate::W<REG> {
        self.variant(Wave::PwmOrOneShot)
    }
    #[doc = "Активировать режим Set-once"]
    #[inline(always)]
    pub fn set_once(self) -> &'a mut crate::W<REG> {
        self.variant(Wave::SetOnce)
    }
}
#[doc = "Полярность формы волны. Бит WAVEPOL управляет полярностью выходного сигнала\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wavwpol {
    #[doc = "0: Выход TIMER16 отражает результаты сравнения между регистрами ARR и CMP"]
    Noninverted = 0,
    #[doc = "1: Выход TIMER16 отражает инверсные результаты сравнения между регистрами ARR и CMP"]
    Inverted = 1,
}
impl From<Wavwpol> for bool {
    #[inline(always)]
    fn from(variant: Wavwpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAVWPOL` reader - Полярность формы волны. Бит WAVEPOL управляет полярностью выходного сигнала"]
pub type WavwpolR = crate::BitReader<Wavwpol>;
impl WavwpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wavwpol {
        match self.bits {
            false => Wavwpol::Noninverted,
            true => Wavwpol::Inverted,
        }
    }
    #[doc = "Выход TIMER16 отражает результаты сравнения между регистрами ARR и CMP"]
    #[inline(always)]
    pub fn is_noninverted(&self) -> bool {
        *self == Wavwpol::Noninverted
    }
    #[doc = "Выход TIMER16 отражает инверсные результаты сравнения между регистрами ARR и CMP"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == Wavwpol::Inverted
    }
}
#[doc = "Field `WAVWPOL` writer - Полярность формы волны. Бит WAVEPOL управляет полярностью выходного сигнала"]
pub type WavwpolW<'a, REG> = crate::BitWriter<'a, REG, Wavwpol>;
impl<'a, REG> WavwpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Выход TIMER16 отражает результаты сравнения между регистрами ARR и CMP"]
    #[inline(always)]
    pub fn noninverted(self) -> &'a mut crate::W<REG> {
        self.variant(Wavwpol::Noninverted)
    }
    #[doc = "Выход TIMER16 отражает инверсные результаты сравнения между регистрами ARR и CMP"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Wavwpol::Inverted)
    }
}
#[doc = "Режим обновления регистров. Бит PRELOAD управляет модальностью обновления регистров ARR и CMP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Preload {
    #[doc = "0: Регистры обновляются после каждого доступа к записи на шине APB"]
    AfterWrite = 0,
    #[doc = "1: Регистры обновляются в конце текущего периода TIMER16"]
    EndPeriod = 1,
}
impl From<Preload> for bool {
    #[inline(always)]
    fn from(variant: Preload) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRELOAD` reader - Режим обновления регистров. Бит PRELOAD управляет модальностью обновления регистров ARR и CMP"]
pub type PreloadR = crate::BitReader<Preload>;
impl PreloadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Preload {
        match self.bits {
            false => Preload::AfterWrite,
            true => Preload::EndPeriod,
        }
    }
    #[doc = "Регистры обновляются после каждого доступа к записи на шине APB"]
    #[inline(always)]
    pub fn is_after_write(&self) -> bool {
        *self == Preload::AfterWrite
    }
    #[doc = "Регистры обновляются в конце текущего периода TIMER16"]
    #[inline(always)]
    pub fn is_end_period(&self) -> bool {
        *self == Preload::EndPeriod
    }
}
#[doc = "Field `PRELOAD` writer - Режим обновления регистров. Бит PRELOAD управляет модальностью обновления регистров ARR и CMP"]
pub type PreloadW<'a, REG> = crate::BitWriter<'a, REG, Preload>;
impl<'a, REG> PreloadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Регистры обновляются после каждого доступа к записи на шине APB"]
    #[inline(always)]
    pub fn after_write(self) -> &'a mut crate::W<REG> {
        self.variant(Preload::AfterWrite)
    }
    #[doc = "Регистры обновляются в конце текущего периода TIMER16"]
    #[inline(always)]
    pub fn end_period(self) -> &'a mut crate::W<REG> {
        self.variant(Preload::EndPeriod)
    }
}
#[doc = "Бит COUNTMODE выбирает, какой источник тактового сигнала используется TIMER16 для синхронизации счетчика\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CountMode {
    #[doc = "0: Счетчик инкрементируется после каждого внутреннего тактового импульса"]
    Internal = 0,
    #[doc = "1: Счетчик увеличивается после каждого действительного тактового импульса на внешнем lnput1 TIMER16"]
    External = 1,
}
impl From<CountMode> for bool {
    #[inline(always)]
    fn from(variant: CountMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COUNT_MODE` reader - Бит COUNTMODE выбирает, какой источник тактового сигнала используется TIMER16 для синхронизации счетчика"]
pub type CountModeR = crate::BitReader<CountMode>;
impl CountModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CountMode {
        match self.bits {
            false => CountMode::Internal,
            true => CountMode::External,
        }
    }
    #[doc = "Счетчик инкрементируется после каждого внутреннего тактового импульса"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == CountMode::Internal
    }
    #[doc = "Счетчик увеличивается после каждого действительного тактового импульса на внешнем lnput1 TIMER16"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == CountMode::External
    }
}
#[doc = "Field `COUNT_MODE` writer - Бит COUNTMODE выбирает, какой источник тактового сигнала используется TIMER16 для синхронизации счетчика"]
pub type CountModeW<'a, REG> = crate::BitWriter<'a, REG, CountMode>;
impl<'a, REG> CountModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Счетчик инкрементируется после каждого внутреннего тактового импульса"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(CountMode::Internal)
    }
    #[doc = "Счетчик увеличивается после каждого действительного тактового импульса на внешнем lnput1 TIMER16"]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(CountMode::External)
    }
}
#[doc = "Разрешение режима энкодера. Бит ENC управляет режимом работы энкодера\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enc {
    #[doc = "0: Режим энкодера отключен"]
    Disable = 0,
    #[doc = "1: Режим энэнкодера включен"]
    Enable = 1,
}
impl From<Enc> for bool {
    #[inline(always)]
    fn from(variant: Enc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENC` reader - Разрешение режима энкодера. Бит ENC управляет режимом работы энкодера"]
pub type EncR = crate::BitReader<Enc>;
impl EncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enc {
        match self.bits {
            false => Enc::Disable,
            true => Enc::Enable,
        }
    }
    #[doc = "Режим энкодера отключен"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Enc::Disable
    }
    #[doc = "Режим энэнкодера включен"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Enc::Enable
    }
}
#[doc = "Field `ENC` writer - Разрешение режима энкодера. Бит ENC управляет режимом работы энкодера"]
pub type EncW<'a, REG> = crate::BitWriter<'a, REG, Enc>;
impl<'a, REG> EncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Режим энкодера отключен"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enc::Disable)
    }
    #[doc = "Режим энэнкодера включен"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enc::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Селектор тактовых импульсов. Бит CKSEL выбирает, какой источник тактовых импульсов будет использо-вать TIMER16"]
    #[inline(always)]
    pub fn cksel(&self) -> CkselR {
        CkselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Полярность синхронизации. Когда TIMER16 тактируется внешним ис-точником тактового сигнала, биты CKPOL используются для настройки активного фронта или фронтов, ис-пользуемых счетчиком"]
    #[inline(always)]
    pub fn ckpol(&self) -> CkpolR {
        CkpolR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Конфигурируемый цифровой фильтр для внешнего тактового генератора. Значение CKFLT устанавливает количество последовательных одинаковых выборок, которые должны быть обнаружены при изменении уровня внешнего тактового сигнала, прежде чем это будет считаться действительным переходом уровня. Для ис-пользования этой функции необходимо наличие внутреннего источника тактового сигнала."]
    #[inline(always)]
    pub fn ckflt(&self) -> CkfltR {
        CkfltR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Конфигурируемый цифровой фильтр для триггера. Значение TRGFLT устанавливает количество последовательных одинаковых выборок, которые должны быть обнаружены при изменении уровня на внутреннем триггере, прежде чем это будет считаться действительным переходом уровня. Для использования этой функции необходимо наличие источника внутреннего тактового сигнала."]
    #[inline(always)]
    pub fn trgflt(&self) -> TrgfltR {
        TrgfltR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 9:11 - Делитель частоты. Биты PRESC задают коэффициент деления делителя. /n"]
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Селектор триггера. Биты TRIGSEL выбирают источник триггера, который будет служить событием запуска для TIMER16, из 8 доступных источников."]
    #[inline(always)]
    pub fn trigsel(&self) -> TrigselR {
        TrigselR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 17:18 - Разрешение и полярность триггера.Бит TRIGEN управляет тем, запускается ли счетчик TIMER16 внешним триггером или нет. Если выбрана опция внешнего запуска, возможны три конфигурации активного фронта триггера"]
    #[inline(always)]
    pub fn trigen(&self) -> TrigenR {
        TrigenR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - Разрешение тайм-аута. Бит TIMOUT управляет функцией тайм-аута"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Форма волны. Бит WAVE управляет формой выходного сигнала"]
    #[inline(always)]
    pub fn wave(&self) -> WaveR {
        WaveR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Полярность формы волны. Бит WAVEPOL управляет полярностью выходного сигнала"]
    #[inline(always)]
    pub fn wavwpol(&self) -> WavwpolR {
        WavwpolR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Режим обновления регистров. Бит PRELOAD управляет модальностью обновления регистров ARR и CMP"]
    #[inline(always)]
    pub fn preload(&self) -> PreloadR {
        PreloadR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Бит COUNTMODE выбирает, какой источник тактового сигнала используется TIMER16 для синхронизации счетчика"]
    #[inline(always)]
    pub fn count_mode(&self) -> CountModeR {
        CountModeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Разрешение режима энкодера. Бит ENC управляет режимом работы энкодера"]
    #[inline(always)]
    pub fn enc(&self) -> EncR {
        EncR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Селектор тактовых импульсов. Бит CKSEL выбирает, какой источник тактовых импульсов будет использо-вать TIMER16"]
    #[inline(always)]
    pub fn cksel(&mut self) -> CkselW<CfgrSpec> {
        CkselW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Полярность синхронизации. Когда TIMER16 тактируется внешним ис-точником тактового сигнала, биты CKPOL используются для настройки активного фронта или фронтов, ис-пользуемых счетчиком"]
    #[inline(always)]
    pub fn ckpol(&mut self) -> CkpolW<CfgrSpec> {
        CkpolW::new(self, 1)
    }
    #[doc = "Bits 3:4 - Конфигурируемый цифровой фильтр для внешнего тактового генератора. Значение CKFLT устанавливает количество последовательных одинаковых выборок, которые должны быть обнаружены при изменении уровня внешнего тактового сигнала, прежде чем это будет считаться действительным переходом уровня. Для ис-пользования этой функции необходимо наличие внутреннего источника тактового сигнала."]
    #[inline(always)]
    pub fn ckflt(&mut self) -> CkfltW<CfgrSpec> {
        CkfltW::new(self, 3)
    }
    #[doc = "Bits 6:7 - Конфигурируемый цифровой фильтр для триггера. Значение TRGFLT устанавливает количество последовательных одинаковых выборок, которые должны быть обнаружены при изменении уровня на внутреннем триггере, прежде чем это будет считаться действительным переходом уровня. Для использования этой функции необходимо наличие источника внутреннего тактового сигнала."]
    #[inline(always)]
    pub fn trgflt(&mut self) -> TrgfltW<CfgrSpec> {
        TrgfltW::new(self, 6)
    }
    #[doc = "Bits 9:11 - Делитель частоты. Биты PRESC задают коэффициент деления делителя. /n"]
    #[inline(always)]
    pub fn presc(&mut self) -> PrescW<CfgrSpec> {
        PrescW::new(self, 9)
    }
    #[doc = "Bits 13:15 - Селектор триггера. Биты TRIGSEL выбирают источник триггера, который будет служить событием запуска для TIMER16, из 8 доступных источников."]
    #[inline(always)]
    pub fn trigsel(&mut self) -> TrigselW<CfgrSpec> {
        TrigselW::new(self, 13)
    }
    #[doc = "Bits 17:18 - Разрешение и полярность триггера.Бит TRIGEN управляет тем, запускается ли счетчик TIMER16 внешним триггером или нет. Если выбрана опция внешнего запуска, возможны три конфигурации активного фронта триггера"]
    #[inline(always)]
    pub fn trigen(&mut self) -> TrigenW<CfgrSpec> {
        TrigenW::new(self, 17)
    }
    #[doc = "Bit 19 - Разрешение тайм-аута. Бит TIMOUT управляет функцией тайм-аута"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TimeoutW<CfgrSpec> {
        TimeoutW::new(self, 19)
    }
    #[doc = "Bit 20 - Форма волны. Бит WAVE управляет формой выходного сигнала"]
    #[inline(always)]
    pub fn wave(&mut self) -> WaveW<CfgrSpec> {
        WaveW::new(self, 20)
    }
    #[doc = "Bit 21 - Полярность формы волны. Бит WAVEPOL управляет полярностью выходного сигнала"]
    #[inline(always)]
    pub fn wavwpol(&mut self) -> WavwpolW<CfgrSpec> {
        WavwpolW::new(self, 21)
    }
    #[doc = "Bit 22 - Режим обновления регистров. Бит PRELOAD управляет модальностью обновления регистров ARR и CMP"]
    #[inline(always)]
    pub fn preload(&mut self) -> PreloadW<CfgrSpec> {
        PreloadW::new(self, 22)
    }
    #[doc = "Bit 23 - Бит COUNTMODE выбирает, какой источник тактового сигнала используется TIMER16 для синхронизации счетчика"]
    #[inline(always)]
    pub fn count_mode(&mut self) -> CountModeW<CfgrSpec> {
        CountModeW::new(self, 23)
    }
    #[doc = "Bit 24 - Разрешение режима энкодера. Бит ENC управляет режимом работы энкодера"]
    #[inline(always)]
    pub fn enc(&mut self) -> EncW<CfgrSpec> {
        EncW::new(self, 24)
    }
}
#[doc = "Регистр конфигурации\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgrSpec;
impl crate::RegisterSpec for CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CfgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CfgrSpec {
    const RESET_VALUE: u32 = 0;
}
