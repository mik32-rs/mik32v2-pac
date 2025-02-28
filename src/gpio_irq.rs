#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    state: State,
    line_mux: LineMux,
    interrupt: Interrupt,
    enable_set: EnableSet,
    enable_clear: EnableClear,
    edge: Edge,
    level: Level,
    level_set: LevelSet,
    level_clear: LevelClear,
    any_edge_set: AnyEdgeSet,
    any_edge_clear: AnyEdgeClear,
    clear: Clear,
}
impl RegisterBlock {
    #[doc = "0x00 - Текущее состояние выводов линий после мультисплексоров. Номер бита соответсвует номеру канала."]
    #[inline(always)]
    pub const fn state(&self) -> &State {
        &self.state
    }
    #[doc = "0x04 - Управление мультиплексорами. Каждый мультиплесор управляется своими 4 битами: Mux_0 – \\[3:0\\]; Mux_1 – \\[7:4\\]; Mux_2 – \\[11:8\\]; Mux_3 – \\[15:12\\]; Mux_4 – \\[19:16\\]; Mux_5– \\[23:20\\]; Mux_6 – \\[27:24\\]; Mux_7 – \\[31:28\\]"]
    #[inline(always)]
    pub const fn line_mux(&self) -> &LineMux {
        &self.line_mux
    }
    #[doc = "0x08 - Текущее состояние прерываний. Номер бита соответсвует номеру канала."]
    #[inline(always)]
    pub const fn interrupt(&self) -> &Interrupt {
        &self.interrupt
    }
    #[doc = "0x0c - Регистр разрешения прерываний При чтении – текущее состояние разрешений прерываний. При записи «1» разрешает прерывание от соответствующего канала."]
    #[inline(always)]
    pub const fn enable_set(&self) -> &EnableSet {
        &self.enable_set
    }
    #[doc = "0x10 - Регистр запрета прерываний При чтении – текущее состояние разрешений прерываний. При записи «1» запрещает прерывание от соответствующего канала"]
    #[inline(always)]
    pub const fn enable_clear(&self) -> &EnableClear {
        &self.enable_clear
    }
    #[doc = "0x14 - Регистр типа прерываний. При чтении – текущий тип прерывания: «1» – по событию (фронт или спад), «0» – по уровню. Запись «1» – прерывание формируется по событию для соответствующего канала Запись «0» - не влияет"]
    #[inline(always)]
    pub const fn edge(&self) -> &Edge {
        &self.edge
    }
    #[doc = "0x18 - Регистр типа прерываний. При чтении – текущий тип прерывания (инвертированный): «0» – по событию; «1» – по уровню. Запись «1» – прерывание формируется по уровню для соответствующего канала Запись «0» - не влияет"]
    #[inline(always)]
    pub const fn level(&self) -> &Level {
        &self.level
    }
    #[doc = "0x1c - Регистр выбора уровня / события прерываний. При чтении: «0» – прерывания формируются по спаду или уровню логического «0»; «1» – прерывания формируются по нарастающему фронту или уровню логической «1» Запись «1» – прерывание формируется по нарастающему фронту или уровню логической «1» для соответствующего канала"]
    #[inline(always)]
    pub const fn level_set(&self) -> &LevelSet {
        &self.level_set
    }
    #[doc = "0x20 - Регистр уровня / со-бытия прерываний. При чтении: «0» – прерывания формируются по спаду или уровню логического «0»; «1» – прерывания формируются по нарастающему фронту или уровню логической «1» Запись «1»– прерывание формируется по спаду или уровню логического «0» для соответствующего канала"]
    #[inline(always)]
    pub const fn level_clear(&self) -> &LevelClear {
        &self.level_clear
    }
    #[doc = "0x24 - Регистр прерываний по любому событию. Запись «1» – прерывание формируется по любому изменению соответствующего канала"]
    #[inline(always)]
    pub const fn any_edge_set(&self) -> &AnyEdgeSet {
        &self.any_edge_set
    }
    #[doc = "0x28 - Регистр прерываний по любому событию. Запись «1» – прерывание не формируется по любому изменению соответствующего канала"]
    #[inline(always)]
    pub const fn any_edge_clear(&self) -> &AnyEdgeClear {
        &self.any_edge_clear
    }
    #[doc = "0x2c - Регистр сброса флагов прерываний по событию. Запись «1» – очищает флаг прерывания соответствующего канала"]
    #[inline(always)]
    pub const fn clear(&self) -> &Clear {
        &self.clear
    }
}
#[doc = "STATE (r) register accessor: Текущее состояние выводов линий после мультисплексоров. Номер бита соответсвует номеру канала.\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`]
module"]
#[doc(alias = "STATE")]
pub type State = crate::Reg<state::StateSpec>;
#[doc = "Текущее состояние выводов линий после мультисплексоров. Номер бита соответсвует номеру канала."]
pub mod state;
#[doc = "LINE_MUX (rw) register accessor: Управление мультиплексорами. Каждый мультиплесор управляется своими 4 битами: Mux_0 – \\[3:0\\]; Mux_1 – \\[7:4\\]; Mux_2 – \\[11:8\\]; Mux_3 – \\[15:12\\]; Mux_4 – \\[19:16\\]; Mux_5– \\[23:20\\]; Mux_6 – \\[27:24\\]; Mux_7 – \\[31:28\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`line_mux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`line_mux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@line_mux`]
module"]
#[doc(alias = "LINE_MUX")]
pub type LineMux = crate::Reg<line_mux::LineMuxSpec>;
#[doc = "Управление мультиплексорами. Каждый мультиплесор управляется своими 4 битами: Mux_0 – \\[3:0\\]; Mux_1 – \\[7:4\\]; Mux_2 – \\[11:8\\]; Mux_3 – \\[15:12\\]; Mux_4 – \\[19:16\\]; Mux_5– \\[23:20\\]; Mux_6 – \\[27:24\\]; Mux_7 – \\[31:28\\]"]
pub mod line_mux;
#[doc = "INTERRUPT (rw) register accessor: Текущее состояние прерываний. Номер бита соответсвует номеру канала.\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrupt`]
module"]
#[doc(alias = "INTERRUPT")]
pub type Interrupt = crate::Reg<interrupt::InterruptSpec>;
#[doc = "Текущее состояние прерываний. Номер бита соответсвует номеру канала."]
pub mod interrupt;
#[doc = "ENABLE_SET (rw) register accessor: Регистр разрешения прерываний При чтении – текущее состояние разрешений прерываний. При записи «1» разрешает прерывание от соответствующего канала.\n\nYou can [`read`](crate::Reg::read) this register and get [`enable_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_set`]
module"]
#[doc(alias = "ENABLE_SET")]
pub type EnableSet = crate::Reg<enable_set::EnableSetSpec>;
#[doc = "Регистр разрешения прерываний При чтении – текущее состояние разрешений прерываний. При записи «1» разрешает прерывание от соответствующего канала."]
pub mod enable_set;
#[doc = "ENABLE_CLEAR (rw) register accessor: Регистр запрета прерываний При чтении – текущее состояние разрешений прерываний. При записи «1» запрещает прерывание от соответствующего канала\n\nYou can [`read`](crate::Reg::read) this register and get [`enable_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_clear`]
module"]
#[doc(alias = "ENABLE_CLEAR")]
pub type EnableClear = crate::Reg<enable_clear::EnableClearSpec>;
#[doc = "Регистр запрета прерываний При чтении – текущее состояние разрешений прерываний. При записи «1» запрещает прерывание от соответствующего канала"]
pub mod enable_clear;
#[doc = "EDGE (rw) register accessor: Регистр типа прерываний. При чтении – текущий тип прерывания: «1» – по событию (фронт или спад), «0» – по уровню. Запись «1» – прерывание формируется по событию для соответствующего канала Запись «0» - не влияет\n\nYou can [`read`](crate::Reg::read) this register and get [`edge::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edge::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edge`]
module"]
#[doc(alias = "EDGE")]
pub type Edge = crate::Reg<edge::EdgeSpec>;
#[doc = "Регистр типа прерываний. При чтении – текущий тип прерывания: «1» – по событию (фронт или спад), «0» – по уровню. Запись «1» – прерывание формируется по событию для соответствующего канала Запись «0» - не влияет"]
pub mod edge;
#[doc = "LEVEL (rw) register accessor: Регистр типа прерываний. При чтении – текущий тип прерывания (инвертированный): «0» – по событию; «1» – по уровню. Запись «1» – прерывание формируется по уровню для соответствующего канала Запись «0» - не влияет\n\nYou can [`read`](crate::Reg::read) this register and get [`level::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`level::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@level`]
module"]
#[doc(alias = "LEVEL")]
pub type Level = crate::Reg<level::LevelSpec>;
#[doc = "Регистр типа прерываний. При чтении – текущий тип прерывания (инвертированный): «0» – по событию; «1» – по уровню. Запись «1» – прерывание формируется по уровню для соответствующего канала Запись «0» - не влияет"]
pub mod level;
#[doc = "LEVEL_SET (rw) register accessor: Регистр выбора уровня / события прерываний. При чтении: «0» – прерывания формируются по спаду или уровню логического «0»; «1» – прерывания формируются по нарастающему фронту или уровню логической «1» Запись «1» – прерывание формируется по нарастающему фронту или уровню логической «1» для соответствующего канала\n\nYou can [`read`](crate::Reg::read) this register and get [`level_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`level_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@level_set`]
module"]
#[doc(alias = "LEVEL_SET")]
pub type LevelSet = crate::Reg<level_set::LevelSetSpec>;
#[doc = "Регистр выбора уровня / события прерываний. При чтении: «0» – прерывания формируются по спаду или уровню логического «0»; «1» – прерывания формируются по нарастающему фронту или уровню логической «1» Запись «1» – прерывание формируется по нарастающему фронту или уровню логической «1» для соответствующего канала"]
pub mod level_set;
#[doc = "LEVEL_CLEAR (rw) register accessor: Регистр уровня / со-бытия прерываний. При чтении: «0» – прерывания формируются по спаду или уровню логического «0»; «1» – прерывания формируются по нарастающему фронту или уровню логической «1» Запись «1»– прерывание формируется по спаду или уровню логического «0» для соответствующего канала\n\nYou can [`read`](crate::Reg::read) this register and get [`level_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`level_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@level_clear`]
module"]
#[doc(alias = "LEVEL_CLEAR")]
pub type LevelClear = crate::Reg<level_clear::LevelClearSpec>;
#[doc = "Регистр уровня / со-бытия прерываний. При чтении: «0» – прерывания формируются по спаду или уровню логического «0»; «1» – прерывания формируются по нарастающему фронту или уровню логической «1» Запись «1»– прерывание формируется по спаду или уровню логического «0» для соответствующего канала"]
pub mod level_clear;
#[doc = "ANY_EDGE_SET (rw) register accessor: Регистр прерываний по любому событию. Запись «1» – прерывание формируется по любому изменению соответствующего канала\n\nYou can [`read`](crate::Reg::read) this register and get [`any_edge_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`any_edge_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@any_edge_set`]
module"]
#[doc(alias = "ANY_EDGE_SET")]
pub type AnyEdgeSet = crate::Reg<any_edge_set::AnyEdgeSetSpec>;
#[doc = "Регистр прерываний по любому событию. Запись «1» – прерывание формируется по любому изменению соответствующего канала"]
pub mod any_edge_set;
#[doc = "ANY_EDGE_CLEAR (rw) register accessor: Регистр прерываний по любому событию. Запись «1» – прерывание не формируется по любому изменению соответствующего канала\n\nYou can [`read`](crate::Reg::read) this register and get [`any_edge_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`any_edge_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@any_edge_clear`]
module"]
#[doc(alias = "ANY_EDGE_CLEAR")]
pub type AnyEdgeClear = crate::Reg<any_edge_clear::AnyEdgeClearSpec>;
#[doc = "Регистр прерываний по любому событию. Запись «1» – прерывание не формируется по любому изменению соответствующего канала"]
pub mod any_edge_clear;
#[doc = "CLEAR (w) register accessor: Регистр сброса флагов прерываний по событию. Запись «1» – очищает флаг прерывания соответствующего канала\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear`]
module"]
#[doc(alias = "CLEAR")]
pub type Clear = crate::Reg<clear::ClearSpec>;
#[doc = "Регистр сброса флагов прерываний по событию. Запись «1» – очищает флаг прерывания соответствующего канала"]
pub mod clear;
