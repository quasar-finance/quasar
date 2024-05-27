# Incentives Gauge Factory

The role of this contract is to create, control and update [merkle-incentives gauges](./../merkle-incentives/) for vaults and pools.

## Initialize the contract

Both parameters can be configured later on so the contract can be initialized without sending any parameters.

**Note:** `admin` defaults to the sender.

```Rust
#[cw_serde]
pub struct InstantiateMsg {
    /// contract admin (defaults to sender during initilization)
    pub admin: Option<String>,

    /// guage contract code id (can be set later on)
    pub gauge_codeid: Option<u64>,
}
```

## Configuring the contract admin

Send `admin_update { "addr": "address" }` with a string to change the contract admin

```Rust
crate::msg::ExecuteMsg::AdminUpdate {
    addr: String
}
```

## Configuring the a new gauge contract code id

Send `gauge_msg { "code_update": 0 }` to modify the gauge contract code id:

```Rust
crate::msg::ExecuteMsg::GaugeMsg(crate::msg::GaugeMsg::CodeUpdate {
    code: u64
})
```

## Create a new pool gauge

```Rust
crate::msg::ExecuteMsg::GaugeMsg(crate::msg::GaugeMsg::Create {
    kind: GaugeKind::new_pool(
        Addr::unchecked("pool_addr"),
        PoolKind::Liquidity,
        "ucosm".to_string(),
        Some("uatom".to_string()),
    ),
    gauge: Gauge {
        period: BlockPeriod {
            start: env.block.height + 1u64,
            end: env.block.height + 10u64,
            expiry: env.block.height + 100u64,
        },
        incentives: vec![coin(1000, "ucosm")],
        clawback: "clawback_addr".to_string(),
    },
    fee: Fee::new(
        "reciever".to_string(),
        Decimal::from_ratio(Uint128::from(500u16), Uint128::one()),
        CoinList::new(vec![coin(100, "ucosm")]),
    ),
}
```

## Querying the list of gauge contracts

The response format in Rust is this:

```Rust
#[cw_serde]
pub struct GaugeListResponse {
    pub gauges: Vec<Gauge>,
    pub kinds: Vec<GaugeKind>,
    pub fees: Vec<Fee>,
}
```

this can be used on the front-end in this way:

```JavaScript
const gauge = {
    gauge: list.gauges[0],
    kind: list.kinds[0],
    fee: list.fees[0]
}
```