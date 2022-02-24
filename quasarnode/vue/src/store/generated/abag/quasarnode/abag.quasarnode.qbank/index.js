import { txClient, queryClient, MissingWalletError, registry } from './module';
// @ts-ignore
import { SpVuexError } from '@starport/vuex';
import { QCoins } from "./module/types/qbank/common";
import { QDenoms } from "./module/types/qbank/common";
import { Deposit } from "./module/types/qbank/deposit";
import { FeeData } from "./module/types/qbank/fee_data";
import { Params } from "./module/types/qbank/params";
import { Withdraw } from "./module/types/qbank/withdraw";
export { QCoins, QDenoms, Deposit, FeeData, Params, Withdraw };
async function initTxClient(vuexGetters) {
    return await txClient(vuexGetters['common/wallet/signer'], {
        addr: vuexGetters['common/env/apiTendermint']
    });
}
async function initQueryClient(vuexGetters) {
    return await queryClient({
        addr: vuexGetters['common/env/apiCosmos']
    });
}
function mergeResults(value, next_values) {
    for (let prop of Object.keys(next_values)) {
        if (Array.isArray(next_values[prop])) {
            value[prop] = [...value[prop], ...next_values[prop]];
        }
        else {
            value[prop] = next_values[prop];
        }
    }
    return value;
}
function getStructure(template) {
    let structure = { fields: [] };
    for (const [key, value] of Object.entries(template)) {
        let field = {};
        field.name = key;
        field.type = typeof value;
        structure.fields.push(field);
    }
    return structure;
}
const getDefaultState = () => {
    return {
        Params: {},
        Deposit: {},
        DepositAll: {},
        UserDenomDeposit: {},
        Withdraw: {},
        WithdrawAll: {},
        FeeData: {},
        UserDeposit: {},
        UserDenomLockupDeposit: {},
        UserDenomEpochLockupDeposit: {},
        UserWithdraw: {},
        UserDenomWithdraw: {},
        UserClaimRewards: {},
        _Structure: {
            QCoins: getStructure(QCoins.fromPartial({})),
            QDenoms: getStructure(QDenoms.fromPartial({})),
            Deposit: getStructure(Deposit.fromPartial({})),
            FeeData: getStructure(FeeData.fromPartial({})),
            Params: getStructure(Params.fromPartial({})),
            Withdraw: getStructure(Withdraw.fromPartial({})),
        },
        _Registry: registry,
        _Subscriptions: new Set(),
    };
};
// initial state
const state = getDefaultState();
export default {
    namespaced: true,
    state,
    mutations: {
        RESET_STATE(state) {
            Object.assign(state, getDefaultState());
        },
        QUERY(state, { query, key, value }) {
            state[query][JSON.stringify(key)] = value;
        },
        SUBSCRIBE(state, subscription) {
            state._Subscriptions.add(JSON.stringify(subscription));
        },
        UNSUBSCRIBE(state, subscription) {
            state._Subscriptions.delete(JSON.stringify(subscription));
        }
    },
    getters: {
        getParams: (state) => (params = { params: {} }) => {
            if (!params.query) {
                params.query = null;
            }
            return state.Params[JSON.stringify(params)] ?? {};
        },
        getDeposit: (state) => (params = { params: {} }) => {
            if (!params.query) {
                params.query = null;
            }
            return state.Deposit[JSON.stringify(params)] ?? {};
        },
        getDepositAll: (state) => (params = { params: {} }) => {
            if (!params.query) {
                params.query = null;
            }
            return state.DepositAll[JSON.stringify(params)] ?? {};
        },
        getUserDenomDeposit: (state) => (params = { params: {} }) => {
            if (!params.query) {
                params.query = null;
            }
            return state.UserDenomDeposit[JSON.stringify(params)] ?? {};
        },
        getWithdraw: (state) => (params = { params: {} }) => {
            if (!params.query) {
                params.query = null;
            }
            return state.Withdraw[JSON.stringify(params)] ?? {};
        },
        getWithdrawAll: (state) => (params = { params: {} }) => {
            if (!params.query) {
                params.query = null;
            }
            return state.WithdrawAll[JSON.stringify(params)] ?? {};
        },
        getFeeData: (state) => (params = { params: {} }) => {
            if (!params.query) {
                params.query = null;
            }
            return state.FeeData[JSON.stringify(params)] ?? {};
        },
        getUserDeposit: (state) => (params = { params: {} }) => {
            if (!params.query) {
                params.query = null;
            }
            return state.UserDeposit[JSON.stringify(params)] ?? {};
        },
        getUserDenomLockupDeposit: (state) => (params = { params: {} }) => {
            if (!params.query) {
                params.query = null;
            }
            return state.UserDenomLockupDeposit[JSON.stringify(params)] ?? {};
        },
        getUserDenomEpochLockupDeposit: (state) => (params = { params: {} }) => {
            if (!params.query) {
                params.query = null;
            }
            return state.UserDenomEpochLockupDeposit[JSON.stringify(params)] ?? {};
        },
        getUserWithdraw: (state) => (params = { params: {} }) => {
            if (!params.query) {
                params.query = null;
            }
            return state.UserWithdraw[JSON.stringify(params)] ?? {};
        },
        getUserDenomWithdraw: (state) => (params = { params: {} }) => {
            if (!params.query) {
                params.query = null;
            }
            return state.UserDenomWithdraw[JSON.stringify(params)] ?? {};
        },
        getUserClaimRewards: (state) => (params = { params: {} }) => {
            if (!params.query) {
                params.query = null;
            }
            return state.UserClaimRewards[JSON.stringify(params)] ?? {};
        },
        getTypeStructure: (state) => (type) => {
            return state._Structure[type].fields;
        },
        getRegistry: (state) => {
            return state._Registry;
        }
    },
    actions: {
        init({ dispatch, rootGetters }) {
            console.log('Vuex module: abag.quasarnode.qbank initialized!');
            if (rootGetters['common/env/client']) {
                rootGetters['common/env/client'].on('newblock', () => {
                    dispatch('StoreUpdate');
                });
            }
        },
        resetState({ commit }) {
            commit('RESET_STATE');
        },
        unsubscribe({ commit }, subscription) {
            commit('UNSUBSCRIBE', subscription);
        },
        async StoreUpdate({ state, dispatch }) {
            state._Subscriptions.forEach(async (subscription) => {
                try {
                    const sub = JSON.parse(subscription);
                    await dispatch(sub.action, sub.payload);
                }
                catch (e) {
                    throw new SpVuexError('Subscriptions: ' + e.message);
                }
            });
        },
        async QueryParams({ commit, rootGetters, getters }, { options: { subscribe, all } = { subscribe: false, all: false }, params, query = null }) {
            try {
                const key = params ?? {};
                const queryClient = await initQueryClient(rootGetters);
                let value = (await queryClient.queryParams()).data;
                commit('QUERY', { query: 'Params', key: { params: { ...key }, query }, value });
                if (subscribe)
                    commit('SUBSCRIBE', { action: 'QueryParams', payload: { options: { all }, params: { ...key }, query } });
                return getters['getParams']({ params: { ...key }, query }) ?? {};
            }
            catch (e) {
                throw new SpVuexError('QueryClient:QueryParams', 'API Node Unavailable. Could not perform query: ' + e.message);
            }
        },
        async QueryDeposit({ commit, rootGetters, getters }, { options: { subscribe, all } = { subscribe: false, all: false }, params, query = null }) {
            try {
                const key = params ?? {};
                const queryClient = await initQueryClient(rootGetters);
                let value = (await queryClient.queryDeposit(key.id)).data;
                commit('QUERY', { query: 'Deposit', key: { params: { ...key }, query }, value });
                if (subscribe)
                    commit('SUBSCRIBE', { action: 'QueryDeposit', payload: { options: { all }, params: { ...key }, query } });
                return getters['getDeposit']({ params: { ...key }, query }) ?? {};
            }
            catch (e) {
                throw new SpVuexError('QueryClient:QueryDeposit', 'API Node Unavailable. Could not perform query: ' + e.message);
            }
        },
        async QueryDepositAll({ commit, rootGetters, getters }, { options: { subscribe, all } = { subscribe: false, all: false }, params, query = null }) {
            try {
                const key = params ?? {};
                const queryClient = await initQueryClient(rootGetters);
                let value = (await queryClient.queryDepositAll(query)).data;
                while (all && value.pagination && value.pagination.next_key != null) {
                    let next_values = (await queryClient.queryDepositAll({ ...query, 'pagination.key': value.pagination.next_key })).data;
                    value = mergeResults(value, next_values);
                }
                commit('QUERY', { query: 'DepositAll', key: { params: { ...key }, query }, value });
                if (subscribe)
                    commit('SUBSCRIBE', { action: 'QueryDepositAll', payload: { options: { all }, params: { ...key }, query } });
                return getters['getDepositAll']({ params: { ...key }, query }) ?? {};
            }
            catch (e) {
                throw new SpVuexError('QueryClient:QueryDepositAll', 'API Node Unavailable. Could not perform query: ' + e.message);
            }
        },
        async QueryUserDenomDeposit({ commit, rootGetters, getters }, { options: { subscribe, all } = { subscribe: false, all: false }, params, query = null }) {
            try {
                const key = params ?? {};
                const queryClient = await initQueryClient(rootGetters);
                let value = (await queryClient.queryUserDenomDeposit(key.userAcc, query)).data;
                while (all && value.pagination && value.pagination.next_key != null) {
                    let next_values = (await queryClient.queryUserDenomDeposit(key.userAcc, { ...query, 'pagination.key': value.pagination.next_key })).data;
                    value = mergeResults(value, next_values);
                }
                commit('QUERY', { query: 'UserDenomDeposit', key: { params: { ...key }, query }, value });
                if (subscribe)
                    commit('SUBSCRIBE', { action: 'QueryUserDenomDeposit', payload: { options: { all }, params: { ...key }, query } });
                return getters['getUserDenomDeposit']({ params: { ...key }, query }) ?? {};
            }
            catch (e) {
                throw new SpVuexError('QueryClient:QueryUserDenomDeposit', 'API Node Unavailable. Could not perform query: ' + e.message);
            }
        },
        async QueryWithdraw({ commit, rootGetters, getters }, { options: { subscribe, all } = { subscribe: false, all: false }, params, query = null }) {
            try {
                const key = params ?? {};
                const queryClient = await initQueryClient(rootGetters);
                let value = (await queryClient.queryWithdraw(key.id)).data;
                commit('QUERY', { query: 'Withdraw', key: { params: { ...key }, query }, value });
                if (subscribe)
                    commit('SUBSCRIBE', { action: 'QueryWithdraw', payload: { options: { all }, params: { ...key }, query } });
                return getters['getWithdraw']({ params: { ...key }, query }) ?? {};
            }
            catch (e) {
                throw new SpVuexError('QueryClient:QueryWithdraw', 'API Node Unavailable. Could not perform query: ' + e.message);
            }
        },
        async QueryWithdrawAll({ commit, rootGetters, getters }, { options: { subscribe, all } = { subscribe: false, all: false }, params, query = null }) {
            try {
                const key = params ?? {};
                const queryClient = await initQueryClient(rootGetters);
                let value = (await queryClient.queryWithdrawAll(query)).data;
                while (all && value.pagination && value.pagination.next_key != null) {
                    let next_values = (await queryClient.queryWithdrawAll({ ...query, 'pagination.key': value.pagination.next_key })).data;
                    value = mergeResults(value, next_values);
                }
                commit('QUERY', { query: 'WithdrawAll', key: { params: { ...key }, query }, value });
                if (subscribe)
                    commit('SUBSCRIBE', { action: 'QueryWithdrawAll', payload: { options: { all }, params: { ...key }, query } });
                return getters['getWithdrawAll']({ params: { ...key }, query }) ?? {};
            }
            catch (e) {
                throw new SpVuexError('QueryClient:QueryWithdrawAll', 'API Node Unavailable. Could not perform query: ' + e.message);
            }
        },
        async QueryFeeData({ commit, rootGetters, getters }, { options: { subscribe, all } = { subscribe: false, all: false }, params, query = null }) {
            try {
                const key = params ?? {};
                const queryClient = await initQueryClient(rootGetters);
                let value = (await queryClient.queryFeeData()).data;
                commit('QUERY', { query: 'FeeData', key: { params: { ...key }, query }, value });
                if (subscribe)
                    commit('SUBSCRIBE', { action: 'QueryFeeData', payload: { options: { all }, params: { ...key }, query } });
                return getters['getFeeData']({ params: { ...key }, query }) ?? {};
            }
            catch (e) {
                throw new SpVuexError('QueryClient:QueryFeeData', 'API Node Unavailable. Could not perform query: ' + e.message);
            }
        },
        async QueryUserDeposit({ commit, rootGetters, getters }, { options: { subscribe, all } = { subscribe: false, all: false }, params, query = null }) {
            try {
                const key = params ?? {};
                const queryClient = await initQueryClient(rootGetters);
                let value = (await queryClient.queryUserDeposit(key.userAcc)).data;
                commit('QUERY', { query: 'UserDeposit', key: { params: { ...key }, query }, value });
                if (subscribe)
                    commit('SUBSCRIBE', { action: 'QueryUserDeposit', payload: { options: { all }, params: { ...key }, query } });
                return getters['getUserDeposit']({ params: { ...key }, query }) ?? {};
            }
            catch (e) {
                throw new SpVuexError('QueryClient:QueryUserDeposit', 'API Node Unavailable. Could not perform query: ' + e.message);
            }
        },
        async QueryUserDenomLockupDeposit({ commit, rootGetters, getters }, { options: { subscribe, all } = { subscribe: false, all: false }, params, query = null }) {
            try {
                const key = params ?? {};
                const queryClient = await initQueryClient(rootGetters);
                let value = (await queryClient.queryUserDenomLockupDeposit(key.userAcc, key.denom, key.lockupType)).data;
                commit('QUERY', { query: 'UserDenomLockupDeposit', key: { params: { ...key }, query }, value });
                if (subscribe)
                    commit('SUBSCRIBE', { action: 'QueryUserDenomLockupDeposit', payload: { options: { all }, params: { ...key }, query } });
                return getters['getUserDenomLockupDeposit']({ params: { ...key }, query }) ?? {};
            }
            catch (e) {
                throw new SpVuexError('QueryClient:QueryUserDenomLockupDeposit', 'API Node Unavailable. Could not perform query: ' + e.message);
            }
        },
        async QueryUserDenomEpochLockupDeposit({ commit, rootGetters, getters }, { options: { subscribe, all } = { subscribe: false, all: false }, params, query = null }) {
            try {
                const key = params ?? {};
                const queryClient = await initQueryClient(rootGetters);
                let value = (await queryClient.queryUserDenomEpochLockupDeposit(key.userAcc, key.denom, key.epochDay, key.lockupType)).data;
                commit('QUERY', { query: 'UserDenomEpochLockupDeposit', key: { params: { ...key }, query }, value });
                if (subscribe)
                    commit('SUBSCRIBE', { action: 'QueryUserDenomEpochLockupDeposit', payload: { options: { all }, params: { ...key }, query } });
                return getters['getUserDenomEpochLockupDeposit']({ params: { ...key }, query }) ?? {};
            }
            catch (e) {
                throw new SpVuexError('QueryClient:QueryUserDenomEpochLockupDeposit', 'API Node Unavailable. Could not perform query: ' + e.message);
            }
        },
        async QueryUserWithdraw({ commit, rootGetters, getters }, { options: { subscribe, all } = { subscribe: false, all: false }, params, query = null }) {
            try {
                const key = params ?? {};
                const queryClient = await initQueryClient(rootGetters);
                let value = (await queryClient.queryUserWithdraw(key.userAcc)).data;
                commit('QUERY', { query: 'UserWithdraw', key: { params: { ...key }, query }, value });
                if (subscribe)
                    commit('SUBSCRIBE', { action: 'QueryUserWithdraw', payload: { options: { all }, params: { ...key }, query } });
                return getters['getUserWithdraw']({ params: { ...key }, query }) ?? {};
            }
            catch (e) {
                throw new SpVuexError('QueryClient:QueryUserWithdraw', 'API Node Unavailable. Could not perform query: ' + e.message);
            }
        },
        async QueryUserDenomWithdraw({ commit, rootGetters, getters }, { options: { subscribe, all } = { subscribe: false, all: false }, params, query = null }) {
            try {
                const key = params ?? {};
                const queryClient = await initQueryClient(rootGetters);
                let value = (await queryClient.queryUserDenomWithdraw(key.userAcc, key.denom)).data;
                commit('QUERY', { query: 'UserDenomWithdraw', key: { params: { ...key }, query }, value });
                if (subscribe)
                    commit('SUBSCRIBE', { action: 'QueryUserDenomWithdraw', payload: { options: { all }, params: { ...key }, query } });
                return getters['getUserDenomWithdraw']({ params: { ...key }, query }) ?? {};
            }
            catch (e) {
                throw new SpVuexError('QueryClient:QueryUserDenomWithdraw', 'API Node Unavailable. Could not perform query: ' + e.message);
            }
        },
        async QueryUserClaimRewards({ commit, rootGetters, getters }, { options: { subscribe, all } = { subscribe: false, all: false }, params, query = null }) {
            try {
                const key = params ?? {};
                const queryClient = await initQueryClient(rootGetters);
                let value = (await queryClient.queryUserClaimRewards(key.userAcc)).data;
                commit('QUERY', { query: 'UserClaimRewards', key: { params: { ...key }, query }, value });
                if (subscribe)
                    commit('SUBSCRIBE', { action: 'QueryUserClaimRewards', payload: { options: { all }, params: { ...key }, query } });
                return getters['getUserClaimRewards']({ params: { ...key }, query }) ?? {};
            }
            catch (e) {
                throw new SpVuexError('QueryClient:QueryUserClaimRewards', 'API Node Unavailable. Could not perform query: ' + e.message);
            }
        },
        async sendMsgRequestWithdrawAll({ rootGetters }, { value, fee = [], memo = '' }) {
            try {
                const txClient = await initTxClient(rootGetters);
                const msg = await txClient.msgRequestWithdrawAll(value);
                const result = await txClient.signAndBroadcast([msg], { fee: { amount: fee,
                        gas: "200000" }, memo });
                return result;
            }
            catch (e) {
                if (e == MissingWalletError) {
                    throw new SpVuexError('TxClient:MsgRequestWithdrawAll:Init', 'Could not initialize signing client. Wallet is required.');
                }
                else {
                    throw new SpVuexError('TxClient:MsgRequestWithdrawAll:Send', 'Could not broadcast Tx: ' + e.message);
                }
            }
        },
        async sendMsgClaimRewards({ rootGetters }, { value, fee = [], memo = '' }) {
            try {
                const txClient = await initTxClient(rootGetters);
                const msg = await txClient.msgClaimRewards(value);
                const result = await txClient.signAndBroadcast([msg], { fee: { amount: fee,
                        gas: "200000" }, memo });
                return result;
            }
            catch (e) {
                if (e == MissingWalletError) {
                    throw new SpVuexError('TxClient:MsgClaimRewards:Init', 'Could not initialize signing client. Wallet is required.');
                }
                else {
                    throw new SpVuexError('TxClient:MsgClaimRewards:Send', 'Could not broadcast Tx: ' + e.message);
                }
            }
        },
        async sendMsgRequestDeposit({ rootGetters }, { value, fee = [], memo = '' }) {
            try {
                const txClient = await initTxClient(rootGetters);
                const msg = await txClient.msgRequestDeposit(value);
                const result = await txClient.signAndBroadcast([msg], { fee: { amount: fee,
                        gas: "200000" }, memo });
                return result;
            }
            catch (e) {
                if (e == MissingWalletError) {
                    throw new SpVuexError('TxClient:MsgRequestDeposit:Init', 'Could not initialize signing client. Wallet is required.');
                }
                else {
                    throw new SpVuexError('TxClient:MsgRequestDeposit:Send', 'Could not broadcast Tx: ' + e.message);
                }
            }
        },
        async sendMsgRequestWithdraw({ rootGetters }, { value, fee = [], memo = '' }) {
            try {
                const txClient = await initTxClient(rootGetters);
                const msg = await txClient.msgRequestWithdraw(value);
                const result = await txClient.signAndBroadcast([msg], { fee: { amount: fee,
                        gas: "200000" }, memo });
                return result;
            }
            catch (e) {
                if (e == MissingWalletError) {
                    throw new SpVuexError('TxClient:MsgRequestWithdraw:Init', 'Could not initialize signing client. Wallet is required.');
                }
                else {
                    throw new SpVuexError('TxClient:MsgRequestWithdraw:Send', 'Could not broadcast Tx: ' + e.message);
                }
            }
        },
        async MsgRequestWithdrawAll({ rootGetters }, { value }) {
            try {
                const txClient = await initTxClient(rootGetters);
                const msg = await txClient.msgRequestWithdrawAll(value);
                return msg;
            }
            catch (e) {
                if (e == MissingWalletError) {
                    throw new SpVuexError('TxClient:MsgRequestWithdrawAll:Init', 'Could not initialize signing client. Wallet is required.');
                }
                else {
                    throw new SpVuexError('TxClient:MsgRequestWithdrawAll:Create', 'Could not create message: ' + e.message);
                }
            }
        },
        async MsgClaimRewards({ rootGetters }, { value }) {
            try {
                const txClient = await initTxClient(rootGetters);
                const msg = await txClient.msgClaimRewards(value);
                return msg;
            }
            catch (e) {
                if (e == MissingWalletError) {
                    throw new SpVuexError('TxClient:MsgClaimRewards:Init', 'Could not initialize signing client. Wallet is required.');
                }
                else {
                    throw new SpVuexError('TxClient:MsgClaimRewards:Create', 'Could not create message: ' + e.message);
                }
            }
        },
        async MsgRequestDeposit({ rootGetters }, { value }) {
            try {
                const txClient = await initTxClient(rootGetters);
                const msg = await txClient.msgRequestDeposit(value);
                return msg;
            }
            catch (e) {
                if (e == MissingWalletError) {
                    throw new SpVuexError('TxClient:MsgRequestDeposit:Init', 'Could not initialize signing client. Wallet is required.');
                }
                else {
                    throw new SpVuexError('TxClient:MsgRequestDeposit:Create', 'Could not create message: ' + e.message);
                }
            }
        },
        async MsgRequestWithdraw({ rootGetters }, { value }) {
            try {
                const txClient = await initTxClient(rootGetters);
                const msg = await txClient.msgRequestWithdraw(value);
                return msg;
            }
            catch (e) {
                if (e == MissingWalletError) {
                    throw new SpVuexError('TxClient:MsgRequestWithdraw:Init', 'Could not initialize signing client. Wallet is required.');
                }
                else {
                    throw new SpVuexError('TxClient:MsgRequestWithdraw:Create', 'Could not create message: ' + e.message);
                }
            }
        },
    }
};
