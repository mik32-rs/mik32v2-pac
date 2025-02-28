#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `SADD_10bit` reader - Адрес ведомого (режим «ведущий») в режиме 10-битного адреса (ADD10 = 1). Изменение этих битов при START=1 не допускается."]
pub type Sadd10bitR = crate::FieldReader<u16>;
#[doc = "Field `SADD_10bit` writer - Адрес ведомого (режим «ведущий») в режиме 10-битного адреса (ADD10 = 1). Изменение этих битов при START=1 не допускается."]
pub type Sadd10bitW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SADD_7bit` reader - Адрес ведомого (режим «ведущий») в режиме 7-битного адреса (ADD10 = 0). Изменение этих битов при START=1 не допускается."]
pub type Sadd7bitR = crate::FieldReader;
#[doc = "Field `SADD_7bit` writer - Адрес ведомого (режим «ведущий») в режиме 7-битного адреса (ADD10 = 0). Изменение этих битов при START=1 не допускается."]
pub type Sadd7bitW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Направление передачи (режим «ведущий»). Изменение этого бита при START=1 не допускается.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RdWrn {
    #[doc = "0: Ведущий запрашивает транзакцию записи"]
    Write = 0,
    #[doc = "1: Ведущий запрашивает транзакцию чтения"]
    Read = 1,
}
impl From<RdWrn> for bool {
    #[inline(always)]
    fn from(variant: RdWrn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RD_WRN` reader - Направление передачи (режим «ведущий»). Изменение этого бита при START=1 не допускается."]
pub type RdWrnR = crate::BitReader<RdWrn>;
impl RdWrnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RdWrn {
        match self.bits {
            false => RdWrn::Write,
            true => RdWrn::Read,
        }
    }
    #[doc = "Ведущий запрашивает транзакцию записи"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == RdWrn::Write
    }
    #[doc = "Ведущий запрашивает транзакцию чтения"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == RdWrn::Read
    }
}
#[doc = "Field `RD_WRN` writer - Направление передачи (режим «ведущий»). Изменение этого бита при START=1 не допускается."]
pub type RdWrnW<'a, REG> = crate::BitWriter<'a, REG, RdWrn>;
impl<'a, REG> RdWrnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ведущий запрашивает транзакцию записи"]
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(RdWrn::Write)
    }
    #[doc = "Ведущий запрашивает транзакцию чтения"]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(RdWrn::Read)
    }
}
#[doc = "Режим 10-битного адреса (режим «ведущий»). Изменение этого бита при START=1 не допускается.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Add10 {
    #[doc = "0: Ведущий работает в режиме 7-битного адреса"]
    _7bit = 0,
    #[doc = "1: Ведущий работает в режиме 10-битного адреса"]
    _10bit = 1,
}
impl From<Add10> for bool {
    #[inline(always)]
    fn from(variant: Add10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADD10` reader - Режим 10-битного адреса (режим «ведущий»). Изменение этого бита при START=1 не допускается."]
pub type Add10R = crate::BitReader<Add10>;
impl Add10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Add10 {
        match self.bits {
            false => Add10::_7bit,
            true => Add10::_10bit,
        }
    }
    #[doc = "Ведущий работает в режиме 7-битного адреса"]
    #[inline(always)]
    pub fn is_7bit(&self) -> bool {
        *self == Add10::_7bit
    }
    #[doc = "Ведущий работает в режиме 10-битного адреса"]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == Add10::_10bit
    }
}
#[doc = "Field `ADD10` writer - Режим 10-битного адреса (режим «ведущий»). Изменение этого бита при START=1 не допускается."]
pub type Add10W<'a, REG> = crate::BitWriter<'a, REG, Add10>;
impl<'a, REG> Add10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ведущий работает в режиме 7-битного адреса"]
    #[inline(always)]
    pub fn _7bit(self) -> &'a mut crate::W<REG> {
        self.variant(Add10::_7bit)
    }
    #[doc = "Ведущий работает в режиме 10-битного адреса"]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut crate::W<REG> {
        self.variant(Add10::_10bit)
    }
}
#[doc = "Поддержка 10-битного адреса в режиме «ведущий», чтение Изменение этого бита при START=1 не допускается\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Head10r {
    #[doc = "0: Ведущий отправляет полную последовательность для чтения для 10 битного адреса: Start + 2 байта ад-реса (запись) + ReStart + заголовок 10-битного адреса (чтение)"]
    Complete = 0,
    #[doc = "1: Ведущий отправляет только заголовок 10-битного адреса (чтение)"]
    Header = 1,
}
impl From<Head10r> for bool {
    #[inline(always)]
    fn from(variant: Head10r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HEAD10R` reader - Поддержка 10-битного адреса в режиме «ведущий», чтение Изменение этого бита при START=1 не допускается"]
pub type Head10rR = crate::BitReader<Head10r>;
impl Head10rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Head10r {
        match self.bits {
            false => Head10r::Complete,
            true => Head10r::Header,
        }
    }
    #[doc = "Ведущий отправляет полную последовательность для чтения для 10 битного адреса: Start + 2 байта ад-реса (запись) + ReStart + заголовок 10-битного адреса (чтение)"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == Head10r::Complete
    }
    #[doc = "Ведущий отправляет только заголовок 10-битного адреса (чтение)"]
    #[inline(always)]
    pub fn is_header(&self) -> bool {
        *self == Head10r::Header
    }
}
#[doc = "Field `HEAD10R` writer - Поддержка 10-битного адреса в режиме «ведущий», чтение Изменение этого бита при START=1 не допускается"]
pub type Head10rW<'a, REG> = crate::BitWriter<'a, REG, Head10r>;
impl<'a, REG> Head10rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ведущий отправляет полную последовательность для чтения для 10 битного адреса: Start + 2 байта ад-реса (запись) + ReStart + заголовок 10-битного адреса (чтение)"]
    #[inline(always)]
    pub fn complete(self) -> &'a mut crate::W<REG> {
        self.variant(Head10r::Complete)
    }
    #[doc = "Ведущий отправляет только заголовок 10-битного адреса (чтение)"]
    #[inline(always)]
    pub fn header(self) -> &'a mut crate::W<REG> {
        self.variant(Head10r::Header)
    }
}
#[doc = "Формирование START. Устанавливается программно, сбрасывается аппаратно после отправки адреса, потере арбитража или PE=0. Запись ‘0’ в этот бит не имеет эффекта. В режиме «ведущий» отправка первой части 10-битного адреса повторяется при получении NACK до получения ACK или до очистки бита START записью в бит ADDRCF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start {
    #[doc = "0: START не формируется"]
    _0 = 0,
    #[doc = "1: формирование START после передачи текущего байта"]
    Start = 1,
}
impl From<Start> for bool {
    #[inline(always)]
    fn from(variant: Start) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - Формирование START. Устанавливается программно, сбрасывается аппаратно после отправки адреса, потере арбитража или PE=0. Запись ‘0’ в этот бит не имеет эффекта. В режиме «ведущий» отправка первой части 10-битного адреса повторяется при получении NACK до получения ACK или до очистки бита START записью в бит ADDRCF"]
pub type StartR = crate::BitReader<Start>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Start {
        match self.bits {
            false => Start::_0,
            true => Start::Start,
        }
    }
    #[doc = "START не формируется"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Start::_0
    }
    #[doc = "формирование START после передачи текущего байта"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Start::Start
    }
}
#[doc = "Field `START` writer - Формирование START. Устанавливается программно, сбрасывается аппаратно после отправки адреса, потере арбитража или PE=0. Запись ‘0’ в этот бит не имеет эффекта. В режиме «ведущий» отправка первой части 10-битного адреса повторяется при получении NACK до получения ACK или до очистки бита START записью в бит ADDRCF"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG, Start>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "START не формируется"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Start::_0)
    }
    #[doc = "формирование START после передачи текущего байта"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Start)
    }
}
#[doc = "Формирование STOP в режиме «ведущий». Устанавливается программно, сбрасывается аппаратно по событию STOP на шине или при PE=0. Запись ‘0’ в этот бит не имеет эффекта.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    #[doc = "0: STOP не формируется"]
    _0 = 0,
    #[doc = "1: Формирование STOP после передачи текущего байта"]
    Stop = 1,
}
impl From<Stop> for bool {
    #[inline(always)]
    fn from(variant: Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` reader - Формирование STOP в режиме «ведущий». Устанавливается программно, сбрасывается аппаратно по событию STOP на шине или при PE=0. Запись ‘0’ в этот бит не имеет эффекта."]
pub type StopR = crate::BitReader<Stop>;
impl StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stop {
        match self.bits {
            false => Stop::_0,
            true => Stop::Stop,
        }
    }
    #[doc = "STOP не формируется"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Stop::_0
    }
    #[doc = "Формирование STOP после передачи текущего байта"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Stop::Stop
    }
}
#[doc = "Field `STOP` writer - Формирование STOP в режиме «ведущий». Устанавливается программно, сбрасывается аппаратно по событию STOP на шине или при PE=0. Запись ‘0’ в этот бит не имеет эффекта."]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG, Stop>;
impl<'a, REG> StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "STOP не формируется"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::_0)
    }
    #[doc = "Формирование STOP после передачи текущего байта"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::Stop)
    }
}
#[doc = "Формирование NACK в режиме «ведомый». Устанавливается программно, сбрасывается аппаратно: • после отправки NACK; • по событию STOP на шине; • при получении свое-го адреса ведомого; • PE=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nack {
    #[doc = "0: Отправка ACK после приёма текущего байта"]
    Ask = 0,
    #[doc = "1: Отправка NACK после приёма текущего байта"]
    Nack = 1,
}
impl From<Nack> for bool {
    #[inline(always)]
    fn from(variant: Nack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACK` reader - Формирование NACK в режиме «ведомый». Устанавливается программно, сбрасывается аппаратно: • после отправки NACK; • по событию STOP на шине; • при получении свое-го адреса ведомого; • PE=0."]
pub type NackR = crate::BitReader<Nack>;
impl NackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nack {
        match self.bits {
            false => Nack::Ask,
            true => Nack::Nack,
        }
    }
    #[doc = "Отправка ACK после приёма текущего байта"]
    #[inline(always)]
    pub fn is_ask(&self) -> bool {
        *self == Nack::Ask
    }
    #[doc = "Отправка NACK после приёма текущего байта"]
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == Nack::Nack
    }
}
#[doc = "Field `NACK` writer - Формирование NACK в режиме «ведомый». Устанавливается программно, сбрасывается аппаратно: • после отправки NACK; • по событию STOP на шине; • при получении свое-го адреса ведомого; • PE=0."]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG, Nack>;
impl<'a, REG> NackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Отправка ACK после приёма текущего байта"]
    #[inline(always)]
    pub fn ask(self) -> &'a mut crate::W<REG> {
        self.variant(Nack::Ask)
    }
    #[doc = "Отправка NACK после приёма текущего байта"]
    #[inline(always)]
    pub fn nack(self) -> &'a mut crate::W<REG> {
        self.variant(Nack::Nack)
    }
}
#[doc = "Field `NBYTES` reader - Количество байт для приема / передачи Не имеет значения в режиме «ведомый» при SBC=0 Не допускается изменение при установленном бите START"]
pub type NbytesR = crate::FieldReader;
#[doc = "Field `NBYTES` writer - Количество байт для приема / передачи Не имеет значения в режиме «ведомый» при SBC=0 Не допускается изменение при установленном бите START"]
pub type NbytesW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Управление режимом перезаписи NBYTES. Бит устанавливается и очищается программой.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reload {
    #[doc = "0: Транзакция завершена после пересылки NBYTES байт данных (на шине ожидаются STOP или RESTART)"]
    Disable = 0,
    #[doc = "1: Транзакция не завершена после пересылки NBYTES байт данных (значение NBYTES будет перезаписано)"]
    Enable = 1,
}
impl From<Reload> for bool {
    #[inline(always)]
    fn from(variant: Reload) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RELOAD` reader - Управление режимом перезаписи NBYTES. Бит устанавливается и очищается программой."]
pub type ReloadR = crate::BitReader<Reload>;
impl ReloadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reload {
        match self.bits {
            false => Reload::Disable,
            true => Reload::Enable,
        }
    }
    #[doc = "Транзакция завершена после пересылки NBYTES байт данных (на шине ожидаются STOP или RESTART)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Reload::Disable
    }
    #[doc = "Транзакция не завершена после пересылки NBYTES байт данных (значение NBYTES будет перезаписано)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Reload::Enable
    }
}
#[doc = "Field `RELOAD` writer - Управление режимом перезаписи NBYTES. Бит устанавливается и очищается программой."]
pub type ReloadW<'a, REG> = crate::BitWriter<'a, REG, Reload>;
impl<'a, REG> ReloadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Транзакция завершена после пересылки NBYTES байт данных (на шине ожидаются STOP или RESTART)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Reload::Disable)
    }
    #[doc = "Транзакция не завершена после пересылки NBYTES байт данных (значение NBYTES будет перезаписано)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Reload::Enable)
    }
}
#[doc = "Управление режимом автоматического окончания в режиме «ведущий». Бит устанавливается и очищается программой. Не имеет значения при установленном бите RE-LOAD и в режиме «ведомый»\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autoend {
    #[doc = "0: Режим автоматического окончания отключен"]
    Disable = 0,
    #[doc = "1: Режим автоматического окончания включен"]
    Enable = 1,
}
impl From<Autoend> for bool {
    #[inline(always)]
    fn from(variant: Autoend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOEND` reader - Управление режимом автоматического окончания в режиме «ведущий». Бит устанавливается и очищается программой. Не имеет значения при установленном бите RE-LOAD и в режиме «ведомый»"]
pub type AutoendR = crate::BitReader<Autoend>;
impl AutoendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autoend {
        match self.bits {
            false => Autoend::Disable,
            true => Autoend::Enable,
        }
    }
    #[doc = "Режим автоматического окончания отключен"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Autoend::Disable
    }
    #[doc = "Режим автоматического окончания включен"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Autoend::Enable
    }
}
#[doc = "Field `AUTOEND` writer - Управление режимом автоматического окончания в режиме «ведущий». Бит устанавливается и очищается программой. Не имеет значения при установленном бите RE-LOAD и в режиме «ведомый»"]
pub type AutoendW<'a, REG> = crate::BitWriter<'a, REG, Autoend>;
impl<'a, REG> AutoendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Режим автоматического окончания отключен"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Autoend::Disable)
    }
    #[doc = "Режим автоматического окончания включен"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Autoend::Enable)
    }
}
impl R {
    #[doc = "Bits 0:9 - Адрес ведомого (режим «ведущий») в режиме 10-битного адреса (ADD10 = 1). Изменение этих битов при START=1 не допускается."]
    #[inline(always)]
    pub fn sadd_10bit(&self) -> Sadd10bitR {
        Sadd10bitR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 1:7 - Адрес ведомого (режим «ведущий») в режиме 7-битного адреса (ADD10 = 0). Изменение этих битов при START=1 не допускается."]
    #[inline(always)]
    pub fn sadd_7bit(&self) -> Sadd7bitR {
        Sadd7bitR::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 10 - Направление передачи (режим «ведущий»). Изменение этого бита при START=1 не допускается."]
    #[inline(always)]
    pub fn rd_wrn(&self) -> RdWrnR {
        RdWrnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Режим 10-битного адреса (режим «ведущий»). Изменение этого бита при START=1 не допускается."]
    #[inline(always)]
    pub fn add10(&self) -> Add10R {
        Add10R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Поддержка 10-битного адреса в режиме «ведущий», чтение Изменение этого бита при START=1 не допускается"]
    #[inline(always)]
    pub fn head10r(&self) -> Head10rR {
        Head10rR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Формирование START. Устанавливается программно, сбрасывается аппаратно после отправки адреса, потере арбитража или PE=0. Запись ‘0’ в этот бит не имеет эффекта. В режиме «ведущий» отправка первой части 10-битного адреса повторяется при получении NACK до получения ACK или до очистки бита START записью в бит ADDRCF"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Формирование STOP в режиме «ведущий». Устанавливается программно, сбрасывается аппаратно по событию STOP на шине или при PE=0. Запись ‘0’ в этот бит не имеет эффекта."]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Формирование NACK в режиме «ведомый». Устанавливается программно, сбрасывается аппаратно: • после отправки NACK; • по событию STOP на шине; • при получении свое-го адреса ведомого; • PE=0."]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Количество байт для приема / передачи Не имеет значения в режиме «ведомый» при SBC=0 Не допускается изменение при установленном бите START"]
    #[inline(always)]
    pub fn nbytes(&self) -> NbytesR {
        NbytesR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Управление режимом перезаписи NBYTES. Бит устанавливается и очищается программой."]
    #[inline(always)]
    pub fn reload(&self) -> ReloadR {
        ReloadR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Управление режимом автоматического окончания в режиме «ведущий». Бит устанавливается и очищается программой. Не имеет значения при установленном бите RE-LOAD и в режиме «ведомый»"]
    #[inline(always)]
    pub fn autoend(&self) -> AutoendR {
        AutoendR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Адрес ведомого (режим «ведущий») в режиме 10-битного адреса (ADD10 = 1). Изменение этих битов при START=1 не допускается."]
    #[inline(always)]
    pub fn sadd_10bit(&mut self) -> Sadd10bitW<Cr2Spec> {
        Sadd10bitW::new(self, 0)
    }
    #[doc = "Bits 1:7 - Адрес ведомого (режим «ведущий») в режиме 7-битного адреса (ADD10 = 0). Изменение этих битов при START=1 не допускается."]
    #[inline(always)]
    pub fn sadd_7bit(&mut self) -> Sadd7bitW<Cr2Spec> {
        Sadd7bitW::new(self, 1)
    }
    #[doc = "Bit 10 - Направление передачи (режим «ведущий»). Изменение этого бита при START=1 не допускается."]
    #[inline(always)]
    pub fn rd_wrn(&mut self) -> RdWrnW<Cr2Spec> {
        RdWrnW::new(self, 10)
    }
    #[doc = "Bit 11 - Режим 10-битного адреса (режим «ведущий»). Изменение этого бита при START=1 не допускается."]
    #[inline(always)]
    pub fn add10(&mut self) -> Add10W<Cr2Spec> {
        Add10W::new(self, 11)
    }
    #[doc = "Bit 12 - Поддержка 10-битного адреса в режиме «ведущий», чтение Изменение этого бита при START=1 не допускается"]
    #[inline(always)]
    pub fn head10r(&mut self) -> Head10rW<Cr2Spec> {
        Head10rW::new(self, 12)
    }
    #[doc = "Bit 13 - Формирование START. Устанавливается программно, сбрасывается аппаратно после отправки адреса, потере арбитража или PE=0. Запись ‘0’ в этот бит не имеет эффекта. В режиме «ведущий» отправка первой части 10-битного адреса повторяется при получении NACK до получения ACK или до очистки бита START записью в бит ADDRCF"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<Cr2Spec> {
        StartW::new(self, 13)
    }
    #[doc = "Bit 14 - Формирование STOP в режиме «ведущий». Устанавливается программно, сбрасывается аппаратно по событию STOP на шине или при PE=0. Запись ‘0’ в этот бит не имеет эффекта."]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<Cr2Spec> {
        StopW::new(self, 14)
    }
    #[doc = "Bit 15 - Формирование NACK в режиме «ведомый». Устанавливается программно, сбрасывается аппаратно: • после отправки NACK; • по событию STOP на шине; • при получении свое-го адреса ведомого; • PE=0."]
    #[inline(always)]
    pub fn nack(&mut self) -> NackW<Cr2Spec> {
        NackW::new(self, 15)
    }
    #[doc = "Bits 16:23 - Количество байт для приема / передачи Не имеет значения в режиме «ведомый» при SBC=0 Не допускается изменение при установленном бите START"]
    #[inline(always)]
    pub fn nbytes(&mut self) -> NbytesW<Cr2Spec> {
        NbytesW::new(self, 16)
    }
    #[doc = "Bit 24 - Управление режимом перезаписи NBYTES. Бит устанавливается и очищается программой."]
    #[inline(always)]
    pub fn reload(&mut self) -> ReloadW<Cr2Spec> {
        ReloadW::new(self, 24)
    }
    #[doc = "Bit 25 - Управление режимом автоматического окончания в режиме «ведущий». Бит устанавливается и очищается программой. Не имеет значения при установленном бите RE-LOAD и в режиме «ведомый»"]
    #[inline(always)]
    pub fn autoend(&mut self) -> AutoendW<Cr2Spec> {
        AutoendW::new(self, 25)
    }
}
#[doc = "Регистр управления 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {
    const RESET_VALUE: u32 = 0;
}
