#[doc = "Register `PAD1_CFG` reader"]
pub type R = crate::R<Pad1CfgSpec>;
#[doc = "Register `PAD1_CFG` writer"]
pub type W = crate::W<Pad1CfgSpec>;
#[doc = "Значения двух бит кодируют выбранный функционал для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port1_0 {
    #[doc = "0: Первая функция (порт общего назначения)"]
    Func1Gpio = 0,
    #[doc = "1: Вторая функция (последовательный интерфейс)"]
    Func2Interface = 1,
    #[doc = "2: Третья функция (последовательный интерфейс или таймер)"]
    Func3InterfaceOrTimer = 2,
    #[doc = "3: Четвертая функция (аналоговый сигнал)"]
    Func4Analog = 3,
}
impl From<Port1_0> for u8 {
    #[inline(always)]
    fn from(variant: Port1_0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port1_0 {
    type Ux = u8;
}
impl crate::IsEnum for Port1_0 {}
#[doc = "Field `Port_1_0` reader - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_0R = crate::FieldReader<Port1_0>;
impl Port1_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Port1_0 {
        match self.bits {
            0 => Port1_0::Func1Gpio,
            1 => Port1_0::Func2Interface,
            2 => Port1_0::Func3InterfaceOrTimer,
            3 => Port1_0::Func4Analog,
            _ => unreachable!(),
        }
    }
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn is_func1_gpio(&self) -> bool {
        *self == Port1_0::Func1Gpio
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn is_func2_interface(&self) -> bool {
        *self == Port1_0::Func2Interface
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn is_func3_interface_or_timer(&self) -> bool {
        *self == Port1_0::Func3InterfaceOrTimer
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn is_func4_analog(&self) -> bool {
        *self == Port1_0::Func4Analog
    }
}
#[doc = "Field `Port_1_0` writer - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port1_0, crate::Safe>;
impl<'a, REG> Port1_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn func1_gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_0::Func1Gpio)
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn func2_interface(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_0::Func2Interface)
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn func3_interface_or_timer(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_0::Func3InterfaceOrTimer)
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn func4_analog(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_0::Func4Analog)
    }
}
#[doc = "Значения двух бит кодируют выбранный функционал для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port1_1 {
    #[doc = "0: Первая функция (порт общего назначения)"]
    Func1Gpio = 0,
    #[doc = "1: Вторая функция (последовательный интерфейс)"]
    Func2Interface = 1,
    #[doc = "2: Третья функция (последовательный интерфейс или таймер)"]
    Func3InterfaceOrTimer = 2,
    #[doc = "3: Четвертая функция (аналоговый сигнал)"]
    Func4Analog = 3,
}
impl From<Port1_1> for u8 {
    #[inline(always)]
    fn from(variant: Port1_1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port1_1 {
    type Ux = u8;
}
impl crate::IsEnum for Port1_1 {}
#[doc = "Field `Port_1_1` reader - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_1R = crate::FieldReader<Port1_1>;
impl Port1_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Port1_1 {
        match self.bits {
            0 => Port1_1::Func1Gpio,
            1 => Port1_1::Func2Interface,
            2 => Port1_1::Func3InterfaceOrTimer,
            3 => Port1_1::Func4Analog,
            _ => unreachable!(),
        }
    }
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn is_func1_gpio(&self) -> bool {
        *self == Port1_1::Func1Gpio
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn is_func2_interface(&self) -> bool {
        *self == Port1_1::Func2Interface
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn is_func3_interface_or_timer(&self) -> bool {
        *self == Port1_1::Func3InterfaceOrTimer
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn is_func4_analog(&self) -> bool {
        *self == Port1_1::Func4Analog
    }
}
#[doc = "Field `Port_1_1` writer - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port1_1, crate::Safe>;
impl<'a, REG> Port1_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn func1_gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_1::Func1Gpio)
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn func2_interface(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_1::Func2Interface)
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn func3_interface_or_timer(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_1::Func3InterfaceOrTimer)
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn func4_analog(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_1::Func4Analog)
    }
}
#[doc = "Значения двух бит кодируют выбранный функционал для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port1_2 {
    #[doc = "0: Первая функция (порт общего назначения)"]
    Func1Gpio = 0,
    #[doc = "1: Вторая функция (последовательный интерфейс)"]
    Func2Interface = 1,
    #[doc = "2: Третья функция (последовательный интерфейс или таймер)"]
    Func3InterfaceOrTimer = 2,
    #[doc = "3: Четвертая функция (аналоговый сигнал)"]
    Func4Analog = 3,
}
impl From<Port1_2> for u8 {
    #[inline(always)]
    fn from(variant: Port1_2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port1_2 {
    type Ux = u8;
}
impl crate::IsEnum for Port1_2 {}
#[doc = "Field `Port_1_2` reader - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_2R = crate::FieldReader<Port1_2>;
impl Port1_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Port1_2 {
        match self.bits {
            0 => Port1_2::Func1Gpio,
            1 => Port1_2::Func2Interface,
            2 => Port1_2::Func3InterfaceOrTimer,
            3 => Port1_2::Func4Analog,
            _ => unreachable!(),
        }
    }
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn is_func1_gpio(&self) -> bool {
        *self == Port1_2::Func1Gpio
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn is_func2_interface(&self) -> bool {
        *self == Port1_2::Func2Interface
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn is_func3_interface_or_timer(&self) -> bool {
        *self == Port1_2::Func3InterfaceOrTimer
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn is_func4_analog(&self) -> bool {
        *self == Port1_2::Func4Analog
    }
}
#[doc = "Field `Port_1_2` writer - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port1_2, crate::Safe>;
impl<'a, REG> Port1_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn func1_gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_2::Func1Gpio)
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn func2_interface(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_2::Func2Interface)
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn func3_interface_or_timer(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_2::Func3InterfaceOrTimer)
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn func4_analog(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_2::Func4Analog)
    }
}
#[doc = "Значения двух бит кодируют выбранный функционал для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port1_3 {
    #[doc = "0: Первая функция (порт общего назначения)"]
    Func1Gpio = 0,
    #[doc = "1: Вторая функция (последовательный интерфейс)"]
    Func2Interface = 1,
    #[doc = "2: Третья функция (последовательный интерфейс или таймер)"]
    Func3InterfaceOrTimer = 2,
    #[doc = "3: Четвертая функция (аналоговый сигнал)"]
    Func4Analog = 3,
}
impl From<Port1_3> for u8 {
    #[inline(always)]
    fn from(variant: Port1_3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port1_3 {
    type Ux = u8;
}
impl crate::IsEnum for Port1_3 {}
#[doc = "Field `Port_1_3` reader - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_3R = crate::FieldReader<Port1_3>;
impl Port1_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Port1_3 {
        match self.bits {
            0 => Port1_3::Func1Gpio,
            1 => Port1_3::Func2Interface,
            2 => Port1_3::Func3InterfaceOrTimer,
            3 => Port1_3::Func4Analog,
            _ => unreachable!(),
        }
    }
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn is_func1_gpio(&self) -> bool {
        *self == Port1_3::Func1Gpio
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn is_func2_interface(&self) -> bool {
        *self == Port1_3::Func2Interface
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn is_func3_interface_or_timer(&self) -> bool {
        *self == Port1_3::Func3InterfaceOrTimer
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn is_func4_analog(&self) -> bool {
        *self == Port1_3::Func4Analog
    }
}
#[doc = "Field `Port_1_3` writer - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port1_3, crate::Safe>;
impl<'a, REG> Port1_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn func1_gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_3::Func1Gpio)
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn func2_interface(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_3::Func2Interface)
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn func3_interface_or_timer(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_3::Func3InterfaceOrTimer)
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn func4_analog(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_3::Func4Analog)
    }
}
#[doc = "Значения двух бит кодируют выбранный функционал для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port1_4 {
    #[doc = "0: Первая функция (порт общего назначения)"]
    Func1Gpio = 0,
    #[doc = "1: Вторая функция (последовательный интерфейс)"]
    Func2Interface = 1,
    #[doc = "2: Третья функция (последовательный интерфейс или таймер)"]
    Func3InterfaceOrTimer = 2,
    #[doc = "3: Четвертая функция (аналоговый сигнал)"]
    Func4Analog = 3,
}
impl From<Port1_4> for u8 {
    #[inline(always)]
    fn from(variant: Port1_4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port1_4 {
    type Ux = u8;
}
impl crate::IsEnum for Port1_4 {}
#[doc = "Field `Port_1_4` reader - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_4R = crate::FieldReader<Port1_4>;
impl Port1_4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Port1_4 {
        match self.bits {
            0 => Port1_4::Func1Gpio,
            1 => Port1_4::Func2Interface,
            2 => Port1_4::Func3InterfaceOrTimer,
            3 => Port1_4::Func4Analog,
            _ => unreachable!(),
        }
    }
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn is_func1_gpio(&self) -> bool {
        *self == Port1_4::Func1Gpio
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn is_func2_interface(&self) -> bool {
        *self == Port1_4::Func2Interface
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn is_func3_interface_or_timer(&self) -> bool {
        *self == Port1_4::Func3InterfaceOrTimer
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn is_func4_analog(&self) -> bool {
        *self == Port1_4::Func4Analog
    }
}
#[doc = "Field `Port_1_4` writer - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_4W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port1_4, crate::Safe>;
impl<'a, REG> Port1_4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn func1_gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_4::Func1Gpio)
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn func2_interface(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_4::Func2Interface)
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn func3_interface_or_timer(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_4::Func3InterfaceOrTimer)
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn func4_analog(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_4::Func4Analog)
    }
}
#[doc = "Значения двух бит кодируют выбранный функционал для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port1_5 {
    #[doc = "0: Первая функция (порт общего назначения)"]
    Func1Gpio = 0,
    #[doc = "1: Вторая функция (последовательный интерфейс)"]
    Func2Interface = 1,
    #[doc = "2: Третья функция (последовательный интерфейс или таймер)"]
    Func3InterfaceOrTimer = 2,
    #[doc = "3: Четвертая функция (аналоговый сигнал)"]
    Func4Analog = 3,
}
impl From<Port1_5> for u8 {
    #[inline(always)]
    fn from(variant: Port1_5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port1_5 {
    type Ux = u8;
}
impl crate::IsEnum for Port1_5 {}
#[doc = "Field `Port_1_5` reader - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_5R = crate::FieldReader<Port1_5>;
impl Port1_5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Port1_5 {
        match self.bits {
            0 => Port1_5::Func1Gpio,
            1 => Port1_5::Func2Interface,
            2 => Port1_5::Func3InterfaceOrTimer,
            3 => Port1_5::Func4Analog,
            _ => unreachable!(),
        }
    }
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn is_func1_gpio(&self) -> bool {
        *self == Port1_5::Func1Gpio
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn is_func2_interface(&self) -> bool {
        *self == Port1_5::Func2Interface
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn is_func3_interface_or_timer(&self) -> bool {
        *self == Port1_5::Func3InterfaceOrTimer
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn is_func4_analog(&self) -> bool {
        *self == Port1_5::Func4Analog
    }
}
#[doc = "Field `Port_1_5` writer - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_5W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port1_5, crate::Safe>;
impl<'a, REG> Port1_5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn func1_gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_5::Func1Gpio)
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn func2_interface(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_5::Func2Interface)
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn func3_interface_or_timer(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_5::Func3InterfaceOrTimer)
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn func4_analog(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_5::Func4Analog)
    }
}
#[doc = "Значения двух бит кодируют выбранный функционал для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port1_6 {
    #[doc = "0: Первая функция (порт общего назначения)"]
    Func1Gpio = 0,
    #[doc = "1: Вторая функция (последовательный интерфейс)"]
    Func2Interface = 1,
    #[doc = "2: Третья функция (последовательный интерфейс или таймер)"]
    Func3InterfaceOrTimer = 2,
    #[doc = "3: Четвертая функция (аналоговый сигнал)"]
    Func4Analog = 3,
}
impl From<Port1_6> for u8 {
    #[inline(always)]
    fn from(variant: Port1_6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port1_6 {
    type Ux = u8;
}
impl crate::IsEnum for Port1_6 {}
#[doc = "Field `Port_1_6` reader - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_6R = crate::FieldReader<Port1_6>;
impl Port1_6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Port1_6 {
        match self.bits {
            0 => Port1_6::Func1Gpio,
            1 => Port1_6::Func2Interface,
            2 => Port1_6::Func3InterfaceOrTimer,
            3 => Port1_6::Func4Analog,
            _ => unreachable!(),
        }
    }
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn is_func1_gpio(&self) -> bool {
        *self == Port1_6::Func1Gpio
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn is_func2_interface(&self) -> bool {
        *self == Port1_6::Func2Interface
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn is_func3_interface_or_timer(&self) -> bool {
        *self == Port1_6::Func3InterfaceOrTimer
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn is_func4_analog(&self) -> bool {
        *self == Port1_6::Func4Analog
    }
}
#[doc = "Field `Port_1_6` writer - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port1_6, crate::Safe>;
impl<'a, REG> Port1_6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn func1_gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_6::Func1Gpio)
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn func2_interface(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_6::Func2Interface)
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn func3_interface_or_timer(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_6::Func3InterfaceOrTimer)
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn func4_analog(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_6::Func4Analog)
    }
}
#[doc = "Значения двух бит кодируют выбранный функционал для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port1_7 {
    #[doc = "0: Первая функция (порт общего назначения)"]
    Func1Gpio = 0,
    #[doc = "1: Вторая функция (последовательный интерфейс)"]
    Func2Interface = 1,
    #[doc = "2: Третья функция (последовательный интерфейс или таймер)"]
    Func3InterfaceOrTimer = 2,
    #[doc = "3: Четвертая функция (аналоговый сигнал)"]
    Func4Analog = 3,
}
impl From<Port1_7> for u8 {
    #[inline(always)]
    fn from(variant: Port1_7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port1_7 {
    type Ux = u8;
}
impl crate::IsEnum for Port1_7 {}
#[doc = "Field `Port_1_7` reader - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_7R = crate::FieldReader<Port1_7>;
impl Port1_7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Port1_7 {
        match self.bits {
            0 => Port1_7::Func1Gpio,
            1 => Port1_7::Func2Interface,
            2 => Port1_7::Func3InterfaceOrTimer,
            3 => Port1_7::Func4Analog,
            _ => unreachable!(),
        }
    }
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn is_func1_gpio(&self) -> bool {
        *self == Port1_7::Func1Gpio
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn is_func2_interface(&self) -> bool {
        *self == Port1_7::Func2Interface
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn is_func3_interface_or_timer(&self) -> bool {
        *self == Port1_7::Func3InterfaceOrTimer
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn is_func4_analog(&self) -> bool {
        *self == Port1_7::Func4Analog
    }
}
#[doc = "Field `Port_1_7` writer - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_7W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port1_7, crate::Safe>;
impl<'a, REG> Port1_7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn func1_gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_7::Func1Gpio)
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn func2_interface(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_7::Func2Interface)
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn func3_interface_or_timer(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_7::Func3InterfaceOrTimer)
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn func4_analog(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_7::Func4Analog)
    }
}
#[doc = "Значения двух бит кодируют выбранный функционал для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port1_8 {
    #[doc = "0: Первая функция (порт общего назначения)"]
    Func1Gpio = 0,
    #[doc = "1: Вторая функция (последовательный интерфейс)"]
    Func2Interface = 1,
    #[doc = "2: Третья функция (последовательный интерфейс или таймер)"]
    Func3InterfaceOrTimer = 2,
    #[doc = "3: Четвертая функция (аналоговый сигнал)"]
    Func4Analog = 3,
}
impl From<Port1_8> for u8 {
    #[inline(always)]
    fn from(variant: Port1_8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port1_8 {
    type Ux = u8;
}
impl crate::IsEnum for Port1_8 {}
#[doc = "Field `Port_1_8` reader - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_8R = crate::FieldReader<Port1_8>;
impl Port1_8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Port1_8 {
        match self.bits {
            0 => Port1_8::Func1Gpio,
            1 => Port1_8::Func2Interface,
            2 => Port1_8::Func3InterfaceOrTimer,
            3 => Port1_8::Func4Analog,
            _ => unreachable!(),
        }
    }
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn is_func1_gpio(&self) -> bool {
        *self == Port1_8::Func1Gpio
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn is_func2_interface(&self) -> bool {
        *self == Port1_8::Func2Interface
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn is_func3_interface_or_timer(&self) -> bool {
        *self == Port1_8::Func3InterfaceOrTimer
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn is_func4_analog(&self) -> bool {
        *self == Port1_8::Func4Analog
    }
}
#[doc = "Field `Port_1_8` writer - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_8W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port1_8, crate::Safe>;
impl<'a, REG> Port1_8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn func1_gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_8::Func1Gpio)
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn func2_interface(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_8::Func2Interface)
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn func3_interface_or_timer(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_8::Func3InterfaceOrTimer)
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn func4_analog(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_8::Func4Analog)
    }
}
#[doc = "Значения двух бит кодируют выбранный функционал для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port1_9 {
    #[doc = "0: Первая функция (порт общего назначения)"]
    Func1Gpio = 0,
    #[doc = "1: Вторая функция (последовательный интерфейс)"]
    Func2Interface = 1,
    #[doc = "2: Третья функция (последовательный интерфейс или таймер)"]
    Func3InterfaceOrTimer = 2,
    #[doc = "3: Четвертая функция (аналоговый сигнал)"]
    Func4Analog = 3,
}
impl From<Port1_9> for u8 {
    #[inline(always)]
    fn from(variant: Port1_9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port1_9 {
    type Ux = u8;
}
impl crate::IsEnum for Port1_9 {}
#[doc = "Field `Port_1_9` reader - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_9R = crate::FieldReader<Port1_9>;
impl Port1_9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Port1_9 {
        match self.bits {
            0 => Port1_9::Func1Gpio,
            1 => Port1_9::Func2Interface,
            2 => Port1_9::Func3InterfaceOrTimer,
            3 => Port1_9::Func4Analog,
            _ => unreachable!(),
        }
    }
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn is_func1_gpio(&self) -> bool {
        *self == Port1_9::Func1Gpio
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn is_func2_interface(&self) -> bool {
        *self == Port1_9::Func2Interface
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn is_func3_interface_or_timer(&self) -> bool {
        *self == Port1_9::Func3InterfaceOrTimer
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn is_func4_analog(&self) -> bool {
        *self == Port1_9::Func4Analog
    }
}
#[doc = "Field `Port_1_9` writer - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_9W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port1_9, crate::Safe>;
impl<'a, REG> Port1_9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn func1_gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_9::Func1Gpio)
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn func2_interface(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_9::Func2Interface)
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn func3_interface_or_timer(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_9::Func3InterfaceOrTimer)
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn func4_analog(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_9::Func4Analog)
    }
}
#[doc = "Значения двух бит кодируют выбранный функционал для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port1_10 {
    #[doc = "0: Первая функция (порт общего назначения)"]
    Func1Gpio = 0,
    #[doc = "1: Вторая функция (последовательный интерфейс)"]
    Func2Interface = 1,
    #[doc = "2: Третья функция (последовательный интерфейс или таймер)"]
    Func3InterfaceOrTimer = 2,
    #[doc = "3: Четвертая функция (аналоговый сигнал)"]
    Func4Analog = 3,
}
impl From<Port1_10> for u8 {
    #[inline(always)]
    fn from(variant: Port1_10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port1_10 {
    type Ux = u8;
}
impl crate::IsEnum for Port1_10 {}
#[doc = "Field `Port_1_10` reader - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_10R = crate::FieldReader<Port1_10>;
impl Port1_10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Port1_10 {
        match self.bits {
            0 => Port1_10::Func1Gpio,
            1 => Port1_10::Func2Interface,
            2 => Port1_10::Func3InterfaceOrTimer,
            3 => Port1_10::Func4Analog,
            _ => unreachable!(),
        }
    }
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn is_func1_gpio(&self) -> bool {
        *self == Port1_10::Func1Gpio
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn is_func2_interface(&self) -> bool {
        *self == Port1_10::Func2Interface
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn is_func3_interface_or_timer(&self) -> bool {
        *self == Port1_10::Func3InterfaceOrTimer
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn is_func4_analog(&self) -> bool {
        *self == Port1_10::Func4Analog
    }
}
#[doc = "Field `Port_1_10` writer - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_10W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port1_10, crate::Safe>;
impl<'a, REG> Port1_10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn func1_gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_10::Func1Gpio)
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn func2_interface(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_10::Func2Interface)
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn func3_interface_or_timer(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_10::Func3InterfaceOrTimer)
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn func4_analog(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_10::Func4Analog)
    }
}
#[doc = "Значения двух бит кодируют выбранный функционал для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port1_11 {
    #[doc = "0: Первая функция (порт общего назначения)"]
    Func1Gpio = 0,
    #[doc = "1: Вторая функция (последовательный интерфейс)"]
    Func2Interface = 1,
    #[doc = "2: Третья функция (последовательный интерфейс или таймер)"]
    Func3InterfaceOrTimer = 2,
    #[doc = "3: Четвертая функция (аналоговый сигнал)"]
    Func4Analog = 3,
}
impl From<Port1_11> for u8 {
    #[inline(always)]
    fn from(variant: Port1_11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port1_11 {
    type Ux = u8;
}
impl crate::IsEnum for Port1_11 {}
#[doc = "Field `Port_1_11` reader - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_11R = crate::FieldReader<Port1_11>;
impl Port1_11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Port1_11 {
        match self.bits {
            0 => Port1_11::Func1Gpio,
            1 => Port1_11::Func2Interface,
            2 => Port1_11::Func3InterfaceOrTimer,
            3 => Port1_11::Func4Analog,
            _ => unreachable!(),
        }
    }
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn is_func1_gpio(&self) -> bool {
        *self == Port1_11::Func1Gpio
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn is_func2_interface(&self) -> bool {
        *self == Port1_11::Func2Interface
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn is_func3_interface_or_timer(&self) -> bool {
        *self == Port1_11::Func3InterfaceOrTimer
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn is_func4_analog(&self) -> bool {
        *self == Port1_11::Func4Analog
    }
}
#[doc = "Field `Port_1_11` writer - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_11W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port1_11, crate::Safe>;
impl<'a, REG> Port1_11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn func1_gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_11::Func1Gpio)
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn func2_interface(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_11::Func2Interface)
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn func3_interface_or_timer(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_11::Func3InterfaceOrTimer)
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn func4_analog(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_11::Func4Analog)
    }
}
#[doc = "Значения двух бит кодируют выбранный функционал для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port1_12 {
    #[doc = "0: Первая функция (порт общего назначения)"]
    Func1Gpio = 0,
    #[doc = "1: Вторая функция (последовательный интерфейс)"]
    Func2Interface = 1,
    #[doc = "2: Третья функция (последовательный интерфейс или таймер)"]
    Func3InterfaceOrTimer = 2,
    #[doc = "3: Четвертая функция (аналоговый сигнал)"]
    Func4Analog = 3,
}
impl From<Port1_12> for u8 {
    #[inline(always)]
    fn from(variant: Port1_12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port1_12 {
    type Ux = u8;
}
impl crate::IsEnum for Port1_12 {}
#[doc = "Field `Port_1_12` reader - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_12R = crate::FieldReader<Port1_12>;
impl Port1_12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Port1_12 {
        match self.bits {
            0 => Port1_12::Func1Gpio,
            1 => Port1_12::Func2Interface,
            2 => Port1_12::Func3InterfaceOrTimer,
            3 => Port1_12::Func4Analog,
            _ => unreachable!(),
        }
    }
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn is_func1_gpio(&self) -> bool {
        *self == Port1_12::Func1Gpio
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn is_func2_interface(&self) -> bool {
        *self == Port1_12::Func2Interface
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn is_func3_interface_or_timer(&self) -> bool {
        *self == Port1_12::Func3InterfaceOrTimer
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn is_func4_analog(&self) -> bool {
        *self == Port1_12::Func4Analog
    }
}
#[doc = "Field `Port_1_12` writer - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_12W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port1_12, crate::Safe>;
impl<'a, REG> Port1_12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn func1_gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_12::Func1Gpio)
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn func2_interface(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_12::Func2Interface)
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn func3_interface_or_timer(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_12::Func3InterfaceOrTimer)
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn func4_analog(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_12::Func4Analog)
    }
}
#[doc = "Значения двух бит кодируют выбранный функционал для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port1_13 {
    #[doc = "0: Первая функция (порт общего назначения)"]
    Func1Gpio = 0,
    #[doc = "1: Вторая функция (последовательный интерфейс)"]
    Func2Interface = 1,
    #[doc = "2: Третья функция (последовательный интерфейс или таймер)"]
    Func3InterfaceOrTimer = 2,
    #[doc = "3: Четвертая функция (аналоговый сигнал)"]
    Func4Analog = 3,
}
impl From<Port1_13> for u8 {
    #[inline(always)]
    fn from(variant: Port1_13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port1_13 {
    type Ux = u8;
}
impl crate::IsEnum for Port1_13 {}
#[doc = "Field `Port_1_13` reader - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_13R = crate::FieldReader<Port1_13>;
impl Port1_13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Port1_13 {
        match self.bits {
            0 => Port1_13::Func1Gpio,
            1 => Port1_13::Func2Interface,
            2 => Port1_13::Func3InterfaceOrTimer,
            3 => Port1_13::Func4Analog,
            _ => unreachable!(),
        }
    }
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn is_func1_gpio(&self) -> bool {
        *self == Port1_13::Func1Gpio
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn is_func2_interface(&self) -> bool {
        *self == Port1_13::Func2Interface
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn is_func3_interface_or_timer(&self) -> bool {
        *self == Port1_13::Func3InterfaceOrTimer
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn is_func4_analog(&self) -> bool {
        *self == Port1_13::Func4Analog
    }
}
#[doc = "Field `Port_1_13` writer - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_13W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port1_13, crate::Safe>;
impl<'a, REG> Port1_13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn func1_gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_13::Func1Gpio)
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn func2_interface(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_13::Func2Interface)
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn func3_interface_or_timer(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_13::Func3InterfaceOrTimer)
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn func4_analog(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_13::Func4Analog)
    }
}
#[doc = "Значения двух бит кодируют выбранный функционал для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port1_14 {
    #[doc = "0: Первая функция (порт общего назначения)"]
    Func1Gpio = 0,
    #[doc = "1: Вторая функция (последовательный интерфейс)"]
    Func2Interface = 1,
    #[doc = "2: Третья функция (последовательный интерфейс или таймер)"]
    Func3InterfaceOrTimer = 2,
    #[doc = "3: Четвертая функция (аналоговый сигнал)"]
    Func4Analog = 3,
}
impl From<Port1_14> for u8 {
    #[inline(always)]
    fn from(variant: Port1_14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port1_14 {
    type Ux = u8;
}
impl crate::IsEnum for Port1_14 {}
#[doc = "Field `Port_1_14` reader - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_14R = crate::FieldReader<Port1_14>;
impl Port1_14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Port1_14 {
        match self.bits {
            0 => Port1_14::Func1Gpio,
            1 => Port1_14::Func2Interface,
            2 => Port1_14::Func3InterfaceOrTimer,
            3 => Port1_14::Func4Analog,
            _ => unreachable!(),
        }
    }
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn is_func1_gpio(&self) -> bool {
        *self == Port1_14::Func1Gpio
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn is_func2_interface(&self) -> bool {
        *self == Port1_14::Func2Interface
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn is_func3_interface_or_timer(&self) -> bool {
        *self == Port1_14::Func3InterfaceOrTimer
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn is_func4_analog(&self) -> bool {
        *self == Port1_14::Func4Analog
    }
}
#[doc = "Field `Port_1_14` writer - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_14W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port1_14, crate::Safe>;
impl<'a, REG> Port1_14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn func1_gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_14::Func1Gpio)
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn func2_interface(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_14::Func2Interface)
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn func3_interface_or_timer(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_14::Func3InterfaceOrTimer)
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn func4_analog(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_14::Func4Analog)
    }
}
#[doc = "Значения двух бит кодируют выбранный функционал для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port1_15 {
    #[doc = "0: Первая функция (порт общего назначения)"]
    Func1Gpio = 0,
    #[doc = "1: Вторая функция (последовательный интерфейс)"]
    Func2Interface = 1,
    #[doc = "2: Третья функция (последовательный интерфейс или таймер)"]
    Func3InterfaceOrTimer = 2,
    #[doc = "3: Четвертая функция (аналоговый сигнал)"]
    Func4Analog = 3,
}
impl From<Port1_15> for u8 {
    #[inline(always)]
    fn from(variant: Port1_15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port1_15 {
    type Ux = u8;
}
impl crate::IsEnum for Port1_15 {}
#[doc = "Field `Port_1_15` reader - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_15R = crate::FieldReader<Port1_15>;
impl Port1_15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Port1_15 {
        match self.bits {
            0 => Port1_15::Func1Gpio,
            1 => Port1_15::Func2Interface,
            2 => Port1_15::Func3InterfaceOrTimer,
            3 => Port1_15::Func4Analog,
            _ => unreachable!(),
        }
    }
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn is_func1_gpio(&self) -> bool {
        *self == Port1_15::Func1Gpio
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn is_func2_interface(&self) -> bool {
        *self == Port1_15::Func2Interface
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn is_func3_interface_or_timer(&self) -> bool {
        *self == Port1_15::Func3InterfaceOrTimer
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn is_func4_analog(&self) -> bool {
        *self == Port1_15::Func4Analog
    }
}
#[doc = "Field `Port_1_15` writer - Значения двух бит кодируют выбранный функционал для вывода"]
pub type Port1_15W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port1_15, crate::Safe>;
impl<'a, REG> Port1_15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Первая функция (порт общего назначения)"]
    #[inline(always)]
    pub fn func1_gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_15::Func1Gpio)
    }
    #[doc = "Вторая функция (последовательный интерфейс)"]
    #[inline(always)]
    pub fn func2_interface(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_15::Func2Interface)
    }
    #[doc = "Третья функция (последовательный интерфейс или таймер)"]
    #[inline(always)]
    pub fn func3_interface_or_timer(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_15::Func3InterfaceOrTimer)
    }
    #[doc = "Четвертая функция (аналоговый сигнал)"]
    #[inline(always)]
    pub fn func4_analog(self) -> &'a mut crate::W<REG> {
        self.variant(Port1_15::Func4Analog)
    }
}
impl R {
    #[doc = "Bits 0:1 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_0(&self) -> Port1_0R {
        Port1_0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_1(&self) -> Port1_1R {
        Port1_1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_2(&self) -> Port1_2R {
        Port1_2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_3(&self) -> Port1_3R {
        Port1_3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_4(&self) -> Port1_4R {
        Port1_4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_5(&self) -> Port1_5R {
        Port1_5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_6(&self) -> Port1_6R {
        Port1_6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_7(&self) -> Port1_7R {
        Port1_7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_8(&self) -> Port1_8R {
        Port1_8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_9(&self) -> Port1_9R {
        Port1_9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_10(&self) -> Port1_10R {
        Port1_10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_11(&self) -> Port1_11R {
        Port1_11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_12(&self) -> Port1_12R {
        Port1_12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_13(&self) -> Port1_13R {
        Port1_13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_14(&self) -> Port1_14R {
        Port1_14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_15(&self) -> Port1_15R {
        Port1_15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_0(&mut self) -> Port1_0W<Pad1CfgSpec> {
        Port1_0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_1(&mut self) -> Port1_1W<Pad1CfgSpec> {
        Port1_1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_2(&mut self) -> Port1_2W<Pad1CfgSpec> {
        Port1_2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_3(&mut self) -> Port1_3W<Pad1CfgSpec> {
        Port1_3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_4(&mut self) -> Port1_4W<Pad1CfgSpec> {
        Port1_4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_5(&mut self) -> Port1_5W<Pad1CfgSpec> {
        Port1_5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_6(&mut self) -> Port1_6W<Pad1CfgSpec> {
        Port1_6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_7(&mut self) -> Port1_7W<Pad1CfgSpec> {
        Port1_7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_8(&mut self) -> Port1_8W<Pad1CfgSpec> {
        Port1_8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_9(&mut self) -> Port1_9W<Pad1CfgSpec> {
        Port1_9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_10(&mut self) -> Port1_10W<Pad1CfgSpec> {
        Port1_10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_11(&mut self) -> Port1_11W<Pad1CfgSpec> {
        Port1_11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_12(&mut self) -> Port1_12W<Pad1CfgSpec> {
        Port1_12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_13(&mut self) -> Port1_13W<Pad1CfgSpec> {
        Port1_13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_14(&mut self) -> Port1_14W<Pad1CfgSpec> {
        Port1_14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Значения двух бит кодируют выбранный функционал для вывода"]
    #[inline(always)]
    pub fn port_1_15(&mut self) -> Port1_15W<Pad1CfgSpec> {
        Port1_15W::new(self, 30)
    }
}
#[doc = "Управление функциями выводов PORT1\n\nYou can [`read`](crate::Reg::read) this register and get [`pad1_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad1_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pad1CfgSpec;
impl crate::RegisterSpec for Pad1CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad1_cfg::R`](R) reader structure"]
impl crate::Readable for Pad1CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pad1_cfg::W`](W) writer structure"]
impl crate::Writable for Pad1CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAD1_CFG to value 0"]
impl crate::Resettable for Pad1CfgSpec {
    const RESET_VALUE: u32 = 0;
}
