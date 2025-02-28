#[doc = "Register `OTPA` reader"]
pub type R = crate::R<OtpaSpec>;
#[doc = "Register `OTPA` writer"]
pub type W = crate::W<OtpaSpec>;
#[doc = "Адрес слова для выполнения записи/чтения.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Addr {
    #[doc = "0: Номер строки основного массива OTP"]
    MainArray = 0,
    #[doc = "4: Тестовая строка OTP"]
    TestRow = 4,
    #[doc = "8: Тестовый столбец OTP"]
    TestColumn = 8,
    #[doc = "12: Тестовый бит"]
    TestBit = 12,
}
impl From<Addr> for u8 {
    #[inline(always)]
    fn from(variant: Addr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Addr {
    type Ux = u8;
}
impl crate::IsEnum for Addr {}
#[doc = "Field `ADDR` reader - Адрес слова для выполнения записи/чтения."]
pub type AddrR = crate::FieldReader<Addr>;
impl AddrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Addr> {
        match self.bits {
            0 => Some(Addr::MainArray),
            4 => Some(Addr::TestRow),
            8 => Some(Addr::TestColumn),
            12 => Some(Addr::TestBit),
            _ => None,
        }
    }
    #[doc = "Номер строки основного массива OTP"]
    #[inline(always)]
    pub fn is_main_array(&self) -> bool {
        *self == Addr::MainArray
    }
    #[doc = "Тестовая строка OTP"]
    #[inline(always)]
    pub fn is_test_row(&self) -> bool {
        *self == Addr::TestRow
    }
    #[doc = "Тестовый столбец OTP"]
    #[inline(always)]
    pub fn is_test_column(&self) -> bool {
        *self == Addr::TestColumn
    }
    #[doc = "Тестовый бит"]
    #[inline(always)]
    pub fn is_test_bit(&self) -> bool {
        *self == Addr::TestBit
    }
}
#[doc = "Field `ADDR` writer - Адрес слова для выполнения записи/чтения."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 5, Addr>;
impl<'a, REG> AddrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Номер строки основного массива OTP"]
    #[inline(always)]
    pub fn main_array(self) -> &'a mut crate::W<REG> {
        self.variant(Addr::MainArray)
    }
    #[doc = "Тестовая строка OTP"]
    #[inline(always)]
    pub fn test_row(self) -> &'a mut crate::W<REG> {
        self.variant(Addr::TestRow)
    }
    #[doc = "Тестовый столбец OTP"]
    #[inline(always)]
    pub fn test_column(self) -> &'a mut crate::W<REG> {
        self.variant(Addr::TestColumn)
    }
    #[doc = "Тестовый бит"]
    #[inline(always)]
    pub fn test_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Addr::TestBit)
    }
}
impl R {
    #[doc = "Bits 0:4 - Адрес слова для выполнения записи/чтения."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Адрес слова для выполнения записи/чтения."]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<OtpaSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Регистр адреса страницы\n\nYou can [`read`](crate::Reg::read) this register and get [`otpa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otpa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpaSpec;
impl crate::RegisterSpec for OtpaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otpa::R`](R) reader structure"]
impl crate::Readable for OtpaSpec {}
#[doc = "`write(|w| ..)` method takes [`otpa::W`](W) writer structure"]
impl crate::Writable for OtpaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTPA to value 0"]
impl crate::Resettable for OtpaSpec {
    const RESET_VALUE: u32 = 0;
}
