#[doc = "Register `DELAY` reader"]
pub type R = crate::R<DelaySpec>;
#[doc = "Register `DELAY` writer"]
pub type W = crate::W<DelaySpec>;
#[doc = "Field `D_INT` reader - (INIT) Дополнительная задержка в периодах опорного тактового сигнала или внешнего тактового сигнала ext_clk между установкой сигнала n_ss_out в «0» и передачей первого бита"]
pub type DIntR = crate::FieldReader;
#[doc = "Field `D_INT` writer - (INIT) Дополнительная задержка в периодах опорного тактового сигнала или внешнего тактового сигнала ext_clk между установкой сигнала n_ss_out в «0» и передачей первого бита"]
pub type DIntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `D_AFTER` reader - (AFTER) Задержка в периодах опорного тактового сигнала или внешнего тактового сигнала ext_clk между последним битом текущего слова и первым битом следующего слова"]
pub type DAfterR = crate::FieldReader;
#[doc = "Field `D_AFTER` writer - (AFTER) Задержка в периодах опорного тактового сигнала или внешнего тактового сигнала ext_clk между последним битом текущего слова и первым битом следующего слова"]
pub type DAfterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `D_BTWN` reader - (BTWN) Задержка в периодах опорного тактового сигнала или внешнего тактового сигнала ext_clk между снятием сигнала выбора одного ведомого устройства и установкой сигнала выбора другого ведомого устройства"]
pub type DBtwnR = crate::FieldReader;
#[doc = "Field `D_BTWN` writer - (BTWN) Задержка в периодах опорного тактового сигнала или внешнего тактового сигнала ext_clk между снятием сигнала выбора одного ведомого устройства и установкой сигнала выбора другого ведомого устройства"]
pub type DBtwnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - (INIT) Дополнительная задержка в периодах опорного тактового сигнала или внешнего тактового сигнала ext_clk между установкой сигнала n_ss_out в «0» и передачей первого бита"]
    #[inline(always)]
    pub fn d_int(&self) -> DIntR {
        DIntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - (AFTER) Задержка в периодах опорного тактового сигнала или внешнего тактового сигнала ext_clk между последним битом текущего слова и первым битом следующего слова"]
    #[inline(always)]
    pub fn d_after(&self) -> DAfterR {
        DAfterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - (BTWN) Задержка в периодах опорного тактового сигнала или внешнего тактового сигнала ext_clk между снятием сигнала выбора одного ведомого устройства и установкой сигнала выбора другого ведомого устройства"]
    #[inline(always)]
    pub fn d_btwn(&self) -> DBtwnR {
        DBtwnR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - (INIT) Дополнительная задержка в периодах опорного тактового сигнала или внешнего тактового сигнала ext_clk между установкой сигнала n_ss_out в «0» и передачей первого бита"]
    #[inline(always)]
    pub fn d_int(&mut self) -> DIntW<DelaySpec> {
        DIntW::new(self, 0)
    }
    #[doc = "Bits 8:15 - (AFTER) Задержка в периодах опорного тактового сигнала или внешнего тактового сигнала ext_clk между последним битом текущего слова и первым битом следующего слова"]
    #[inline(always)]
    pub fn d_after(&mut self) -> DAfterW<DelaySpec> {
        DAfterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - (BTWN) Задержка в периодах опорного тактового сигнала или внешнего тактового сигнала ext_clk между снятием сигнала выбора одного ведомого устройства и установкой сигнала выбора другого ведомого устройства"]
    #[inline(always)]
    pub fn d_btwn(&mut self) -> DBtwnW<DelaySpec> {
        DBtwnW::new(self, 16)
    }
}
#[doc = "Регистр задержек\n\nYou can [`read`](crate::Reg::read) this register and get [`delay::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`delay::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DelaySpec;
impl crate::RegisterSpec for DelaySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`delay::R`](R) reader structure"]
impl crate::Readable for DelaySpec {}
#[doc = "`write(|w| ..)` method takes [`delay::W`](W) writer structure"]
impl crate::Writable for DelaySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DELAY to value 0"]
impl crate::Resettable for DelaySpec {
    const RESET_VALUE: u32 = 0;
}
