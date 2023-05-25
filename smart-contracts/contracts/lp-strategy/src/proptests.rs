#![cfg(test)]
mod tests {
    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env},
        Addr, Coin, IbcEndpoint, Timestamp, Uint128,
    };
    use cw20::BalanceResponse;
    use proptest::prelude::*;
    use quasar_types::{
        ibc::{ChannelInfo, ChannelType},
        ica::handshake::IcaMetadata,
    };

    use crate::{
        error::Trap,
        helpers::IbcMsgKind,
        ibc_lock::{IbcLock, Lock},
        msg::{
            ChannelsResponse, ConfigResponse, IcaAddressResponse, IcaBalanceResponse,
            IcaChannelResponse, LockResponse, LpSharesResponse, PrimitiveSharesResponse, QueryMsg,
            TrappedErrorsResponse, UnbondingClaimResponse,
        },
        queries::query,
        state::{
            Config, LpCache, Unbond, CONFIG, IBC_LOCK, LP_SHARES, SHARES, TOTAL_VAULT_BALANCE,
            TRAPS, UNBONDING_CLAIMS,
        },
        test_helpers::{setup_default_ica, setup_default_icq},
    };

    impl Arbitrary for Config {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            (
                any::<u64>(),    // lock_period
                any::<u64>(),    // pool_id
                any::<String>(), // pool_denom
                any::<String>(), // base_denom
                any::<String>(), // quote_denom
                any::<String>(), // local_denom
                any::<String>(), // transfer_channel
                any::<String>(), // return_source_channel
                any::<String>(), // expected_connection
            )
                .prop_map(
                    |(
                        lock_period,
                        pool_id,
                        pool_denom,
                        base_denom,
                        quote_denom,
                        local_denom,
                        transfer_channel,
                        return_source_channel,
                        expected_connection,
                    )| {
                        Config {
                            lock_period,
                            pool_id,
                            pool_denom,
                            base_denom,
                            quote_denom,
                            local_denom,
                            transfer_channel,
                            return_source_channel,
                            expected_connection,
                        }
                    },
                )
                .boxed()
        }
    }

    fn address_strategy(prefix: &str) -> impl Strategy<Value = String> {
        let prefix = prefix.to_string();
        let len = 38; // hardcoded, not sure if this is correct

        prop::collection::vec("[a-z0-9]", len..=len)
            .prop_map(move |chars| format!("{}1{}", prefix, chars.join("")))
    }

    impl Arbitrary for IbcLock {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            prop_oneof![Just(IbcLock::Locked), Just(IbcLock::Unlocked),].boxed()
        }
    }

    impl Arbitrary for Lock {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
            (
                any::<IbcLock>(),
                any::<IbcLock>(),
                any::<IbcLock>(),
                any::<IbcLock>(),
                any::<IbcLock>(),
            )
                .prop_map(|(bond, start_unbond, unbond, recovery, migration)| Self {
                    bond,
                    start_unbond,
                    unbond,
                    recovery,
                    migration,
                })
                .boxed()
        }
    }

    #[test]
    fn get_channels_works() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        setup_default_ica(deps.as_mut().storage).unwrap();
        setup_default_icq(deps.as_mut().storage).unwrap();

        let q = QueryMsg::Channels {};
        let res = query(deps.as_ref(), env, q).unwrap();
        let channels_response: ChannelsResponse = from_binary(&res).unwrap();

        assert_eq!(
            &channels_response.channels[0],
            &ChannelInfo {
                id: "channel-1".to_string(),
                counterparty_endpoint: IbcEndpoint {
                    port_id: "icqhost".to_string(),
                    channel_id: "channel-1".to_string(),
                },
                connection_id: "connection-0".to_string(),
                channel_type: ChannelType::Icq {
                    channel_ty: "icq-1".to_string(),
                },
                handshake_state: quasar_types::ibc::HandshakeState::Open,
            }
        );
        assert_eq!(
            &channels_response.channels[1],
            &ChannelInfo {
                id: "channel-2".to_string(),
                counterparty_endpoint: IbcEndpoint {
                    port_id: "icahost".to_string(),
                    channel_id: "channel-2".to_string(),
                },
                connection_id: "connection-0".to_string(),
                channel_type: ChannelType::Ica {
                    channel_ty: IcaMetadata::with_connections(
                        "connection-1".to_string(),
                        "connection-2".to_string(),
                    ),
                    counter_party_address: Some("osmo-address".to_string()),
                },
                handshake_state: quasar_types::ibc::HandshakeState::Open,
            }
        );
    }

    #[test]
    fn get_ica_address_works() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        setup_default_ica(deps.as_mut().storage).unwrap();
        let q = QueryMsg::IcaAddress {};
        let res: IcaAddressResponse = from_binary(&query(deps.as_ref(), env, q).unwrap()).unwrap();
        assert_eq!(res.address, "osmo-address");
    }

    #[test]
    fn get_ica_channel_works() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        setup_default_ica(deps.as_mut().storage).unwrap();

        let q = QueryMsg::IcaChannel {};
        let res: IcaChannelResponse = from_binary(&query(deps.as_ref(), env, q).unwrap()).unwrap();
        assert_eq!(res.channel, "channel-2");
    }

    proptest! {
        #[test]
        fn get_config_works(
            config in any::<Config>()
        ) {
            let mut deps = mock_dependencies();
            let env = mock_env();
            CONFIG.save(deps.as_mut().storage, &config).unwrap();
            let q = QueryMsg::Config {};
            let res = query(deps.as_ref(), env, q).unwrap();
            let config_response: ConfigResponse = from_binary(&res).unwrap();
            prop_assert_eq!(config_response.config, config);
        }

        #[test]
        fn get_balance_works(
            addr in address_strategy("quasar"),
            bal in any::<u128>()
        ) {
            let mut deps = mock_dependencies();
            let env = mock_env();
            let address = Addr::unchecked(&addr);
            let balance = Uint128::from(bal);

            SHARES.save(deps.as_mut().storage, address.clone(), &balance).unwrap();

            let q = QueryMsg::Balance { address: address.to_string() };
            let res: BalanceResponse = from_binary(&query(deps.as_ref(), env, q).unwrap()).unwrap();

            prop_assert_eq!(res.balance, balance);
        }

        #[test]
        fn get_primitive_shares_works(
            (addr, bal) in (0..10usize).prop_flat_map(|size| {
                (
                    proptest::collection::vec(address_strategy("quasar"), size),
                    // use u64 to make sure we don't overflow; can't use Uint because it doesn't implement Arbitrary
                    proptest::collection::vec(any::<u64>(), size)
                )
            })
        ) {
            let mut deps = mock_dependencies();
            let env = mock_env();

            // create shares for each address
            for (address, balance) in addr.iter().zip(bal.iter()) {
                SHARES.save(deps.as_mut().storage, Addr::unchecked(address), &Uint128::from(*balance)).unwrap();
            }

            let balance = Uint128::from(bal.iter().fold(0, |acc, x| acc + *x as u128));

            let q = QueryMsg::PrimitiveShares { };
            let res: PrimitiveSharesResponse = from_binary(&query(deps.as_ref(), env, q).unwrap()).unwrap();

            prop_assert_eq!(res.total, balance);
        }

        #[test]
        fn get_ica_balance_works(
            config in any::<Config>(),
            bal in any::<u128>()
        ) {
            let mut deps = mock_dependencies();
            let env = mock_env();
            let balance = Uint128::from(bal);
            CONFIG.save(deps.as_mut().storage, &config).unwrap();
            TOTAL_VAULT_BALANCE.save(deps.as_mut().storage, &balance).unwrap();

            let q = QueryMsg::IcaBalance {};
            let res: IcaBalanceResponse = from_binary(&query(deps.as_ref(), env, q).unwrap()).unwrap();
            assert_eq!(res.amount, Coin {
                denom: config.local_denom,
                amount: balance,
            });

        }

        #[test]
        fn get_lock_works(
            lock in any::<Lock>()
        ) {
            let mut deps = mock_dependencies();
            let env = mock_env();
            IBC_LOCK.save(deps.as_mut().storage, &lock).unwrap();

            let q = QueryMsg::Lock {};
            let res: LockResponse = from_binary(&query(deps.as_ref(), env, q).unwrap()).unwrap();
            assert_eq!(res.lock, lock);
        }

        #[test]
        fn get_lp_shares_works(
            locked_shares in any::<u128>(),
            w_unlocked_shares in any::<u128>(),
            d_unlocked_shares in any::<u128>(),
            ) {
            let mut deps = mock_dependencies();
            let env = mock_env();
            let shares = LpCache {
                locked_shares: Uint128::from(locked_shares),
                w_unlocked_shares: Uint128::from(w_unlocked_shares),
                d_unlocked_shares: Uint128::from(d_unlocked_shares),
            };
            LP_SHARES.save(deps.as_mut().storage, &shares).unwrap();

            let q = QueryMsg::LpShares {};
            let res: LpSharesResponse = from_binary(&query(deps.as_ref(), env, q).unwrap()).unwrap();
            assert_eq!(res.lp_shares, shares);
        }

        #[test]
        fn get_trapped_errors_works(
            seq in any::<u64>(),
            chan in any::<String>(),
            error in any::<String>(),
            last_succesful in any::<bool>(),
        ) {
            let mut deps = mock_dependencies();
            let env = mock_env();
            let trap = Trap {
                error:error,
                // hardcoded for simplicity
                step: IbcMsgKind::Icq,
                last_succesful: last_succesful,
            };
            TRAPS.save(deps.as_mut().storage, (seq.clone(), chan.clone()), &trap).unwrap();

            let q = QueryMsg::TrappedErrors {};
            let res: TrappedErrorsResponse = from_binary(&query(deps.as_ref(), env, q).unwrap()).unwrap();
            let key = format!("{}-{}", seq, chan);
            assert_eq!(res.errors.get(&key).unwrap(), &trap);
        }

        #[test]
        fn get_unbonding_claims(
            addr in address_strategy("quasar"),
            id in any::<String>(),
            lp_shares in any::<u128>(),
            unlock_time in any::<u64>(),
            attempted in any::<bool>(),
        ) {
            let mut deps = mock_dependencies();
            let env = mock_env();

            let unbond = Unbond {
                lp_shares: Uint128::from(lp_shares),
                unlock_time: Timestamp::from_nanos(unlock_time),
                attempted: attempted,
                owner: Addr::unchecked(addr.clone()),
                id: id.clone(),
            };
            UNBONDING_CLAIMS.save(deps.as_mut().storage, (Addr::unchecked(&addr), id.clone()), &unbond).unwrap();

            let q = QueryMsg::UnbondingClaim { addr: Addr::unchecked(&addr), id};
            let res: UnbondingClaimResponse = from_binary(&query(deps.as_ref(), env, q).unwrap()).unwrap();

            assert_eq!(res.unbond, Some(unbond));
        }

    }
}
