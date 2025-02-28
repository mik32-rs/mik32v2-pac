#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mask_edge_set: MaskEdgeSet,
    mask_edge_clear: MaskEdgeClear,
    mask_level_set: MaskLevelSet,
    mask_level_clear: MaskLevelClear,
    _reserved4: [u8; 0x08],
    clear: Clear,
    status: Status,
    raw_status: RawStatus,
}
impl RegisterBlock {
    #[doc = "0x00 - При чтении – текущее состоянии масок прерываний по фронту. При записи, «1» разрешает прерывание по фронту соответствующего источника прерываний."]
    #[inline(always)]
    pub const fn mask_edge_set(&self) -> &MaskEdgeSet {
        &self.mask_edge_set
    }
    #[doc = "0x04 - Установка маски прерываний по фронту"]
    #[inline(always)]
    pub const fn mask_edge_clear(&self) -> &MaskEdgeClear {
        &self.mask_edge_clear
    }
    #[doc = "0x08 - Установка маски прерываний по уровню"]
    #[inline(always)]
    pub const fn mask_level_set(&self) -> &MaskLevelSet {
        &self.mask_level_set
    }
    #[doc = "0x0c - Сброс маски прерываний по уровню"]
    #[inline(always)]
    pub const fn mask_level_clear(&self) -> &MaskLevelClear {
        &self.mask_level_clear
    }
    #[doc = "0x18 - Сброс флагов в статусе прерываний"]
    #[inline(always)]
    pub const fn clear(&self) -> &Clear {
        &self.clear
    }
    #[doc = "0x1c - Сброс прерываний"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x20 - Текущее состоянии линий прерываний"]
    #[inline(always)]
    pub const fn raw_status(&self) -> &RawStatus {
        &self.raw_status
    }
}
#[doc = "MASK_EDGE_SET (rw) register accessor: При чтении – текущее состоянии масок прерываний по фронту. При записи, «1» разрешает прерывание по фронту соответствующего источника прерываний.\n\nYou can [`read`](crate::Reg::read) this register and get [`mask_edge_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_edge_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask_edge_set`]
module"]
#[doc(alias = "MASK_EDGE_SET")]
pub type MaskEdgeSet = crate::Reg<mask_edge_set::MaskEdgeSetSpec>;
#[doc = "При чтении – текущее состоянии масок прерываний по фронту. При записи, «1» разрешает прерывание по фронту соответствующего источника прерываний."]
pub mod mask_edge_set;
#[doc = "MASK_EDGE_CLEAR (rw) register accessor: Установка маски прерываний по фронту\n\nYou can [`read`](crate::Reg::read) this register and get [`mask_edge_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_edge_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask_edge_clear`]
module"]
#[doc(alias = "MASK_EDGE_CLEAR")]
pub type MaskEdgeClear = crate::Reg<mask_edge_clear::MaskEdgeClearSpec>;
#[doc = "Установка маски прерываний по фронту"]
pub mod mask_edge_clear;
#[doc = "MASK_LEVEL_SET (w) register accessor: Установка маски прерываний по уровню\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_level_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask_level_set`]
module"]
#[doc(alias = "MASK_LEVEL_SET")]
pub type MaskLevelSet = crate::Reg<mask_level_set::MaskLevelSetSpec>;
#[doc = "Установка маски прерываний по уровню"]
pub mod mask_level_set;
#[doc = "MASK_LEVEL_CLEAR (w) register accessor: Сброс маски прерываний по уровню\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask_level_clear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask_level_clear`]
module"]
#[doc(alias = "MASK_LEVEL_CLEAR")]
pub type MaskLevelClear = crate::Reg<mask_level_clear::MaskLevelClearSpec>;
#[doc = "Сброс маски прерываний по уровню"]
pub mod mask_level_clear;
#[doc = "CLEAR (w) register accessor: Сброс флагов в статусе прерываний\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear`]
module"]
#[doc(alias = "CLEAR")]
pub type Clear = crate::Reg<clear::ClearSpec>;
#[doc = "Сброс флагов в статусе прерываний"]
pub mod clear;
#[doc = "STATUS (r) register accessor: Сброс прерываний\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Сброс прерываний"]
pub mod status;
#[doc = "RAW_STATUS (r) register accessor: Текущее состоянии линий прерываний\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_status`]
module"]
#[doc(alias = "RAW_STATUS")]
pub type RawStatus = crate::Reg<raw_status::RawStatusSpec>;
#[doc = "Текущее состоянии линий прерываний"]
pub mod raw_status;
