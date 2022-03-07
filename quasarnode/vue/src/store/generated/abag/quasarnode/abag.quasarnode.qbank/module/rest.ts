/* eslint-disable */
/* tslint:disable */
/*
 * ---------------------------------------------------------------
 * ## THIS FILE WAS GENERATED VIA SWAGGER-TYPESCRIPT-API        ##
 * ##                                                           ##
 * ## AUTHOR: acacode                                           ##
 * ## SOURCE: https://github.com/acacode/swagger-typescript-api ##
 * ---------------------------------------------------------------
 */

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

/**
 * FeeData defines the data object for the fee collection fields.
 */
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

export enum QbankLockupTypes {
  Invalid = "Invalid",
  Days7 = "Days_7",
  Days21 = "Days_21",
  Months1 = "Months_1",
  Months3 = "Months_3",
}

export type QbankMsgClaimRewardsResponse = object;

export type QbankMsgRequestDepositResponse = object;

export type QbankMsgRequestWithdrawAllResponse = object;

export type QbankMsgRequestWithdrawResponse = object;

/**
 * Params defines the parameters for the module.
 */
export type QbankParams = object;

/**
 * QCoins defines encoding/decoding for the slice of sdk.coins to be used in KV stores.
 */
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
  /** FeeData defines the data object for the fee collection fields. */
  FeeData?: QbankFeeData;
}

export interface QbankQueryGetWithdrawResponse {
  /** Withdraw defines the withdraw object to be stored in the KV store. */
  Withdraw?: QbankWithdraw;
}

/**
 * QueryParamsResponse is response type for the Query/Params RPC method.
 */
export interface QbankQueryParamsResponse {
  /** params holds all the parameters of this module. */
  params?: QbankParams;
}

export interface QbankQueryUserClaimRewardsResponse {
  /** QCoins defines encoding/decoding for the slice of sdk.coins to be used in KV stores. */
  coins?: QbankQCoins;
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

export interface QbankQueryUserDenomWithdrawResponse {
  /** @format uint64 */
  amount?: string;
}

export interface QbankQueryUserDepositResponse {
  /** QCoins defines encoding/decoding for the slice of sdk.coins to be used in KV stores. */
  coins?: QbankQCoins;
}

export interface QbankQueryUserWithdrawResponse {
  /** QCoins defines encoding/decoding for the slice of sdk.coins to be used in KV stores. */
  coins?: QbankQCoins;
}

export interface QbankQueryWithdrableResponse {
  /**
   * Coin defines a token with a denomination and an amount.
   *
   * NOTE: The amount field is an Int which implements the custom method
   * signatures required by gogoproto.
   */
  coin?: V1Beta1Coin;
}

/**
 * Withdraw defines the withdraw object to be stored in the KV store.
 */
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

export type QueryParamsType = Record<string | number, any>;
export type ResponseFormat = keyof Omit<Body, "body" | "bodyUsed">;

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

export type RequestParams = Omit<FullRequestParams, "body" | "method" | "query" | "path">;

export interface ApiConfig<SecurityDataType = unknown> {
  baseUrl?: string;
  baseApiParams?: Omit<RequestParams, "baseUrl" | "cancelToken" | "signal">;
  securityWorker?: (securityData: SecurityDataType) => RequestParams | void;
}

export interface HttpResponse<D extends unknown, E extends unknown = unknown> extends Response {
  data: D;
  error: E;
}

type CancelToken = Symbol | string | number;

export enum ContentType {
  Json = "application/json",
  FormData = "multipart/form-data",
  UrlEncoded = "application/x-www-form-urlencoded",
}

export class HttpClient<SecurityDataType = unknown> {
  public baseUrl: string = "";
  private securityData: SecurityDataType = null as any;
  private securityWorker: null | ApiConfig<SecurityDataType>["securityWorker"] = null;
  private abortControllers = new Map<CancelToken, AbortController>();

  private baseApiParams: RequestParams = {
    credentials: "same-origin",
    headers: {},
    redirect: "follow",
    referrerPolicy: "no-referrer",
  };

  constructor(apiConfig: ApiConfig<SecurityDataType> = {}) {
    Object.assign(this, apiConfig);
  }

  public setSecurityData = (data: SecurityDataType) => {
    this.securityData = data;
  };

  private addQueryParam(query: QueryParamsType, key: string) {
    const value = query[key];

    return (
      encodeURIComponent(key) +
      "=" +
      encodeURIComponent(Array.isArray(value) ? value.join(",") : typeof value === "number" ? value : `${value}`)
    );
  }

  protected toQueryString(rawQuery?: QueryParamsType): string {
    const query = rawQuery || {};
    const keys = Object.keys(query).filter((key) => "undefined" !== typeof query[key]);
    return keys
      .map((key) =>
        typeof query[key] === "object" && !Array.isArray(query[key])
          ? this.toQueryString(query[key] as QueryParamsType)
          : this.addQueryParam(query, key),
      )
      .join("&");
  }

  protected addQueryParams(rawQuery?: QueryParamsType): string {
    const queryString = this.toQueryString(rawQuery);
    return queryString ? `?${queryString}` : "";
  }

  private contentFormatters: Record<ContentType, (input: any) => any> = {
    [ContentType.Json]: (input: any) =>
      input !== null && (typeof input === "object" || typeof input === "string") ? JSON.stringify(input) : input,
    [ContentType.FormData]: (input: any) =>
      Object.keys(input || {}).reduce((data, key) => {
        data.append(key, input[key]);
        return data;
      }, new FormData()),
    [ContentType.UrlEncoded]: (input: any) => this.toQueryString(input),
  };

  private mergeRequestParams(params1: RequestParams, params2?: RequestParams): RequestParams {
    return {
      ...this.baseApiParams,
      ...params1,
      ...(params2 || {}),
      headers: {
        ...(this.baseApiParams.headers || {}),
        ...(params1.headers || {}),
        ...((params2 && params2.headers) || {}),
      },
    };
  }

  private createAbortSignal = (cancelToken: CancelToken): AbortSignal | undefined => {
    if (this.abortControllers.has(cancelToken)) {
      const abortController = this.abortControllers.get(cancelToken);
      if (abortController) {
        return abortController.signal;
      }
      return void 0;
    }

    const abortController = new AbortController();
    this.abortControllers.set(cancelToken, abortController);
    return abortController.signal;
  };

  public abortRequest = (cancelToken: CancelToken) => {
    const abortController = this.abortControllers.get(cancelToken);

    if (abortController) {
      abortController.abort();
      this.abortControllers.delete(cancelToken);
    }
  };

  public request = <T = any, E = any>({
    body,
    secure,
    path,
    type,
    query,
    format = "json",
    baseUrl,
    cancelToken,
    ...params
  }: FullRequestParams): Promise<HttpResponse<T, E>> => {
    const secureParams = (secure && this.securityWorker && this.securityWorker(this.securityData)) || {};
    const requestParams = this.mergeRequestParams(params, secureParams);
    const queryString = query && this.toQueryString(query);
    const payloadFormatter = this.contentFormatters[type || ContentType.Json];

    return fetch(`${baseUrl || this.baseUrl || ""}${path}${queryString ? `?${queryString}` : ""}`, {
      ...requestParams,
      headers: {
        ...(type && type !== ContentType.FormData ? { "Content-Type": type } : {}),
        ...(requestParams.headers || {}),
      },
      signal: cancelToken ? this.createAbortSignal(cancelToken) : void 0,
      body: typeof body === "undefined" || body === null ? null : payloadFormatter(body),
    }).then(async (response) => {
      const r = response as HttpResponse<T, E>;
      r.data = (null as unknown) as T;
      r.error = (null as unknown) as E;

      const data = await response[format]()
        .then((data) => {
          if (r.ok) {
            r.data = data;
          } else {
            r.error = data;
          }
          return r;
        })
        .catch((e) => {
          r.error = e;
          return r;
        });

      if (cancelToken) {
        this.abortControllers.delete(cancelToken);
      }

      if (!response.ok) throw data;
      return data;
    });
  };
}

/**
 * @title qbank/common.proto
 * @version version not set
 */
export class Api<SecurityDataType extends unknown> extends HttpClient<SecurityDataType> {
  /**
   * No description
   *
   * @tags Query
   * @name QueryDepositAll
   * @summary Queries a list of Deposit items.
   * @request GET:/abag/quasarnode/qbank/deposit
   */
  queryDepositAll = (
    query?: {
      "pagination.key"?: string;
      "pagination.offset"?: string;
      "pagination.limit"?: string;
      "pagination.countTotal"?: boolean;
      "pagination.reverse"?: boolean;
    },
    params: RequestParams = {},
  ) =>
    this.request<QbankQueryAllDepositResponse, RpcStatus>({
      path: `/abag/quasarnode/qbank/deposit`,
      method: "GET",
      query: query,
      format: "json",
      ...params,
    });

  /**
   * No description
   *
   * @tags Query
   * @name QueryDeposit
   * @summary Queries a Deposit by id.
   * @request GET:/abag/quasarnode/qbank/deposit/{id}
   */
  queryDeposit = (id: string, params: RequestParams = {}) =>
    this.request<QbankQueryGetDepositResponse, RpcStatus>({
      path: `/abag/quasarnode/qbank/deposit/${id}`,
      method: "GET",
      format: "json",
      ...params,
    });

  /**
   * No description
   *
   * @tags Query
   * @name QueryFeeData
   * @summary Queries a FeeData by index.
   * @request GET:/abag/quasarnode/qbank/fee_data
   */
  queryFeeData = (params: RequestParams = {}) =>
    this.request<QbankQueryGetFeeDataResponse, RpcStatus>({
      path: `/abag/quasarnode/qbank/fee_data`,
      method: "GET",
      format: "json",
      ...params,
    });

  /**
   * No description
   *
   * @tags Query
   * @name QueryParams
   * @summary Parameters queries the parameters of the module.
   * @request GET:/abag/quasarnode/qbank/params
   */
  queryParams = (params: RequestParams = {}) =>
    this.request<QbankQueryParamsResponse, RpcStatus>({
      path: `/abag/quasarnode/qbank/params`,
      method: "GET",
      format: "json",
      ...params,
    });

  /**
   * No description
   *
   * @tags Query
   * @name QueryUserClaimRewards
   * @summary Queries a list of UserClaimRewards items.
   * @request GET:/abag/quasarnode/qbank/user_claim_rewards/{userAcc}
   */
  queryUserClaimRewards = (userAcc: string, params: RequestParams = {}) =>
    this.request<QbankQueryUserClaimRewardsResponse, RpcStatus>({
      path: `/abag/quasarnode/qbank/user_claim_rewards/${userAcc}`,
      method: "GET",
      format: "json",
      ...params,
    });

  /**
   * No description
   *
   * @tags Query
   * @name QueryUserDenomDeposit
   * @summary Queries a list of UserDenomDeposit items.
   * @request GET:/abag/quasarnode/qbank/user_denom_deposit/{userAcc}
   */
  queryUserDenomDeposit = (userAcc: string, query?: { denom?: string }, params: RequestParams = {}) =>
    this.request<QbankQueryUserDenomDepositResponse, RpcStatus>({
      path: `/abag/quasarnode/qbank/user_denom_deposit/${userAcc}`,
      method: "GET",
      query: query,
      format: "json",
      ...params,
    });

  /**
   * No description
   *
   * @tags Query
   * @name QueryUserDenomEpochLockupDeposit
   * @summary Queries a list of UserDenomEpochLockupDeposit items.
   * @request GET:/abag/quasarnode/qbank/user_denom_epoch_lockup_deposit/{userAcc}/{denom}/{epochDay}/{lockupType}
   */
  queryUserDenomEpochLockupDeposit = (
    userAcc: string,
    denom: string,
    epochDay: string[],
    lockupType: string,
    params: RequestParams = {},
  ) =>
    this.request<QbankQueryUserDenomEpochLockupDepositResponse, RpcStatus>({
      path: `/abag/quasarnode/qbank/user_denom_epoch_lockup_deposit/${userAcc}/${denom}/${epochDay}/${lockupType}`,
      method: "GET",
      format: "json",
      ...params,
    });

  /**
   * No description
   *
   * @tags Query
   * @name QueryUserDenomLockupDeposit
   * @summary Queries a list of UserDenomLockupDeposit items.
   * @request GET:/abag/quasarnode/qbank/user_denom_lockup_deposit/{userAcc}/{denom}/{lockupType}
   */
  queryUserDenomLockupDeposit = (userAcc: string, denom: string, lockupType: string, params: RequestParams = {}) =>
    this.request<QbankQueryUserDenomLockupDepositResponse, RpcStatus>({
      path: `/abag/quasarnode/qbank/user_denom_lockup_deposit/${userAcc}/${denom}/${lockupType}`,
      method: "GET",
      format: "json",
      ...params,
    });

  /**
   * No description
   *
   * @tags Query
   * @name QueryUserDenomWithdraw
   * @summary Queries a list of UserDenomWithdraw items.
   * @request GET:/abag/quasarnode/qbank/user_denom_withdraw/{userAcc}/{denom}
   */
  queryUserDenomWithdraw = (userAcc: string, denom: string, params: RequestParams = {}) =>
    this.request<QbankQueryUserDenomWithdrawResponse, RpcStatus>({
      path: `/abag/quasarnode/qbank/user_denom_withdraw/${userAcc}/${denom}`,
      method: "GET",
      format: "json",
      ...params,
    });

  /**
   * No description
   *
   * @tags Query
   * @name QueryUserDeposit
   * @summary Queries a list of UserDeposit items.
   * @request GET:/abag/quasarnode/qbank/user_deposit/{userAcc}
   */
  queryUserDeposit = (userAcc: string, params: RequestParams = {}) =>
    this.request<QbankQueryUserDepositResponse, RpcStatus>({
      path: `/abag/quasarnode/qbank/user_deposit/${userAcc}`,
      method: "GET",
      format: "json",
      ...params,
    });

  /**
   * No description
   *
   * @tags Query
   * @name QueryUserWithdraw
   * @summary Queries a list of UserWithdraw items.
   * @request GET:/abag/quasarnode/qbank/user_withdraw/{userAcc}
   */
  queryUserWithdraw = (userAcc: string, params: RequestParams = {}) =>
    this.request<QbankQueryUserWithdrawResponse, RpcStatus>({
      path: `/abag/quasarnode/qbank/user_withdraw/${userAcc}`,
      method: "GET",
      format: "json",
      ...params,
    });

  /**
   * No description
   *
   * @tags Query
   * @name QueryWithdrable
   * @summary Queries a list of Withdrable items.
   * @request GET:/abag/quasarnode/qbank/withdrable/{userAccount}/{denom}
   */
  queryWithdrable = (userAccount: string, denom: string, params: RequestParams = {}) =>
    this.request<QbankQueryWithdrableResponse, RpcStatus>({
      path: `/abag/quasarnode/qbank/withdrable/${userAccount}/${denom}`,
      method: "GET",
      format: "json",
      ...params,
    });

  /**
   * No description
   *
   * @tags Query
   * @name QueryWithdrawAll
   * @summary Queries a list of Withdraw items.
   * @request GET:/abag/quasarnode/qbank/withdraw
   */
  queryWithdrawAll = (
    query?: {
      "pagination.key"?: string;
      "pagination.offset"?: string;
      "pagination.limit"?: string;
      "pagination.countTotal"?: boolean;
      "pagination.reverse"?: boolean;
    },
    params: RequestParams = {},
  ) =>
    this.request<QbankQueryAllWithdrawResponse, RpcStatus>({
      path: `/abag/quasarnode/qbank/withdraw`,
      method: "GET",
      query: query,
      format: "json",
      ...params,
    });

  /**
   * No description
   *
   * @tags Query
   * @name QueryWithdraw
   * @summary Queries a Withdraw by id.
   * @request GET:/abag/quasarnode/qbank/withdraw/{id}
   */
  queryWithdraw = (id: string, params: RequestParams = {}) =>
    this.request<QbankQueryGetWithdrawResponse, RpcStatus>({
      path: `/abag/quasarnode/qbank/withdraw/${id}`,
      method: "GET",
      format: "json",
      ...params,
    });
}
