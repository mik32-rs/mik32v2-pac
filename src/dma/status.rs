#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `CHANNEL_READY` reader - Статус состояния каналов: «1» - готов к работе; «0» - занят В режиме чтения текущего статуса (Current_valuе=1) возвращает: \\[0\\]
- состояния ошибки при чтении (0 – ошибки не было; 1 – была зафиксирована ошибка шины при чтении) \\[1\\]
- состояния ошибки при записи (0 – ошибки не было; 1 – была зафиксирована ошибка шины при записи) \\[3:2\\]
состояния статуса контроллера канала (2’b01 – канал в состоянии чтения; 2’b10 – канала в состоянии записи)"]
pub type ChannelReadyR = crate::FieldReader;
#[doc = "Field `CHANNEL1_IRQ` reader - Статус прерываний: «1» - есть прерывания; «0» - нет прерываний"]
pub type Channel1IrqR = crate::BitReader;
#[doc = "Field `CHANNEL2_IRQ` reader - Статус прерываний: «1» - есть прерывания; «0» - нет прерываний"]
pub type Channel2IrqR = crate::BitReader;
#[doc = "Field `CHANNEL3_IRQ` reader - Статус прерываний: «1» - есть прерывания; «0» - нет прерываний"]
pub type Channel3IrqR = crate::BitReader;
#[doc = "Field `CHANNEL4_IRQ` reader - Статус прерываний: «1» - есть прерывания; «0» - нет прерываний"]
pub type Channel4IrqR = crate::BitReader;
#[doc = "Field `CHANNEL1_BUS_ERROR` reader - Статус состояния каналов при ошибках на шине: «1» - есть ошибка; «0» - нет ошибки"]
pub type Channel1BusErrorR = crate::BitReader;
#[doc = "Field `CHANNEL2_BUS_ERROR` reader - Статус состояния каналов при ошибках на шине: «1» - есть ошибка; «0» - нет ошибки"]
pub type Channel2BusErrorR = crate::BitReader;
#[doc = "Field `CHANNEL3_BUS_ERROR` reader - Статус состояния каналов при ошибках на шине: «1» - есть ошибка; «0» - нет ошибки"]
pub type Channel3BusErrorR = crate::BitReader;
#[doc = "Field `CHANNEL4_BUS_ERROR` reader - Статус состояния каналов при ошибках на шине: «1» - есть ошибка; «0» - нет ошибки"]
pub type Channel4BusErrorR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Статус состояния каналов: «1» - готов к работе; «0» - занят В режиме чтения текущего статуса (Current_valuе=1) возвращает: \\[0\\]
- состояния ошибки при чтении (0 – ошибки не было; 1 – была зафиксирована ошибка шины при чтении) \\[1\\]
- состояния ошибки при записи (0 – ошибки не было; 1 – была зафиксирована ошибка шины при записи) \\[3:2\\]
состояния статуса контроллера канала (2’b01 – канал в состоянии чтения; 2’b10 – канала в состоянии записи)"]
    #[inline(always)]
    pub fn channel_ready(&self) -> ChannelReadyR {
        ChannelReadyR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Статус прерываний: «1» - есть прерывания; «0» - нет прерываний"]
    #[inline(always)]
    pub fn channel1_irq(&self) -> Channel1IrqR {
        Channel1IrqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Статус прерываний: «1» - есть прерывания; «0» - нет прерываний"]
    #[inline(always)]
    pub fn channel2_irq(&self) -> Channel2IrqR {
        Channel2IrqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Статус прерываний: «1» - есть прерывания; «0» - нет прерываний"]
    #[inline(always)]
    pub fn channel3_irq(&self) -> Channel3IrqR {
        Channel3IrqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Статус прерываний: «1» - есть прерывания; «0» - нет прерываний"]
    #[inline(always)]
    pub fn channel4_irq(&self) -> Channel4IrqR {
        Channel4IrqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Статус состояния каналов при ошибках на шине: «1» - есть ошибка; «0» - нет ошибки"]
    #[inline(always)]
    pub fn channel1_bus_error(&self) -> Channel1BusErrorR {
        Channel1BusErrorR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Статус состояния каналов при ошибках на шине: «1» - есть ошибка; «0» - нет ошибки"]
    #[inline(always)]
    pub fn channel2_bus_error(&self) -> Channel2BusErrorR {
        Channel2BusErrorR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Статус состояния каналов при ошибках на шине: «1» - есть ошибка; «0» - нет ошибки"]
    #[inline(always)]
    pub fn channel3_bus_error(&self) -> Channel3BusErrorR {
        Channel3BusErrorR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Статус состояния каналов при ошибках на шине: «1» - есть ошибка; «0» - нет ошибки"]
    #[inline(always)]
    pub fn channel4_bus_error(&self) -> Channel4BusErrorR {
        Channel4BusErrorR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Регистр прерываний и настройки контроллера\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
