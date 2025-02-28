#[doc = "Register `CONTROL2` reader"]
pub type R = crate::R<Control2Spec>;
#[doc = "Register `CONTROL2` writer"]
pub type W = crate::W<Control2Spec>;
#[doc = "Управление прерыванием при обнаружении break состояния на RX линии\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbdie {
    #[doc = "0: Прерывание выключено"]
    Disable = 0,
    #[doc = "1: Прерывание включено"]
    Enable = 1,
}
impl From<Lbdie> for bool {
    #[inline(always)]
    fn from(variant: Lbdie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBDIE` reader - Управление прерыванием при обнаружении break состояния на RX линии"]
pub type LbdieR = crate::BitReader<Lbdie>;
impl LbdieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbdie {
        match self.bits {
            false => Lbdie::Disable,
            true => Lbdie::Enable,
        }
    }
    #[doc = "Прерывание выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Lbdie::Disable
    }
    #[doc = "Прерывание включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Lbdie::Enable
    }
}
#[doc = "Field `LBDIE` writer - Управление прерыванием при обнаружении break состояния на RX линии"]
pub type LbdieW<'a, REG> = crate::BitWriter<'a, REG, Lbdie>;
impl<'a, REG> LbdieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Lbdie::Disable)
    }
    #[doc = "Прерывание включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lbdie::Enable)
    }
}
#[doc = "Управление последним тактовым импульсом. Этот бит может быть изменен только при остановке работы (UE=0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbcl {
    #[doc = "0: Последний тактовый импульс отсутствует"]
    _0 = 0,
    #[doc = "1: Последний тактовый импульс присутствует."]
    _1 = 1,
}
impl From<Lbcl> for bool {
    #[inline(always)]
    fn from(variant: Lbcl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBCL` reader - Управление последним тактовым импульсом. Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type LbclR = crate::BitReader<Lbcl>;
impl LbclR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbcl {
        match self.bits {
            false => Lbcl::_0,
            true => Lbcl::_1,
        }
    }
    #[doc = "Последний тактовый импульс отсутствует"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Lbcl::_0
    }
    #[doc = "Последний тактовый импульс присутствует."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Lbcl::_1
    }
}
#[doc = "Field `LBCL` writer - Управление последним тактовым импульсом. Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type LbclW<'a, REG> = crate::BitWriter<'a, REG, Lbcl>;
impl<'a, REG> LbclW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Последний тактовый импульс отсутствует"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Lbcl::_0)
    }
    #[doc = "Последний тактовый импульс присутствует."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Lbcl::_1)
    }
}
#[doc = "Управление фазой выходного тактового сигнала CK. Этот бит может быть изменен только при остановке работы (UE=0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpha {
    #[doc = "0: Первое изменение тактового сигнала – фронт"]
    _0 = 0,
    #[doc = "1: Первое изменение тактового сигнала – срез (обратный фронт)."]
    _1 = 1,
}
impl From<Cpha> for bool {
    #[inline(always)]
    fn from(variant: Cpha) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHA` reader - Управление фазой выходного тактового сигнала CK. Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type CphaR = crate::BitReader<Cpha>;
impl CphaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpha {
        match self.bits {
            false => Cpha::_0,
            true => Cpha::_1,
        }
    }
    #[doc = "Первое изменение тактового сигнала – фронт"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cpha::_0
    }
    #[doc = "Первое изменение тактового сигнала – срез (обратный фронт)."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cpha::_1
    }
}
#[doc = "Field `CPHA` writer - Управление фазой выходного тактового сигнала CK. Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG, Cpha>;
impl<'a, REG> CphaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Первое изменение тактового сигнала – фронт"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpha::_0)
    }
    #[doc = "Первое изменение тактового сигнала – срез (обратный фронт)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpha::_1)
    }
}
#[doc = "Управление полярностью выходного тактового сигнала CK. Этот бит может быть изменен только при остановке работы (UE=0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpol {
    #[doc = "0: Логический «0» вне транзакции"]
    _0 = 0,
    #[doc = "1: Логическая «1» вне транзакции"]
    _1 = 1,
}
impl From<Cpol> for bool {
    #[inline(always)]
    fn from(variant: Cpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - Управление полярностью выходного тактового сигнала CK. Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type CpolR = crate::BitReader<Cpol>;
impl CpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpol {
        match self.bits {
            false => Cpol::_0,
            true => Cpol::_1,
        }
    }
    #[doc = "Логический «0» вне транзакции"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Cpol::_0
    }
    #[doc = "Логическая «1» вне транзакции"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Cpol::_1
    }
}
#[doc = "Field `CPOL` writer - Управление полярностью выходного тактового сигнала CK. Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG, Cpol>;
impl<'a, REG> CpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Логический «0» вне транзакции"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpol::_0)
    }
    #[doc = "Логическая «1» вне транзакции"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpol::_1)
    }
}
#[doc = "Управление синхронным режимом. Этот бит может быть изменен только при остановке работы (UE=0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clken {
    #[doc = "0: Синхронный режим выключен"]
    Asynchronous = 0,
    #[doc = "1: Синхронный режим включен"]
    Synchronous = 1,
}
impl From<Clken> for bool {
    #[inline(always)]
    fn from(variant: Clken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKEN` reader - Управление синхронным режимом. Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type ClkenR = crate::BitReader<Clken>;
impl ClkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clken {
        match self.bits {
            false => Clken::Asynchronous,
            true => Clken::Synchronous,
        }
    }
    #[doc = "Синхронный режим выключен"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == Clken::Asynchronous
    }
    #[doc = "Синхронный режим включен"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        *self == Clken::Synchronous
    }
}
#[doc = "Field `CLKEN` writer - Управление синхронным режимом. Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG, Clken>;
impl<'a, REG> ClkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Синхронный режим выключен"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Clken::Asynchronous)
    }
    #[doc = "Синхронный режим включен"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut crate::W<REG> {
        self.variant(Clken::Synchronous)
    }
}
#[doc = "Количество стоп битов для приемника и передатчика. Этот бит может быть изменен только при остановке работы (UE=0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop1 {
    #[doc = "0: 1 стоп бит"]
    _1bit = 0,
    #[doc = "1: 2 стоп бита"]
    _2bits = 1,
}
impl From<Stop1> for bool {
    #[inline(always)]
    fn from(variant: Stop1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP_1` reader - Количество стоп битов для приемника и передатчика. Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type Stop1R = crate::BitReader<Stop1>;
impl Stop1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stop1 {
        match self.bits {
            false => Stop1::_1bit,
            true => Stop1::_2bits,
        }
    }
    #[doc = "1 стоп бит"]
    #[inline(always)]
    pub fn is_1bit(&self) -> bool {
        *self == Stop1::_1bit
    }
    #[doc = "2 стоп бита"]
    #[inline(always)]
    pub fn is_2bits(&self) -> bool {
        *self == Stop1::_2bits
    }
}
#[doc = "Field `STOP_1` writer - Количество стоп битов для приемника и передатчика. Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type Stop1W<'a, REG> = crate::BitWriter<'a, REG, Stop1>;
impl<'a, REG> Stop1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1 стоп бит"]
    #[inline(always)]
    pub fn _1bit(self) -> &'a mut crate::W<REG> {
        self.variant(Stop1::_1bit)
    }
    #[doc = "2 стоп бита"]
    #[inline(always)]
    pub fn _2bits(self) -> &'a mut crate::W<REG> {
        self.variant(Stop1::_2bits)
    }
}
#[doc = "Регистр режима обратной внутренней петли. Этот бит может быть изменен только при остановке работы (UE=0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbm {
    #[doc = "0: Обычное функционирование"]
    Normal = 0,
    #[doc = "1: Сигналы TX и RTS подаются внутрь блока в обход входов RX и CTS. При этом TX = 1 и RTS = 1, входы RX и CTS не активны"]
    Loopback = 1,
}
impl From<Lbm> for bool {
    #[inline(always)]
    fn from(variant: Lbm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBM` reader - Регистр режима обратной внутренней петли. Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type LbmR = crate::BitReader<Lbm>;
impl LbmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbm {
        match self.bits {
            false => Lbm::Normal,
            true => Lbm::Loopback,
        }
    }
    #[doc = "Обычное функционирование"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Lbm::Normal
    }
    #[doc = "Сигналы TX и RTS подаются внутрь блока в обход входов RX и CTS. При этом TX = 1 и RTS = 1, входы RX и CTS не активны"]
    #[inline(always)]
    pub fn is_loopback(&self) -> bool {
        *self == Lbm::Loopback
    }
}
#[doc = "Field `LBM` writer - Регистр режима обратной внутренней петли. Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type LbmW<'a, REG> = crate::BitWriter<'a, REG, Lbm>;
impl<'a, REG> LbmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Обычное функционирование"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Lbm::Normal)
    }
    #[doc = "Сигналы TX и RTS подаются внутрь блока в обход входов RX и CTS. При этом TX = 1 и RTS = 1, входы RX и CTS не активны"]
    #[inline(always)]
    pub fn loopback(self) -> &'a mut crate::W<REG> {
        self.variant(Lbm::Loopback)
    }
}
#[doc = "Обмен функциями TX выхода и RX входа. Этот бит может быть изменен только при остановке работы (UE=0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swap {
    #[doc = "0: Обычное функционирование"]
    Normal = 0,
    #[doc = "1: Обмен функциями (TX = RX и RX = TX)."]
    Swap = 1,
}
impl From<Swap> for bool {
    #[inline(always)]
    fn from(variant: Swap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWAP` reader - Обмен функциями TX выхода и RX входа. Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type SwapR = crate::BitReader<Swap>;
impl SwapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swap {
        match self.bits {
            false => Swap::Normal,
            true => Swap::Swap,
        }
    }
    #[doc = "Обычное функционирование"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Swap::Normal
    }
    #[doc = "Обмен функциями (TX = RX и RX = TX)."]
    #[inline(always)]
    pub fn is_swap(&self) -> bool {
        *self == Swap::Swap
    }
}
#[doc = "Field `SWAP` writer - Обмен функциями TX выхода и RX входа. Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type SwapW<'a, REG> = crate::BitWriter<'a, REG, Swap>;
impl<'a, REG> SwapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Обычное функционирование"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Swap::Normal)
    }
    #[doc = "Обмен функциями (TX = RX и RX = TX)."]
    #[inline(always)]
    pub fn swap(self) -> &'a mut crate::W<REG> {
        self.variant(Swap::Swap)
    }
}
#[doc = "Управление полярностью RX выхода. Регистр влияет не только на данные, но и на стоп биты, старт биты и тд. Этот бит может быть изменен только при остановке работы (UE=0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxinv {
    #[doc = "0: Обычная полярность. 0 - низкий уровень, 1 - высокий уровень"]
    Direct = 0,
    #[doc = "1: Обратная полярность. 1 - низкий уровень, 0 - высокий уровень"]
    Inverted = 1,
}
impl From<Rxinv> for bool {
    #[inline(always)]
    fn from(variant: Rxinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXINV` reader - Управление полярностью RX выхода. Регистр влияет не только на данные, но и на стоп биты, старт биты и тд. Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type RxinvR = crate::BitReader<Rxinv>;
impl RxinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxinv {
        match self.bits {
            false => Rxinv::Direct,
            true => Rxinv::Inverted,
        }
    }
    #[doc = "Обычная полярность. 0 - низкий уровень, 1 - высокий уровень"]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        *self == Rxinv::Direct
    }
    #[doc = "Обратная полярность. 1 - низкий уровень, 0 - высокий уровень"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == Rxinv::Inverted
    }
}
#[doc = "Field `RXINV` writer - Управление полярностью RX выхода. Регистр влияет не только на данные, но и на стоп биты, старт биты и тд. Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type RxinvW<'a, REG> = crate::BitWriter<'a, REG, Rxinv>;
impl<'a, REG> RxinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Обычная полярность. 0 - низкий уровень, 1 - высокий уровень"]
    #[inline(always)]
    pub fn direct(self) -> &'a mut crate::W<REG> {
        self.variant(Rxinv::Direct)
    }
    #[doc = "Обратная полярность. 1 - низкий уровень, 0 - высокий уровень"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Rxinv::Inverted)
    }
}
#[doc = "Управление полярностью TX выхода. Регистр влияет не только на данные, но и на стоп биты, старт биты и тд. Этот бит может быть изменен только при остановке работы (UE=0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txinv {
    #[doc = "0: Обычная полярность. 0 - низкий уровень, 1 - высокий уровень"]
    Direct = 0,
    #[doc = "1: Обратная полярность. 1 - низкий уровень, 0 - высокий уровень"]
    Inverted = 1,
}
impl From<Txinv> for bool {
    #[inline(always)]
    fn from(variant: Txinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXINV` reader - Управление полярностью TX выхода. Регистр влияет не только на данные, но и на стоп биты, старт биты и тд. Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type TxinvR = crate::BitReader<Txinv>;
impl TxinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txinv {
        match self.bits {
            false => Txinv::Direct,
            true => Txinv::Inverted,
        }
    }
    #[doc = "Обычная полярность. 0 - низкий уровень, 1 - высокий уровень"]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        *self == Txinv::Direct
    }
    #[doc = "Обратная полярность. 1 - низкий уровень, 0 - высокий уровень"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == Txinv::Inverted
    }
}
#[doc = "Field `TXINV` writer - Управление полярностью TX выхода. Регистр влияет не только на данные, но и на стоп биты, старт биты и тд. Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type TxinvW<'a, REG> = crate::BitWriter<'a, REG, Txinv>;
impl<'a, REG> TxinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Обычная полярность. 0 - низкий уровень, 1 - высокий уровень"]
    #[inline(always)]
    pub fn direct(self) -> &'a mut crate::W<REG> {
        self.variant(Txinv::Direct)
    }
    #[doc = "Обратная полярность. 1 - низкий уровень, 0 - высокий уровень"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Txinv::Inverted)
    }
}
#[doc = "Управление полярностью принимаемых и передаваемых данных. Изменение полярности не влияет на бит четности. Этот бит может быть изменен только при остановке работы (UE=0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datainv {
    #[doc = "0: Обычная полярность. 0 - низкий уровень, 1 - высокий уровень"]
    Direct = 0,
    #[doc = "1: Обратная полярность. 1 - низкий уровень, 0 - высокий уровень"]
    Inverted = 1,
}
impl From<Datainv> for bool {
    #[inline(always)]
    fn from(variant: Datainv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAINV` reader - Управление полярностью принимаемых и передаваемых данных. Изменение полярности не влияет на бит четности. Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type DatainvR = crate::BitReader<Datainv>;
impl DatainvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datainv {
        match self.bits {
            false => Datainv::Direct,
            true => Datainv::Inverted,
        }
    }
    #[doc = "Обычная полярность. 0 - низкий уровень, 1 - высокий уровень"]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        *self == Datainv::Direct
    }
    #[doc = "Обратная полярность. 1 - низкий уровень, 0 - высокий уровень"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == Datainv::Inverted
    }
}
#[doc = "Field `DATAINV` writer - Управление полярностью принимаемых и передаваемых данных. Изменение полярности не влияет на бит четности. Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type DatainvW<'a, REG> = crate::BitWriter<'a, REG, Datainv>;
impl<'a, REG> DatainvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Обычная полярность. 0 - низкий уровень, 1 - высокий уровень"]
    #[inline(always)]
    pub fn direct(self) -> &'a mut crate::W<REG> {
        self.variant(Datainv::Direct)
    }
    #[doc = "Обратная полярность. 1 - низкий уровень, 0 - высокий уровень"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(Datainv::Inverted)
    }
}
#[doc = "Управление очередностью приема и передачи данных. Этот бит не влияет на передачу и прием бита четности. Этот бит может быть изменен только при остановке работы (UE=0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msbfirst {
    #[doc = "0: Начинать с 0 бита"]
    Lsb = 0,
    #[doc = "1: Начинать с 9, 8 или 7 в зависимости от настроек длины посылки."]
    Msb = 1,
}
impl From<Msbfirst> for bool {
    #[inline(always)]
    fn from(variant: Msbfirst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSBFIRST` reader - Управление очередностью приема и передачи данных. Этот бит не влияет на передачу и прием бита четности. Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type MsbfirstR = crate::BitReader<Msbfirst>;
impl MsbfirstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msbfirst {
        match self.bits {
            false => Msbfirst::Lsb,
            true => Msbfirst::Msb,
        }
    }
    #[doc = "Начинать с 0 бита"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == Msbfirst::Lsb
    }
    #[doc = "Начинать с 9, 8 или 7 в зависимости от настроек длины посылки."]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == Msbfirst::Msb
    }
}
#[doc = "Field `MSBFIRST` writer - Управление очередностью приема и передачи данных. Этот бит не влияет на передачу и прием бита четности. Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type MsbfirstW<'a, REG> = crate::BitWriter<'a, REG, Msbfirst>;
impl<'a, REG> MsbfirstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Начинать с 0 бита"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut crate::W<REG> {
        self.variant(Msbfirst::Lsb)
    }
    #[doc = "Начинать с 9, 8 или 7 в зависимости от настроек длины посылки."]
    #[inline(always)]
    pub fn msb(self) -> &'a mut crate::W<REG> {
        self.variant(Msbfirst::Msb)
    }
}
impl R {
    #[doc = "Bit 6 - Управление прерыванием при обнаружении break состояния на RX линии"]
    #[inline(always)]
    pub fn lbdie(&self) -> LbdieR {
        LbdieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Управление последним тактовым импульсом. Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn lbcl(&self) -> LbclR {
        LbclR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Управление фазой выходного тактового сигнала CK. Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Управление полярностью выходного тактового сигнала CK. Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Управление синхронным режимом. Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn clken(&self) -> ClkenR {
        ClkenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Количество стоп битов для приемника и передатчика. Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn stop_1(&self) -> Stop1R {
        Stop1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Регистр режима обратной внутренней петли. Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn lbm(&self) -> LbmR {
        LbmR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Обмен функциями TX выхода и RX входа. Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn swap(&self) -> SwapR {
        SwapR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Управление полярностью RX выхода. Регистр влияет не только на данные, но и на стоп биты, старт биты и тд. Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn rxinv(&self) -> RxinvR {
        RxinvR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Управление полярностью TX выхода. Регистр влияет не только на данные, но и на стоп биты, старт биты и тд. Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn txinv(&self) -> TxinvR {
        TxinvR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Управление полярностью принимаемых и передаваемых данных. Изменение полярности не влияет на бит четности. Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn datainv(&self) -> DatainvR {
        DatainvR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Управление очередностью приема и передачи данных. Этот бит не влияет на передачу и прием бита четности. Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn msbfirst(&self) -> MsbfirstR {
        MsbfirstR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Управление прерыванием при обнаружении break состояния на RX линии"]
    #[inline(always)]
    pub fn lbdie(&mut self) -> LbdieW<Control2Spec> {
        LbdieW::new(self, 6)
    }
    #[doc = "Bit 8 - Управление последним тактовым импульсом. Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn lbcl(&mut self) -> LbclW<Control2Spec> {
        LbclW::new(self, 8)
    }
    #[doc = "Bit 9 - Управление фазой выходного тактового сигнала CK. Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CphaW<Control2Spec> {
        CphaW::new(self, 9)
    }
    #[doc = "Bit 10 - Управление полярностью выходного тактового сигнала CK. Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<Control2Spec> {
        CpolW::new(self, 10)
    }
    #[doc = "Bit 11 - Управление синхронным режимом. Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn clken(&mut self) -> ClkenW<Control2Spec> {
        ClkenW::new(self, 11)
    }
    #[doc = "Bit 13 - Количество стоп битов для приемника и передатчика. Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn stop_1(&mut self) -> Stop1W<Control2Spec> {
        Stop1W::new(self, 13)
    }
    #[doc = "Bit 14 - Регистр режима обратной внутренней петли. Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn lbm(&mut self) -> LbmW<Control2Spec> {
        LbmW::new(self, 14)
    }
    #[doc = "Bit 15 - Обмен функциями TX выхода и RX входа. Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn swap(&mut self) -> SwapW<Control2Spec> {
        SwapW::new(self, 15)
    }
    #[doc = "Bit 16 - Управление полярностью RX выхода. Регистр влияет не только на данные, но и на стоп биты, старт биты и тд. Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn rxinv(&mut self) -> RxinvW<Control2Spec> {
        RxinvW::new(self, 16)
    }
    #[doc = "Bit 17 - Управление полярностью TX выхода. Регистр влияет не только на данные, но и на стоп биты, старт биты и тд. Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn txinv(&mut self) -> TxinvW<Control2Spec> {
        TxinvW::new(self, 17)
    }
    #[doc = "Bit 18 - Управление полярностью принимаемых и передаваемых данных. Изменение полярности не влияет на бит четности. Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn datainv(&mut self) -> DatainvW<Control2Spec> {
        DatainvW::new(self, 18)
    }
    #[doc = "Bit 19 - Управление очередностью приема и передачи данных. Этот бит не влияет на передачу и прием бита четности. Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn msbfirst(&mut self) -> MsbfirstW<Control2Spec> {
        MsbfirstW::new(self, 19)
    }
}
#[doc = "Регистр управления 2\n\nYou can [`read`](crate::Reg::read) this register and get [`control2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Control2Spec;
impl crate::RegisterSpec for Control2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control2::R`](R) reader structure"]
impl crate::Readable for Control2Spec {}
#[doc = "`write(|w| ..)` method takes [`control2::W`](W) writer structure"]
impl crate::Writable for Control2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONTROL2 to value 0"]
impl crate::Resettable for Control2Spec {
    const RESET_VALUE: u32 = 0;
}
