import { Params } from "../qoracle/params";
import { PoolPosition } from "../qoracle/pool_position";
import { PoolRanking } from "../qoracle/pool_ranking";
import { PoolSpotPrice } from "../qoracle/pool_spot_price";
import { PoolInfo } from "../qoracle/pool_info";
import { Writer, Reader } from "protobufjs/minimal";
export declare const protobufPackage = "abag.quasarnode.qoracle";
/** GenesisState defines the qoracle module's genesis state. */
export interface GenesisState {
    params: Params | undefined;
    poolPositionList: PoolPosition[];
    poolRanking: PoolRanking | undefined;
    poolSpotPriceList: PoolSpotPrice[];
    /** this line is used by starport scaffolding # genesis/proto/state */
    poolInfoList: PoolInfo[];
}
export declare const GenesisState: {
    encode(message: GenesisState, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): GenesisState;
    fromJSON(object: any): GenesisState;
    toJSON(message: GenesisState): unknown;
    fromPartial(object: DeepPartial<GenesisState>): GenesisState;
};
declare type Builtin = Date | Function | Uint8Array | string | number | undefined;
export declare type DeepPartial<T> = T extends Builtin ? T : T extends Array<infer U> ? Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>> : T extends {} ? {
    [K in keyof T]?: DeepPartial<T[K]>;
} : Partial<T>;
export {};
