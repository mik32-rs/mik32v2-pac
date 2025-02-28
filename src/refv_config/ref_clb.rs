#[doc = "Register `REF_CLB` reader"]
pub type R = crate::R<RefClbSpec>;
#[doc = "Register `REF_CLB` writer"]
pub type W = crate::W<RefClbSpec>;
#[doc = "Field `COEF_REFVCLB` reader - Коэфициент настройки опорного источника напряжения"]
pub type CoefRefvclbR = crate::FieldReader;
#[doc = "Field `COEF_REFVCLB` writer - Коэфициент настройки опорного источника напряжения"]
pub type CoefRefvclbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `COEF_REFICLB` reader - Коэфициент настройки опорного источника тока"]
pub type CoefReficlbR = crate::FieldReader;
#[doc = "Field `COEF_REFICLB` writer - Коэфициент настройки опорного источника тока"]
pub type CoefReficlbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Разрешение работы калибруемых источников\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClbEn {
    #[doc = "0: Включить"]
    Disable = 0,
    #[doc = "1: Выключить"]
    Enable = 1,
}
impl From<ClbEn> for bool {
    #[inline(always)]
    fn from(variant: ClbEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLB_EN` reader - Разрешение работы калибруемых источников"]
pub type ClbEnR = crate::BitReader<ClbEn>;
impl ClbEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClbEn {
        match self.bits {
            false => ClbEn::Disable,
            true => ClbEn::Enable,
        }
    }
    #[doc = "Включить"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ClbEn::Disable
    }
    #[doc = "Выключить"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ClbEn::Enable
    }
}
#[doc = "Field `CLB_EN` writer - Разрешение работы калибруемых источников"]
pub type ClbEnW<'a, REG> = crate::BitWriter<'a, REG, ClbEn>;
impl<'a, REG> ClbEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Включить"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ClbEn::Disable)
    }
    #[doc = "Выключить"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ClbEn::Enable)
    }
}
impl R {
    #[doc = "Bits 0:3 - Коэфициент настройки опорного источника напряжения"]
    #[inline(always)]
    pub fn coef_refvclb(&self) -> CoefRefvclbR {
        CoefRefvclbR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Коэфициент настройки опорного источника тока"]
    #[inline(always)]
    pub fn coef_reficlb(&self) -> CoefReficlbR {
        CoefReficlbR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Разрешение работы калибруемых источников"]
    #[inline(always)]
    pub fn clb_en(&self) -> ClbEnR {
        ClbEnR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Коэфициент настройки опорного источника напряжения"]
    #[inline(always)]
    pub fn coef_refvclb(&mut self) -> CoefRefvclbW<RefClbSpec> {
        CoefRefvclbW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Коэфициент настройки опорного источника тока"]
    #[inline(always)]
    pub fn coef_reficlb(&mut self) -> CoefReficlbW<RefClbSpec> {
        CoefReficlbW::new(self, 4)
    }
    #[doc = "Bit 8 - Разрешение работы калибруемых источников"]
    #[inline(always)]
    pub fn clb_en(&mut self) -> ClbEnW<RefClbSpec> {
        ClbEnW::new(self, 8)
    }
}
#[doc = "Управление калибруемыми источниками тока и напряжения\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_clb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_clb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RefClbSpec;
impl crate::RegisterSpec for RefClbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ref_clb::R`](R) reader structure"]
impl crate::Readable for RefClbSpec {}
#[doc = "`write(|w| ..)` method takes [`ref_clb::W`](W) writer structure"]
impl crate::Writable for RefClbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REF_CLB to value 0x0188"]
impl crate::Resettable for RefClbSpec {
    const RESET_VALUE: u32 = 0x0188;
}
