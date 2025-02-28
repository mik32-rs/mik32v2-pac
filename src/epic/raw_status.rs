#[doc = "Register `RAW_STATUS` reader"]
pub type R = crate::R<RawStatusSpec>;
#[doc = "Field `Timer32_0` reader - Линия прерывания Timer32_0"]
pub type Timer32_0R = crate::BitReader;
#[doc = "Field `USART_0` reader - Линия прерывания USART_0"]
pub type Usart0R = crate::BitReader;
#[doc = "Field `USART_1` reader - Линия прерывания USART_1"]
pub type Usart1R = crate::BitReader;
#[doc = "Field `SPI_0` reader - Линия прерывания SPI_0"]
pub type Spi0R = crate::BitReader;
#[doc = "Field `SPI_1` reader - Линия прерывания SPI_1"]
pub type Spi1R = crate::BitReader;
#[doc = "Field `GPIO` reader - Линия прерывания GPIO"]
pub type GpioR = crate::BitReader;
#[doc = "Field `I2C_0` reader - Линия прерывания I2C_0"]
pub type I2c0R = crate::BitReader;
#[doc = "Field `I2C_1` reader - Линия прерывания I2C_1"]
pub type I2c1R = crate::BitReader;
#[doc = "Field `WDT` reader - Линия прерывания сторожевого таймера (WDT)"]
pub type WdtR = crate::BitReader;
#[doc = "Field `Timer16_0` reader - Линия прерывания Timer16_0"]
pub type Timer16_0R = crate::BitReader;
#[doc = "Field `Timer16_1` reader - Линия прерывания Timer16_1"]
pub type Timer16_1R = crate::BitReader;
#[doc = "Field `Timer16_2` reader - Линия прерывания Timer16_2"]
pub type Timer16_2R = crate::BitReader;
#[doc = "Field `Timer32_1` reader - Линия прерывания Timer32_1"]
pub type Timer32_1R = crate::BitReader;
#[doc = "Field `Timer32_2` reader - Линия прерывания Timer32_2"]
pub type Timer32_2R = crate::BitReader;
#[doc = "Field `SPIFI` reader - Линия прерывания SPIFI"]
pub type SpifiR = crate::BitReader;
#[doc = "Field `RTC` reader - Линия прерывания RTC"]
pub type RtcR = crate::BitReader;
#[doc = "Field `EEPROM` reader - Линия прерывания EEPROM"]
pub type EepromR = crate::BitReader;
#[doc = "Field `WDT_BUS_DOM3` reader - Линия прерывания сторожевого таймера шины (периферийные устройства)"]
pub type WdtBusDom3R = crate::BitReader;
#[doc = "Field `WDT_BUS_SPIFI` reader - Линия прерывания сторожевого таймера шины (SPIFI)"]
pub type WdtBusSpifiR = crate::BitReader;
#[doc = "Field `WDT_BUS_EEPROM` reader - Линия прерывания сторожевого таймера шины (EEPROM)"]
pub type WdtBusEepromR = crate::BitReader;
#[doc = "Field `DMA` reader - Линия прерывания ПДП"]
pub type DmaR = crate::BitReader;
#[doc = "Field `Frequency_monitor` reader - Линия прерывания монитора частоты"]
pub type FrequencyMonitorR = crate::BitReader;
#[doc = "Field `PVD_AVCC_under` reader - Линия прерывания монитора напряжения AVCC (ниже порога)"]
pub type PvdAvccUnderR = crate::BitReader;
#[doc = "Field `PVD_AVCC_over` reader - Линия прерывания монитора напряжения AVCC (выше порога)"]
pub type PvdAvccOverR = crate::BitReader;
#[doc = "Field `PVD_VCC_under` reader - Линия прерывания монитора напряжения VCC (ниже порога)"]
pub type PvdVccUnderR = crate::BitReader;
#[doc = "Field `PVD_VCC_over` reader - Линия прерывания монитора напряжения VCC (выше порога)"]
pub type PvdVccOverR = crate::BitReader;
#[doc = "Field `BATTERY_NON_GOOD` reader - Линия прерывания недостаточного напряжения батареи"]
pub type BatteryNonGoodR = crate::BitReader;
#[doc = "Field `BOR` reader - Линия прерывания BrouwnOut детектора"]
pub type BorR = crate::BitReader;
#[doc = "Field `TSENS` reader - Линия прерывания монитора температуры"]
pub type TsensR = crate::BitReader;
#[doc = "Field `ADC` reader - Линия прерывания АЦП"]
pub type AdcR = crate::BitReader;
#[doc = "Field `DAC0` reader - Линия прерывания ЦАП0"]
pub type Dac0R = crate::BitReader;
#[doc = "Field `DAC1` reader - Линия прерывания ЦАП1"]
pub type Dac1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Линия прерывания Timer32_0"]
    #[inline(always)]
    pub fn timer32_0(&self) -> Timer32_0R {
        Timer32_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Линия прерывания USART_0"]
    #[inline(always)]
    pub fn usart_0(&self) -> Usart0R {
        Usart0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Линия прерывания USART_1"]
    #[inline(always)]
    pub fn usart_1(&self) -> Usart1R {
        Usart1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Линия прерывания SPI_0"]
    #[inline(always)]
    pub fn spi_0(&self) -> Spi0R {
        Spi0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Линия прерывания SPI_1"]
    #[inline(always)]
    pub fn spi_1(&self) -> Spi1R {
        Spi1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Линия прерывания GPIO"]
    #[inline(always)]
    pub fn gpio(&self) -> GpioR {
        GpioR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Линия прерывания I2C_0"]
    #[inline(always)]
    pub fn i2c_0(&self) -> I2c0R {
        I2c0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Линия прерывания I2C_1"]
    #[inline(always)]
    pub fn i2c_1(&self) -> I2c1R {
        I2c1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Линия прерывания сторожевого таймера (WDT)"]
    #[inline(always)]
    pub fn wdt(&self) -> WdtR {
        WdtR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Линия прерывания Timer16_0"]
    #[inline(always)]
    pub fn timer16_0(&self) -> Timer16_0R {
        Timer16_0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Линия прерывания Timer16_1"]
    #[inline(always)]
    pub fn timer16_1(&self) -> Timer16_1R {
        Timer16_1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Линия прерывания Timer16_2"]
    #[inline(always)]
    pub fn timer16_2(&self) -> Timer16_2R {
        Timer16_2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Линия прерывания Timer32_1"]
    #[inline(always)]
    pub fn timer32_1(&self) -> Timer32_1R {
        Timer32_1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Линия прерывания Timer32_2"]
    #[inline(always)]
    pub fn timer32_2(&self) -> Timer32_2R {
        Timer32_2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Линия прерывания SPIFI"]
    #[inline(always)]
    pub fn spifi(&self) -> SpifiR {
        SpifiR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Линия прерывания RTC"]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Линия прерывания EEPROM"]
    #[inline(always)]
    pub fn eeprom(&self) -> EepromR {
        EepromR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Линия прерывания сторожевого таймера шины (периферийные устройства)"]
    #[inline(always)]
    pub fn wdt_bus_dom3(&self) -> WdtBusDom3R {
        WdtBusDom3R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Линия прерывания сторожевого таймера шины (SPIFI)"]
    #[inline(always)]
    pub fn wdt_bus_spifi(&self) -> WdtBusSpifiR {
        WdtBusSpifiR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Линия прерывания сторожевого таймера шины (EEPROM)"]
    #[inline(always)]
    pub fn wdt_bus_eeprom(&self) -> WdtBusEepromR {
        WdtBusEepromR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Линия прерывания ПДП"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Линия прерывания монитора частоты"]
    #[inline(always)]
    pub fn frequency_monitor(&self) -> FrequencyMonitorR {
        FrequencyMonitorR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Линия прерывания монитора напряжения AVCC (ниже порога)"]
    #[inline(always)]
    pub fn pvd_avcc_under(&self) -> PvdAvccUnderR {
        PvdAvccUnderR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Линия прерывания монитора напряжения AVCC (выше порога)"]
    #[inline(always)]
    pub fn pvd_avcc_over(&self) -> PvdAvccOverR {
        PvdAvccOverR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Линия прерывания монитора напряжения VCC (ниже порога)"]
    #[inline(always)]
    pub fn pvd_vcc_under(&self) -> PvdVccUnderR {
        PvdVccUnderR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Линия прерывания монитора напряжения VCC (выше порога)"]
    #[inline(always)]
    pub fn pvd_vcc_over(&self) -> PvdVccOverR {
        PvdVccOverR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Линия прерывания недостаточного напряжения батареи"]
    #[inline(always)]
    pub fn battery_non_good(&self) -> BatteryNonGoodR {
        BatteryNonGoodR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Линия прерывания BrouwnOut детектора"]
    #[inline(always)]
    pub fn bor(&self) -> BorR {
        BorR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Линия прерывания монитора температуры"]
    #[inline(always)]
    pub fn tsens(&self) -> TsensR {
        TsensR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Линия прерывания АЦП"]
    #[inline(always)]
    pub fn adc(&self) -> AdcR {
        AdcR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Линия прерывания ЦАП0"]
    #[inline(always)]
    pub fn dac0(&self) -> Dac0R {
        Dac0R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Линия прерывания ЦАП1"]
    #[inline(always)]
    pub fn dac1(&self) -> Dac1R {
        Dac1R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Текущее состоянии линий прерываний\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawStatusSpec;
impl crate::RegisterSpec for RawStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_status::R`](R) reader structure"]
impl crate::Readable for RawStatusSpec {}
#[doc = "`reset()` method sets RAW_STATUS to value 0"]
impl crate::Resettable for RawStatusSpec {
    const RESET_VALUE: u32 = 0;
}
