#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "(MSTREN) Выбор режима\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModeSel {
    #[doc = "0: SPI в режиме ведомого устройства"]
    Slave = 0,
    #[doc = "1: SPI в режиме ведущего устройства"]
    Master = 1,
}
impl From<ModeSel> for bool {
    #[inline(always)]
    fn from(variant: ModeSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE_SEL` reader - (MSTREN) Выбор режима"]
pub type ModeSelR = crate::BitReader<ModeSel>;
impl ModeSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ModeSel {
        match self.bits {
            false => ModeSel::Slave,
            true => ModeSel::Master,
        }
    }
    #[doc = "SPI в режиме ведомого устройства"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == ModeSel::Slave
    }
    #[doc = "SPI в режиме ведущего устройства"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == ModeSel::Master
    }
}
#[doc = "Field `MODE_SEL` writer - (MSTREN) Выбор режима"]
pub type ModeSelW<'a, REG> = crate::BitWriter<'a, REG, ModeSel>;
impl<'a, REG> ModeSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI в режиме ведомого устройства"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut crate::W<REG> {
        self.variant(ModeSel::Slave)
    }
    #[doc = "SPI в режиме ведущего устройства"]
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(ModeSel::Master)
    }
}
#[doc = "(CPOL)Выбор полярности тактового сигнала вне слова\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkPol {
    #[doc = "0: Тактовый сигнал вне слова удерживается на низком уровне"]
    _0 = 0,
    #[doc = "1: Тактовый сигнал вне слова удерживается на высоком уровне"]
    _1 = 1,
}
impl From<ClkPol> for bool {
    #[inline(always)]
    fn from(variant: ClkPol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_POL` reader - (CPOL)Выбор полярности тактового сигнала вне слова"]
pub type ClkPolR = crate::BitReader<ClkPol>;
impl ClkPolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkPol {
        match self.bits {
            false => ClkPol::_0,
            true => ClkPol::_1,
        }
    }
    #[doc = "Тактовый сигнал вне слова удерживается на низком уровне"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ClkPol::_0
    }
    #[doc = "Тактовый сигнал вне слова удерживается на высоком уровне"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ClkPol::_1
    }
}
#[doc = "Field `CLK_POL` writer - (CPOL)Выбор полярности тактового сигнала вне слова"]
pub type ClkPolW<'a, REG> = crate::BitWriter<'a, REG, ClkPol>;
impl<'a, REG> ClkPolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактовый сигнал вне слова удерживается на низком уровне"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkPol::_0)
    }
    #[doc = "Тактовый сигнал вне слова удерживается на высоком уровне"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkPol::_1)
    }
}
#[doc = "(CPHA)Выбор фазы тактового сигнала\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkPh {
    #[doc = "0: Тактовая частота SPI неактивна вне слова"]
    _0 = 0,
    #[doc = "1: Тактовая частота SPI активна вне слова"]
    _1 = 1,
}
impl From<ClkPh> for bool {
    #[inline(always)]
    fn from(variant: ClkPh) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLK_PH` reader - (CPHA)Выбор фазы тактового сигнала"]
pub type ClkPhR = crate::BitReader<ClkPh>;
impl ClkPhR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkPh {
        match self.bits {
            false => ClkPh::_0,
            true => ClkPh::_1,
        }
    }
    #[doc = "Тактовая частота SPI неактивна вне слова"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ClkPh::_0
    }
    #[doc = "Тактовая частота SPI активна вне слова"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ClkPh::_1
    }
}
#[doc = "Field `CLK_PH` writer - (CPHA)Выбор фазы тактового сигнала"]
pub type ClkPhW<'a, REG> = crate::BitWriter<'a, REG, ClkPh>;
impl<'a, REG> ClkPhW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактовая частота SPI неактивна вне слова"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkPh::_0)
    }
    #[doc = "Тактовая частота SPI активна вне слова"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkPh::_1)
    }
}
#[doc = "Управляет скоростью передачи данных, задает коэффициент деления частоты spi_ref_clk\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BaudRateDiv {
    #[doc = "0: Не поддерживается"]
    NotSupported = 0,
    #[doc = "1: Деление на 4"]
    Div4 = 1,
    #[doc = "2: Деление на 8"]
    Div8 = 2,
    #[doc = "3: Деление на 16"]
    Div16 = 3,
    #[doc = "4: Деление на 32"]
    Div32 = 4,
    #[doc = "5: деление на 64"]
    Div64 = 5,
    #[doc = "6: деление на 128"]
    Div128 = 6,
    #[doc = "7: деление на 256"]
    Div256 = 7,
}
impl From<BaudRateDiv> for u8 {
    #[inline(always)]
    fn from(variant: BaudRateDiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BaudRateDiv {
    type Ux = u8;
}
impl crate::IsEnum for BaudRateDiv {}
#[doc = "Field `BAUD_RATE_DIV` reader - Управляет скоростью передачи данных, задает коэффициент деления частоты spi_ref_clk"]
pub type BaudRateDivR = crate::FieldReader<BaudRateDiv>;
impl BaudRateDivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BaudRateDiv {
        match self.bits {
            0 => BaudRateDiv::NotSupported,
            1 => BaudRateDiv::Div4,
            2 => BaudRateDiv::Div8,
            3 => BaudRateDiv::Div16,
            4 => BaudRateDiv::Div32,
            5 => BaudRateDiv::Div64,
            6 => BaudRateDiv::Div128,
            7 => BaudRateDiv::Div256,
            _ => unreachable!(),
        }
    }
    #[doc = "Не поддерживается"]
    #[inline(always)]
    pub fn is_not_supported(&self) -> bool {
        *self == BaudRateDiv::NotSupported
    }
    #[doc = "Деление на 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == BaudRateDiv::Div4
    }
    #[doc = "Деление на 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == BaudRateDiv::Div8
    }
    #[doc = "Деление на 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == BaudRateDiv::Div16
    }
    #[doc = "Деление на 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == BaudRateDiv::Div32
    }
    #[doc = "деление на 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == BaudRateDiv::Div64
    }
    #[doc = "деление на 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == BaudRateDiv::Div128
    }
    #[doc = "деление на 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == BaudRateDiv::Div256
    }
}
#[doc = "Field `BAUD_RATE_DIV` writer - Управляет скоростью передачи данных, задает коэффициент деления частоты spi_ref_clk"]
pub type BaudRateDivW<'a, REG> = crate::FieldWriter<'a, REG, 3, BaudRateDiv, crate::Safe>;
impl<'a, REG> BaudRateDivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Не поддерживается"]
    #[inline(always)]
    pub fn not_supported(self) -> &'a mut crate::W<REG> {
        self.variant(BaudRateDiv::NotSupported)
    }
    #[doc = "Деление на 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(BaudRateDiv::Div4)
    }
    #[doc = "Деление на 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(BaudRateDiv::Div8)
    }
    #[doc = "Деление на 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(BaudRateDiv::Div16)
    }
    #[doc = "Деление на 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(BaudRateDiv::Div32)
    }
    #[doc = "деление на 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(BaudRateDiv::Div64)
    }
    #[doc = "деление на 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(BaudRateDiv::Div128)
    }
    #[doc = "деление на 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(BaudRateDiv::Div256)
    }
}
#[doc = "Выбор опорной тактовой частоты\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefClk {
    #[doc = "0: Не поддерживается"]
    NotSupported = 0,
    #[doc = "1: Используется опорная частота SPI"]
    ApbPClk = 1,
}
impl From<RefClk> for bool {
    #[inline(always)]
    fn from(variant: RefClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REF_CLK` reader - Выбор опорной тактовой частоты"]
pub type RefClkR = crate::BitReader<RefClk>;
impl RefClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RefClk {
        match self.bits {
            false => RefClk::NotSupported,
            true => RefClk::ApbPClk,
        }
    }
    #[doc = "Не поддерживается"]
    #[inline(always)]
    pub fn is_not_supported(&self) -> bool {
        *self == RefClk::NotSupported
    }
    #[doc = "Используется опорная частота SPI"]
    #[inline(always)]
    pub fn is_apb_p_clk(&self) -> bool {
        *self == RefClk::ApbPClk
    }
}
#[doc = "Field `REF_CLK` writer - Выбор опорной тактовой частоты"]
pub type RefClkW<'a, REG> = crate::BitWriter<'a, REG, RefClk>;
impl<'a, REG> RefClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Не поддерживается"]
    #[inline(always)]
    pub fn not_supported(self) -> &'a mut crate::W<REG> {
        self.variant(RefClk::NotSupported)
    }
    #[doc = "Используется опорная частота SPI"]
    #[inline(always)]
    pub fn apb_p_clk(self) -> &'a mut crate::W<REG> {
        self.variant(RefClk::ApbPClk)
    }
}
#[doc = "Выбор ведомых устройств\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cs {
    #[doc = "15: Ведомые устройства не выбраны"]
    NotSelected = 15,
    #[doc = "1: Выбрано устройство 1"]
    Cs1 = 1,
    #[doc = "3: Выбрано устройство 2"]
    Cs2 = 3,
    #[doc = "7: Выбрано устройство 3"]
    Cs3 = 7,
    #[doc = "0: Выбрано устройство 4"]
    Cs4 = 0,
}
impl From<Cs> for u8 {
    #[inline(always)]
    fn from(variant: Cs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cs {
    type Ux = u8;
}
impl crate::IsEnum for Cs {}
#[doc = "Field `CS` reader - Выбор ведомых устройств"]
pub type CsR = crate::FieldReader<Cs>;
impl CsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cs> {
        match self.bits {
            15 => Some(Cs::NotSelected),
            1 => Some(Cs::Cs1),
            3 => Some(Cs::Cs2),
            7 => Some(Cs::Cs3),
            0 => Some(Cs::Cs4),
            _ => None,
        }
    }
    #[doc = "Ведомые устройства не выбраны"]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == Cs::NotSelected
    }
    #[doc = "Выбрано устройство 1"]
    #[inline(always)]
    pub fn is_cs1(&self) -> bool {
        *self == Cs::Cs1
    }
    #[doc = "Выбрано устройство 2"]
    #[inline(always)]
    pub fn is_cs2(&self) -> bool {
        *self == Cs::Cs2
    }
    #[doc = "Выбрано устройство 3"]
    #[inline(always)]
    pub fn is_cs3(&self) -> bool {
        *self == Cs::Cs3
    }
    #[doc = "Выбрано устройство 4"]
    #[inline(always)]
    pub fn is_cs4(&self) -> bool {
        *self == Cs::Cs4
    }
}
#[doc = "Field `CS` writer - Выбор ведомых устройств"]
pub type CsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cs>;
impl<'a, REG> CsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Ведомые устройства не выбраны"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(Cs::NotSelected)
    }
    #[doc = "Выбрано устройство 1"]
    #[inline(always)]
    pub fn cs1(self) -> &'a mut crate::W<REG> {
        self.variant(Cs::Cs1)
    }
    #[doc = "Выбрано устройство 2"]
    #[inline(always)]
    pub fn cs2(self) -> &'a mut crate::W<REG> {
        self.variant(Cs::Cs2)
    }
    #[doc = "Выбрано устройство 3"]
    #[inline(always)]
    pub fn cs3(self) -> &'a mut crate::W<REG> {
        self.variant(Cs::Cs3)
    }
    #[doc = "Выбрано устройство 4"]
    #[inline(always)]
    pub fn cs4(self) -> &'a mut crate::W<REG> {
        self.variant(Cs::Cs4)
    }
}
#[doc = "Выбор режима управления сигналом выбора ведомого\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ManualCs {
    #[doc = "0: Автоматический режим"]
    Automatic = 0,
    #[doc = "1: Ручной режим"]
    Manual = 1,
}
impl From<ManualCs> for bool {
    #[inline(always)]
    fn from(variant: ManualCs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Manual_CS` reader - Выбор режима управления сигналом выбора ведомого"]
pub type ManualCsR = crate::BitReader<ManualCs>;
impl ManualCsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ManualCs {
        match self.bits {
            false => ManualCs::Automatic,
            true => ManualCs::Manual,
        }
    }
    #[doc = "Автоматический режим"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == ManualCs::Automatic
    }
    #[doc = "Ручной режим"]
    #[inline(always)]
    pub fn is_manual(&self) -> bool {
        *self == ManualCs::Manual
    }
}
#[doc = "Field `Manual_CS` writer - Выбор режима управления сигналом выбора ведомого"]
pub type ManualCsW<'a, REG> = crate::BitWriter<'a, REG, ManualCs>;
impl<'a, REG> ManualCsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Автоматический режим"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut crate::W<REG> {
        self.variant(ManualCs::Automatic)
    }
    #[doc = "Ручной режим"]
    #[inline(always)]
    pub fn manual(self) -> &'a mut crate::W<REG> {
        self.variant(ManualCs::Manual)
    }
}
impl R {
    #[doc = "Bit 0 - (MSTREN) Выбор режима"]
    #[inline(always)]
    pub fn mode_sel(&self) -> ModeSelR {
        ModeSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - (CPOL)Выбор полярности тактового сигнала вне слова"]
    #[inline(always)]
    pub fn clk_pol(&self) -> ClkPolR {
        ClkPolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - (CPHA)Выбор фазы тактового сигнала"]
    #[inline(always)]
    pub fn clk_ph(&self) -> ClkPhR {
        ClkPhR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Управляет скоростью передачи данных, задает коэффициент деления частоты spi_ref_clk"]
    #[inline(always)]
    pub fn baud_rate_div(&self) -> BaudRateDivR {
        BaudRateDivR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 8 - Выбор опорной тактовой частоты"]
    #[inline(always)]
    pub fn ref_clk(&self) -> RefClkR {
        RefClkR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Выбор ведомых устройств"]
    #[inline(always)]
    pub fn cs(&self) -> CsR {
        CsR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - Выбор режима управления сигналом выбора ведомого"]
    #[inline(always)]
    pub fn manual_cs(&self) -> ManualCsR {
        ManualCsR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - (MSTREN) Выбор режима"]
    #[inline(always)]
    pub fn mode_sel(&mut self) -> ModeSelW<ConfigSpec> {
        ModeSelW::new(self, 0)
    }
    #[doc = "Bit 1 - (CPOL)Выбор полярности тактового сигнала вне слова"]
    #[inline(always)]
    pub fn clk_pol(&mut self) -> ClkPolW<ConfigSpec> {
        ClkPolW::new(self, 1)
    }
    #[doc = "Bit 2 - (CPHA)Выбор фазы тактового сигнала"]
    #[inline(always)]
    pub fn clk_ph(&mut self) -> ClkPhW<ConfigSpec> {
        ClkPhW::new(self, 2)
    }
    #[doc = "Bits 3:5 - Управляет скоростью передачи данных, задает коэффициент деления частоты spi_ref_clk"]
    #[inline(always)]
    pub fn baud_rate_div(&mut self) -> BaudRateDivW<ConfigSpec> {
        BaudRateDivW::new(self, 3)
    }
    #[doc = "Bit 8 - Выбор опорной тактовой частоты"]
    #[inline(always)]
    pub fn ref_clk(&mut self) -> RefClkW<ConfigSpec> {
        RefClkW::new(self, 8)
    }
    #[doc = "Bits 10:13 - Выбор ведомых устройств"]
    #[inline(always)]
    pub fn cs(&mut self) -> CsW<ConfigSpec> {
        CsW::new(self, 10)
    }
    #[doc = "Bit 14 - Выбор режима управления сигналом выбора ведомого"]
    #[inline(always)]
    pub fn manual_cs(&mut self) -> ManualCsW<ConfigSpec> {
        ManualCsW::new(self, 14)
    }
}
#[doc = "Регистр конфигурации SPI\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
