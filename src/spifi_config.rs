#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    cmd: Cmd,
    address: Address,
    idata: Idata,
    climit: Climit,
    data: Data,
    mcmd: Mcmd,
    stat: Stat,
}
impl RegisterBlock {
    #[doc = "0x00 - SPIFI регистр управления"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - SPIFI регистр команд. Этот регистр может быть записан только тогда, когда биты CMD и MCINIT равны нулю"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x08 - SPIFI регистр адреса. Данный регистр должен быть настроен перед инициализацией любой команды. При выполнении команды первым выводится старший бит адреса"]
    #[inline(always)]
    pub const fn address(&self) -> &Address {
        &self.address
    }
    #[doc = "0x0c - SPIFI регистр промежуточных данных. Данный регистр должен быть настроен перед инициализацией команды, в которой используются промежуточные данные. При выполнении команды первым выводится младший бит слова"]
    #[inline(always)]
    pub const fn idata(&self) -> &Idata {
        &self.idata
    }
    #[doc = "0x10 - SPIFI регистр верхней границы адреса кеширования"]
    #[inline(always)]
    pub const fn climit(&self) -> &Climit {
        &self.climit
    }
    #[doc = "0x14 - SPIFI регистр данных. Если выходной буфер чтения пуст или входной буфер записи полон, то при отправке запроса по шине AHB будет вызвано исключение (код 5 “Load access fault”)."]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    #[doc = "0x18 - SPIFI регистр команд памяти"]
    #[inline(always)]
    pub const fn mcmd(&self) -> &Mcmd {
        &self.mcmd
    }
    #[doc = "0x1c - SPIFI регистр статуса"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
}
#[doc = "CTRL (rw) register accessor: SPIFI регистр управления\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "SPIFI регистр управления"]
pub mod ctrl;
#[doc = "CMD (rw) register accessor: SPIFI регистр команд. Этот регистр может быть записан только тогда, когда биты CMD и MCINIT равны нулю\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "SPIFI регистр команд. Этот регистр может быть записан только тогда, когда биты CMD и MCINIT равны нулю"]
pub mod cmd;
#[doc = "ADDRESS (rw) register accessor: SPIFI регистр адреса. Данный регистр должен быть настроен перед инициализацией любой команды. При выполнении команды первым выводится старший бит адреса\n\nYou can [`read`](crate::Reg::read) this register and get [`address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@address`]
module"]
#[doc(alias = "ADDRESS")]
pub type Address = crate::Reg<address::AddressSpec>;
#[doc = "SPIFI регистр адреса. Данный регистр должен быть настроен перед инициализацией любой команды. При выполнении команды первым выводится старший бит адреса"]
pub mod address;
#[doc = "IDATA (rw) register accessor: SPIFI регистр промежуточных данных. Данный регистр должен быть настроен перед инициализацией команды, в которой используются промежуточные данные. При выполнении команды первым выводится младший бит слова\n\nYou can [`read`](crate::Reg::read) this register and get [`idata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idata`]
module"]
#[doc(alias = "IDATA")]
pub type Idata = crate::Reg<idata::IdataSpec>;
#[doc = "SPIFI регистр промежуточных данных. Данный регистр должен быть настроен перед инициализацией команды, в которой используются промежуточные данные. При выполнении команды первым выводится младший бит слова"]
pub mod idata;
#[doc = "CLIMIT (rw) register accessor: SPIFI регистр верхней границы адреса кеширования\n\nYou can [`read`](crate::Reg::read) this register and get [`climit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`climit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@climit`]
module"]
#[doc(alias = "CLIMIT")]
pub type Climit = crate::Reg<climit::ClimitSpec>;
#[doc = "SPIFI регистр верхней границы адреса кеширования"]
pub mod climit;
#[doc = "DATA (rw) register accessor: SPIFI регистр данных. Если выходной буфер чтения пуст или входной буфер записи полон, то при отправке запроса по шине AHB будет вызвано исключение (код 5 “Load access fault”).\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "SPIFI регистр данных. Если выходной буфер чтения пуст или входной буфер записи полон, то при отправке запроса по шине AHB будет вызвано исключение (код 5 “Load access fault”)."]
pub mod data;
#[doc = "MCMD (rw) register accessor: SPIFI регистр команд памяти\n\nYou can [`read`](crate::Reg::read) this register and get [`mcmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcmd`]
module"]
#[doc(alias = "MCMD")]
pub type Mcmd = crate::Reg<mcmd::McmdSpec>;
#[doc = "SPIFI регистр команд памяти"]
pub mod mcmd;
#[doc = "STAT (rw) register accessor: SPIFI регистр статуса\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "SPIFI регистр статуса"]
pub mod stat;
