import prompts from 'prompts'
import { simple_test } from './vault/src/driver'

async function main() {
  let response = await prompts({
    type: 'text',
    name: 'vaultAddress',
    message: 'Enter the vault address',
  })
  console.log('vault addr:', response.vaultAddress)

  await simple_test(response.vaultAddress)
}

main()
