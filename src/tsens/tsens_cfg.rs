#[doc = "Register `TSENS_CFG` reader"]
pub type R = crate::R<TsensCfgSpec>;
#[doc = "Register `TSENS_CFG` writer"]
pub type W = crate::W<TsensCfgSpec>;
#[doc = "Управление питанием сенсора\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Npd {
    #[doc = "0: Сенсор выключен"]
    Disable = 0,
    #[doc = "1: Сенсор включен"]
    Enable = 1,
}
impl From<Npd> for bool {
    #[inline(always)]
    fn from(variant: Npd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NPD` reader - Управление питанием сенсора"]
pub type NpdR = crate::BitReader<Npd>;
impl NpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Npd {
        match self.bits {
            false => Npd::Disable,
            true => Npd::Enable,
        }
    }
    #[doc = "Сенсор выключен"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Npd::Disable
    }
    #[doc = "Сенсор включен"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Npd::Enable
    }
}
#[doc = "Field `NPD` writer - Управление питанием сенсора"]
pub type NpdW<'a, REG> = crate::BitWriter<'a, REG, Npd>;
impl<'a, REG> NpdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Сенсор выключен"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Npd::Disable)
    }
    #[doc = "Сенсор включен"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Npd::Enable)
    }
}
#[doc = "Управление тактированием сенсора\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NpdClk {
    #[doc = "0: Тактирование сенсора выключено"]
    Disable = 0,
    #[doc = "1: Тактирование сенсора включено"]
    Enable = 1,
}
impl From<NpdClk> for bool {
    #[inline(always)]
    fn from(variant: NpdClk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NPD_CLK` reader - Управление тактированием сенсора"]
pub type NpdClkR = crate::BitReader<NpdClk>;
impl NpdClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NpdClk {
        match self.bits {
            false => NpdClk::Disable,
            true => NpdClk::Enable,
        }
    }
    #[doc = "Тактирование сенсора выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == NpdClk::Disable
    }
    #[doc = "Тактирование сенсора включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == NpdClk::Enable
    }
}
#[doc = "Field `NPD_CLK` writer - Управление тактированием сенсора"]
pub type NpdClkW<'a, REG> = crate::BitWriter<'a, REG, NpdClk>;
impl<'a, REG> NpdClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование сенсора выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(NpdClk::Disable)
    }
    #[doc = "Тактирование сенсора включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(NpdClk::Enable)
    }
}
#[doc = "Field `NRST` reader - Управление сбросом сенсора, активный уровень «0»"]
pub type NrstR = crate::BitReader;
#[doc = "Field `NRST` writer - Управление сбросом сенсора, активный уровень «0»"]
pub type NrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Выбор источника тактирования сенсора (Fin)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkMux {
    #[doc = "0: системная частота (sys_clk)"]
    SysClk = 0,
    #[doc = "1: Частота шины AHB (hclk)"]
    Hclk = 1,
    #[doc = "2: Частота внешнего осциллятора 32 МГц"]
    Osc32m = 2,
    #[doc = "3: Частота HSI32M"]
    Hsi32m = 3,
    #[doc = "4: Частота внешнего осциллятора 32 кГц"]
    Osc32k = 4,
    #[doc = "5: Частота LSI32K"]
    Lsi32k = 5,
}
impl From<ClkMux> for u8 {
    #[inline(always)]
    fn from(variant: ClkMux) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkMux {
    type Ux = u8;
}
impl crate::IsEnum for ClkMux {}
#[doc = "Field `CLK_MUX` reader - Выбор источника тактирования сенсора (Fin)"]
pub type ClkMuxR = crate::FieldReader<ClkMux>;
impl ClkMuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkMux> {
        match self.bits {
            0 => Some(ClkMux::SysClk),
            1 => Some(ClkMux::Hclk),
            2 => Some(ClkMux::Osc32m),
            3 => Some(ClkMux::Hsi32m),
            4 => Some(ClkMux::Osc32k),
            5 => Some(ClkMux::Lsi32k),
            _ => None,
        }
    }
    #[doc = "системная частота (sys_clk)"]
    #[inline(always)]
    pub fn is_sys_clk(&self) -> bool {
        *self == ClkMux::SysClk
    }
    #[doc = "Частота шины AHB (hclk)"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        *self == ClkMux::Hclk
    }
    #[doc = "Частота внешнего осциллятора 32 МГц"]
    #[inline(always)]
    pub fn is_osc32m(&self) -> bool {
        *self == ClkMux::Osc32m
    }
    #[doc = "Частота HSI32M"]
    #[inline(always)]
    pub fn is_hsi32m(&self) -> bool {
        *self == ClkMux::Hsi32m
    }
    #[doc = "Частота внешнего осциллятора 32 кГц"]
    #[inline(always)]
    pub fn is_osc32k(&self) -> bool {
        *self == ClkMux::Osc32k
    }
    #[doc = "Частота LSI32K"]
    #[inline(always)]
    pub fn is_lsi32k(&self) -> bool {
        *self == ClkMux::Lsi32k
    }
}
#[doc = "Field `CLK_MUX` writer - Выбор источника тактирования сенсора (Fin)"]
pub type ClkMuxW<'a, REG> = crate::FieldWriter<'a, REG, 3, ClkMux>;
impl<'a, REG> ClkMuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "системная частота (sys_clk)"]
    #[inline(always)]
    pub fn sys_clk(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMux::SysClk)
    }
    #[doc = "Частота шины AHB (hclk)"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMux::Hclk)
    }
    #[doc = "Частота внешнего осциллятора 32 МГц"]
    #[inline(always)]
    pub fn osc32m(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMux::Osc32m)
    }
    #[doc = "Частота HSI32M"]
    #[inline(always)]
    pub fn hsi32m(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMux::Hsi32m)
    }
    #[doc = "Частота внешнего осциллятора 32 кГц"]
    #[inline(always)]
    pub fn osc32k(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMux::Osc32k)
    }
    #[doc = "Частота LSI32K"]
    #[inline(always)]
    pub fn lsi32k(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMux::Lsi32k)
    }
}
#[doc = "Field `DIV` reader - Значение делителя тактового сигнала. Частота сенсора определяется как Tsens = Fin/(2*(Div + 1))"]
pub type DivR = crate::FieldReader<u16>;
#[doc = "Field `DIV` writer - Значение делителя тактового сигнала. Частота сенсора определяется как Tsens = Fin/(2*(Div + 1))"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0 - Управление питанием сенсора"]
    #[inline(always)]
    pub fn npd(&self) -> NpdR {
        NpdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Управление тактированием сенсора"]
    #[inline(always)]
    pub fn npd_clk(&self) -> NpdClkR {
        NpdClkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Управление сбросом сенсора, активный уровень «0»"]
    #[inline(always)]
    pub fn nrst(&self) -> NrstR {
        NrstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Выбор источника тактирования сенсора (Fin)"]
    #[inline(always)]
    pub fn clk_mux(&self) -> ClkMuxR {
        ClkMuxR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:15 - Значение делителя тактового сигнала. Частота сенсора определяется как Tsens = Fin/(2*(Div + 1))"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new(((self.bits >> 6) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Управление питанием сенсора"]
    #[inline(always)]
    pub fn npd(&mut self) -> NpdW<TsensCfgSpec> {
        NpdW::new(self, 0)
    }
    #[doc = "Bit 1 - Управление тактированием сенсора"]
    #[inline(always)]
    pub fn npd_clk(&mut self) -> NpdClkW<TsensCfgSpec> {
        NpdClkW::new(self, 1)
    }
    #[doc = "Bit 2 - Управление сбросом сенсора, активный уровень «0»"]
    #[inline(always)]
    pub fn nrst(&mut self) -> NrstW<TsensCfgSpec> {
        NrstW::new(self, 2)
    }
    #[doc = "Bits 3:5 - Выбор источника тактирования сенсора (Fin)"]
    #[inline(always)]
    pub fn clk_mux(&mut self) -> ClkMuxW<TsensCfgSpec> {
        ClkMuxW::new(self, 3)
    }
    #[doc = "Bits 6:15 - Значение делителя тактового сигнала. Частота сенсора определяется как Tsens = Fin/(2*(Div + 1))"]
    #[inline(always)]
    pub fn div(&mut self) -> DivW<TsensCfgSpec> {
        DivW::new(self, 6)
    }
}
#[doc = "Регистр настроек\n\nYou can [`read`](crate::Reg::read) this register and get [`tsens_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsens_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsensCfgSpec;
impl crate::RegisterSpec for TsensCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsens_cfg::R`](R) reader structure"]
impl crate::Readable for TsensCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`tsens_cfg::W`](W) writer structure"]
impl crate::Writable for TsensCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSENS_CFG to value 0"]
impl crate::Resettable for TsensCfgSpec {
    const RESET_VALUE: u32 = 0;
}
