#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    adc_config: AdcConfig,
    adc_continuous: AdcContinuous,
    adc_single: AdcSingle,
    adc_valid: AdcValid,
    adc_value: AdcValue,
}
impl RegisterBlock {
    #[doc = "0x00 - Регистр настойки"]
    #[inline(always)]
    pub const fn adc_config(&self) -> &AdcConfig {
        &self.adc_config
    }
    #[doc = "0x04 - Управление непрерывным измерением"]
    #[inline(always)]
    pub const fn adc_continuous(&self) -> &AdcContinuous {
        &self.adc_continuous
    }
    #[doc = "0x08 - Управление однократным измерением"]
    #[inline(always)]
    pub const fn adc_single(&self) -> &AdcSingle {
        &self.adc_single
    }
    #[doc = "0x0c - Регистр статуса"]
    #[inline(always)]
    pub const fn adc_valid(&self) -> &AdcValid {
        &self.adc_valid
    }
    #[doc = "0x10 - Регистр данных"]
    #[inline(always)]
    pub const fn adc_value(&self) -> &AdcValue {
        &self.adc_value
    }
}
#[doc = "ADC_CONFIG (rw) register accessor: Регистр настойки\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_config`]
module"]
#[doc(alias = "ADC_CONFIG")]
pub type AdcConfig = crate::Reg<adc_config::AdcConfigSpec>;
#[doc = "Регистр настойки"]
pub mod adc_config;
#[doc = "ADC_CONTINUOUS (rw) register accessor: Управление непрерывным измерением\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_continuous::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_continuous::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_continuous`]
module"]
#[doc(alias = "ADC_CONTINUOUS")]
pub type AdcContinuous = crate::Reg<adc_continuous::AdcContinuousSpec>;
#[doc = "Управление непрерывным измерением"]
pub mod adc_continuous;
#[doc = "ADC_SINGLE (w) register accessor: Управление однократным измерением\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_single::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_single`]
module"]
#[doc(alias = "ADC_SINGLE")]
pub type AdcSingle = crate::Reg<adc_single::AdcSingleSpec>;
#[doc = "Управление однократным измерением"]
pub mod adc_single;
#[doc = "ADC_VALID (r) register accessor: Регистр статуса\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_valid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_valid`]
module"]
#[doc(alias = "ADC_VALID")]
pub type AdcValid = crate::Reg<adc_valid::AdcValidSpec>;
#[doc = "Регистр статуса"]
pub mod adc_valid;
#[doc = "ADC_VALUE (r) register accessor: Регистр данных\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_value::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_value`]
module"]
#[doc(alias = "ADC_VALUE")]
pub type AdcValue = crate::Reg<adc_value::AdcValueSpec>;
#[doc = "Регистр данных"]
pub mod adc_value;
