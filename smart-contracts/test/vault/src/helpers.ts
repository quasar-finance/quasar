import { PendingUnbondsResponse } from '../BasicVault.types'

export function get_max_unlock_time(unbonds_response: PendingUnbondsResponse) {
  let max = null
  for (const unbond of unbonds_response.pending_unbonds) {
    for (const stub of unbond.stub) {
      if (stub.unlock_time && Number(stub.unlock_time) > (max || 0)) {
        max = Number(stub.unlock_time)
      }
    }
  }
  return max
}
