#[doc = "Register `FREQ_MASK` reader"]
pub type R = crate::R<FreqMaskSpec>;
#[doc = "Register `FREQ_MASK` writer"]
pub type W = crate::W<FreqMaskSpec>;
#[doc = "Разрешение прерывания при пропадании частоты LSI32K\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaskLsi32k {
    #[doc = "0: Запретить прерывание"]
    Disable = 0,
    #[doc = "1: Разрешить прерывание"]
    Enable = 1,
}
impl From<MaskLsi32k> for bool {
    #[inline(always)]
    fn from(variant: MaskLsi32k) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK_LSI32K` reader - Разрешение прерывания при пропадании частоты LSI32K"]
pub type MaskLsi32kR = crate::BitReader<MaskLsi32k>;
impl MaskLsi32kR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MaskLsi32k {
        match self.bits {
            false => MaskLsi32k::Disable,
            true => MaskLsi32k::Enable,
        }
    }
    #[doc = "Запретить прерывание"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MaskLsi32k::Disable
    }
    #[doc = "Разрешить прерывание"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MaskLsi32k::Enable
    }
}
#[doc = "Field `MASK_LSI32K` writer - Разрешение прерывания при пропадании частоты LSI32K"]
pub type MaskLsi32kW<'a, REG> = crate::BitWriter<'a, REG, MaskLsi32k>;
impl<'a, REG> MaskLsi32kW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Запретить прерывание"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(MaskLsi32k::Disable)
    }
    #[doc = "Разрешить прерывание"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(MaskLsi32k::Enable)
    }
}
#[doc = "Разрешение прерывания при пропадании частоты внешнего осциллятора 32 кГц\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaskOsc32k {
    #[doc = "0: Запретить прерывание"]
    Disable = 0,
    #[doc = "1: Разрешить прерывание"]
    Enable = 1,
}
impl From<MaskOsc32k> for bool {
    #[inline(always)]
    fn from(variant: MaskOsc32k) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK_OSC32K` reader - Разрешение прерывания при пропадании частоты внешнего осциллятора 32 кГц"]
pub type MaskOsc32kR = crate::BitReader<MaskOsc32k>;
impl MaskOsc32kR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MaskOsc32k {
        match self.bits {
            false => MaskOsc32k::Disable,
            true => MaskOsc32k::Enable,
        }
    }
    #[doc = "Запретить прерывание"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MaskOsc32k::Disable
    }
    #[doc = "Разрешить прерывание"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MaskOsc32k::Enable
    }
}
#[doc = "Field `MASK_OSC32K` writer - Разрешение прерывания при пропадании частоты внешнего осциллятора 32 кГц"]
pub type MaskOsc32kW<'a, REG> = crate::BitWriter<'a, REG, MaskOsc32k>;
impl<'a, REG> MaskOsc32kW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Запретить прерывание"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(MaskOsc32k::Disable)
    }
    #[doc = "Разрешить прерывание"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(MaskOsc32k::Enable)
    }
}
#[doc = "Разрешение прерывания при пропадании частоты HSI32M\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaskHsi32m {
    #[doc = "0: Запретить прерывание"]
    Disable = 0,
    #[doc = "1: Разрешить прерывание"]
    Enable = 1,
}
impl From<MaskHsi32m> for bool {
    #[inline(always)]
    fn from(variant: MaskHsi32m) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK_HSI32M` reader - Разрешение прерывания при пропадании частоты HSI32M"]
pub type MaskHsi32mR = crate::BitReader<MaskHsi32m>;
impl MaskHsi32mR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MaskHsi32m {
        match self.bits {
            false => MaskHsi32m::Disable,
            true => MaskHsi32m::Enable,
        }
    }
    #[doc = "Запретить прерывание"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MaskHsi32m::Disable
    }
    #[doc = "Разрешить прерывание"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MaskHsi32m::Enable
    }
}
#[doc = "Field `MASK_HSI32M` writer - Разрешение прерывания при пропадании частоты HSI32M"]
pub type MaskHsi32mW<'a, REG> = crate::BitWriter<'a, REG, MaskHsi32m>;
impl<'a, REG> MaskHsi32mW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Запретить прерывание"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(MaskHsi32m::Disable)
    }
    #[doc = "Разрешить прерывание"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(MaskHsi32m::Enable)
    }
}
#[doc = "Разрешение прерывания при пропадании частоты внешнего осциллятора 32 МГц\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaskOsc32m {
    #[doc = "0: Запретить прерывание"]
    Disable = 0,
    #[doc = "1: Разрешить прерывание"]
    Enable = 1,
}
impl From<MaskOsc32m> for bool {
    #[inline(always)]
    fn from(variant: MaskOsc32m) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK_OSC32M` reader - Разрешение прерывания при пропадании частоты внешнего осциллятора 32 МГц"]
pub type MaskOsc32mR = crate::BitReader<MaskOsc32m>;
impl MaskOsc32mR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MaskOsc32m {
        match self.bits {
            false => MaskOsc32m::Disable,
            true => MaskOsc32m::Enable,
        }
    }
    #[doc = "Запретить прерывание"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MaskOsc32m::Disable
    }
    #[doc = "Разрешить прерывание"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MaskOsc32m::Enable
    }
}
#[doc = "Field `MASK_OSC32M` writer - Разрешение прерывания при пропадании частоты внешнего осциллятора 32 МГц"]
pub type MaskOsc32mW<'a, REG> = crate::BitWriter<'a, REG, MaskOsc32m>;
impl<'a, REG> MaskOsc32mW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Запретить прерывание"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(MaskOsc32m::Disable)
    }
    #[doc = "Разрешить прерывание"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(MaskOsc32m::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Разрешение прерывания при пропадании частоты LSI32K"]
    #[inline(always)]
    pub fn mask_lsi32k(&self) -> MaskLsi32kR {
        MaskLsi32kR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Разрешение прерывания при пропадании частоты внешнего осциллятора 32 кГц"]
    #[inline(always)]
    pub fn mask_osc32k(&self) -> MaskOsc32kR {
        MaskOsc32kR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Разрешение прерывания при пропадании частоты HSI32M"]
    #[inline(always)]
    pub fn mask_hsi32m(&self) -> MaskHsi32mR {
        MaskHsi32mR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Разрешение прерывания при пропадании частоты внешнего осциллятора 32 МГц"]
    #[inline(always)]
    pub fn mask_osc32m(&self) -> MaskOsc32mR {
        MaskOsc32mR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Разрешение прерывания при пропадании частоты LSI32K"]
    #[inline(always)]
    pub fn mask_lsi32k(&mut self) -> MaskLsi32kW<FreqMaskSpec> {
        MaskLsi32kW::new(self, 0)
    }
    #[doc = "Bit 1 - Разрешение прерывания при пропадании частоты внешнего осциллятора 32 кГц"]
    #[inline(always)]
    pub fn mask_osc32k(&mut self) -> MaskOsc32kW<FreqMaskSpec> {
        MaskOsc32kW::new(self, 1)
    }
    #[doc = "Bit 2 - Разрешение прерывания при пропадании частоты HSI32M"]
    #[inline(always)]
    pub fn mask_hsi32m(&mut self) -> MaskHsi32mW<FreqMaskSpec> {
        MaskHsi32mW::new(self, 2)
    }
    #[doc = "Bit 3 - Разрешение прерывания при пропадании частоты внешнего осциллятора 32 МГц"]
    #[inline(always)]
    pub fn mask_osc32m(&mut self) -> MaskOsc32mW<FreqMaskSpec> {
        MaskOsc32mW::new(self, 3)
    }
}
#[doc = "Настройки прерываний монитора частоты\n\nYou can [`read`](crate::Reg::read) this register and get [`freq_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`freq_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FreqMaskSpec;
impl crate::RegisterSpec for FreqMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`freq_mask::R`](R) reader structure"]
impl crate::Readable for FreqMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`freq_mask::W`](W) writer structure"]
impl crate::Writable for FreqMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FREQ_MASK to value 0"]
impl crate::Resettable for FreqMaskSpec {
    const RESET_VALUE: u32 = 0;
}
