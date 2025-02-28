#[doc = "Register `RRTC_DATE` reader"]
pub type R = crate::R<RrtcDateSpec>;
#[doc = "Register `RRTC_DATE` writer"]
pub type W = crate::W<RrtcDateSpec>;
#[doc = "Field `D` reader - Поле единиц дней. Допустимые значения: - TD = 2 - от 0 до 3; - TD = 3 - от 0 до 1"]
pub type DR = crate::FieldReader;
#[doc = "Field `D` writer - Поле единиц дней. Допустимые значения: - TD = 2 - от 0 до 3; - TD = 3 - от 0 до 1"]
pub type DW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TD` reader - Поле десятков дней. Допустимые значения: - {TM,M} != 03 - от 0 до 3; - {TM,M} == 03 - от 0 до 2"]
pub type TdR = crate::FieldReader;
#[doc = "Field `TD` writer - Поле десятков дней. Допустимые значения: - {TM,M} != 03 - от 0 до 3; - {TM,M} == 03 - от 0 до 2"]
pub type TdW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `M` reader - Поле единиц месяцев. Допустимые значения: - TM = 0 - от 0 до 9; - TM = 1 - от 0 до 2"]
pub type MR = crate::FieldReader;
#[doc = "Field `M` writer - Поле единиц месяцев. Допустимые значения: - TM = 0 - от 0 до 9; - TM = 1 - от 0 до 2"]
pub type MW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TM` reader - Поле десятков месяцев. Допустимые значения от 0 до 1"]
pub type TmR = crate::BitReader;
#[doc = "Field `TM` writer - Поле десятков месяцев. Допустимые значения от 0 до 1"]
pub type TmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Y` reader - Поле единиц годов. Допустимые значения от 0 до 9"]
pub type YR = crate::FieldReader;
#[doc = "Field `Y` writer - Поле единиц годов. Допустимые значения от 0 до 9"]
pub type YW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TC` reader - Поле десятков веков. Допустимые значения от 0 до 9"]
pub type TcR = crate::FieldReader<u16>;
#[doc = "Field `TC` writer - Поле десятков веков. Допустимые значения от 0 до 9"]
pub type TcW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `TY` reader - Поле десятков годов. Допустимые значения от 0 до 9"]
pub type TyR = crate::FieldReader;
#[doc = "Field `TY` writer - Поле десятков годов. Допустимые значения от 0 до 9"]
pub type TyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `C` reader - Поле единиц веков. Допустимые значения от 0 до 9"]
pub type CR = crate::FieldReader;
#[doc = "Field `C` writer - Поле единиц веков. Допустимые значения от 0 до 9"]
pub type CW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Поле единиц дней. Допустимые значения: - TD = 2 - от 0 до 3; - TD = 3 - от 0 до 1"]
    #[inline(always)]
    pub fn d(&self) -> DR {
        DR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Поле десятков дней. Допустимые значения: - {TM,M} != 03 - от 0 до 3; - {TM,M} == 03 - от 0 до 2"]
    #[inline(always)]
    pub fn td(&self) -> TdR {
        TdR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:9 - Поле единиц месяцев. Допустимые значения: - TM = 0 - от 0 до 9; - TM = 1 - от 0 до 2"]
    #[inline(always)]
    pub fn m(&self) -> MR {
        MR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - Поле десятков месяцев. Допустимые значения от 0 до 1"]
    #[inline(always)]
    pub fn tm(&self) -> TmR {
        TmR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - Поле единиц годов. Допустимые значения от 0 до 9"]
    #[inline(always)]
    pub fn y(&self) -> YR {
        YR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 13:26 - Поле десятков веков. Допустимые значения от 0 до 9"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 13) & 0x3fff) as u16)
    }
    #[doc = "Bits 15:18 - Поле десятков годов. Допустимые значения от 0 до 9"]
    #[inline(always)]
    pub fn ty(&self) -> TyR {
        TyR::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:22 - Поле единиц веков. Допустимые значения от 0 до 9"]
    #[inline(always)]
    pub fn c(&self) -> CR {
        CR::new(((self.bits >> 19) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Поле единиц дней. Допустимые значения: - TD = 2 - от 0 до 3; - TD = 3 - от 0 до 1"]
    #[inline(always)]
    pub fn d(&mut self) -> DW<RrtcDateSpec> {
        DW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Поле десятков дней. Допустимые значения: - {TM,M} != 03 - от 0 до 3; - {TM,M} == 03 - от 0 до 2"]
    #[inline(always)]
    pub fn td(&mut self) -> TdW<RrtcDateSpec> {
        TdW::new(self, 4)
    }
    #[doc = "Bits 6:9 - Поле единиц месяцев. Допустимые значения: - TM = 0 - от 0 до 9; - TM = 1 - от 0 до 2"]
    #[inline(always)]
    pub fn m(&mut self) -> MW<RrtcDateSpec> {
        MW::new(self, 6)
    }
    #[doc = "Bit 10 - Поле десятков месяцев. Допустимые значения от 0 до 1"]
    #[inline(always)]
    pub fn tm(&mut self) -> TmW<RrtcDateSpec> {
        TmW::new(self, 10)
    }
    #[doc = "Bits 11:14 - Поле единиц годов. Допустимые значения от 0 до 9"]
    #[inline(always)]
    pub fn y(&mut self) -> YW<RrtcDateSpec> {
        YW::new(self, 11)
    }
    #[doc = "Bits 13:26 - Поле десятков веков. Допустимые значения от 0 до 9"]
    #[inline(always)]
    pub fn tc(&mut self) -> TcW<RrtcDateSpec> {
        TcW::new(self, 13)
    }
    #[doc = "Bits 15:18 - Поле десятков годов. Допустимые значения от 0 до 9"]
    #[inline(always)]
    pub fn ty(&mut self) -> TyW<RrtcDateSpec> {
        TyW::new(self, 15)
    }
    #[doc = "Bits 19:22 - Поле единиц веков. Допустимые значения от 0 до 9"]
    #[inline(always)]
    pub fn c(&mut self) -> CW<RrtcDateSpec> {
        CW::new(self, 19)
    }
}
#[doc = "Регистр установки даты. Используется BCD-кодировка\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RrtcDateSpec;
impl crate::RegisterSpec for RrtcDateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rrtc_date::R`](R) reader structure"]
impl crate::Readable for RrtcDateSpec {}
#[doc = "`write(|w| ..)` method takes [`rrtc_date::W`](W) writer structure"]
impl crate::Writable for RrtcDateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RRTC_DATE to value 0"]
impl crate::Resettable for RrtcDateSpec {
    const RESET_VALUE: u32 = 0;
}
