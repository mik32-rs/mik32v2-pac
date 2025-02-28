#[doc = "Register `CLK_AHB_CLEAR` reader"]
pub type R = crate::R<ClkAhbClearSpec>;
#[doc = "Register `CLK_AHB_CLEAR` writer"]
pub type W = crate::W<ClkAhbClearSpec>;
#[doc = "Ядро\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Core {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<Core> for bool {
    #[inline(always)]
    fn from(variant: Core) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Core` reader - Ядро"]
pub type CoreR = crate::BitReader<Core>;
impl CoreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Core {
        match self.bits {
            false => Core::Disable,
            true => Core::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Core::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Core::Enable
    }
}
#[doc = "Ядро\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoreWO {
    #[doc = "1: Тактирование выключено"]
    Disable = 1,
    #[doc = "0: Тактирование включено"]
    Enable = 0,
}
impl From<CoreWO> for bool {
    #[inline(always)]
    fn from(variant: CoreWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Core` writer - Ядро"]
pub type CoreW<'a, REG> = crate::BitWriter<'a, REG, CoreWO>;
impl<'a, REG> CoreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CoreWO::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CoreWO::Enable)
    }
}
#[doc = "EEPROM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eeprom {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<Eeprom> for bool {
    #[inline(always)]
    fn from(variant: Eeprom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEPROM` reader - EEPROM"]
pub type EepromR = crate::BitReader<Eeprom>;
impl EepromR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eeprom {
        match self.bits {
            false => Eeprom::Disable,
            true => Eeprom::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Eeprom::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Eeprom::Enable
    }
}
#[doc = "EEPROM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EepromWO {
    #[doc = "1: Тактирование выключено"]
    Disable = 1,
    #[doc = "0: Тактирование включено"]
    Enable = 0,
}
impl From<EepromWO> for bool {
    #[inline(always)]
    fn from(variant: EepromWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EEPROM` writer - EEPROM"]
pub type EepromW<'a, REG> = crate::BitWriter<'a, REG, EepromWO>;
impl<'a, REG> EepromW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EepromWO::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EepromWO::Enable)
    }
}
#[doc = "RAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ram {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<Ram> for bool {
    #[inline(always)]
    fn from(variant: Ram) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM` reader - RAM"]
pub type RamR = crate::BitReader<Ram>;
impl RamR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ram {
        match self.bits {
            false => Ram::Disable,
            true => Ram::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ram::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ram::Enable
    }
}
#[doc = "RAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RamWO {
    #[doc = "1: Тактирование выключено"]
    Disable = 1,
    #[doc = "0: Тактирование включено"]
    Enable = 0,
}
impl From<RamWO> for bool {
    #[inline(always)]
    fn from(variant: RamWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM` writer - RAM"]
pub type RamW<'a, REG> = crate::BitWriter<'a, REG, RamWO>;
impl<'a, REG> RamW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(RamWO::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(RamWO::Enable)
    }
}
#[doc = "SPIFI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spifi {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<Spifi> for bool {
    #[inline(always)]
    fn from(variant: Spifi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIFI` reader - SPIFI"]
pub type SpifiR = crate::BitReader<Spifi>;
impl SpifiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spifi {
        match self.bits {
            false => Spifi::Disable,
            true => Spifi::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Spifi::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Spifi::Enable
    }
}
#[doc = "SPIFI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpifiWO {
    #[doc = "1: Тактирование выключено"]
    Disable = 1,
    #[doc = "0: Тактирование включено"]
    Enable = 0,
}
impl From<SpifiWO> for bool {
    #[inline(always)]
    fn from(variant: SpifiWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIFI` writer - SPIFI"]
pub type SpifiW<'a, REG> = crate::BitWriter<'a, REG, SpifiWO>;
impl<'a, REG> SpifiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SpifiWO::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SpifiWO::Enable)
    }
}
#[doc = "TCB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcb {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<Tcb> for bool {
    #[inline(always)]
    fn from(variant: Tcb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCB` reader - TCB"]
pub type TcbR = crate::BitReader<Tcb>;
impl TcbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcb {
        match self.bits {
            false => Tcb::Disable,
            true => Tcb::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tcb::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tcb::Enable
    }
}
#[doc = "TCB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TcbWO {
    #[doc = "1: Тактирование выключено"]
    Disable = 1,
    #[doc = "0: Тактирование включено"]
    Enable = 0,
}
impl From<TcbWO> for bool {
    #[inline(always)]
    fn from(variant: TcbWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCB` writer - TCB"]
pub type TcbW<'a, REG> = crate::BitWriter<'a, REG, TcbWO>;
impl<'a, REG> TcbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(TcbWO::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(TcbWO::Enable)
    }
}
#[doc = "DMA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<Dma> for bool {
    #[inline(always)]
    fn from(variant: Dma) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA` reader - DMA"]
pub type DmaR = crate::BitReader<Dma>;
impl DmaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma {
        match self.bits {
            false => Dma::Disable,
            true => Dma::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dma::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dma::Enable
    }
}
#[doc = "DMA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaWO {
    #[doc = "1: Тактирование выключено"]
    Disable = 1,
    #[doc = "0: Тактирование включено"]
    Enable = 0,
}
impl From<DmaWO> for bool {
    #[inline(always)]
    fn from(variant: DmaWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA` writer - DMA"]
pub type DmaW<'a, REG> = crate::BitWriter<'a, REG, DmaWO>;
impl<'a, REG> DmaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmaWO::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmaWO::Enable)
    }
}
#[doc = "Crypto\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crypto {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<Crypto> for bool {
    #[inline(always)]
    fn from(variant: Crypto) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Crypto` reader - Crypto"]
pub type CryptoR = crate::BitReader<Crypto>;
impl CryptoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crypto {
        match self.bits {
            false => Crypto::Disable,
            true => Crypto::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Crypto::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Crypto::Enable
    }
}
#[doc = "Crypto\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CryptoWO {
    #[doc = "1: Тактирование выключено"]
    Disable = 1,
    #[doc = "0: Тактирование включено"]
    Enable = 0,
}
impl From<CryptoWO> for bool {
    #[inline(always)]
    fn from(variant: CryptoWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Crypto` writer - Crypto"]
pub type CryptoW<'a, REG> = crate::BitWriter<'a, REG, CryptoWO>;
impl<'a, REG> CryptoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CryptoWO::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CryptoWO::Enable)
    }
}
#[doc = "CRC32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crc32 {
    #[doc = "0: Тактирование выключено"]
    Disable = 0,
    #[doc = "1: Тактирование включено"]
    Enable = 1,
}
impl From<Crc32> for bool {
    #[inline(always)]
    fn from(variant: Crc32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRC32` reader - CRC32"]
pub type Crc32R = crate::BitReader<Crc32>;
impl Crc32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crc32 {
        match self.bits {
            false => Crc32::Disable,
            true => Crc32::Enable,
        }
    }
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Crc32::Disable
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Crc32::Enable
    }
}
#[doc = "CRC32\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crc32WO {
    #[doc = "1: Тактирование выключено"]
    Disable = 1,
    #[doc = "0: Тактирование включено"]
    Enable = 0,
}
impl From<Crc32WO> for bool {
    #[inline(always)]
    fn from(variant: Crc32WO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRC32` writer - CRC32"]
pub type Crc32W<'a, REG> = crate::BitWriter<'a, REG, Crc32WO>;
impl<'a, REG> Crc32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Тактирование выключено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Crc32WO::Disable)
    }
    #[doc = "Тактирование включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Crc32WO::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Ядро"]
    #[inline(always)]
    pub fn core(&self) -> CoreR {
        CoreR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EEPROM"]
    #[inline(always)]
    pub fn eeprom(&self) -> EepromR {
        EepromR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RAM"]
    #[inline(always)]
    pub fn ram(&self) -> RamR {
        RamR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SPIFI"]
    #[inline(always)]
    pub fn spifi(&self) -> SpifiR {
        SpifiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TCB"]
    #[inline(always)]
    pub fn tcb(&self) -> TcbR {
        TcbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Crypto"]
    #[inline(always)]
    pub fn crypto(&self) -> CryptoR {
        CryptoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CRC32"]
    #[inline(always)]
    pub fn crc32(&self) -> Crc32R {
        Crc32R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ядро"]
    #[inline(always)]
    pub fn core(&mut self) -> CoreW<ClkAhbClearSpec> {
        CoreW::new(self, 0)
    }
    #[doc = "Bit 1 - EEPROM"]
    #[inline(always)]
    pub fn eeprom(&mut self) -> EepromW<ClkAhbClearSpec> {
        EepromW::new(self, 1)
    }
    #[doc = "Bit 2 - RAM"]
    #[inline(always)]
    pub fn ram(&mut self) -> RamW<ClkAhbClearSpec> {
        RamW::new(self, 2)
    }
    #[doc = "Bit 3 - SPIFI"]
    #[inline(always)]
    pub fn spifi(&mut self) -> SpifiW<ClkAhbClearSpec> {
        SpifiW::new(self, 3)
    }
    #[doc = "Bit 4 - TCB"]
    #[inline(always)]
    pub fn tcb(&mut self) -> TcbW<ClkAhbClearSpec> {
        TcbW::new(self, 4)
    }
    #[doc = "Bit 5 - DMA"]
    #[inline(always)]
    pub fn dma(&mut self) -> DmaW<ClkAhbClearSpec> {
        DmaW::new(self, 5)
    }
    #[doc = "Bit 6 - Crypto"]
    #[inline(always)]
    pub fn crypto(&mut self) -> CryptoW<ClkAhbClearSpec> {
        CryptoW::new(self, 6)
    }
    #[doc = "Bit 7 - CRC32"]
    #[inline(always)]
    pub fn crc32(&mut self) -> Crc32W<ClkAhbClearSpec> {
        Crc32W::new(self, 7)
    }
}
#[doc = "Регистр выключения тактированием устройств на шине AHB. Каждому биту соответствует устройство, аналогично CLK_AHB_SET\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_ahb_clear::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_ahb_clear::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkAhbClearSpec;
impl crate::RegisterSpec for ClkAhbClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_ahb_clear::R`](R) reader structure"]
impl crate::Readable for ClkAhbClearSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_ahb_clear::W`](W) writer structure"]
impl crate::Writable for ClkAhbClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_AHB_CLEAR to value 0"]
impl crate::Resettable for ClkAhbClearSpec {
    const RESET_VALUE: u32 = 0;
}
