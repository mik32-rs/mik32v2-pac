#[doc = "Register `CLOCKS_SYS` reader"]
pub type R = crate::R<ClocksSysSpec>;
#[doc = "Register `CLOCKS_SYS` writer"]
pub type W = crate::W<ClocksSysSpec>;
#[doc = "Включение/отключение внешнего осциллятора на 32 МГц\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Osc32mEn {
    #[doc = "0: Включение OSC32M"]
    Enable = 0,
    #[doc = "1: Отключение OSC32M"]
    Disable = 1,
}
impl From<Osc32mEn> for bool {
    #[inline(always)]
    fn from(variant: Osc32mEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSC32M_EN` reader - Включение/отключение внешнего осциллятора на 32 МГц"]
pub type Osc32mEnR = crate::BitReader<Osc32mEn>;
impl Osc32mEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Osc32mEn {
        match self.bits {
            false => Osc32mEn::Enable,
            true => Osc32mEn::Disable,
        }
    }
    #[doc = "Включение OSC32M"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Osc32mEn::Enable
    }
    #[doc = "Отключение OSC32M"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Osc32mEn::Disable
    }
}
#[doc = "Field `OSC32M_EN` writer - Включение/отключение внешнего осциллятора на 32 МГц"]
pub type Osc32mEnW<'a, REG> = crate::BitWriter<'a, REG, Osc32mEn>;
impl<'a, REG> Osc32mEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Включение OSC32M"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Osc32mEn::Enable)
    }
    #[doc = "Отключение OSC32M"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Osc32mEn::Disable)
    }
}
#[doc = "Включение/отключение HSI32M\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hsi32mEn {
    #[doc = "0: Включение HSI32M"]
    Enable = 0,
    #[doc = "1: Отключение HSI32M"]
    Disable = 1,
}
impl From<Hsi32mEn> for bool {
    #[inline(always)]
    fn from(variant: Hsi32mEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSI32M_EN` reader - Включение/отключение HSI32M"]
pub type Hsi32mEnR = crate::BitReader<Hsi32mEn>;
impl Hsi32mEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hsi32mEn {
        match self.bits {
            false => Hsi32mEn::Enable,
            true => Hsi32mEn::Disable,
        }
    }
    #[doc = "Включение HSI32M"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hsi32mEn::Enable
    }
    #[doc = "Отключение HSI32M"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hsi32mEn::Disable
    }
}
#[doc = "Field `HSI32M_EN` writer - Включение/отключение HSI32M"]
pub type Hsi32mEnW<'a, REG> = crate::BitWriter<'a, REG, Hsi32mEn>;
impl<'a, REG> Hsi32mEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Включение HSI32M"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hsi32mEn::Enable)
    }
    #[doc = "Отключение HSI32M"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hsi32mEn::Disable)
    }
}
#[doc = "Field `ADJ_HSI32M` reader - Поправочные коэффициенты HSI32M"]
pub type AdjHsi32mR = crate::FieldReader;
#[doc = "Field `ADJ_HSI32M` writer - Поправочные коэффициенты HSI32M"]
pub type AdjHsi32mW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Принудительное переключение на опорный источник для монитора частоты\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Force32kClk {
    #[doc = "0: Автоматический выбор"]
    Automatic = 0,
    #[doc = "1: Принудительно выбран LSI32K"]
    Lsi32k = 1,
    #[doc = "2: Принудительно выбран OSC32K"]
    Osc32k = 2,
    // #[doc = "3: Автоматический выбор"]
    // Automatic = 3,
}
impl From<Force32kClk> for u8 {
    #[inline(always)]
    fn from(variant: Force32kClk) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Force32kClk {
    type Ux = u8;
}
impl crate::IsEnum for Force32kClk {}
#[doc = "Field `FORCE_32K_CLK` reader - Принудительное переключение на опорный источник для монитора частоты"]
pub type Force32kClkR = crate::FieldReader<Force32kClk>;
impl Force32kClkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Force32kClk {
        match self.bits {
            0 => Force32kClk::Automatic,
            1 => Force32kClk::Lsi32k,
            2 => Force32kClk::Osc32k,
            3 => Force32kClk::Automatic,
            _ => unreachable!(),
        }
    }
    #[doc = "Автоматический выбор"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == Force32kClk::Automatic
    }
    #[doc = "Принудительно выбран LSI32K"]
    #[inline(always)]
    pub fn is_lsi32k(&self) -> bool {
        *self == Force32kClk::Lsi32k
    }
    #[doc = "Принудительно выбран OSC32K"]
    #[inline(always)]
    pub fn is_osc32k(&self) -> bool {
        *self == Force32kClk::Osc32k
    }
}
#[doc = "Field `FORCE_32K_CLK` writer - Принудительное переключение на опорный источник для монитора частоты"]
pub type Force32kClkW<'a, REG> = crate::FieldWriter<'a, REG, 2, Force32kClk, crate::Safe>;
impl<'a, REG> Force32kClkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Автоматический выбор"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut crate::W<REG> {
        self.variant(Force32kClk::Automatic)
    }
    #[doc = "Принудительно выбран LSI32K"]
    #[inline(always)]
    pub fn lsi32k(self) -> &'a mut crate::W<REG> {
        self.variant(Force32kClk::Lsi32k)
    }
    #[doc = "Принудительно выбран OSC32K"]
    #[inline(always)]
    pub fn osc32k(self) -> &'a mut crate::W<REG> {
        self.variant(Force32kClk::Osc32k)
    }
}
impl R {
    #[doc = "Bit 0 - Включение/отключение внешнего осциллятора на 32 МГц"]
    #[inline(always)]
    pub fn osc32m_en(&self) -> Osc32mEnR {
        Osc32mEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Включение/отключение HSI32M"]
    #[inline(always)]
    pub fn hsi32m_en(&self) -> Hsi32mEnR {
        Hsi32mEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:9 - Поправочные коэффициенты HSI32M"]
    #[inline(always)]
    pub fn adj_hsi32m(&self) -> AdjHsi32mR {
        AdjHsi32mR::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bits 10:11 - Принудительное переключение на опорный источник для монитора частоты"]
    #[inline(always)]
    pub fn force_32k_clk(&self) -> Force32kClkR {
        Force32kClkR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Включение/отключение внешнего осциллятора на 32 МГц"]
    #[inline(always)]
    pub fn osc32m_en(&mut self) -> Osc32mEnW<ClocksSysSpec> {
        Osc32mEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Включение/отключение HSI32M"]
    #[inline(always)]
    pub fn hsi32m_en(&mut self) -> Hsi32mEnW<ClocksSysSpec> {
        Hsi32mEnW::new(self, 1)
    }
    #[doc = "Bits 2:9 - Поправочные коэффициенты HSI32M"]
    #[inline(always)]
    pub fn adj_hsi32m(&mut self) -> AdjHsi32mW<ClocksSysSpec> {
        AdjHsi32mW::new(self, 2)
    }
    #[doc = "Bits 10:11 - Принудительное переключение на опорный источник для монитора частоты"]
    #[inline(always)]
    pub fn force_32k_clk(&mut self) -> Force32kClkW<ClocksSysSpec> {
        Force32kClkW::new(self, 10)
    }
}
#[doc = "Регистр управления тактированием системного домена\n\nYou can [`read`](crate::Reg::read) this register and get [`clocks_sys::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clocks_sys::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClocksSysSpec;
impl crate::RegisterSpec for ClocksSysSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clocks_sys::R`](R) reader structure"]
impl crate::Readable for ClocksSysSpec {}
#[doc = "`write(|w| ..)` method takes [`clocks_sys::W`](W) writer structure"]
impl crate::Writable for ClocksSysSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLOCKS_SYS to value 0x0200"]
impl crate::Resettable for ClocksSysSpec {
    const RESET_VALUE: u32 = 0x0200;
}
