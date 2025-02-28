#[doc = "Register `RRTC_REG2` reader"]
pub type R = crate::R<RrtcReg2Spec>;
#[doc = "Register `RRTC_REG2` writer"]
pub type W = crate::W<RrtcReg2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Регистры общего назначения REG2\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_reg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_reg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RrtcReg2Spec;
impl crate::RegisterSpec for RrtcReg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rrtc_reg2::R`](R) reader structure"]
impl crate::Readable for RrtcReg2Spec {}
#[doc = "`write(|w| ..)` method takes [`rrtc_reg2::W`](W) writer structure"]
impl crate::Writable for RrtcReg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RRTC_REG2 to value 0"]
impl crate::Resettable for RrtcReg2Spec {
    const RESET_VALUE: u32 = 0;
}
