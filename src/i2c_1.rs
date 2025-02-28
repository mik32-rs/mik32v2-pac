#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: Cr1,
    cr2: Cr2,
    oar1: Oar1,
    oar2: Oar2,
    timingr: Timingr,
    _reserved5: [u8; 0x04],
    isr: Isr,
    icr: Icr,
    rxdr: Rxdr,
    txdr: Txdr,
}
impl RegisterBlock {
    #[doc = "0x00 - Регистр управления 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x04 - Регистр управления 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    #[doc = "0x08 - Регистр адреса 1"]
    #[inline(always)]
    pub const fn oar1(&self) -> &Oar1 {
        &self.oar1
    }
    #[doc = "0x0c - Регистр адреса 2"]
    #[inline(always)]
    pub const fn oar2(&self) -> &Oar2 {
        &self.oar2
    }
    #[doc = "0x10 - Регистр настройки временных ограничений. Регистр должен конфигурироваться, пока интерфейс заблокирован (PE=0)."]
    #[inline(always)]
    pub const fn timingr(&self) -> &Timingr {
        &self.timingr
    }
    #[doc = "0x18 - Регистр флагов прерываний"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x1c - Регистр сроса флагов прерываний"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x20 - Регистр принятых данных"]
    #[inline(always)]
    pub const fn rxdr(&self) -> &Rxdr {
        &self.rxdr
    }
    #[doc = "0x24 - Регистр передаваемых данных"]
    #[inline(always)]
    pub const fn txdr(&self) -> &Txdr {
        &self.txdr
    }
}
#[doc = "CR1 (rw) register accessor: Регистр управления 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "Регистр управления 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: Регистр управления 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = "Регистр управления 2"]
pub mod cr2;
#[doc = "OAR1 (rw) register accessor: Регистр адреса 1\n\nYou can [`read`](crate::Reg::read) this register and get [`oar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oar1`]
module"]
#[doc(alias = "OAR1")]
pub type Oar1 = crate::Reg<oar1::Oar1Spec>;
#[doc = "Регистр адреса 1"]
pub mod oar1;
#[doc = "OAR2 (rw) register accessor: Регистр адреса 2\n\nYou can [`read`](crate::Reg::read) this register and get [`oar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oar2`]
module"]
#[doc(alias = "OAR2")]
pub type Oar2 = crate::Reg<oar2::Oar2Spec>;
#[doc = "Регистр адреса 2"]
pub mod oar2;
#[doc = "TIMINGR (rw) register accessor: Регистр настройки временных ограничений. Регистр должен конфигурироваться, пока интерфейс заблокирован (PE=0).\n\nYou can [`read`](crate::Reg::read) this register and get [`timingr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timingr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timingr`]
module"]
#[doc(alias = "TIMINGR")]
pub type Timingr = crate::Reg<timingr::TimingrSpec>;
#[doc = "Регистр настройки временных ограничений. Регистр должен конфигурироваться, пока интерфейс заблокирован (PE=0)."]
pub mod timingr;
#[doc = "ISR (rw) register accessor: Регистр флагов прерываний\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "Регистр флагов прерываний"]
pub mod isr;
#[doc = "ICR (w) register accessor: Регистр сроса флагов прерываний\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "Регистр сроса флагов прерываний"]
pub mod icr;
#[doc = "RXDR (r) register accessor: Регистр принятых данных\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdr`]
module"]
#[doc(alias = "RXDR")]
pub type Rxdr = crate::Reg<rxdr::RxdrSpec>;
#[doc = "Регистр принятых данных"]
pub mod rxdr;
#[doc = "TXDR (rw) register accessor: Регистр передаваемых данных\n\nYou can [`read`](crate::Reg::read) this register and get [`txdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdr`]
module"]
#[doc(alias = "TXDR")]
pub type Txdr = crate::Reg<txdr::TxdrSpec>;
#[doc = "Регистр передаваемых данных"]
pub mod txdr;
