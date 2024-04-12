use cosmwasm_schema::cw_serde;
use vaultenator::config::Configure;

#[cw_serde]
pub struct Config;

impl Configure for Config {
    const CONFIG_KEY: &'static str = "config";

    fn init_config<M>(
        deps: &mut cosmwasm_std::DepsMut,
        msg: &M,
    ) -> Result<Self, vaultenator::errors::ContractError>
    where
        M: serde::Serialize + serde::de::DeserializeOwned,
        Self: Sized,
    {
        todo!()
    }

    fn update_strategy_denom(&mut self, denom: String) {
        todo!()
    }
}
