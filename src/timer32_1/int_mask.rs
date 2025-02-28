#[doc = "Register `INT_MASK` reader"]
pub type R = crate::R<IntMaskSpec>;
#[doc = "Register `INT_MASK` writer"]
pub type W = crate::W<IntMaskSpec>;
#[doc = "Field `OVF_Int` reader - Маска прерывания по переполнению счетчика"]
pub type OvfIntR = crate::BitReader;
#[doc = "Field `OVF_Int` writer - Маска прерывания по переполнению счетчика"]
pub type OvfIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDF_Int` reader - Маска прерывания опустошения счетчика"]
pub type UdfIntR = crate::BitReader;
#[doc = "Field `UDF_Int` writer - Маска прерывания опустошения счетчика"]
pub type UdfIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IC_Int_CH1` reader - Маска прерывания захвата 1 канала таймера"]
pub type IcIntCh1R = crate::BitReader;
#[doc = "Field `IC_Int_CH1` writer - Маска прерывания захвата 1 канала таймера"]
pub type IcIntCh1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IC_Int_CH2` reader - Маска прерывания захвата 2 канала таймера"]
pub type IcIntCh2R = crate::BitReader;
#[doc = "Field `IC_Int_CH2` writer - Маска прерывания захвата 2 канала таймера"]
pub type IcIntCh2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IC_Int_CH3` reader - Маска прерывания захвата 3 канала таймера"]
pub type IcIntCh3R = crate::BitReader;
#[doc = "Field `IC_Int_CH3` writer - Маска прерывания захвата 3 канала таймера"]
pub type IcIntCh3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IC_Int_CH4` reader - Маска прерывания захвата 4 канала таймера"]
pub type IcIntCh4R = crate::BitReader;
#[doc = "Field `IC_Int_CH4` writer - Маска прерывания захвата 4 канала таймера"]
pub type IcIntCh4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC_Int_CH1` reader - Маска прерывания совпадения 1 канала таймера"]
pub type OcIntCh1R = crate::BitReader;
#[doc = "Field `OC_Int_CH1` writer - Маска прерывания совпадения 1 канала таймера"]
pub type OcIntCh1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC_Int_CH2` reader - Маска прерывания совпадения 2 канала таймера"]
pub type OcIntCh2R = crate::BitReader;
#[doc = "Field `OC_Int_CH2` writer - Маска прерывания совпадения 2 канала таймера"]
pub type OcIntCh2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC_Int_CH3` reader - Маска прерывания совпадения 3 канала таймера"]
pub type OcIntCh3R = crate::BitReader;
#[doc = "Field `OC_Int_CH3` writer - Маска прерывания совпадения 3 канала таймера"]
pub type OcIntCh3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC_Int_CH4` reader - Маска прерывания совпадения 4 канала таймера"]
pub type OcIntCh4R = crate::BitReader;
#[doc = "Field `OC_Int_CH4` writer - Маска прерывания совпадения 4 канала таймера"]
pub type OcIntCh4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Маска прерывания по переполнению счетчика"]
    #[inline(always)]
    pub fn ovf_int(&self) -> OvfIntR {
        OvfIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Маска прерывания опустошения счетчика"]
    #[inline(always)]
    pub fn udf_int(&self) -> UdfIntR {
        UdfIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Маска прерывания захвата 1 канала таймера"]
    #[inline(always)]
    pub fn ic_int_ch1(&self) -> IcIntCh1R {
        IcIntCh1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Маска прерывания захвата 2 канала таймера"]
    #[inline(always)]
    pub fn ic_int_ch2(&self) -> IcIntCh2R {
        IcIntCh2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Маска прерывания захвата 3 канала таймера"]
    #[inline(always)]
    pub fn ic_int_ch3(&self) -> IcIntCh3R {
        IcIntCh3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Маска прерывания захвата 4 канала таймера"]
    #[inline(always)]
    pub fn ic_int_ch4(&self) -> IcIntCh4R {
        IcIntCh4R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Маска прерывания совпадения 1 канала таймера"]
    #[inline(always)]
    pub fn oc_int_ch1(&self) -> OcIntCh1R {
        OcIntCh1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Маска прерывания совпадения 2 канала таймера"]
    #[inline(always)]
    pub fn oc_int_ch2(&self) -> OcIntCh2R {
        OcIntCh2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Маска прерывания совпадения 3 канала таймера"]
    #[inline(always)]
    pub fn oc_int_ch3(&self) -> OcIntCh3R {
        OcIntCh3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Маска прерывания совпадения 4 канала таймера"]
    #[inline(always)]
    pub fn oc_int_ch4(&self) -> OcIntCh4R {
        OcIntCh4R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Маска прерывания по переполнению счетчика"]
    #[inline(always)]
    pub fn ovf_int(&mut self) -> OvfIntW<IntMaskSpec> {
        OvfIntW::new(self, 0)
    }
    #[doc = "Bit 1 - Маска прерывания опустошения счетчика"]
    #[inline(always)]
    pub fn udf_int(&mut self) -> UdfIntW<IntMaskSpec> {
        UdfIntW::new(self, 1)
    }
    #[doc = "Bit 2 - Маска прерывания захвата 1 канала таймера"]
    #[inline(always)]
    pub fn ic_int_ch1(&mut self) -> IcIntCh1W<IntMaskSpec> {
        IcIntCh1W::new(self, 2)
    }
    #[doc = "Bit 3 - Маска прерывания захвата 2 канала таймера"]
    #[inline(always)]
    pub fn ic_int_ch2(&mut self) -> IcIntCh2W<IntMaskSpec> {
        IcIntCh2W::new(self, 3)
    }
    #[doc = "Bit 4 - Маска прерывания захвата 3 канала таймера"]
    #[inline(always)]
    pub fn ic_int_ch3(&mut self) -> IcIntCh3W<IntMaskSpec> {
        IcIntCh3W::new(self, 4)
    }
    #[doc = "Bit 5 - Маска прерывания захвата 4 канала таймера"]
    #[inline(always)]
    pub fn ic_int_ch4(&mut self) -> IcIntCh4W<IntMaskSpec> {
        IcIntCh4W::new(self, 5)
    }
    #[doc = "Bit 6 - Маска прерывания совпадения 1 канала таймера"]
    #[inline(always)]
    pub fn oc_int_ch1(&mut self) -> OcIntCh1W<IntMaskSpec> {
        OcIntCh1W::new(self, 6)
    }
    #[doc = "Bit 7 - Маска прерывания совпадения 2 канала таймера"]
    #[inline(always)]
    pub fn oc_int_ch2(&mut self) -> OcIntCh2W<IntMaskSpec> {
        OcIntCh2W::new(self, 7)
    }
    #[doc = "Bit 8 - Маска прерывания совпадения 3 канала таймера"]
    #[inline(always)]
    pub fn oc_int_ch3(&mut self) -> OcIntCh3W<IntMaskSpec> {
        OcIntCh3W::new(self, 8)
    }
    #[doc = "Bit 9 - Маска прерывания совпадения 4 канала таймера"]
    #[inline(always)]
    pub fn oc_int_ch4(&mut self) -> OcIntCh4W<IntMaskSpec> {
        OcIntCh4W::new(self, 9)
    }
}
#[doc = "Регистр маски прерываний\n\nYou can [`read`](crate::Reg::read) this register and get [`int_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntMaskSpec;
impl crate::RegisterSpec for IntMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_mask::R`](R) reader structure"]
impl crate::Readable for IntMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`int_mask::W`](W) writer structure"]
impl crate::Writable for IntMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_MASK to value 0"]
impl crate::Resettable for IntMaskSpec {
    const RESET_VALUE: u32 = 0;
}
