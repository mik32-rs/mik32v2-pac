#[doc = "Register `CH4_OCR` reader"]
pub type R = crate::R<Ch4OcrSpec>;
#[doc = "Register `CH4_OCR` writer"]
pub type W = crate::W<Ch4OcrSpec>;
#[doc = "Field `OCR` reader - Значение таймера в режиме сравнения"]
pub type OcrR = crate::FieldReader<u32>;
#[doc = "Field `OCR` writer - Значение таймера в режиме сравнения"]
pub type OcrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Значение таймера в режиме сравнения"]
    #[inline(always)]
    pub fn ocr(&self) -> OcrR {
        OcrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Значение таймера в режиме сравнения"]
    #[inline(always)]
    pub fn ocr(&mut self) -> OcrW<Ch4OcrSpec> {
        OcrW::new(self, 0)
    }
}
#[doc = "Значение сравнения 4 канала\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_ocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_ocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch4OcrSpec;
impl crate::RegisterSpec for Ch4OcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4_ocr::R`](R) reader structure"]
impl crate::Readable for Ch4OcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch4_ocr::W`](W) writer structure"]
impl crate::Writable for Ch4OcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH4_OCR to value 0"]
impl crate::Resettable for Ch4OcrSpec {
    const RESET_VALUE: u32 = 0;
}
