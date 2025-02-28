#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    otpdat: Otpdat,
    otpa: Otpa,
    otpcon: Otpcon,
    otpsta: Otpsta,
    otpdec: Otpdec,
    otpadj: Otpadj,
    otpwt1: Otpwt1,
    otpwt2: Otpwt2,
}
impl RegisterBlock {
    #[doc = "0x00 - Регистр чтения/записи данных"]
    #[inline(always)]
    pub const fn otpdat(&self) -> &Otpdat {
        &self.otpdat
    }
    #[doc = "0x04 - Регистр адреса страницы"]
    #[inline(always)]
    pub const fn otpa(&self) -> &Otpa {
        &self.otpa
    }
    #[doc = "0x08 - Регистр управления"]
    #[inline(always)]
    pub const fn otpcon(&self) -> &Otpcon {
        &self.otpcon
    }
    #[doc = "0x0c - Регистр статуса"]
    #[inline(always)]
    pub const fn otpsta(&self) -> &Otpsta {
        &self.otpsta
    }
    #[doc = "0x10 - Регистр дешифратора строк"]
    #[inline(always)]
    pub const fn otpdec(&self) -> &Otpdec {
        &self.otpdec
    }
    #[doc = "0x14 - Регистр управления временными параметрами процедуры чтения и доп. настройками"]
    #[inline(always)]
    pub const fn otpadj(&self) -> &Otpadj {
        &self.otpadj
    }
    #[doc = "0x18 - Регистр подстройки длительности процедуры записи 1"]
    #[inline(always)]
    pub const fn otpwt1(&self) -> &Otpwt1 {
        &self.otpwt1
    }
    #[doc = "0x1c - Регистр подстройки длительности процедуры записи 2"]
    #[inline(always)]
    pub const fn otpwt2(&self) -> &Otpwt2 {
        &self.otpwt2
    }
}
#[doc = "OTPDAT (rw) register accessor: Регистр чтения/записи данных\n\nYou can [`read`](crate::Reg::read) this register and get [`otpdat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otpdat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otpdat`]
module"]
#[doc(alias = "OTPDAT")]
pub type Otpdat = crate::Reg<otpdat::OtpdatSpec>;
#[doc = "Регистр чтения/записи данных"]
pub mod otpdat;
#[doc = "OTPA (rw) register accessor: Регистр адреса страницы\n\nYou can [`read`](crate::Reg::read) this register and get [`otpa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otpa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otpa`]
module"]
#[doc(alias = "OTPA")]
pub type Otpa = crate::Reg<otpa::OtpaSpec>;
#[doc = "Регистр адреса страницы"]
pub mod otpa;
#[doc = "OTPCON (w) register accessor: Регистр управления\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otpcon::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otpcon`]
module"]
#[doc(alias = "OTPCON")]
pub type Otpcon = crate::Reg<otpcon::OtpconSpec>;
#[doc = "Регистр управления"]
pub mod otpcon;
#[doc = "OTPSTA (r) register accessor: Регистр статуса\n\nYou can [`read`](crate::Reg::read) this register and get [`otpsta::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otpsta`]
module"]
#[doc(alias = "OTPSTA")]
pub type Otpsta = crate::Reg<otpsta::OtpstaSpec>;
#[doc = "Регистр статуса"]
pub mod otpsta;
#[doc = "OTPDEC (r) register accessor: Регистр дешифратора строк\n\nYou can [`read`](crate::Reg::read) this register and get [`otpdec::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otpdec`]
module"]
#[doc(alias = "OTPDEC")]
pub type Otpdec = crate::Reg<otpdec::OtpdecSpec>;
#[doc = "Регистр дешифратора строк"]
pub mod otpdec;
#[doc = "OTPADJ (rw) register accessor: Регистр управления временными параметрами процедуры чтения и доп. настройками\n\nYou can [`read`](crate::Reg::read) this register and get [`otpadj::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otpadj::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otpadj`]
module"]
#[doc(alias = "OTPADJ")]
pub type Otpadj = crate::Reg<otpadj::OtpadjSpec>;
#[doc = "Регистр управления временными параметрами процедуры чтения и доп. настройками"]
pub mod otpadj;
#[doc = "OTPWT1 (rw) register accessor: Регистр подстройки длительности процедуры записи 1\n\nYou can [`read`](crate::Reg::read) this register and get [`otpwt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otpwt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otpwt1`]
module"]
#[doc(alias = "OTPWT1")]
pub type Otpwt1 = crate::Reg<otpwt1::Otpwt1Spec>;
#[doc = "Регистр подстройки длительности процедуры записи 1"]
pub mod otpwt1;
#[doc = "OTPWT2 (rw) register accessor: Регистр подстройки длительности процедуры записи 2\n\nYou can [`read`](crate::Reg::read) this register and get [`otpwt2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otpwt2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otpwt2`]
module"]
#[doc(alias = "OTPWT2")]
pub type Otpwt2 = crate::Reg<otpwt2::Otpwt2Spec>;
#[doc = "Регистр подстройки длительности процедуры записи 2"]
pub mod otpwt2;
