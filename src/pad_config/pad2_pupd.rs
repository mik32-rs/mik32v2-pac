#[doc = "Register `PAD2_PUPD` reader"]
pub type R = crate::R<Pad2PupdSpec>;
#[doc = "Register `PAD2_PUPD` writer"]
pub type W = crate::W<Pad2PupdSpec>;
#[doc = "Значения двух бит кодируют выбранный режим для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port2_0 {
    #[doc = "0: Без подтяжки"]
    PullNone = 0,
    #[doc = "1: Подтяжка к питанию"]
    PullUp = 1,
    #[doc = "2: Подтяжка к земле"]
    PullDown = 2,
}
impl From<Port2_0> for u8 {
    #[inline(always)]
    fn from(variant: Port2_0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port2_0 {
    type Ux = u8;
}
impl crate::IsEnum for Port2_0 {}
#[doc = "Field `Port_2_0` reader - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port2_0R = crate::FieldReader<Port2_0>;
impl Port2_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Port2_0> {
        match self.bits {
            0 => Some(Port2_0::PullNone),
            1 => Some(Port2_0::PullUp),
            2 => Some(Port2_0::PullDown),
            _ => None,
        }
    }
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn is_pull_none(&self) -> bool {
        *self == Port2_0::PullNone
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Port2_0::PullUp
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Port2_0::PullDown
    }
}
#[doc = "Field `Port_2_0` writer - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port2_0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port2_0>;
impl<'a, REG> Port2_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn pull_none(self) -> &'a mut crate::W<REG> {
        self.variant(Port2_0::PullNone)
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Port2_0::PullUp)
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Port2_0::PullDown)
    }
}
#[doc = "Значения двух бит кодируют выбранный режим для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port2_1 {
    #[doc = "0: Без подтяжки"]
    PullNone = 0,
    #[doc = "1: Подтяжка к питанию"]
    PullUp = 1,
    #[doc = "2: Подтяжка к земле"]
    PullDown = 2,
}
impl From<Port2_1> for u8 {
    #[inline(always)]
    fn from(variant: Port2_1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port2_1 {
    type Ux = u8;
}
impl crate::IsEnum for Port2_1 {}
#[doc = "Field `Port_2_1` reader - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port2_1R = crate::FieldReader<Port2_1>;
impl Port2_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Port2_1> {
        match self.bits {
            0 => Some(Port2_1::PullNone),
            1 => Some(Port2_1::PullUp),
            2 => Some(Port2_1::PullDown),
            _ => None,
        }
    }
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn is_pull_none(&self) -> bool {
        *self == Port2_1::PullNone
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Port2_1::PullUp
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Port2_1::PullDown
    }
}
#[doc = "Field `Port_2_1` writer - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port2_1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port2_1>;
impl<'a, REG> Port2_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn pull_none(self) -> &'a mut crate::W<REG> {
        self.variant(Port2_1::PullNone)
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Port2_1::PullUp)
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Port2_1::PullDown)
    }
}
#[doc = "Значения двух бит кодируют выбранный режим для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port2_2 {
    #[doc = "0: Без подтяжки"]
    PullNone = 0,
    #[doc = "1: Подтяжка к питанию"]
    PullUp = 1,
    #[doc = "2: Подтяжка к земле"]
    PullDown = 2,
}
impl From<Port2_2> for u8 {
    #[inline(always)]
    fn from(variant: Port2_2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port2_2 {
    type Ux = u8;
}
impl crate::IsEnum for Port2_2 {}
#[doc = "Field `Port_2_2` reader - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port2_2R = crate::FieldReader<Port2_2>;
impl Port2_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Port2_2> {
        match self.bits {
            0 => Some(Port2_2::PullNone),
            1 => Some(Port2_2::PullUp),
            2 => Some(Port2_2::PullDown),
            _ => None,
        }
    }
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn is_pull_none(&self) -> bool {
        *self == Port2_2::PullNone
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Port2_2::PullUp
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Port2_2::PullDown
    }
}
#[doc = "Field `Port_2_2` writer - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port2_2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port2_2>;
impl<'a, REG> Port2_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn pull_none(self) -> &'a mut crate::W<REG> {
        self.variant(Port2_2::PullNone)
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Port2_2::PullUp)
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Port2_2::PullDown)
    }
}
#[doc = "Значения двух бит кодируют выбранный режим для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port2_3 {
    #[doc = "0: Без подтяжки"]
    PullNone = 0,
    #[doc = "1: Подтяжка к питанию"]
    PullUp = 1,
    #[doc = "2: Подтяжка к земле"]
    PullDown = 2,
}
impl From<Port2_3> for u8 {
    #[inline(always)]
    fn from(variant: Port2_3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port2_3 {
    type Ux = u8;
}
impl crate::IsEnum for Port2_3 {}
#[doc = "Field `Port_2_3` reader - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port2_3R = crate::FieldReader<Port2_3>;
impl Port2_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Port2_3> {
        match self.bits {
            0 => Some(Port2_3::PullNone),
            1 => Some(Port2_3::PullUp),
            2 => Some(Port2_3::PullDown),
            _ => None,
        }
    }
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn is_pull_none(&self) -> bool {
        *self == Port2_3::PullNone
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Port2_3::PullUp
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Port2_3::PullDown
    }
}
#[doc = "Field `Port_2_3` writer - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port2_3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port2_3>;
impl<'a, REG> Port2_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn pull_none(self) -> &'a mut crate::W<REG> {
        self.variant(Port2_3::PullNone)
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Port2_3::PullUp)
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Port2_3::PullDown)
    }
}
#[doc = "Значения двух бит кодируют выбранный режим для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port2_4 {
    #[doc = "0: Без подтяжки"]
    PullNone = 0,
    #[doc = "1: Подтяжка к питанию"]
    PullUp = 1,
    #[doc = "2: Подтяжка к земле"]
    PullDown = 2,
}
impl From<Port2_4> for u8 {
    #[inline(always)]
    fn from(variant: Port2_4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port2_4 {
    type Ux = u8;
}
impl crate::IsEnum for Port2_4 {}
#[doc = "Field `Port_2_4` reader - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port2_4R = crate::FieldReader<Port2_4>;
impl Port2_4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Port2_4> {
        match self.bits {
            0 => Some(Port2_4::PullNone),
            1 => Some(Port2_4::PullUp),
            2 => Some(Port2_4::PullDown),
            _ => None,
        }
    }
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn is_pull_none(&self) -> bool {
        *self == Port2_4::PullNone
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Port2_4::PullUp
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Port2_4::PullDown
    }
}
#[doc = "Field `Port_2_4` writer - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port2_4W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port2_4>;
impl<'a, REG> Port2_4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn pull_none(self) -> &'a mut crate::W<REG> {
        self.variant(Port2_4::PullNone)
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Port2_4::PullUp)
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Port2_4::PullDown)
    }
}
#[doc = "Значения двух бит кодируют выбранный режим для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port2_5 {
    #[doc = "0: Без подтяжки"]
    PullNone = 0,
    #[doc = "1: Подтяжка к питанию"]
    PullUp = 1,
    #[doc = "2: Подтяжка к земле"]
    PullDown = 2,
}
impl From<Port2_5> for u8 {
    #[inline(always)]
    fn from(variant: Port2_5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port2_5 {
    type Ux = u8;
}
impl crate::IsEnum for Port2_5 {}
#[doc = "Field `Port_2_5` reader - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port2_5R = crate::FieldReader<Port2_5>;
impl Port2_5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Port2_5> {
        match self.bits {
            0 => Some(Port2_5::PullNone),
            1 => Some(Port2_5::PullUp),
            2 => Some(Port2_5::PullDown),
            _ => None,
        }
    }
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn is_pull_none(&self) -> bool {
        *self == Port2_5::PullNone
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Port2_5::PullUp
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Port2_5::PullDown
    }
}
#[doc = "Field `Port_2_5` writer - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port2_5W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port2_5>;
impl<'a, REG> Port2_5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn pull_none(self) -> &'a mut crate::W<REG> {
        self.variant(Port2_5::PullNone)
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Port2_5::PullUp)
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Port2_5::PullDown)
    }
}
#[doc = "Значения двух бит кодируют выбранный режим для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port2_6 {
    #[doc = "0: Без подтяжки"]
    PullNone = 0,
    #[doc = "1: Подтяжка к питанию"]
    PullUp = 1,
    #[doc = "2: Подтяжка к земле"]
    PullDown = 2,
}
impl From<Port2_6> for u8 {
    #[inline(always)]
    fn from(variant: Port2_6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port2_6 {
    type Ux = u8;
}
impl crate::IsEnum for Port2_6 {}
#[doc = "Field `Port_2_6` reader - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port2_6R = crate::FieldReader<Port2_6>;
impl Port2_6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Port2_6> {
        match self.bits {
            0 => Some(Port2_6::PullNone),
            1 => Some(Port2_6::PullUp),
            2 => Some(Port2_6::PullDown),
            _ => None,
        }
    }
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn is_pull_none(&self) -> bool {
        *self == Port2_6::PullNone
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Port2_6::PullUp
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Port2_6::PullDown
    }
}
#[doc = "Field `Port_2_6` writer - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port2_6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port2_6>;
impl<'a, REG> Port2_6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn pull_none(self) -> &'a mut crate::W<REG> {
        self.variant(Port2_6::PullNone)
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Port2_6::PullUp)
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Port2_6::PullDown)
    }
}
#[doc = "Значения двух бит кодируют выбранный режим для вывода\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Port2_7 {
    #[doc = "0: Без подтяжки"]
    PullNone = 0,
    #[doc = "1: Подтяжка к питанию"]
    PullUp = 1,
    #[doc = "2: Подтяжка к земле"]
    PullDown = 2,
}
impl From<Port2_7> for u8 {
    #[inline(always)]
    fn from(variant: Port2_7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Port2_7 {
    type Ux = u8;
}
impl crate::IsEnum for Port2_7 {}
#[doc = "Field `Port_2_7` reader - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port2_7R = crate::FieldReader<Port2_7>;
impl Port2_7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Port2_7> {
        match self.bits {
            0 => Some(Port2_7::PullNone),
            1 => Some(Port2_7::PullUp),
            2 => Some(Port2_7::PullDown),
            _ => None,
        }
    }
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn is_pull_none(&self) -> bool {
        *self == Port2_7::PullNone
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Port2_7::PullUp
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Port2_7::PullDown
    }
}
#[doc = "Field `Port_2_7` writer - Значения двух бит кодируют выбранный режим для вывода"]
pub type Port2_7W<'a, REG> = crate::FieldWriter<'a, REG, 2, Port2_7>;
impl<'a, REG> Port2_7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Без подтяжки"]
    #[inline(always)]
    pub fn pull_none(self) -> &'a mut crate::W<REG> {
        self.variant(Port2_7::PullNone)
    }
    #[doc = "Подтяжка к питанию"]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Port2_7::PullUp)
    }
    #[doc = "Подтяжка к земле"]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Port2_7::PullDown)
    }
}
impl R {
    #[doc = "Bits 0:1 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_2_0(&self) -> Port2_0R {
        Port2_0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_2_1(&self) -> Port2_1R {
        Port2_1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_2_2(&self) -> Port2_2R {
        Port2_2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_2_3(&self) -> Port2_3R {
        Port2_3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_2_4(&self) -> Port2_4R {
        Port2_4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_2_5(&self) -> Port2_5R {
        Port2_5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_2_6(&self) -> Port2_6R {
        Port2_6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_2_7(&self) -> Port2_7R {
        Port2_7R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_2_0(&mut self) -> Port2_0W<Pad2PupdSpec> {
        Port2_0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_2_1(&mut self) -> Port2_1W<Pad2PupdSpec> {
        Port2_1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_2_2(&mut self) -> Port2_2W<Pad2PupdSpec> {
        Port2_2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_2_3(&mut self) -> Port2_3W<Pad2PupdSpec> {
        Port2_3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_2_4(&mut self) -> Port2_4W<Pad2PupdSpec> {
        Port2_4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_2_5(&mut self) -> Port2_5W<Pad2PupdSpec> {
        Port2_5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_2_6(&mut self) -> Port2_6W<Pad2PupdSpec> {
        Port2_6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Значения двух бит кодируют выбранный режим для вывода"]
    #[inline(always)]
    pub fn port_2_7(&mut self) -> Port2_7W<Pad2PupdSpec> {
        Port2_7W::new(self, 14)
    }
}
#[doc = "Управление резисторами подтяжки выводов PORT2\n\nYou can [`read`](crate::Reg::read) this register and get [`pad2_pupd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad2_pupd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pad2PupdSpec;
impl crate::RegisterSpec for Pad2PupdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad2_pupd::R`](R) reader structure"]
impl crate::Readable for Pad2PupdSpec {}
#[doc = "`write(|w| ..)` method takes [`pad2_pupd::W`](W) writer structure"]
impl crate::Writable for Pad2PupdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAD2_PUPD to value 0"]
impl crate::Resettable for Pad2PupdSpec {
    const RESET_VALUE: u32 = 0;
}
