#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Задаёт режим хода вычислительного ядра\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Decode {
    #[doc = "0: Прямой ход (шифрование)"]
    Encode = 0,
    #[doc = "1: Обратный ход (расшифровка)"]
    Decode = 1,
}
impl From<Decode> for bool {
    #[inline(always)]
    fn from(variant: Decode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DECODE` reader - Задаёт режим хода вычислительного ядра"]
pub type DecodeR = crate::BitReader<Decode>;
impl DecodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Decode {
        match self.bits {
            false => Decode::Encode,
            true => Decode::Decode,
        }
    }
    #[doc = "Прямой ход (шифрование)"]
    #[inline(always)]
    pub fn is_encode(&self) -> bool {
        *self == Decode::Encode
    }
    #[doc = "Обратный ход (расшифровка)"]
    #[inline(always)]
    pub fn is_decode(&self) -> bool {
        *self == Decode::Decode
    }
}
#[doc = "Field `DECODE` writer - Задаёт режим хода вычислительного ядра"]
pub type DecodeW<'a, REG> = crate::BitWriter<'a, REG, Decode>;
impl<'a, REG> DecodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прямой ход (шифрование)"]
    #[inline(always)]
    pub fn encode(self) -> &'a mut crate::W<REG> {
        self.variant(Decode::Encode)
    }
    #[doc = "Обратный ход (расшифровка)"]
    #[inline(always)]
    pub fn decode(self) -> &'a mut crate::W<REG> {
        self.variant(Decode::Decode)
    }
}
#[doc = "Регистр выбора алгоритма шифрования\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CoreSel {
    #[doc = "0: «Кузнечик»"]
    Kuznechik = 0,
    #[doc = "1: «Магма»"]
    Magma = 1,
    #[doc = "2: «AES»"]
    Aes = 2,
}
impl From<CoreSel> for u8 {
    #[inline(always)]
    fn from(variant: CoreSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CoreSel {
    type Ux = u8;
}
impl crate::IsEnum for CoreSel {}
#[doc = "Field `CORE_SEL` reader - Регистр выбора алгоритма шифрования"]
pub type CoreSelR = crate::FieldReader<CoreSel>;
impl CoreSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CoreSel> {
        match self.bits {
            0 => Some(CoreSel::Kuznechik),
            1 => Some(CoreSel::Magma),
            2 => Some(CoreSel::Aes),
            _ => None,
        }
    }
    #[doc = "«Кузнечик»"]
    #[inline(always)]
    pub fn is_kuznechik(&self) -> bool {
        *self == CoreSel::Kuznechik
    }
    #[doc = "«Магма»"]
    #[inline(always)]
    pub fn is_magma(&self) -> bool {
        *self == CoreSel::Magma
    }
    #[doc = "«AES»"]
    #[inline(always)]
    pub fn is_aes(&self) -> bool {
        *self == CoreSel::Aes
    }
}
#[doc = "Field `CORE_SEL` writer - Регистр выбора алгоритма шифрования"]
pub type CoreSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, CoreSel>;
impl<'a, REG> CoreSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "«Кузнечик»"]
    #[inline(always)]
    pub fn kuznechik(self) -> &'a mut crate::W<REG> {
        self.variant(CoreSel::Kuznechik)
    }
    #[doc = "«Магма»"]
    #[inline(always)]
    pub fn magma(self) -> &'a mut crate::W<REG> {
        self.variant(CoreSel::Magma)
    }
    #[doc = "«AES»"]
    #[inline(always)]
    pub fn aes(self) -> &'a mut crate::W<REG> {
        self.variant(CoreSel::Aes)
    }
}
#[doc = "Регистр выбора режима шифрования\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ModeSel {
    #[doc = "0: Режиме простой замены (ECB)"]
    Ecb = 0,
    #[doc = "1: Режим сцепления блоков (CBC)"]
    Cbc = 1,
    #[doc = "2: Режим гаммирования (CTR)"]
    Ctr = 2,
}
impl From<ModeSel> for u8 {
    #[inline(always)]
    fn from(variant: ModeSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ModeSel {
    type Ux = u8;
}
impl crate::IsEnum for ModeSel {}
#[doc = "Field `MODE_SEL` reader - Регистр выбора режима шифрования"]
pub type ModeSelR = crate::FieldReader<ModeSel>;
impl ModeSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ModeSel> {
        match self.bits {
            0 => Some(ModeSel::Ecb),
            1 => Some(ModeSel::Cbc),
            2 => Some(ModeSel::Ctr),
            _ => None,
        }
    }
    #[doc = "Режиме простой замены (ECB)"]
    #[inline(always)]
    pub fn is_ecb(&self) -> bool {
        *self == ModeSel::Ecb
    }
    #[doc = "Режим сцепления блоков (CBC)"]
    #[inline(always)]
    pub fn is_cbc(&self) -> bool {
        *self == ModeSel::Cbc
    }
    #[doc = "Режим гаммирования (CTR)"]
    #[inline(always)]
    pub fn is_ctr(&self) -> bool {
        *self == ModeSel::Ctr
    }
}
#[doc = "Field `MODE_SEL` writer - Регистр выбора режима шифрования"]
pub type ModeSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, ModeSel>;
impl<'a, REG> ModeSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Режиме простой замены (ECB)"]
    #[inline(always)]
    pub fn ecb(self) -> &'a mut crate::W<REG> {
        self.variant(ModeSel::Ecb)
    }
    #[doc = "Режим сцепления блоков (CBC)"]
    #[inline(always)]
    pub fn cbc(self) -> &'a mut crate::W<REG> {
        self.variant(ModeSel::Cbc)
    }
    #[doc = "Режим гаммирования (CTR)"]
    #[inline(always)]
    pub fn ctr(self) -> &'a mut crate::W<REG> {
        self.variant(ModeSel::Ctr)
    }
}
#[doc = "Регистр выбора режима перестановки слова\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwapMode {
    #[doc = "0: Нет перестановки"]
    None = 0,
    #[doc = "1: Перестановка по полуслову"]
    HalfWord = 1,
    #[doc = "2: Перестановки по байтам"]
    Byte = 2,
    #[doc = "3: Перестановка по битам"]
    Bit = 3,
}
impl From<SwapMode> for u8 {
    #[inline(always)]
    fn from(variant: SwapMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwapMode {
    type Ux = u8;
}
impl crate::IsEnum for SwapMode {}
#[doc = "Field `SWAP_MODE` reader - Регистр выбора режима перестановки слова"]
pub type SwapModeR = crate::FieldReader<SwapMode>;
impl SwapModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwapMode {
        match self.bits {
            0 => SwapMode::None,
            1 => SwapMode::HalfWord,
            2 => SwapMode::Byte,
            3 => SwapMode::Bit,
            _ => unreachable!(),
        }
    }
    #[doc = "Нет перестановки"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SwapMode::None
    }
    #[doc = "Перестановка по полуслову"]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == SwapMode::HalfWord
    }
    #[doc = "Перестановки по байтам"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == SwapMode::Byte
    }
    #[doc = "Перестановка по битам"]
    #[inline(always)]
    pub fn is_bit(&self) -> bool {
        *self == SwapMode::Bit
    }
}
#[doc = "Field `SWAP_MODE` writer - Регистр выбора режима перестановки слова"]
pub type SwapModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, SwapMode, crate::Safe>;
impl<'a, REG> SwapModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Нет перестановки"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(SwapMode::None)
    }
    #[doc = "Перестановка по полуслову"]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut crate::W<REG> {
        self.variant(SwapMode::HalfWord)
    }
    #[doc = "Перестановки по байтам"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(SwapMode::Byte)
    }
    #[doc = "Перестановка по битам"]
    #[inline(always)]
    pub fn bit_(self) -> &'a mut crate::W<REG> {
        self.variant(SwapMode::Bit)
    }
}
#[doc = "Выбор порядка загрузки/выгрузки\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrderMode {
    #[doc = "0: От младшего слова к старшему"]
    Lsw = 0,
    #[doc = "1: От старшего слова к младшему"]
    Msw = 1,
}
impl From<OrderMode> for bool {
    #[inline(always)]
    fn from(variant: OrderMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ORDER_MODE` reader - Выбор порядка загрузки/выгрузки"]
pub type OrderModeR = crate::BitReader<OrderMode>;
impl OrderModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OrderMode {
        match self.bits {
            false => OrderMode::Lsw,
            true => OrderMode::Msw,
        }
    }
    #[doc = "От младшего слова к старшему"]
    #[inline(always)]
    pub fn is_lsw(&self) -> bool {
        *self == OrderMode::Lsw
    }
    #[doc = "От старшего слова к младшему"]
    #[inline(always)]
    pub fn is_msw(&self) -> bool {
        *self == OrderMode::Msw
    }
}
#[doc = "Field `ORDER_MODE` writer - Выбор порядка загрузки/выгрузки"]
pub type OrderModeW<'a, REG> = crate::BitWriter<'a, REG, OrderMode>;
impl<'a, REG> OrderModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "От младшего слова к старшему"]
    #[inline(always)]
    pub fn lsw(self) -> &'a mut crate::W<REG> {
        self.variant(OrderMode::Lsw)
    }
    #[doc = "От старшего слова к младшему"]
    #[inline(always)]
    pub fn msw(self) -> &'a mut crate::W<REG> {
        self.variant(OrderMode::Msw)
    }
}
#[doc = "Field `C_RESET` reader - Сброс счётчиков загружаемых/выгружаемых данных"]
pub type CResetR = crate::BitReader;
#[doc = "Field `C_RESET` writer - Сброс счётчиков загружаемых/выгружаемых данных"]
pub type CResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Состояние доступности модуля\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ready {
    #[doc = "0: Занято"]
    Busy = 0,
    #[doc = "1: Доступно"]
    Available = 1,
}
impl From<Ready> for bool {
    #[inline(always)]
    fn from(variant: Ready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` reader - Состояние доступности модуля"]
pub type ReadyR = crate::BitReader<Ready>;
impl ReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ready {
        match self.bits {
            false => Ready::Busy,
            true => Ready::Available,
        }
    }
    #[doc = "Занято"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Ready::Busy
    }
    #[doc = "Доступно"]
    #[inline(always)]
    pub fn is_available(&self) -> bool {
        *self == Ready::Available
    }
}
#[doc = "Буфер данных доступен для записи\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteStatus {
    #[doc = "0: Недоступно"]
    Unavailable = 0,
    #[doc = "1: Доступно"]
    Available = 1,
}
impl From<WriteStatus> for bool {
    #[inline(always)]
    fn from(variant: WriteStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITE_STATUS` reader - Буфер данных доступен для записи"]
pub type WriteStatusR = crate::BitReader<WriteStatus>;
impl WriteStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WriteStatus {
        match self.bits {
            false => WriteStatus::Unavailable,
            true => WriteStatus::Available,
        }
    }
    #[doc = "Недоступно"]
    #[inline(always)]
    pub fn is_unavailable(&self) -> bool {
        *self == WriteStatus::Unavailable
    }
    #[doc = "Доступно"]
    #[inline(always)]
    pub fn is_available(&self) -> bool {
        *self == WriteStatus::Available
    }
}
#[doc = "Буфер данных доступен для чтения\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadStatus {
    #[doc = "0: Недоступно"]
    Unavailable = 0,
    #[doc = "1: Доступно"]
    Available = 1,
}
impl From<ReadStatus> for bool {
    #[inline(always)]
    fn from(variant: ReadStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ_STATUS` reader - Буфер данных доступен для чтения"]
pub type ReadStatusR = crate::BitReader<ReadStatus>;
impl ReadStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ReadStatus {
        match self.bits {
            false => ReadStatus::Unavailable,
            true => ReadStatus::Available,
        }
    }
    #[doc = "Недоступно"]
    #[inline(always)]
    pub fn is_unavailable(&self) -> bool {
        *self == ReadStatus::Unavailable
    }
    #[doc = "Доступно"]
    #[inline(always)]
    pub fn is_available(&self) -> bool {
        *self == ReadStatus::Available
    }
}
impl R {
    #[doc = "Bit 0 - Задаёт режим хода вычислительного ядра"]
    #[inline(always)]
    pub fn decode(&self) -> DecodeR {
        DecodeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Регистр выбора алгоритма шифрования"]
    #[inline(always)]
    pub fn core_sel(&self) -> CoreSelR {
        CoreSelR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - Регистр выбора режима шифрования"]
    #[inline(always)]
    pub fn mode_sel(&self) -> ModeSelR {
        ModeSelR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Регистр выбора режима перестановки слова"]
    #[inline(always)]
    pub fn swap_mode(&self) -> SwapModeR {
        SwapModeR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Выбор порядка загрузки/выгрузки"]
    #[inline(always)]
    pub fn order_mode(&self) -> OrderModeR {
        OrderModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Сброс счётчиков загружаемых/выгружаемых данных"]
    #[inline(always)]
    pub fn c_reset(&self) -> CResetR {
        CResetR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Состояние доступности модуля"]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Буфер данных доступен для записи"]
    #[inline(always)]
    pub fn write_status(&self) -> WriteStatusR {
        WriteStatusR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Буфер данных доступен для чтения"]
    #[inline(always)]
    pub fn read_status(&self) -> ReadStatusR {
        ReadStatusR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Задаёт режим хода вычислительного ядра"]
    #[inline(always)]
    pub fn decode(&mut self) -> DecodeW<ConfigSpec> {
        DecodeW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Регистр выбора алгоритма шифрования"]
    #[inline(always)]
    pub fn core_sel(&mut self) -> CoreSelW<ConfigSpec> {
        CoreSelW::new(self, 1)
    }
    #[doc = "Bits 3:4 - Регистр выбора режима шифрования"]
    #[inline(always)]
    pub fn mode_sel(&mut self) -> ModeSelW<ConfigSpec> {
        ModeSelW::new(self, 3)
    }
    #[doc = "Bits 5:6 - Регистр выбора режима перестановки слова"]
    #[inline(always)]
    pub fn swap_mode(&mut self) -> SwapModeW<ConfigSpec> {
        SwapModeW::new(self, 5)
    }
    #[doc = "Bit 7 - Выбор порядка загрузки/выгрузки"]
    #[inline(always)]
    pub fn order_mode(&mut self) -> OrderModeW<ConfigSpec> {
        OrderModeW::new(self, 7)
    }
    #[doc = "Bit 8 - Сброс счётчиков загружаемых/выгружаемых данных"]
    #[inline(always)]
    pub fn c_reset(&mut self) -> CResetW<ConfigSpec> {
        CResetW::new(self, 8)
    }
}
#[doc = "Регистр конфигурации\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0;
}
