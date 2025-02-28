#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    data: Data,
    poly: Poly,
    ctrl: Ctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - Регистр данных"]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    #[doc = "0x04 - Регистр полинома"]
    #[inline(always)]
    pub const fn poly(&self) -> &Poly {
        &self.poly
    }
    #[doc = "0x08 - Регистр управления"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
}
#[doc = "DATA (rw) register accessor: Регистр данных\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "Регистр данных"]
pub mod data;
#[doc = "POLY (rw) register accessor: Регистр полинома\n\nYou can [`read`](crate::Reg::read) this register and get [`poly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@poly`]
module"]
#[doc(alias = "POLY")]
pub type Poly = crate::Reg<poly::PolySpec>;
#[doc = "Регистр полинома"]
pub mod poly;
#[doc = "CTRL (rw) register accessor: Регистр управления\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Регистр управления"]
pub mod ctrl;
