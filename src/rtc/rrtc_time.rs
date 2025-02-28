#[doc = "Register `RRTC_TIME` reader"]
pub type R = crate::R<RrtcTimeSpec>;
#[doc = "Register `RRTC_TIME` writer"]
pub type W = crate::W<RrtcTimeSpec>;
#[doc = "Field `S` reader - Поле единиц секунд. Допустимые значения от 0 до 9"]
pub type SR = crate::FieldReader;
#[doc = "Field `S` writer - Поле единиц секунд. Допустимые значения от 0 до 9"]
pub type SW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TS` reader - Поле десятков секунд. Допустимые значения от 0 до 5"]
pub type TsR = crate::FieldReader;
#[doc = "Field `TS` writer - Поле десятков секунд. Допустимые значения от 0 до 5"]
pub type TsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `M` reader - Поле единиц минут. Допустимые значения от 0 до 9"]
pub type MR = crate::FieldReader;
#[doc = "Field `M` writer - Поле единиц минут. Допустимые значения от 0 до 9"]
pub type MW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TM` reader - Поле десятков минут. Допустимые значения от 0 до 5"]
pub type TmR = crate::FieldReader;
#[doc = "Field `TM` writer - Поле десятков минут. Допустимые значения от 0 до 5"]
pub type TmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `H` reader - Поле единиц часов. Допустимые значения: - TH = 0 - От 0 до 9; - TH = 2 - От 0 до 3"]
pub type HR = crate::FieldReader;
#[doc = "Field `H` writer - Поле единиц часов. Допустимые значения: - TH = 0 - От 0 до 9; - TH = 2 - От 0 до 3"]
pub type HW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TH` reader - Поле десятков часов. Доступные значения от 0 до 2"]
pub type ThR = crate::FieldReader;
#[doc = "Field `TH` writer - Поле десятков часов. Доступные значения от 0 до 2"]
pub type ThW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "День недели\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DayOfWeek {
    #[doc = "1: Понедельник"]
    Monday = 1,
    #[doc = "2: Вторник"]
    Tuesday = 2,
    #[doc = "3: Среда"]
    Wednesday = 3,
    #[doc = "4: Четверг"]
    Thursday = 4,
    #[doc = "5: Пятница"]
    Friday = 5,
    #[doc = "6: Суббота"]
    Saturday = 6,
    #[doc = "7: Воскресенье"]
    Sunday = 7,
}
impl From<DayOfWeek> for u8 {
    #[inline(always)]
    fn from(variant: DayOfWeek) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DayOfWeek {
    type Ux = u8;
}
impl crate::IsEnum for DayOfWeek {}
#[doc = "Field `DOW` reader - День недели"]
pub type DowR = crate::FieldReader<DayOfWeek>;
impl DowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DayOfWeek> {
        match self.bits {
            1 => Some(DayOfWeek::Monday),
            2 => Some(DayOfWeek::Tuesday),
            3 => Some(DayOfWeek::Wednesday),
            4 => Some(DayOfWeek::Thursday),
            5 => Some(DayOfWeek::Friday),
            6 => Some(DayOfWeek::Saturday),
            7 => Some(DayOfWeek::Sunday),
            _ => None,
        }
    }
    #[doc = "Понедельник"]
    #[inline(always)]
    pub fn is_monday(&self) -> bool {
        *self == DayOfWeek::Monday
    }
    #[doc = "Вторник"]
    #[inline(always)]
    pub fn is_tuesday(&self) -> bool {
        *self == DayOfWeek::Tuesday
    }
    #[doc = "Среда"]
    #[inline(always)]
    pub fn is_wednesday(&self) -> bool {
        *self == DayOfWeek::Wednesday
    }
    #[doc = "Четверг"]
    #[inline(always)]
    pub fn is_thursday(&self) -> bool {
        *self == DayOfWeek::Thursday
    }
    #[doc = "Пятница"]
    #[inline(always)]
    pub fn is_friday(&self) -> bool {
        *self == DayOfWeek::Friday
    }
    #[doc = "Суббота"]
    #[inline(always)]
    pub fn is_saturday(&self) -> bool {
        *self == DayOfWeek::Saturday
    }
    #[doc = "Воскресенье"]
    #[inline(always)]
    pub fn is_sunday(&self) -> bool {
        *self == DayOfWeek::Sunday
    }
}
#[doc = "Field `DOW` writer - День недели"]
pub type DowW<'a, REG> = crate::FieldWriter<'a, REG, 3, DayOfWeek>;
impl<'a, REG> DowW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Понедельник"]
    #[inline(always)]
    pub fn monday(self) -> &'a mut crate::W<REG> {
        self.variant(DayOfWeek::Monday)
    }
    #[doc = "Вторник"]
    #[inline(always)]
    pub fn tuesday(self) -> &'a mut crate::W<REG> {
        self.variant(DayOfWeek::Tuesday)
    }
    #[doc = "Среда"]
    #[inline(always)]
    pub fn wednesday(self) -> &'a mut crate::W<REG> {
        self.variant(DayOfWeek::Wednesday)
    }
    #[doc = "Четверг"]
    #[inline(always)]
    pub fn thursday(self) -> &'a mut crate::W<REG> {
        self.variant(DayOfWeek::Thursday)
    }
    #[doc = "Пятница"]
    #[inline(always)]
    pub fn friday(self) -> &'a mut crate::W<REG> {
        self.variant(DayOfWeek::Friday)
    }
    #[doc = "Суббота"]
    #[inline(always)]
    pub fn saturday(self) -> &'a mut crate::W<REG> {
        self.variant(DayOfWeek::Saturday)
    }
    #[doc = "Воскресенье"]
    #[inline(always)]
    pub fn sunday(self) -> &'a mut crate::W<REG> {
        self.variant(DayOfWeek::Sunday)
    }
}
impl R {
    #[doc = "Bits 4:7 - Поле единиц секунд. Допустимые значения от 0 до 9"]
    #[inline(always)]
    pub fn s(&self) -> SR {
        SR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Поле десятков секунд. Допустимые значения от 0 до 5"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:14 - Поле единиц минут. Допустимые значения от 0 до 9"]
    #[inline(always)]
    pub fn m(&self) -> MR {
        MR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 15:17 - Поле десятков минут. Допустимые значения от 0 до 5"]
    #[inline(always)]
    pub fn tm(&self) -> TmR {
        TmR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:21 - Поле единиц часов. Допустимые значения: - TH = 0 - От 0 до 9; - TH = 2 - От 0 до 3"]
    #[inline(always)]
    pub fn h(&self) -> HR {
        HR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - Поле десятков часов. Доступные значения от 0 до 2"]
    #[inline(always)]
    pub fn th(&self) -> ThR {
        ThR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:26 - День недели"]
    #[inline(always)]
    pub fn dow(&self) -> DowR {
        DowR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7 - Поле единиц секунд. Допустимые значения от 0 до 9"]
    #[inline(always)]
    pub fn s(&mut self) -> SW<RrtcTimeSpec> {
        SW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Поле десятков секунд. Допустимые значения от 0 до 5"]
    #[inline(always)]
    pub fn ts(&mut self) -> TsW<RrtcTimeSpec> {
        TsW::new(self, 8)
    }
    #[doc = "Bits 11:14 - Поле единиц минут. Допустимые значения от 0 до 9"]
    #[inline(always)]
    pub fn m(&mut self) -> MW<RrtcTimeSpec> {
        MW::new(self, 11)
    }
    #[doc = "Bits 15:17 - Поле десятков минут. Допустимые значения от 0 до 5"]
    #[inline(always)]
    pub fn tm(&mut self) -> TmW<RrtcTimeSpec> {
        TmW::new(self, 15)
    }
    #[doc = "Bits 18:21 - Поле единиц часов. Допустимые значения: - TH = 0 - От 0 до 9; - TH = 2 - От 0 до 3"]
    #[inline(always)]
    pub fn h(&mut self) -> HW<RrtcTimeSpec> {
        HW::new(self, 18)
    }
    #[doc = "Bits 22:23 - Поле десятков часов. Доступные значения от 0 до 2"]
    #[inline(always)]
    pub fn th(&mut self) -> ThW<RrtcTimeSpec> {
        ThW::new(self, 22)
    }
    #[doc = "Bits 24:26 - День недели"]
    #[inline(always)]
    pub fn dow(&mut self) -> DowW<RrtcTimeSpec> {
        DowW::new(self, 24)
    }
}
#[doc = "Регистр установки времени. Используется BCD-кодировка\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RrtcTimeSpec;
impl crate::RegisterSpec for RrtcTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rrtc_time::R`](R) reader structure"]
impl crate::Readable for RrtcTimeSpec {}
#[doc = "`write(|w| ..)` method takes [`rrtc_time::W`](W) writer structure"]
impl crate::Writable for RrtcTimeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RRTC_TIME to value 0"]
impl crate::Resettable for RrtcTimeSpec {
    const RESET_VALUE: u32 = 0;
}
