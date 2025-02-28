#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `TIMEOUT` reader - Количество периодов сигнала SPIFI_SCK без чтения данных в режиме работы с памятью, которое вызывает завершение выполнения команды установкой сигнала SPIFI_CS в состояние «1» и сбросом бита CMD"]
pub type TimeoutR = crate::FieldReader<u16>;
#[doc = "Field `TIMEOUT` writer - Количество периодов сигнала SPIFI_SCK без чтения данных в режиме работы с памятью, которое вызывает завершение выполнения команды установкой сигнала SPIFI_CS в состояние «1» и сбросом бита CMD"]
pub type TimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CSHIGH` reader - Количество периодов сигнала SPIFI_SCK минус один, в течение которых сигнал SPIFI_CS остается в неактивном состоянии перед началом выполнения команды"]
pub type CshighR = crate::FieldReader;
#[doc = "Field `CSHIGH` writer - Количество периодов сигнала SPIFI_SCK минус один, в течение которых сигнал SPIFI_CS остается в неактивном состоянии перед началом выполнения команды"]
pub type CshighW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Бит разрешения кэширования\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CacheEn {
    #[doc = "0: Кэширование отключено"]
    Disable = 0,
    #[doc = "1: Кэширование включено"]
    Enable = 1,
}
impl From<CacheEn> for bool {
    #[inline(always)]
    fn from(variant: CacheEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CACHE_EN` reader - Бит разрешения кэширования"]
pub type CacheEnR = crate::BitReader<CacheEn>;
impl CacheEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CacheEn {
        match self.bits {
            false => CacheEn::Disable,
            true => CacheEn::Enable,
        }
    }
    #[doc = "Кэширование отключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CacheEn::Disable
    }
    #[doc = "Кэширование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CacheEn::Enable
    }
}
#[doc = "Field `CACHE_EN` writer - Бит разрешения кэширования"]
pub type CacheEnW<'a, REG> = crate::BitWriter<'a, REG, CacheEn>;
impl<'a, REG> CacheEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Кэширование отключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CacheEn::Disable)
    }
    #[doc = "Кэширование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CacheEn::Enable)
    }
}
#[doc = "Бит запрещения кеширования данных (транзакций AHB, для которых сигнал HPROT\\[0:0\\]
= 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCacheDis {
    #[doc = "0: При разрешенном кэшировании кэширование данных выполняется"]
    DataIsCached = 0,
    #[doc = "1: Данные не кэшируются"]
    DataIsNotCached = 1,
}
impl From<DCacheDis> for bool {
    #[inline(always)]
    fn from(variant: DCacheDis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `D_CACHE_DIS` reader - Бит запрещения кеширования данных (транзакций AHB, для которых сигнал HPROT\\[0:0\\]
= 1)"]
pub type DCacheDisR = crate::BitReader<DCacheDis>;
impl DCacheDisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DCacheDis {
        match self.bits {
            false => DCacheDis::DataIsCached,
            true => DCacheDis::DataIsNotCached,
        }
    }
    #[doc = "При разрешенном кэшировании кэширование данных выполняется"]
    #[inline(always)]
    pub fn is_data_is_cached(&self) -> bool {
        *self == DCacheDis::DataIsCached
    }
    #[doc = "Данные не кэшируются"]
    #[inline(always)]
    pub fn is_data_is_not_cached(&self) -> bool {
        *self == DCacheDis::DataIsNotCached
    }
}
#[doc = "Field `D_CACHE_DIS` writer - Бит запрещения кеширования данных (транзакций AHB, для которых сигнал HPROT\\[0:0\\]
= 1)"]
pub type DCacheDisW<'a, REG> = crate::BitWriter<'a, REG, DCacheDis>;
impl<'a, REG> DCacheDisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "При разрешенном кэшировании кэширование данных выполняется"]
    #[inline(always)]
    pub fn data_is_cached(self) -> &'a mut crate::W<REG> {
        self.variant(DCacheDis::DataIsCached)
    }
    #[doc = "Данные не кэшируются"]
    #[inline(always)]
    pub fn data_is_not_cached(self) -> &'a mut crate::W<REG> {
        self.variant(DCacheDis::DataIsNotCached)
    }
}
#[doc = "Бит разрешения прерывания при завершении выполнения команды (если этот бит равен «1», то прерывание разрешено)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inten {
    #[doc = "0: Прерывнаие запрещено"]
    Disable = 0,
    #[doc = "1: Прерывание разрешено"]
    Enable = 1,
}
impl From<Inten> for bool {
    #[inline(always)]
    fn from(variant: Inten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEN` reader - Бит разрешения прерывания при завершении выполнения команды (если этот бит равен «1», то прерывание разрешено)"]
pub type IntenR = crate::BitReader<Inten>;
impl IntenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inten {
        match self.bits {
            false => Inten::Disable,
            true => Inten::Enable,
        }
    }
    #[doc = "Прерывнаие запрещено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Inten::Disable
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Inten::Enable
    }
}
#[doc = "Field `INTEN` writer - Бит разрешения прерывания при завершении выполнения команды (если этот бит равен «1», то прерывание разрешено)"]
pub type IntenW<'a, REG> = crate::BitWriter<'a, REG, Inten>;
impl<'a, REG> IntenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывнаие запрещено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Inten::Disable)
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Inten::Enable)
    }
}
#[doc = "Бит режима 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode3 {
    #[doc = "0: Режим 0 (CPOL = 0, CPHA = 0). CPOL = 0 — исходное состояние сигнала синхронизации - низкий уровень CPHA = 0 — выборка данных производится по переднему фронту (переключению) сигнала синхронизации"]
    Mode0 = 0,
    #[doc = "1: Режим 3 (CPOL = 1, CPHA = 1). CPHA = 1 - исходное состояние сигнала синхронизации - высокий уровень CPOL = 1 - выборка данных производится по заднему фронту (переключению) сигнала синхронизации"]
    Mode3 = 1,
}
impl From<Mode3> for bool {
    #[inline(always)]
    fn from(variant: Mode3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE3` reader - Бит режима 3"]
pub type Mode3R = crate::BitReader<Mode3>;
impl Mode3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode3 {
        match self.bits {
            false => Mode3::Mode0,
            true => Mode3::Mode3,
        }
    }
    #[doc = "Режим 0 (CPOL = 0, CPHA = 0). CPOL = 0 — исходное состояние сигнала синхронизации - низкий уровень CPHA = 0 — выборка данных производится по переднему фронту (переключению) сигнала синхронизации"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == Mode3::Mode0
    }
    #[doc = "Режим 3 (CPOL = 1, CPHA = 1). CPHA = 1 - исходное состояние сигнала синхронизации - высокий уровень CPOL = 1 - выборка данных производится по заднему фронту (переключению) сигнала синхронизации"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == Mode3::Mode3
    }
}
#[doc = "Field `MODE3` writer - Бит режима 3"]
pub type Mode3W<'a, REG> = crate::BitWriter<'a, REG, Mode3>;
impl<'a, REG> Mode3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Режим 0 (CPOL = 0, CPHA = 0). CPOL = 0 — исходное состояние сигнала синхронизации - низкий уровень CPHA = 0 — выборка данных производится по переднему фронту (переключению) сигнала синхронизации"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut crate::W<REG> {
        self.variant(Mode3::Mode0)
    }
    #[doc = "Режим 3 (CPOL = 1, CPHA = 1). CPHA = 1 - исходное состояние сигнала синхронизации - высокий уровень CPOL = 1 - выборка данных производится по заднему фронту (переключению) сигнала синхронизации"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
        self.variant(Mode3::Mode3)
    }
}
#[doc = "Field `SCK_DIV` reader - Делитель для получения тактового сигнала SPIFI_SCK из системного тактового сигнала HCLK. Частота рассчитывается по формуле: FSPIFI_SCK = FHCLK / 2^(SCK_DIV+1)"]
pub type SckDivR = crate::FieldReader;
#[doc = "Field `SCK_DIV` writer - Делитель для получения тактового сигнала SPIFI_SCK из системного тактового сигнала HCLK. Частота рассчитывается по формуле: FSPIFI_SCK = FHCLK / 2^(SCK_DIV+1)"]
pub type SckDivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Бит разрешения упреждающих выборок кэш памяти «0» – выборки разрешены\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrftchDis {
    #[doc = "0: Выборки разрешены"]
    Enable = 0,
    #[doc = "1: Выборки запрещены"]
    Disable = 1,
}
impl From<PrftchDis> for bool {
    #[inline(always)]
    fn from(variant: PrftchDis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRFTCH_DIS` reader - Бит разрешения упреждающих выборок кэш памяти «0» – выборки разрешены"]
pub type PrftchDisR = crate::BitReader<PrftchDis>;
impl PrftchDisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PrftchDis {
        match self.bits {
            false => PrftchDis::Enable,
            true => PrftchDis::Disable,
        }
    }
    #[doc = "Выборки разрешены"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PrftchDis::Enable
    }
    #[doc = "Выборки запрещены"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PrftchDis::Disable
    }
}
#[doc = "Field `PRFTCH_DIS` writer - Бит разрешения упреждающих выборок кэш памяти «0» – выборки разрешены"]
pub type PrftchDisW<'a, REG> = crate::BitWriter<'a, REG, PrftchDis>;
impl<'a, REG> PrftchDisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Выборки разрешены"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PrftchDis::Enable)
    }
    #[doc = "Выборки запрещены"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PrftchDis::Disable)
    }
}
#[doc = "Бит выбора протокола: сигналы IO\\[3:0\\]
– (0–4) битовый протокол; сигналы IO\\[1:0\\]
– (1–2) битовый протокол\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dual {
    #[doc = "0: Сигналы IO\\[3:0\\]
– (0–4) битовый протокол"]
    SingleOrQuad = 0,
    #[doc = "1: Сигналы IO\\[1:0\\]
– (1–2) битовый протокол"]
    Dual = 1,
}
impl From<Dual> for bool {
    #[inline(always)]
    fn from(variant: Dual) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DUAL` reader - Бит выбора протокола: сигналы IO\\[3:0\\]
– (0–4) битовый протокол; сигналы IO\\[1:0\\]
– (1–2) битовый протокол"]
pub type DualR = crate::BitReader<Dual>;
impl DualR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dual {
        match self.bits {
            false => Dual::SingleOrQuad,
            true => Dual::Dual,
        }
    }
    #[doc = "Сигналы IO\\[3:0\\]
– (0–4) битовый протокол"]
    #[inline(always)]
    pub fn is_single_or_quad(&self) -> bool {
        *self == Dual::SingleOrQuad
    }
    #[doc = "Сигналы IO\\[1:0\\]
– (1–2) битовый протокол"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == Dual::Dual
    }
}
#[doc = "Field `DUAL` writer - Бит выбора протокола: сигналы IO\\[3:0\\]
– (0–4) битовый протокол; сигналы IO\\[1:0\\]
– (1–2) битовый протокол"]
pub type DualW<'a, REG> = crate::BitWriter<'a, REG, Dual>;
impl<'a, REG> DualW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Сигналы IO\\[3:0\\]
– (0–4) битовый протокол"]
    #[inline(always)]
    pub fn single_or_quad(self) -> &'a mut crate::W<REG> {
        self.variant(Dual::SingleOrQuad)
    }
    #[doc = "Сигналы IO\\[1:0\\]
– (1–2) битовый протокол"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut crate::W<REG> {
        self.variant(Dual::Dual)
    }
}
#[doc = "Бит выбора активного перепада сигнала для стробирования входных данных\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfclk {
    #[doc = "0: Положительный перепад"]
    RisingEdge = 0,
    #[doc = "1: Отрицательный перепад"]
    FallingEdge = 1,
}
impl From<Rfclk> for bool {
    #[inline(always)]
    fn from(variant: Rfclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFCLK` reader - Бит выбора активного перепада сигнала для стробирования входных данных"]
pub type RfclkR = crate::BitReader<Rfclk>;
impl RfclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfclk {
        match self.bits {
            false => Rfclk::RisingEdge,
            true => Rfclk::FallingEdge,
        }
    }
    #[doc = "Положительный перепад"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == Rfclk::RisingEdge
    }
    #[doc = "Отрицательный перепад"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == Rfclk::FallingEdge
    }
}
#[doc = "Field `RFCLK` writer - Бит выбора активного перепада сигнала для стробирования входных данных"]
pub type RfclkW<'a, REG> = crate::BitWriter<'a, REG, Rfclk>;
impl<'a, REG> RfclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Положительный перепад"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Rfclk::RisingEdge)
    }
    #[doc = "Отрицательный перепад"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Rfclk::FallingEdge)
    }
}
#[doc = "Бит выбора сигнала стробирования входных данных с выходного буфера тактового сигнала\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fblk {
    #[doc = "0: Внутренний сигнал"]
    InternalSignal = 0,
    #[doc = "1: Внешний сигнал"]
    ExternalSignal = 1,
}
impl From<Fblk> for bool {
    #[inline(always)]
    fn from(variant: Fblk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FBLK` reader - Бит выбора сигнала стробирования входных данных с выходного буфера тактового сигнала"]
pub type FblkR = crate::BitReader<Fblk>;
impl FblkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fblk {
        match self.bits {
            false => Fblk::InternalSignal,
            true => Fblk::ExternalSignal,
        }
    }
    #[doc = "Внутренний сигнал"]
    #[inline(always)]
    pub fn is_internal_signal(&self) -> bool {
        *self == Fblk::InternalSignal
    }
    #[doc = "Внешний сигнал"]
    #[inline(always)]
    pub fn is_external_signal(&self) -> bool {
        *self == Fblk::ExternalSignal
    }
}
#[doc = "Field `FBLK` writer - Бит выбора сигнала стробирования входных данных с выходного буфера тактового сигнала"]
pub type FblkW<'a, REG> = crate::BitWriter<'a, REG, Fblk>;
impl<'a, REG> FblkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Внутренний сигнал"]
    #[inline(always)]
    pub fn internal_signal(self) -> &'a mut crate::W<REG> {
        self.variant(Fblk::InternalSignal)
    }
    #[doc = "Внешний сигнал"]
    #[inline(always)]
    pub fn external_signal(self) -> &'a mut crate::W<REG> {
        self.variant(Fblk::ExternalSignal)
    }
}
#[doc = "Бит разрешения запросов DMA от контроллера SPIFI Устанавливать только в периферийном режиме\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaen {
    #[doc = "0: Запросы запрещены"]
    Disable = 0,
    #[doc = "1: Запросы разрешены"]
    Enable = 1,
}
impl From<Dmaen> for bool {
    #[inline(always)]
    fn from(variant: Dmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - Бит разрешения запросов DMA от контроллера SPIFI Устанавливать только в периферийном режиме"]
pub type DmaenR = crate::BitReader<Dmaen>;
impl DmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaen {
        match self.bits {
            false => Dmaen::Disable,
            true => Dmaen::Enable,
        }
    }
    #[doc = "Запросы запрещены"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dmaen::Disable
    }
    #[doc = "Запросы разрешены"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dmaen::Enable
    }
}
#[doc = "Field `DMAEN` writer - Бит разрешения запросов DMA от контроллера SPIFI Устанавливать только в периферийном режиме"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG, Dmaen>;
impl<'a, REG> DmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Запросы запрещены"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::Disable)
    }
    #[doc = "Запросы разрешены"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::Enable)
    }
}
impl R {
    #[doc = "Bits 0:15 - Количество периодов сигнала SPIFI_SCK без чтения данных в режиме работы с памятью, которое вызывает завершение выполнения команды установкой сигнала SPIFI_CS в состояние «1» и сбросом бита CMD"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Количество периодов сигнала SPIFI_SCK минус один, в течение которых сигнал SPIFI_CS остается в неактивном состоянии перед началом выполнения команды"]
    #[inline(always)]
    pub fn cshigh(&self) -> CshighR {
        CshighR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Бит разрешения кэширования"]
    #[inline(always)]
    pub fn cache_en(&self) -> CacheEnR {
        CacheEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Бит запрещения кеширования данных (транзакций AHB, для которых сигнал HPROT\\[0:0\\]
= 1)"]
    #[inline(always)]
    pub fn d_cache_dis(&self) -> DCacheDisR {
        DCacheDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Бит разрешения прерывания при завершении выполнения команды (если этот бит равен «1», то прерывание разрешено)"]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Бит режима 3"]
    #[inline(always)]
    pub fn mode3(&self) -> Mode3R {
        Mode3R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Делитель для получения тактового сигнала SPIFI_SCK из системного тактового сигнала HCLK. Частота рассчитывается по формуле: FSPIFI_SCK = FHCLK / 2^(SCK_DIV+1)"]
    #[inline(always)]
    pub fn sck_div(&self) -> SckDivR {
        SckDivR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Бит разрешения упреждающих выборок кэш памяти «0» – выборки разрешены"]
    #[inline(always)]
    pub fn prftch_dis(&self) -> PrftchDisR {
        PrftchDisR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Бит выбора протокола: сигналы IO\\[3:0\\]
– (0–4) битовый протокол; сигналы IO\\[1:0\\]
– (1–2) битовый протокол"]
    #[inline(always)]
    pub fn dual(&self) -> DualR {
        DualR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Бит выбора активного перепада сигнала для стробирования входных данных"]
    #[inline(always)]
    pub fn rfclk(&self) -> RfclkR {
        RfclkR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Бит выбора сигнала стробирования входных данных с выходного буфера тактового сигнала"]
    #[inline(always)]
    pub fn fblk(&self) -> FblkR {
        FblkR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Бит разрешения запросов DMA от контроллера SPIFI Устанавливать только в периферийном режиме"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Количество периодов сигнала SPIFI_SCK без чтения данных в режиме работы с памятью, которое вызывает завершение выполнения команды установкой сигнала SPIFI_CS в состояние «1» и сбросом бита CMD"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TimeoutW<CtrlSpec> {
        TimeoutW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Количество периодов сигнала SPIFI_SCK минус один, в течение которых сигнал SPIFI_CS остается в неактивном состоянии перед началом выполнения команды"]
    #[inline(always)]
    pub fn cshigh(&mut self) -> CshighW<CtrlSpec> {
        CshighW::new(self, 16)
    }
    #[doc = "Bit 20 - Бит разрешения кэширования"]
    #[inline(always)]
    pub fn cache_en(&mut self) -> CacheEnW<CtrlSpec> {
        CacheEnW::new(self, 20)
    }
    #[doc = "Bit 21 - Бит запрещения кеширования данных (транзакций AHB, для которых сигнал HPROT\\[0:0\\]
= 1)"]
    #[inline(always)]
    pub fn d_cache_dis(&mut self) -> DCacheDisW<CtrlSpec> {
        DCacheDisW::new(self, 21)
    }
    #[doc = "Bit 22 - Бит разрешения прерывания при завершении выполнения команды (если этот бит равен «1», то прерывание разрешено)"]
    #[inline(always)]
    pub fn inten(&mut self) -> IntenW<CtrlSpec> {
        IntenW::new(self, 22)
    }
    #[doc = "Bit 23 - Бит режима 3"]
    #[inline(always)]
    pub fn mode3(&mut self) -> Mode3W<CtrlSpec> {
        Mode3W::new(self, 23)
    }
    #[doc = "Bits 24:26 - Делитель для получения тактового сигнала SPIFI_SCK из системного тактового сигнала HCLK. Частота рассчитывается по формуле: FSPIFI_SCK = FHCLK / 2^(SCK_DIV+1)"]
    #[inline(always)]
    pub fn sck_div(&mut self) -> SckDivW<CtrlSpec> {
        SckDivW::new(self, 24)
    }
    #[doc = "Bit 27 - Бит разрешения упреждающих выборок кэш памяти «0» – выборки разрешены"]
    #[inline(always)]
    pub fn prftch_dis(&mut self) -> PrftchDisW<CtrlSpec> {
        PrftchDisW::new(self, 27)
    }
    #[doc = "Bit 28 - Бит выбора протокола: сигналы IO\\[3:0\\]
– (0–4) битовый протокол; сигналы IO\\[1:0\\]
– (1–2) битовый протокол"]
    #[inline(always)]
    pub fn dual(&mut self) -> DualW<CtrlSpec> {
        DualW::new(self, 28)
    }
    #[doc = "Bit 29 - Бит выбора активного перепада сигнала для стробирования входных данных"]
    #[inline(always)]
    pub fn rfclk(&mut self) -> RfclkW<CtrlSpec> {
        RfclkW::new(self, 29)
    }
    #[doc = "Bit 30 - Бит выбора сигнала стробирования входных данных с выходного буфера тактового сигнала"]
    #[inline(always)]
    pub fn fblk(&mut self) -> FblkW<CtrlSpec> {
        FblkW::new(self, 30)
    }
    #[doc = "Bit 31 - Бит разрешения запросов DMA от контроллера SPIFI Устанавливать только в периферийном режиме"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<CtrlSpec> {
        DmaenW::new(self, 31)
    }
}
#[doc = "SPIFI регистр управления\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x400f_ffff"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x400f_ffff;
}
