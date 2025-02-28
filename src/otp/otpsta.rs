#[doc = "Register `OTPSTA` reader"]
pub type R = crate::R<OtpstaSpec>;
#[doc = "Блок занят (выполняется запрошенная операция)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bsy {
    #[doc = "0: Блок готов"]
    Ready = 0,
    #[doc = "1: Блок занят"]
    Busy = 1,
}
impl From<Bsy> for bool {
    #[inline(always)]
    fn from(variant: Bsy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSY` reader - Блок занят (выполняется запрошенная операция)"]
pub type BsyR = crate::BitReader<Bsy>;
impl BsyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bsy {
        match self.bits {
            false => Bsy::Ready,
            true => Bsy::Busy,
        }
    }
    #[doc = "Блок готов"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Bsy::Ready
    }
    #[doc = "Блок занят"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Bsy::Busy
    }
}
impl R {
    #[doc = "Bit 0 - Блок занят (выполняется запрошенная операция)"]
    #[inline(always)]
    pub fn bsy(&self) -> BsyR {
        BsyR::new((self.bits & 1) != 0)
    }
}
#[doc = "Регистр статуса\n\nYou can [`read`](crate::Reg::read) this register and get [`otpsta::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpstaSpec;
impl crate::RegisterSpec for OtpstaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otpsta::R`](R) reader structure"]
impl crate::Readable for OtpstaSpec {}
#[doc = "`reset()` method sets OTPSTA to value 0"]
impl crate::Resettable for OtpstaSpec {
    const RESET_VALUE: u32 = 0;
}
