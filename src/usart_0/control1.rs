#[doc = "Register `CONTROL1` reader"]
pub type R = crate::R<Control1Spec>;
#[doc = "Register `CONTROL1` writer"]
pub type W = crate::W<Control1Spec>;
#[doc = "Ресет USART. Отменяет все текущие операции. Не очищает регистры конфигурации, но сбрасывает флаги\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ue {
    #[doc = "0: USART выключен"]
    Disable = 0,
    #[doc = "1: USART включен"]
    Enable = 1,
}
impl From<Ue> for bool {
    #[inline(always)]
    fn from(variant: Ue) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UE` reader - Ресет USART. Отменяет все текущие операции. Не очищает регистры конфигурации, но сбрасывает флаги"]
pub type UeR = crate::BitReader<Ue>;
impl UeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ue {
        match self.bits {
            false => Ue::Disable,
            true => Ue::Enable,
        }
    }
    #[doc = "USART выключен"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ue::Disable
    }
    #[doc = "USART включен"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ue::Enable
    }
}
#[doc = "Field `UE` writer - Ресет USART. Отменяет все текущие операции. Не очищает регистры конфигурации, но сбрасывает флаги"]
pub type UeW<'a, REG> = crate::BitWriter<'a, REG, Ue>;
impl<'a, REG> UeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USART выключен"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ue::Disable)
    }
    #[doc = "USART включен"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ue::Enable)
    }
}
#[doc = "Управление приемником\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Re {
    #[doc = "0: Приемник выключен"]
    Disable = 0,
    #[doc = "1: Приемник включен"]
    Enable = 1,
}
impl From<Re> for bool {
    #[inline(always)]
    fn from(variant: Re) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RE` reader - Управление приемником"]
pub type ReR = crate::BitReader<Re>;
impl ReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Re {
        match self.bits {
            false => Re::Disable,
            true => Re::Enable,
        }
    }
    #[doc = "Приемник выключен"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Re::Disable
    }
    #[doc = "Приемник включен"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Re::Enable
    }
}
#[doc = "Field `RE` writer - Управление приемником"]
pub type ReW<'a, REG> = crate::BitWriter<'a, REG, Re>;
impl<'a, REG> ReW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Приемник выключен"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Re::Disable)
    }
    #[doc = "Приемник включен"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Re::Enable)
    }
}
#[doc = "Управление передатчиком\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te {
    #[doc = "0: Передатчик выключен"]
    Disable = 0,
    #[doc = "1: Передатчик включен"]
    Enable = 1,
}
impl From<Te> for bool {
    #[inline(always)]
    fn from(variant: Te) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE` reader - Управление передатчиком"]
pub type TeR = crate::BitReader<Te>;
impl TeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te {
        match self.bits {
            false => Te::Disable,
            true => Te::Enable,
        }
    }
    #[doc = "Передатчик выключен"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Te::Disable
    }
    #[doc = "Передатчик включен"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Te::Enable
    }
}
#[doc = "Field `TE` writer - Управление передатчиком"]
pub type TeW<'a, REG> = crate::BitWriter<'a, REG, Te>;
impl<'a, REG> TeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Передатчик выключен"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Te::Disable)
    }
    #[doc = "Передатчик включен"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Te::Enable)
    }
}
#[doc = "Управление прерыванием при отсутствии входных транзакций\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idleie {
    #[doc = "0: Прерывание выключено"]
    Disable = 0,
    #[doc = "1: Прерывание включено"]
    Enable = 1,
}
impl From<Idleie> for bool {
    #[inline(always)]
    fn from(variant: Idleie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLEIE` reader - Управление прерыванием при отсутствии входных транзакций"]
pub type IdleieR = crate::BitReader<Idleie>;
impl IdleieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idleie {
        match self.bits {
            false => Idleie::Disable,
            true => Idleie::Enable,
        }
    }
    #[doc = "Прерывание выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Idleie::Disable
    }
    #[doc = "Прерывание включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Idleie::Enable
    }
}
#[doc = "Field `IDLEIE` writer - Управление прерыванием при отсутствии входных транзакций"]
pub type IdleieW<'a, REG> = crate::BitWriter<'a, REG, Idleie>;
impl<'a, REG> IdleieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Idleie::Disable)
    }
    #[doc = "Прерывание включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Idleie::Enable)
    }
}
#[doc = "Управление прерыванием при успешном приеме данных или перезаписи полученных данных\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxneie {
    #[doc = "0: Прерывание выключено"]
    Disable = 0,
    #[doc = "1: Прерывание включено"]
    Enable = 1,
}
impl From<Rxneie> for bool {
    #[inline(always)]
    fn from(variant: Rxneie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXNEIE` reader - Управление прерыванием при успешном приеме данных или перезаписи полученных данных"]
pub type RxneieR = crate::BitReader<Rxneie>;
impl RxneieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxneie {
        match self.bits {
            false => Rxneie::Disable,
            true => Rxneie::Enable,
        }
    }
    #[doc = "Прерывание выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rxneie::Disable
    }
    #[doc = "Прерывание включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rxneie::Enable
    }
}
#[doc = "Field `RXNEIE` writer - Управление прерыванием при успешном приеме данных или перезаписи полученных данных"]
pub type RxneieW<'a, REG> = crate::BitWriter<'a, REG, Rxneie>;
impl<'a, REG> RxneieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rxneie::Disable)
    }
    #[doc = "Прерывание включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rxneie::Enable)
    }
}
#[doc = "Управление прерыванием при успешной передаче данных\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcie {
    #[doc = "0: Прерывание выключено"]
    Disable = 0,
    #[doc = "1: Прерывание включено"]
    Enable = 1,
}
impl From<Tcie> for bool {
    #[inline(always)]
    fn from(variant: Tcie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIE` reader - Управление прерыванием при успешной передаче данных"]
pub type TcieR = crate::BitReader<Tcie>;
impl TcieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcie {
        match self.bits {
            false => Tcie::Disable,
            true => Tcie::Enable,
        }
    }
    #[doc = "Прерывание выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tcie::Disable
    }
    #[doc = "Прерывание включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tcie::Enable
    }
}
#[doc = "Field `TCIE` writer - Управление прерыванием при успешной передаче данных"]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG, Tcie>;
impl<'a, REG> TcieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tcie::Disable)
    }
    #[doc = "Прерывание включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tcie::Enable)
    }
}
#[doc = "Управление прерыванием при передаче данных в момент записи данных в сдвиговый регистр\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txeie {
    #[doc = "0: Прерывание выключено"]
    Disable = 0,
    #[doc = "1: Прерывание включено"]
    Enable = 1,
}
impl From<Txeie> for bool {
    #[inline(always)]
    fn from(variant: Txeie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEIE` reader - Управление прерыванием при передаче данных в момент записи данных в сдвиговый регистр"]
pub type TxeieR = crate::BitReader<Txeie>;
impl TxeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txeie {
        match self.bits {
            false => Txeie::Disable,
            true => Txeie::Enable,
        }
    }
    #[doc = "Прерывание выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Txeie::Disable
    }
    #[doc = "Прерывание включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Txeie::Enable
    }
}
#[doc = "Field `TXEIE` writer - Управление прерыванием при передаче данных в момент записи данных в сдвиговый регистр"]
pub type TxeieW<'a, REG> = crate::BitWriter<'a, REG, Txeie>;
impl<'a, REG> TxeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Txeie::Disable)
    }
    #[doc = "Прерывание включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Txeie::Enable)
    }
}
#[doc = "Управление прерыванием при ошибке в принятом бите четности\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peie {
    #[doc = "0: Прерывание по ошибке в бите четности выключено"]
    Disable = 0,
    #[doc = "1: Прерывание по ошибке в бите четности включено"]
    Enable = 1,
}
impl From<Peie> for bool {
    #[inline(always)]
    fn from(variant: Peie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEIE` reader - Управление прерыванием при ошибке в принятом бите четности"]
pub type PeieR = crate::BitReader<Peie>;
impl PeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peie {
        match self.bits {
            false => Peie::Disable,
            true => Peie::Enable,
        }
    }
    #[doc = "Прерывание по ошибке в бите четности выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Peie::Disable
    }
    #[doc = "Прерывание по ошибке в бите четности включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Peie::Enable
    }
}
#[doc = "Field `PEIE` writer - Управление прерыванием при ошибке в принятом бите четности"]
pub type PeieW<'a, REG> = crate::BitWriter<'a, REG, Peie>;
impl<'a, REG> PeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание по ошибке в бите четности выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Peie::Disable)
    }
    #[doc = "Прерывание по ошибке в бите четности включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Peie::Enable)
    }
}
#[doc = "Выбор способа формирования бита четности. Примечание: Этот бит может быть изменен только при остановке работы (UE=0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ps {
    #[doc = "0: Бит четности"]
    Parity = 0,
    #[doc = "1: Бит нечетности"]
    Odd = 1,
}
impl From<Ps> for bool {
    #[inline(always)]
    fn from(variant: Ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PS` reader - Выбор способа формирования бита четности. Примечание: Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type PsR = crate::BitReader<Ps>;
impl PsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ps {
        match self.bits {
            false => Ps::Parity,
            true => Ps::Odd,
        }
    }
    #[doc = "Бит четности"]
    #[inline(always)]
    pub fn is_parity(&self) -> bool {
        *self == Ps::Parity
    }
    #[doc = "Бит нечетности"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == Ps::Odd
    }
}
#[doc = "Field `PS` writer - Выбор способа формирования бита четности. Примечание: Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type PsW<'a, REG> = crate::BitWriter<'a, REG, Ps>;
impl<'a, REG> PsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Бит четности"]
    #[inline(always)]
    pub fn parity(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::Parity)
    }
    #[doc = "Бит нечетности"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::Odd)
    }
}
#[doc = "Контроль четности. Примечание: Этот бит может быть изме-нен только при остановке работы (UE=0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pce {
    #[doc = "0: Контроль четности выключен"]
    Disable = 0,
    #[doc = "1: Контроль четности включен"]
    Enable = 1,
}
impl From<Pce> for bool {
    #[inline(always)]
    fn from(variant: Pce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCE` reader - Контроль четности. Примечание: Этот бит может быть изме-нен только при остановке работы (UE=0)"]
pub type PceR = crate::BitReader<Pce>;
impl PceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pce {
        match self.bits {
            false => Pce::Disable,
            true => Pce::Enable,
        }
    }
    #[doc = "Контроль четности выключен"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pce::Disable
    }
    #[doc = "Контроль четности включен"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pce::Enable
    }
}
#[doc = "Field `PCE` writer - Контроль четности. Примечание: Этот бит может быть изме-нен только при остановке работы (UE=0)"]
pub type PceW<'a, REG> = crate::BitWriter<'a, REG, Pce>;
impl<'a, REG> PceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Контроль четности выключен"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pce::Disable)
    }
    #[doc = "Контроль четности включен"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pce::Enable)
    }
}
#[doc = "M1\\[28\\]
и M0\\[12\\]
управляют длиной кадра. Примечание: Эти биты можгут быть изменены только при остановке работы (UE=0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum M {
    #[doc = "0: 8 бит данных"]
    _8bits = 0,
    #[doc = "1: 9 бит данных"]
    _9bits = 1,
    #[doc = "65536: 7 бит данных"]
    _7bits = 65536,
}
impl From<M> for u32 {
    #[inline(always)]
    fn from(variant: M) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for M {
    type Ux = u32;
}
impl crate::IsEnum for M {}
#[doc = "Field `M` reader - M1\\[28\\]
и M0\\[12\\]
управляют длиной кадра. Примечание: Эти биты можгут быть изменены только при остановке работы (UE=0)"]
pub type MR = crate::FieldReader<M>;
impl MR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<M> {
        match self.bits {
            0 => Some(M::_8bits),
            1 => Some(M::_9bits),
            65536 => Some(M::_7bits),
            _ => None,
        }
    }
    #[doc = "8 бит данных"]
    #[inline(always)]
    pub fn is_8bits(&self) -> bool {
        *self == M::_8bits
    }
    #[doc = "9 бит данных"]
    #[inline(always)]
    pub fn is_9bits(&self) -> bool {
        *self == M::_9bits
    }
    #[doc = "7 бит данных"]
    #[inline(always)]
    pub fn is_7bits(&self) -> bool {
        *self == M::_7bits
    }
}
#[doc = "Field `M` writer - M1\\[28\\]
и M0\\[12\\]
управляют длиной кадра. Примечание: Эти биты можгут быть изменены только при остановке работы (UE=0)"]
pub type MW<'a, REG> = crate::FieldWriter<'a, REG, 17, M>;
impl<'a, REG> MW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "8 бит данных"]
    #[inline(always)]
    pub fn _8bits(self) -> &'a mut crate::W<REG> {
        self.variant(M::_8bits)
    }
    #[doc = "9 бит данных"]
    #[inline(always)]
    pub fn _9bits(self) -> &'a mut crate::W<REG> {
        self.variant(M::_9bits)
    }
    #[doc = "7 бит данных"]
    #[inline(always)]
    pub fn _7bits(self) -> &'a mut crate::W<REG> {
        self.variant(M::_7bits)
    }
}
impl R {
    #[doc = "Bit 0 - Ресет USART. Отменяет все текущие операции. Не очищает регистры конфигурации, но сбрасывает флаги"]
    #[inline(always)]
    pub fn ue(&self) -> UeR {
        UeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Управление приемником"]
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Управление передатчиком"]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Управление прерыванием при отсутствии входных транзакций"]
    #[inline(always)]
    pub fn idleie(&self) -> IdleieR {
        IdleieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Управление прерыванием при успешном приеме данных или перезаписи полученных данных"]
    #[inline(always)]
    pub fn rxneie(&self) -> RxneieR {
        RxneieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Управление прерыванием при успешной передаче данных"]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Управление прерыванием при передаче данных в момент записи данных в сдвиговый регистр"]
    #[inline(always)]
    pub fn txeie(&self) -> TxeieR {
        TxeieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Управление прерыванием при ошибке в принятом бите четности"]
    #[inline(always)]
    pub fn peie(&self) -> PeieR {
        PeieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Выбор способа формирования бита четности. Примечание: Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Контроль четности. Примечание: Этот бит может быть изме-нен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn pce(&self) -> PceR {
        PceR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:28 - M1\\[28\\]
и M0\\[12\\]
управляют длиной кадра. Примечание: Эти биты можгут быть изменены только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn m(&self) -> MR {
        MR::new((self.bits >> 12) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Ресет USART. Отменяет все текущие операции. Не очищает регистры конфигурации, но сбрасывает флаги"]
    #[inline(always)]
    pub fn ue(&mut self) -> UeW<Control1Spec> {
        UeW::new(self, 0)
    }
    #[doc = "Bit 2 - Управление приемником"]
    #[inline(always)]
    pub fn re(&mut self) -> ReW<Control1Spec> {
        ReW::new(self, 2)
    }
    #[doc = "Bit 3 - Управление передатчиком"]
    #[inline(always)]
    pub fn te(&mut self) -> TeW<Control1Spec> {
        TeW::new(self, 3)
    }
    #[doc = "Bit 4 - Управление прерыванием при отсутствии входных транзакций"]
    #[inline(always)]
    pub fn idleie(&mut self) -> IdleieW<Control1Spec> {
        IdleieW::new(self, 4)
    }
    #[doc = "Bit 5 - Управление прерыванием при успешном приеме данных или перезаписи полученных данных"]
    #[inline(always)]
    pub fn rxneie(&mut self) -> RxneieW<Control1Spec> {
        RxneieW::new(self, 5)
    }
    #[doc = "Bit 6 - Управление прерыванием при успешной передаче данных"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TcieW<Control1Spec> {
        TcieW::new(self, 6)
    }
    #[doc = "Bit 7 - Управление прерыванием при передаче данных в момент записи данных в сдвиговый регистр"]
    #[inline(always)]
    pub fn txeie(&mut self) -> TxeieW<Control1Spec> {
        TxeieW::new(self, 7)
    }
    #[doc = "Bit 8 - Управление прерыванием при ошибке в принятом бите четности"]
    #[inline(always)]
    pub fn peie(&mut self) -> PeieW<Control1Spec> {
        PeieW::new(self, 8)
    }
    #[doc = "Bit 9 - Выбор способа формирования бита четности. Примечание: Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn ps(&mut self) -> PsW<Control1Spec> {
        PsW::new(self, 9)
    }
    #[doc = "Bit 10 - Контроль четности. Примечание: Этот бит может быть изме-нен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn pce(&mut self) -> PceW<Control1Spec> {
        PceW::new(self, 10)
    }
    #[doc = "Bits 12:28 - M1\\[28\\]
и M0\\[12\\]
управляют длиной кадра. Примечание: Эти биты можгут быть изменены только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn m(&mut self) -> MW<Control1Spec> {
        MW::new(self, 12)
    }
}
#[doc = "Регистр управления 1\n\nYou can [`read`](crate::Reg::read) this register and get [`control1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Control1Spec;
impl crate::RegisterSpec for Control1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control1::R`](R) reader structure"]
impl crate::Readable for Control1Spec {}
#[doc = "`write(|w| ..)` method takes [`control1::W`](W) writer structure"]
impl crate::Writable for Control1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONTROL1 to value 0"]
impl crate::Resettable for Control1Spec {
    const RESET_VALUE: u32 = 0;
}
