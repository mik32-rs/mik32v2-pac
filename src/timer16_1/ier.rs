#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "запрет/разрешение прерывания CMPM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpmie {
    #[doc = "0: Прерывание запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание разрешено"]
    Enable = 1,
}
impl From<Cmpmie> for bool {
    #[inline(always)]
    fn from(variant: Cmpmie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPMIE` reader - запрет/разрешение прерывания CMPM"]
pub type CmpmieR = crate::BitReader<Cmpmie>;
impl CmpmieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpmie {
        match self.bits {
            false => Cmpmie::Disable,
            true => Cmpmie::Enable,
        }
    }
    #[doc = "Прерывание запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmpmie::Disable
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cmpmie::Enable
    }
}
#[doc = "Field `CMPMIE` writer - запрет/разрешение прерывания CMPM"]
pub type CmpmieW<'a, REG> = crate::BitWriter<'a, REG, Cmpmie>;
impl<'a, REG> CmpmieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmie::Disable)
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmie::Enable)
    }
}
#[doc = "запрет/разрешение прерывания EXTTRIG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exttrigie {
    #[doc = "0: Прерывание запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание разрешено"]
    Enable = 1,
}
impl From<Exttrigie> for bool {
    #[inline(always)]
    fn from(variant: Exttrigie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTTRIGIE` reader - запрет/разрешение прерывания EXTTRIG"]
pub type ExttrigieR = crate::BitReader<Exttrigie>;
impl ExttrigieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exttrigie {
        match self.bits {
            false => Exttrigie::Disable,
            true => Exttrigie::Enable,
        }
    }
    #[doc = "Прерывание запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Exttrigie::Disable
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Exttrigie::Enable
    }
}
#[doc = "Field `EXTTRIGIE` writer - запрет/разрешение прерывания EXTTRIG"]
pub type ExttrigieW<'a, REG> = crate::BitWriter<'a, REG, Exttrigie>;
impl<'a, REG> ExttrigieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Exttrigie::Disable)
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Exttrigie::Enable)
    }
}
#[doc = "запрет/разрешение прерывания CMPOK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpokie {
    #[doc = "0: Прерывание запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание разрешено"]
    Enable = 1,
}
impl From<Cmpokie> for bool {
    #[inline(always)]
    fn from(variant: Cmpokie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPOKIE` reader - запрет/разрешение прерывания CMPOK"]
pub type CmpokieR = crate::BitReader<Cmpokie>;
impl CmpokieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpokie {
        match self.bits {
            false => Cmpokie::Disable,
            true => Cmpokie::Enable,
        }
    }
    #[doc = "Прерывание запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmpokie::Disable
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cmpokie::Enable
    }
}
#[doc = "Field `CMPOKIE` writer - запрет/разрешение прерывания CMPOK"]
pub type CmpokieW<'a, REG> = crate::BitWriter<'a, REG, Cmpokie>;
impl<'a, REG> CmpokieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpokie::Disable)
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpokie::Enable)
    }
}
#[doc = "запрет/разрешение прерывания ARRROK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arrokie {
    #[doc = "0: Прерывание запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание разрешено"]
    Enable = 1,
}
impl From<Arrokie> for bool {
    #[inline(always)]
    fn from(variant: Arrokie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARROKIE` reader - запрет/разрешение прерывания ARRROK"]
pub type ArrokieR = crate::BitReader<Arrokie>;
impl ArrokieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Arrokie {
        match self.bits {
            false => Arrokie::Disable,
            true => Arrokie::Enable,
        }
    }
    #[doc = "Прерывание запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Arrokie::Disable
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Arrokie::Enable
    }
}
#[doc = "Field `ARROKIE` writer - запрет/разрешение прерывания ARRROK"]
pub type ArrokieW<'a, REG> = crate::BitWriter<'a, REG, Arrokie>;
impl<'a, REG> ArrokieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Arrokie::Disable)
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Arrokie::Enable)
    }
}
#[doc = "запрет/разрешение прерывания UP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Upie {
    #[doc = "0: Прерывание запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание разрешено"]
    Enable = 1,
}
impl From<Upie> for bool {
    #[inline(always)]
    fn from(variant: Upie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UPIE` reader - запрет/разрешение прерывания UP"]
pub type UpieR = crate::BitReader<Upie>;
impl UpieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Upie {
        match self.bits {
            false => Upie::Disable,
            true => Upie::Enable,
        }
    }
    #[doc = "Прерывание запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Upie::Disable
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Upie::Enable
    }
}
#[doc = "Field `UPIE` writer - запрет/разрешение прерывания UP"]
pub type UpieW<'a, REG> = crate::BitWriter<'a, REG, Upie>;
impl<'a, REG> UpieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Upie::Disable)
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Upie::Enable)
    }
}
#[doc = "запрет/разрешение прерывания ARRM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arrmie {
    #[doc = "0: Прерывание запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание разрешено"]
    Enable = 1,
}
impl From<Arrmie> for bool {
    #[inline(always)]
    fn from(variant: Arrmie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARRMIE` reader - запрет/разрешение прерывания ARRM"]
pub type ArrmieR = crate::BitReader<Arrmie>;
impl ArrmieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Arrmie {
        match self.bits {
            false => Arrmie::Disable,
            true => Arrmie::Enable,
        }
    }
    #[doc = "Прерывание запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Arrmie::Disable
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Arrmie::Enable
    }
}
#[doc = "Field `ARRMIE` writer - запрет/разрешение прерывания ARRM"]
pub type ArrmieW<'a, REG> = crate::BitWriter<'a, REG, Arrmie>;
impl<'a, REG> ArrmieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Arrmie::Disable)
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Arrmie::Enable)
    }
}
#[doc = "запрет/разрешение прерывания DOWN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Downie {
    #[doc = "0: Прерывание запрешено"]
    Disable = 0,
    #[doc = "1: Прерывание разрешено"]
    Enable = 1,
}
impl From<Downie> for bool {
    #[inline(always)]
    fn from(variant: Downie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOWNIE` reader - запрет/разрешение прерывания DOWN"]
pub type DownieR = crate::BitReader<Downie>;
impl DownieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Downie {
        match self.bits {
            false => Downie::Disable,
            true => Downie::Enable,
        }
    }
    #[doc = "Прерывание запрешено"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Downie::Disable
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Downie::Enable
    }
}
#[doc = "Field `DOWNIE` writer - запрет/разрешение прерывания DOWN"]
pub type DownieW<'a, REG> = crate::BitWriter<'a, REG, Downie>;
impl<'a, REG> DownieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Прерывание запрешено"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Downie::Disable)
    }
    #[doc = "Прерывание разрешено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Downie::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - запрет/разрешение прерывания CMPM"]
    #[inline(always)]
    pub fn cmpmie(&self) -> CmpmieR {
        CmpmieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - запрет/разрешение прерывания EXTTRIG"]
    #[inline(always)]
    pub fn exttrigie(&self) -> ExttrigieR {
        ExttrigieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - запрет/разрешение прерывания CMPOK"]
    #[inline(always)]
    pub fn cmpokie(&self) -> CmpokieR {
        CmpokieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - запрет/разрешение прерывания ARRROK"]
    #[inline(always)]
    pub fn arrokie(&self) -> ArrokieR {
        ArrokieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - запрет/разрешение прерывания UP"]
    #[inline(always)]
    pub fn upie(&self) -> UpieR {
        UpieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 5 - запрет/разрешение прерывания ARRM"]
    #[inline(always)]
    pub fn arrmie(&self) -> ArrmieR {
        ArrmieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - запрет/разрешение прерывания DOWN"]
    #[inline(always)]
    pub fn downie(&self) -> DownieR {
        DownieR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - запрет/разрешение прерывания CMPM"]
    #[inline(always)]
    pub fn cmpmie(&mut self) -> CmpmieW<IerSpec> {
        CmpmieW::new(self, 0)
    }
    #[doc = "Bit 2 - запрет/разрешение прерывания EXTTRIG"]
    #[inline(always)]
    pub fn exttrigie(&mut self) -> ExttrigieW<IerSpec> {
        ExttrigieW::new(self, 2)
    }
    #[doc = "Bit 3 - запрет/разрешение прерывания CMPOK"]
    #[inline(always)]
    pub fn cmpokie(&mut self) -> CmpokieW<IerSpec> {
        CmpokieW::new(self, 3)
    }
    #[doc = "Bit 4 - запрет/разрешение прерывания ARRROK"]
    #[inline(always)]
    pub fn arrokie(&mut self) -> ArrokieW<IerSpec> {
        ArrokieW::new(self, 4)
    }
    #[doc = "Bit 5 - запрет/разрешение прерывания UP"]
    #[inline(always)]
    pub fn upie(&mut self) -> UpieW<IerSpec> {
        UpieW::new(self, 5)
    }
    #[doc = "Bit 5 - запрет/разрешение прерывания ARRM"]
    #[inline(always)]
    pub fn arrmie(&mut self) -> ArrmieW<IerSpec> {
        ArrmieW::new(self, 5)
    }
    #[doc = "Bit 6 - запрет/разрешение прерывания DOWN"]
    #[inline(always)]
    pub fn downie(&mut self) -> DownieW<IerSpec> {
        DownieW::new(self, 6)
    }
}
#[doc = "Регистр разрешения прерываний\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {
    const RESET_VALUE: u32 = 0;
}
