use std::str::FromStr;

use crate::{
    msg::{DepositRatioResponse, PrimitiveConfig},
    multitest::common::*,
};
use cosmwasm_schema::{schemars, serde};
use cosmwasm_std::{
    testing::MockApi, Addr, Binary, IbcChannel, IbcEndpoint, IbcMsg, IbcOrder, IbcQuery,
    MemoryStorage, StdError,
};
use cw_multi_test::{
    ibc::Ibc, App, AppBuilder, BankKeeper, CosmosRouter, DistributionKeeper, FailingModule, Module,
    StakeKeeper, WasmKeeper,
};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct QuasarVaultSuite {
    #[derivative(Debug = "ignore")]
    pub app: App<
        BankKeeper,
        MockApi,
        MemoryStorage,
        FailingModule<Empty, Empty, Empty>,
        WasmKeeper<Empty, Empty>,
        StakeKeeper,
        DistributionKeeper,
        AcceptingModule,
    >,
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

pub struct AcceptingModule;

impl Module for AcceptingModule {
    type ExecT = IbcMsg;
    type QueryT = IbcQuery;
    type SudoT = Empty;

    fn execute<ExecC, QueryC>(
        &self,
        _api: &dyn cosmwasm_std::Api,
        _storage: &mut dyn cosmwasm_std::Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &cosmwasm_std::BlockInfo,
        _sender: cosmwasm_std::Addr,
        _msg: Self::ExecT,
    ) -> anyhow::Result<AppResponse>
    where
        ExecC: std::fmt::Debug
            + Clone
            + PartialEq
            + schemars::JsonSchema
            + serde::de::DeserializeOwned
            + 'static,
        QueryC: cosmwasm_std::CustomQuery + serde::de::DeserializeOwned + 'static,
    {
        Ok(AppResponse::default())
    }

    fn sudo<ExecC, QueryC>(
        &self,
        _api: &dyn cosmwasm_std::Api,
        _storage: &mut dyn cosmwasm_std::Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &cosmwasm_std::BlockInfo,
        _msg: Self::SudoT,
    ) -> anyhow::Result<AppResponse>
    where
        ExecC: std::fmt::Debug
            + Clone
            + PartialEq
            + schemars::JsonSchema
            + serde::de::DeserializeOwned
            + 'static,
        QueryC: cosmwasm_std::CustomQuery + serde::de::DeserializeOwned + 'static,
    {
        Ok(AppResponse::default())
    }

    fn query(
        &self,
        _api: &dyn cosmwasm_std::Api,
        _storage: &dyn cosmwasm_std::Storage,
        _querier: &dyn cosmwasm_std::Querier,
        _block: &cosmwasm_std::BlockInfo,
        _request: Self::QueryT,
    ) -> anyhow::Result<cosmwasm_std::Binary> {
        Ok(Binary::default())
    }
}

impl Ibc for AcceptingModule {}

impl QuasarVaultSuite {
    pub fn init(
        init_msg: Option<VaultInstantiateMsg>,
        funds: Option<Vec<Coin>>,
    ) -> Result<QuasarVaultSuite> {
        let genesis_funds = vec![coin(150000, DENOM), coin(150000, LOCAL_DENOM)];
        let deployer = Addr::unchecked(DEPLOYER);
        let executor = Addr::unchecked(EXECUTOR);
        let user = Addr::unchecked(USER);
        let mut app = AppBuilder::new()
            .with_ibc(AcceptingModule)
            .build(|router, _, storage| {
                router
                    .bank
                    .init_balance(storage, &deployer, genesis_funds)
                    .unwrap();
            });
        // let mut app = App::new(|router, _, storage| {
        //     router
        //         .bank
        //         .init_balance(storage, &deployer, genesis_funds)
        //         .unwrap();
        // });
        app.send_tokens(
            deployer.clone(),
            user.clone(),
            &[coin(50000, DENOM), coin(50000, LOCAL_DENOM)],
        )?;
        app.send_tokens(
            deployer.clone(),
            executor.clone(),
            &[coin(50000, DENOM), coin(50000, LOCAL_DENOM)],
        )?;

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
                    expected_connection: "connection-0".to_string(),
                },
                &[],
                "router_contract",
                Some(deployer.to_string()),
            )
            .unwrap();

        // IbcChannelOpenMsg::OpenInit { channel: () }
        // app.wasm_sudo(contract_addr, msg)
        let endpoint = IbcEndpoint {
            port_id: "wasm.my_addr".to_string(),
            channel_id: "channel-1".to_string(),
        };
        let counterparty_endpoint = IbcEndpoint {
            port_id: "icahost".to_string(),
            channel_id: "channel-2".to_string(),
        };

        let version = r#"{"version":"ics27-1","encoding":"proto3","tx_type":"sdk_multi_msg","controller_connection_id":"connection-0","host_connection_id":"connection-0"}"#.to_string();
        let _channel = IbcChannel::new(
            endpoint,
            counterparty_endpoint,
            IbcOrder::Ordered,
            version,
            "connection-0".to_string(),
        );

        // Todo: keep track of this issue for ibc mock support in cw-multi-test: https://github.com/CosmWasm/cw-multi-test/issues/27
        // let ibc_channel_open_msg = IbcChannelOpenMsg::OpenInit { channel };
        // let res = app.execute(
        //     primitive.clone(),
        //     CosmosMsg::Ibc(IbcMsg::SendPacket {
        //         channel_id: "channel-0".to_string(),
        //         data: to_binary(&ibc_channel_open_msg)?,
        //         timeout: IbcTimeout::with_block(IbcTimeoutBlock {
        //             revision: 1,
        //             height: app.block_info().height + 5,
        //         }),
        //     }),
        // );
        // res.unwrap();
        // IbcChannelConnectMsg::OpenConfirm { channel: () }

        let vault = app
            .instantiate_contract(
                vault_id,
                deployer.clone(),
                &init_msg.unwrap_or(VaultInstantiateMsg {
                    name: "orion".to_string(),
                    thesis: "to generate yield, I guess".to_string(),
                    symbol: "ORN".to_string(),
                    decimals: 6,
                    min_withdrawal: 1u128.into(),
                    total_cap: 100000000u128.into(),
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
                            expected_connection: "connection-0".to_string(),
                        }),
                    }],
                }),
                &funds.unwrap_or(vec![]),
                "vault_contract",
                Some(deployer.to_string()), // admin: Option<String>, will need this for upgrading
            )
            .unwrap();

        // set depositor on primitive as the vault address
        let msg = PrimitiveExecuteMsg::SetDepositor {
            depositor: vault.to_string(),
        };
        app.execute_contract(deployer.clone(), primitive.clone(), &msg, &[])
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
        self.app.wrap().query_balance(addr.as_str(), DENOM)
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
        let msg = VaultExecuteMsg::Bond {
            recipient: Option::None,
        };
        self.app
            .execute_contract(sender.clone(), self.vault.clone(), &msg, &funds)
            .map_err(|err| match err.downcast::<VaultContractError>() {
                Ok(err_unwrapped) => err_unwrapped,
                Err(e) => VaultContractError::Std(StdError::GenericErr {
                    msg: e.root_cause().to_string(),
                }),
            })
            .map(|_| ())
    }

    pub fn query_deposit_ratio(&self, funds: Vec<Coin>) -> StdResult<DepositRatioResponse> {
        let msg = VaultQueryMsg::DepositRatio { funds };
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
