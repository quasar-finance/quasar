import {
  getBalance,
  getChainBalance,
  getPendingUnbonds,
  try_icq,
} from './vault'
import TimeAgo from 'javascript-time-ago'
import en from 'javascript-time-ago/locale/en'
import { get_max_unlock_time } from './helpers'

TimeAgo.addDefaultLocale(en)
const ta = new TimeAgo('en-US')

export async function expect_balance_increase(
  vaultAddress: string,
  alice: boolean,
  bob: boolean,
  charlie: boolean,
) {
  let start = new Date()
  let alice_balance_initial = await getBalance(vaultAddress, 'alice')
  let bob_balance_initial = await getBalance(vaultAddress, 'bob')
  let charlie_balance_initial = await getBalance(vaultAddress, 'charlie')

  setTimeout(async () => {
    await try_icq({ vaultAddress, from: 'alice' })
  }, 20000)

  await new Promise<void>((r) => {
    let interval = setInterval(async () => {
      console.log('\nQuerying balances')
      let alice_balance = await getBalance(vaultAddress, 'alice')
      let bob_balance = await getBalance(vaultAddress, 'bob')
      let charlie_balance = await getBalance(vaultAddress, 'charlie')
      console.log(
        'Alice|Bob|Charlie balance:',
        alice_balance.balance,
        '|',
        bob_balance.balance,
        '|',
        charlie_balance.balance,
      )

      if (
        (!alice ||
          Number(alice_balance.balance) >
            Number(alice_balance_initial.balance)) &&
        (!bob ||
          Number(bob_balance.balance) > Number(bob_balance_initial.balance)) &&
        (!charlie ||
          Number(charlie_balance.balance) >
            Number(charlie_balance_initial.balance))
      ) {
        console.log('\n=== Bond test passed ===')
        console.log(
          'Bond took ' + (new Date().getTime() - start.getTime()) / 1000 + 's',
        )
        clearInterval(interval)
        r()
      }
      // if its been longer than 90 seconds, display warning message
      else if (new Date().getTime() - start.getTime() > 90000) {
        console.log(
          'WARNING: Bond test has been running for over 90 seconds. This may be a problem.',
        )
      }
    }, 5000)
  })
}

export async function expect_unlock_time_passed(
  vaultAddress: string,
  alice: boolean,
  bob: boolean,
  charlie: boolean,
) {
  const start = new Date()
  await new Promise<void>((r) => {
    setTimeout(async () => {
      await try_icq({ vaultAddress, from: 'alice' })
    }, 30000)

    let interval = setInterval(async () => {
      console.log('\nQuerying pending unbonds')
      let alice_pending_unbonds = await getPendingUnbonds(vaultAddress, 'alice')
      let bob_pending_unbonds = await getPendingUnbonds(vaultAddress, 'bob')
      let charlie_pending_unbonds = await getPendingUnbonds(
        vaultAddress,
        'charlie',
      )
      const alice_unlock_time =
        Number(get_max_unlock_time(alice_pending_unbonds)) / 1000000 //millis
      const bob_unlock_time =
        Number(get_max_unlock_time(bob_pending_unbonds)) / 1000000 //millis
      const charlie_unlock_time =
        Number(get_max_unlock_time(charlie_pending_unbonds)) / 1000000 //millis
      alice && console.log('Alice unlock_time:', ta.format(alice_unlock_time))
      bob && console.log('Bob unlock_time:', ta.format(bob_unlock_time))
      charlie &&
        console.log('Charlie unlock_time:', ta.format(charlie_unlock_time))

      if (
        (!alice ||
          (alice_unlock_time !== 0 &&
            alice_unlock_time < new Date().getTime())) &&
        (!bob ||
          (bob_unlock_time !== 0 && bob_unlock_time < new Date().getTime())) &&
        (!charlie ||
          (charlie_unlock_time !== 0 &&
            charlie_unlock_time < new Date().getTime()))
      ) {
        console.log('\n=== Start Simple Unbond test passed ===')
        console.log(
          'Start unbond took ' +
            (new Date().getTime() - start.getTime()) / 1000 +
            's',
        )
        console.log('Ready to unbond')
        clearInterval(interval)
        r()
      } // if its been longer than 90 seconds, display warning message
      else if (new Date().getTime() - start.getTime() > 90000) {
        console.log(
          'WARNING: Bond test has been running for over 90 seconds. This may be a problem.',
        )
      }
    }, 5000)
  })
}

export async function expect_chain_balance_increase(
  alice: boolean,
  bob: boolean,
  charlie: boolean,
) {
  const start = new Date()
  let orig_alice_balance = await getChainBalance('alice')
  let orig_bob_balance = await getChainBalance('bob')
  let orig_charlie_balance = await getChainBalance('charlie')

  await new Promise<void>(async (r) => {
    let interval = setInterval(async () => {
      console.log('\nQuerying claim result & balance')
      let alice_balance = await getChainBalance('alice')
      let bob_balance = await getChainBalance('bob')
      let charlie_balance = await getChainBalance('bob')

      alice &&
        console.log(
          'Alice balance orig/current:',
          orig_alice_balance.amount,
          '/',
          alice_balance.amount,
        )
      bob &&
        console.log(
          'Bob balance orig/current:',
          orig_bob_balance.amount,
          '/',
          bob_balance.amount,
        )
      charlie &&
        console.log(
          'Charlie balance orig/current:',
          orig_charlie_balance.amount,
          '/',
          charlie_balance.amount,
        )
      if (
        (!alice ||
          Number(alice_balance.amount) > Number(orig_alice_balance.amount)) &&
        (!bob ||
          Number(bob_balance.amount) > Number(orig_bob_balance.amount)) &&
        (!charlie ||
          Number(charlie_balance.amount) > Number(orig_charlie_balance.amount))
      ) {
        console.log('\n=== Simple Claim test passed ===')
        console.log(
          'Claim took ' + (new Date().getTime() - start.getTime()) / 1000 + 's',
        )
        clearInterval(interval)
        r()
      } else if (
        Number(alice_balance.amount) > Number(orig_alice_balance.amount) ||
        Number(bob_balance.amount) > Number(orig_bob_balance.amount) ||
        Number(charlie_balance.amount) > Number(orig_charlie_balance.amount)
      ) {
        // if just alice, run try_icq
        console.log('Only one balance increased. We may need to hit try icq')
      }
      // if its been longer than 90 seconds, display warning message
      if (new Date().getTime() - start.getTime() > 90000) {
        console.log(
          'WARNING: Bond test has been running for over 90 seconds. This may be a problem.',
        )
      }
    }, 5000)
  })
}
