#![cfg(test)]
mod tests {
    use cosmwasm_std::{
        from_binary,
        testing::{mock_dependencies, mock_env},
        Addr, IbcEndpoint, Uint128,
    };
    use cw20::BalanceResponse;
    use proptest::prelude::*;
    use quasar_types::{
        ibc::{ChannelInfo, ChannelType},
        ica::handshake::IcaMetadata,
    };

    use crate::{
        msg::{
            ChannelsResponse, ConfigResponse, IcaAddressResponse, PrimitiveSharesResponse, QueryMsg,
        },
        queries::query,
        state::{Config, CONFIG, SHARES},
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
        let res = query(deps.as_ref(), env, q).unwrap();
        let ica_address_response: IcaAddressResponse = from_binary(&res).unwrap();
        assert_eq!(ica_address_response.address, "osmo-address");
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


    }
}
