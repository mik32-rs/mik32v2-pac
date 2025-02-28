#[doc = "Register `EECON` writer"]
pub type W = crate::W<EeconSpec>;
#[doc = "Field `EX` writer - Запуск процедуры. Запись 1 в данный бит инициирует выпол-нение процедуры, процедура определяется битами OP"]
pub type ExW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Выбор процедуры\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Op {
    #[doc = "0: Чтение страницы"]
    ReadPage = 0,
    #[doc = "1: Стирание"]
    Erase = 1,
    #[doc = "2: Программирование"]
    Programming = 2,
}
impl From<Op> for u8 {
    #[inline(always)]
    fn from(variant: Op) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Op {
    type Ux = u8;
}
impl crate::IsEnum for Op {}
#[doc = "Field `OP` writer - Выбор процедуры"]
pub type OpW<'a, REG> = crate::FieldWriter<'a, REG, 2, Op>;
impl<'a, REG> OpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Чтение страницы"]
    #[inline(always)]
    pub fn read_page(self) -> &'a mut crate::W<REG> {
        self.variant(Op::ReadPage)
    }
    #[doc = "Стирание"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut crate::W<REG> {
        self.variant(Op::Erase)
    }
    #[doc = "Программирование"]
    #[inline(always)]
    pub fn programming(self) -> &'a mut crate::W<REG> {
        self.variant(Op::Programming)
    }
}
#[doc = "Поведение операции стирания/програмирования. Данные биты должны быть установлены перед операцией заполнения буфера записи перед операциями стирания и программирования\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wrben {
    #[doc = "0: обычное стирание/програмирование одной страницы;"]
    OnePage = 0,
    #[doc = "1: стирание/програмирование всех четных страниц;"]
    EvenPages = 1,
    #[doc = "2: стирание/програмирование всех нечетных страниц;"]
    OddPages = 2,
    #[doc = "3: стирание/програмирование всех страниц."]
    AllPages = 3,
}
impl From<Wrben> for u8 {
    #[inline(always)]
    fn from(variant: Wrben) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wrben {
    type Ux = u8;
}
impl crate::IsEnum for Wrben {}
#[doc = "Field `WRBEN` writer - Поведение операции стирания/програмирования. Данные биты должны быть установлены перед операцией заполнения буфера записи перед операциями стирания и программирования"]
pub type WrbenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wrben, crate::Safe>;
impl<'a, REG> WrbenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "обычное стирание/програмирование одной страницы;"]
    #[inline(always)]
    pub fn one_page(self) -> &'a mut crate::W<REG> {
        self.variant(Wrben::OnePage)
    }
    #[doc = "стирание/програмирование всех четных страниц;"]
    #[inline(always)]
    pub fn even_pages(self) -> &'a mut crate::W<REG> {
        self.variant(Wrben::EvenPages)
    }
    #[doc = "стирание/програмирование всех нечетных страниц;"]
    #[inline(always)]
    pub fn odd_pages(self) -> &'a mut crate::W<REG> {
        self.variant(Wrben::OddPages)
    }
    #[doc = "стирание/програмирование всех страниц."]
    #[inline(always)]
    pub fn all_pages(self) -> &'a mut crate::W<REG> {
        self.variant(Wrben::AllPages)
    }
}
#[doc = "Отключение вставки тактов ожидания в процессе обмена по APB при заполнении буфера записи (во время записи в EEDAT) и при записи адреса для процедуры чтения данных (EEA). Если такты ожидания отключены (APBNWS=1), то требуется производить опрос флага EESTA.BSY после этих операций до тех пор, пока EESTA.BSY не станет равным 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Apbnws {
    #[doc = "0: Такты ожидания включены"]
    Disable = 0,
    #[doc = "1: Такты ожидания отключены"]
    Enable = 1,
}
impl From<Apbnws> for bool {
    #[inline(always)]
    fn from(variant: Apbnws) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APBNWS` writer - Отключение вставки тактов ожидания в процессе обмена по APB при заполнении буфера записи (во время записи в EEDAT) и при записи адреса для процедуры чтения данных (EEA). Если такты ожидания отключены (APBNWS=1), то требуется производить опрос флага EESTA.BSY после этих операций до тех пор, пока EESTA.BSY не станет равным 0"]
pub type ApbnwsW<'a, REG> = crate::BitWriter<'a, REG, Apbnws>;
impl<'a, REG> ApbnwsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Такты ожидания включены"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Apbnws::Disable)
    }
    #[doc = "Такты ожидания отключены"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Apbnws::Enable)
    }
}
#[doc = "Отключение схемы коррекции ошибок во время выполнения запрошенной процедуры. Отлючение производится записью «1» в этот бит. Если схема коррекции отключена, то при записи значение бит коррекции определяется шестью млад¬шими битами слова. Если схема коррекции включена, то при записи значение бит коррекции вычисляется схемой SEC32 - ENC. Если схема коррекции отключена, то при чтении не про¬изводится коррекция возможных ошибок в слове. Если схема коррекции включена, то при чтении данные корректируются схемой DEC32_ENC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disecc {
    #[doc = "0: Включение схемы коррекции ошибок во время выполнения запрошенной процедуры"]
    Disable = 0,
    #[doc = "1: Отключение схемы коррекции ошибок во время выполнения запрошенной процедуры"]
    Enable = 1,
}
impl From<Disecc> for bool {
    #[inline(always)]
    fn from(variant: Disecc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISECC` writer - Отключение схемы коррекции ошибок во время выполнения запрошенной процедуры. Отлючение производится записью «1» в этот бит. Если схема коррекции отключена, то при записи значение бит коррекции определяется шестью млад¬шими битами слова. Если схема коррекции включена, то при записи значение бит коррекции вычисляется схемой SEC32 - ENC. Если схема коррекции отключена, то при чтении не про¬изводится коррекция возможных ошибок в слове. Если схема коррекции включена, то при чтении данные корректируются схемой DEC32_ENC"]
pub type DiseccW<'a, REG> = crate::BitWriter<'a, REG, Disecc>;
impl<'a, REG> DiseccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Включение схемы коррекции ошибок во время выполнения запрошенной процедуры"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Disecc::Disable)
    }
    #[doc = "Отключение схемы коррекции ошибок во время выполнения запрошенной процедуры"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Disecc::Enable)
    }
}
#[doc = "Field `BWE` writer - Разрешение записи в буфер. Данный бит следует устанавливать в «1» перед операцией заполнения буфера записи перед операциями стирания и программирования. После операций стирания и программирования данный бит автоматически очищается (то есть для последующей записи в буфер следует повторно выполнить запись «1» в этот бит)"]
pub type BweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Разрешение запроса прерывания при поднятии флага SERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IeseraR {
    #[doc = "0: Запрос не поступает"]
    Disable = 0,
    #[doc = "1: Запрос поступает"]
    Enable = 1,
}
impl From<IeseraR> for bool {
    #[inline(always)]
    fn from(variant: IeseraR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IESERaR` writer - Разрешение запроса прерывания при поднятии флага SERR"]
pub type IeseraRW<'a, REG> = crate::BitWriter<'a, REG, IeseraR>;
impl<'a, REG> IeseraRW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Запрос не поступает"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(IeseraR::Disable)
    }
    #[doc = "Запрос поступает"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(IeseraR::Enable)
    }
}
impl W {
    #[doc = "Bit 0 - Запуск процедуры. Запись 1 в данный бит инициирует выпол-нение процедуры, процедура определяется битами OP"]
    #[inline(always)]
    pub fn ex(&mut self) -> ExW<EeconSpec> {
        ExW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Выбор процедуры"]
    #[inline(always)]
    pub fn op(&mut self) -> OpW<EeconSpec> {
        OpW::new(self, 1)
    }
    #[doc = "Bits 3:4 - Поведение операции стирания/програмирования. Данные биты должны быть установлены перед операцией заполнения буфера записи перед операциями стирания и программирования"]
    #[inline(always)]
    pub fn wrben(&mut self) -> WrbenW<EeconSpec> {
        WrbenW::new(self, 3)
    }
    #[doc = "Bit 5 - Отключение вставки тактов ожидания в процессе обмена по APB при заполнении буфера записи (во время записи в EEDAT) и при записи адреса для процедуры чтения данных (EEA). Если такты ожидания отключены (APBNWS=1), то требуется производить опрос флага EESTA.BSY после этих операций до тех пор, пока EESTA.BSY не станет равным 0"]
    #[inline(always)]
    pub fn apbnws(&mut self) -> ApbnwsW<EeconSpec> {
        ApbnwsW::new(self, 5)
    }
    #[doc = "Bit 6 - Отключение схемы коррекции ошибок во время выполнения запрошенной процедуры. Отлючение производится записью «1» в этот бит. Если схема коррекции отключена, то при записи значение бит коррекции определяется шестью млад¬шими битами слова. Если схема коррекции включена, то при записи значение бит коррекции вычисляется схемой SEC32 - ENC. Если схема коррекции отключена, то при чтении не про¬изводится коррекция возможных ошибок в слове. Если схема коррекции включена, то при чтении данные корректируются схемой DEC32_ENC"]
    #[inline(always)]
    pub fn disecc(&mut self) -> DiseccW<EeconSpec> {
        DiseccW::new(self, 6)
    }
    #[doc = "Bit 7 - Разрешение записи в буфер. Данный бит следует устанавливать в «1» перед операцией заполнения буфера записи перед операциями стирания и программирования. После операций стирания и программирования данный бит автоматически очищается (то есть для последующей записи в буфер следует повторно выполнить запись «1» в этот бит)"]
    #[inline(always)]
    pub fn bwe(&mut self) -> BweW<EeconSpec> {
        BweW::new(self, 7)
    }
    #[doc = "Bit 8 - Разрешение запроса прерывания при поднятии флага SERR"]
    #[inline(always)]
    pub fn iesera_r(&mut self) -> IeseraRW<EeconSpec> {
        IeseraRW::new(self, 8)
    }
}
#[doc = "Регистр управления\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eecon::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EeconSpec;
impl crate::RegisterSpec for EeconSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`eecon::W`](W) writer structure"]
impl crate::Writable for EeconSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EECON to value 0"]
impl crate::Resettable for EeconSpec {
    const RESET_VALUE: u32 = 0;
}
