#[doc = "Register `CH3_ICR` reader"]
pub type R = crate::R<Ch3IcrSpec>;
#[doc = "Register `CH3_ICR` writer"]
pub type W = crate::W<Ch3IcrSpec>;
#[doc = "Field `ICR` reader - Значение таймера в режиме захвата"]
pub type IcrR = crate::FieldReader<u32>;
#[doc = "Field `ICR` writer - Значение таймера в режиме захвата"]
pub type IcrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Значение таймера в режиме захвата"]
    #[inline(always)]
    pub fn icr(&self) -> IcrR {
        IcrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Значение таймера в режиме захвата"]
    #[inline(always)]
    pub fn icr(&mut self) -> IcrW<Ch3IcrSpec> {
        IcrW::new(self, 0)
    }
}
#[doc = "Значение захвата 3 канала\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch3IcrSpec;
impl crate::RegisterSpec for Ch3IcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch3_icr::R`](R) reader structure"]
impl crate::Readable for Ch3IcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch3_icr::W`](W) writer structure"]
impl crate::Writable for Ch3IcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH3_ICR to value 0"]
impl crate::Resettable for Ch3IcrSpec {
    const RESET_VALUE: u32 = 0;
}
