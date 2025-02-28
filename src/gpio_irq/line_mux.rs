#[doc = "Register `LINE_MUX` reader"]
pub type R = crate::R<LineMuxSpec>;
#[doc = "Register `LINE_MUX` writer"]
pub type W = crate::W<LineMuxSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Управление мультиплексорами. Каждый мультиплесор управляется своими 4 битами: Mux_0 – \\[3:0\\]; Mux_1 – \\[7:4\\]; Mux_2 – \\[11:8\\]; Mux_3 – \\[15:12\\]; Mux_4 – \\[19:16\\]; Mux_5– \\[23:20\\]; Mux_6 – \\[27:24\\]; Mux_7 – \\[31:28\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`line_mux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`line_mux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LineMuxSpec;
impl crate::RegisterSpec for LineMuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`line_mux::R`](R) reader structure"]
impl crate::Readable for LineMuxSpec {}
#[doc = "`write(|w| ..)` method takes [`line_mux::W`](W) writer structure"]
impl crate::Writable for LineMuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LINE_MUX to value 0"]
impl crate::Resettable for LineMuxSpec {
    const RESET_VALUE: u32 = 0;
}
