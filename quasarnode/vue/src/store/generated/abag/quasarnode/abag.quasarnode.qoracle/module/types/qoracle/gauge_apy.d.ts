import { Writer, Reader } from "protobufjs/minimal";
export declare const protobufPackage = "abag.quasarnode.qoracle";
export interface GaugeAPY {
    gaugeId: number;
    duration: string;
    aPY: string;
}
export declare const GaugeAPY: {
    encode(message: GaugeAPY, writer?: Writer): Writer;
    decode(input: Reader | Uint8Array, length?: number): GaugeAPY;
    fromJSON(object: any): GaugeAPY;
    toJSON(message: GaugeAPY): unknown;
    fromPartial(object: DeepPartial<GaugeAPY>): GaugeAPY;
};
declare type Builtin = Date | Function | Uint8Array | string | number | undefined;
export declare type DeepPartial<T> = T extends Builtin ? T : T extends Array<infer U> ? Array<DeepPartial<U>> : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>> : T extends {} ? {
    [K in keyof T]?: DeepPartial<T[K]>;
} : Partial<T>;
export {};
