import { txClient, queryClient, MissingWalletError , registry} from './module'
// @ts-ignore
import { SpVuexError } from '@starport/vuex'

import { IntergammPacketData } from "./module/types/intergamm/packet"
import { NoData } from "./module/types/intergamm/packet"
import { IbcCreatePoolPacketData } from "./module/types/intergamm/packet"
import { IbcCreatePoolPacketAck } from "./module/types/intergamm/packet"
import { IbcJoinPoolPacketData } from "./module/types/intergamm/packet"
import { IbcJoinPoolPacketAck } from "./module/types/intergamm/packet"
import { IbcExitPoolPacketData } from "./module/types/intergamm/packet"
import { IbcExitPoolPacketAck } from "./module/types/intergamm/packet"
import { IbcWithdrawPacketData } from "./module/types/intergamm/packet"
import { IbcWithdrawPacketAck } from "./module/types/intergamm/packet"


export { IntergammPacketData, NoData, IbcCreatePoolPacketData, IbcCreatePoolPacketAck, IbcJoinPoolPacketData, IbcJoinPoolPacketAck, IbcExitPoolPacketData, IbcExitPoolPacketAck, IbcWithdrawPacketData, IbcWithdrawPacketAck };

async function initTxClient(vuexGetters) {
	return await txClient(vuexGetters['common/wallet/signer'], {
		addr: vuexGetters['common/env/apiTendermint']
	})
}

async function initQueryClient(vuexGetters) {
	return await queryClient({
		addr: vuexGetters['common/env/apiCosmos']
	})
}

function mergeResults(value, next_values) {
	for (let prop of Object.keys(next_values)) {
		if (Array.isArray(next_values[prop])) {
			value[prop]=[...value[prop], ...next_values[prop]]
		}else{
			value[prop]=next_values[prop]
		}
	}
	return value
}

function getStructure(template) {
	let structure = { fields: [] }
	for (const [key, value] of Object.entries(template)) {
		let field: any = {}
		field.name = key
		field.type = typeof value
		structure.fields.push(field)
	}
	return structure
}

const getDefaultState = () => {
	return {
				
				_Structure: {
						IntergammPacketData: getStructure(IntergammPacketData.fromPartial({})),
						NoData: getStructure(NoData.fromPartial({})),
						IbcCreatePoolPacketData: getStructure(IbcCreatePoolPacketData.fromPartial({})),
						IbcCreatePoolPacketAck: getStructure(IbcCreatePoolPacketAck.fromPartial({})),
						IbcJoinPoolPacketData: getStructure(IbcJoinPoolPacketData.fromPartial({})),
						IbcJoinPoolPacketAck: getStructure(IbcJoinPoolPacketAck.fromPartial({})),
						IbcExitPoolPacketData: getStructure(IbcExitPoolPacketData.fromPartial({})),
						IbcExitPoolPacketAck: getStructure(IbcExitPoolPacketAck.fromPartial({})),
						IbcWithdrawPacketData: getStructure(IbcWithdrawPacketData.fromPartial({})),
						IbcWithdrawPacketAck: getStructure(IbcWithdrawPacketAck.fromPartial({})),
						
		},
		_Registry: registry,
		_Subscriptions: new Set(),
	}
}

// initial state
const state = getDefaultState()

export default {
	namespaced: true,
	state,
	mutations: {
		RESET_STATE(state) {
			Object.assign(state, getDefaultState())
		},
		QUERY(state, { query, key, value }) {
			state[query][JSON.stringify(key)] = value
		},
		SUBSCRIBE(state, subscription) {
			state._Subscriptions.add(JSON.stringify(subscription))
		},
		UNSUBSCRIBE(state, subscription) {
			state._Subscriptions.delete(JSON.stringify(subscription))
		}
	},
	getters: {
				
		getTypeStructure: (state) => (type) => {
			return state._Structure[type].fields
		},
		getRegistry: (state) => {
			return state._Registry
		}
	},
	actions: {
		init({ dispatch, rootGetters }) {
			console.log('Vuex module: intergamm initialized!')
			if (rootGetters['common/env/client']) {
				rootGetters['common/env/client'].on('newblock', () => {
					dispatch('StoreUpdate')
				})
			}
		},
		resetState({ commit }) {
			commit('RESET_STATE')
		},
		unsubscribe({ commit }, subscription) {
			commit('UNSUBSCRIBE', subscription)
		},
		async StoreUpdate({ state, dispatch }) {
			state._Subscriptions.forEach(async (subscription) => {
				try {
					const sub=JSON.parse(subscription)
					await dispatch(sub.action, sub.payload)
				}catch(e) {
					throw new SpVuexError('Subscriptions: ' + e.message)
				}
			})
		},
		
		
		
	}
}
