#[doc = "Register `NCYCRL` reader"]
pub type R = crate::R<NcycrlSpec>;
#[doc = "Register `NCYCRL` writer"]
pub type W = crate::W<NcycrlSpec>;
#[doc = "Field `N_LD` reader - Количество тактов между передними и задними фронтами сигналов Hard IP Phi1Ee и Phi1ShiftEe и задержка от нача¬ла обращения по шине до первого фронта Phi1Ee. Рекомен¬дуемое значение при любых частотах clk 1 такт. Запрещено устанавливать равным 0"]
pub type NLdR = crate::FieldReader;
#[doc = "Field `N_LD` writer - Количество тактов между передними и задними фронтами сигналов Hard IP Phi1Ee и Phi1ShiftEe и задержка от нача¬ла обращения по шине до первого фронта Phi1Ee. Рекомен¬дуемое значение при любых частотах clk 1 такт. Запрещено устанавливать равным 0"]
pub type NLdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `N_R_1` reader - Количество тактов, в течение которого Phi1ShiftEe имеет вы¬сокий уровень в процедуре чтения. Рекомендуемое значение N RA = ceil(51/Pclk), где Pclk – период тактового сигнала в нс, ceil – функция округления до ближайшего большего целого числа. Запрещено устанавливать равным 0"]
pub type NR1R = crate::FieldReader;
#[doc = "Field `N_R_1` writer - Количество тактов, в течение которого Phi1ShiftEe имеет вы¬сокий уровень в процедуре чтения. Рекомендуемое значение N RA = ceil(51/Pclk), где Pclk – период тактового сигнала в нс, ceil – функция округления до ближайшего большего целого числа. Запрещено устанавливать равным 0"]
pub type NR1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `N_R_2` reader - Количество тактов, требуемое для завершения процедуры чте¬ния (перевода OkSelEe на высокий уровень после процедуры чтения). Рекомендуемое значение при любых частотах clk 1 такт. Запрещено устанавливать равным 0"]
pub type NR2R = crate::FieldReader;
#[doc = "Field `N_R_2` writer - Количество тактов, требуемое для завершения процедуры чте¬ния (перевода OkSelEe на высокий уровень после процедуры чтения). Рекомендуемое значение при любых частотах clk 1 такт. Запрещено устанавливать равным 0"]
pub type NR2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Количество тактов между передними и задними фронтами сигналов Hard IP Phi1Ee и Phi1ShiftEe и задержка от нача¬ла обращения по шине до первого фронта Phi1Ee. Рекомен¬дуемое значение при любых частотах clk 1 такт. Запрещено устанавливать равным 0"]
    #[inline(always)]
    pub fn n_ld(&self) -> NLdR {
        NLdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Количество тактов, в течение которого Phi1ShiftEe имеет вы¬сокий уровень в процедуре чтения. Рекомендуемое значение N RA = ceil(51/Pclk), где Pclk – период тактового сигнала в нс, ceil – функция округления до ближайшего большего целого числа. Запрещено устанавливать равным 0"]
    #[inline(always)]
    pub fn n_r_1(&self) -> NR1R {
        NR1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Количество тактов, требуемое для завершения процедуры чте¬ния (перевода OkSelEe на высокий уровень после процедуры чтения). Рекомендуемое значение при любых частотах clk 1 такт. Запрещено устанавливать равным 0"]
    #[inline(always)]
    pub fn n_r_2(&self) -> NR2R {
        NR2R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Количество тактов между передними и задними фронтами сигналов Hard IP Phi1Ee и Phi1ShiftEe и задержка от нача¬ла обращения по шине до первого фронта Phi1Ee. Рекомен¬дуемое значение при любых частотах clk 1 такт. Запрещено устанавливать равным 0"]
    #[inline(always)]
    pub fn n_ld(&mut self) -> NLdW<NcycrlSpec> {
        NLdW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Количество тактов, в течение которого Phi1ShiftEe имеет вы¬сокий уровень в процедуре чтения. Рекомендуемое значение N RA = ceil(51/Pclk), где Pclk – период тактового сигнала в нс, ceil – функция округления до ближайшего большего целого числа. Запрещено устанавливать равным 0"]
    #[inline(always)]
    pub fn n_r_1(&mut self) -> NR1W<NcycrlSpec> {
        NR1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Количество тактов, требуемое для завершения процедуры чте¬ния (перевода OkSelEe на высокий уровень после процедуры чтения). Рекомендуемое значение при любых частотах clk 1 такт. Запрещено устанавливать равным 0"]
    #[inline(always)]
    pub fn n_r_2(&mut self) -> NR2W<NcycrlSpec> {
        NR2W::new(self, 16)
    }
}
#[doc = "Регистр подстройки длительности процедур чтения и заполнения буфера записи\n\nYou can [`read`](crate::Reg::read) this register and get [`ncycrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ncycrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NcycrlSpec;
impl crate::RegisterSpec for NcycrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ncycrl::R`](R) reader structure"]
impl crate::Readable for NcycrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ncycrl::W`](W) writer structure"]
impl crate::Writable for NcycrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NCYCRL to value 0x0001_0201"]
impl crate::Resettable for NcycrlSpec {
    const RESET_VALUE: u32 = 0x0001_0201;
}
