#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pad0_cfg: Pad0Cfg,
    pad0_ds: Pad0Ds,
    pad0_pupd: Pad0Pupd,
    pad1_cfg: Pad1Cfg,
    pad1_ds: Pad1Ds,
    pad1_pupd: Pad1Pupd,
    pad2_cfg: Pad2Cfg,
    pad2_ds: Pad2Ds,
    pad2_pupd: Pad2Pupd,
}
impl RegisterBlock {
    #[doc = "0x00 - Управление функциями выводов PORT0"]
    #[inline(always)]
    pub const fn pad0_cfg(&self) -> &Pad0Cfg {
        &self.pad0_cfg
    }
    #[doc = "0x04 - Управление нагрузочной способностью выводов PORT0"]
    #[inline(always)]
    pub const fn pad0_ds(&self) -> &Pad0Ds {
        &self.pad0_ds
    }
    #[doc = "0x08 - Управление резисторами подтяжки выводов PORT0"]
    #[inline(always)]
    pub const fn pad0_pupd(&self) -> &Pad0Pupd {
        &self.pad0_pupd
    }
    #[doc = "0x0c - Управление функциями выводов PORT1"]
    #[inline(always)]
    pub const fn pad1_cfg(&self) -> &Pad1Cfg {
        &self.pad1_cfg
    }
    #[doc = "0x10 - Управление нагрузочной способностью выводов PORT1"]
    #[inline(always)]
    pub const fn pad1_ds(&self) -> &Pad1Ds {
        &self.pad1_ds
    }
    #[doc = "0x14 - Управление резисторами подтяжки выводов PORT1"]
    #[inline(always)]
    pub const fn pad1_pupd(&self) -> &Pad1Pupd {
        &self.pad1_pupd
    }
    #[doc = "0x18 - Управление функциями выводов PORT2"]
    #[inline(always)]
    pub const fn pad2_cfg(&self) -> &Pad2Cfg {
        &self.pad2_cfg
    }
    #[doc = "0x1c - Управление нагрузочной способностью выводов PORT2"]
    #[inline(always)]
    pub const fn pad2_ds(&self) -> &Pad2Ds {
        &self.pad2_ds
    }
    #[doc = "0x20 - Управление резисторами подтяжки выводов PORT2"]
    #[inline(always)]
    pub const fn pad2_pupd(&self) -> &Pad2Pupd {
        &self.pad2_pupd
    }
}
#[doc = "PAD0_CFG (rw) register accessor: Управление функциями выводов PORT0\n\nYou can [`read`](crate::Reg::read) this register and get [`pad0_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad0_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad0_cfg`]
module"]
#[doc(alias = "PAD0_CFG")]
pub type Pad0Cfg = crate::Reg<pad0_cfg::Pad0CfgSpec>;
#[doc = "Управление функциями выводов PORT0"]
pub mod pad0_cfg;
#[doc = "PAD0_DS (rw) register accessor: Управление нагрузочной способностью выводов PORT0\n\nYou can [`read`](crate::Reg::read) this register and get [`pad0_ds::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad0_ds::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad0_ds`]
module"]
#[doc(alias = "PAD0_DS")]
pub type Pad0Ds = crate::Reg<pad0_ds::Pad0DsSpec>;
#[doc = "Управление нагрузочной способностью выводов PORT0"]
pub mod pad0_ds;
#[doc = "PAD0_PUPD (rw) register accessor: Управление резисторами подтяжки выводов PORT0\n\nYou can [`read`](crate::Reg::read) this register and get [`pad0_pupd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad0_pupd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad0_pupd`]
module"]
#[doc(alias = "PAD0_PUPD")]
pub type Pad0Pupd = crate::Reg<pad0_pupd::Pad0PupdSpec>;
#[doc = "Управление резисторами подтяжки выводов PORT0"]
pub mod pad0_pupd;
#[doc = "PAD1_CFG (rw) register accessor: Управление функциями выводов PORT1\n\nYou can [`read`](crate::Reg::read) this register and get [`pad1_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad1_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad1_cfg`]
module"]
#[doc(alias = "PAD1_CFG")]
pub type Pad1Cfg = crate::Reg<pad1_cfg::Pad1CfgSpec>;
#[doc = "Управление функциями выводов PORT1"]
pub mod pad1_cfg;
#[doc = "PAD1_DS (rw) register accessor: Управление нагрузочной способностью выводов PORT1\n\nYou can [`read`](crate::Reg::read) this register and get [`pad1_ds::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad1_ds::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad1_ds`]
module"]
#[doc(alias = "PAD1_DS")]
pub type Pad1Ds = crate::Reg<pad1_ds::Pad1DsSpec>;
#[doc = "Управление нагрузочной способностью выводов PORT1"]
pub mod pad1_ds;
#[doc = "PAD1_PUPD (rw) register accessor: Управление резисторами подтяжки выводов PORT1\n\nYou can [`read`](crate::Reg::read) this register and get [`pad1_pupd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad1_pupd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad1_pupd`]
module"]
#[doc(alias = "PAD1_PUPD")]
pub type Pad1Pupd = crate::Reg<pad1_pupd::Pad1PupdSpec>;
#[doc = "Управление резисторами подтяжки выводов PORT1"]
pub mod pad1_pupd;
#[doc = "PAD2_CFG (rw) register accessor: Управление функциями выводов PORT2\n\nYou can [`read`](crate::Reg::read) this register and get [`pad2_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad2_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad2_cfg`]
module"]
#[doc(alias = "PAD2_CFG")]
pub type Pad2Cfg = crate::Reg<pad2_cfg::Pad2CfgSpec>;
#[doc = "Управление функциями выводов PORT2"]
pub mod pad2_cfg;
#[doc = "PAD2_DS (rw) register accessor: Управление нагрузочной способностью выводов PORT2\n\nYou can [`read`](crate::Reg::read) this register and get [`pad2_ds::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad2_ds::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad2_ds`]
module"]
#[doc(alias = "PAD2_DS")]
pub type Pad2Ds = crate::Reg<pad2_ds::Pad2DsSpec>;
#[doc = "Управление нагрузочной способностью выводов PORT2"]
pub mod pad2_ds;
#[doc = "PAD2_PUPD (rw) register accessor: Управление резисторами подтяжки выводов PORT2\n\nYou can [`read`](crate::Reg::read) this register and get [`pad2_pupd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad2_pupd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pad2_pupd`]
module"]
#[doc(alias = "PAD2_PUPD")]
pub type Pad2Pupd = crate::Reg<pad2_pupd::Pad2PupdSpec>;
#[doc = "Управление резисторами подтяжки выводов PORT2"]
pub mod pad2_pupd;
