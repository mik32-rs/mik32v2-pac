#[doc = "Register `CH4_CFG` reader"]
pub type R = crate::R<Ch4CfgSpec>;
#[doc = "Register `CH4_CFG` writer"]
pub type W = crate::W<Ch4CfgSpec>;
#[doc = "Разрешение работы канала\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Принудительная остановка"]
    Stop = 0,
    #[doc = "1: Инициализация работы канала"]
    Start = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Разрешение работы канала"]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::Stop,
            true => Enable::Start,
        }
    }
    #[doc = "Принудительная остановка"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Enable::Stop
    }
    #[doc = "Инициализация работы канала"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Enable::Start
    }
}
#[doc = "Field `ENABLE` writer - Разрешение работы канала"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Принудительная остановка"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Stop)
    }
    #[doc = "Инициализация работы канала"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Start)
    }
}
#[doc = "Приоритет канала\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prior {
    #[doc = "0: Принудительная остановка"]
    Low = 0,
    #[doc = "1: Инициализация работы канала"]
    Medium = 1,
    #[doc = "2: Инициализация работы канала"]
    High = 2,
    #[doc = "3: Инициализация работы канала"]
    VeryHigh = 3,
}
impl From<Prior> for u8 {
    #[inline(always)]
    fn from(variant: Prior) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prior {
    type Ux = u8;
}
impl crate::IsEnum for Prior {}
#[doc = "Field `PRIOR` reader - Приоритет канала"]
pub type PriorR = crate::FieldReader<Prior>;
impl PriorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prior {
        match self.bits {
            0 => Prior::Low,
            1 => Prior::Medium,
            2 => Prior::High,
            3 => Prior::VeryHigh,
            _ => unreachable!(),
        }
    }
    #[doc = "Принудительная остановка"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Prior::Low
    }
    #[doc = "Инициализация работы канала"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == Prior::Medium
    }
    #[doc = "Инициализация работы канала"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Prior::High
    }
    #[doc = "Инициализация работы канала"]
    #[inline(always)]
    pub fn is_very_high(&self) -> bool {
        *self == Prior::VeryHigh
    }
}
#[doc = "Field `PRIOR` writer - Приоритет канала"]
pub type PriorW<'a, REG> = crate::FieldWriter<'a, REG, 2, Prior, crate::Safe>;
impl<'a, REG> PriorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Принудительная остановка"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Prior::Low)
    }
    #[doc = "Инициализация работы канала"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut crate::W<REG> {
        self.variant(Prior::Medium)
    }
    #[doc = "Инициализация работы канала"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Prior::High)
    }
    #[doc = "Инициализация работы канала"]
    #[inline(always)]
    pub fn very_high(self) -> &'a mut crate::W<REG> {
        self.variant(Prior::VeryHigh)
    }
}
#[doc = "Режим адреса источника\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadMode {
    #[doc = "0: Принудительная остановка"]
    Periphery = 0,
    #[doc = "1: Инициализация работы канала"]
    Memory = 1,
}
impl From<ReadMode> for bool {
    #[inline(always)]
    fn from(variant: ReadMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ_MODE` reader - Режим адреса источника"]
pub type ReadModeR = crate::BitReader<ReadMode>;
impl ReadModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ReadMode {
        match self.bits {
            false => ReadMode::Periphery,
            true => ReadMode::Memory,
        }
    }
    #[doc = "Принудительная остановка"]
    #[inline(always)]
    pub fn is_periphery(&self) -> bool {
        *self == ReadMode::Periphery
    }
    #[doc = "Инициализация работы канала"]
    #[inline(always)]
    pub fn is_memory(&self) -> bool {
        *self == ReadMode::Memory
    }
}
#[doc = "Field `READ_MODE` writer - Режим адреса источника"]
pub type ReadModeW<'a, REG> = crate::BitWriter<'a, REG, ReadMode>;
impl<'a, REG> ReadModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Принудительная остановка"]
    #[inline(always)]
    pub fn periphery(self) -> &'a mut crate::W<REG> {
        self.variant(ReadMode::Periphery)
    }
    #[doc = "Инициализация работы канала"]
    #[inline(always)]
    pub fn memory(self) -> &'a mut crate::W<REG> {
        self.variant(ReadMode::Memory)
    }
}
#[doc = "Режим адреса назначения\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteMode {
    #[doc = "0: Принудительная остановка"]
    Periphery = 0,
    #[doc = "1: Инициализация работы канала"]
    Memory = 1,
}
impl From<WriteMode> for bool {
    #[inline(always)]
    fn from(variant: WriteMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITE_MODE` reader - Режим адреса назначения"]
pub type WriteModeR = crate::BitReader<WriteMode>;
impl WriteModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WriteMode {
        match self.bits {
            false => WriteMode::Periphery,
            true => WriteMode::Memory,
        }
    }
    #[doc = "Принудительная остановка"]
    #[inline(always)]
    pub fn is_periphery(&self) -> bool {
        *self == WriteMode::Periphery
    }
    #[doc = "Инициализация работы канала"]
    #[inline(always)]
    pub fn is_memory(&self) -> bool {
        *self == WriteMode::Memory
    }
}
#[doc = "Field `WRITE_MODE` writer - Режим адреса назначения"]
pub type WriteModeW<'a, REG> = crate::BitWriter<'a, REG, WriteMode>;
impl<'a, REG> WriteModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Принудительная остановка"]
    #[inline(always)]
    pub fn periphery(self) -> &'a mut crate::W<REG> {
        self.variant(WriteMode::Periphery)
    }
    #[doc = "Инициализация работы канала"]
    #[inline(always)]
    pub fn memory(self) -> &'a mut crate::W<REG> {
        self.variant(WriteMode::Memory)
    }
}
#[doc = "Инкремент адреса источника\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadIncrement {
    #[doc = "0: Нет инкремента"]
    NoIncrement = 0,
    #[doc = "1: Есть инкремент"]
    Increment = 1,
}
impl From<ReadIncrement> for bool {
    #[inline(always)]
    fn from(variant: ReadIncrement) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ_INCREMENT` reader - Инкремент адреса источника"]
pub type ReadIncrementR = crate::BitReader<ReadIncrement>;
impl ReadIncrementR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ReadIncrement {
        match self.bits {
            false => ReadIncrement::NoIncrement,
            true => ReadIncrement::Increment,
        }
    }
    #[doc = "Нет инкремента"]
    #[inline(always)]
    pub fn is_no_increment(&self) -> bool {
        *self == ReadIncrement::NoIncrement
    }
    #[doc = "Есть инкремент"]
    #[inline(always)]
    pub fn is_increment(&self) -> bool {
        *self == ReadIncrement::Increment
    }
}
#[doc = "Field `READ_INCREMENT` writer - Инкремент адреса источника"]
pub type ReadIncrementW<'a, REG> = crate::BitWriter<'a, REG, ReadIncrement>;
impl<'a, REG> ReadIncrementW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Нет инкремента"]
    #[inline(always)]
    pub fn no_increment(self) -> &'a mut crate::W<REG> {
        self.variant(ReadIncrement::NoIncrement)
    }
    #[doc = "Есть инкремент"]
    #[inline(always)]
    pub fn increment(self) -> &'a mut crate::W<REG> {
        self.variant(ReadIncrement::Increment)
    }
}
#[doc = "Инкремент адреса назначения\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteIncrement {
    #[doc = "0: Нет инкремента"]
    NoIncrement = 0,
    #[doc = "1: Есть инкремент"]
    Increment = 1,
}
impl From<WriteIncrement> for bool {
    #[inline(always)]
    fn from(variant: WriteIncrement) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITE_INCREMENT` reader - Инкремент адреса назначения"]
pub type WriteIncrementR = crate::BitReader<WriteIncrement>;
impl WriteIncrementR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WriteIncrement {
        match self.bits {
            false => WriteIncrement::NoIncrement,
            true => WriteIncrement::Increment,
        }
    }
    #[doc = "Нет инкремента"]
    #[inline(always)]
    pub fn is_no_increment(&self) -> bool {
        *self == WriteIncrement::NoIncrement
    }
    #[doc = "Есть инкремент"]
    #[inline(always)]
    pub fn is_increment(&self) -> bool {
        *self == WriteIncrement::Increment
    }
}
#[doc = "Field `WRITE_INCREMENT` writer - Инкремент адреса назначения"]
pub type WriteIncrementW<'a, REG> = crate::BitWriter<'a, REG, WriteIncrement>;
impl<'a, REG> WriteIncrementW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Нет инкремента"]
    #[inline(always)]
    pub fn no_increment(self) -> &'a mut crate::W<REG> {
        self.variant(WriteIncrement::NoIncrement)
    }
    #[doc = "Есть инкремент"]
    #[inline(always)]
    pub fn increment(self) -> &'a mut crate::W<REG> {
        self.variant(WriteIncrement::Increment)
    }
}
#[doc = "Разрядность адреса источника. Должно быть кратно data_len\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ReadSize {
    #[doc = "0: Байт"]
    Byte = 0,
    #[doc = "1: Полуслово"]
    _2byte = 1,
    #[doc = "2: Слово"]
    _4byte = 2,
}
impl From<ReadSize> for u8 {
    #[inline(always)]
    fn from(variant: ReadSize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ReadSize {
    type Ux = u8;
}
impl crate::IsEnum for ReadSize {}
#[doc = "Field `READ_SIZE` reader - Разрядность адреса источника. Должно быть кратно data_len"]
pub type ReadSizeR = crate::FieldReader<ReadSize>;
impl ReadSizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ReadSize> {
        match self.bits {
            0 => Some(ReadSize::Byte),
            1 => Some(ReadSize::_2byte),
            2 => Some(ReadSize::_4byte),
            _ => None,
        }
    }
    #[doc = "Байт"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == ReadSize::Byte
    }
    #[doc = "Полуслово"]
    #[inline(always)]
    pub fn is_2byte(&self) -> bool {
        *self == ReadSize::_2byte
    }
    #[doc = "Слово"]
    #[inline(always)]
    pub fn is_4byte(&self) -> bool {
        *self == ReadSize::_4byte
    }
}
#[doc = "Field `READ_SIZE` writer - Разрядность адреса источника. Должно быть кратно data_len"]
pub type ReadSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, ReadSize>;
impl<'a, REG> ReadSizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Байт"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(ReadSize::Byte)
    }
    #[doc = "Полуслово"]
    #[inline(always)]
    pub fn _2byte(self) -> &'a mut crate::W<REG> {
        self.variant(ReadSize::_2byte)
    }
    #[doc = "Слово"]
    #[inline(always)]
    pub fn _4byte(self) -> &'a mut crate::W<REG> {
        self.variant(ReadSize::_4byte)
    }
}
#[doc = "Разрядность адреса назначения. Должно быть кратно LEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WriteSize {
    #[doc = "0: Байт"]
    Byte = 0,
    #[doc = "1: Полуслово"]
    _2byte = 1,
    #[doc = "2: Слово"]
    _4byte = 2,
}
impl From<WriteSize> for u8 {
    #[inline(always)]
    fn from(variant: WriteSize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WriteSize {
    type Ux = u8;
}
impl crate::IsEnum for WriteSize {}
#[doc = "Field `WRITE_SIZE` reader - Разрядность адреса назначения. Должно быть кратно LEN"]
pub type WriteSizeR = crate::FieldReader<WriteSize>;
impl WriteSizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WriteSize> {
        match self.bits {
            0 => Some(WriteSize::Byte),
            1 => Some(WriteSize::_2byte),
            2 => Some(WriteSize::_4byte),
            _ => None,
        }
    }
    #[doc = "Байт"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == WriteSize::Byte
    }
    #[doc = "Полуслово"]
    #[inline(always)]
    pub fn is_2byte(&self) -> bool {
        *self == WriteSize::_2byte
    }
    #[doc = "Слово"]
    #[inline(always)]
    pub fn is_4byte(&self) -> bool {
        *self == WriteSize::_4byte
    }
}
#[doc = "Field `WRITE_SIZE` writer - Разрядность адреса назначения. Должно быть кратно LEN"]
pub type WriteSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, WriteSize>;
impl<'a, REG> WriteSizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Байт"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(WriteSize::Byte)
    }
    #[doc = "Полуслово"]
    #[inline(always)]
    pub fn _2byte(self) -> &'a mut crate::W<REG> {
        self.variant(WriteSize::_2byte)
    }
    #[doc = "Слово"]
    #[inline(always)]
    pub fn _4byte(self) -> &'a mut crate::W<REG> {
        self.variant(WriteSize::_4byte)
    }
}
#[doc = "Field `READ_BURST_SIZE` reader - Количество байт в пакете. Определяется как 2^read_block_size. Должно быть кратно read_size."]
pub type ReadBurstSizeR = crate::FieldReader;
#[doc = "Field `READ_BURST_SIZE` writer - Количество байт в пакете. Определяется как 2^read_block_size. Должно быть кратно read_size."]
pub type ReadBurstSizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WRITE_BURST_SIZE` reader - Количество байт в пакете. Определяется как 2^write_block_size. Должно быть кратно write_size."]
pub type WriteBurstSizeR = crate::FieldReader;
#[doc = "Field `WRITE_BURST_SIZE` writer - Количество байт в пакете. Определяется как 2^write_block_size. Должно быть кратно write_size."]
pub type WriteBurstSizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Выбор периферийной линии источника\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ReadRequest {
    #[doc = "0: Линия запросов от USART_0"]
    Usart0 = 0,
    #[doc = "1: Линия запросов от USART_1"]
    Usart1 = 1,
    #[doc = "2: Линия запросов от крипто-блока"]
    Crypto = 2,
    #[doc = "3: Линия запросов от SPI_0"]
    Spi0 = 3,
    #[doc = "4: Линия запросов от SPI_1"]
    Spi1 = 4,
    #[doc = "5: Линия запросов от I2C_0"]
    I2c0 = 5,
    #[doc = "6: Линия запросов от I2C_1"]
    I2c1 = 6,
    #[doc = "7: Линия запросов от SPIFI"]
    Spifi = 7,
    #[doc = "8: Линия запросов от Timer32_1"]
    Timer32_1 = 8,
    #[doc = "9: Линия запросов от Timer32_2"]
    Timer32_2 = 9,
    #[doc = "10: Линия запросов от DAC0"]
    Dac0 = 10,
    #[doc = "11: Линия запросов от DAC1"]
    Dac1 = 11,
    #[doc = "12: Линия запросов от Timer32_0"]
    Timer32_0 = 12,
}
impl From<ReadRequest> for u8 {
    #[inline(always)]
    fn from(variant: ReadRequest) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ReadRequest {
    type Ux = u8;
}
impl crate::IsEnum for ReadRequest {}
#[doc = "Field `READ_REQUEST` reader - Выбор периферийной линии источника"]
pub type ReadRequestR = crate::FieldReader<ReadRequest>;
impl ReadRequestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ReadRequest> {
        match self.bits {
            0 => Some(ReadRequest::Usart0),
            1 => Some(ReadRequest::Usart1),
            2 => Some(ReadRequest::Crypto),
            3 => Some(ReadRequest::Spi0),
            4 => Some(ReadRequest::Spi1),
            5 => Some(ReadRequest::I2c0),
            6 => Some(ReadRequest::I2c1),
            7 => Some(ReadRequest::Spifi),
            8 => Some(ReadRequest::Timer32_1),
            9 => Some(ReadRequest::Timer32_2),
            10 => Some(ReadRequest::Dac0),
            11 => Some(ReadRequest::Dac1),
            12 => Some(ReadRequest::Timer32_0),
            _ => None,
        }
    }
    #[doc = "Линия запросов от USART_0"]
    #[inline(always)]
    pub fn is_usart_0(&self) -> bool {
        *self == ReadRequest::Usart0
    }
    #[doc = "Линия запросов от USART_1"]
    #[inline(always)]
    pub fn is_usart_1(&self) -> bool {
        *self == ReadRequest::Usart1
    }
    #[doc = "Линия запросов от крипто-блока"]
    #[inline(always)]
    pub fn is_crypto(&self) -> bool {
        *self == ReadRequest::Crypto
    }
    #[doc = "Линия запросов от SPI_0"]
    #[inline(always)]
    pub fn is_spi_0(&self) -> bool {
        *self == ReadRequest::Spi0
    }
    #[doc = "Линия запросов от SPI_1"]
    #[inline(always)]
    pub fn is_spi_1(&self) -> bool {
        *self == ReadRequest::Spi1
    }
    #[doc = "Линия запросов от I2C_0"]
    #[inline(always)]
    pub fn is_i2c_0(&self) -> bool {
        *self == ReadRequest::I2c0
    }
    #[doc = "Линия запросов от I2C_1"]
    #[inline(always)]
    pub fn is_i2c_1(&self) -> bool {
        *self == ReadRequest::I2c1
    }
    #[doc = "Линия запросов от SPIFI"]
    #[inline(always)]
    pub fn is_spifi(&self) -> bool {
        *self == ReadRequest::Spifi
    }
    #[doc = "Линия запросов от Timer32_1"]
    #[inline(always)]
    pub fn is_timer32_1(&self) -> bool {
        *self == ReadRequest::Timer32_1
    }
    #[doc = "Линия запросов от Timer32_2"]
    #[inline(always)]
    pub fn is_timer32_2(&self) -> bool {
        *self == ReadRequest::Timer32_2
    }
    #[doc = "Линия запросов от DAC0"]
    #[inline(always)]
    pub fn is_dac0(&self) -> bool {
        *self == ReadRequest::Dac0
    }
    #[doc = "Линия запросов от DAC1"]
    #[inline(always)]
    pub fn is_dac1(&self) -> bool {
        *self == ReadRequest::Dac1
    }
    #[doc = "Линия запросов от Timer32_0"]
    #[inline(always)]
    pub fn is_timer32_0(&self) -> bool {
        *self == ReadRequest::Timer32_0
    }
}
#[doc = "Field `READ_REQUEST` writer - Выбор периферийной линии источника"]
pub type ReadRequestW<'a, REG> = crate::FieldWriter<'a, REG, 4, ReadRequest>;
impl<'a, REG> ReadRequestW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Линия запросов от USART_0"]
    #[inline(always)]
    pub fn usart_0(self) -> &'a mut crate::W<REG> {
        self.variant(ReadRequest::Usart0)
    }
    #[doc = "Линия запросов от USART_1"]
    #[inline(always)]
    pub fn usart_1(self) -> &'a mut crate::W<REG> {
        self.variant(ReadRequest::Usart1)
    }
    #[doc = "Линия запросов от крипто-блока"]
    #[inline(always)]
    pub fn crypto(self) -> &'a mut crate::W<REG> {
        self.variant(ReadRequest::Crypto)
    }
    #[doc = "Линия запросов от SPI_0"]
    #[inline(always)]
    pub fn spi_0(self) -> &'a mut crate::W<REG> {
        self.variant(ReadRequest::Spi0)
    }
    #[doc = "Линия запросов от SPI_1"]
    #[inline(always)]
    pub fn spi_1(self) -> &'a mut crate::W<REG> {
        self.variant(ReadRequest::Spi1)
    }
    #[doc = "Линия запросов от I2C_0"]
    #[inline(always)]
    pub fn i2c_0(self) -> &'a mut crate::W<REG> {
        self.variant(ReadRequest::I2c0)
    }
    #[doc = "Линия запросов от I2C_1"]
    #[inline(always)]
    pub fn i2c_1(self) -> &'a mut crate::W<REG> {
        self.variant(ReadRequest::I2c1)
    }
    #[doc = "Линия запросов от SPIFI"]
    #[inline(always)]
    pub fn spifi(self) -> &'a mut crate::W<REG> {
        self.variant(ReadRequest::Spifi)
    }
    #[doc = "Линия запросов от Timer32_1"]
    #[inline(always)]
    pub fn timer32_1(self) -> &'a mut crate::W<REG> {
        self.variant(ReadRequest::Timer32_1)
    }
    #[doc = "Линия запросов от Timer32_2"]
    #[inline(always)]
    pub fn timer32_2(self) -> &'a mut crate::W<REG> {
        self.variant(ReadRequest::Timer32_2)
    }
    #[doc = "Линия запросов от DAC0"]
    #[inline(always)]
    pub fn dac0(self) -> &'a mut crate::W<REG> {
        self.variant(ReadRequest::Dac0)
    }
    #[doc = "Линия запросов от DAC1"]
    #[inline(always)]
    pub fn dac1(self) -> &'a mut crate::W<REG> {
        self.variant(ReadRequest::Dac1)
    }
    #[doc = "Линия запросов от Timer32_0"]
    #[inline(always)]
    pub fn timer32_0(self) -> &'a mut crate::W<REG> {
        self.variant(ReadRequest::Timer32_0)
    }
}
#[doc = "Выбор периферийной линии назначения\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WriteRequest {
    #[doc = "0: Линия запросов от USART_0"]
    Usart0 = 0,
    #[doc = "1: Линия запросов от USART_1"]
    Usart1 = 1,
    #[doc = "2: Линия запросов от крипто-блока"]
    Crypto = 2,
    #[doc = "3: Линия запросов от SPI_0"]
    Spi0 = 3,
    #[doc = "4: Линия запросов от SPI_1"]
    Spi1 = 4,
    #[doc = "5: Линия запросов от I2C_0"]
    I2c0 = 5,
    #[doc = "6: Линия запросов от I2C_1"]
    I2c1 = 6,
    #[doc = "7: Линия запросов от SPIFI"]
    Spifi = 7,
    #[doc = "8: Линия запросов от Timer32_1"]
    Timer32_1 = 8,
    #[doc = "9: Линия запросов от Timer32_2"]
    Timer32_2 = 9,
    #[doc = "10: Линия запросов от DAC0"]
    Dac0 = 10,
    #[doc = "11: Линия запросов от DAC1"]
    Dac1 = 11,
    #[doc = "12: Линия запросов от Timer32_0"]
    Timer32_0 = 12,
}
impl From<WriteRequest> for u8 {
    #[inline(always)]
    fn from(variant: WriteRequest) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WriteRequest {
    type Ux = u8;
}
impl crate::IsEnum for WriteRequest {}
#[doc = "Field `WRITE_REQUEST` reader - Выбор периферийной линии назначения"]
pub type WriteRequestR = crate::FieldReader<WriteRequest>;
impl WriteRequestR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WriteRequest> {
        match self.bits {
            0 => Some(WriteRequest::Usart0),
            1 => Some(WriteRequest::Usart1),
            2 => Some(WriteRequest::Crypto),
            3 => Some(WriteRequest::Spi0),
            4 => Some(WriteRequest::Spi1),
            5 => Some(WriteRequest::I2c0),
            6 => Some(WriteRequest::I2c1),
            7 => Some(WriteRequest::Spifi),
            8 => Some(WriteRequest::Timer32_1),
            9 => Some(WriteRequest::Timer32_2),
            10 => Some(WriteRequest::Dac0),
            11 => Some(WriteRequest::Dac1),
            12 => Some(WriteRequest::Timer32_0),
            _ => None,
        }
    }
    #[doc = "Линия запросов от USART_0"]
    #[inline(always)]
    pub fn is_usart_0(&self) -> bool {
        *self == WriteRequest::Usart0
    }
    #[doc = "Линия запросов от USART_1"]
    #[inline(always)]
    pub fn is_usart_1(&self) -> bool {
        *self == WriteRequest::Usart1
    }
    #[doc = "Линия запросов от крипто-блока"]
    #[inline(always)]
    pub fn is_crypto(&self) -> bool {
        *self == WriteRequest::Crypto
    }
    #[doc = "Линия запросов от SPI_0"]
    #[inline(always)]
    pub fn is_spi_0(&self) -> bool {
        *self == WriteRequest::Spi0
    }
    #[doc = "Линия запросов от SPI_1"]
    #[inline(always)]
    pub fn is_spi_1(&self) -> bool {
        *self == WriteRequest::Spi1
    }
    #[doc = "Линия запросов от I2C_0"]
    #[inline(always)]
    pub fn is_i2c_0(&self) -> bool {
        *self == WriteRequest::I2c0
    }
    #[doc = "Линия запросов от I2C_1"]
    #[inline(always)]
    pub fn is_i2c_1(&self) -> bool {
        *self == WriteRequest::I2c1
    }
    #[doc = "Линия запросов от SPIFI"]
    #[inline(always)]
    pub fn is_spifi(&self) -> bool {
        *self == WriteRequest::Spifi
    }
    #[doc = "Линия запросов от Timer32_1"]
    #[inline(always)]
    pub fn is_timer32_1(&self) -> bool {
        *self == WriteRequest::Timer32_1
    }
    #[doc = "Линия запросов от Timer32_2"]
    #[inline(always)]
    pub fn is_timer32_2(&self) -> bool {
        *self == WriteRequest::Timer32_2
    }
    #[doc = "Линия запросов от DAC0"]
    #[inline(always)]
    pub fn is_dac0(&self) -> bool {
        *self == WriteRequest::Dac0
    }
    #[doc = "Линия запросов от DAC1"]
    #[inline(always)]
    pub fn is_dac1(&self) -> bool {
        *self == WriteRequest::Dac1
    }
    #[doc = "Линия запросов от Timer32_0"]
    #[inline(always)]
    pub fn is_timer32_0(&self) -> bool {
        *self == WriteRequest::Timer32_0
    }
}
#[doc = "Field `WRITE_REQUEST` writer - Выбор периферийной линии назначения"]
pub type WriteRequestW<'a, REG> = crate::FieldWriter<'a, REG, 4, WriteRequest>;
impl<'a, REG> WriteRequestW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Линия запросов от USART_0"]
    #[inline(always)]
    pub fn usart_0(self) -> &'a mut crate::W<REG> {
        self.variant(WriteRequest::Usart0)
    }
    #[doc = "Линия запросов от USART_1"]
    #[inline(always)]
    pub fn usart_1(self) -> &'a mut crate::W<REG> {
        self.variant(WriteRequest::Usart1)
    }
    #[doc = "Линия запросов от крипто-блока"]
    #[inline(always)]
    pub fn crypto(self) -> &'a mut crate::W<REG> {
        self.variant(WriteRequest::Crypto)
    }
    #[doc = "Линия запросов от SPI_0"]
    #[inline(always)]
    pub fn spi_0(self) -> &'a mut crate::W<REG> {
        self.variant(WriteRequest::Spi0)
    }
    #[doc = "Линия запросов от SPI_1"]
    #[inline(always)]
    pub fn spi_1(self) -> &'a mut crate::W<REG> {
        self.variant(WriteRequest::Spi1)
    }
    #[doc = "Линия запросов от I2C_0"]
    #[inline(always)]
    pub fn i2c_0(self) -> &'a mut crate::W<REG> {
        self.variant(WriteRequest::I2c0)
    }
    #[doc = "Линия запросов от I2C_1"]
    #[inline(always)]
    pub fn i2c_1(self) -> &'a mut crate::W<REG> {
        self.variant(WriteRequest::I2c1)
    }
    #[doc = "Линия запросов от SPIFI"]
    #[inline(always)]
    pub fn spifi(self) -> &'a mut crate::W<REG> {
        self.variant(WriteRequest::Spifi)
    }
    #[doc = "Линия запросов от Timer32_1"]
    #[inline(always)]
    pub fn timer32_1(self) -> &'a mut crate::W<REG> {
        self.variant(WriteRequest::Timer32_1)
    }
    #[doc = "Линия запросов от Timer32_2"]
    #[inline(always)]
    pub fn timer32_2(self) -> &'a mut crate::W<REG> {
        self.variant(WriteRequest::Timer32_2)
    }
    #[doc = "Линия запросов от DAC0"]
    #[inline(always)]
    pub fn dac0(self) -> &'a mut crate::W<REG> {
        self.variant(WriteRequest::Dac0)
    }
    #[doc = "Линия запросов от DAC1"]
    #[inline(always)]
    pub fn dac1(self) -> &'a mut crate::W<REG> {
        self.variant(WriteRequest::Dac1)
    }
    #[doc = "Линия запросов от Timer32_0"]
    #[inline(always)]
    pub fn timer32_0(self) -> &'a mut crate::W<REG> {
        self.variant(WriteRequest::Timer32_0)
    }
}
#[doc = "Разрешение работы логики с откликом для адресата источника\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadAckEn {
    #[doc = "0: Запрещено"]
    Disable = 0,
    #[doc = "1: Разрешено"]
    Enable = 1,
}
impl From<ReadAckEn> for bool {
    #[inline(always)]
    fn from(variant: ReadAckEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ_ACK_EN` reader - Разрешение работы логики с откликом для адресата источника"]
pub type ReadAckEnR = crate::BitReader<ReadAckEn>;
impl ReadAckEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ReadAckEn {
        match self.bits {
            false => ReadAckEn::Disable,
            true => ReadAckEn::Enable,
        }
    }
    #[doc = "Запрещено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ReadAckEn::Disable
    }
    #[doc = "Разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ReadAckEn::Enable
    }
}
#[doc = "Field `READ_ACK_EN` writer - Разрешение работы логики с откликом для адресата источника"]
pub type ReadAckEnW<'a, REG> = crate::BitWriter<'a, REG, ReadAckEn>;
impl<'a, REG> ReadAckEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Запрещено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ReadAckEn::Disable)
    }
    #[doc = "Разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ReadAckEn::Enable)
    }
}
#[doc = "Разрешение работы логики с откликом для адресата назначения\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteAckEn {
    #[doc = "0: Запрещено"]
    Disable = 0,
    #[doc = "1: Разрешено"]
    Enable = 1,
}
impl From<WriteAckEn> for bool {
    #[inline(always)]
    fn from(variant: WriteAckEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRITE_ACK_EN` reader - Разрешение работы логики с откликом для адресата назначения"]
pub type WriteAckEnR = crate::BitReader<WriteAckEn>;
impl WriteAckEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WriteAckEn {
        match self.bits {
            false => WriteAckEn::Disable,
            true => WriteAckEn::Enable,
        }
    }
    #[doc = "Запрещено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WriteAckEn::Disable
    }
    #[doc = "Разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WriteAckEn::Enable
    }
}
#[doc = "Field `WRITE_ACK_EN` writer - Разрешение работы логики с откликом для адресата назначения"]
pub type WriteAckEnW<'a, REG> = crate::BitWriter<'a, REG, WriteAckEn>;
impl<'a, REG> WriteAckEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Запрещено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(WriteAckEn::Disable)
    }
    #[doc = "Разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(WriteAckEn::Enable)
    }
}
#[doc = "Разрешение формирования пре-рывания по завершении работы канала: 0 – прерывание не формируется; 1 – прерывание формируется\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrqEn {
    #[doc = "0: Прерывание не формируется"]
    Disable = 0,
    #[doc = "1: Прерывание формируется"]
    Enable = 1,
}
impl From<IrqEn> for bool {
    #[inline(always)]
    fn from(variant: IrqEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRQ_EN` reader - Разрешение формирования пре-рывания по завершении работы канала: 0 – прерывание не формируется; 1 – прерывание формируется"]
pub type IrqEnR = crate::BitReader<IrqEn>;
impl IrqEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IrqEn {
        match self.bits {
            false => IrqEn::Disable,
            true => IrqEn::Enable,
        }
    }
    #[doc = "Прерывание не формируется"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IrqEn::Disable
    }
    #[doc = "Прерывание формируется"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IrqEn::Enable
    }
}
#[doc = "Field `IRQ_EN` writer - Разрешение формирования пре-рывания по завершении работы канала: 0 – прерывание не формируется; 1 – прерывание формируется"]
pub type IrqEnW<'a, REG> = crate::BitWriter<'a, REG, IrqEn>;
impl<'a, REG> IrqEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание не формируется"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(IrqEn::Disable)
    }
    #[doc = "Прерывание формируется"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(IrqEn::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Разрешение работы канала"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Приоритет канала"]
    #[inline(always)]
    pub fn prior(&self) -> PriorR {
        PriorR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Режим адреса источника"]
    #[inline(always)]
    pub fn read_mode(&self) -> ReadModeR {
        ReadModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Режим адреса назначения"]
    #[inline(always)]
    pub fn write_mode(&self) -> WriteModeR {
        WriteModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Инкремент адреса источника"]
    #[inline(always)]
    pub fn read_increment(&self) -> ReadIncrementR {
        ReadIncrementR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 5 - Инкремент адреса назначения"]
    #[inline(always)]
    pub fn write_increment(&self) -> WriteIncrementR {
        WriteIncrementR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 7:8 - Разрядность адреса источника. Должно быть кратно data_len"]
    #[inline(always)]
    pub fn read_size(&self) -> ReadSizeR {
        ReadSizeR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10 - Разрядность адреса назначения. Должно быть кратно LEN"]
    #[inline(always)]
    pub fn write_size(&self) -> WriteSizeR {
        WriteSizeR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:13 - Количество байт в пакете. Определяется как 2^read_block_size. Должно быть кратно read_size."]
    #[inline(always)]
    pub fn read_burst_size(&self) -> ReadBurstSizeR {
        ReadBurstSizeR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:16 - Количество байт в пакете. Определяется как 2^write_block_size. Должно быть кратно write_size."]
    #[inline(always)]
    pub fn write_burst_size(&self) -> WriteBurstSizeR {
        WriteBurstSizeR::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:20 - Выбор периферийной линии источника"]
    #[inline(always)]
    pub fn read_request(&self) -> ReadRequestR {
        ReadRequestR::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:24 - Выбор периферийной линии назначения"]
    #[inline(always)]
    pub fn write_request(&self) -> WriteRequestR {
        WriteRequestR::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bit 25 - Разрешение работы логики с откликом для адресата источника"]
    #[inline(always)]
    pub fn read_ack_en(&self) -> ReadAckEnR {
        ReadAckEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Разрешение работы логики с откликом для адресата назначения"]
    #[inline(always)]
    pub fn write_ack_en(&self) -> WriteAckEnR {
        WriteAckEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Разрешение формирования пре-рывания по завершении работы канала: 0 – прерывание не формируется; 1 – прерывание формируется"]
    #[inline(always)]
    pub fn irq_en(&self) -> IrqEnR {
        IrqEnR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Разрешение работы канала"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<Ch4CfgSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Приоритет канала"]
    #[inline(always)]
    pub fn prior(&mut self) -> PriorW<Ch4CfgSpec> {
        PriorW::new(self, 1)
    }
    #[doc = "Bit 3 - Режим адреса источника"]
    #[inline(always)]
    pub fn read_mode(&mut self) -> ReadModeW<Ch4CfgSpec> {
        ReadModeW::new(self, 3)
    }
    #[doc = "Bit 4 - Режим адреса назначения"]
    #[inline(always)]
    pub fn write_mode(&mut self) -> WriteModeW<Ch4CfgSpec> {
        WriteModeW::new(self, 4)
    }
    #[doc = "Bit 5 - Инкремент адреса источника"]
    #[inline(always)]
    pub fn read_increment(&mut self) -> ReadIncrementW<Ch4CfgSpec> {
        ReadIncrementW::new(self, 5)
    }
    #[doc = "Bit 5 - Инкремент адреса назначения"]
    #[inline(always)]
    pub fn write_increment(&mut self) -> WriteIncrementW<Ch4CfgSpec> {
        WriteIncrementW::new(self, 5)
    }
    #[doc = "Bits 7:8 - Разрядность адреса источника. Должно быть кратно data_len"]
    #[inline(always)]
    pub fn read_size(&mut self) -> ReadSizeW<Ch4CfgSpec> {
        ReadSizeW::new(self, 7)
    }
    #[doc = "Bits 9:10 - Разрядность адреса назначения. Должно быть кратно LEN"]
    #[inline(always)]
    pub fn write_size(&mut self) -> WriteSizeW<Ch4CfgSpec> {
        WriteSizeW::new(self, 9)
    }
    #[doc = "Bits 11:13 - Количество байт в пакете. Определяется как 2^read_block_size. Должно быть кратно read_size."]
    #[inline(always)]
    pub fn read_burst_size(&mut self) -> ReadBurstSizeW<Ch4CfgSpec> {
        ReadBurstSizeW::new(self, 11)
    }
    #[doc = "Bits 14:16 - Количество байт в пакете. Определяется как 2^write_block_size. Должно быть кратно write_size."]
    #[inline(always)]
    pub fn write_burst_size(&mut self) -> WriteBurstSizeW<Ch4CfgSpec> {
        WriteBurstSizeW::new(self, 14)
    }
    #[doc = "Bits 17:20 - Выбор периферийной линии источника"]
    #[inline(always)]
    pub fn read_request(&mut self) -> ReadRequestW<Ch4CfgSpec> {
        ReadRequestW::new(self, 17)
    }
    #[doc = "Bits 21:24 - Выбор периферийной линии назначения"]
    #[inline(always)]
    pub fn write_request(&mut self) -> WriteRequestW<Ch4CfgSpec> {
        WriteRequestW::new(self, 21)
    }
    #[doc = "Bit 25 - Разрешение работы логики с откликом для адресата источника"]
    #[inline(always)]
    pub fn read_ack_en(&mut self) -> ReadAckEnW<Ch4CfgSpec> {
        ReadAckEnW::new(self, 25)
    }
    #[doc = "Bit 26 - Разрешение работы логики с откликом для адресата назначения"]
    #[inline(always)]
    pub fn write_ack_en(&mut self) -> WriteAckEnW<Ch4CfgSpec> {
        WriteAckEnW::new(self, 26)
    }
    #[doc = "Bit 27 - Разрешение формирования пре-рывания по завершении работы канала: 0 – прерывание не формируется; 1 – прерывание формируется"]
    #[inline(always)]
    pub fn irq_en(&mut self) -> IrqEnW<Ch4CfgSpec> {
        IrqEnW::new(self, 27)
    }
}
#[doc = "Регистр управления и конфигурации канала 4\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch4CfgSpec;
impl crate::RegisterSpec for Ch4CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4_cfg::R`](R) reader structure"]
impl crate::Readable for Ch4CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ch4_cfg::W`](W) writer structure"]
impl crate::Writable for Ch4CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH4_CFG to value 0"]
impl crate::Resettable for Ch4CfgSpec {
    const RESET_VALUE: u32 = 0;
}
