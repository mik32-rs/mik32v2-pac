#[doc = "Register `RRTC_REG3` reader"]
pub type R = crate::R<RrtcReg3Spec>;
#[doc = "Register `RRTC_REG3` writer"]
pub type W = crate::W<RrtcReg3Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Регистры общего назначения REG3\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_reg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_reg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RrtcReg3Spec;
impl crate::RegisterSpec for RrtcReg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rrtc_reg3::R`](R) reader structure"]
impl crate::Readable for RrtcReg3Spec {}
#[doc = "`write(|w| ..)` method takes [`rrtc_reg3::W`](W) writer structure"]
impl crate::Writable for RrtcReg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RRTC_REG3 to value 0"]
impl crate::Resettable for RrtcReg3Spec {
    const RESET_VALUE: u32 = 0;
}
