#[doc = "Register `OTPADJ` reader"]
pub type R = crate::R<OtpadjSpec>;
#[doc = "Register `OTPADJ` writer"]
pub type W = crate::W<OtpadjSpec>;
#[doc = "Field `N_RSU` reader - Время между моментом начала транзакции на APB и положительным фронтом re i в тактах. Должно использоваться для обеспечения требования к временам предустановки Hard IP. Учитывая то, что на входе i удерживаются стабильные уровни в результате предыдущей операции записи в регистр OTPA, при частотах ниже 200 МГц данное значение рекомендуется устанавливать равным 0"]
pub type NRsuR = crate::FieldReader;
#[doc = "Field `N_RSU` writer - Время между моментом начала транзакции на APB и положительным фронтом re i в тактах. Должно использоваться для обеспечения требования к временам предустановки Hard IP. Учитывая то, что на входе i удерживаются стабильные уровни в результате предыдущей операции записи в регистр OTPA, при частотах ниже 200 МГц данное значение рекомендуется устанавливать равным 0"]
pub type NRsuW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `N_RA` reader - Длительность высокого уровня сигнала re_i (вход Hard IP) в тактах. Рекомендуемое значение N_RA = ceil(40/Pclk), где Pclk – период тактового сигнала в ceil – функция округле¬ния до ближайшего большего целого числа. Пример: два такта для частоты Fclk = 33,3 МГц. Для корректной работы схемы значение должно быть больше 0"]
pub type NRaR = crate::FieldReader;
#[doc = "Field `N_RA` writer - Длительность высокого уровня сигнала re_i (вход Hard IP) в тактах. Рекомендуемое значение N_RA = ceil(40/Pclk), где Pclk – период тактового сигнала в ceil – функция округле¬ния до ближайшего большего целого числа. Пример: два такта для частоты Fclk = 33,3 МГц. Для корректной работы схемы значение должно быть больше 0"]
pub type NRaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `N_RH` reader - Время между задним фронтом re_i и моментом, в который мо¬жет быть начата новая транзакция на APB. Должно исполь¬зоваться для обеспечения требования к временам удержания Hard IP. Рекомендуемое значение N_RH = =ceil(40/Pclk), где Pclk – период тактового сигнала в нс, ceil – функция округле¬ния до ближайшего большего целого числа. Пример: два такта для частоты Fclk = 33,3 МГц. Для корректной работы схемы значение должно быть больше 0"]
pub type NRhR = crate::FieldReader;
#[doc = "Field `N_RH` writer - Время между задним фронтом re_i и моментом, в который мо¬жет быть начата новая транзакция на APB. Должно исполь¬зоваться для обеспечения требования к временам удержания Hard IP. Рекомендуемое значение N_RH = =ceil(40/Pclk), где Pclk – период тактового сигнала в нс, ceil – функция округле¬ния до ближайшего большего целого числа. Пример: два такта для частоты Fclk = 33,3 МГц. Для корректной работы схемы значение должно быть больше 0"]
pub type NRhW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Выбор напряжения чтения\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SelUppReadI {
    #[doc = "0: Напряжения чтения 2,0 В"]
    _2v = 0,
    #[doc = "1: Напряжения чтения 2,5 B"]
    _2_5v = 1,
    #[doc = "3: Напряжения чтения 3,0 B"]
    _3v = 3,
    #[doc = "2: Напряжения чтения VDD18"]
    Vdd18 = 2,
    #[doc = "6: Напряжения чтения VDD5"]
    Vdd5 = 6,
}
impl From<SelUppReadI> for u8 {
    #[inline(always)]
    fn from(variant: SelUppReadI) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SelUppReadI {
    type Ux = u8;
}
impl crate::IsEnum for SelUppReadI {}
#[doc = "Field `sel_upp_read_i` reader - Выбор напряжения чтения"]
pub type SelUppReadIR = crate::FieldReader<SelUppReadI>;
impl SelUppReadIR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SelUppReadI> {
        match self.bits {
            0 => Some(SelUppReadI::_2v),
            1 => Some(SelUppReadI::_2_5v),
            3 => Some(SelUppReadI::_3v),
            2 => Some(SelUppReadI::Vdd18),
            6 => Some(SelUppReadI::Vdd5),
            _ => None,
        }
    }
    #[doc = "Напряжения чтения 2,0 В"]
    #[inline(always)]
    pub fn is_2v(&self) -> bool {
        *self == SelUppReadI::_2v
    }
    #[doc = "Напряжения чтения 2,5 B"]
    #[inline(always)]
    pub fn is_2_5v(&self) -> bool {
        *self == SelUppReadI::_2_5v
    }
    #[doc = "Напряжения чтения 3,0 B"]
    #[inline(always)]
    pub fn is_3v(&self) -> bool {
        *self == SelUppReadI::_3v
    }
    #[doc = "Напряжения чтения VDD18"]
    #[inline(always)]
    pub fn is_vdd18(&self) -> bool {
        *self == SelUppReadI::Vdd18
    }
    #[doc = "Напряжения чтения VDD5"]
    #[inline(always)]
    pub fn is_vdd5(&self) -> bool {
        *self == SelUppReadI::Vdd5
    }
}
#[doc = "Field `sel_upp_read_i` writer - Выбор напряжения чтения"]
pub type SelUppReadIW<'a, REG> = crate::FieldWriter<'a, REG, 3, SelUppReadI>;
impl<'a, REG> SelUppReadIW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Напряжения чтения 2,0 В"]
    #[inline(always)]
    pub fn _2v(self) -> &'a mut crate::W<REG> {
        self.variant(SelUppReadI::_2v)
    }
    #[doc = "Напряжения чтения 2,5 B"]
    #[inline(always)]
    pub fn _2_5v(self) -> &'a mut crate::W<REG> {
        self.variant(SelUppReadI::_2_5v)
    }
    #[doc = "Напряжения чтения 3,0 B"]
    #[inline(always)]
    pub fn _3v(self) -> &'a mut crate::W<REG> {
        self.variant(SelUppReadI::_3v)
    }
    #[doc = "Напряжения чтения VDD18"]
    #[inline(always)]
    pub fn vdd18(self) -> &'a mut crate::W<REG> {
        self.variant(SelUppReadI::Vdd18)
    }
    #[doc = "Напряжения чтения VDD5"]
    #[inline(always)]
    pub fn vdd5(self) -> &'a mut crate::W<REG> {
        self.variant(SelUppReadI::Vdd5)
    }
}
#[doc = "Выбор тока считывания. Используется для тестирования. Не рекомендуется изменять в штатном режиме работы.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelReadCurI {
    #[doc = "0: Ток считывания 2мкА"]
    _2uA = 0,
    #[doc = "1: Ток считывания 0,2мкА"]
    _0_2uA = 1,
}
impl From<SelReadCurI> for bool {
    #[inline(always)]
    fn from(variant: SelReadCurI) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `sel_read_cur_i` reader - Выбор тока считывания. Используется для тестирования. Не рекомендуется изменять в штатном режиме работы."]
pub type SelReadCurIR = crate::BitReader<SelReadCurI>;
impl SelReadCurIR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SelReadCurI {
        match self.bits {
            false => SelReadCurI::_2uA,
            true => SelReadCurI::_0_2uA,
        }
    }
    #[doc = "Ток считывания 2мкА"]
    #[inline(always)]
    pub fn is_2u_a(&self) -> bool {
        *self == SelReadCurI::_2uA
    }
    #[doc = "Ток считывания 0,2мкА"]
    #[inline(always)]
    pub fn is_0_2u_a(&self) -> bool {
        *self == SelReadCurI::_0_2uA
    }
}
#[doc = "Field `sel_read_cur_i` writer - Выбор тока считывания. Используется для тестирования. Не рекомендуется изменять в штатном режиме работы."]
pub type SelReadCurIW<'a, REG> = crate::BitWriter<'a, REG, SelReadCurI>;
impl<'a, REG> SelReadCurIW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ток считывания 2мкА"]
    #[inline(always)]
    pub fn _2u_a(self) -> &'a mut crate::W<REG> {
        self.variant(SelReadCurI::_2uA)
    }
    #[doc = "Ток считывания 0,2мкА"]
    #[inline(always)]
    pub fn _0_2u_a(self) -> &'a mut crate::W<REG> {
        self.variant(SelReadCurI::_0_2uA)
    }
}
#[doc = "Режим пониженного энергопотребления. После вывода Hard IP из режима пониженного энергопотребления требуется некоторое время перед тем, как могут быть начаты новые операции\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PowerOffI {
    #[doc = "0: Hard IP выведен из режима пониженного энергопотребления и может выполнять операции чтения и записи"]
    ActiveMode = 0,
    #[doc = "1: Hard IP введен в режим пониженного энергопотребления, операции записи и чтения запрещены"]
    LowPowerMode = 1,
}
impl From<PowerOffI> for bool {
    #[inline(always)]
    fn from(variant: PowerOffI) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `power_off_i` reader - Режим пониженного энергопотребления. После вывода Hard IP из режима пониженного энергопотребления требуется некоторое время перед тем, как могут быть начаты новые операции"]
pub type PowerOffIR = crate::BitReader<PowerOffI>;
impl PowerOffIR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PowerOffI {
        match self.bits {
            false => PowerOffI::ActiveMode,
            true => PowerOffI::LowPowerMode,
        }
    }
    #[doc = "Hard IP выведен из режима пониженного энергопотребления и может выполнять операции чтения и записи"]
    #[inline(always)]
    pub fn is_active_mode(&self) -> bool {
        *self == PowerOffI::ActiveMode
    }
    #[doc = "Hard IP введен в режим пониженного энергопотребления, операции записи и чтения запрещены"]
    #[inline(always)]
    pub fn is_low_power_mode(&self) -> bool {
        *self == PowerOffI::LowPowerMode
    }
}
#[doc = "Field `power_off_i` writer - Режим пониженного энергопотребления. После вывода Hard IP из режима пониженного энергопотребления требуется некоторое время перед тем, как могут быть начаты новые операции"]
pub type PowerOffIW<'a, REG> = crate::BitWriter<'a, REG, PowerOffI>;
impl<'a, REG> PowerOffIW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hard IP выведен из режима пониженного энергопотребления и может выполнять операции чтения и записи"]
    #[inline(always)]
    pub fn active_mode(self) -> &'a mut crate::W<REG> {
        self.variant(PowerOffI::ActiveMode)
    }
    #[doc = "Hard IP введен в режим пониженного энергопотребления, операции записи и чтения запрещены"]
    #[inline(always)]
    pub fn low_power_mode(self) -> &'a mut crate::W<REG> {
        self.variant(PowerOffI::LowPowerMode)
    }
}
impl R {
    #[doc = "Bits 0:2 - Время между моментом начала транзакции на APB и положительным фронтом re i в тактах. Должно использоваться для обеспечения требования к временам предустановки Hard IP. Учитывая то, что на входе i удерживаются стабильные уровни в результате предыдущей операции записи в регистр OTPA, при частотах ниже 200 МГц данное значение рекомендуется устанавливать равным 0"]
    #[inline(always)]
    pub fn n_rsu(&self) -> NRsuR {
        NRsuR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - Длительность высокого уровня сигнала re_i (вход Hard IP) в тактах. Рекомендуемое значение N_RA = ceil(40/Pclk), где Pclk – период тактового сигнала в ceil – функция округле¬ния до ближайшего большего целого числа. Пример: два такта для частоты Fclk = 33,3 МГц. Для корректной работы схемы значение должно быть больше 0"]
    #[inline(always)]
    pub fn n_ra(&self) -> NRaR {
        NRaR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Время между задним фронтом re_i и моментом, в который мо¬жет быть начата новая транзакция на APB. Должно исполь¬зоваться для обеспечения требования к временам удержания Hard IP. Рекомендуемое значение N_RH = =ceil(40/Pclk), где Pclk – период тактового сигнала в нс, ceil – функция округле¬ния до ближайшего большего целого числа. Пример: два такта для частоты Fclk = 33,3 МГц. Для корректной работы схемы значение должно быть больше 0"]
    #[inline(always)]
    pub fn n_rh(&self) -> NRhR {
        NRhR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Выбор напряжения чтения"]
    #[inline(always)]
    pub fn sel_upp_read_i(&self) -> SelUppReadIR {
        SelUppReadIR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Выбор тока считывания. Используется для тестирования. Не рекомендуется изменять в штатном режиме работы."]
    #[inline(always)]
    pub fn sel_read_cur_i(&self) -> SelReadCurIR {
        SelReadCurIR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Режим пониженного энергопотребления. После вывода Hard IP из режима пониженного энергопотребления требуется некоторое время перед тем, как могут быть начаты новые операции"]
    #[inline(always)]
    pub fn power_off_i(&self) -> PowerOffIR {
        PowerOffIR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Время между моментом начала транзакции на APB и положительным фронтом re i в тактах. Должно использоваться для обеспечения требования к временам предустановки Hard IP. Учитывая то, что на входе i удерживаются стабильные уровни в результате предыдущей операции записи в регистр OTPA, при частотах ниже 200 МГц данное значение рекомендуется устанавливать равным 0"]
    #[inline(always)]
    pub fn n_rsu(&mut self) -> NRsuW<OtpadjSpec> {
        NRsuW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Длительность высокого уровня сигнала re_i (вход Hard IP) в тактах. Рекомендуемое значение N_RA = ceil(40/Pclk), где Pclk – период тактового сигнала в ceil – функция округле¬ния до ближайшего большего целого числа. Пример: два такта для частоты Fclk = 33,3 МГц. Для корректной работы схемы значение должно быть больше 0"]
    #[inline(always)]
    pub fn n_ra(&mut self) -> NRaW<OtpadjSpec> {
        NRaW::new(self, 8)
    }
    #[doc = "Bits 16:18 - Время между задним фронтом re_i и моментом, в который мо¬жет быть начата новая транзакция на APB. Должно исполь¬зоваться для обеспечения требования к временам удержания Hard IP. Рекомендуемое значение N_RH = =ceil(40/Pclk), где Pclk – период тактового сигнала в нс, ceil – функция округле¬ния до ближайшего большего целого числа. Пример: два такта для частоты Fclk = 33,3 МГц. Для корректной работы схемы значение должно быть больше 0"]
    #[inline(always)]
    pub fn n_rh(&mut self) -> NRhW<OtpadjSpec> {
        NRhW::new(self, 16)
    }
    #[doc = "Bits 24:26 - Выбор напряжения чтения"]
    #[inline(always)]
    pub fn sel_upp_read_i(&mut self) -> SelUppReadIW<OtpadjSpec> {
        SelUppReadIW::new(self, 24)
    }
    #[doc = "Bit 27 - Выбор тока считывания. Используется для тестирования. Не рекомендуется изменять в штатном режиме работы."]
    #[inline(always)]
    pub fn sel_read_cur_i(&mut self) -> SelReadCurIW<OtpadjSpec> {
        SelReadCurIW::new(self, 27)
    }
    #[doc = "Bit 28 - Режим пониженного энергопотребления. После вывода Hard IP из режима пониженного энергопотребления требуется некоторое время перед тем, как могут быть начаты новые операции"]
    #[inline(always)]
    pub fn power_off_i(&mut self) -> PowerOffIW<OtpadjSpec> {
        PowerOffIW::new(self, 28)
    }
}
#[doc = "Регистр управления временными параметрами процедуры чтения и доп. настройками\n\nYou can [`read`](crate::Reg::read) this register and get [`otpadj::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otpadj::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpadjSpec;
impl crate::RegisterSpec for OtpadjSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otpadj::R`](R) reader structure"]
impl crate::Readable for OtpadjSpec {}
#[doc = "`write(|w| ..)` method takes [`otpadj::W`](W) writer structure"]
impl crate::Writable for OtpadjSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTPADJ to value 0x0101_0100"]
impl crate::Resettable for OtpadjSpec {
    const RESET_VALUE: u32 = 0x0101_0100;
}
