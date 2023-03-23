import { getBalance } from './vault'

export async function expect_balance_increase(
  vaultAddress: string,
  alice: boolean,
  bob: boolean,
) {
  let start = new Date()
  let alice_balance_initial = await getBalance(vaultAddress, 'alice')
  let bob_balance_initial = await getBalance(vaultAddress, 'bob')

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
        (!alice ||
          Number(alice_balance.balance) >
            Number(alice_balance_initial.balance)) &&
        (!bob ||
          Number(bob_balance.balance) > Number(bob_balance_initial.balance))
      ) {
        console.log('\n=== Bond test passed ===')
        console.log(
          'Bond took ' + (new Date().getTime() - start.getTime()) / 1000 + 's',
        )
        clearInterval(interval)
        r()
      }
    }, 5000)
  })
}
