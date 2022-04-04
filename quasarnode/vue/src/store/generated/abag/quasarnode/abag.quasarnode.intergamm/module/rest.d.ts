export interface GammPoolAsset {
    /**
     * Coins we are talking about,
     * the denomination must be unique amongst all PoolAssets for this pool.
     */
    token?: V1Beta1Coin;
    weight?: string;
}
export declare type IntergammMsgCreatePoolResponse = object;
export declare type IntergammMsgExitPoolResponse = object;
export declare type IntergammMsgIbcTransferResponse = object;
export declare type IntergammMsgJoinPoolResponse = object;
export declare type IntergammMsgRegisterAccountResponse = object;
export interface IntergammQueryInterchainAccountFromAddressResponse {
    interchainAccountAddress?: string;
}
/**
 * QueryParamsResponse is response type for the Query/Params RPC method.
 */
export interface IntergammQueryParamsResponse {
    /** params holds all the parameters of this module. */
    params?: QuasarnodeintergammParams;
}
/**
* BalancerPoolParams defined the parameters that will be managed by the pool
governance in the future. This params are not managed by the chain
governance. Instead they will be managed by the token holders of the pool.
The pool's token holders are specified in future_pool_governor.
*/
export interface PoolmodelsBalancerPoolParams {
    swapFee?: string;
    exitFee?: string;
    smoothWeightChangeParams?: PoolmodelsSmoothWeightChangeParams;
}
export interface PoolmodelsSmoothWeightChangeParams {
    /**
     * The start time for beginning the weight change.
     * If a parameter change / pool instantiation leaves this blank,
     * it should be generated by the state_machine as the current time.
     * @format date-time
     */
    startTime?: string;
    duration?: string;
    /**
     * The initial pool weights. These are copied from the pool's settings
     * at the time of weight change instantiation.
     * The amount PoolAsset.token.amount field is ignored if present,
     * future type refactorings should just have a type with the denom & weight
     * here.
     */
    initialPoolWeights?: GammPoolAsset[];
    /**
     * The target pool weights. The pool weights will change linearly with respect
     * to time between start_time, and start_time + duration. The amount
     * PoolAsset.token.amount field is ignored if present, future type
     * refactorings should just have a type with the denom & weight here.
     */
    targetPoolWeights?: GammPoolAsset[];
}
/**
* `Any` contains an arbitrary serialized protocol buffer message along with a
URL that describes the type of the serialized message.

Protobuf library provides support to pack/unpack Any values in the form
of utility functions or additional generated methods of the Any type.

Example 1: Pack and unpack a message in C++.

    Foo foo = ...;
    Any any;
    any.PackFrom(foo);
    ...
    if (any.UnpackTo(&foo)) {
      ...
    }

Example 2: Pack and unpack a message in Java.

    Foo foo = ...;
    Any any = Any.pack(foo);
    ...
    if (any.is(Foo.class)) {
      foo = any.unpack(Foo.class);
    }

 Example 3: Pack and unpack a message in Python.

    foo = Foo(...)
    any = Any()
    any.Pack(foo)
    ...
    if any.Is(Foo.DESCRIPTOR):
      any.Unpack(foo)
      ...

 Example 4: Pack and unpack a message in Go

     foo := &pb.Foo{...}
     any, err := anypb.New(foo)
     if err != nil {
       ...
     }
     ...
     foo := &pb.Foo{}
     if err := any.UnmarshalTo(foo); err != nil {
       ...
     }

The pack methods provided by protobuf library will by default use
'type.googleapis.com/full.type.name' as the type URL and the unpack
methods only use the fully qualified type name after the last '/'
in the type URL, for example "foo.bar.com/x/y.z" will yield type
name "y.z".


JSON
====
The JSON representation of an `Any` value uses the regular
representation of the deserialized, embedded message, with an
additional field `@type` which contains the type URL. Example:

    package google.profile;
    message Person {
      string first_name = 1;
      string last_name = 2;
    }

    {
      "@type": "type.googleapis.com/google.profile.Person",
      "firstName": <string>,
      "lastName": <string>
    }

If the embedded message type is well-known and has a custom JSON
representation, that representation will be embedded adding a field
`value` which holds the custom JSON in addition to the `@type`
field. Example (for message [google.protobuf.Duration][]):

    {
      "@type": "type.googleapis.com/google.protobuf.Duration",
      "value": "1.212s"
    }
*/
export interface ProtobufAny {
    /**
     * A URL/resource name that uniquely identifies the type of the serialized
     * protocol buffer message. This string must contain at least
     * one "/" character. The last segment of the URL's path must represent
     * the fully qualified name of the type (as in
     * `path/google.protobuf.Duration`). The name should be in a canonical form
     * (e.g., leading "." is not accepted).
     *
     * In practice, teams usually precompile into the binary all types that they
     * expect it to use in the context of Any. However, for URLs which use the
     * scheme `http`, `https`, or no scheme, one can optionally set up a type
     * server that maps type URLs to message definitions as follows:
     *
     * * If no scheme is provided, `https` is assumed.
     * * An HTTP GET on the URL must yield a [google.protobuf.Type][]
     *   value in binary format, or produce an error.
     * * Applications are allowed to cache lookup results based on the
     *   URL, or have them precompiled into a binary to avoid any
     *   lookup. Therefore, binary compatibility needs to be preserved
     *   on changes to types. (Use versioned type names to manage
     *   breaking changes.)
     *
     * Note: this functionality is not currently available in the official
     * protobuf release, and it is not used for type URLs beginning with
     * type.googleapis.com.
     *
     * Schemes other than `http`, `https` (or the empty scheme) might be
     * used with implementation specific semantics.
     */
    "@type"?: string;
}
/**
 * Params defines the parameters for the module.
 */
export declare type QuasarnodeintergammParams = object;
export interface RpcStatus {
    /** @format int32 */
    code?: number;
    message?: string;
    details?: ProtobufAny[];
}
/**
* Coin defines a token with a denomination and an amount.

NOTE: The amount field is an Int which implements the custom method
signatures required by gogoproto.
*/
export interface V1Beta1Coin {
    denom?: string;
    amount?: string;
}
export declare type QueryParamsType = Record<string | number, any>;
export declare type ResponseFormat = keyof Omit<Body, "body" | "bodyUsed">;
export interface FullRequestParams extends Omit<RequestInit, "body"> {
    /** set parameter to `true` for call `securityWorker` for this request */
    secure?: boolean;
    /** request path */
    path: string;
    /** content type of request body */
    type?: ContentType;
    /** query params */
    query?: QueryParamsType;
    /** format of response (i.e. response.json() -> format: "json") */
    format?: keyof Omit<Body, "body" | "bodyUsed">;
    /** request body */
    body?: unknown;
    /** base url */
    baseUrl?: string;
    /** request cancellation token */
    cancelToken?: CancelToken;
}
export declare type RequestParams = Omit<FullRequestParams, "body" | "method" | "query" | "path">;
export interface ApiConfig<SecurityDataType = unknown> {
    baseUrl?: string;
    baseApiParams?: Omit<RequestParams, "baseUrl" | "cancelToken" | "signal">;
    securityWorker?: (securityData: SecurityDataType) => RequestParams | void;
}
export interface HttpResponse<D extends unknown, E extends unknown = unknown> extends Response {
    data: D;
    error: E;
}
declare type CancelToken = Symbol | string | number;
export declare enum ContentType {
    Json = "application/json",
    FormData = "multipart/form-data",
    UrlEncoded = "application/x-www-form-urlencoded"
}
export declare class HttpClient<SecurityDataType = unknown> {
    baseUrl: string;
    private securityData;
    private securityWorker;
    private abortControllers;
    private baseApiParams;
    constructor(apiConfig?: ApiConfig<SecurityDataType>);
    setSecurityData: (data: SecurityDataType) => void;
    private addQueryParam;
    protected toQueryString(rawQuery?: QueryParamsType): string;
    protected addQueryParams(rawQuery?: QueryParamsType): string;
    private contentFormatters;
    private mergeRequestParams;
    private createAbortSignal;
    abortRequest: (cancelToken: CancelToken) => void;
    request: <T = any, E = any>({ body, secure, path, type, query, format, baseUrl, cancelToken, ...params }: FullRequestParams) => Promise<HttpResponse<T, E>>;
}
/**
 * @title intergamm/genesis.proto
 * @version version not set
 */
export declare class Api<SecurityDataType extends unknown> extends HttpClient<SecurityDataType> {
    /**
     * No description
     *
     * @tags Query
     * @name QueryInterchainAccountFromAddress
     * @summary Queries a list of InterchainAccountFromAddress items.
     * @request GET:/abag/quasarnode/intergamm/interchain_account_from_address
     */
    queryInterchainAccountFromAddress: (query?: {
        owner?: string;
        connectionId?: string;
    }, params?: RequestParams) => Promise<HttpResponse<IntergammQueryInterchainAccountFromAddressResponse, RpcStatus>>;
    /**
     * No description
     *
     * @tags Query
     * @name QueryParams
     * @summary Parameters queries the parameters of the module.
     * @request GET:/abag/quasarnode/intergamm/params
     */
    queryParams: (params?: RequestParams) => Promise<HttpResponse<IntergammQueryParamsResponse, RpcStatus>>;
}
export {};
