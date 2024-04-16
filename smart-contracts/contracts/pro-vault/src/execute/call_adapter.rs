use cosmwasm_std::{DepsMut, Env, Response};

use crate::{
    adapters::{
        generic_vault::{
            MultiAssetAnyDepositVaultAdapterWrapper, MultiAssetExactDepositVaultAdapterWrapper,
            SingeAssetVaultAdapterWrapper, VaultAction, VaultAdapters,
        },
        r#trait::{Adapter, VaultAdapter},
    },
    msg::AdapterExtensionMsg,
    state::{Adapters, ADAPTERS},
    ContractError,
};

pub fn execute_call_adapter(
    deps: DepsMut,
    env: Env,
    msg: AdapterExtensionMsg,
) -> Result<Response, ContractError> {
    match msg {
        AdapterExtensionMsg::Vault { address, action } => {
            execute_vault_adapter(deps, env, address, action)
        }
        AdapterExtensionMsg::Debt {} => todo!(),
        AdapterExtensionMsg::Swap {} => todo!(),
    }
}

fn execute_vault_adapter(
    deps: DepsMut,
    env: Env,
    address: String,
    action: VaultAction,
) -> Result<Response, ContractError> {
    let address = deps.api.addr_validate(&address)?;

    match ADAPTERS.load(deps.storage, address.clone())? {
        Adapters::Vault(a) => match a {
            VaultAdapters::SingleAsset => {
                do_action(SingeAssetVaultAdapterWrapper { address }, action)
            }
            VaultAdapters::MultiAsssetExact => do_action(
                MultiAssetExactDepositVaultAdapterWrapper { address },
                action,
            ),
            VaultAdapters::MultiAssetAny => {
                do_action(MultiAssetAnyDepositVaultAdapterWrapper { address }, action)
            }
        },
        _ => unimplemented!(),
    }
}

fn do_action<T: VaultAdapter>(adapter: T, action: VaultAction) -> Result<Response, ContractError>
where
    ContractError: From<<T as VaultAdapter>::AdapterError>,
{
    match action {
        VaultAction::Deposit { assets } => Ok(adapter.deposit(assets)?),
        VaultAction::Withdraw { shares } => Ok(adapter.withdraw(shares)?),
        VaultAction::Claim {} => Ok(adapter.claim_incentives()?),
    }
}

pub fn execute_debt_adapter()
