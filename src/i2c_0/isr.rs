#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<IsrSpec>;
#[doc = "Field `TXE` reader - Флаг «буфер передаваемых данных» TXDR пуст (режимы отправки). Устанавливается аппаратно, если буфер пуст; при PE=0 или программно, чтобы сбросить содержимое регистра TXDR. Сбрасывается записью следующего байта данных в регистр TXDR."]
pub type TxeR = crate::BitReader;
#[doc = "Field `TXIS` reader - Состояние прерывания передатчика. Устанавливается аппаратно, когда регистр TXDR пуст и следующий байт данных должен быть в него записан. Сбрасывается записью следующего байта данных в регистр TXDR или аппаратно при PE=0. Этот бит может быть установлен программой только при NOSTRETCH=1 для выработки события TXIS (в результате: прерывание, при TXIE=1 или DMA запрос, при TXDMAEN=1)"]
pub type TxisR = crate::BitReader;
#[doc = "Field `RXNE` reader - Флаг «буфер принятых данных заполнен» (режимы приёма). Устанавливается аппарат-но, после записи принятых данных в регистр RXDR. Сбрасывается при чтении RXDR или аппаратно при PE=0."]
pub type RxneR = crate::BitReader;
#[doc = "Field `RXNE` writer - Флаг «буфер принятых данных заполнен» (режимы приёма). Устанавливается аппарат-но, после записи принятых данных в регистр RXDR. Сбрасывается при чтении RXDR или аппаратно при PE=0."]
pub type RxneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR` reader - Флаг соответствия адреса (режим «ведомый»). Устанавливается аппаратно, если полученный адрес совпадает с одним из разрешенных в OAR1, OAR2. Сбрасывается программной установкой бита ADDRCF или аппаратно при PE=0."]
pub type AddrR = crate::BitReader;
#[doc = "Field `NACKF` reader - Флаг «не получено под-тверждение» (NACK). Устанавливается аппарат-но, после передачи байта. Сбрасывается программной установкой бита NACKCF или аппаратно при PE=0."]
pub type NackfR = crate::BitReader;
#[doc = "Field `STOPF` reader - Флаг детектирования STOP на шине. Устанавливается аппарат-но, если интерфейс участ-вует в передаче. Сбрасывается программной установкой бита STOPCF или аппаратно при PE=0."]
pub type StopfR = crate::BitReader;
#[doc = "Field `TC` reader - Флаг окончания передачи (режим «ведущий»). Уста-навливается аппаратно при RELOAD=0, AUTOEND=0, после передачи NBYTES байт. Сбрасывается программной установкой бита START или STOP или аппаратно при PE=0."]
pub type TcR = crate::BitReader;
#[doc = "Field `TCR` reader - Флаг окончания передачи (режим «ведущий» или «ведомый» с установлен-ным битом SBC). Устанав-ливается аппаратно при RELOAD=1, после переда-чи NBYTES байт. Сбрасывается записью в NBYTES ненулевого значе-ния или аппаратно при PE=0."]
pub type TcrR = crate::BitReader;
#[doc = "Field `BERR` reader - Флаг ошибки шины Устанавливается аппаратно при детектировании не-уместного события START или STOP на шине, если интерфейс участвует в пе-редаче. Не устанавливается в фазе адреса в режиме «ведомый». Сбрасывается программной установкой бита BERRCF или аппарат-но при PE=0."]
pub type BerrR = crate::BitReader;
#[doc = "Field `ARLO` reader - Флаг проигрыша арбитража. Устанавливается аппаратно, сбрасывается программной установкой бита ARLOCF или аппаратно при PE=0."]
pub type ArloR = crate::BitReader;
#[doc = "Field `OVR` reader - Флаг переполнения/недозагрузки (режим «ведомый» при NOSTRETCH=1) Устанавливается аппаратно, сбрасывается программной установкой бита OVRCF или аппаратно при PE=0."]
pub type OvrR = crate::BitReader;
#[doc = "Field `BUSY` reader - Флаг индикации занятой шины. Устанавливается по-сле события START на шине и сбрасывается после события STOP на шине"]
pub type BusyR = crate::BitReader;
#[doc = "Направление передачи (режим «ведомый»). Обновляется при совпадении адреса (ADDR=1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    #[doc = "0: Тип передачи «запись», ведомый переходит в режим приемника"]
    Write = 0,
    #[doc = "1: Тип передачи «чтение», ведомый переходит в режим передатчика"]
    Read = 1,
}
impl From<Dir> for bool {
    #[inline(always)]
    fn from(variant: Dir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - Направление передачи (режим «ведомый»). Обновляется при совпадении адреса (ADDR=1)"]
pub type DirR = crate::BitReader<Dir>;
impl DirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dir {
        match self.bits {
            false => Dir::Write,
            true => Dir::Read,
        }
    }
    #[doc = "Тип передачи «запись», ведомый переходит в режим приемника"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == Dir::Write
    }
    #[doc = "Тип передачи «чтение», ведомый переходит в режим передатчика"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == Dir::Read
    }
}
#[doc = "Field `ADDCODE` reader - Код совпавшего адреса. Обновляется в режиме «ве-домый» при совпадении адреса (ADDR=1). В режиме 10-битного адре-са содержит заголовок (5b11110) и два старших бита адреса."]
pub type AddcodeR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Флаг «буфер передаваемых данных» TXDR пуст (режимы отправки). Устанавливается аппаратно, если буфер пуст; при PE=0 или программно, чтобы сбросить содержимое регистра TXDR. Сбрасывается записью следующего байта данных в регистр TXDR."]
    #[inline(always)]
    pub fn txe(&self) -> TxeR {
        TxeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Состояние прерывания передатчика. Устанавливается аппаратно, когда регистр TXDR пуст и следующий байт данных должен быть в него записан. Сбрасывается записью следующего байта данных в регистр TXDR или аппаратно при PE=0. Этот бит может быть установлен программой только при NOSTRETCH=1 для выработки события TXIS (в результате: прерывание, при TXIE=1 или DMA запрос, при TXDMAEN=1)"]
    #[inline(always)]
    pub fn txis(&self) -> TxisR {
        TxisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Флаг «буфер принятых данных заполнен» (режимы приёма). Устанавливается аппарат-но, после записи принятых данных в регистр RXDR. Сбрасывается при чтении RXDR или аппаратно при PE=0."]
    #[inline(always)]
    pub fn rxne(&self) -> RxneR {
        RxneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Флаг соответствия адреса (режим «ведомый»). Устанавливается аппаратно, если полученный адрес совпадает с одним из разрешенных в OAR1, OAR2. Сбрасывается программной установкой бита ADDRCF или аппаратно при PE=0."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Флаг «не получено под-тверждение» (NACK). Устанавливается аппарат-но, после передачи байта. Сбрасывается программной установкой бита NACKCF или аппаратно при PE=0."]
    #[inline(always)]
    pub fn nackf(&self) -> NackfR {
        NackfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Флаг детектирования STOP на шине. Устанавливается аппарат-но, если интерфейс участ-вует в передаче. Сбрасывается программной установкой бита STOPCF или аппаратно при PE=0."]
    #[inline(always)]
    pub fn stopf(&self) -> StopfR {
        StopfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Флаг окончания передачи (режим «ведущий»). Уста-навливается аппаратно при RELOAD=0, AUTOEND=0, после передачи NBYTES байт. Сбрасывается программной установкой бита START или STOP или аппаратно при PE=0."]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Флаг окончания передачи (режим «ведущий» или «ведомый» с установлен-ным битом SBC). Устанав-ливается аппаратно при RELOAD=1, после переда-чи NBYTES байт. Сбрасывается записью в NBYTES ненулевого значе-ния или аппаратно при PE=0."]
    #[inline(always)]
    pub fn tcr(&self) -> TcrR {
        TcrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Флаг ошибки шины Устанавливается аппаратно при детектировании не-уместного события START или STOP на шине, если интерфейс участвует в пе-редаче. Не устанавливается в фазе адреса в режиме «ведомый». Сбрасывается программной установкой бита BERRCF или аппарат-но при PE=0."]
    #[inline(always)]
    pub fn berr(&self) -> BerrR {
        BerrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Флаг проигрыша арбитража. Устанавливается аппаратно, сбрасывается программной установкой бита ARLOCF или аппаратно при PE=0."]
    #[inline(always)]
    pub fn arlo(&self) -> ArloR {
        ArloR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Флаг переполнения/недозагрузки (режим «ведомый» при NOSTRETCH=1) Устанавливается аппаратно, сбрасывается программной установкой бита OVRCF или аппаратно при PE=0."]
    #[inline(always)]
    pub fn ovr(&self) -> OvrR {
        OvrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Флаг индикации занятой шины. Устанавливается по-сле события START на шине и сбрасывается после события STOP на шине"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Направление передачи (режим «ведомый»). Обновляется при совпадении адреса (ADDR=1)"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - Код совпавшего адреса. Обновляется в режиме «ве-домый» при совпадении адреса (ADDR=1). В режиме 10-битного адре-са содержит заголовок (5b11110) и два старших бита адреса."]
    #[inline(always)]
    pub fn addcode(&self) -> AddcodeR {
        AddcodeR::new(((self.bits >> 17) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Флаг «буфер принятых данных заполнен» (режимы приёма). Устанавливается аппарат-но, после записи принятых данных в регистр RXDR. Сбрасывается при чтении RXDR или аппаратно при PE=0."]
    #[inline(always)]
    pub fn rxne(&mut self) -> RxneW<IsrSpec> {
        RxneW::new(self, 2)
    }
}
#[doc = "Регистр флагов прерываний\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for IsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {
    const RESET_VALUE: u32 = 0;
}
