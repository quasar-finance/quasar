export interface ProtobufAny {
    "@type"?: string;
}
/**
 * Depsoit message object to be stored in the KV store.
 */
export interface QbankDeposit {
    /** @format uint64 */
    id?: string;
    riskProfile?: string;
    vaultID?: string;
    depositorAccAddress?: string;
    /**
     * Coin defines a token with a denomination and an amount.
     *
     * NOTE: The amount field is an Int which implements the custom method
     * signatures required by gogoproto.
     */
    coin?: V1Beta1Coin;
    lockupPeriod?: QbankLockupTypes;
}
export interface QbankFeeData {
    feeCollector?: string;
    fromAddress?: string;
    /**
     * Coin defines a token with a denomination and an amount.
     *
     * NOTE: The amount field is an Int which implements the custom method
     * signatures required by gogoproto.
     */
    fee?: V1Beta1Coin;
    /** @format uint64 */
    feeType?: string;
    /** @format uint64 */
    blockHeight?: string;
    memo?: string;
}
export declare enum QbankLockupTypes {
    Invalid = "Invalid",
    Days7 = "Days_7",
    Days21 = "Days_21",
    Months1 = "Months_1",
    Months3 = "Months_3"
}
export declare type QbankMsgClaimRewardsResponse = object;
export declare type QbankMsgRequestDepositResponse = object;
export declare type QbankMsgRequestWithdrawResponse = object;
/**
 * Params defines the parameters for the module.
 */
export declare type QbankParams = object;
export interface QbankQCoins {
    coins?: V1Beta1Coin[];
}
export interface QbankQueryAllDepositResponse {
    Deposit?: QbankDeposit[];
    /**
     * PageResponse is to be embedded in gRPC response messages where the
     * corresponding request message has used PageRequest.
     *
     *  message SomeResponse {
     *          repeated Bar results = 1;
     *          PageResponse page = 2;
     *  }
     */
    pagination?: V1Beta1PageResponse;
}
export interface QbankQueryAllWithdrawResponse {
    Withdraw?: QbankWithdraw[];
    /**
     * PageResponse is to be embedded in gRPC response messages where the
     * corresponding request message has used PageRequest.
     *
     *  message SomeResponse {
     *          repeated Bar results = 1;
     *          PageResponse page = 2;
     *  }
     */
    pagination?: V1Beta1PageResponse;
}
export interface QbankQueryGetDepositResponse {
    /** Depsoit message object to be stored in the KV store. */
    Deposit?: QbankDeposit;
}
export interface QbankQueryGetFeeDataResponse {
    FeeData?: QbankFeeData;
}
export interface QbankQueryGetWithdrawResponse {
    Withdraw?: QbankWithdraw;
}
/**
 * QueryParamsResponse is response type for the Query/Params RPC method.
 */
export interface QbankQueryParamsResponse {
    /** params holds all the parameters of this module. */
    params?: QbankParams;
}
export interface QbankQueryUserDenomDepositResponse {
    /** @format uint64 */
    amount?: string;
}
export interface QbankQueryUserDenomEpochLockupDepositResponse {
    /** @format uint64 */
    amount?: string;
}
export interface QbankQueryUserDenomLockupDepositResponse {
    /** @format uint64 */
    amount?: string;
}
export interface QbankQueryUserDepositResponse {
    coins?: QbankQCoins;
}
export interface QbankWithdraw {
    /** @format uint64 */
    id?: string;
    riskProfile?: string;
    vaultID?: string;
    depositorAccAddress?: string;
    /**
     * Coin defines a token with a denomination and an amount.
     *
     * NOTE: The amount field is an Int which implements the custom method
     * signatures required by gogoproto.
     */
    coin?: V1Beta1Coin;
}
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
/**
* message SomeRequest {
         Foo some_parameter = 1;
         PageRequest pagination = 2;
 }
*/
export interface V1Beta1PageRequest {
    /**
     * key is a value returned in PageResponse.next_key to begin
     * querying the next page most efficiently. Only one of offset or key
     * should be set.
     * @format byte
     */
    key?: string;
    /**
     * offset is a numeric offset that can be used when key is unavailable.
     * It is less efficient than using key. Only one of offset or key should
     * be set.
     * @format uint64
     */
    offset?: string;
    /**
     * limit is the total number of results to be returned in the result page.
     * If left empty it will default to a value to be set by each app.
     * @format uint64
     */
    limit?: string;
    /**
     * count_total is set to true  to indicate that the result set should include
     * a count of the total number of items available for pagination in UIs.
     * count_total is only respected when offset is used. It is ignored when key
     * is set.
     */
    countTotal?: boolean;
    /**
     * reverse is set to true if results are to be returned in the descending order.
     *
     * Since: cosmos-sdk 0.43
     */
    reverse?: boolean;
}
/**
* PageResponse is to be embedded in gRPC response messages where the
corresponding request message has used PageRequest.

 message SomeResponse {
         repeated Bar results = 1;
         PageResponse page = 2;
 }
*/
export interface V1Beta1PageResponse {
    /** @format byte */
    nextKey?: string;
    /** @format uint64 */
    total?: string;
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
 * @title qbank/common.proto
 * @version version not set
 */
export declare class Api<SecurityDataType extends unknown> extends HttpClient<SecurityDataType> {
    /**
     * No description
     *
     * @tags Query
     * @name QueryDepositAll
     * @summary Queries a list of Deposit items.
     * @request GET:/abag/quasarnode/qbank/deposit
     */
    queryDepositAll: (query?: {
        "pagination.key"?: string;
        "pagination.offset"?: string;
        "pagination.limit"?: string;
        "pagination.countTotal"?: boolean;
        "pagination.reverse"?: boolean;
    }, params?: RequestParams) => Promise<HttpResponse<QbankQueryAllDepositResponse, RpcStatus>>;
    /**
     * No description
     *
     * @tags Query
     * @name QueryDeposit
     * @summary Queries a Deposit by id.
     * @request GET:/abag/quasarnode/qbank/deposit/{id}
     */
    queryDeposit: (id: string, params?: RequestParams) => Promise<HttpResponse<QbankQueryGetDepositResponse, RpcStatus>>;
    /**
     * No description
     *
     * @tags Query
     * @name QueryFeeData
     * @summary Queries a FeeData by index.
     * @request GET:/abag/quasarnode/qbank/fee_data
     */
    queryFeeData: (params?: RequestParams) => Promise<HttpResponse<QbankQueryGetFeeDataResponse, RpcStatus>>;
    /**
     * No description
     *
     * @tags Query
     * @name QueryParams
     * @summary Parameters queries the parameters of the module.
     * @request GET:/abag/quasarnode/qbank/params
     */
    queryParams: (params?: RequestParams) => Promise<HttpResponse<QbankQueryParamsResponse, RpcStatus>>;
    /**
     * No description
     *
     * @tags Query
     * @name QueryUserDenomDeposit
     * @summary Queries a list of UserDenomDeposit items.
     * @request GET:/abag/quasarnode/qbank/user_denom_deposit/{userAcc}
     */
    queryUserDenomDeposit: (userAcc: string, query?: {
        denom?: string;
    }, params?: RequestParams) => Promise<HttpResponse<QbankQueryUserDenomDepositResponse, RpcStatus>>;
    /**
     * No description
     *
     * @tags Query
     * @name QueryUserDenomEpochLockupDeposit
     * @summary Queries a list of UserDenomEpochLockupDeposit items.
     * @request GET:/abag/quasarnode/qbank/user_denom_epoch_lockup_deposit/{userAcc}/{denom}/{epochDay}/{lockupType}
     */
    queryUserDenomEpochLockupDeposit: (userAcc: string, denom: string, epochDay: string[], lockupType: string, params?: RequestParams) => Promise<HttpResponse<QbankQueryUserDenomEpochLockupDepositResponse, RpcStatus>>;
    /**
     * No description
     *
     * @tags Query
     * @name QueryUserDenomLockupDeposit
     * @summary Queries a list of UserDenomLockupDeposit items.
     * @request GET:/abag/quasarnode/qbank/user_denom_lockup_deposit/{userAcc}/{denom}/{lockupType}
     */
    queryUserDenomLockupDeposit: (userAcc: string, denom: string, lockupType: string, params?: RequestParams) => Promise<HttpResponse<QbankQueryUserDenomLockupDepositResponse, RpcStatus>>;
    /**
     * No description
     *
     * @tags Query
     * @name QueryUserDeposit
     * @summary Queries a list of UserDeposit items.
     * @request GET:/abag/quasarnode/qbank/user_deposit/{userAcc}
     */
    queryUserDeposit: (userAcc: string, params?: RequestParams) => Promise<HttpResponse<QbankQueryUserDepositResponse, RpcStatus>>;
    /**
     * No description
     *
     * @tags Query
     * @name QueryWithdrawAll
     * @summary Queries a list of Withdraw items.
     * @request GET:/abag/quasarnode/qbank/withdraw
     */
    queryWithdrawAll: (query?: {
        "pagination.key"?: string;
        "pagination.offset"?: string;
        "pagination.limit"?: string;
        "pagination.countTotal"?: boolean;
        "pagination.reverse"?: boolean;
    }, params?: RequestParams) => Promise<HttpResponse<QbankQueryAllWithdrawResponse, RpcStatus>>;
    /**
     * No description
     *
     * @tags Query
     * @name QueryWithdraw
     * @summary Queries a Withdraw by id.
     * @request GET:/abag/quasarnode/qbank/withdraw/{id}
     */
    queryWithdraw: (id: string, params?: RequestParams) => Promise<HttpResponse<QbankQueryGetWithdrawResponse, RpcStatus>>;
}
export {};
