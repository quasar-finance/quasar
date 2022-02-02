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
export var QbankLockupTypes;
(function (QbankLockupTypes) {
    QbankLockupTypes["Invalid"] = "Invalid";
    QbankLockupTypes["Days7"] = "Days_7";
    QbankLockupTypes["Days21"] = "Days_21";
    QbankLockupTypes["Months1"] = "Months_1";
    QbankLockupTypes["Months3"] = "Months_3";
})(QbankLockupTypes || (QbankLockupTypes = {}));
export var ContentType;
(function (ContentType) {
    ContentType["Json"] = "application/json";
    ContentType["FormData"] = "multipart/form-data";
    ContentType["UrlEncoded"] = "application/x-www-form-urlencoded";
})(ContentType || (ContentType = {}));
export class HttpClient {
    constructor(apiConfig = {}) {
        this.baseUrl = "";
        this.securityData = null;
        this.securityWorker = null;
        this.abortControllers = new Map();
        this.baseApiParams = {
            credentials: "same-origin",
            headers: {},
            redirect: "follow",
            referrerPolicy: "no-referrer",
        };
        this.setSecurityData = (data) => {
            this.securityData = data;
        };
        this.contentFormatters = {
            [ContentType.Json]: (input) => input !== null && (typeof input === "object" || typeof input === "string") ? JSON.stringify(input) : input,
            [ContentType.FormData]: (input) => Object.keys(input || {}).reduce((data, key) => {
                data.append(key, input[key]);
                return data;
            }, new FormData()),
            [ContentType.UrlEncoded]: (input) => this.toQueryString(input),
        };
        this.createAbortSignal = (cancelToken) => {
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
        this.abortRequest = (cancelToken) => {
            const abortController = this.abortControllers.get(cancelToken);
            if (abortController) {
                abortController.abort();
                this.abortControllers.delete(cancelToken);
            }
        };
        this.request = ({ body, secure, path, type, query, format = "json", baseUrl, cancelToken, ...params }) => {
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
                const r = response;
                r.data = null;
                r.error = null;
                const data = await response[format]()
                    .then((data) => {
                    if (r.ok) {
                        r.data = data;
                    }
                    else {
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
                if (!response.ok)
                    throw data;
                return data;
            });
        };
        Object.assign(this, apiConfig);
    }
    addQueryParam(query, key) {
        const value = query[key];
        return (encodeURIComponent(key) +
            "=" +
            encodeURIComponent(Array.isArray(value) ? value.join(",") : typeof value === "number" ? value : `${value}`));
    }
    toQueryString(rawQuery) {
        const query = rawQuery || {};
        const keys = Object.keys(query).filter((key) => "undefined" !== typeof query[key]);
        return keys
            .map((key) => typeof query[key] === "object" && !Array.isArray(query[key])
            ? this.toQueryString(query[key])
            : this.addQueryParam(query, key))
            .join("&");
    }
    addQueryParams(rawQuery) {
        const queryString = this.toQueryString(rawQuery);
        return queryString ? `?${queryString}` : "";
    }
    mergeRequestParams(params1, params2) {
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
}
/**
 * @title qbank/common.proto
 * @version version not set
 */
export class Api extends HttpClient {
    constructor() {
        super(...arguments);
        /**
         * No description
         *
         * @tags Query
         * @name QueryDepositAll
         * @summary Queries a list of Deposit items.
         * @request GET:/abag/quasarnode/qbank/deposit
         */
        this.queryDepositAll = (query, params = {}) => this.request({
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
        this.queryDeposit = (id, params = {}) => this.request({
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
        this.queryFeeData = (params = {}) => this.request({
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
        this.queryParams = (params = {}) => this.request({
            path: `/abag/quasarnode/qbank/params`,
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
        this.queryUserDenomDeposit = (userAcc, query, params = {}) => this.request({
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
        this.queryUserDenomEpochLockupDeposit = (userAcc, denom, epochDay, lockupType, params = {}) => this.request({
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
        this.queryUserDenomLockupDeposit = (userAcc, denom, lockupType, params = {}) => this.request({
            path: `/abag/quasarnode/qbank/user_denom_lockup_deposit/${userAcc}/${denom}/${lockupType}`,
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
        this.queryUserDeposit = (userAcc, params = {}) => this.request({
            path: `/abag/quasarnode/qbank/user_deposit/${userAcc}`,
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
        this.queryWithdrawAll = (query, params = {}) => this.request({
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
        this.queryWithdraw = (id, params = {}) => this.request({
            path: `/abag/quasarnode/qbank/withdraw/${id}`,
            method: "GET",
            format: "json",
            ...params,
        });
    }
}
