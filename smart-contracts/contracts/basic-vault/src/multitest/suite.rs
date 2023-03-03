use std::str::FromStr;

use crate::{
    msg::{DepositRatioResponse, PrimitiveConfig},
    multitest::common::*,
};
use cosmwasm_std::Addr;
use cw_multi_test::App;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct QuasarVaultSuite {
    #[derivative(Debug = "ignore")]
    pub app: App,
    // The account that deploys everything
    pub deployer: Addr,
    // executor address
    pub executor: Addr,
    // user address
    pub user: Addr,
    // vault address
    pub vault: Addr,
    // primitive address
    pub primitive: Addr,
}

impl QuasarVaultSuite {
    pub fn init(
        init_msg: Option<VaultInstantiateMsg>,
        funds: Option<Vec<Coin>>,
    ) -> Result<QuasarVaultSuite> {
        let genesis_funds = vec![coin(150000, DENOM), coin(150000, LOCAL_DENOM)];
        let deployer = Addr::unchecked(DEPLOYER);
        let executor = Addr::unchecked(EXECUTOR);
        let user = Addr::unchecked(USER);
        let mut app = App::new(|router, _, storage| {
            router
                .bank
                .init_balance(storage, &deployer, genesis_funds)
                .unwrap();
        });
        app.send_tokens(deployer.clone(), user.clone(), &[coin(50000, DENOM),coin(50000, LOCAL_DENOM)])?;
        app.send_tokens(deployer.clone(), executor.clone(), &[coin(50000, DENOM),coin(50000, LOCAL_DENOM)])?;

        let vault_id = app.store_code(contract_vault());
        let primitive_id = app.store_code(contract_primitive());

        let primitive = app
            .instantiate_contract(
                primitive_id,
                deployer.clone(),
                &PrimitiveInstantiateMsg {
                    lock_period: 64,
                    pool_id: 1,
                    pool_denom: "gamm/pool/1".to_string(),
                    local_denom: LOCAL_DENOM.to_string(),
                    base_denom: DENOM.to_string(),
                    quote_denom: "uatom".to_string(),
                    transfer_channel: "channel-0".to_string(),
                    return_source_channel: "channel-0".to_string(),
                },
                &[],
                "router_contract",
                Some(deployer.to_string()),
            )
            .unwrap();

        let vault = app
            .instantiate_contract(
                vault_id,
                deployer.clone(),
                &init_msg.unwrap_or(VaultInstantiateMsg {
                    name: "orion".to_string(),
                    symbol: "ORN".to_string(),
                    decimals: 6,
                    min_withdrawal: 1u128.into(),
                    primitives: vec![PrimitiveConfig {
                        weight: Decimal::from_str("0.33333333333")?,
                        address: primitive.to_string(),
                        init: crate::msg::PrimitiveInitMsg::LP(PrimitiveInstantiateMsg {
                            lock_period: 64,
                            pool_id: 1,
                            pool_denom: "gamm/pool/1".to_string(),
                            local_denom: LOCAL_DENOM.to_string(),
                            base_denom: DENOM.to_string(),
                            quote_denom: "uatom".to_string(),
                            transfer_channel: "channel-0".to_string(),
                            return_source_channel: "channel-0".to_string(),
                        }),
                    }],
                }),
                &funds.unwrap_or(vec![]),
                "vault_contract",
                Some(deployer.to_string()), // admin: Option<String>, will need this for upgrading
            )
            .unwrap();

        Ok(QuasarVaultSuite {
            app,
            user,
            executor,
            deployer,
            primitive,
            vault,
        })
    }

    pub fn query_balance(&self, addr: &Addr) -> StdResult<Coin> {
        Ok(self.app.wrap().query_balance(addr.as_str(), DENOM)?)
    }

    pub fn unbond(
        &mut self,
        sender: &Addr,
        unbond_amount: Option<Uint128>,
    ) -> Result<(), VaultContractError> {
        let msg = VaultExecuteMsg::Unbond {
            amount: unbond_amount,
        };
        self.app
            .execute_contract(sender.clone(), self.vault.clone(), &msg, &[])
            .map_err(|err| err.downcast().unwrap())
            .map(|_| ())
    }

    pub fn bond(&mut self, sender: &Addr, funds: Vec<Coin>) -> Result<(), VaultContractError> {
        let msg = VaultExecuteMsg::Bond {};
        self.app
            .execute_contract(sender.clone(), self.vault.clone(), &msg, &funds)
            .map_err(|err| err.downcast().unwrap())
            .map(|_| ())
    }

    pub fn query_deposit_ratio(&self, funds: Vec<Coin>) -> StdResult<DepositRatioResponse> {
        let msg = VaultQueryMsg::DepositRatio { funds: funds };
        self.app.wrap().query_wasm_smart(self.vault.clone(), &msg)
    }

    pub fn fast_forward_block_time(&mut self, forward_time_sec: u64) {
        let block = self.app.block_info();

        let mock_block = BlockInfo {
            height: block.height + 10,
            chain_id: block.chain_id,
            time: block.time.plus_seconds(forward_time_sec),
        };

        self.app.set_block(mock_block);
    }
}
