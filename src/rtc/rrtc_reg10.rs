#[doc = "Register `RRTC_REG10` reader"]
pub type R = crate::R<RrtcReg10Spec>;
#[doc = "Register `RRTC_REG10` writer"]
pub type W = crate::W<RrtcReg10Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Регистры общего назначения REG10\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_reg10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_reg10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RrtcReg10Spec;
impl crate::RegisterSpec for RrtcReg10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rrtc_reg10::R`](R) reader structure"]
impl crate::Readable for RrtcReg10Spec {}
#[doc = "`write(|w| ..)` method takes [`rrtc_reg10::W`](W) writer structure"]
impl crate::Writable for RrtcReg10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RRTC_REG10 to value 0"]
impl crate::Resettable for RrtcReg10Spec {
    const RESET_VALUE: u32 = 0;
}
