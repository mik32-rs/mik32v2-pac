#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x84],
    con: Con,
    _reserved1: [u8; 0x14],
    _reserved_1_key: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x84 - Конфигурация"]
    #[inline(always)]
    pub const fn con(&self) -> &Con {
        &self.con
    }
    #[doc = "0x9c - Регистр состояния"]
    #[inline(always)]
    pub const fn sta(&self) -> &Sta {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(156).cast() }
    }
    #[doc = "0x9c - Регистр ключа"]
    #[inline(always)]
    pub const fn key(&self) -> &Key {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(156).cast() }
    }
}
#[doc = "CON (rw) register accessor: Конфигурация\n\nYou can [`read`](crate::Reg::read) this register and get [`con::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`con::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@con`]
module"]
#[doc(alias = "CON")]
pub type Con = crate::Reg<con::ConSpec>;
#[doc = "Конфигурация"]
#[path = "wdt/con_.rs"]
pub mod con;
#[doc = "KEY (w) register accessor: Регистр ключа\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key`]
module"]
#[doc(alias = "KEY")]
pub type Key = crate::Reg<key::KeySpec>;
#[doc = "Регистр ключа"]
pub mod key;
#[doc = "STA (r) register accessor: Регистр состояния\n\nYou can [`read`](crate::Reg::read) this register and get [`sta::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sta`]
module"]
#[doc(alias = "STA")]
pub type Sta = crate::Reg<sta::StaSpec>;
#[doc = "Регистр состояния"]
pub mod sta;
