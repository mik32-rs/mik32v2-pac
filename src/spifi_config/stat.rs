#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Бит устанавливается в «1», если программное обеспечение успешно записало регистр команд памяти. Сброс бита осуществляется аппаратным сбросом или установкой в «1» бита RESET\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mcinit {
    #[doc = "0: Программное обеспечение не записало регистр команд памяти"]
    Idle = 0,
    #[doc = "1: Программное обеспечение успешно записало регистр команд памяти"]
    Success = 1,
}
impl From<Mcinit> for bool {
    #[inline(always)]
    fn from(variant: Mcinit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCINIT` reader - Бит устанавливается в «1», если программное обеспечение успешно записало регистр команд памяти. Сброс бита осуществляется аппаратным сбросом или установкой в «1» бита RESET"]
pub type McinitR = crate::BitReader<Mcinit>;
impl McinitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mcinit {
        match self.bits {
            false => Mcinit::Idle,
            true => Mcinit::Success,
        }
    }
    #[doc = "Программное обеспечение не записало регистр команд памяти"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Mcinit::Idle
    }
    #[doc = "Программное обеспечение успешно записало регистр команд памяти"]
    #[inline(always)]
    pub fn is_success(&self) -> bool {
        *self == Mcinit::Success
    }
}
#[doc = "Бит устанавливается в «1», если программное обеспечение успешно записало регистр команд. Сброс этого бита осуществляется теми же сигналами, что и бит MCINIT. Также сброс бита CMD происходит при завершении выполнения команды, когда деактивируется сигнал SPIFI_CS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmd {
    #[doc = "0: Программное обеспечение не записало регистр команд"]
    Idle = 0,
    #[doc = "1: Программное обеспечение успешно записало регистр команд"]
    Success = 1,
}
impl From<Cmd> for bool {
    #[inline(always)]
    fn from(variant: Cmd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD` reader - Бит устанавливается в «1», если программное обеспечение успешно записало регистр команд. Сброс этого бита осуществляется теми же сигналами, что и бит MCINIT. Также сброс бита CMD происходит при завершении выполнения команды, когда деактивируется сигнал SPIFI_CS"]
pub type CmdR = crate::BitReader<Cmd>;
impl CmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd {
        match self.bits {
            false => Cmd::Idle,
            true => Cmd::Success,
        }
    }
    #[doc = "Программное обеспечение не записало регистр команд"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Cmd::Idle
    }
    #[doc = "Программное обеспечение успешно записало регистр команд"]
    #[inline(always)]
    pub fn is_success(&self) -> bool {
        *self == Cmd::Success
    }
}
#[doc = "Field `CMD` writer - Бит устанавливается в «1», если программное обеспечение успешно записало регистр команд. Сброс этого бита осуществляется теми же сигналами, что и бит MCINIT. Также сброс бита CMD происходит при завершении выполнения команды, когда деактивируется сигнал SPIFI_CS"]
pub type CmdW<'a, REG> = crate::BitWriter<'a, REG, Cmd>;
impl<'a, REG> CmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Программное обеспечение не записало регистр команд"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::Idle)
    }
    #[doc = "Программное обеспечение успешно записало регистр команд"]
    #[inline(always)]
    pub fn success(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::Success)
    }
}
#[doc = "Бит предназначен для того, чтобы прервать текущую команду периферийного режима или режима памяти. Бит сбрасывается, когда контроллер готов к выполнению новой команды\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reset {
    #[doc = "0: Контроллер готов к выполнению новой команды"]
    Ready = 0,
    #[doc = "1: Прервать текущую команду периферийного режима или режима памяти"]
    Reset = 1,
}
impl From<Reset> for bool {
    #[inline(always)]
    fn from(variant: Reset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET` reader - Бит предназначен для того, чтобы прервать текущую команду периферийного режима или режима памяти. Бит сбрасывается, когда контроллер готов к выполнению новой команды"]
pub type ResetR = crate::BitReader<Reset>;
impl ResetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reset {
        match self.bits {
            false => Reset::Ready,
            true => Reset::Reset,
        }
    }
    #[doc = "Контроллер готов к выполнению новой команды"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Reset::Ready
    }
    #[doc = "Прервать текущую команду периферийного режима или режима памяти"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Reset::Reset
    }
}
#[doc = "Field `RESET` writer - Бит предназначен для того, чтобы прервать текущую команду периферийного режима или режима памяти. Бит сбрасывается, когда контроллер готов к выполнению новой команды"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG, Reset>;
impl<'a, REG> ResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Контроллер готов к выполнению новой команды"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::Ready)
    }
    #[doc = "Прервать текущую команду периферийного режима или режима памяти"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::Reset)
    }
}
#[doc = "Запись «1» в бит сбрасывает запрос на прерывание от контроллера SPIFI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intrq {
    #[doc = "1: Сбрость запрос на прерывание от контроллера SPIFI"]
    ClearInterrupt = 1,
}
impl From<Intrq> for bool {
    #[inline(always)]
    fn from(variant: Intrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTRQ` reader - Запись «1» в бит сбрасывает запрос на прерывание от контроллера SPIFI"]
pub type IntrqR = crate::BitReader<Intrq>;
impl IntrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Intrq> {
        match self.bits {
            true => Some(Intrq::ClearInterrupt),
            _ => None,
        }
    }
    #[doc = "Сбрость запрос на прерывание от контроллера SPIFI"]
    #[inline(always)]
    pub fn is_clear_interrupt(&self) -> bool {
        *self == Intrq::ClearInterrupt
    }
}
#[doc = "Field `INTRQ` writer - Запись «1» в бит сбрасывает запрос на прерывание от контроллера SPIFI"]
pub type IntrqW<'a, REG> = crate::BitWriter<'a, REG, Intrq>;
impl<'a, REG> IntrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Сбрость запрос на прерывание от контроллера SPIFI"]
    #[inline(always)]
    pub fn clear_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Intrq::ClearInterrupt)
    }
}
#[doc = "Field `VERSION` reader - Версия контроллера SPIFI (поле доступно только для чтения)"]
pub type VersionR = crate::FieldReader;
#[doc = "Field `VERSION` writer - Версия контроллера SPIFI (поле доступно только для чтения)"]
pub type VersionW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Бит устанавливается в «1», если программное обеспечение успешно записало регистр команд памяти. Сброс бита осуществляется аппаратным сбросом или установкой в «1» бита RESET"]
    #[inline(always)]
    pub fn mcinit(&self) -> McinitR {
        McinitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Бит устанавливается в «1», если программное обеспечение успешно записало регистр команд. Сброс этого бита осуществляется теми же сигналами, что и бит MCINIT. Также сброс бита CMD происходит при завершении выполнения команды, когда деактивируется сигнал SPIFI_CS"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Бит предназначен для того, чтобы прервать текущую команду периферийного режима или режима памяти. Бит сбрасывается, когда контроллер готов к выполнению новой команды"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Запись «1» в бит сбрасывает запрос на прерывание от контроллера SPIFI"]
    #[inline(always)]
    pub fn intrq(&self) -> IntrqR {
        IntrqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Версия контроллера SPIFI (поле доступно только для чтения)"]
    #[inline(always)]
    pub fn version(&self) -> VersionR {
        VersionR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Бит устанавливается в «1», если программное обеспечение успешно записало регистр команд. Сброс этого бита осуществляется теми же сигналами, что и бит MCINIT. Также сброс бита CMD происходит при завершении выполнения команды, когда деактивируется сигнал SPIFI_CS"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<StatSpec> {
        CmdW::new(self, 1)
    }
    #[doc = "Bit 4 - Бит предназначен для того, чтобы прервать текущую команду периферийного режима или режима памяти. Бит сбрасывается, когда контроллер готов к выполнению новой команды"]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<StatSpec> {
        ResetW::new(self, 4)
    }
    #[doc = "Bit 5 - Запись «1» в бит сбрасывает запрос на прерывание от контроллера SPIFI"]
    #[inline(always)]
    pub fn intrq(&mut self) -> IntrqW<StatSpec> {
        IntrqW::new(self, 5)
    }
    #[doc = "Bits 24:31 - Версия контроллера SPIFI (поле доступно только для чтения)"]
    #[inline(always)]
    pub fn version(&mut self) -> VersionW<StatSpec> {
        VersionW::new(self, 24)
    }
}
#[doc = "SPIFI регистр статуса\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT to value 0x0200_0001"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0x0200_0001;
}
