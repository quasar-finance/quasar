import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { Registry } from "@cosmjs/proto-signing";
import { AminoTypes, GasPrice } from "@cosmjs/stargate";
import { chains } from 'chain-registry';
import { getOfflineSignerAmino as getOfflineSigner } from 'cosmjs-utils';
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
import fs from 'node:fs';

export const RPC_ENDPOINT = "https://rpc.osmosis.zone";
export const DAODAO = "osmo1kylkpkx7zy40429zcs5yq6hm2mlw07u872fsqud26ylyc5urpxfsvw98vn";
export const CONTRACTS = "./smart-contracts/contracts.json";

export function readFileAsJSON(file) {
    try {
        const data = fs.readFileSync(file, 'utf8');
        return JSON.parse(data);
    } catch (err) {
        console.error(err);
    }
}

export function writeFileFromJSON(file, json) {
    try {
        fs.writeFileSync(file, JSON.stringify(json));
    } catch (err) {
        console.error(err);
    }
}

export function createMigrateMsg(contract, newCodeId, migrate_msg) {
    const binary_msg = Buffer.from(JSON.stringify(migrate_msg)).toString('base64');
    return {
        wasm: {
            migrate: {
                contract_addr: contract,
                msg: binary_msg,
                new_code_id: newCodeId
            }
        }
    }
}

export function createProposalMsg(title, description, msgs) {
    return {
        propose: {
            msg: {
                propose: {
                    description,
                    msgs,
                    title,
                }
            }
        }
    }
}

export async function getClient(mnemonic) {
    const chain = chains.find(({ chain_id }) => chain_id === "osmosis-1");
    const signer = await getOfflineSigner({
        mnemonic,
        chain
    });
    const gasPrice = GasPrice.fromString("0.125uosmo");

    const accounts = await signer.getAccounts();
    const sender = accounts[0].address;

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

    const client = await SigningCosmWasmClient.connectWithSigner(
        RPC_ENDPOINT,
        signer,
        { chain, gasPrice, registry, aminoTypes }
    );

    return client;
}
