#[doc = "Register `MASK_LEVEL_SET` writer"]
pub type W = crate::W<MaskLevelSetSpec>;
#[doc = "Field `Timer32_0` writer - Линия прерывания Timer32_0"]
pub type Timer32_0W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `USART_0` writer - Линия прерывания USART_0"]
pub type Usart0W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `USART_1` writer - Линия прерывания USART_1"]
pub type Usart1W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SPI_0` writer - Линия прерывания SPI_0"]
pub type Spi0W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SPI_1` writer - Линия прерывания SPI_1"]
pub type Spi1W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `GPIO` writer - Линия прерывания GPIO"]
pub type GpioW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `I2C_0` writer - Линия прерывания I2C_0"]
pub type I2c0W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `I2C_1` writer - Линия прерывания I2C_1"]
pub type I2c1W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `WDT` writer - Линия прерывания сторожевого таймера (WDT)"]
pub type WdtW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `Timer16_0` writer - Линия прерывания Timer16_0"]
pub type Timer16_0W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `Timer16_1` writer - Линия прерывания Timer16_1"]
pub type Timer16_1W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `Timer16_2` writer - Линия прерывания Timer16_2"]
pub type Timer16_2W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `Timer32_1` writer - Линия прерывания Timer32_1"]
pub type Timer32_1W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `Timer32_2` writer - Линия прерывания Timer32_2"]
pub type Timer32_2W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SPIFI` writer - Линия прерывания SPIFI"]
pub type SpifiW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `RTC` writer - Линия прерывания RTC"]
pub type RtcW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `EEPROM` writer - Линия прерывания EEPROM"]
pub type EepromW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `WDT_BUS_DOM3` writer - Линия прерывания сторожевого таймера шины (периферийные устройства)"]
pub type WdtBusDom3W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `WDT_BUS_SPIFI` writer - Линия прерывания сторожевого таймера шины (SPIFI)"]
pub type WdtBusSpifiW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `WDT_BUS_EEPROM` writer - Линия прерывания сторожевого таймера шины (EEPROM)"]
pub type WdtBusEepromW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `DMA` writer - Линия прерывания ПДП"]
pub type DmaW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `Frequency_monitor` writer - Линия прерывания монитора частоты"]
pub type FrequencyMonitorW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `PVD_AVCC_under` writer - Линия прерывания монитора напряжения AVCC (ниже порога)"]
pub type PvdAvccUnderW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `PVD_AVCC_over` writer - Линия прерывания монитора напряжения AVCC (выше порога)"]
pub type PvdAvccOverW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `PVD_VCC_under` writer - Линия прерывания монитора напряжения VCC (ниже порога)"]
pub type PvdVccUnderW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `PVD_VCC_over` writer - Линия прерывания монитора напряжения VCC (выше порога)"]
pub type PvdVccOverW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `BATTERY_NON_GOOD` writer - Линия прерывания недостаточного напряжения батареи"]
pub type BatteryNonGoodW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `BOR` writer - Линия прерывания BrouwnOut детектора"]
pub type BorW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `TSENS` writer - Линия прерывания монитора температуры"]
pub type TsensW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `ADC` writer - Линия прерывания АЦП"]
pub type AdcW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `DAC0` writer - Линия прерывания ЦАП0"]
pub type Dac0W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `DAC1` writer - Линия прерывания ЦАП1"]
pub type Dac1W<'a, REG> = crate::BitWriter1S<'a, REG>;
impl W {
    #[doc = "Bit 0 - Линия прерывания Timer32_0"]
    #[inline(always)]
    pub fn timer32_0(&mut self) -> Timer32_0W<MaskLevelSetSpec> {
        Timer32_0W::new(self, 0)
    }
    #[doc = "Bit 1 - Линия прерывания USART_0"]
    #[inline(always)]
    pub fn usart_0(&mut self) -> Usart0W<MaskLevelSetSpec> {
        Usart0W::new(self, 1)
    }
    #[doc = "Bit 2 - Линия прерывания USART_1"]
    #[inline(always)]
    pub fn usart_1(&mut self) -> Usart1W<MaskLevelSetSpec> {
        Usart1W::new(self, 2)
    }
    #[doc = "Bit 3 - Линия прерывания SPI_0"]
    #[inline(always)]
    pub fn spi_0(&mut self) -> Spi0W<MaskLevelSetSpec> {
        Spi0W::new(self, 3)
    }
    #[doc = "Bit 4 - Линия прерывания SPI_1"]
    #[inline(always)]
    pub fn spi_1(&mut self) -> Spi1W<MaskLevelSetSpec> {
        Spi1W::new(self, 4)
    }
    #[doc = "Bit 5 - Линия прерывания GPIO"]
    #[inline(always)]
    pub fn gpio(&mut self) -> GpioW<MaskLevelSetSpec> {
        GpioW::new(self, 5)
    }
    #[doc = "Bit 6 - Линия прерывания I2C_0"]
    #[inline(always)]
    pub fn i2c_0(&mut self) -> I2c0W<MaskLevelSetSpec> {
        I2c0W::new(self, 6)
    }
    #[doc = "Bit 7 - Линия прерывания I2C_1"]
    #[inline(always)]
    pub fn i2c_1(&mut self) -> I2c1W<MaskLevelSetSpec> {
        I2c1W::new(self, 7)
    }
    #[doc = "Bit 8 - Линия прерывания сторожевого таймера (WDT)"]
    #[inline(always)]
    pub fn wdt(&mut self) -> WdtW<MaskLevelSetSpec> {
        WdtW::new(self, 8)
    }
    #[doc = "Bit 9 - Линия прерывания Timer16_0"]
    #[inline(always)]
    pub fn timer16_0(&mut self) -> Timer16_0W<MaskLevelSetSpec> {
        Timer16_0W::new(self, 9)
    }
    #[doc = "Bit 10 - Линия прерывания Timer16_1"]
    #[inline(always)]
    pub fn timer16_1(&mut self) -> Timer16_1W<MaskLevelSetSpec> {
        Timer16_1W::new(self, 10)
    }
    #[doc = "Bit 11 - Линия прерывания Timer16_2"]
    #[inline(always)]
    pub fn timer16_2(&mut self) -> Timer16_2W<MaskLevelSetSpec> {
        Timer16_2W::new(self, 11)
    }
    #[doc = "Bit 12 - Линия прерывания Timer32_1"]
    #[inline(always)]
    pub fn timer32_1(&mut self) -> Timer32_1W<MaskLevelSetSpec> {
        Timer32_1W::new(self, 12)
    }
    #[doc = "Bit 13 - Линия прерывания Timer32_2"]
    #[inline(always)]
    pub fn timer32_2(&mut self) -> Timer32_2W<MaskLevelSetSpec> {
        Timer32_2W::new(self, 13)
    }
    #[doc = "Bit 14 - Линия прерывания SPIFI"]
    #[inline(always)]
    pub fn spifi(&mut self) -> SpifiW<MaskLevelSetSpec> {
        SpifiW::new(self, 14)
    }
    #[doc = "Bit 15 - Линия прерывания RTC"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RtcW<MaskLevelSetSpec> {
        RtcW::new(self, 15)
    }
    #[doc = "Bit 16 - Линия прерывания EEPROM"]
    #[inline(always)]
    pub fn eeprom(&mut self) -> EepromW<MaskLevelSetSpec> {
        EepromW::new(self, 16)
    }
    #[doc = "Bit 17 - Линия прерывания сторожевого таймера шины (периферийные устройства)"]
    #[inline(always)]
    pub fn wdt_bus_dom3(&mut self) -> WdtBusDom3W<MaskLevelSetSpec> {
        WdtBusDom3W::new(self, 17)
    }
    #[doc = "Bit 18 - Линия прерывания сторожевого таймера шины (SPIFI)"]
    #[inline(always)]
    pub fn wdt_bus_spifi(&mut self) -> WdtBusSpifiW<MaskLevelSetSpec> {
        WdtBusSpifiW::new(self, 18)
    }
    #[doc = "Bit 19 - Линия прерывания сторожевого таймера шины (EEPROM)"]
    #[inline(always)]
    pub fn wdt_bus_eeprom(&mut self) -> WdtBusEepromW<MaskLevelSetSpec> {
        WdtBusEepromW::new(self, 19)
    }
    #[doc = "Bit 20 - Линия прерывания ПДП"]
    #[inline(always)]
    pub fn dma(&mut self) -> DmaW<MaskLevelSetSpec> {
        DmaW::new(self, 20)
    }
    #[doc = "Bit 21 - Линия прерывания монитора частоты"]
    #[inline(always)]
    pub fn frequency_monitor(&mut self) -> FrequencyMonitorW<MaskLevelSetSpec> {
        FrequencyMonitorW::new(self, 21)
    }
    #[doc = "Bit 22 - Линия прерывания монитора напряжения AVCC (ниже порога)"]
    #[inline(always)]
    pub fn pvd_avcc_under(&mut self) -> PvdAvccUnderW<MaskLevelSetSpec> {
        PvdAvccUnderW::new(self, 22)
    }
    #[doc = "Bit 23 - Линия прерывания монитора напряжения AVCC (выше порога)"]
    #[inline(always)]
    pub fn pvd_avcc_over(&mut self) -> PvdAvccOverW<MaskLevelSetSpec> {
        PvdAvccOverW::new(self, 23)
    }
    #[doc = "Bit 24 - Линия прерывания монитора напряжения VCC (ниже порога)"]
    #[inline(always)]
    pub fn pvd_vcc_under(&mut self) -> PvdVccUnderW<MaskLevelSetSpec> {
        PvdVccUnderW::new(self, 24)
    }
    #[doc = "Bit 25 - Линия прерывания монитора напряжения VCC (выше порога)"]
    #[inline(always)]
    pub fn pvd_vcc_over(&mut self) -> PvdVccOverW<MaskLevelSetSpec> {
        PvdVccOverW::new(self, 25)
    }
    #[doc = "Bit 26 - Линия прерывания недостаточного напряжения батареи"]
    #[inline(always)]
    pub fn battery_non_good(&mut self) -> BatteryNonGoodW<MaskLevelSetSpec> {
        BatteryNonGoodW::new(self, 26)
    }
    #[doc = "Bit 27 - Линия прерывания BrouwnOut детектора"]
    #[inline(always)]
    pub fn bor(&mut self) -> BorW<MaskLevelSetSpec> {
        BorW::new(self, 27)
    }
    #[doc = "Bit 28 - Линия прерывания монитора температуры"]
    #[inline(always)]
    pub fn tsens(&mut self) -> TsensW<MaskLevelSetSpec> {
        TsensW::new(self, 28)
    }
    #[doc = "Bit 29 - Линия прерывания АЦП"]
    #[inline(always)]
    pub fn adc(&mut self) -> AdcW<MaskLevelSetSpec> {
        AdcW::new(self, 29)
    }
    #[doc = "Bit 30 - Линия прерывания ЦАП0"]
    #[inline(always)]
    pub fn dac0(&mut self) -> Dac0W<MaskLevelSetSpec> {
        Dac0W::new(self, 30)
    }
    #[doc = "Bit 31 - Линия прерывания ЦАП1"]
    #[inline(always)]
    pub fn dac1(&mut self) -> Dac1W<MaskLevelSetSpec> {
        Dac1W::new(self, 31)
    }
}
#[doc = "Установка маски прерываний по уровню\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_level_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaskLevelSetSpec;
impl crate::RegisterSpec for MaskLevelSetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mask_level_set::W`](W) writer structure"]
impl crate::Writable for MaskLevelSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets MASK_LEVEL_SET to value 0"]
impl crate::Resettable for MaskLevelSetSpec {
    const RESET_VALUE: u32 = 0;
}
