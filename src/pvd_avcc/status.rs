#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `OUT_UNDER_VALUE` reader - Текущее состояние линии монитора нижнего порога без цифровой фильтрации"]
pub type OutUnderValueR = crate::BitReader;
#[doc = "Field `OUT_OVER_VALUE` reader - Текущее состояние линии монитора верхнего порога без цифровой фильтрации"]
pub type OutOverValueR = crate::BitReader;
#[doc = "Field `OUT_UNDER_FLAG` reader - Текущее состояние линии монитора нижнего порога с учетом цифровой фильтрации"]
pub type OutUnderFlagR = crate::BitReader;
#[doc = "Field `OUT_OVER_FLAG` reader - Текущее состояние линии монитора верхнего порога с учетом цифровой фильтрации"]
pub type OutOverFlagR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Текущее состояние линии монитора нижнего порога без цифровой фильтрации"]
    #[inline(always)]
    pub fn out_under_value(&self) -> OutUnderValueR {
        OutUnderValueR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Текущее состояние линии монитора верхнего порога без цифровой фильтрации"]
    #[inline(always)]
    pub fn out_over_value(&self) -> OutOverValueR {
        OutOverValueR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Текущее состояние линии монитора нижнего порога с учетом цифровой фильтрации"]
    #[inline(always)]
    pub fn out_under_flag(&self) -> OutUnderFlagR {
        OutUnderFlagR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Текущее состояние линии монитора верхнего порога с учетом цифровой фильтрации"]
    #[inline(always)]
    pub fn out_over_flag(&self) -> OutOverFlagR {
        OutOverFlagR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Регистр статуса\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
