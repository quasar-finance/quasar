import { cosmwasm, osmosis } from 'osmojs';

import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { Registry } from "@cosmjs/proto-signing";
import { AminoTypes, GasPrice, coins } from "@cosmjs/stargate";
import { chains } from 'chain-registry';
import { SendAuthorization } from 'cosmjs-types/cosmos/bank/v1beta1/authz.js';
import { getOfflineSignerAmino as getOfflineSigner } from 'cosmjs-utils';
import { readFileSync } from "fs";
import {
  cosmosAminoConverters,
  cosmosProtoRegistry,
  cosmwasmAminoConverters,
  cosmwasmProtoRegistry,
  ibcAminoConverters,
  ibcProtoRegistry,
  osmosisAminoConverters,
  osmosisProtoRegistry
} from 'osmojs';
const {
  QueryParamsRequest
} = osmosis.tokenfactory.v1beta1;
const { createRPCQueryClient } = osmosis.ClientFactory;
const {
  executeContract,
} = cosmwasm.wasm.v1.MessageComposer.withTypeUrl;

const RPC_ENDPOINT = "https://rpc.testnet.osmosis.zone";

function grantSendAuthorization(grantee, granter, duration, coin) {
  const msgValue = {
    granter,
    grantee,
    grant: {
      authorization: {
        typeUrl: "/cosmos.bank.v1beta1.SendAuthorization",
        value: SendAuthorization.encode(SendAuthorization.fromPartial({
          spendLimit: [coin]
        })).finish(),
      },
    }
  }
  return {
     typeUrl: "/cosmos.authz.v1beta1.MsgGrant",
     value: msgValue,
  }
}

async function main() {
  const basePath = "/workspaces/quasar/artifacts";
  // const basePath = "/workspaces/quasar/smart-contracts/artifacts";
  const mnemonic = process.env.MNEMONIC;
    const fee = {
      amount: coins(15000, 'uosmo'),
      gas: '6000000'
    }
    const chain = chains.find(({ chain_id }) => chain_id === "osmo-test-5");
    console.log(chain);
    const signer = await getOfflineSigner({
        mnemonic,
        chain
    });
    const gasPrice = GasPrice.fromString("0.125uosmo");

    const accounts = await signer.getAccounts();
    const sender = accounts[0].address;
    console.log(sender);

    const protoRegistry = [
      ...cosmosProtoRegistry,
      ...cosmwasmProtoRegistry,
      ...ibcProtoRegistry,
      ...osmosisProtoRegistry
  ];
  
  const aminoConverters = {
      ...cosmosAminoConverters,
      ...cosmwasmAminoConverters,
      ...ibcAminoConverters,
      ...osmosisAminoConverters
  };

  const registry = new Registry(protoRegistry);
  const aminoTypes = new AminoTypes(aminoConverters);

    console.log("create client");
    const client = await SigningCosmWasmClient.connectWithSigner(
      RPC_ENDPOINT,
      signer,
      // { chain, gasPrice }
      { chain, gasPrice, registry, aminoTypes }
    );


    async function storeContract(name) {
      const filename = name.replace(/-/g,'_') + '.wasm';
      const path = basePath + '/' + filename;
      console.log("Store contract (" + name + ")");

      const wasm = readFileSync(path);
      const response = await client.upload(sender, wasm, fee);
      console.log(response);
      return response.codeId;
    }

    async function instantiateContract(codeId, label, msg) {
      const response = await client.instantiate(sender, codeId, msg, label, fee);
      console.log(response)
      return response.contractAddress;
    }

    const ATOM = "ibc/A8C2D23A1E6F95DA4E48BA349667E322BD7A6C996D8A4AAE8BA72E190F3D1477";
    const OSMO = "uosmo";
    const ION = "uion";
    const USDC = "ibc/6F34E1BD664C36CE49ACC28E60D62559A5F96C4F9A6CCE4FC5A67B2852E24CFE";

    const contract = "osmo17lzj7g3uyukhvavmxmagrql3spqtq0sw96cdz2ugldy4wlje9raqlax02u";
    const name = "cw-ica-controller";
    // const codeId = await storeContract(name);
    // console.log(codeId);
    // const codeId = 9769; // lst-adapter
    // const codeId = 9770; // oracle
    const codeId = 9973; // cw-ica-controller
    const oracle = "osmo13fl6cxkmps4yk8hdgd62tc3pqtdm9eshwks90rwhx2u58rth9gcs5rmphz";
    const cw_ica = "osmo1zzuccql3ttljzm7rmuyqe2a9yqnkth90qa63ytfc07sedsl53cesy5wpxd";
    // const addr = await instantiateContract(codeId, name, { redemption_rate: "1.0" });
    // const addr = await instantiateContract(codeId, name, {
    //   channel_open_init_options: {
    //     connection_id: "connection-3027",
    //     counterparty_connection_id: "connection-120",
    //   }
    //  });
    // const response1 = await client.execute(sender, cw_ica, { create_channel: {}}, "auto");
    // console.log(response1);

    const query = {
      get_contract_state: {}
    };
    const response = await client.queryContractSmart(cw_ica, query);
    console.log(response);
    // const addr = await instantiateContract(codeId, name, {
    //   module: { 
    //     owner: sender,
    //     vault: sender,
    //     observer: sender,
    //     denoms: {
    //       lst: "uosmo",
    //       underlying: "uosmo",
    //     },
    //     stride_oracle: oracle,
    //     unbond_period_secs: 500,
    //   }
    //  });
    // console.log(addr);

    // offer_asset: AssetInfoUnchecked,
    // ask_asset: AssetInfoUnchecked,
    // path: Vec<SwapAmountInRoute>,
    // bidirectional: bool,
    // const msg = {
    //   swap: {
    //     out_denom: USDC,
    //     minimum_receive: "1",
    //     path: [{
    //       poolID: "94",
    //       token_out_denom: "uosmo"
    //     }, {
    //       poolID: "117",
    //       token_out_denom: USDC
    //     }],
    //   }
    // };
    // const msg = {
    //   set_path: {
    //     offer_denom: USDC,
    //     ask_denom: ATOM,
    //     path: [117, 94],
    //     bidirectional: true
    //   }
    // }
    // const response1 = await client.execute(sender, contract, msg, "auto", "", [{amount: "123", denom: ATOM}]);
    // console.log(response1);
    // const q = client.osmosis.gamm.
    // const query = {
    //   simulate_swaps: {
    //     offer: {
    //       info: {
    //         native:  ION
    //       },
    //       amount: "100",
    //     },
    //     routes: [{poolID: "1", token_out_denom: OSMO}, {poolID: "12", token_out_denom: ATOM}],
    //     //         offer_asset: {
    //     //   native:  ION
    //     // },
    //     // offer_amount: "100",
    //     // ask_asset: {
    //     //   native:  ATOM
    //     // },
    //   }
    // };
    // const query = {
    //   best_path_for_pair: {
    //     offer: {
    //       amount: "123",
    //       denom: USDC,
    //     },
    //     ask_denom: ATOM
    //   }
    // };
    // const query = {
    //   simulate_swaps: {
    //     offer: {
    //       amount: "123",
    //       denom: USDC,
    //     },
    //     routes: [{poolID: "117", token_out_denom: "uosmo"}, {poolID: "95", token_out_denom: ATOM}]
    //   }
    // };
    // const query = {
    //   paths_for_pair: {
    //     offer_denom: USDC,
    //     ask_denom: ATOM,
    //   }
    // };
    // const query = {
    //   supported_offer_assets: {
    //     ask_denom: USDC
    //   }
    // };
    // const response = await client.queryContractSmart(contract, query);
    // console.log(response);
}

main();
// import { osmosis } from "./codegen";
// const { createRPCQueryClient } = osmosis.ClientFactory;


// async function getBalance() {

//     const client = await createRPCQueryClient({ rpcEndpoint: "https://rpc.osmosis.zone" });
//     // // now you can query the cosmos modules
//     const balance = await client.cosmos.bank.v1beta1
//         .allBalances({ address: 'osmo1fl48vsnmsdzcv85q5d2q4z5ajdha8yu3aq6l09' });
// }

// async function getPools() {

//     const client = await createRPCQueryClient({ rpcEndpoint: "https://rpc.osmosis.zone" });
//     const pools = await client.osmosis.gamm.v1beta1.pools();
//     console.log(pools);
// }

// async function getPool() {
//   const client = await createRPCQueryClient({ rpcEndpoint: "https://rpc.osmosis.zone" });
//   const p = await client.osmosis.poolmanager.v1beta1.pool({poolId: BigInt(10)});
//   console.log(p.pool.poolAssets);
// }

// // getPools().catch(console.error);
// // getBalance().catch(console.error);
// getPool().catch(console.error);
