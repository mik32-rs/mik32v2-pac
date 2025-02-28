#[doc = "Register `MODEM` reader"]
pub type R = crate::R<ModemSpec>;
#[doc = "Register `MODEM` writer"]
pub type W = crate::W<ModemSpec>;
#[doc = "Флаг взводится при изме-нении входного сигнала DSR. Флаг сбрасывается записью 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsrif {
    #[doc = "0: Изменений сигнала не обнаружено с момента сброса флага"]
    _0 = 0,
    #[doc = "1: Сигнал изменен с момента сброса флага"]
    _1 = 1,
}
impl From<Dsrif> for bool {
    #[inline(always)]
    fn from(variant: Dsrif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSRIF` reader - Флаг взводится при изме-нении входного сигнала DSR. Флаг сбрасывается записью 1"]
pub type DsrifR = crate::BitReader<Dsrif>;
impl DsrifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsrif {
        match self.bits {
            false => Dsrif::_0,
            true => Dsrif::_1,
        }
    }
    #[doc = "Изменений сигнала не обнаружено с момента сброса флага"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dsrif::_0
    }
    #[doc = "Сигнал изменен с момента сброса флага"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dsrif::_1
    }
}
#[doc = "Field `DSRIF` writer - Флаг взводится при изме-нении входного сигнала DSR. Флаг сбрасывается записью 1"]
pub type DsrifW<'a, REG> = crate::BitWriter1C<'a, REG, Dsrif>;
impl<'a, REG> DsrifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Изменений сигнала не обнаружено с момента сброса флага"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dsrif::_0)
    }
    #[doc = "Сигнал изменен с момента сброса флага"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dsrif::_1)
    }
}
#[doc = "Флаг взводится при изме-нении входного сигнала RI с 0 на 1. Флаг сбрасывается записью 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Riif {
    #[doc = "0: Изменений сигнала с 0 на 1 не обнаружено с момента сброса флага"]
    _0 = 0,
    #[doc = "1: Сигнал изменен с момента сброса флага"]
    _1 = 1,
}
impl From<Riif> for bool {
    #[inline(always)]
    fn from(variant: Riif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIIF` reader - Флаг взводится при изме-нении входного сигнала RI с 0 на 1. Флаг сбрасывается записью 1"]
pub type RiifR = crate::BitReader<Riif>;
impl RiifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Riif {
        match self.bits {
            false => Riif::_0,
            true => Riif::_1,
        }
    }
    #[doc = "Изменений сигнала с 0 на 1 не обнаружено с момента сброса флага"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Riif::_0
    }
    #[doc = "Сигнал изменен с момента сброса флага"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Riif::_1
    }
}
#[doc = "Field `RIIF` writer - Флаг взводится при изме-нении входного сигнала RI с 0 на 1. Флаг сбрасывается записью 1"]
pub type RiifW<'a, REG> = crate::BitWriter1C<'a, REG, Riif>;
impl<'a, REG> RiifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Изменений сигнала с 0 на 1 не обнаружено с момента сброса флага"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Riif::_0)
    }
    #[doc = "Сигнал изменен с момента сброса флага"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Riif::_1)
    }
}
#[doc = "Флаг взводится при изменении входного сигнала DCD. Флаг сбрасывается записью 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcdif {
    #[doc = "0: Изменений сигнала не обнаружено с момента сброса флага"]
    _0 = 0,
    #[doc = "1: Сигнал изменен с момента сброса флага"]
    _1 = 1,
}
impl From<Dcdif> for bool {
    #[inline(always)]
    fn from(variant: Dcdif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCDIF` reader - Флаг взводится при изменении входного сигнала DCD. Флаг сбрасывается записью 1"]
pub type DcdifR = crate::BitReader<Dcdif>;
impl DcdifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcdif {
        match self.bits {
            false => Dcdif::_0,
            true => Dcdif::_1,
        }
    }
    #[doc = "Изменений сигнала не обнаружено с момента сброса флага"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dcdif::_0
    }
    #[doc = "Сигнал изменен с момента сброса флага"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dcdif::_1
    }
}
#[doc = "Field `DCDIF` writer - Флаг взводится при изменении входного сигнала DCD. Флаг сбрасывается записью 1"]
pub type DcdifW<'a, REG> = crate::BitWriter1C<'a, REG, Dcdif>;
impl<'a, REG> DcdifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Изменений сигнала не обнаружено с момента сброса флага"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Dcdif::_0)
    }
    #[doc = "Сигнал изменен с момента сброса флага"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Dcdif::_1)
    }
}
#[doc = "Регистр содержит текущее значение сигнала DSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dsr {
    #[doc = "0: Источник данных не готов"]
    Unready = 0,
    #[doc = "1: Источник данных готов"]
    Ready = 1,
}
impl From<Dsr> for bool {
    #[inline(always)]
    fn from(variant: Dsr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSR` reader - Регистр содержит текущее значение сигнала DSR"]
pub type DsrR = crate::BitReader<Dsr>;
impl DsrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsr {
        match self.bits {
            false => Dsr::Unready,
            true => Dsr::Ready,
        }
    }
    #[doc = "Источник данных не готов"]
    #[inline(always)]
    pub fn is_unready(&self) -> bool {
        *self == Dsr::Unready
    }
    #[doc = "Источник данных готов"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Dsr::Ready
    }
}
#[doc = "Регистр содержит текущее значение сигнала RI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ri {
    #[doc = "0: Отсутствие звонка"]
    _0 = 0,
    #[doc = "1: Звонок (вызов) на телефон-ной линии"]
    _1 = 1,
}
impl From<Ri> for bool {
    #[inline(always)]
    fn from(variant: Ri) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RI` reader - Регистр содержит текущее значение сигнала RI"]
pub type RiR = crate::BitReader<Ri>;
impl RiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ri {
        match self.bits {
            false => Ri::_0,
            true => Ri::_1,
        }
    }
    #[doc = "Отсутствие звонка"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ri::_0
    }
    #[doc = "Звонок (вызов) на телефон-ной линии"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ri::_1
    }
}
#[doc = "Регистр содержит текущее значение сигнала DCD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcd {
    #[doc = "0: Отсутствие несущей"]
    _0 = 0,
    #[doc = "1: Обнаружение несущей"]
    _1 = 1,
}
impl From<Dcd> for bool {
    #[inline(always)]
    fn from(variant: Dcd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCD` reader - Регистр содержит текущее значение сигнала DCD"]
pub type DcdR = crate::BitReader<Dcd>;
impl DcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcd {
        match self.bits {
            false => Dcd::_0,
            true => Dcd::_1,
        }
    }
    #[doc = "Отсутствие несущей"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Dcd::_0
    }
    #[doc = "Обнаружение несущей"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Dcd::_1
    }
}
#[doc = "Регистр управления сигналом DTR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtr {
    #[doc = "0: Отсутствие сигнала готов-ности"]
    Unready = 0,
    #[doc = "1: Готовность к приему дан-ных"]
    Ready = 1,
}
impl From<Dtr> for bool {
    #[inline(always)]
    fn from(variant: Dtr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTR` reader - Регистр управления сигналом DTR"]
pub type DtrR = crate::BitReader<Dtr>;
impl DtrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtr {
        match self.bits {
            false => Dtr::Unready,
            true => Dtr::Ready,
        }
    }
    #[doc = "Отсутствие сигнала готов-ности"]
    #[inline(always)]
    pub fn is_unready(&self) -> bool {
        *self == Dtr::Unready
    }
    #[doc = "Готовность к приему дан-ных"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Dtr::Ready
    }
}
#[doc = "Field `DTR` writer - Регистр управления сигналом DTR"]
pub type DtrW<'a, REG> = crate::BitWriter<'a, REG, Dtr>;
impl<'a, REG> DtrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Отсутствие сигнала готов-ности"]
    #[inline(always)]
    pub fn unready(self) -> &'a mut crate::W<REG> {
        self.variant(Dtr::Unready)
    }
    #[doc = "Готовность к приему дан-ных"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(Dtr::Ready)
    }
}
impl R {
    #[doc = "Bit 1 - Флаг взводится при изме-нении входного сигнала DSR. Флаг сбрасывается записью 1"]
    #[inline(always)]
    pub fn dsrif(&self) -> DsrifR {
        DsrifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Флаг взводится при изме-нении входного сигнала RI с 0 на 1. Флаг сбрасывается записью 1"]
    #[inline(always)]
    pub fn riif(&self) -> RiifR {
        RiifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Флаг взводится при изменении входного сигнала DCD. Флаг сбрасывается записью 1"]
    #[inline(always)]
    pub fn dcdif(&self) -> DcdifR {
        DcdifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Регистр содержит текущее значение сигнала DSR"]
    #[inline(always)]
    pub fn dsr(&self) -> DsrR {
        DsrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Регистр содержит текущее значение сигнала RI"]
    #[inline(always)]
    pub fn ri(&self) -> RiR {
        RiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Регистр содержит текущее значение сигнала DCD"]
    #[inline(always)]
    pub fn dcd(&self) -> DcdR {
        DcdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - Регистр управления сигналом DTR"]
    #[inline(always)]
    pub fn dtr(&self) -> DtrR {
        DtrR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Флаг взводится при изме-нении входного сигнала DSR. Флаг сбрасывается записью 1"]
    #[inline(always)]
    pub fn dsrif(&mut self) -> DsrifW<ModemSpec> {
        DsrifW::new(self, 1)
    }
    #[doc = "Bit 2 - Флаг взводится при изме-нении входного сигнала RI с 0 на 1. Флаг сбрасывается записью 1"]
    #[inline(always)]
    pub fn riif(&mut self) -> RiifW<ModemSpec> {
        RiifW::new(self, 2)
    }
    #[doc = "Bit 3 - Флаг взводится при изменении входного сигнала DCD. Флаг сбрасывается записью 1"]
    #[inline(always)]
    pub fn dcdif(&mut self) -> DcdifW<ModemSpec> {
        DcdifW::new(self, 3)
    }
    #[doc = "Bit 12 - Регистр управления сигналом DTR"]
    #[inline(always)]
    pub fn dtr(&mut self) -> DtrW<ModemSpec> {
        DtrW::new(self, 12)
    }
}
#[doc = "Регистр управления модемом\n\nYou can [`read`](crate::Reg::read) this register and get [`modem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModemSpec;
impl crate::RegisterSpec for ModemSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modem::R`](R) reader structure"]
impl crate::Readable for ModemSpec {}
#[doc = "`write(|w| ..)` method takes [`modem::W`](W) writer structure"]
impl crate::Writable for ModemSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0e;
}
#[doc = "`reset()` method sets MODEM to value 0"]
impl crate::Resettable for ModemSpec {
    const RESET_VALUE: u32 = 0;
}
