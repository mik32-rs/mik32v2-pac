#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    block: Block,
    key: Key,
    init: Init,
    config: Config,
}
impl RegisterBlock {
    #[doc = "0x00 - Буфер данных для шифрования"]
    #[inline(always)]
    pub const fn block(&self) -> &Block {
        &self.block
    }
    #[doc = "0x04 - Буфер данных ключа"]
    #[inline(always)]
    pub const fn key(&self) -> &Key {
        &self.key
    }
    #[doc = "0x08 - Буфер данных вектора инициализации"]
    #[inline(always)]
    pub const fn init(&self) -> &Init {
        &self.init
    }
    #[doc = "0x0c - Регистр конфигурации"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
}
#[doc = "BLOCK (rw) register accessor: Буфер данных для шифрования\n\nYou can [`read`](crate::Reg::read) this register and get [`block::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`block::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@block`]
module"]
#[doc(alias = "BLOCK")]
pub type Block = crate::Reg<block::BlockSpec>;
#[doc = "Буфер данных для шифрования"]
pub mod block;
#[doc = "KEY (w) register accessor: Буфер данных ключа\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key`]
module"]
#[doc(alias = "KEY")]
pub type Key = crate::Reg<key::KeySpec>;
#[doc = "Буфер данных ключа"]
pub mod key;
#[doc = "INIT (w) register accessor: Буфер данных вектора инициализации\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@init`]
module"]
#[doc(alias = "INIT")]
pub type Init = crate::Reg<init::InitSpec>;
#[doc = "Буфер данных вектора инициализации"]
pub mod init;
#[doc = "CONFIG (rw) register accessor: Регистр конфигурации\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Регистр конфигурации"]
pub mod config;
