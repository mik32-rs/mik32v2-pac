#[doc = "Register `DATA` reader"]
pub type R = crate::R<DataSpec>;
#[doc = "Register `DATA` writer"]
pub type W = crate::W<DataSpec>;
#[doc = "Field `DATA8` reader - Входные или выходные данные"]
pub type Data8R = crate::FieldReader;
#[doc = "Field `DATA8` writer - Входные или выходные данные"]
pub type Data8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA16` reader - Входные или выходные данные"]
pub type Data16R = crate::FieldReader<u16>;
#[doc = "Field `DATA16` writer - Входные или выходные данные"]
pub type Data16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DATA32` reader - Входные или выходные данные"]
pub type Data32R = crate::FieldReader<u32>;
#[doc = "Field `DATA32` writer - Входные или выходные данные"]
pub type Data32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:7 - Входные или выходные данные"]
    #[inline(always)]
    pub fn data8(&self) -> Data8R {
        Data8R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 0:15 - Входные или выходные данные"]
    #[inline(always)]
    pub fn data16(&self) -> Data16R {
        Data16R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 0:31 - Входные или выходные данные"]
    #[inline(always)]
    pub fn data32(&self) -> Data32R {
        Data32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Входные или выходные данные"]
    #[inline(always)]
    pub fn data8(&mut self) -> Data8W<DataSpec> {
        Data8W::new(self, 0)
    }
    #[doc = "Bits 0:15 - Входные или выходные данные"]
    #[inline(always)]
    pub fn data16(&mut self) -> Data16W<DataSpec> {
        Data16W::new(self, 0)
    }
    #[doc = "Bits 0:31 - Входные или выходные данные"]
    #[inline(always)]
    pub fn data32(&mut self) -> Data32W<DataSpec> {
        Data32W::new(self, 0)
    }
}
#[doc = "SPIFI регистр данных. Если выходной буфер чтения пуст или входной буфер записи полон, то при отправке запроса по шине AHB будет вызвано исключение (код 5 “Load access fault”).\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataSpec;
impl crate::RegisterSpec for DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data::R`](R) reader structure"]
impl crate::Readable for DataSpec {}
#[doc = "`write(|w| ..)` method takes [`data::W`](W) writer structure"]
impl crate::Writable for DataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA to value 0"]
impl crate::Resettable for DataSpec {
    const RESET_VALUE: u32 = 0;
}
