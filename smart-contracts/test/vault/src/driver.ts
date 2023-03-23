import { OSMO_DENOM } from './config'
import {
  bond,
  claim,
  getBalance,
  getChainBalance,
  getPendingUnbonds,
  start_unbond,
} from './vault'
import {
  expect_balance_increase,
  expect_chain_balance_increase,
  expect_unlock_time_passed,
} from './verifier'

export async function seed_liquidity_from_alice(vaultAddress: string) {
  console.log('=== Seeding Liquidity from alice (bad solution) ===')

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

  await expect_balance_increase(vaultAddress, true, false)
  console.log('Seed liq complete')
}

export async function stupid_test(vaultAddress: string) {
  console.log('=== Starting Stupid Test ===')

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
  //   console.log('Bond result for alice:', JSON.stringify(bond_result, null, 2))

  await expect_balance_increase(vaultAddress, true, false)

  console.log('\n=== Start Stupid Unbond Test ===')
  let unbond_result = await start_unbond({
    from: 'alice',
    vaultAddress,
    amount: '50',
  })
  //   console.log(
  //     'Start unbond result for alice:',
  //     JSON.stringify(unbond_result, null, 2),
  //   )

  setInterval(async () => {
    console.log('\nQuerying pending unbonds')
    let alice_pending_unbonds = await getPendingUnbonds(vaultAddress, 'alice')
    console.log('Alice pending unbonds:', alice_pending_unbonds)
  }, 5000)
}

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
  //   console.log('Bond result for alice:', JSON.stringify(bond_result, null, 2))

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
  //   console.log('Bond result for bob:', JSON.stringify(bond_result_2, null, 2))

  await expect_balance_increase(vaultAddress, true, true)

  console.log('\n=== Start Simple Start Unbond Test ===')
  let start_unbond_result = await start_unbond({
    from: 'alice',
    vaultAddress,
    amount: '50',
  })
  //   console.log(
  //     'Start unbond result for alice:',
  //     JSON.stringify(start_unbond_result, null, 2),
  //   )

  let start_unbond_result_2 = await start_unbond({
    from: 'bob',
    vaultAddress,
    amount: '50',
  })
  //   console.log(
  //     'Start unbond result for bob:',
  //     JSON.stringify(start_unbond_result_2, null, 2),
  //   )

  await expect_unlock_time_passed(vaultAddress)

  console.log('\n=== Start Simple Claim Test ===')
  let claim_result = await claim({ from: 'alice', vaultAddress })
  let claim_result_2 = await claim({ from: 'bob', vaultAddress })

  await expect_chain_balance_increase()

  console.log('=== Simple Test Complete ===')
}
