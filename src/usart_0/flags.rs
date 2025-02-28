#[doc = "Register `FLAGS` reader"]
pub type R = crate::R<FlagsSpec>;
#[doc = "Register `FLAGS` writer"]
pub type W = crate::W<FlagsSpec>;
#[doc = "Флаг обнаружения оши-бочного бита четности. Флаг сбрасывается записью 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pe {
    #[doc = "0: Ошибок в битах четности не обнаружено с момента сброса флага"]
    _0 = 0,
    #[doc = "1: Ошибка в бите четности обнаружена с момента сброса флага"]
    _1 = 1,
}
impl From<Pe> for bool {
    #[inline(always)]
    fn from(variant: Pe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PE` reader - Флаг обнаружения оши-бочного бита четности. Флаг сбрасывается записью 1"]
pub type PeR = crate::BitReader<Pe>;
impl PeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pe {
        match self.bits {
            false => Pe::_0,
            true => Pe::_1,
        }
    }
    #[doc = "Ошибок в битах четности не обнаружено с момента сброса флага"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Pe::_0
    }
    #[doc = "Ошибка в бите четности обнаружена с момента сброса флага"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Pe::_1
    }
}
#[doc = "Field `PE` writer - Флаг обнаружения оши-бочного бита четности. Флаг сбрасывается записью 1"]
pub type PeW<'a, REG> = crate::BitWriter1C<'a, REG, Pe>;
impl<'a, REG> PeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ошибок в битах четности не обнаружено с момента сброса флага"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::_0)
    }
    #[doc = "Ошибка в бите четности обнаружена с момента сброса флага"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::_1)
    }
}
#[doc = "Флаг взводится при обнаружении ошибок в стоп би-те или битах. Флаг сбрасы-вается записью 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fe {
    #[doc = "0: Ошибок приема не обнаружено с момента сброса флага"]
    _0 = 0,
    #[doc = "1: Ошибка приема обнаружена с момента сброса флага"]
    _1 = 1,
}
impl From<Fe> for bool {
    #[inline(always)]
    fn from(variant: Fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE` reader - Флаг взводится при обнаружении ошибок в стоп би-те или битах. Флаг сбрасы-вается записью 1"]
pub type FeR = crate::BitReader<Fe>;
impl FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fe {
        match self.bits {
            false => Fe::_0,
            true => Fe::_1,
        }
    }
    #[doc = "Ошибок приема не обнаружено с момента сброса флага"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Fe::_0
    }
    #[doc = "Ошибка приема обнаружена с момента сброса флага"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Fe::_1
    }
}
#[doc = "Field `FE` writer - Флаг взводится при обнаружении ошибок в стоп би-те или битах. Флаг сбрасы-вается записью 1"]
pub type FeW<'a, REG> = crate::BitWriter1C<'a, REG, Fe>;
impl<'a, REG> FeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ошибок приема не обнаружено с момента сброса флага"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Fe::_0)
    }
    #[doc = "Ошибка приема обнаружена с момента сброса флага"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Fe::_1)
    }
}
#[doc = "Флаг обнаружения ложных переключений на линии RX. Флаг сбрасывается за-писью 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nf {
    #[doc = "0: Помех не обнаружено с момента сброса флага"]
    _0 = 0,
    #[doc = "1: Помехи обнаружены с момента сброса флага"]
    _1 = 1,
}
impl From<Nf> for bool {
    #[inline(always)]
    fn from(variant: Nf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NF` reader - Флаг обнаружения ложных переключений на линии RX. Флаг сбрасывается за-писью 1"]
pub type NfR = crate::BitReader<Nf>;
impl NfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nf {
        match self.bits {
            false => Nf::_0,
            true => Nf::_1,
        }
    }
    #[doc = "Помех не обнаружено с момента сброса флага"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Nf::_0
    }
    #[doc = "Помехи обнаружены с момента сброса флага"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Nf::_1
    }
}
#[doc = "Field `NF` writer - Флаг обнаружения ложных переключений на линии RX. Флаг сбрасывается за-писью 1"]
pub type NfW<'a, REG> = crate::BitWriter1C<'a, REG, Nf>;
impl<'a, REG> NfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Помех не обнаружено с момента сброса флага"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Nf::_0)
    }
    #[doc = "Помехи обнаружены с момента сброса флага"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Nf::_1)
    }
}
#[doc = "Флаг взводится при попытке перезаписи RDR. Флаг сбрасывается записью 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ore {
    #[doc = "0: Попыток перезаписи не обнаружено с момента сброса флага"]
    _0 = 0,
    #[doc = "1: Попытка перезаписи обнаружена с момента сброса флага"]
    _1 = 1,
}
impl From<Ore> for bool {
    #[inline(always)]
    fn from(variant: Ore) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ORE` reader - Флаг взводится при попытке перезаписи RDR. Флаг сбрасывается записью 1"]
pub type OreR = crate::BitReader<Ore>;
impl OreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ore {
        match self.bits {
            false => Ore::_0,
            true => Ore::_1,
        }
    }
    #[doc = "Попыток перезаписи не обнаружено с момента сброса флага"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ore::_0
    }
    #[doc = "Попытка перезаписи обнаружена с момента сброса флага"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ore::_1
    }
}
#[doc = "Field `ORE` writer - Флаг взводится при попытке перезаписи RDR. Флаг сбрасывается записью 1"]
pub type OreW<'a, REG> = crate::BitWriter1C<'a, REG, Ore>;
impl<'a, REG> OreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Попыток перезаписи не обнаружено с момента сброса флага"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ore::_0)
    }
    #[doc = "Попытка перезаписи обнаружена с момента сброса флага"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ore::_1)
    }
}
#[doc = "Флаг взводится при отсутствии активности на линии RX в течении 8 битовых тактов при взведенном флаге RXNE. Флаг сбрасывается записью 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idle {
    #[doc = "0: Состояние idle не обнаружено после сброса флага"]
    _0 = 0,
    #[doc = "1: Состояние idle обнаружено после сброса флага"]
    _1 = 1,
}
impl From<Idle> for bool {
    #[inline(always)]
    fn from(variant: Idle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE` reader - Флаг взводится при отсутствии активности на линии RX в течении 8 битовых тактов при взведенном флаге RXNE. Флаг сбрасывается записью 1"]
pub type IdleR = crate::BitReader<Idle>;
impl IdleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idle {
        match self.bits {
            false => Idle::_0,
            true => Idle::_1,
        }
    }
    #[doc = "Состояние idle не обнаружено после сброса флага"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Idle::_0
    }
    #[doc = "Состояние idle обнаружено после сброса флага"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Idle::_1
    }
}
#[doc = "Field `IDLE` writer - Флаг взводится при отсутствии активности на линии RX в течении 8 битовых тактов при взведенном флаге RXNE. Флаг сбрасывается записью 1"]
pub type IdleW<'a, REG> = crate::BitWriter1C<'a, REG, Idle>;
impl<'a, REG> IdleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Состояние idle не обнаружено после сброса флага"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Idle::_0)
    }
    #[doc = "Состояние idle обнаружено после сброса флага"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Idle::_1)
    }
}
#[doc = "Флаг взводится при записи принятых данных в регистр RDR из сдвигового реги-стра. Флаг сбрасывается записью 1 или чтением ре-гистра RDR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxne {
    #[doc = "0: Данные не принимались после сброса флага"]
    _0 = 0,
    #[doc = "1: Данные принимались после сброса флага"]
    _1 = 1,
}
impl From<Rxne> for bool {
    #[inline(always)]
    fn from(variant: Rxne) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXNE` reader - Флаг взводится при записи принятых данных в регистр RDR из сдвигового реги-стра. Флаг сбрасывается записью 1 или чтением ре-гистра RDR"]
pub type RxneR = crate::BitReader<Rxne>;
impl RxneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxne {
        match self.bits {
            false => Rxne::_0,
            true => Rxne::_1,
        }
    }
    #[doc = "Данные не принимались после сброса флага"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rxne::_0
    }
    #[doc = "Данные принимались после сброса флага"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rxne::_1
    }
}
#[doc = "Field `RXNE` writer - Флаг взводится при записи принятых данных в регистр RDR из сдвигового реги-стра. Флаг сбрасывается записью 1 или чтением ре-гистра RDR"]
pub type RxneW<'a, REG> = crate::BitWriter1C<'a, REG, Rxne>;
impl<'a, REG> RxneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Данные не принимались после сброса флага"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxne::_0)
    }
    #[doc = "Данные принимались после сброса флага"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxne::_1)
    }
}
#[doc = "Флаг взводится при переда-че данных в момент от-правки последнего стоп би-та. Флаг сбрасывается за-писью 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tc {
    #[doc = "0: Данные не отправлялись с момента сброса флага"]
    _0 = 0,
    #[doc = "1: Данные отправлялись с момента сброса флага"]
    _1 = 1,
}
impl From<Tc> for bool {
    #[inline(always)]
    fn from(variant: Tc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC` reader - Флаг взводится при переда-че данных в момент от-правки последнего стоп би-та. Флаг сбрасывается за-писью 1"]
pub type TcR = crate::BitReader<Tc>;
impl TcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tc {
        match self.bits {
            false => Tc::_0,
            true => Tc::_1,
        }
    }
    #[doc = "Данные не отправлялись с момента сброса флага"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Tc::_0
    }
    #[doc = "Данные отправлялись с момента сброса флага"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Tc::_1
    }
}
#[doc = "Field `TC` writer - Флаг взводится при переда-че данных в момент от-правки последнего стоп би-та. Флаг сбрасывается за-писью 1"]
pub type TcW<'a, REG> = crate::BitWriter1C<'a, REG, Tc>;
impl<'a, REG> TcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Данные не отправлялись с момента сброса флага"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Tc::_0)
    }
    #[doc = "Данные отправлялись с момента сброса флага"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Tc::_1)
    }
}
#[doc = "Флаг взводится при передаче данных в момент записи данных в сдвиговый регистр. Флаг сбрасывается записью данных в регистр TDR или записью 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txe {
    #[doc = "0: Данные не записывались в сдвиговый регистр с момента сброса флага"]
    _0 = 0,
    #[doc = "1: Данные записывались в сдвиговый регистр"]
    _1 = 1,
}
impl From<Txe> for bool {
    #[inline(always)]
    fn from(variant: Txe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXE` reader - Флаг взводится при передаче данных в момент записи данных в сдвиговый регистр. Флаг сбрасывается записью данных в регистр TDR или записью 1"]
pub type TxeR = crate::BitReader<Txe>;
impl TxeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txe {
        match self.bits {
            false => Txe::_0,
            true => Txe::_1,
        }
    }
    #[doc = "Данные не записывались в сдвиговый регистр с момента сброса флага"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Txe::_0
    }
    #[doc = "Данные записывались в сдвиговый регистр"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Txe::_1
    }
}
#[doc = "Field `TXE` writer - Флаг взводится при передаче данных в момент записи данных в сдвиговый регистр. Флаг сбрасывается записью данных в регистр TDR или записью 1"]
pub type TxeW<'a, REG> = crate::BitWriter1C<'a, REG, Txe>;
impl<'a, REG> TxeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Данные не записывались в сдвиговый регистр с момента сброса флага"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Txe::_0)
    }
    #[doc = "Данные записывались в сдвиговый регистр"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Txe::_1)
    }
}
#[doc = "Флаг обнаружения break состояния на линии RX. Флаг сбрасывается записью 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbdf {
    #[doc = "0: Состояние break не обнаружено с момента сброса флага"]
    NotDetected = 0,
    #[doc = "1: Состояние break обнаружено с момента сброса флага"]
    Detected = 1,
}
impl From<Lbdf> for bool {
    #[inline(always)]
    fn from(variant: Lbdf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBDF` reader - Флаг обнаружения break состояния на линии RX. Флаг сбрасывается записью 1"]
pub type LbdfR = crate::BitReader<Lbdf>;
impl LbdfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbdf {
        match self.bits {
            false => Lbdf::NotDetected,
            true => Lbdf::Detected,
        }
    }
    #[doc = "Состояние break не обнаружено с момента сброса флага"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Lbdf::NotDetected
    }
    #[doc = "Состояние break обнаружено с момента сброса флага"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Lbdf::Detected
    }
}
#[doc = "Field `LBDF` writer - Флаг обнаружения break состояния на линии RX. Флаг сбрасывается записью 1"]
pub type LbdfW<'a, REG> = crate::BitWriter1C<'a, REG, Lbdf>;
impl<'a, REG> LbdfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Состояние break не обнаружено с момента сброса флага"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Lbdf::NotDetected)
    }
    #[doc = "Состояние break обнаружено с момента сброса флага"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Lbdf::Detected)
    }
}
#[doc = "Флаг взводится при изме-нении значения сигнала CTS. Флаг сбрасывается записью 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsif {
    #[doc = "0: Состояние CTS не менялось с момента сброса флага"]
    _0 = 0,
    #[doc = "1: Состояние CTS менялось с момента сброса флага"]
    _1 = 1,
}
impl From<Ctsif> for bool {
    #[inline(always)]
    fn from(variant: Ctsif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSIF` reader - Флаг взводится при изме-нении значения сигнала CTS. Флаг сбрасывается записью 1"]
pub type CtsifR = crate::BitReader<Ctsif>;
impl CtsifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsif {
        match self.bits {
            false => Ctsif::_0,
            true => Ctsif::_1,
        }
    }
    #[doc = "Состояние CTS не менялось с момента сброса флага"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ctsif::_0
    }
    #[doc = "Состояние CTS менялось с момента сброса флага"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ctsif::_1
    }
}
#[doc = "Field `CTSIF` writer - Флаг взводится при изме-нении значения сигнала CTS. Флаг сбрасывается записью 1"]
pub type CtsifW<'a, REG> = crate::BitWriter1C<'a, REG, Ctsif>;
impl<'a, REG> CtsifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Состояние CTS не менялось с момента сброса флага"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsif::_0)
    }
    #[doc = "Состояние CTS менялось с момента сброса флага"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsif::_1)
    }
}
#[doc = "Флаг отображает текущее значение сигнала CTS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cts {
    #[doc = "0: CTS = 0"]
    _0 = 0,
    #[doc = "1: CTS = 1"]
    _1 = 1,
}
impl From<Cts> for bool {
    #[inline(always)]
    fn from(variant: Cts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTS` reader - Флаг отображает текущее значение сигнала CTS"]
pub type CtsR = crate::BitReader<Cts>;
impl CtsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cts {
        match self.bits {
            false => Cts::_0,
            true => Cts::_1,
        }
    }
    #[doc = "CTS = 0"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cts::_0
    }
    #[doc = "CTS = 1"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cts::_1
    }
}
#[doc = "Флаг активности на линии RX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: Нет активности на линии RX"]
    Inactive = 0,
    #[doc = "1: Идет передача данных или break состояние"]
    Active = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Флаг активности на линии RX"]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            false => Busy::Inactive,
            true => Busy::Active,
        }
    }
    #[doc = "Нет активности на линии RX"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Busy::Inactive
    }
    #[doc = "Идет передача данных или break состояние"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Busy::Active
    }
}
#[doc = "Флаг готовности передатчика к работе после снятия сброса (UE и TE)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Teack {
    #[doc = "0: Ресет UE и TE активен"]
    Unready = 0,
    #[doc = "1: Ресет UE и TE снят"]
    Ready = 1,
}
impl From<Teack> for bool {
    #[inline(always)]
    fn from(variant: Teack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEACK` reader - Флаг готовности передатчика к работе после снятия сброса (UE и TE)"]
pub type TeackR = crate::BitReader<Teack>;
impl TeackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Teack {
        match self.bits {
            false => Teack::Unready,
            true => Teack::Ready,
        }
    }
    #[doc = "Ресет UE и TE активен"]
    #[inline(always)]
    pub fn is_unready(&self) -> bool {
        *self == Teack::Unready
    }
    #[doc = "Ресет UE и TE снят"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Teack::Ready
    }
}
#[doc = "Флаг готовности приемника к работе после снятия сброса (UE и RE)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reack {
    #[doc = "0: Ресет UE и RE активен"]
    Unready = 0,
    #[doc = "1: Ресет UE и RE снят"]
    Ready = 1,
}
impl From<Reack> for bool {
    #[inline(always)]
    fn from(variant: Reack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REACK` reader - Флаг готовности приемника к работе после снятия сброса (UE и RE)"]
pub type ReackR = crate::BitReader<Reack>;
impl ReackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reack {
        match self.bits {
            false => Reack::Unready,
            true => Reack::Ready,
        }
    }
    #[doc = "Ресет UE и RE активен"]
    #[inline(always)]
    pub fn is_unready(&self) -> bool {
        *self == Reack::Unready
    }
    #[doc = "Ресет UE и RE снят"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Reack::Ready
    }
}
impl R {
    #[doc = "Bit 0 - Флаг обнаружения оши-бочного бита четности. Флаг сбрасывается записью 1"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Флаг взводится при обнаружении ошибок в стоп би-те или битах. Флаг сбрасы-вается записью 1"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Флаг обнаружения ложных переключений на линии RX. Флаг сбрасывается за-писью 1"]
    #[inline(always)]
    pub fn nf(&self) -> NfR {
        NfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Флаг взводится при попытке перезаписи RDR. Флаг сбрасывается записью 1"]
    #[inline(always)]
    pub fn ore(&self) -> OreR {
        OreR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Флаг взводится при отсутствии активности на линии RX в течении 8 битовых тактов при взведенном флаге RXNE. Флаг сбрасывается записью 1"]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Флаг взводится при записи принятых данных в регистр RDR из сдвигового реги-стра. Флаг сбрасывается записью 1 или чтением ре-гистра RDR"]
    #[inline(always)]
    pub fn rxne(&self) -> RxneR {
        RxneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Флаг взводится при переда-че данных в момент от-правки последнего стоп би-та. Флаг сбрасывается за-писью 1"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Флаг взводится при передаче данных в момент записи данных в сдвиговый регистр. Флаг сбрасывается записью данных в регистр TDR или записью 1"]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Флаг обнаружения break состояния на линии RX. Флаг сбрасывается записью 1"]
    #[inline(always)]
    pub fn lbdf(&self) -> LbdfR {
        LbdfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Флаг взводится при изме-нении значения сигнала CTS. Флаг сбрасывается записью 1"]
    #[inline(always)]
    pub fn ctsif(&self) -> CtsifR {
        CtsifR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Флаг отображает текущее значение сигнала CTS"]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Флаг активности на линии RX"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 21 - Флаг готовности передатчика к работе после снятия сброса (UE и TE)"]
    #[inline(always)]
    pub fn teack(&self) -> TeackR {
        TeackR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Флаг готовности приемника к работе после снятия сброса (UE и RE)"]
    #[inline(always)]
    pub fn reack(&self) -> ReackR {
        ReackR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Флаг обнаружения оши-бочного бита четности. Флаг сбрасывается записью 1"]
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<FlagsSpec> {
        PeW::new(self, 0)
    }
    #[doc = "Bit 1 - Флаг взводится при обнаружении ошибок в стоп би-те или битах. Флаг сбрасы-вается записью 1"]
    #[inline(always)]
    pub fn fe(&mut self) -> FeW<FlagsSpec> {
        FeW::new(self, 1)
    }
    #[doc = "Bit 2 - Флаг обнаружения ложных переключений на линии RX. Флаг сбрасывается за-писью 1"]
    #[inline(always)]
    pub fn nf(&mut self) -> NfW<FlagsSpec> {
        NfW::new(self, 2)
    }
    #[doc = "Bit 3 - Флаг взводится при попытке перезаписи RDR. Флаг сбрасывается записью 1"]
    #[inline(always)]
    pub fn ore(&mut self) -> OreW<FlagsSpec> {
        OreW::new(self, 3)
    }
    #[doc = "Bit 4 - Флаг взводится при отсутствии активности на линии RX в течении 8 битовых тактов при взведенном флаге RXNE. Флаг сбрасывается записью 1"]
    #[inline(always)]
    pub fn idle(&mut self) -> IdleW<FlagsSpec> {
        IdleW::new(self, 4)
    }
    #[doc = "Bit 5 - Флаг взводится при записи принятых данных в регистр RDR из сдвигового реги-стра. Флаг сбрасывается записью 1 или чтением ре-гистра RDR"]
    #[inline(always)]
    pub fn rxne(&mut self) -> RxneW<FlagsSpec> {
        RxneW::new(self, 5)
    }
    #[doc = "Bit 6 - Флаг взводится при переда-че данных в момент от-правки последнего стоп би-та. Флаг сбрасывается за-писью 1"]
    #[inline(always)]
    pub fn tc(&mut self) -> TcW<FlagsSpec> {
        TcW::new(self, 6)
    }
    #[doc = "Bit 7 - Флаг взводится при передаче данных в момент записи данных в сдвиговый регистр. Флаг сбрасывается записью данных в регистр TDR или записью 1"]
    #[inline(always)]
    pub fn txe(&mut self) -> TxeW<FlagsSpec> {
        TxeW::new(self, 7)
    }
    #[doc = "Bit 8 - Флаг обнаружения break состояния на линии RX. Флаг сбрасывается записью 1"]
    #[inline(always)]
    pub fn lbdf(&mut self) -> LbdfW<FlagsSpec> {
        LbdfW::new(self, 8)
    }
    #[doc = "Bit 9 - Флаг взводится при изме-нении значения сигнала CTS. Флаг сбрасывается записью 1"]
    #[inline(always)]
    pub fn ctsif(&mut self) -> CtsifW<FlagsSpec> {
        CtsifW::new(self, 9)
    }
}
#[doc = "Регистр прерываний\n\nYou can [`read`](crate::Reg::read) this register and get [`flags::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flags::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlagsSpec;
impl crate::RegisterSpec for FlagsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flags::R`](R) reader structure"]
impl crate::Readable for FlagsSpec {}
#[doc = "`write(|w| ..)` method takes [`flags::W`](W) writer structure"]
impl crate::Writable for FlagsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x03ff;
}
#[doc = "`reset()` method sets FLAGS to value 0"]
impl crate::Resettable for FlagsSpec {
    const RESET_VALUE: u32 = 0;
}
