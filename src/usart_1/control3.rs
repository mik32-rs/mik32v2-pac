#[doc = "Register `CONTROL3` reader"]
pub type R = crate::R<Control3Spec>;
#[doc = "Register `CONTROL3` writer"]
pub type W = crate::W<Control3Spec>;
#[doc = "Управление прерыванием при обнаружении ошибок приема данных (FE, ORE, NF)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eie {
    #[doc = "0: Прерывание выключено"]
    Disable = 0,
    #[doc = "1: Прерывание включено"]
    Enable = 1,
}
impl From<Eie> for bool {
    #[inline(always)]
    fn from(variant: Eie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EIE` reader - Управление прерыванием при обнаружении ошибок приема данных (FE, ORE, NF)"]
pub type EieR = crate::BitReader<Eie>;
impl EieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eie {
        match self.bits {
            false => Eie::Disable,
            true => Eie::Enable,
        }
    }
    #[doc = "Прерывание выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Eie::Disable
    }
    #[doc = "Прерывание включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Eie::Enable
    }
}
#[doc = "Field `EIE` writer - Управление прерыванием при обнаружении ошибок приема данных (FE, ORE, NF)"]
pub type EieW<'a, REG> = crate::BitWriter<'a, REG, Eie>;
impl<'a, REG> EieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Eie::Disable)
    }
    #[doc = "Прерывание включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Eie::Enable)
    }
}
#[doc = "Переход передатчика в break состояние\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbkrq {
    #[doc = "0: Обычный режим работы"]
    Normal = 0,
    #[doc = "1: Состояние break на линии TX"]
    BreakTx = 1,
}
impl From<Sbkrq> for bool {
    #[inline(always)]
    fn from(variant: Sbkrq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBKRQ` reader - Переход передатчика в break состояние"]
pub type SbkrqR = crate::BitReader<Sbkrq>;
impl SbkrqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sbkrq {
        match self.bits {
            false => Sbkrq::Normal,
            true => Sbkrq::BreakTx,
        }
    }
    #[doc = "Обычный режим работы"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Sbkrq::Normal
    }
    #[doc = "Состояние break на линии TX"]
    #[inline(always)]
    pub fn is_break_tx(&self) -> bool {
        *self == Sbkrq::BreakTx
    }
}
#[doc = "Field `SBKRQ` writer - Переход передатчика в break состояние"]
pub type SbkrqW<'a, REG> = crate::BitWriter<'a, REG, Sbkrq>;
impl<'a, REG> SbkrqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Обычный режим работы"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Sbkrq::Normal)
    }
    #[doc = "Состояние break на линии TX"]
    #[inline(always)]
    pub fn break_tx(self) -> &'a mut crate::W<REG> {
        self.variant(Sbkrq::BreakTx)
    }
}
#[doc = "Выбор между полудуплексным и дуплексным режимами работы. В полудуплексном режиме RX не используется, все данные передаются и принимаются через TX. При наличии данных на передачу происходит отправка данных в остальное время прием. Этот бит может быть изменен только при остановке работы (UE=0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdsel {
    #[doc = "0: Дуплексный режим работы"]
    Duplex = 0,
    #[doc = "1: Полудуплексный режим работы"]
    HalfDuplex = 1,
}
impl From<Hdsel> for bool {
    #[inline(always)]
    fn from(variant: Hdsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDSEL` reader - Выбор между полудуплексным и дуплексным режимами работы. В полудуплексном режиме RX не используется, все данные передаются и принимаются через TX. При наличии данных на передачу происходит отправка данных в остальное время прием. Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type HdselR = crate::BitReader<Hdsel>;
impl HdselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdsel {
        match self.bits {
            false => Hdsel::Duplex,
            true => Hdsel::HalfDuplex,
        }
    }
    #[doc = "Дуплексный режим работы"]
    #[inline(always)]
    pub fn is_duplex(&self) -> bool {
        *self == Hdsel::Duplex
    }
    #[doc = "Полудуплексный режим работы"]
    #[inline(always)]
    pub fn is_half_duplex(&self) -> bool {
        *self == Hdsel::HalfDuplex
    }
}
#[doc = "Field `HDSEL` writer - Выбор между полудуплексным и дуплексным режимами работы. В полудуплексном режиме RX не используется, все данные передаются и принимаются через TX. При наличии данных на передачу происходит отправка данных в остальное время прием. Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type HdselW<'a, REG> = crate::BitWriter<'a, REG, Hdsel>;
impl<'a, REG> HdselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Дуплексный режим работы"]
    #[inline(always)]
    pub fn duplex(self) -> &'a mut crate::W<REG> {
        self.variant(Hdsel::Duplex)
    }
    #[doc = "Полудуплексный режим работы"]
    #[inline(always)]
    pub fn half_duplex(self) -> &'a mut crate::W<REG> {
        self.variant(Hdsel::HalfDuplex)
    }
}
#[doc = "Управление работой сигна-ла dma_rrq\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmar {
    #[doc = "0: Сигнал выключен"]
    Disable = 0,
    #[doc = "1: Сигнал включен"]
    Enable = 1,
}
impl From<Dmar> for bool {
    #[inline(always)]
    fn from(variant: Dmar) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAR` reader - Управление работой сигна-ла dma_rrq"]
pub type DmarR = crate::BitReader<Dmar>;
impl DmarR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmar {
        match self.bits {
            false => Dmar::Disable,
            true => Dmar::Enable,
        }
    }
    #[doc = "Сигнал выключен"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dmar::Disable
    }
    #[doc = "Сигнал включен"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dmar::Enable
    }
}
#[doc = "Field `DMAR` writer - Управление работой сигна-ла dma_rrq"]
pub type DmarW<'a, REG> = crate::BitWriter<'a, REG, Dmar>;
impl<'a, REG> DmarW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Сигнал выключен"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmar::Disable)
    }
    #[doc = "Сигнал включен"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmar::Enable)
    }
}
#[doc = "Управление работой сигнала dma_trq\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmat {
    #[doc = "0: Сигнал выключен"]
    Disable = 0,
    #[doc = "1: Сигнал включен"]
    Enable = 1,
}
impl From<Dmat> for bool {
    #[inline(always)]
    fn from(variant: Dmat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAT` reader - Управление работой сигнала dma_trq"]
pub type DmatR = crate::BitReader<Dmat>;
impl DmatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmat {
        match self.bits {
            false => Dmat::Disable,
            true => Dmat::Enable,
        }
    }
    #[doc = "Сигнал выключен"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dmat::Disable
    }
    #[doc = "Сигнал включен"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dmat::Enable
    }
}
#[doc = "Field `DMAT` writer - Управление работой сигнала dma_trq"]
pub type DmatW<'a, REG> = crate::BitWriter<'a, REG, Dmat>;
impl<'a, REG> DmatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Сигнал выключен"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmat::Disable)
    }
    #[doc = "Сигнал включен"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmat::Enable)
    }
}
#[doc = "Управление выходным сигналом RTS. Этот бит может быть изменен только при остановке работы (UE=0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtse {
    #[doc = "0: Сигнал всегда в разрешающем состоянии (RTS = 0)"]
    _0 = 0,
    #[doc = "1: Сигнал находится в разрешающем состоянии (RTS = 0), только когда приемник готов принять данные"]
    _1 = 1,
}
impl From<Rtse> for bool {
    #[inline(always)]
    fn from(variant: Rtse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSE` reader - Управление выходным сигналом RTS. Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type RtseR = crate::BitReader<Rtse>;
impl RtseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtse {
        match self.bits {
            false => Rtse::_0,
            true => Rtse::_1,
        }
    }
    #[doc = "Сигнал всегда в разрешающем состоянии (RTS = 0)"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Rtse::_0
    }
    #[doc = "Сигнал находится в разрешающем состоянии (RTS = 0), только когда приемник готов принять данные"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Rtse::_1
    }
}
#[doc = "Field `RTSE` writer - Управление выходным сигналом RTS. Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type RtseW<'a, REG> = crate::BitWriter<'a, REG, Rtse>;
impl<'a, REG> RtseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Сигнал всегда в разрешающем состоянии (RTS = 0)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtse::_0)
    }
    #[doc = "Сигнал находится в разрешающем состоянии (RTS = 0), только когда приемник готов принять данные"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtse::_1)
    }
}
#[doc = "Выбор реакции на входной сигнал CTS. Этот бит может быть изме-нен только при остановке работы (UE=0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctse {
    #[doc = "0: Сигнал игнорируется"]
    Ignored = 0,
    #[doc = "1: сигнал управляет передачей данных трансмиттером. Передача разрешена при (CTS = 0)"]
    Control = 1,
}
impl From<Ctse> for bool {
    #[inline(always)]
    fn from(variant: Ctse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSE` reader - Выбор реакции на входной сигнал CTS. Этот бит может быть изме-нен только при остановке работы (UE=0)"]
pub type CtseR = crate::BitReader<Ctse>;
impl CtseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctse {
        match self.bits {
            false => Ctse::Ignored,
            true => Ctse::Control,
        }
    }
    #[doc = "Сигнал игнорируется"]
    #[inline(always)]
    pub fn is_ignored(&self) -> bool {
        *self == Ctse::Ignored
    }
    #[doc = "сигнал управляет передачей данных трансмиттером. Передача разрешена при (CTS = 0)"]
    #[inline(always)]
    pub fn is_control(&self) -> bool {
        *self == Ctse::Control
    }
}
#[doc = "Field `CTSE` writer - Выбор реакции на входной сигнал CTS. Этот бит может быть изме-нен только при остановке работы (UE=0)"]
pub type CtseW<'a, REG> = crate::BitWriter<'a, REG, Ctse>;
impl<'a, REG> CtseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Сигнал игнорируется"]
    #[inline(always)]
    pub fn ignored(self) -> &'a mut crate::W<REG> {
        self.variant(Ctse::Ignored)
    }
    #[doc = "сигнал управляет передачей данных трансмиттером. Передача разрешена при (CTS = 0)"]
    #[inline(always)]
    pub fn control(self) -> &'a mut crate::W<REG> {
        self.variant(Ctse::Control)
    }
}
#[doc = "Управление прерыванием при обнаружении измене-ния CTS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsie {
    #[doc = "0: Прерывание выключено"]
    Disable = 0,
    #[doc = "1: Прерывание включено"]
    Enable = 1,
}
impl From<Ctsie> for bool {
    #[inline(always)]
    fn from(variant: Ctsie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSIE` reader - Управление прерыванием при обнаружении измене-ния CTS"]
pub type CtsieR = crate::BitReader<Ctsie>;
impl CtsieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsie {
        match self.bits {
            false => Ctsie::Disable,
            true => Ctsie::Enable,
        }
    }
    #[doc = "Прерывание выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ctsie::Disable
    }
    #[doc = "Прерывание включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ctsie::Enable
    }
}
#[doc = "Field `CTSIE` writer - Управление прерыванием при обнаружении измене-ния CTS"]
pub type CtsieW<'a, REG> = crate::BitWriter<'a, REG, Ctsie>;
impl<'a, REG> CtsieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsie::Disable)
    }
    #[doc = "Прерывание включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsie::Enable)
    }
}
#[doc = "Выбор реакции на переполнение. Этот бит может быть изменен только при остановке работы (UE=0)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ovrdis {
    #[doc = "0: При переполнении взводится флаг ошибки (ORE), новые данные не записываются"]
    _0 = 0,
    #[doc = "1: флаг переполнения не взводится, данные перезаписываются"]
    _1 = 1,
}
impl From<Ovrdis> for bool {
    #[inline(always)]
    fn from(variant: Ovrdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRDIS` reader - Выбор реакции на переполнение. Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type OvrdisR = crate::BitReader<Ovrdis>;
impl OvrdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ovrdis {
        match self.bits {
            false => Ovrdis::_0,
            true => Ovrdis::_1,
        }
    }
    #[doc = "При переполнении взводится флаг ошибки (ORE), новые данные не записываются"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Ovrdis::_0
    }
    #[doc = "флаг переполнения не взводится, данные перезаписываются"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == Ovrdis::_1
    }
}
#[doc = "Field `OVRDIS` writer - Выбор реакции на переполнение. Этот бит может быть изменен только при остановке работы (UE=0)"]
pub type OvrdisW<'a, REG> = crate::BitWriter<'a, REG, Ovrdis>;
impl<'a, REG> OvrdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "При переполнении взводится флаг ошибки (ORE), новые данные не записываются"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrdis::_0)
    }
    #[doc = "флаг переполнения не взводится, данные перезаписываются"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(Ovrdis::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Управление прерыванием при обнаружении ошибок приема данных (FE, ORE, NF)"]
    #[inline(always)]
    pub fn eie(&self) -> EieR {
        EieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Переход передатчика в break состояние"]
    #[inline(always)]
    pub fn sbkrq(&self) -> SbkrqR {
        SbkrqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Выбор между полудуплексным и дуплексным режимами работы. В полудуплексном режиме RX не используется, все данные передаются и принимаются через TX. При наличии данных на передачу происходит отправка данных в остальное время прием. Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn hdsel(&self) -> HdselR {
        HdselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Управление работой сигна-ла dma_rrq"]
    #[inline(always)]
    pub fn dmar(&self) -> DmarR {
        DmarR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Управление работой сигнала dma_trq"]
    #[inline(always)]
    pub fn dmat(&self) -> DmatR {
        DmatR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Управление выходным сигналом RTS. Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn rtse(&self) -> RtseR {
        RtseR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Выбор реакции на входной сигнал CTS. Этот бит может быть изме-нен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn ctse(&self) -> CtseR {
        CtseR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Управление прерыванием при обнаружении измене-ния CTS"]
    #[inline(always)]
    pub fn ctsie(&self) -> CtsieR {
        CtsieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Выбор реакции на переполнение. Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn ovrdis(&self) -> OvrdisR {
        OvrdisR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Управление прерыванием при обнаружении ошибок приема данных (FE, ORE, NF)"]
    #[inline(always)]
    pub fn eie(&mut self) -> EieW<Control3Spec> {
        EieW::new(self, 0)
    }
    #[doc = "Bit 2 - Переход передатчика в break состояние"]
    #[inline(always)]
    pub fn sbkrq(&mut self) -> SbkrqW<Control3Spec> {
        SbkrqW::new(self, 2)
    }
    #[doc = "Bit 3 - Выбор между полудуплексным и дуплексным режимами работы. В полудуплексном режиме RX не используется, все данные передаются и принимаются через TX. При наличии данных на передачу происходит отправка данных в остальное время прием. Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn hdsel(&mut self) -> HdselW<Control3Spec> {
        HdselW::new(self, 3)
    }
    #[doc = "Bit 6 - Управление работой сигна-ла dma_rrq"]
    #[inline(always)]
    pub fn dmar(&mut self) -> DmarW<Control3Spec> {
        DmarW::new(self, 6)
    }
    #[doc = "Bit 7 - Управление работой сигнала dma_trq"]
    #[inline(always)]
    pub fn dmat(&mut self) -> DmatW<Control3Spec> {
        DmatW::new(self, 7)
    }
    #[doc = "Bit 8 - Управление выходным сигналом RTS. Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn rtse(&mut self) -> RtseW<Control3Spec> {
        RtseW::new(self, 8)
    }
    #[doc = "Bit 9 - Выбор реакции на входной сигнал CTS. Этот бит может быть изме-нен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn ctse(&mut self) -> CtseW<Control3Spec> {
        CtseW::new(self, 9)
    }
    #[doc = "Bit 10 - Управление прерыванием при обнаружении измене-ния CTS"]
    #[inline(always)]
    pub fn ctsie(&mut self) -> CtsieW<Control3Spec> {
        CtsieW::new(self, 10)
    }
    #[doc = "Bit 12 - Выбор реакции на переполнение. Этот бит может быть изменен только при остановке работы (UE=0)"]
    #[inline(always)]
    pub fn ovrdis(&mut self) -> OvrdisW<Control3Spec> {
        OvrdisW::new(self, 12)
    }
}
#[doc = "Регистр управления 3\n\nYou can [`read`](crate::Reg::read) this register and get [`control3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Control3Spec;
impl crate::RegisterSpec for Control3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control3::R`](R) reader structure"]
impl crate::Readable for Control3Spec {}
#[doc = "`write(|w| ..)` method takes [`control3::W`](W) writer structure"]
impl crate::Writable for Control3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONTROL3 to value 0"]
impl crate::Resettable for Control3Spec {
    const RESET_VALUE: u32 = 0;
}
