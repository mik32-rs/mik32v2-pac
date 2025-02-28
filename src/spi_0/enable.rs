#[doc = "Register `ENABLE` reader"]
pub type R = crate::R<EnableSpec>;
#[doc = "Register `ENABLE` writer"]
pub type W = crate::W<EnableSpec>;
#[doc = "(ENABLE) Включение/выключение модуля SPI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpiEn {
    #[doc = "0: Выключение модуля SPI"]
    Disable = 0,
    #[doc = "1: Включение модуля SPI"]
    Enable = 1,
}
impl From<SpiEn> for bool {
    #[inline(always)]
    fn from(variant: SpiEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI_EN` reader - (ENABLE) Включение/выключение модуля SPI"]
pub type SpiEnR = crate::BitReader<SpiEn>;
impl SpiEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpiEn {
        match self.bits {
            false => SpiEn::Disable,
            true => SpiEn::Enable,
        }
    }
    #[doc = "Выключение модуля SPI"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SpiEn::Disable
    }
    #[doc = "Включение модуля SPI"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SpiEn::Enable
    }
}
#[doc = "Field `SPI_EN` writer - (ENABLE) Включение/выключение модуля SPI"]
pub type SpiEnW<'a, REG> = crate::BitWriter<'a, REG, SpiEn>;
impl<'a, REG> SpiEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Выключение модуля SPI"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SpiEn::Disable)
    }
    #[doc = "Включение модуля SPI"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SpiEn::Enable)
    }
}
#[doc = "Field `CLEAR_TX_FIFO` writer - Запись 1 при SPI_EN = 0 очищает буфер TX_FIFO."]
pub type ClearTxFifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_PX_FIFO` writer - Запись 1 при SPI_EN = 0 очищает буфер RX_FIFO."]
pub type ClearPxFifoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - (ENABLE) Включение/выключение модуля SPI"]
    #[inline(always)]
    pub fn spi_en(&self) -> SpiEnR {
        SpiEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - (ENABLE) Включение/выключение модуля SPI"]
    #[inline(always)]
    pub fn spi_en(&mut self) -> SpiEnW<EnableSpec> {
        SpiEnW::new(self, 0)
    }
    #[doc = "Bit 2 - Запись 1 при SPI_EN = 0 очищает буфер TX_FIFO."]
    #[inline(always)]
    pub fn clear_tx_fifo(&mut self) -> ClearTxFifoW<EnableSpec> {
        ClearTxFifoW::new(self, 2)
    }
    #[doc = "Bit 3 - Запись 1 при SPI_EN = 0 очищает буфер RX_FIFO."]
    #[inline(always)]
    pub fn clear_px_fifo(&mut self) -> ClearPxFifoW<EnableSpec> {
        ClearPxFifoW::new(self, 3)
    }
}
#[doc = "Регистр включения/выключения SPI\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnableSpec;
impl crate::RegisterSpec for EnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable::R`](R) reader structure"]
impl crate::Readable for EnableSpec {}
#[doc = "`write(|w| ..)` method takes [`enable::W`](W) writer structure"]
impl crate::Writable for EnableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENABLE to value 0"]
impl crate::Resettable for EnableSpec {
    const RESET_VALUE: u32 = 0;
}
