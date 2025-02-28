#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Управление интерфейсом. После очистки, бит должен оставаться в ‘0’ минимум три периода тактового сигнала APB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pe {
    #[doc = "0: Интерфейс выключен"]
    Disable = 0,
    #[doc = "1: Интерфейс включен"]
    Enable = 1,
}
impl From<Pe> for bool {
    #[inline(always)]
    fn from(variant: Pe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PE` reader - Управление интерфейсом. После очистки, бит должен оставаться в ‘0’ минимум три периода тактового сигнала APB"]
pub type PeR = crate::BitReader<Pe>;
impl PeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pe {
        match self.bits {
            false => Pe::Disable,
            true => Pe::Enable,
        }
    }
    #[doc = "Интерфейс выключен"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Pe::Disable
    }
    #[doc = "Интерфейс включен"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pe::Enable
    }
}
#[doc = "Field `PE` writer - Управление интерфейсом. После очистки, бит должен оставаться в ‘0’ минимум три периода тактового сигнала APB"]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG, Pe>;
impl<'a, REG> PeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Интерфейс выключен"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::Disable)
    }
    #[doc = "Интерфейс включен"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::Enable)
    }
}
#[doc = "Разрешение прерывания при передаче\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txie {
    #[doc = "0: Прерывание запрещено"]
    Disable = 0,
    #[doc = "1: Прерывание разрешено"]
    Enable = 1,
}
impl From<Txie> for bool {
    #[inline(always)]
    fn from(variant: Txie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXIE` reader - Разрешение прерывания при передаче"]
pub type TxieR = crate::BitReader<Txie>;
impl TxieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txie {
        match self.bits {
            false => Txie::Disable,
            true => Txie::Enable,
        }
    }
    #[doc = "Прерывание запрещено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Txie::Disable
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Txie::Enable
    }
}
#[doc = "Field `TXIE` writer - Разрешение прерывания при передаче"]
pub type TxieW<'a, REG> = crate::BitWriter<'a, REG, Txie>;
impl<'a, REG> TxieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание запрещено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Txie::Disable)
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Txie::Enable)
    }
}
#[doc = "Разрешение прерывания при приеме\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxie {
    #[doc = "0: Прерывание запрещено"]
    Disable = 0,
    #[doc = "1: Прерывание разрешено"]
    Enable = 1,
}
impl From<Rxie> for bool {
    #[inline(always)]
    fn from(variant: Rxie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXIE` reader - Разрешение прерывания при приеме"]
pub type RxieR = crate::BitReader<Rxie>;
impl RxieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxie {
        match self.bits {
            false => Rxie::Disable,
            true => Rxie::Enable,
        }
    }
    #[doc = "Прерывание запрещено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rxie::Disable
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rxie::Enable
    }
}
#[doc = "Field `RXIE` writer - Разрешение прерывания при приеме"]
pub type RxieW<'a, REG> = crate::BitWriter<'a, REG, Rxie>;
impl<'a, REG> RxieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание запрещено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rxie::Disable)
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rxie::Enable)
    }
}
#[doc = "Разрешение прерывания соот-ветствия адреса в режиме «ведомый»\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addrie {
    #[doc = "0: Прерывание запрещено"]
    Disable = 0,
    #[doc = "1: Прерывание разрешено"]
    Enable = 1,
}
impl From<Addrie> for bool {
    #[inline(always)]
    fn from(variant: Addrie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRIE` reader - Разрешение прерывания соот-ветствия адреса в режиме «ведомый»"]
pub type AddrieR = crate::BitReader<Addrie>;
impl AddrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addrie {
        match self.bits {
            false => Addrie::Disable,
            true => Addrie::Enable,
        }
    }
    #[doc = "Прерывание запрещено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Addrie::Disable
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Addrie::Enable
    }
}
#[doc = "Field `ADDRIE` writer - Разрешение прерывания соот-ветствия адреса в режиме «ведомый»"]
pub type AddrieW<'a, REG> = crate::BitWriter<'a, REG, Addrie>;
impl<'a, REG> AddrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание запрещено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Addrie::Disable)
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Addrie::Enable)
    }
}
#[doc = "Разрешение прерывания прием NACK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nackie {
    #[doc = "0: Прерывание запрещено"]
    Disable = 0,
    #[doc = "1: Прерывание разрешено"]
    Enable = 1,
}
impl From<Nackie> for bool {
    #[inline(always)]
    fn from(variant: Nackie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACKIE` reader - Разрешение прерывания прием NACK"]
pub type NackieR = crate::BitReader<Nackie>;
impl NackieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nackie {
        match self.bits {
            false => Nackie::Disable,
            true => Nackie::Enable,
        }
    }
    #[doc = "Прерывание запрещено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Nackie::Disable
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Nackie::Enable
    }
}
#[doc = "Field `NACKIE` writer - Разрешение прерывания прием NACK"]
pub type NackieW<'a, REG> = crate::BitWriter<'a, REG, Nackie>;
impl<'a, REG> NackieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание запрещено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Nackie::Disable)
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Nackie::Enable)
    }
}
#[doc = "Разрешение прерывания обнаружения STOP на линии\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopie {
    #[doc = "0: Прерывание запрещено"]
    Disable = 0,
    #[doc = "1: Прерывание разрешено"]
    Enable = 1,
}
impl From<Stopie> for bool {
    #[inline(always)]
    fn from(variant: Stopie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPIE` reader - Разрешение прерывания обнаружения STOP на линии"]
pub type StopieR = crate::BitReader<Stopie>;
impl StopieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopie {
        match self.bits {
            false => Stopie::Disable,
            true => Stopie::Enable,
        }
    }
    #[doc = "Прерывание запрещено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Stopie::Disable
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Stopie::Enable
    }
}
#[doc = "Field `STOPIE` writer - Разрешение прерывания обнаружения STOP на линии"]
pub type StopieW<'a, REG> = crate::BitWriter<'a, REG, Stopie>;
impl<'a, REG> StopieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание запрещено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Stopie::Disable)
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Stopie::Enable)
    }
}
#[doc = "Разрешение прерывания окончания передачи. События, вызывающие прерывание: - окончание передачи (TC); - окончание передачи при RELOAD=1 (TCR).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcie {
    #[doc = "0: Прерывание запрещено"]
    Disable = 0,
    #[doc = "1: Прерывание разрешено"]
    Enable = 1,
}
impl From<Tcie> for bool {
    #[inline(always)]
    fn from(variant: Tcie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIE` reader - Разрешение прерывания окончания передачи. События, вызывающие прерывание: - окончание передачи (TC); - окончание передачи при RELOAD=1 (TCR)."]
pub type TcieR = crate::BitReader<Tcie>;
impl TcieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcie {
        match self.bits {
            false => Tcie::Disable,
            true => Tcie::Enable,
        }
    }
    #[doc = "Прерывание запрещено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tcie::Disable
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tcie::Enable
    }
}
#[doc = "Field `TCIE` writer - Разрешение прерывания окончания передачи. События, вызывающие прерывание: - окончание передачи (TC); - окончание передачи при RELOAD=1 (TCR)."]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG, Tcie>;
impl<'a, REG> TcieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание запрещено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tcie::Disable)
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tcie::Enable)
    }
}
#[doc = "Разрешение прерывания при ошибке. События, вызывающие прерывание: - потеря арбитража (ARLO); - ошибка шины (BERR); - переполне-ние/недозагрузка (OVR).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errie {
    #[doc = "0: Прерывание запрещено"]
    Disable = 0,
    #[doc = "1: Прерывание разрешено"]
    Enable = 1,
}
impl From<Errie> for bool {
    #[inline(always)]
    fn from(variant: Errie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Разрешение прерывания при ошибке. События, вызывающие прерывание: - потеря арбитража (ARLO); - ошибка шины (BERR); - переполне-ние/недозагрузка (OVR)."]
pub type ErrieR = crate::BitReader<Errie>;
impl ErrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errie {
        match self.bits {
            false => Errie::Disable,
            true => Errie::Enable,
        }
    }
    #[doc = "Прерывание запрещено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Errie::Disable
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Errie::Enable
    }
}
#[doc = "Field `ERRIE` writer - Разрешение прерывания при ошибке. События, вызывающие прерывание: - потеря арбитража (ARLO); - ошибка шины (BERR); - переполне-ние/недозагрузка (OVR)."]
pub type ErrieW<'a, REG> = crate::BitWriter<'a, REG, Errie>;
impl<'a, REG> ErrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание запрещено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Errie::Disable)
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Errie::Enable)
    }
}
#[doc = "Управление цифровым фильтром шумов. Изменение значения допускается только при выключенном блоке (PE=0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dnf {
    #[doc = "0: Цифровой фильтр выключен"]
    Disable = 0,
    #[doc = "1: Цифровой фильтр установлен на 1 такт I2CCLK"]
    _1 = 1,
    #[doc = "2: Цифровой фильтр установлен на 2 такта I2CCLK"]
    _2 = 2,
    #[doc = "3: Цифровой фильтр установлен на 3 такта I2CCLK"]
    _3 = 3,
    #[doc = "4: Цифровой фильтр установлен на 4 такта I2CCLK"]
    _4 = 4,
    #[doc = "5: Цифровой фильтр установлен на 5 тактов I2CCLK"]
    _5 = 5,
    #[doc = "6: Цифровой фильтр установлен на 6 тактов I2CCLK"]
    _6 = 6,
    #[doc = "7: Цифровой фильтр установлен на 7 тактов I2CCLK"]
    _7 = 7,
    #[doc = "8: Цифровой фильтр установлен на 8 тактов I2CCLK"]
    _8 = 8,
    #[doc = "9: Цифровой фильтр установлен на 9 тактов I2CCLK"]
    _9 = 9,
    #[doc = "10: Цифровой фильтр установлен на 10 тактов I2CCLK"]
    _10 = 10,
    #[doc = "11: Цифровой фильтр установлен на 11 тактов I2CCLK"]
    _11 = 11,
    #[doc = "12: Цифровой фильтр установлен на 12 тактов I2CCLK"]
    _12 = 12,
    #[doc = "13: Цифровой фильтр установлен на 13 тактов I2CCLK"]
    _13 = 13,
    #[doc = "14: цифровой фильтр установлен на 14 тактов I2CCLK"]
    _14 = 14,
    #[doc = "15: Цифровой фильтр установлен на 15 тактов I2CCLK"]
    _15 = 15,
}
impl From<Dnf> for u8 {
    #[inline(always)]
    fn from(variant: Dnf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dnf {
    type Ux = u8;
}
impl crate::IsEnum for Dnf {}
#[doc = "Field `DNF` reader - Управление цифровым фильтром шумов. Изменение значения допускается только при выключенном блоке (PE=0)"]
pub type DnfR = crate::FieldReader<Dnf>;
impl DnfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dnf {
        match self.bits {
            0 => Dnf::Disable,
            1 => Dnf::_1,
            2 => Dnf::_2,
            3 => Dnf::_3,
            4 => Dnf::_4,
            5 => Dnf::_5,
            6 => Dnf::_6,
            7 => Dnf::_7,
            8 => Dnf::_8,
            9 => Dnf::_9,
            10 => Dnf::_10,
            11 => Dnf::_11,
            12 => Dnf::_12,
            13 => Dnf::_13,
            14 => Dnf::_14,
            15 => Dnf::_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Цифровой фильтр выключен"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dnf::Disable
    }
    #[doc = "Цифровой фильтр установлен на 1 такт I2CCLK"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dnf::_1
    }
    #[doc = "Цифровой фильтр установлен на 2 такта I2CCLK"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == Dnf::_2
    }
    #[doc = "Цифровой фильтр установлен на 3 такта I2CCLK"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == Dnf::_3
    }
    #[doc = "Цифровой фильтр установлен на 4 такта I2CCLK"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == Dnf::_4
    }
    #[doc = "Цифровой фильтр установлен на 5 тактов I2CCLK"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == Dnf::_5
    }
    #[doc = "Цифровой фильтр установлен на 6 тактов I2CCLK"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == Dnf::_6
    }
    #[doc = "Цифровой фильтр установлен на 7 тактов I2CCLK"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == Dnf::_7
    }
    #[doc = "Цифровой фильтр установлен на 8 тактов I2CCLK"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == Dnf::_8
    }
    #[doc = "Цифровой фильтр установлен на 9 тактов I2CCLK"]
    #[inline(always)]
    pub fn is_9(&self) -> bool {
        *self == Dnf::_9
    }
    #[doc = "Цифровой фильтр установлен на 10 тактов I2CCLK"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == Dnf::_10
    }
    #[doc = "Цифровой фильтр установлен на 11 тактов I2CCLK"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == Dnf::_11
    }
    #[doc = "Цифровой фильтр установлен на 12 тактов I2CCLK"]
    #[inline(always)]
    pub fn is_12(&self) -> bool {
        *self == Dnf::_12
    }
    #[doc = "Цифровой фильтр установлен на 13 тактов I2CCLK"]
    #[inline(always)]
    pub fn is_13(&self) -> bool {
        *self == Dnf::_13
    }
    #[doc = "цифровой фильтр установлен на 14 тактов I2CCLK"]
    #[inline(always)]
    pub fn is_14(&self) -> bool {
        *self == Dnf::_14
    }
    #[doc = "Цифровой фильтр установлен на 15 тактов I2CCLK"]
    #[inline(always)]
    pub fn is_15(&self) -> bool {
        *self == Dnf::_15
    }
}
#[doc = "Field `DNF` writer - Управление цифровым фильтром шумов. Изменение значения допускается только при выключенном блоке (PE=0)"]
pub type DnfW<'a, REG> = crate::FieldWriter<'a, REG, 4, Dnf, crate::Safe>;
impl<'a, REG> DnfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Цифровой фильтр выключен"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dnf::Disable)
    }
    #[doc = "Цифровой фильтр установлен на 1 такт I2CCLK"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dnf::_1)
    }
    #[doc = "Цифровой фильтр установлен на 2 такта I2CCLK"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut crate::W<REG> {
        self.variant(Dnf::_2)
    }
    #[doc = "Цифровой фильтр установлен на 3 такта I2CCLK"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut crate::W<REG> {
        self.variant(Dnf::_3)
    }
    #[doc = "Цифровой фильтр установлен на 4 такта I2CCLK"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut crate::W<REG> {
        self.variant(Dnf::_4)
    }
    #[doc = "Цифровой фильтр установлен на 5 тактов I2CCLK"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut crate::W<REG> {
        self.variant(Dnf::_5)
    }
    #[doc = "Цифровой фильтр установлен на 6 тактов I2CCLK"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut crate::W<REG> {
        self.variant(Dnf::_6)
    }
    #[doc = "Цифровой фильтр установлен на 7 тактов I2CCLK"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut crate::W<REG> {
        self.variant(Dnf::_7)
    }
    #[doc = "Цифровой фильтр установлен на 8 тактов I2CCLK"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut crate::W<REG> {
        self.variant(Dnf::_8)
    }
    #[doc = "Цифровой фильтр установлен на 9 тактов I2CCLK"]
    #[inline(always)]
    pub fn _9(self) -> &'a mut crate::W<REG> {
        self.variant(Dnf::_9)
    }
    #[doc = "Цифровой фильтр установлен на 10 тактов I2CCLK"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(Dnf::_10)
    }
    #[doc = "Цифровой фильтр установлен на 11 тактов I2CCLK"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(Dnf::_11)
    }
    #[doc = "Цифровой фильтр установлен на 12 тактов I2CCLK"]
    #[inline(always)]
    pub fn _12(self) -> &'a mut crate::W<REG> {
        self.variant(Dnf::_12)
    }
    #[doc = "Цифровой фильтр установлен на 13 тактов I2CCLK"]
    #[inline(always)]
    pub fn _13(self) -> &'a mut crate::W<REG> {
        self.variant(Dnf::_13)
    }
    #[doc = "цифровой фильтр установлен на 14 тактов I2CCLK"]
    #[inline(always)]
    pub fn _14(self) -> &'a mut crate::W<REG> {
        self.variant(Dnf::_14)
    }
    #[doc = "Цифровой фильтр установлен на 15 тактов I2CCLK"]
    #[inline(always)]
    pub fn _15(self) -> &'a mut crate::W<REG> {
        self.variant(Dnf::_15)
    }
}
#[doc = "Управление аналоговым фильтром шумов\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anfoff {
    #[doc = "0: Фильтр выключен"]
    Disable = 0,
    #[doc = "1: Фильтр включен"]
    Enable = 1,
}
impl From<Anfoff> for bool {
    #[inline(always)]
    fn from(variant: Anfoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANFOFF` reader - Управление аналоговым фильтром шумов"]
pub type AnfoffR = crate::BitReader<Anfoff>;
impl AnfoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Anfoff {
        match self.bits {
            false => Anfoff::Disable,
            true => Anfoff::Enable,
        }
    }
    #[doc = "Фильтр выключен"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Anfoff::Disable
    }
    #[doc = "Фильтр включен"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Anfoff::Enable
    }
}
#[doc = "Field `ANFOFF` writer - Управление аналоговым фильтром шумов"]
pub type AnfoffW<'a, REG> = crate::BitWriter<'a, REG, Anfoff>;
impl<'a, REG> AnfoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Фильтр выключен"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Anfoff::Disable)
    }
    #[doc = "Фильтр включен"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Anfoff::Enable)
    }
}
#[doc = "Режим поддержки DMA при передаче данных\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txdmaen {
    #[doc = "0: Поддержка DMA выключена"]
    Disable = 0,
    #[doc = "1: Поддержка DMA включена"]
    Enable = 1,
}
impl From<Txdmaen> for bool {
    #[inline(always)]
    fn from(variant: Txdmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDMAEN` reader - Режим поддержки DMA при передаче данных"]
pub type TxdmaenR = crate::BitReader<Txdmaen>;
impl TxdmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txdmaen {
        match self.bits {
            false => Txdmaen::Disable,
            true => Txdmaen::Enable,
        }
    }
    #[doc = "Поддержка DMA выключена"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Txdmaen::Disable
    }
    #[doc = "Поддержка DMA включена"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Txdmaen::Enable
    }
}
#[doc = "Field `TXDMAEN` writer - Режим поддержки DMA при передаче данных"]
pub type TxdmaenW<'a, REG> = crate::BitWriter<'a, REG, Txdmaen>;
impl<'a, REG> TxdmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Поддержка DMA выключена"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Txdmaen::Disable)
    }
    #[doc = "Поддержка DMA включена"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Txdmaen::Enable)
    }
}
#[doc = "Режим поддержки DMA при приеме данных\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxdmaen {
    #[doc = "0: Поддержка DMA выключена"]
    Disable = 0,
    #[doc = "1: Поддержка DMA включенa"]
    Enable = 1,
}
impl From<Rxdmaen> for bool {
    #[inline(always)]
    fn from(variant: Rxdmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDMAEN` reader - Режим поддержки DMA при приеме данных"]
pub type RxdmaenR = crate::BitReader<Rxdmaen>;
impl RxdmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxdmaen {
        match self.bits {
            false => Rxdmaen::Disable,
            true => Rxdmaen::Enable,
        }
    }
    #[doc = "Поддержка DMA выключена"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rxdmaen::Disable
    }
    #[doc = "Поддержка DMA включенa"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rxdmaen::Enable
    }
}
#[doc = "Field `RXDMAEN` writer - Режим поддержки DMA при приеме данных"]
pub type RxdmaenW<'a, REG> = crate::BitWriter<'a, REG, Rxdmaen>;
impl<'a, REG> RxdmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Поддержка DMA выключена"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdmaen::Disable)
    }
    #[doc = "Поддержка DMA включенa"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdmaen::Enable)
    }
}
#[doc = "Режим аппаратного контроля передачи данных в режиме «ведомый»\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbc {
    #[doc = "0: Контроль выключен"]
    Disable = 0,
    #[doc = "1: Контроль включен"]
    Enable = 1,
}
impl From<Sbc> for bool {
    #[inline(always)]
    fn from(variant: Sbc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBC` reader - Режим аппаратного контроля передачи данных в режиме «ведомый»"]
pub type SbcR = crate::BitReader<Sbc>;
impl SbcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sbc {
        match self.bits {
            false => Sbc::Disable,
            true => Sbc::Enable,
        }
    }
    #[doc = "Контроль выключен"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sbc::Disable
    }
    #[doc = "Контроль включен"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Sbc::Enable
    }
}
#[doc = "Field `SBC` writer - Режим аппаратного контроля передачи данных в режиме «ведомый»"]
pub type SbcW<'a, REG> = crate::BitWriter<'a, REG, Sbc>;
impl<'a, REG> SbcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Контроль выключен"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sbc::Disable)
    }
    #[doc = "Контроль включен"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Sbc::Enable)
    }
}
#[doc = "Отключение растягивания тактового сигнала в режиме «ведомый». Изменение значения допуска-ется только при выключенном блоке (PE=0). В режиме «ве-дущий» бит должен быть установлен в ‘0’\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nostretch {
    #[doc = "0: Растягивание активно"]
    StretchingEnabled = 0,
    #[doc = "1: Растягивание выключено"]
    StretchingDisable = 1,
}
impl From<Nostretch> for bool {
    #[inline(always)]
    fn from(variant: Nostretch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOSTRETCH` reader - Отключение растягивания тактового сигнала в режиме «ведомый». Изменение значения допуска-ется только при выключенном блоке (PE=0). В режиме «ве-дущий» бит должен быть установлен в ‘0’"]
pub type NostretchR = crate::BitReader<Nostretch>;
impl NostretchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nostretch {
        match self.bits {
            false => Nostretch::StretchingEnabled,
            true => Nostretch::StretchingDisable,
        }
    }
    #[doc = "Растягивание активно"]
    #[inline(always)]
    pub fn is_stretching_enabled(&self) -> bool {
        *self == Nostretch::StretchingEnabled
    }
    #[doc = "Растягивание выключено"]
    #[inline(always)]
    pub fn is_stretching_disable(&self) -> bool {
        *self == Nostretch::StretchingDisable
    }
}
#[doc = "Field `NOSTRETCH` writer - Отключение растягивания тактового сигнала в режиме «ведомый». Изменение значения допуска-ется только при выключенном блоке (PE=0). В режиме «ве-дущий» бит должен быть установлен в ‘0’"]
pub type NostretchW<'a, REG> = crate::BitWriter<'a, REG, Nostretch>;
impl<'a, REG> NostretchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Растягивание активно"]
    #[inline(always)]
    pub fn stretching_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Nostretch::StretchingEnabled)
    }
    #[doc = "Растягивание выключено"]
    #[inline(always)]
    pub fn stretching_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Nostretch::StretchingDisable)
    }
}
#[doc = "Разрешение адреса общего вызова\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gcen {
    #[doc = "0: Адрес 0b00000000 запрещен, формируется NACK"]
    Disabled = 0,
    #[doc = "1: Адрес 0b00000000 разрешен, формируется ACK"]
    Enable = 1,
}
impl From<Gcen> for bool {
    #[inline(always)]
    fn from(variant: Gcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GCEN` reader - Разрешение адреса общего вызова"]
pub type GcenR = crate::BitReader<Gcen>;
impl GcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gcen {
        match self.bits {
            false => Gcen::Disabled,
            true => Gcen::Enable,
        }
    }
    #[doc = "Адрес 0b00000000 запрещен, формируется NACK"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Gcen::Disabled
    }
    #[doc = "Адрес 0b00000000 разрешен, формируется ACK"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Gcen::Enable
    }
}
#[doc = "Field `GCEN` writer - Разрешение адреса общего вызова"]
pub type GcenW<'a, REG> = crate::BitWriter<'a, REG, Gcen>;
impl<'a, REG> GcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Адрес 0b00000000 запрещен, формируется NACK"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Gcen::Disabled)
    }
    #[doc = "Адрес 0b00000000 разрешен, формируется ACK"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Gcen::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Управление интерфейсом. После очистки, бит должен оставаться в ‘0’ минимум три периода тактового сигнала APB"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Разрешение прерывания при передаче"]
    #[inline(always)]
    pub fn txie(&self) -> TxieR {
        TxieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Разрешение прерывания при приеме"]
    #[inline(always)]
    pub fn rxie(&self) -> RxieR {
        RxieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Разрешение прерывания соот-ветствия адреса в режиме «ведомый»"]
    #[inline(always)]
    pub fn addrie(&self) -> AddrieR {
        AddrieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Разрешение прерывания прием NACK"]
    #[inline(always)]
    pub fn nackie(&self) -> NackieR {
        NackieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Разрешение прерывания обнаружения STOP на линии"]
    #[inline(always)]
    pub fn stopie(&self) -> StopieR {
        StopieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Разрешение прерывания окончания передачи. События, вызывающие прерывание: - окончание передачи (TC); - окончание передачи при RELOAD=1 (TCR)."]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Разрешение прерывания при ошибке. События, вызывающие прерывание: - потеря арбитража (ARLO); - ошибка шины (BERR); - переполне-ние/недозагрузка (OVR)."]
    #[inline(always)]
    pub fn errie(&self) -> ErrieR {
        ErrieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Управление цифровым фильтром шумов. Изменение значения допускается только при выключенном блоке (PE=0)"]
    #[inline(always)]
    pub fn dnf(&self) -> DnfR {
        DnfR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Управление аналоговым фильтром шумов"]
    #[inline(always)]
    pub fn anfoff(&self) -> AnfoffR {
        AnfoffR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Режим поддержки DMA при передаче данных"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TxdmaenR {
        TxdmaenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Режим поддержки DMA при приеме данных"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RxdmaenR {
        RxdmaenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Режим аппаратного контроля передачи данных в режиме «ведомый»"]
    #[inline(always)]
    pub fn sbc(&self) -> SbcR {
        SbcR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Отключение растягивания тактового сигнала в режиме «ведомый». Изменение значения допуска-ется только при выключенном блоке (PE=0). В режиме «ве-дущий» бит должен быть установлен в ‘0’"]
    #[inline(always)]
    pub fn nostretch(&self) -> NostretchR {
        NostretchR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Разрешение адреса общего вызова"]
    #[inline(always)]
    pub fn gcen(&self) -> GcenR {
        GcenR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Управление интерфейсом. После очистки, бит должен оставаться в ‘0’ минимум три периода тактового сигнала APB"]
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<Cr1Spec> {
        PeW::new(self, 0)
    }
    #[doc = "Bit 1 - Разрешение прерывания при передаче"]
    #[inline(always)]
    pub fn txie(&mut self) -> TxieW<Cr1Spec> {
        TxieW::new(self, 1)
    }
    #[doc = "Bit 2 - Разрешение прерывания при приеме"]
    #[inline(always)]
    pub fn rxie(&mut self) -> RxieW<Cr1Spec> {
        RxieW::new(self, 2)
    }
    #[doc = "Bit 3 - Разрешение прерывания соот-ветствия адреса в режиме «ведомый»"]
    #[inline(always)]
    pub fn addrie(&mut self) -> AddrieW<Cr1Spec> {
        AddrieW::new(self, 3)
    }
    #[doc = "Bit 4 - Разрешение прерывания прием NACK"]
    #[inline(always)]
    pub fn nackie(&mut self) -> NackieW<Cr1Spec> {
        NackieW::new(self, 4)
    }
    #[doc = "Bit 5 - Разрешение прерывания обнаружения STOP на линии"]
    #[inline(always)]
    pub fn stopie(&mut self) -> StopieW<Cr1Spec> {
        StopieW::new(self, 5)
    }
    #[doc = "Bit 6 - Разрешение прерывания окончания передачи. События, вызывающие прерывание: - окончание передачи (TC); - окончание передачи при RELOAD=1 (TCR)."]
    #[inline(always)]
    pub fn tcie(&mut self) -> TcieW<Cr1Spec> {
        TcieW::new(self, 6)
    }
    #[doc = "Bit 7 - Разрешение прерывания при ошибке. События, вызывающие прерывание: - потеря арбитража (ARLO); - ошибка шины (BERR); - переполне-ние/недозагрузка (OVR)."]
    #[inline(always)]
    pub fn errie(&mut self) -> ErrieW<Cr1Spec> {
        ErrieW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Управление цифровым фильтром шумов. Изменение значения допускается только при выключенном блоке (PE=0)"]
    #[inline(always)]
    pub fn dnf(&mut self) -> DnfW<Cr1Spec> {
        DnfW::new(self, 8)
    }
    #[doc = "Bit 12 - Управление аналоговым фильтром шумов"]
    #[inline(always)]
    pub fn anfoff(&mut self) -> AnfoffW<Cr1Spec> {
        AnfoffW::new(self, 12)
    }
    #[doc = "Bit 14 - Режим поддержки DMA при передаче данных"]
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TxdmaenW<Cr1Spec> {
        TxdmaenW::new(self, 14)
    }
    #[doc = "Bit 15 - Режим поддержки DMA при приеме данных"]
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RxdmaenW<Cr1Spec> {
        RxdmaenW::new(self, 15)
    }
    #[doc = "Bit 16 - Режим аппаратного контроля передачи данных в режиме «ведомый»"]
    #[inline(always)]
    pub fn sbc(&mut self) -> SbcW<Cr1Spec> {
        SbcW::new(self, 16)
    }
    #[doc = "Bit 17 - Отключение растягивания тактового сигнала в режиме «ведомый». Изменение значения допуска-ется только при выключенном блоке (PE=0). В режиме «ве-дущий» бит должен быть установлен в ‘0’"]
    #[inline(always)]
    pub fn nostretch(&mut self) -> NostretchW<Cr1Spec> {
        NostretchW::new(self, 17)
    }
    #[doc = "Bit 19 - Разрешение адреса общего вызова"]
    #[inline(always)]
    pub fn gcen(&mut self) -> GcenW<Cr1Spec> {
        GcenW::new(self, 19)
    }
}
#[doc = "Регистр управления 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {
    const RESET_VALUE: u32 = 0;
}
