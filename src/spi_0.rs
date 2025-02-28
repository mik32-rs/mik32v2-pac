#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    config: Config,
    status: Status,
    int_enable: IntEnable,
    int_disable: IntDisable,
    int_mask: IntMask,
    enable: Enable,
    delay: Delay,
    txdata: Txdata,
    rxdata: Rxdata,
    sic: Sic,
    tx_thr: TxThr,
    id: Id,
}
impl RegisterBlock {
    #[doc = "0x00 - Регистр конфигурации SPI"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x04 - Регистр статуса. Примечание: биты регистра \\[6:0\\]
устанавливаются в «1», если произошло событие вызывающее прерывание."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x08 - Регистр разрешения прерываний"]
    #[inline(always)]
    pub const fn int_enable(&self) -> &IntEnable {
        &self.int_enable
    }
    #[doc = "0x0c - Регистр запрета прерываний"]
    #[inline(always)]
    pub const fn int_disable(&self) -> &IntDisable {
        &self.int_disable
    }
    #[doc = "0x10 - Регистр текущих масок прерываний"]
    #[inline(always)]
    pub const fn int_mask(&self) -> &IntMask {
        &self.int_mask
    }
    #[doc = "0x14 - Регистр включения/выключения SPI"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x18 - Регистр задержек"]
    #[inline(always)]
    pub const fn delay(&self) -> &Delay {
        &self.delay
    }
    #[doc = "0x1c - Регистр передаваемых данных"]
    #[inline(always)]
    pub const fn txdata(&self) -> &Txdata {
        &self.txdata
    }
    #[doc = "0x20 - Регистр принимаемых данных"]
    #[inline(always)]
    pub const fn rxdata(&self) -> &Rxdata {
        &self.rxdata
    }
    #[doc = "0x24 - Регистр счетчика останова ведомого устройства"]
    #[inline(always)]
    pub const fn sic(&self) -> &Sic {
        &self.sic
    }
    #[doc = "0x28 - Регистр пороговых значений TX_FIFO"]
    #[inline(always)]
    pub const fn tx_thr(&self) -> &TxThr {
        &self.tx_thr
    }
    #[doc = "0x2c - Идентификационный номер модуля"]
    #[inline(always)]
    pub const fn id(&self) -> &Id {
        &self.id
    }
}
#[doc = "CONFIG (rw) register accessor: Регистр конфигурации SPI\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Регистр конфигурации SPI"]
pub mod config;
#[doc = "STATUS (rw) register accessor: Регистр статуса. Примечание: биты регистра \\[6:0\\]
устанавливаются в «1», если произошло событие вызывающее прерывание.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Регистр статуса. Примечание: биты регистра \\[6:0\\]
устанавливаются в «1», если произошло событие вызывающее прерывание."]
pub mod status;
#[doc = "INT_ENABLE (w) register accessor: Регистр разрешения прерываний\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_enable::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_enable`]
module"]
#[doc(alias = "INT_ENABLE")]
pub type IntEnable = crate::Reg<int_enable::IntEnableSpec>;
#[doc = "Регистр разрешения прерываний"]
pub mod int_enable;
#[doc = "INT_DISABLE (w) register accessor: Регистр запрета прерываний\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_disable::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_disable`]
module"]
#[doc(alias = "INT_DISABLE")]
pub type IntDisable = crate::Reg<int_disable::IntDisableSpec>;
#[doc = "Регистр запрета прерываний"]
pub mod int_disable;
#[doc = "INT_MASK (r) register accessor: Регистр текущих масок прерываний\n\nYou can [`read`](crate::Reg::read) this register and get [`int_mask::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_mask`]
module"]
#[doc(alias = "INT_MASK")]
pub type IntMask = crate::Reg<int_mask::IntMaskSpec>;
#[doc = "Регистр текущих масок прерываний"]
pub mod int_mask;
#[doc = "ENABLE (rw) register accessor: Регистр включения/выключения SPI\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Регистр включения/выключения SPI"]
pub mod enable;
#[doc = "DELAY (rw) register accessor: Регистр задержек\n\nYou can [`read`](crate::Reg::read) this register and get [`delay::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`delay::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@delay`]
module"]
#[doc(alias = "DELAY")]
pub type Delay = crate::Reg<delay::DelaySpec>;
#[doc = "Регистр задержек"]
pub mod delay;
#[doc = "TXDATA (w) register accessor: Регистр передаваемых данных\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata`]
module"]
#[doc(alias = "TXDATA")]
pub type Txdata = crate::Reg<txdata::TxdataSpec>;
#[doc = "Регистр передаваемых данных"]
pub mod txdata;
#[doc = "RXDATA (r) register accessor: Регистр принимаемых данных\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata`]
module"]
#[doc(alias = "RXDATA")]
pub type Rxdata = crate::Reg<rxdata::RxdataSpec>;
#[doc = "Регистр принимаемых данных"]
pub mod rxdata;
#[doc = "SIC (rw) register accessor: Регистр счетчика останова ведомого устройства\n\nYou can [`read`](crate::Reg::read) this register and get [`sic::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sic::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sic`]
module"]
#[doc(alias = "SIC")]
pub type Sic = crate::Reg<sic::SicSpec>;
#[doc = "Регистр счетчика останова ведомого устройства"]
pub mod sic;
#[doc = "TX_THR (rw) register accessor: Регистр пороговых значений TX_FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_thr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_thr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_thr`]
module"]
#[doc(alias = "TX_THR")]
pub type TxThr = crate::Reg<tx_thr::TxThrSpec>;
#[doc = "Регистр пороговых значений TX_FIFO"]
pub mod tx_thr;
#[doc = "ID (r) register accessor: Идентификационный номер модуля\n\nYou can [`read`](crate::Reg::read) this register and get [`id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
#[doc(alias = "ID")]
pub type Id = crate::Reg<id::IdSpec>;
#[doc = "Идентификационный номер модуля"]
pub mod id;
