#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ch1_dst: Ch1Dst,
    ch1_src: Ch1Src,
    ch1_len: Ch1Len,
    ch1_cfg: Ch1Cfg,
    ch2_dst: Ch2Dst,
    ch2_src: Ch2Src,
    ch2_len: Ch2Len,
    ch2_cfg: Ch2Cfg,
    ch3_dst: Ch3Dst,
    ch3_src: Ch3Src,
    ch3_len: Ch3Len,
    ch3_cfg: Ch3Cfg,
    ch4_dst: Ch4Dst,
    ch4_src: Ch4Src,
    ch4_len: Ch4Len,
    ch4_cfg: Ch4Cfg,
    _reserved_16_config: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - Регистр адреса назначения канала 1"]
    #[inline(always)]
    pub const fn ch1_dst(&self) -> &Ch1Dst {
        &self.ch1_dst
    }
    #[doc = "0x04 - Регистр адреса источника канала 1"]
    #[inline(always)]
    pub const fn ch1_src(&self) -> &Ch1Src {
        &self.ch1_src
    }
    #[doc = "0x08 - Регистр размера передаваемых данных канала 1"]
    #[inline(always)]
    pub const fn ch1_len(&self) -> &Ch1Len {
        &self.ch1_len
    }
    #[doc = "0x0c - Регистр управления и конфигурации канала 1"]
    #[inline(always)]
    pub const fn ch1_cfg(&self) -> &Ch1Cfg {
        &self.ch1_cfg
    }
    #[doc = "0x10 - Регистр адреса назначения канала 2"]
    #[inline(always)]
    pub const fn ch2_dst(&self) -> &Ch2Dst {
        &self.ch2_dst
    }
    #[doc = "0x14 - Регистр адреса источника канала 2"]
    #[inline(always)]
    pub const fn ch2_src(&self) -> &Ch2Src {
        &self.ch2_src
    }
    #[doc = "0x18 - Регистр размера передаваемых данных канала 2"]
    #[inline(always)]
    pub const fn ch2_len(&self) -> &Ch2Len {
        &self.ch2_len
    }
    #[doc = "0x1c - Регистр управления и конфигурации канала 2"]
    #[inline(always)]
    pub const fn ch2_cfg(&self) -> &Ch2Cfg {
        &self.ch2_cfg
    }
    #[doc = "0x20 - Регистр адреса назначения канала 3"]
    #[inline(always)]
    pub const fn ch3_dst(&self) -> &Ch3Dst {
        &self.ch3_dst
    }
    #[doc = "0x24 - Регистр адреса источника канала 3"]
    #[inline(always)]
    pub const fn ch3_src(&self) -> &Ch3Src {
        &self.ch3_src
    }
    #[doc = "0x28 - Регистр размера передаваемых данных канала 3"]
    #[inline(always)]
    pub const fn ch3_len(&self) -> &Ch3Len {
        &self.ch3_len
    }
    #[doc = "0x2c - Регистр управления и конфигурации канала 3"]
    #[inline(always)]
    pub const fn ch3_cfg(&self) -> &Ch3Cfg {
        &self.ch3_cfg
    }
    #[doc = "0x30 - Регистр адреса назначения канала 4"]
    #[inline(always)]
    pub const fn ch4_dst(&self) -> &Ch4Dst {
        &self.ch4_dst
    }
    #[doc = "0x34 - Регистр адреса источника канала 4"]
    #[inline(always)]
    pub const fn ch4_src(&self) -> &Ch4Src {
        &self.ch4_src
    }
    #[doc = "0x38 - Регистр размера передаваемых данных канала 4"]
    #[inline(always)]
    pub const fn ch4_len(&self) -> &Ch4Len {
        &self.ch4_len
    }
    #[doc = "0x3c - Регистр управления и конфигурации канала 4"]
    #[inline(always)]
    pub const fn ch4_cfg(&self) -> &Ch4Cfg {
        &self.ch4_cfg
    }
    #[doc = "0x40 - Регистр прерываний и настройки контроллера"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x40 - Регистр прерываний и настройки контроллера"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(64).cast() }
    }
}
#[doc = "CH1_DST (rw) register accessor: Регистр адреса назначения канала 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_dst`]
module"]
#[doc(alias = "CH1_DST")]
pub type Ch1Dst = crate::Reg<ch1_dst::Ch1DstSpec>;
#[doc = "Регистр адреса назначения канала 1"]
pub mod ch1_dst;
#[doc = "CH1_SRC (rw) register accessor: Регистр адреса источника канала 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_src`]
module"]
#[doc(alias = "CH1_SRC")]
pub type Ch1Src = crate::Reg<ch1_src::Ch1SrcSpec>;
#[doc = "Регистр адреса источника канала 1"]
pub mod ch1_src;
#[doc = "CH1_LEN (rw) register accessor: Регистр размера передаваемых данных канала 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_len`]
module"]
#[doc(alias = "CH1_LEN")]
pub type Ch1Len = crate::Reg<ch1_len::Ch1LenSpec>;
#[doc = "Регистр размера передаваемых данных канала 1"]
pub mod ch1_len;
#[doc = "CH1_CFG (rw) register accessor: Регистр управления и конфигурации канала 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_cfg`]
module"]
#[doc(alias = "CH1_CFG")]
pub type Ch1Cfg = crate::Reg<ch1_cfg::Ch1CfgSpec>;
#[doc = "Регистр управления и конфигурации канала 1"]
pub mod ch1_cfg;
#[doc = "CH2_DST (rw) register accessor: Регистр адреса назначения канала 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_dst`]
module"]
#[doc(alias = "CH2_DST")]
pub type Ch2Dst = crate::Reg<ch2_dst::Ch2DstSpec>;
#[doc = "Регистр адреса назначения канала 2"]
pub mod ch2_dst;
#[doc = "CH2_SRC (rw) register accessor: Регистр адреса источника канала 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_src`]
module"]
#[doc(alias = "CH2_SRC")]
pub type Ch2Src = crate::Reg<ch2_src::Ch2SrcSpec>;
#[doc = "Регистр адреса источника канала 2"]
pub mod ch2_src;
#[doc = "CH2_LEN (rw) register accessor: Регистр размера передаваемых данных канала 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_len`]
module"]
#[doc(alias = "CH2_LEN")]
pub type Ch2Len = crate::Reg<ch2_len::Ch2LenSpec>;
#[doc = "Регистр размера передаваемых данных канала 2"]
pub mod ch2_len;
#[doc = "CH2_CFG (rw) register accessor: Регистр управления и конфигурации канала 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_cfg`]
module"]
#[doc(alias = "CH2_CFG")]
pub type Ch2Cfg = crate::Reg<ch2_cfg::Ch2CfgSpec>;
#[doc = "Регистр управления и конфигурации канала 2"]
pub mod ch2_cfg;
#[doc = "CH3_DST (rw) register accessor: Регистр адреса назначения канала 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_dst`]
module"]
#[doc(alias = "CH3_DST")]
pub type Ch3Dst = crate::Reg<ch3_dst::Ch3DstSpec>;
#[doc = "Регистр адреса назначения канала 3"]
pub mod ch3_dst;
#[doc = "CH3_SRC (rw) register accessor: Регистр адреса источника канала 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_src`]
module"]
#[doc(alias = "CH3_SRC")]
pub type Ch3Src = crate::Reg<ch3_src::Ch3SrcSpec>;
#[doc = "Регистр адреса источника канала 3"]
pub mod ch3_src;
#[doc = "CH3_LEN (rw) register accessor: Регистр размера передаваемых данных канала 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_len`]
module"]
#[doc(alias = "CH3_LEN")]
pub type Ch3Len = crate::Reg<ch3_len::Ch3LenSpec>;
#[doc = "Регистр размера передаваемых данных канала 3"]
pub mod ch3_len;
#[doc = "CH3_CFG (rw) register accessor: Регистр управления и конфигурации канала 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_cfg`]
module"]
#[doc(alias = "CH3_CFG")]
pub type Ch3Cfg = crate::Reg<ch3_cfg::Ch3CfgSpec>;
#[doc = "Регистр управления и конфигурации канала 3"]
pub mod ch3_cfg;
#[doc = "CH4_DST (rw) register accessor: Регистр адреса назначения канала 4\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_dst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_dst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_dst`]
module"]
#[doc(alias = "CH4_DST")]
pub type Ch4Dst = crate::Reg<ch4_dst::Ch4DstSpec>;
#[doc = "Регистр адреса назначения канала 4"]
pub mod ch4_dst;
#[doc = "CH4_SRC (rw) register accessor: Регистр адреса источника канала 4\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_src::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_src::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_src`]
module"]
#[doc(alias = "CH4_SRC")]
pub type Ch4Src = crate::Reg<ch4_src::Ch4SrcSpec>;
#[doc = "Регистр адреса источника канала 4"]
pub mod ch4_src;
#[doc = "CH4_LEN (rw) register accessor: Регистр размера передаваемых данных канала 4\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_len`]
module"]
#[doc(alias = "CH4_LEN")]
pub type Ch4Len = crate::Reg<ch4_len::Ch4LenSpec>;
#[doc = "Регистр размера передаваемых данных канала 4"]
pub mod ch4_len;
#[doc = "CH4_CFG (rw) register accessor: Регистр управления и конфигурации канала 4\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_cfg`]
module"]
#[doc(alias = "CH4_CFG")]
pub type Ch4Cfg = crate::Reg<ch4_cfg::Ch4CfgSpec>;
#[doc = "Регистр управления и конфигурации канала 4"]
pub mod ch4_cfg;
#[doc = "CONFIG (w) register accessor: Регистр прерываний и настройки контроллера\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Регистр прерываний и настройки контроллера"]
pub mod config;
#[doc = "STATUS (r) register accessor: Регистр прерываний и настройки контроллера\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Регистр прерываний и настройки контроллера"]
pub mod status;
