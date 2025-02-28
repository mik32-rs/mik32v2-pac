#[doc = "Register `CMD` reader"]
pub type R = crate::R<CmdSpec>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `DATALEN` reader - В тех случаях, когда бит POLL равен «0», это поле определяет количество байт данных при выполнении команды. Если это поле равно «0», то выполняемая команда не содержит данных"]
pub type DatalenR = crate::FieldReader<u16>;
#[doc = "Field `DATALEN` writer - В тех случаях, когда бит POLL равен «0», это поле определяет количество байт данных при выполнении команды. Если это поле равно «0», то выполняемая команда не содержит данных"]
pub type DatalenW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Бит должен быть установлен при выполнении команды, которая содержит входное поле данных и циклически запрашивает состояние бита входного потока битов из регистра статуса флэш-памяти. Номер проверяемого бита в байте статуса должен быть указан в битах DATALEN\\[2:0\\], а требуемое значение бита – в бите DATALEN\\[3\\]. Как только значения этих битов станут равны, контроллер завершает выполнение текущей команды, деактивируя сигнал SPIFI_CS, и формирует прерывание, если оно разрешено. После завершения процесса поллинга необходимо считать один байт из регистра DATA, так как в нем сохряняется последний принятый статус FLASH.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Poll {
    #[doc = "0: Режим поллинга выключен"]
    Disable = 0,
    #[doc = "1: Режим поллинга включен"]
    Enable = 1,
}
impl From<Poll> for bool {
    #[inline(always)]
    fn from(variant: Poll) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POLL` reader - Бит должен быть установлен при выполнении команды, которая содержит входное поле данных и циклически запрашивает состояние бита входного потока битов из регистра статуса флэш-памяти. Номер проверяемого бита в байте статуса должен быть указан в битах DATALEN\\[2:0\\], а требуемое значение бита – в бите DATALEN\\[3\\]. Как только значения этих битов станут равны, контроллер завершает выполнение текущей команды, деактивируя сигнал SPIFI_CS, и формирует прерывание, если оно разрешено. После завершения процесса поллинга необходимо считать один байт из регистра DATA, так как в нем сохряняется последний принятый статус FLASH."]
pub type PollR = crate::BitReader<Poll>;
impl PollR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Poll {
        match self.bits {
            false => Poll::Disable,
            true => Poll::Enable,
        }
    }
    #[doc = "Режим поллинга выключен"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Poll::Disable
    }
    #[doc = "Режим поллинга включен"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Poll::Enable
    }
}
#[doc = "Field `POLL` writer - Бит должен быть установлен при выполнении команды, которая содержит входное поле данных и циклически запрашивает состояние бита входного потока битов из регистра статуса флэш-памяти. Номер проверяемого бита в байте статуса должен быть указан в битах DATALEN\\[2:0\\], а требуемое значение бита – в бите DATALEN\\[3\\]. Как только значения этих битов станут равны, контроллер завершает выполнение текущей команды, деактивируя сигнал SPIFI_CS, и формирует прерывание, если оно разрешено. После завершения процесса поллинга необходимо считать один байт из регистра DATA, так как в нем сохряняется последний принятый статус FLASH."]
pub type PollW<'a, REG> = crate::BitWriter<'a, REG, Poll>;
impl<'a, REG> PollW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Режим поллинга выключен"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Poll::Disable)
    }
    #[doc = "Режим поллинга включен"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Poll::Enable)
    }
}
#[doc = "Бит направления передачи данных. «0» – чтение из флэш\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout {
    #[doc = "0: Чтение из флэш"]
    Read = 0,
    #[doc = "1: Запись во флэш"]
    Write = 1,
}
impl From<Dout> for bool {
    #[inline(always)]
    fn from(variant: Dout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT` reader - Бит направления передачи данных. «0» – чтение из флэш"]
pub type DoutR = crate::BitReader<Dout>;
impl DoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout {
        match self.bits {
            false => Dout::Read,
            true => Dout::Write,
        }
    }
    #[doc = "Чтение из флэш"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == Dout::Read
    }
    #[doc = "Запись во флэш"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == Dout::Write
    }
}
#[doc = "Field `DOUT` writer - Бит направления передачи данных. «0» – чтение из флэш"]
pub type DoutW<'a, REG> = crate::BitWriter<'a, REG, Dout>;
impl<'a, REG> DoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Чтение из флэш"]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(Dout::Read)
    }
    #[doc = "Запись во флэш"]
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(Dout::Write)
    }
}
#[doc = "Field `INTLEN` reader - Количество байт промежуточных данных, которые хранятся в регистре IDATA. Если количество байт больше, чем может хранить регистр IDATA, то остаток добивается нулевыми значениями"]
pub type IntlenR = crate::FieldReader;
#[doc = "Field `INTLEN` writer - Количество байт промежуточных данных, которые хранятся в регистре IDATA. Если количество байт больше, чем может хранить регистр IDATA, то остаток добивается нулевыми значениями"]
pub type IntlenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Формат вывода полей команды\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fieldform {
    #[doc = "0: Все поля выводятся в последовательном режиме"]
    AllSerial = 0,
    #[doc = "1: Данные выводятся в четырех или двух битовом режиме, а остальные поля в последовательном режиме"]
    DataParallel = 1,
    #[doc = "2: Код операции выводится в последовательном режиме, а остальные в четырех или двух битовом"]
    OpcodeSerial = 2,
    #[doc = "3: Все поля в четырех или двух битовом режиме"]
    AllParallel = 3,
}
impl From<Fieldform> for u8 {
    #[inline(always)]
    fn from(variant: Fieldform) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fieldform {
    type Ux = u8;
}
impl crate::IsEnum for Fieldform {}
#[doc = "Field `FIELDFORM` reader - Формат вывода полей команды"]
pub type FieldformR = crate::FieldReader<Fieldform>;
impl FieldformR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fieldform {
        match self.bits {
            0 => Fieldform::AllSerial,
            1 => Fieldform::DataParallel,
            2 => Fieldform::OpcodeSerial,
            3 => Fieldform::AllParallel,
            _ => unreachable!(),
        }
    }
    #[doc = "Все поля выводятся в последовательном режиме"]
    #[inline(always)]
    pub fn is_all_serial(&self) -> bool {
        *self == Fieldform::AllSerial
    }
    #[doc = "Данные выводятся в четырех или двух битовом режиме, а остальные поля в последовательном режиме"]
    #[inline(always)]
    pub fn is_data_parallel(&self) -> bool {
        *self == Fieldform::DataParallel
    }
    #[doc = "Код операции выводится в последовательном режиме, а остальные в четырех или двух битовом"]
    #[inline(always)]
    pub fn is_opcode_serial(&self) -> bool {
        *self == Fieldform::OpcodeSerial
    }
    #[doc = "Все поля в четырех или двух битовом режиме"]
    #[inline(always)]
    pub fn is_all_parallel(&self) -> bool {
        *self == Fieldform::AllParallel
    }
}
#[doc = "Field `FIELDFORM` writer - Формат вывода полей команды"]
pub type FieldformW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fieldform, crate::Safe>;
impl<'a, REG> FieldformW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Все поля выводятся в последовательном режиме"]
    #[inline(always)]
    pub fn all_serial(self) -> &'a mut crate::W<REG> {
        self.variant(Fieldform::AllSerial)
    }
    #[doc = "Данные выводятся в четырех или двух битовом режиме, а остальные поля в последовательном режиме"]
    #[inline(always)]
    pub fn data_parallel(self) -> &'a mut crate::W<REG> {
        self.variant(Fieldform::DataParallel)
    }
    #[doc = "Код операции выводится в последовательном режиме, а остальные в четырех или двух битовом"]
    #[inline(always)]
    pub fn opcode_serial(self) -> &'a mut crate::W<REG> {
        self.variant(Fieldform::OpcodeSerial)
    }
    #[doc = "Все поля в четырех или двух битовом режиме"]
    #[inline(always)]
    pub fn all_parallel(self) -> &'a mut crate::W<REG> {
        self.variant(Fieldform::AllParallel)
    }
}
#[doc = "Бит управления полями кода операции и адреса команды\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Frameform {
    #[doc = "1: Выдается только код операции, адреса нет"]
    OpcodeNoaddr = 1,
    #[doc = "2: Код операции и младший байт адреса"]
    Opcode1addr = 2,
    #[doc = "3: Код операции и два младших байта адреса"]
    Opcode2addr = 3,
    #[doc = "4: Код операции и три младших байта адреса"]
    Opcode3addr = 4,
    #[doc = "5: Код операции и 4 байта адреса"]
    Opcode4addr = 5,
    #[doc = "6: Нет кода операции, три младших байта адреса"]
    Noopcode3addr = 6,
    #[doc = "7: Нет кода операции, 4 байта адреса"]
    Noopcode4addr = 7,
}
impl From<Frameform> for u8 {
    #[inline(always)]
    fn from(variant: Frameform) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Frameform {
    type Ux = u8;
}
impl crate::IsEnum for Frameform {}
#[doc = "Field `FRAMEFORM` reader - Бит управления полями кода операции и адреса команды"]
pub type FrameformR = crate::FieldReader<Frameform>;
impl FrameformR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Frameform> {
        match self.bits {
            1 => Some(Frameform::OpcodeNoaddr),
            2 => Some(Frameform::Opcode1addr),
            3 => Some(Frameform::Opcode2addr),
            4 => Some(Frameform::Opcode3addr),
            5 => Some(Frameform::Opcode4addr),
            6 => Some(Frameform::Noopcode3addr),
            7 => Some(Frameform::Noopcode4addr),
            _ => None,
        }
    }
    #[doc = "Выдается только код операции, адреса нет"]
    #[inline(always)]
    pub fn is_opcode_noaddr(&self) -> bool {
        *self == Frameform::OpcodeNoaddr
    }
    #[doc = "Код операции и младший байт адреса"]
    #[inline(always)]
    pub fn is_opcode_1addr(&self) -> bool {
        *self == Frameform::Opcode1addr
    }
    #[doc = "Код операции и два младших байта адреса"]
    #[inline(always)]
    pub fn is_opcode_2addr(&self) -> bool {
        *self == Frameform::Opcode2addr
    }
    #[doc = "Код операции и три младших байта адреса"]
    #[inline(always)]
    pub fn is_opcode_3addr(&self) -> bool {
        *self == Frameform::Opcode3addr
    }
    #[doc = "Код операции и 4 байта адреса"]
    #[inline(always)]
    pub fn is_opcode_4addr(&self) -> bool {
        *self == Frameform::Opcode4addr
    }
    #[doc = "Нет кода операции, три младших байта адреса"]
    #[inline(always)]
    pub fn is_noopcode_3addr(&self) -> bool {
        *self == Frameform::Noopcode3addr
    }
    #[doc = "Нет кода операции, 4 байта адреса"]
    #[inline(always)]
    pub fn is_noopcode_4addr(&self) -> bool {
        *self == Frameform::Noopcode4addr
    }
}
#[doc = "Field `FRAMEFORM` writer - Бит управления полями кода операции и адреса команды"]
pub type FrameformW<'a, REG> = crate::FieldWriter<'a, REG, 3, Frameform>;
impl<'a, REG> FrameformW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Выдается только код операции, адреса нет"]
    #[inline(always)]
    pub fn opcode_noaddr(self) -> &'a mut crate::W<REG> {
        self.variant(Frameform::OpcodeNoaddr)
    }
    #[doc = "Код операции и младший байт адреса"]
    #[inline(always)]
    pub fn opcode_1addr(self) -> &'a mut crate::W<REG> {
        self.variant(Frameform::Opcode1addr)
    }
    #[doc = "Код операции и два младших байта адреса"]
    #[inline(always)]
    pub fn opcode_2addr(self) -> &'a mut crate::W<REG> {
        self.variant(Frameform::Opcode2addr)
    }
    #[doc = "Код операции и три младших байта адреса"]
    #[inline(always)]
    pub fn opcode_3addr(self) -> &'a mut crate::W<REG> {
        self.variant(Frameform::Opcode3addr)
    }
    #[doc = "Код операции и 4 байта адреса"]
    #[inline(always)]
    pub fn opcode_4addr(self) -> &'a mut crate::W<REG> {
        self.variant(Frameform::Opcode4addr)
    }
    #[doc = "Нет кода операции, три младших байта адреса"]
    #[inline(always)]
    pub fn noopcode_3addr(self) -> &'a mut crate::W<REG> {
        self.variant(Frameform::Noopcode3addr)
    }
    #[doc = "Нет кода операции, 4 байта адреса"]
    #[inline(always)]
    pub fn noopcode_4addr(self) -> &'a mut crate::W<REG> {
        self.variant(Frameform::Noopcode4addr)
    }
}
#[doc = "Field `OPCODE` reader - Код операции (не используется для некоторых значений поля FRAMEFORM)"]
pub type OpcodeR = crate::FieldReader;
#[doc = "Field `OPCODE` writer - Код операции (не используется для некоторых значений поля FRAMEFORM)"]
pub type OpcodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:13 - В тех случаях, когда бит POLL равен «0», это поле определяет количество байт данных при выполнении команды. Если это поле равно «0», то выполняемая команда не содержит данных"]
    #[inline(always)]
    pub fn datalen(&self) -> DatalenR {
        DatalenR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 14 - Бит должен быть установлен при выполнении команды, которая содержит входное поле данных и циклически запрашивает состояние бита входного потока битов из регистра статуса флэш-памяти. Номер проверяемого бита в байте статуса должен быть указан в битах DATALEN\\[2:0\\], а требуемое значение бита – в бите DATALEN\\[3\\]. Как только значения этих битов станут равны, контроллер завершает выполнение текущей команды, деактивируя сигнал SPIFI_CS, и формирует прерывание, если оно разрешено. После завершения процесса поллинга необходимо считать один байт из регистра DATA, так как в нем сохряняется последний принятый статус FLASH."]
    #[inline(always)]
    pub fn poll(&self) -> PollR {
        PollR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Бит направления передачи данных. «0» – чтение из флэш"]
    #[inline(always)]
    pub fn dout(&self) -> DoutR {
        DoutR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Количество байт промежуточных данных, которые хранятся в регистре IDATA. Если количество байт больше, чем может хранить регистр IDATA, то остаток добивается нулевыми значениями"]
    #[inline(always)]
    pub fn intlen(&self) -> IntlenR {
        IntlenR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:20 - Формат вывода полей команды"]
    #[inline(always)]
    pub fn fieldform(&self) -> FieldformR {
        FieldformR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 21:23 - Бит управления полями кода операции и адреса команды"]
    #[inline(always)]
    pub fn frameform(&self) -> FrameformR {
        FrameformR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:31 - Код операции (не используется для некоторых значений поля FRAMEFORM)"]
    #[inline(always)]
    pub fn opcode(&self) -> OpcodeR {
        OpcodeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - В тех случаях, когда бит POLL равен «0», это поле определяет количество байт данных при выполнении команды. Если это поле равно «0», то выполняемая команда не содержит данных"]
    #[inline(always)]
    pub fn datalen(&mut self) -> DatalenW<CmdSpec> {
        DatalenW::new(self, 0)
    }
    #[doc = "Bit 14 - Бит должен быть установлен при выполнении команды, которая содержит входное поле данных и циклически запрашивает состояние бита входного потока битов из регистра статуса флэш-памяти. Номер проверяемого бита в байте статуса должен быть указан в битах DATALEN\\[2:0\\], а требуемое значение бита – в бите DATALEN\\[3\\]. Как только значения этих битов станут равны, контроллер завершает выполнение текущей команды, деактивируя сигнал SPIFI_CS, и формирует прерывание, если оно разрешено. После завершения процесса поллинга необходимо считать один байт из регистра DATA, так как в нем сохряняется последний принятый статус FLASH."]
    #[inline(always)]
    pub fn poll(&mut self) -> PollW<CmdSpec> {
        PollW::new(self, 14)
    }
    #[doc = "Bit 15 - Бит направления передачи данных. «0» – чтение из флэш"]
    #[inline(always)]
    pub fn dout(&mut self) -> DoutW<CmdSpec> {
        DoutW::new(self, 15)
    }
    #[doc = "Bits 16:18 - Количество байт промежуточных данных, которые хранятся в регистре IDATA. Если количество байт больше, чем может хранить регистр IDATA, то остаток добивается нулевыми значениями"]
    #[inline(always)]
    pub fn intlen(&mut self) -> IntlenW<CmdSpec> {
        IntlenW::new(self, 16)
    }
    #[doc = "Bits 19:20 - Формат вывода полей команды"]
    #[inline(always)]
    pub fn fieldform(&mut self) -> FieldformW<CmdSpec> {
        FieldformW::new(self, 19)
    }
    #[doc = "Bits 21:23 - Бит управления полями кода операции и адреса команды"]
    #[inline(always)]
    pub fn frameform(&mut self) -> FrameformW<CmdSpec> {
        FrameformW::new(self, 21)
    }
    #[doc = "Bits 24:31 - Код операции (не используется для некоторых значений поля FRAMEFORM)"]
    #[inline(always)]
    pub fn opcode(&mut self) -> OpcodeW<CmdSpec> {
        OpcodeW::new(self, 24)
    }
}
#[doc = "SPIFI регистр команд. Этот регистр может быть записан только тогда, когда биты CMD и MCINIT равны нулю\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CmdSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CmdSpec {
    const RESET_VALUE: u32 = 0;
}
