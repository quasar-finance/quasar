import prompts from 'prompts'
import {
  deposit_only,
  extreme_test,
  seed_liquidity_from_alice,
  simple_test,
} from './vault/src/driver'

async function main() {
  let response = await prompts({
    type: 'text',
    name: 'vaultAddress',
    message: 'Enter the vault address',
  })
  console.log('vault addr:', response.vaultAddress)

  await seed_liquidity_from_alice(response.vaultAddress)

  //   await simple_test(response.vaultAddress)
  //   await extreme_test(response.vaultAddress)
  await deposit_only(response.vaultAddress)
}

main()
