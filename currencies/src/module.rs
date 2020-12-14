use common_structure::digital_currency::DigitalCurrencyWrapper;

#[derive(Serialize, Deserialize, Clone)]
pub struct UnlockCurrency{
    pub currency: DigitalCurrencyWrapper,
}

