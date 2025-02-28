#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Инверсия контрольной суммы. Некоторые протоколы подсчета контрольной суммы требуют инверсии вычисленного значения контрольной суммы (выполняется операция XOR со значением 0xFFFFFFFF)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fxor {
    #[doc = "0: Инверсия выключена"]
    InversionDisable = 0,
    #[doc = "1: Инверсия включена (операция XOR выполняется)"]
    InversionEnable = 1,
}
impl From<Fxor> for u8 {
    #[inline(always)]
    fn from(variant: Fxor) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fxor {
    type Ux = u8;
}
impl crate::IsEnum for Fxor {}
#[doc = "Field `FXOR` reader - Инверсия контрольной суммы. Некоторые протоколы подсчета контрольной суммы требуют инверсии вычисленного значения контрольной суммы (выполняется операция XOR со значением 0xFFFFFFFF)"]
pub type FxorR = crate::FieldReader<Fxor>;
impl FxorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fxor> {
        match self.bits {
            0 => Some(Fxor::InversionDisable),
            1 => Some(Fxor::InversionEnable),
            _ => None,
        }
    }
    #[doc = "Инверсия выключена"]
    #[inline(always)]
    pub fn is_inversion_disable(&self) -> bool {
        *self == Fxor::InversionDisable
    }
    #[doc = "Инверсия включена (операция XOR выполняется)"]
    #[inline(always)]
    pub fn is_inversion_enable(&self) -> bool {
        *self == Fxor::InversionEnable
    }
}
#[doc = "Field `FXOR` writer - Инверсия контрольной суммы. Некоторые протоколы подсчета контрольной суммы требуют инверсии вычисленного значения контрольной суммы (выполняется операция XOR со значением 0xFFFFFFFF)"]
pub type FxorW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fxor>;
impl<'a, REG> FxorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Инверсия выключена"]
    #[inline(always)]
    pub fn inversion_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Fxor::InversionDisable)
    }
    #[doc = "Инверсия включена (операция XOR выполняется)"]
    #[inline(always)]
    pub fn inversion_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Fxor::InversionEnable)
    }
}
#[doc = "Назначение регистра данных\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Was {
    #[doc = "0: Записываем данные"]
    Data = 0,
    #[doc = "1: Записываем начальное значение"]
    InitData = 1,
}
impl From<Was> for bool {
    #[inline(always)]
    fn from(variant: Was) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAS` reader - Назначение регистра данных"]
pub type WasR = crate::BitReader<Was>;
impl WasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Was {
        match self.bits {
            false => Was::Data,
            true => Was::InitData,
        }
    }
    #[doc = "Записываем данные"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == Was::Data
    }
    #[doc = "Записываем начальное значение"]
    #[inline(always)]
    pub fn is_init_data(&self) -> bool {
        *self == Was::InitData
    }
}
#[doc = "Field `WAS` writer - Назначение регистра данных"]
pub type WasW<'a, REG> = crate::BitWriter<'a, REG, Was>;
impl<'a, REG> WasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Записываем данные"]
    #[inline(always)]
    pub fn data(self) -> &'a mut crate::W<REG> {
        self.variant(Was::Data)
    }
    #[doc = "Записываем начальное значение"]
    #[inline(always)]
    pub fn init_data(self) -> &'a mut crate::W<REG> {
        self.variant(Was::InitData)
    }
}
#[doc = "Бит занятости автомата. После записи в регистр данных слова для вычисления контрольной суммы бит занятости перейдет в состояние единицы через один такт после такта записи. То есть чтение регистра управления, идущее на шине AHB сразу на следующем такте после записи данных, вернет результат с нулевым (еще не обновленным) значением бита Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: Автомат закончил вычисления"]
    Ready = 0,
    #[doc = "1: Автомат занят"]
    Busy = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Busy` reader - Бит занятости автомата. После записи в регистр данных слова для вычисления контрольной суммы бит занятости перейдет в состояние единицы через один такт после такта записи. То есть чтение регистра управления, идущее на шине AHB сразу на следующем такте после записи данных, вернет результат с нулевым (еще не обновленным) значением бита Busy"]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            false => Busy::Ready,
            true => Busy::Busy,
        }
    }
    #[doc = "Автомат закончил вычисления"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Busy::Ready
    }
    #[doc = "Автомат занят"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Busy::Busy
    }
}
#[doc = "Перестановки битов/байтов выходных данных\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Totr {
    #[doc = "0: Перестановка выключена"]
    None = 0,
    #[doc = "1: Биты в байтах перестанавливаются (бит 7 с битом 0, бит 6 с битом 1 и т.д.; бит 15 c битом 8, бит 14 с битом 9 и т.д., с остальными байтами так же), байты НЕ перестанавливаются"]
    Bits = 1,
    #[doc = "2: Перестанавливаются и биты, и байты"]
    BitsBytes = 2,
    #[doc = "3: Биты в байтах НЕ перестанавливаются, байты перестанавливаются (меняются местами байты 3 и 0, 2 и 1);"]
    Bytes = 3,
}
impl From<Totr> for u8 {
    #[inline(always)]
    fn from(variant: Totr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Totr {
    type Ux = u8;
}
impl crate::IsEnum for Totr {}
#[doc = "Field `TOTR` reader - Перестановки битов/байтов выходных данных"]
pub type TotrR = crate::FieldReader<Totr>;
impl TotrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Totr {
        match self.bits {
            0 => Totr::None,
            1 => Totr::Bits,
            2 => Totr::BitsBytes,
            3 => Totr::Bytes,
            _ => unreachable!(),
        }
    }
    #[doc = "Перестановка выключена"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Totr::None
    }
    #[doc = "Биты в байтах перестанавливаются (бит 7 с битом 0, бит 6 с битом 1 и т.д.; бит 15 c битом 8, бит 14 с битом 9 и т.д., с остальными байтами так же), байты НЕ перестанавливаются"]
    #[inline(always)]
    pub fn is_bits(&self) -> bool {
        *self == Totr::Bits
    }
    #[doc = "Перестанавливаются и биты, и байты"]
    #[inline(always)]
    pub fn is_bits_bytes(&self) -> bool {
        *self == Totr::BitsBytes
    }
    #[doc = "Биты в байтах НЕ перестанавливаются, байты перестанавливаются (меняются местами байты 3 и 0, 2 и 1);"]
    #[inline(always)]
    pub fn is_bytes(&self) -> bool {
        *self == Totr::Bytes
    }
}
#[doc = "Field `TOTR` writer - Перестановки битов/байтов выходных данных"]
pub type TotrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Totr, crate::Safe>;
impl<'a, REG> TotrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Перестановка выключена"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Totr::None)
    }
    #[doc = "Биты в байтах перестанавливаются (бит 7 с битом 0, бит 6 с битом 1 и т.д.; бит 15 c битом 8, бит 14 с битом 9 и т.д., с остальными байтами так же), байты НЕ перестанавливаются"]
    #[inline(always)]
    pub fn bits_(self) -> &'a mut crate::W<REG> {
        self.variant(Totr::Bits)
    }
    #[doc = "Перестанавливаются и биты, и байты"]
    #[inline(always)]
    pub fn bits_bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Totr::BitsBytes)
    }
    #[doc = "Биты в байтах НЕ перестанавливаются, байты перестанавливаются (меняются местами байты 3 и 0, 2 и 1);"]
    #[inline(always)]
    pub fn bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Totr::Bytes)
    }
}
#[doc = "Перестановка битов/байтов входных данных\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tot {
    #[doc = "0: Перестановка выключена"]
    None = 0,
    #[doc = "1: Биты в байтах перестанавливаются (бит 7 с битом 0, бит 15 с битом 8, с остальными байтами так же), байты НЕ перестанавливаются;"]
    Bits = 1,
    #[doc = "2: Перестанавливаются и биты, и байты"]
    BitsBytes = 2,
    #[doc = "3: Биты в байтах НЕ перестанавливаются, байты перестанавливаются (меняются местами байты 3 и 0, 2 и 1);"]
    Bytes = 3,
}
impl From<Tot> for u8 {
    #[inline(always)]
    fn from(variant: Tot) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tot {
    type Ux = u8;
}
impl crate::IsEnum for Tot {}
#[doc = "Field `TOT` reader - Перестановка битов/байтов входных данных"]
pub type TotR = crate::FieldReader<Tot>;
impl TotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tot {
        match self.bits {
            0 => Tot::None,
            1 => Tot::Bits,
            2 => Tot::BitsBytes,
            3 => Tot::Bytes,
            _ => unreachable!(),
        }
    }
    #[doc = "Перестановка выключена"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Tot::None
    }
    #[doc = "Биты в байтах перестанавливаются (бит 7 с битом 0, бит 15 с битом 8, с остальными байтами так же), байты НЕ перестанавливаются;"]
    #[inline(always)]
    pub fn is_bits(&self) -> bool {
        *self == Tot::Bits
    }
    #[doc = "Перестанавливаются и биты, и байты"]
    #[inline(always)]
    pub fn is_bits_bytes(&self) -> bool {
        *self == Tot::BitsBytes
    }
    #[doc = "Биты в байтах НЕ перестанавливаются, байты перестанавливаются (меняются местами байты 3 и 0, 2 и 1);"]
    #[inline(always)]
    pub fn is_bytes(&self) -> bool {
        *self == Tot::Bytes
    }
}
#[doc = "Field `TOT` writer - Перестановка битов/байтов входных данных"]
pub type TotW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tot, crate::Safe>;
impl<'a, REG> TotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Перестановка выключена"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Tot::None)
    }
    #[doc = "Биты в байтах перестанавливаются (бит 7 с битом 0, бит 15 с битом 8, с остальными байтами так же), байты НЕ перестанавливаются;"]
    #[inline(always)]
    pub fn bits_(self) -> &'a mut crate::W<REG> {
        self.variant(Tot::Bits)
    }
    #[doc = "Перестанавливаются и биты, и байты"]
    #[inline(always)]
    pub fn bits_bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Tot::BitsBytes)
    }
    #[doc = "Биты в байтах НЕ перестанавливаются, байты перестанавливаются (меняются местами байты 3 и 0, 2 и 1);"]
    #[inline(always)]
    pub fn bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Tot::Bytes)
    }
}
impl R {
    #[doc = "Bits 1:2 - Инверсия контрольной суммы. Некоторые протоколы подсчета контрольной суммы требуют инверсии вычисленного значения контрольной суммы (выполняется операция XOR со значением 0xFFFFFFFF)"]
    #[inline(always)]
    pub fn fxor(&self) -> FxorR {
        FxorR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 25 - Назначение регистра данных"]
    #[inline(always)]
    pub fn was(&self) -> WasR {
        WasR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 25 - Бит занятости автомата. После записи в регистр данных слова для вычисления контрольной суммы бит занятости перейдет в состояние единицы через один такт после такта записи. То есть чтение регистра управления, идущее на шине AHB сразу на следующем такте после записи данных, вернет результат с нулевым (еще не обновленным) значением бита Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Перестановки битов/байтов выходных данных"]
    #[inline(always)]
    pub fn totr(&self) -> TotrR {
        TotrR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Перестановка битов/байтов входных данных"]
    #[inline(always)]
    pub fn tot(&self) -> TotR {
        TotR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 1:2 - Инверсия контрольной суммы. Некоторые протоколы подсчета контрольной суммы требуют инверсии вычисленного значения контрольной суммы (выполняется операция XOR со значением 0xFFFFFFFF)"]
    #[inline(always)]
    pub fn fxor(&mut self) -> FxorW<CtrlSpec> {
        FxorW::new(self, 1)
    }
    #[doc = "Bit 25 - Назначение регистра данных"]
    #[inline(always)]
    pub fn was(&mut self) -> WasW<CtrlSpec> {
        WasW::new(self, 25)
    }
    #[doc = "Bits 28:29 - Перестановки битов/байтов выходных данных"]
    #[inline(always)]
    pub fn totr(&mut self) -> TotrW<CtrlSpec> {
        TotrW::new(self, 28)
    }
    #[doc = "Bits 30:31 - Перестановка битов/байтов входных данных"]
    #[inline(always)]
    pub fn tot(&mut self) -> TotW<CtrlSpec> {
        TotW::new(self, 30)
    }
}
#[doc = "Регистр управления\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
