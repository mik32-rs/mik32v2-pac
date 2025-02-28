#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    eedat: Eedat,
    eea: Eea,
    eecon: Eecon,
    eesta: Eesta,
    eerb: Eerb,
    eeadj: Eeadj,
    ncycrl: Ncycrl,
    ncycep1: Ncycep1,
    ncycep2: Ncycep2,
}
impl RegisterBlock {
    #[doc = "0x00 - Регистр чтения/записи данных"]
    #[inline(always)]
    pub const fn eedat(&self) -> &Eedat {
        &self.eedat
    }
    #[doc = "0x04 - Регистр адреса страницы"]
    #[inline(always)]
    pub const fn eea(&self) -> &Eea {
        &self.eea
    }
    #[doc = "0x08 - Регистр управления"]
    #[inline(always)]
    pub const fn eecon(&self) -> &Eecon {
        &self.eecon
    }
    #[doc = "0x0c - Регистр статуса"]
    #[inline(always)]
    pub const fn eesta(&self) -> &Eesta {
        &self.eesta
    }
    #[doc = "0x10 - Регистр бит коррекции прочитанного слова"]
    #[inline(always)]
    pub const fn eerb(&self) -> &Eerb {
        &self.eerb
    }
    #[doc = "0x14 - Регистр настроек"]
    #[inline(always)]
    pub const fn eeadj(&self) -> &Eeadj {
        &self.eeadj
    }
    #[doc = "0x18 - Регистр подстройки длительности процедур чтения и заполнения буфера записи"]
    #[inline(always)]
    pub const fn ncycrl(&self) -> &Ncycrl {
        &self.ncycrl
    }
    #[doc = "0x1c - Регистр 1 подстройки длительности процедур стирания и программирования"]
    #[inline(always)]
    pub const fn ncycep1(&self) -> &Ncycep1 {
        &self.ncycep1
    }
    #[doc = "0x20 - Регистр 2 подстройки длительности процедур стирания и программирования"]
    #[inline(always)]
    pub const fn ncycep2(&self) -> &Ncycep2 {
        &self.ncycep2
    }
}
#[doc = "EEDAT (rw) register accessor: Регистр чтения/записи данных\n\nYou can [`read`](crate::Reg::read) this register and get [`eedat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eedat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eedat`]
module"]
#[doc(alias = "EEDAT")]
pub type Eedat = crate::Reg<eedat::EedatSpec>;
#[doc = "Регистр чтения/записи данных"]
pub mod eedat;
#[doc = "EEA (w) register accessor: Регистр адреса страницы\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eea::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eea`]
module"]
#[doc(alias = "EEA")]
pub type Eea = crate::Reg<eea::EeaSpec>;
#[doc = "Регистр адреса страницы"]
pub mod eea;
#[doc = "EECON (w) register accessor: Регистр управления\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eecon::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eecon`]
module"]
#[doc(alias = "EECON")]
pub type Eecon = crate::Reg<eecon::EeconSpec>;
#[doc = "Регистр управления"]
pub mod eecon;
#[doc = "EESTA (rw) register accessor: Регистр статуса\n\nYou can [`read`](crate::Reg::read) this register and get [`eesta::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eesta::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eesta`]
module"]
#[doc(alias = "EESTA")]
pub type Eesta = crate::Reg<eesta::EestaSpec>;
#[doc = "Регистр статуса"]
pub mod eesta;
#[doc = "EERB (r) register accessor: Регистр бит коррекции прочитанного слова\n\nYou can [`read`](crate::Reg::read) this register and get [`eerb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eerb`]
module"]
#[doc(alias = "EERB")]
pub type Eerb = crate::Reg<eerb::EerbSpec>;
#[doc = "Регистр бит коррекции прочитанного слова"]
pub mod eerb;
#[doc = "EEADJ (rw) register accessor: Регистр настроек\n\nYou can [`read`](crate::Reg::read) this register and get [`eeadj::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eeadj::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eeadj`]
module"]
#[doc(alias = "EEADJ")]
pub type Eeadj = crate::Reg<eeadj::EeadjSpec>;
#[doc = "Регистр настроек"]
pub mod eeadj;
#[doc = "NCYCRL (rw) register accessor: Регистр подстройки длительности процедур чтения и заполнения буфера записи\n\nYou can [`read`](crate::Reg::read) this register and get [`ncycrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ncycrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ncycrl`]
module"]
#[doc(alias = "NCYCRL")]
pub type Ncycrl = crate::Reg<ncycrl::NcycrlSpec>;
#[doc = "Регистр подстройки длительности процедур чтения и заполнения буфера записи"]
pub mod ncycrl;
#[doc = "NCYCEP1 (rw) register accessor: Регистр 1 подстройки длительности процедур стирания и программирования\n\nYou can [`read`](crate::Reg::read) this register and get [`ncycep1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ncycep1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ncycep1`]
module"]
#[doc(alias = "NCYCEP1")]
pub type Ncycep1 = crate::Reg<ncycep1::Ncycep1Spec>;
#[doc = "Регистр 1 подстройки длительности процедур стирания и программирования"]
pub mod ncycep1;
#[doc = "NCYCEP2 (rw) register accessor: Регистр 2 подстройки длительности процедур стирания и программирования\n\nYou can [`read`](crate::Reg::read) this register and get [`ncycep2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ncycep2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ncycep2`]
module"]
#[doc(alias = "NCYCEP2")]
pub type Ncycep2 = crate::Reg<ncycep2::Ncycep2Spec>;
#[doc = "Регистр 2 подстройки длительности процедур стирания и программирования"]
pub mod ncycep2;
