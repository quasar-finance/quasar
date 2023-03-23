import { OSMO_DENOM } from './config'
import {
  bond,
  getBalances as getBalance,
  getPendingUnbonds,
  start_unbond,
} from './vault'

export async function simple_test(vaultAddress: string) {
  console.log('=== Starting Simple Bond Test ===')

  let bond_result = await bond({
    from: 'alice',
    vaultAddress,
    funds: [
      {
        amount: '50',
        denom: OSMO_DENOM,
      },
    ],
  })
  console.log('Bond result for alice:', JSON.stringify(bond_result, null, 2))

  let bond_result_2 = await bond({
    from: 'bob',
    vaultAddress,
    funds: [
      {
        amount: '50',
        denom: OSMO_DENOM,
      },
    ],
  })
  console.log('Bond result for bob:', JSON.stringify(bond_result_2, null, 2))

  await new Promise<void>((r) => {
    let interval = setInterval(async () => {
      console.log('\nQuerying balances')
      let alice_balance = await getBalance(vaultAddress, 'alice')
      let bob_balance = await getBalance(vaultAddress, 'bob')
      console.log(
        'Alice|Bob balance:',
        alice_balance.balance,
        '|',
        bob_balance.balance,
      )

      if (
        Number(alice_balance.balance) > 0 &&
        Number(bob_balance.balance) > 0
      ) {
        console.log('=== Bond test passed ===')
        clearInterval(interval)
        r()
      }
    }, 5000)
  })

  console.log('=== Start Simple Unbond Test ===')
  let unbond_result = await start_unbond({
    from: 'alice',
    vaultAddress,
    amount: '50',
  })
  console.log(
    'Unbond result for alice:',
    JSON.stringify(unbond_result, null, 2),
  )

  setInterval(async () => {
    console.log('\nQuerying pending unbonds')
    let alice_pending_unbonds = await getPendingUnbonds(vaultAddress, 'alice')
    console.log('Alice pending unbonds:', alice_pending_unbonds)
  }, 5000)
}
