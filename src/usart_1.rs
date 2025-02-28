#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    control1: Control1,
    control2: Control2,
    control3: Control3,
    divider: Divider,
    _reserved4: [u8; 0x0c],
    flags: Flags,
    _reserved5: [u8; 0x04],
    rxdata: Rxdata,
    txdata: Txdata,
    modem: Modem,
}
impl RegisterBlock {
    #[doc = "0x00 - Регистр управления 1"]
    #[inline(always)]
    pub const fn control1(&self) -> &Control1 {
        &self.control1
    }
    #[doc = "0x04 - Регистр управления 2"]
    #[inline(always)]
    pub const fn control2(&self) -> &Control2 {
        &self.control2
    }
    #[doc = "0x08 - Регистр управления 3"]
    #[inline(always)]
    pub const fn control3(&self) -> &Control3 {
        &self.control3
    }
    #[doc = "0x0c - Регистр настройки делителя"]
    #[inline(always)]
    pub const fn divider(&self) -> &Divider {
        &self.divider
    }
    #[doc = "0x1c - Регистр прерываний"]
    #[inline(always)]
    pub const fn flags(&self) -> &Flags {
        &self.flags
    }
    #[doc = "0x24 - Регистр принятых данных"]
    #[inline(always)]
    pub const fn rxdata(&self) -> &Rxdata {
        &self.rxdata
    }
    #[doc = "0x28 - Регистр передаваемых данных"]
    #[inline(always)]
    pub const fn txdata(&self) -> &Txdata {
        &self.txdata
    }
    #[doc = "0x2c - Регистр управления модемом"]
    #[inline(always)]
    pub const fn modem(&self) -> &Modem {
        &self.modem
    }
}
#[doc = "CONTROL1 (rw) register accessor: Регистр управления 1\n\nYou can [`read`](crate::Reg::read) this register and get [`control1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control1`]
module"]
#[doc(alias = "CONTROL1")]
pub type Control1 = crate::Reg<control1::Control1Spec>;
#[doc = "Регистр управления 1"]
pub mod control1;
#[doc = "CONTROL2 (rw) register accessor: Регистр управления 2\n\nYou can [`read`](crate::Reg::read) this register and get [`control2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control2`]
module"]
#[doc(alias = "CONTROL2")]
pub type Control2 = crate::Reg<control2::Control2Spec>;
#[doc = "Регистр управления 2"]
pub mod control2;
#[doc = "CONTROL3 (rw) register accessor: Регистр управления 3\n\nYou can [`read`](crate::Reg::read) this register and get [`control3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control3`]
module"]
#[doc(alias = "CONTROL3")]
pub type Control3 = crate::Reg<control3::Control3Spec>;
#[doc = "Регистр управления 3"]
pub mod control3;
#[doc = "DIVIDER (rw) register accessor: Регистр настройки делителя\n\nYou can [`read`](crate::Reg::read) this register and get [`divider::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divider::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divider`]
module"]
#[doc(alias = "DIVIDER")]
pub type Divider = crate::Reg<divider::DividerSpec>;
#[doc = "Регистр настройки делителя"]
pub mod divider;
#[doc = "FLAGS (rw) register accessor: Регистр прерываний\n\nYou can [`read`](crate::Reg::read) this register and get [`flags::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flags::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flags`]
module"]
#[doc(alias = "FLAGS")]
pub type Flags = crate::Reg<flags::FlagsSpec>;
#[doc = "Регистр прерываний"]
pub mod flags;
#[doc = "RXDATA (r) register accessor: Регистр принятых данных\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata`]
module"]
#[doc(alias = "RXDATA")]
pub type Rxdata = crate::Reg<rxdata::RxdataSpec>;
#[doc = "Регистр принятых данных"]
pub mod rxdata;
#[doc = "TXDATA (rw) register accessor: Регистр передаваемых данных\n\nYou can [`read`](crate::Reg::read) this register and get [`txdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata`]
module"]
#[doc(alias = "TXDATA")]
pub type Txdata = crate::Reg<txdata::TxdataSpec>;
#[doc = "Регистр передаваемых данных"]
pub mod txdata;
#[doc = "MODEM (rw) register accessor: Регистр управления модемом\n\nYou can [`read`](crate::Reg::read) this register and get [`modem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modem`]
module"]
#[doc(alias = "MODEM")]
pub type Modem = crate::Reg<modem::ModemSpec>;
#[doc = "Регистр управления модемом"]
pub mod modem;
