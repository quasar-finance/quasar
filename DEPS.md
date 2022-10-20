# Project Dependencies
This documents contains a list of all third party projects and libraries that either used directly by this project or this project depends on them to work properly.

## cosmos-sdk
World’s most popular framework for building application-specific blockchains.

<table>
  <tr>
    <th>Version</th>
    <td>v0.45.6</td>
  </tr>
  <tr>
    <th>License</th>
    <td>Apache 2.0</td>
  </tr>
  <tr>
    <th>Source Code</th>
    <td>
        <a>https://github.com/cosmos/cosmos-sdk/tree/v0.45.6</a>
    </td>
  </tr>
  <tr>
    <th>Usage</th>
    <td>
        <code>go get github.com/cosmos/cosmos-sdk@v0.45.6</code>
    </td>
  </tr>
</table>

### Note
*Right now, the major chains like cosmos-hub and osmosis are in the sdk `v0.45.x` series.*

*Cosmos-hub (with `cosmos-sdk v0.45.9` ) 
Osmosis (`github.com/osmosis-labs/cosmos-sdk v0.45.1-0.20221014170742-63f6778c9648`)*

*In this versioning, as you can see, `cosmos-sdk` has another fork with osmosis which they are using in their chain and is much improved. Eco-system has a consensus to merge the osmosis `cosmos-sdk` fork to upstream osmosis `cosmos-sdk` in `cosmos-sdk v0.47`. `cosmos-sdk v0.47` also has native liquid staking in the default staking module, param module migration, and probably Interchain security.*

*In between there is `cosmos-sdk v0.46` release which is not acceptable on osmosis and wasm. Which also makes it incompatible with Quasar too.*

## ibc-go
Interblockchain Communication Protocol (IBC) implementation in Golang. (Unofficial version with ICQ module implementation)

<table>
  <tr>
    <th>Version</th>
    <td>v3.3.0</td>
  </tr>
  <tr>
    <th>License</th>
    <td>MIT</td>
  </tr>
  <th>Official Source Code</th>
    <td>
        <a>https://github.com/cosmos/ibc-go/tree/v3.3.0</a>
    </td>
  <tr>
    <th>Source Code</th>
    <td>
        <a>https://github.com/strangelove-ventures/ibc-go/tree/v3.3.0-icq</a>
    </td>
  </tr>
  <tr>
    <th>Usage</th>
    <td>
        <code>go mod edit --replace=github.com/cosmos/ibc-go/v3=github.com/strangelove-ventures/ibc-go/v3@v3.0.0-20221014082552-99c8caa484af</code>
    </td>
  </tr>
</table>

### Note
*We use strange-loves fork of ibc-go currently because of our ICQ implementation. In future, after ICQ becomes an official part of ibc-go we will migrate to the official repository.*

## wasmd
First implementation of a cosmos zone with wasm smart contracts enabled.

<table>
  <tr>
    <th>Version</th>
    <td>v0.27.0</td>
  </tr>
  <tr>
    <th>License</th>
    <td>Apache 2.0</td>
  </tr>
  <tr>
    <th>Source Code</th>
    <td>
        <a>https://github.com/CosmWasm/wasmd/tree/v0.27.0</a>
    </td>
  </tr>
  <tr>
    <th>Usage</th>
    <td>
        <code>go get github.com/CosmWasm/wasmd@v0.27.0</code>
    </td>
  </tr>
</table>

## Osmosis
A fair-launched, customizable automated market maker for interchain assets that allows the creation and management of non-custodial, self-balancing, interchain token index similar to one of Balancer. (Unofficial ICQ enabled fork)

<table>
  <tr>
    <th>Version</th>
    <td>v12.0.0</td>
  </tr>
  <tr>
    <th>License</th>
    <td>Apache 2.0</td>
  </tr>
  <th>Official Source Code</th>
    <td>
        <a>https://github.com/osmosis-labs/osmosis/tree/v12.0.0</a>
    </td>
  <tr>
  <th>Source Code</th>
    <td>
        <a>https://github.com/quasar-finance/osmosis/tree/v12.0.0-icq</a>
    </td>
  </tr>
  <tr>
    <th>Usage</th>
    <td>
        IBC (ICA and ICQ connections)
    </td>
  </tr>
</table>

### Note
*Since ICQ is not part of official ibc-go, we had to make a fork of osmosis, replace ibc-go in `go.mod` with strange-loves fork and integrate ICQ module in `app.go`.*

## Bandchain
High-performance Blockchain Built for Data Oracle.

<table>
  <tr>
    <th>Version</th>
    <td>v2.4.0</td>
  </tr>
  <tr>
    <th>License</th>
    <td>GPL 3.0</td>
  </tr>
  <tr>
    <th>Source Code</th>
    <td>
        <a>https://github.com/bandprotocol/chain/tree/v2.4.0</a>
    </td>
  </tr>
  <tr>
    <th>Testnet</th>
    <td>
        <a>https://laozi-testnet5.bandchain.org</a>
    </td>
  </tr>
  <tr>
    <th>Usage</th>
    <td>
        IBC
    </td>
  </tr>
</table>

## Gaia (Cosmos Hub)
A blockchain built base on cosmos-sdk.

<table>
  <tr>
    <th>Version</th>
    <td>v7.0.1</td>
  </tr>
  <tr>
    <th>License</th>
    <td>Apache 2.0</td>
  </tr>
  <th>Official Source Code</th>
    <td>
        <a>https://github.com/cosmos/gaia/tree/v7.0.1</a>
    </td>
  <tr>
  <tr>
    <th>Source Code</th>
    <td>
        <a>https://github.com/quasar-finance/gaia/tree/bugfix/replace_default_transfer_with_router_module</a>
    </td>
  </tr>
  <tr>
    <th>Usage</th>
    <td>
        IBC Transfer
    </td>
  </tr>
</table>

## Note
*We may need to switch to official repository since the problem with strange-loves transfer router is fixed*