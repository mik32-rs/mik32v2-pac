#[doc = "Register `MCMD` reader"]
pub type R = crate::R<McmdSpec>;
#[doc = "Register `MCMD` writer"]
pub type W = crate::W<McmdSpec>;
#[doc = "Бит должен быть установлен в «0»\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Poll {
    #[doc = "0: Режим поллинга выключен"]
    Disable = 0,
}
impl From<Poll> for bool {
    #[inline(always)]
    fn from(variant: Poll) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POLL` reader - Бит должен быть установлен в «0»"]
pub type PollR = crate::BitReader<Poll>;
impl PollR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Poll> {
        match self.bits {
            false => Some(Poll::Disable),
            _ => None,
        }
    }
    #[doc = "Режим поллинга выключен"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Poll::Disable
    }
}
#[doc = "Field `POLL` writer - Бит должен быть установлен в «0»"]
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
}
#[doc = "Бит должен быть установлен в «0»\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout {
    #[doc = "0: Чтение из флэш"]
    Read = 0,
}
impl From<Dout> for bool {
    #[inline(always)]
    fn from(variant: Dout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT` reader - Бит должен быть установлен в «0»"]
pub type DoutR = crate::BitReader<Dout>;
impl DoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dout> {
        match self.bits {
            false => Some(Dout::Read),
            _ => None,
        }
    }
    #[doc = "Чтение из флэш"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == Dout::Read
    }
}
#[doc = "Field `DOUT` writer - Бит должен быть установлен в «0»"]
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
}
#[doc = "Field `INTLEN` reader - Кол-во байт промежуточных данных (они хранятся в регистре IDATA). Если кол-во байт больше, чем может хранить регистр IDATA, остаток заполняется нулевыми значениями"]
pub type IntlenR = crate::FieldReader;
#[doc = "Field `INTLEN` writer - Кол-во байт промежуточных данных (они хранятся в регистре IDATA). Если кол-во байт больше, чем может хранить регистр IDATA, остаток заполняется нулевыми значениями"]
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
#[doc = "Бит управления полями кода операции и адреса команды\n\nValue on reset: 4"]
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
    #[doc = "Bit 14 - Бит должен быть установлен в «0»"]
    #[inline(always)]
    pub fn poll(&self) -> PollR {
        PollR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Бит должен быть установлен в «0»"]
    #[inline(always)]
    pub fn dout(&self) -> DoutR {
        DoutR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Кол-во байт промежуточных данных (они хранятся в регистре IDATA). Если кол-во байт больше, чем может хранить регистр IDATA, остаток заполняется нулевыми значениями"]
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
    #[doc = "Bit 14 - Бит должен быть установлен в «0»"]
    #[inline(always)]
    pub fn poll(&mut self) -> PollW<McmdSpec> {
        PollW::new(self, 14)
    }
    #[doc = "Bit 15 - Бит должен быть установлен в «0»"]
    #[inline(always)]
    pub fn dout(&mut self) -> DoutW<McmdSpec> {
        DoutW::new(self, 15)
    }
    #[doc = "Bits 16:18 - Кол-во байт промежуточных данных (они хранятся в регистре IDATA). Если кол-во байт больше, чем может хранить регистр IDATA, остаток заполняется нулевыми значениями"]
    #[inline(always)]
    pub fn intlen(&mut self) -> IntlenW<McmdSpec> {
        IntlenW::new(self, 16)
    }
    #[doc = "Bits 19:20 - Формат вывода полей команды"]
    #[inline(always)]
    pub fn fieldform(&mut self) -> FieldformW<McmdSpec> {
        FieldformW::new(self, 19)
    }
    #[doc = "Bits 21:23 - Бит управления полями кода операции и адреса команды"]
    #[inline(always)]
    pub fn frameform(&mut self) -> FrameformW<McmdSpec> {
        FrameformW::new(self, 21)
    }
    #[doc = "Bits 24:31 - Код операции (не используется для некоторых значений поля FRAMEFORM)"]
    #[inline(always)]
    pub fn opcode(&mut self) -> OpcodeW<McmdSpec> {
        OpcodeW::new(self, 24)
    }
}
#[doc = "SPIFI регистр команд памяти\n\nYou can [`read`](crate::Reg::read) this register and get [`mcmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McmdSpec;
impl crate::RegisterSpec for McmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcmd::R`](R) reader structure"]
impl crate::Readable for McmdSpec {}
#[doc = "`write(|w| ..)` method takes [`mcmd::W`](W) writer structure"]
impl crate::Writable for McmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCMD to value 0x0380_0000"]
impl crate::Resettable for McmdSpec {
    const RESET_VALUE: u32 = 0x0380_0000;
}
