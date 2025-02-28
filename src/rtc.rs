#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rrtc_time: RrtcTime,
    rrtc_date: RrtcDate,
    rrtc_talrm: RrtcTalrm,
    rrtc_dalrm: RrtcDalrm,
    rrtc_ctrl: RrtcCtrl,
    _reserved5: [u8; 0x0c],
    rrtc_reg0: RrtcReg0,
    rrtc_reg1: RrtcReg1,
    rrtc_reg2: RrtcReg2,
    rrtc_reg3: RrtcReg3,
    rrtc_reg4: RrtcReg4,
    rrtc_reg5: RrtcReg5,
    rrtc_reg6: RrtcReg6,
    rrtc_reg7: RrtcReg7,
    rrtc_reg8: RrtcReg8,
    rrtc_reg9: RrtcReg9,
    rrtc_reg10: RrtcReg10,
    rrtc_reg11: RrtcReg11,
    rrtc_reg12: RrtcReg12,
    rrtc_reg13: RrtcReg13,
    rrtc_reg14: RrtcReg14,
    rrtc_reg15: RrtcReg15,
}
impl RegisterBlock {
    #[doc = "0x00 - Регистр установки времени. Используется BCD-кодировка"]
    #[inline(always)]
    pub const fn rrtc_time(&self) -> &RrtcTime {
        &self.rrtc_time
    }
    #[doc = "0x04 - Регистр установки даты. Используется BCD-кодировка"]
    #[inline(always)]
    pub const fn rrtc_date(&self) -> &RrtcDate {
        &self.rrtc_date
    }
    #[doc = "0x08 - регистр хранит время, при совпадении которого со значением регистра RRTC_TIME, будет сгенерировано соответствующее прерывание"]
    #[inline(always)]
    pub const fn rrtc_talrm(&self) -> &RrtcTalrm {
        &self.rrtc_talrm
    }
    #[doc = "0x0c - регистр хранит время, при совпадении которого со значением регистра RRTC_TIME, будет сгенерировано соответствующее прерывание."]
    #[inline(always)]
    pub const fn rrtc_dalrm(&self) -> &RrtcDalrm {
        &self.rrtc_dalrm
    }
    #[doc = "0x10 - регистр управления модулем"]
    #[inline(always)]
    pub const fn rrtc_ctrl(&self) -> &RrtcCtrl {
        &self.rrtc_ctrl
    }
    #[doc = "0x20 - Регистры общего назначения REG0"]
    #[inline(always)]
    pub const fn rrtc_reg0(&self) -> &RrtcReg0 {
        &self.rrtc_reg0
    }
    #[doc = "0x24 - Регистры общего назначения REG1"]
    #[inline(always)]
    pub const fn rrtc_reg1(&self) -> &RrtcReg1 {
        &self.rrtc_reg1
    }
    #[doc = "0x28 - Регистры общего назначения REG2"]
    #[inline(always)]
    pub const fn rrtc_reg2(&self) -> &RrtcReg2 {
        &self.rrtc_reg2
    }
    #[doc = "0x2c - Регистры общего назначения REG3"]
    #[inline(always)]
    pub const fn rrtc_reg3(&self) -> &RrtcReg3 {
        &self.rrtc_reg3
    }
    #[doc = "0x30 - Регистры общего назначения REG4"]
    #[inline(always)]
    pub const fn rrtc_reg4(&self) -> &RrtcReg4 {
        &self.rrtc_reg4
    }
    #[doc = "0x34 - Регистры общего назначения REG5"]
    #[inline(always)]
    pub const fn rrtc_reg5(&self) -> &RrtcReg5 {
        &self.rrtc_reg5
    }
    #[doc = "0x38 - Регистры общего назначения REG6"]
    #[inline(always)]
    pub const fn rrtc_reg6(&self) -> &RrtcReg6 {
        &self.rrtc_reg6
    }
    #[doc = "0x3c - Регистры общего назначения REG7"]
    #[inline(always)]
    pub const fn rrtc_reg7(&self) -> &RrtcReg7 {
        &self.rrtc_reg7
    }
    #[doc = "0x40 - Регистры общего назначения REG8"]
    #[inline(always)]
    pub const fn rrtc_reg8(&self) -> &RrtcReg8 {
        &self.rrtc_reg8
    }
    #[doc = "0x44 - Регистры общего назначения REG9"]
    #[inline(always)]
    pub const fn rrtc_reg9(&self) -> &RrtcReg9 {
        &self.rrtc_reg9
    }
    #[doc = "0x48 - Регистры общего назначения REG10"]
    #[inline(always)]
    pub const fn rrtc_reg10(&self) -> &RrtcReg10 {
        &self.rrtc_reg10
    }
    #[doc = "0x4c - Регистры общего назначения REG11"]
    #[inline(always)]
    pub const fn rrtc_reg11(&self) -> &RrtcReg11 {
        &self.rrtc_reg11
    }
    #[doc = "0x50 - Регистры общего назначения REG12"]
    #[inline(always)]
    pub const fn rrtc_reg12(&self) -> &RrtcReg12 {
        &self.rrtc_reg12
    }
    #[doc = "0x54 - Регистры общего назначения REG13"]
    #[inline(always)]
    pub const fn rrtc_reg13(&self) -> &RrtcReg13 {
        &self.rrtc_reg13
    }
    #[doc = "0x58 - Регистры общего назначения REG14"]
    #[inline(always)]
    pub const fn rrtc_reg14(&self) -> &RrtcReg14 {
        &self.rrtc_reg14
    }
    #[doc = "0x5c - Регистры общего назначения REG14"]
    #[inline(always)]
    pub const fn rrtc_reg15(&self) -> &RrtcReg15 {
        &self.rrtc_reg15
    }
}
#[doc = "RRTC_TIME (rw) register accessor: Регистр установки времени. Используется BCD-кодировка\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrtc_time`]
module"]
#[doc(alias = "RRTC_TIME")]
pub type RrtcTime = crate::Reg<rrtc_time::RrtcTimeSpec>;
#[doc = "Регистр установки времени. Используется BCD-кодировка"]
pub mod rrtc_time;
#[doc = "RRTC_DATE (rw) register accessor: Регистр установки даты. Используется BCD-кодировка\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrtc_date`]
module"]
#[doc(alias = "RRTC_DATE")]
pub type RrtcDate = crate::Reg<rrtc_date::RrtcDateSpec>;
#[doc = "Регистр установки даты. Используется BCD-кодировка"]
pub mod rrtc_date;
#[doc = "RRTC_TALRM (rw) register accessor: регистр хранит время, при совпадении которого со значением регистра RRTC_TIME, будет сгенерировано соответствующее прерывание\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_talrm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_talrm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrtc_talrm`]
module"]
#[doc(alias = "RRTC_TALRM")]
pub type RrtcTalrm = crate::Reg<rrtc_talrm::RrtcTalrmSpec>;
#[doc = "регистр хранит время, при совпадении которого со значением регистра RRTC_TIME, будет сгенерировано соответствующее прерывание"]
pub mod rrtc_talrm;
#[doc = "RRTC_DALRM (rw) register accessor: регистр хранит время, при совпадении которого со значением регистра RRTC_TIME, будет сгенерировано соответствующее прерывание.\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_dalrm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_dalrm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrtc_dalrm`]
module"]
#[doc(alias = "RRTC_DALRM")]
pub type RrtcDalrm = crate::Reg<rrtc_dalrm::RrtcDalrmSpec>;
#[doc = "регистр хранит время, при совпадении которого со значением регистра RRTC_TIME, будет сгенерировано соответствующее прерывание."]
pub mod rrtc_dalrm;
#[doc = "RRTC_CTRL (rw) register accessor: регистр управления модулем\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrtc_ctrl`]
module"]
#[doc(alias = "RRTC_CTRL")]
pub type RrtcCtrl = crate::Reg<rrtc_ctrl::RrtcCtrlSpec>;
#[doc = "регистр управления модулем"]
pub mod rrtc_ctrl;
#[doc = "RRTC_REG0 (rw) register accessor: Регистры общего назначения REG0\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrtc_reg0`]
module"]
#[doc(alias = "RRTC_REG0")]
pub type RrtcReg0 = crate::Reg<rrtc_reg0::RrtcReg0Spec>;
#[doc = "Регистры общего назначения REG0"]
pub mod rrtc_reg0;
#[doc = "RRTC_REG1 (rw) register accessor: Регистры общего назначения REG1\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrtc_reg1`]
module"]
#[doc(alias = "RRTC_REG1")]
pub type RrtcReg1 = crate::Reg<rrtc_reg1::RrtcReg1Spec>;
#[doc = "Регистры общего назначения REG1"]
pub mod rrtc_reg1;
#[doc = "RRTC_REG2 (rw) register accessor: Регистры общего назначения REG2\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_reg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_reg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrtc_reg2`]
module"]
#[doc(alias = "RRTC_REG2")]
pub type RrtcReg2 = crate::Reg<rrtc_reg2::RrtcReg2Spec>;
#[doc = "Регистры общего назначения REG2"]
pub mod rrtc_reg2;
#[doc = "RRTC_REG3 (rw) register accessor: Регистры общего назначения REG3\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_reg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_reg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrtc_reg3`]
module"]
#[doc(alias = "RRTC_REG3")]
pub type RrtcReg3 = crate::Reg<rrtc_reg3::RrtcReg3Spec>;
#[doc = "Регистры общего назначения REG3"]
pub mod rrtc_reg3;
#[doc = "RRTC_REG4 (rw) register accessor: Регистры общего назначения REG4\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_reg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_reg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrtc_reg4`]
module"]
#[doc(alias = "RRTC_REG4")]
pub type RrtcReg4 = crate::Reg<rrtc_reg4::RrtcReg4Spec>;
#[doc = "Регистры общего назначения REG4"]
pub mod rrtc_reg4;
#[doc = "RRTC_REG5 (rw) register accessor: Регистры общего назначения REG5\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_reg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_reg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrtc_reg5`]
module"]
#[doc(alias = "RRTC_REG5")]
pub type RrtcReg5 = crate::Reg<rrtc_reg5::RrtcReg5Spec>;
#[doc = "Регистры общего назначения REG5"]
pub mod rrtc_reg5;
#[doc = "RRTC_REG6 (rw) register accessor: Регистры общего назначения REG6\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_reg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_reg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrtc_reg6`]
module"]
#[doc(alias = "RRTC_REG6")]
pub type RrtcReg6 = crate::Reg<rrtc_reg6::RrtcReg6Spec>;
#[doc = "Регистры общего назначения REG6"]
pub mod rrtc_reg6;
#[doc = "RRTC_REG7 (rw) register accessor: Регистры общего назначения REG7\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_reg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_reg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrtc_reg7`]
module"]
#[doc(alias = "RRTC_REG7")]
pub type RrtcReg7 = crate::Reg<rrtc_reg7::RrtcReg7Spec>;
#[doc = "Регистры общего назначения REG7"]
pub mod rrtc_reg7;
#[doc = "RRTC_REG8 (rw) register accessor: Регистры общего назначения REG8\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_reg8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_reg8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrtc_reg8`]
module"]
#[doc(alias = "RRTC_REG8")]
pub type RrtcReg8 = crate::Reg<rrtc_reg8::RrtcReg8Spec>;
#[doc = "Регистры общего назначения REG8"]
pub mod rrtc_reg8;
#[doc = "RRTC_REG9 (rw) register accessor: Регистры общего назначения REG9\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_reg9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_reg9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrtc_reg9`]
module"]
#[doc(alias = "RRTC_REG9")]
pub type RrtcReg9 = crate::Reg<rrtc_reg9::RrtcReg9Spec>;
#[doc = "Регистры общего назначения REG9"]
pub mod rrtc_reg9;
#[doc = "RRTC_REG10 (rw) register accessor: Регистры общего назначения REG10\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_reg10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_reg10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrtc_reg10`]
module"]
#[doc(alias = "RRTC_REG10")]
pub type RrtcReg10 = crate::Reg<rrtc_reg10::RrtcReg10Spec>;
#[doc = "Регистры общего назначения REG10"]
pub mod rrtc_reg10;
#[doc = "RRTC_REG11 (rw) register accessor: Регистры общего назначения REG11\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_reg11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_reg11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrtc_reg11`]
module"]
#[doc(alias = "RRTC_REG11")]
pub type RrtcReg11 = crate::Reg<rrtc_reg11::RrtcReg11Spec>;
#[doc = "Регистры общего назначения REG11"]
pub mod rrtc_reg11;
#[doc = "RRTC_REG12 (rw) register accessor: Регистры общего назначения REG12\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_reg12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_reg12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrtc_reg12`]
module"]
#[doc(alias = "RRTC_REG12")]
pub type RrtcReg12 = crate::Reg<rrtc_reg12::RrtcReg12Spec>;
#[doc = "Регистры общего назначения REG12"]
pub mod rrtc_reg12;
#[doc = "RRTC_REG13 (rw) register accessor: Регистры общего назначения REG13\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_reg13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_reg13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrtc_reg13`]
module"]
#[doc(alias = "RRTC_REG13")]
pub type RrtcReg13 = crate::Reg<rrtc_reg13::RrtcReg13Spec>;
#[doc = "Регистры общего назначения REG13"]
pub mod rrtc_reg13;
#[doc = "RRTC_REG14 (rw) register accessor: Регистры общего назначения REG14\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_reg14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_reg14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrtc_reg14`]
module"]
#[doc(alias = "RRTC_REG14")]
pub type RrtcReg14 = crate::Reg<rrtc_reg14::RrtcReg14Spec>;
#[doc = "Регистры общего назначения REG14"]
pub mod rrtc_reg14;
#[doc = "RRTC_REG15 (rw) register accessor: Регистры общего назначения REG14\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_reg15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_reg15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rrtc_reg15`]
module"]
#[doc(alias = "RRTC_REG15")]
pub type RrtcReg15 = crate::Reg<rrtc_reg15::RrtcReg15Spec>;
#[doc = "Регистры общего назначения REG14"]
pub mod rrtc_reg15;
