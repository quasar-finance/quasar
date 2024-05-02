use crate::{
    error::ContractResult, helpers::sort_tokens, msg::ExecuteMsg, state::POSITION, ContractError,
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    coin, to_json_binary, Attribute, BankMsg, Coin, CosmosMsg, Decimal, Deps, Env, Fraction,
    Response, StdError, SubMsg,
};
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin as OsmoCoin,
    osmosis::concentratedliquidity::v1beta1::{MsgCollectIncentives, MsgCollectSpreadRewards},
};

/// prepends a callback to the contract to claim any rewards
pub fn prepend_claim_msg(env: &Env, response: Response) -> Result<Response, ContractError> {
    let claim_msg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
        contract_addr: env.contract.address.to_string(),
        msg: to_json_binary(&ExecuteMsg::VaultExtension(
            crate::msg::ExtensionExecuteMsg::CollectRewards {},
        ))?,
        funds: vec![],
    });

    Ok(prepend_msg(response, SubMsg::new(claim_msg)))
}

/// Prepend a msg to the start of the messages in a response
pub fn prepend_msg(mut response: Response, msg: SubMsg) -> Response {
    response.messages.splice(0..0, vec![msg.into()]);
    response
}

pub fn get_collect_incentives_msg(
    deps: Deps,
    env: Env,
) -> Result<MsgCollectIncentives, ContractError> {
    let position = POSITION.load(deps.storage)?;
    Ok(MsgCollectIncentives {
        position_ids: vec![position.position_id],
        sender: env.contract.address.into(),
    })
}

pub fn get_collect_spread_rewards_msg(
    deps: Deps,
    env: Env,
) -> Result<MsgCollectSpreadRewards, ContractError> {
    let position = POSITION.load(deps.storage)?;
    Ok(MsgCollectSpreadRewards {
        position_ids: vec![position.position_id],
        sender: env.contract.address.into(),
    })
}

/// COIN LIST

#[cw_serde]
#[derive(Default)]
pub struct CoinList(Vec<Coin>);

impl CoinList {
    pub fn new() -> CoinList {
        CoinList::default()
    }

    pub fn osmo_coin_from_coin_list(&self) -> Vec<OsmoCoin> {
        let mut temp_coins = vec![];
        for coin in self.coins() {
            temp_coins.push(OsmoCoin {
                amount: coin.amount.to_string(),
                denom: coin.denom,
            })
        }
        temp_coins
    }

    /// calculates the ratio of the current rewards
    pub fn mul_ratio(&self, ratio: Decimal) -> CoinList {
        CoinList(
            self.0
                .iter()
                .map(|c| {
                    coin(
                        c.amount
                            .multiply_ratio(ratio.numerator(), ratio.denominator())
                            .u128(),
                        c.denom.clone(),
                    )
                })
                .collect(),
        )
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// merge any values already in Rewards and append any others
    pub fn update_rewards(&mut self, rewards: &[OsmoCoin]) -> ContractResult<()> {
        let parsed_rewards: ContractResult<Vec<Coin>> = rewards
            .iter()
            .map(|c| Ok(coin(c.amount.parse()?, c.denom.clone())))
            .collect();

        // Append and merge to
        self.merge(parsed_rewards?)?;
        Ok(())
    }

    // TODO: Cant we get Coins from a coinlist and use above function?
    pub fn update_rewards_coin_list(&mut self, rewards: CoinList) -> ContractResult<()> {
        let parsed_rewards: ContractResult<Vec<Coin>> = rewards
            .coins()
            .into_iter()
            .map(|c| Ok(coin(c.amount.u128(), c.denom)))
            .collect();

        // Append and merge to
        self.merge(parsed_rewards?)?;
        Ok(())
    }

    /// add rewards to self and mutate self
    pub fn add(&mut self, rewards: CoinList) -> ContractResult<()> {
        self.merge(rewards.coins())?;
        Ok(())
    }

    pub fn merge(&mut self, coins: Vec<Coin>) -> ContractResult<()> {
        for c in coins {
            let same = self.0.iter_mut().find(|c2| c.denom == c2.denom);
            if let Some(c2) = same {
                c2.amount = c.amount + c2.amount
            } else {
                self.0.push(c)
            }
        }
        Ok(())
    }

    /// substract a percentage from self, mutate self and return the subtracted rewards
    pub fn sub_ratio(&mut self, ratio: Decimal) -> ContractResult<CoinList> {
        let to_sub = self.mul_ratio(ratio);

        // actually subtract the funds
        self.sub(&to_sub)?;
        Ok(to_sub)
    }

    /// subtract to_sub from self, ignores any coins in to_sub that don't exist in self and vice versa
    /// every item in self is expected to be greater or equal to the amount of the coin with the same denom
    /// in to_sub
    pub fn sub(&mut self, to_sub: &CoinList) -> ContractResult<()> {
        to_sub
            .0
            .iter()
            .try_for_each(|sub_coin| -> ContractResult<()> {
                let coin = self.0.iter_mut().find(|coin| sub_coin.denom == coin.denom);
                if let Some(c) = coin {
                    c.amount = c.amount.checked_sub(sub_coin.amount)?;
                }
                Ok(())
            })
    }

    pub fn claim(&mut self, recipient: &str) -> ContractResult<CosmosMsg> {
        let rewards = sort_tokens(self.coins());
        self.0.clear();

        Ok(BankMsg::Send {
            to_address: recipient.into(),
            amount: rewards,
        }
        .into())
    }

    pub fn into_attributes(self) -> Vec<Attribute> {
        self.0
            .iter()
            .map(|c| Attribute {
                key: c.denom.clone(),
                value: c.amount.to_string(),
            })
            .collect()
    }

    pub fn coins(&self) -> Vec<Coin> {
        sort_tokens(self.0.clone())
    }

    pub fn from_coins(coins: Vec<Coin>) -> Self {
        CoinList(coins)
    }

    pub fn find_coin(&self, denom: String) -> Coin {
        self.0
            .clone()
            .into_iter()
            .find(|c| c.denom == denom)
            .unwrap_or(Coin {
                denom,
                amount: 0u128.into(),
            })
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Uint128};

    use super::*;

    #[test]
    fn test_prepend_msg_with_empty_response() {
        let response = Response::default();
        let msg = CosmosMsg::Bank(BankMsg::Burn { amount: vec![coin(100, "stake")] });

        let updated_response = prepend_msg(response, SubMsg::new(msg.clone()));
        assert_eq!(updated_response.messages.len(), 1);
        assert_eq!(updated_response.messages[0].msg, msg);
    }

    #[test]
    fn test_prepend_msg_with_non_empty_response() {
        let existing_msg = CosmosMsg::Bank(BankMsg::Send { to_address: "bob".to_string(), amount: vec![coin(100, "stake")] });
        let new_msg = CosmosMsg::Bank(BankMsg::Burn { amount: vec![coin(100, "stake")] });

        let response = Response::new().add_message(existing_msg.clone());

        let updated_response = prepend_msg(response.clone(), SubMsg::new(new_msg.clone()));
        assert_eq!(updated_response.messages.len(), 2);
        assert_eq!(updated_response.messages[0].msg, new_msg);
        assert_eq!(updated_response.messages[1].msg, existing_msg);
    }

    #[test]
    fn test_prepend_claim_msg_normal_operation() {
        let env = mock_env();
        let msg = CosmosMsg::Bank(BankMsg::Burn { amount: vec![coin(100, "stake")] });
        let response = Response::new().add_message(msg.clone());

        let claim_msg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: env.contract.address.to_string(),
            msg: to_json_binary(&ExecuteMsg::VaultExtension(
                crate::msg::ExtensionExecuteMsg::CollectRewards {},
            )).unwrap(),
            funds: vec![],
        });
    

        let updated_response = prepend_claim_msg(&env, response).unwrap();
        assert_eq!(updated_response.messages.len(), 2);
        assert_eq!(updated_response.messages[0].msg, claim_msg);
        assert_eq!(updated_response.messages[1].msg, msg);
    }

    #[test]
    fn sub_works() {
        let mut rewards = CoinList::new();
        rewards
            .update_rewards(&vec![
                OsmoCoin {
                    denom: "uosmo".into(),
                    amount: "1000".into(),
                },
                OsmoCoin {
                    denom: "uatom".into(),
                    amount: "2000".into(),
                },
                OsmoCoin {
                    denom: "uqsr".into(),
                    amount: "3000".into(),
                },
            ])
            .unwrap();

        assert_eq!(
            rewards,
            CoinList(vec![
                coin(1000, "uosmo"),
                coin(2000, "uatom"),
                coin(3000, "uqsr")
            ])
        );

        rewards
            .sub(&CoinList::from_coins(vec![coin(1500, "uqsr")]))
            .unwrap();

        assert_eq!(
            rewards,
            CoinList(vec![
                coin(1000, "uosmo"),
                coin(2000, "uatom"),
                coin(1500, "uqsr")
            ])
        );

        rewards
            .sub(&CoinList::from_coins(vec![coin(2000, "uqsr")]))
            .unwrap_err();

        rewards
            .sub(&CoinList::from_coins(vec![
                coin(999, "uqsr"),
                coin(999, "uosmo"),
            ]))
            .unwrap();

        assert_eq!(
            rewards,
            CoinList(vec![
                coin(1, "uosmo"),
                coin(2000, "uatom"),
                coin(501, "uqsr")
            ])
        );
    }

    #[test]
    fn percentage_works() {
        let mut rewards = CoinList::new();
        rewards
            .update_rewards(&vec![
                OsmoCoin {
                    denom: "uosmo".into(),
                    amount: "1000".into(),
                },
                OsmoCoin {
                    denom: "uatom".into(),
                    amount: "2000".into(),
                },
                OsmoCoin {
                    denom: "uqsr".into(),
                    amount: "3000".into(),
                },
            ])
            .unwrap();

        let ratio = rewards.mul_ratio(Decimal::from_ratio(Uint128::new(10), Uint128::new(100)));
        assert_eq!(
            ratio,
            CoinList(vec![
                coin(100, "uosmo"),
                coin(200, "uatom"),
                coin(300, "uqsr")
            ])
        )
    }

    #[test]
    fn sub_percentage_works() {
        let mut rewards = CoinList::new();
        rewards
            .update_rewards(&vec![
                OsmoCoin {
                    denom: "uosmo".into(),
                    amount: "1000".into(),
                },
                OsmoCoin {
                    denom: "uatom".into(),
                    amount: "2000".into(),
                },
                OsmoCoin {
                    denom: "uqsr".into(),
                    amount: "3000".into(),
                },
            ])
            .unwrap();

        let ratio = rewards
            .sub_ratio(Decimal::from_ratio(Uint128::new(10), Uint128::new(100)))
            .unwrap();
        assert_eq!(
            ratio,
            CoinList(vec![
                coin(100, "uosmo"),
                coin(200, "uatom"),
                coin(300, "uqsr")
            ])
        );
        assert_eq!(
            rewards,
            CoinList(vec![
                coin(900, "uosmo"),
                coin(1800, "uatom"),
                coin(2700, "uqsr")
            ])
        )
    }

    #[test]
    fn merge_works() {}

    #[test]
    fn add_works() {
        let mut rewards = CoinList::new();
        rewards
            .update_rewards(&vec![
                OsmoCoin {
                    denom: "uosmo".into(),
                    amount: "1000".into(),
                },
                OsmoCoin {
                    denom: "uatom".into(),
                    amount: "2000".into(),
                },
                OsmoCoin {
                    denom: "uqsr".into(),
                    amount: "3000".into(),
                },
            ])
            .unwrap();
        rewards
            .add(CoinList::from_coins(vec![
                coin(2000, "uosmo"),
                coin(2000, "uatom"),
                coin(6000, "uqsr"),
                coin(1234, "umars"),
            ]))
            .unwrap();
        assert_eq!(
            rewards,
            CoinList::from_coins(vec![
                coin(3000, "uosmo"),
                coin(4000, "uatom"),
                coin(9000, "uqsr"),
                coin(1234, "umars")
            ])
        )
    }

    #[test]
    fn update_rewards_works() {
        let mut rewards = CoinList::new();
        rewards
            .update_rewards(&vec![
                OsmoCoin {
                    denom: "uosmo".into(),
                    amount: "1000".into(),
                },
                OsmoCoin {
                    denom: "uatom".into(),
                    amount: "2000".into(),
                },
                OsmoCoin {
                    denom: "uqsr".into(),
                    amount: "3000".into(),
                },
            ])
            .unwrap();

        rewards
            .update_rewards(&vec![
                OsmoCoin {
                    denom: "uosmo".into(),
                    amount: "1000".into(),
                },
                OsmoCoin {
                    denom: "umars".into(),
                    amount: "1234".into(),
                },
                OsmoCoin {
                    denom: "uqsr".into(),
                    amount: "3000".into(),
                },
            ])
            .unwrap();

        assert_eq!(
            rewards,
            CoinList::from_coins(vec![
                coin(2000, "uosmo"),
                coin(2000, "uatom"),
                coin(6000, "uqsr"),
                coin(1234, "umars")
            ])
        );
    }
}
