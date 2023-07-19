import { OSMO_DENOM } from "./config";
import {
  bond,
  claim,
  getBalance,
  getChainBalance,
  getPendingUnbonds,
  start_unbond,
  try_icq,
} from "./vault";
import {
  expect_balance_increase,
  expect_chain_balance_increase,
  expect_unlock_time_passed,
} from "./verifier";

export async function seed_liquidity_from_alice(vaultAddress: string) {
  console.log("=== Seeding Liquidity from alice (bad solution) ===");

  let bond_result = await bond({
    from: "alice",
    vaultAddress,
    funds: [
      {
        amount: "50",
        denom: OSMO_DENOM,
      },
    ],
  });

  await expect_balance_increase(vaultAddress, true, false, false);
  console.log("Seed liq complete");
}

export async function stupid_test(vaultAddress: string) {
  console.log("=== Starting Stupid Test ===");

  let bond_result = await bond({
    from: "alice",
    vaultAddress,
    funds: [
      {
        amount: "50",
        denom: OSMO_DENOM,
      },
    ],
  });
  //   console.log('Bond result for alice:', JSON.stringify(bond_result, null, 2))

  await expect_balance_increase(vaultAddress, true, false, false);

  console.log("\n=== Start Stupid Unbond Test ===");
  let unbond_result = await start_unbond({
    from: "alice",
    vaultAddress,
    amount: "50",
  });
  //   console.log(
  //     'Start unbond result for alice:',
  //     JSON.stringify(unbond_result, null, 2),
  //   )

  setInterval(async () => {
    console.log("\nQuerying pending unbonds");
    let alice_pending_unbonds = await getPendingUnbonds(vaultAddress, "alice");
    console.log("Alice pending unbonds:", alice_pending_unbonds);
  }, 5000);
}

export async function simple_test(vaultAddress: string) {
  console.log("=== Starting Simple Bond Test ===");

  let bond_result = await bond({
    from: "alice",
    vaultAddress,
    funds: [
      {
        amount: "50",
        denom: OSMO_DENOM,
      },
    ],
  });
  //   console.log('Bond result for alice:', JSON.stringify(bond_result, null, 2))

  // let bond_result_2 = await bond({
  //   from: 'bob',
  //   vaultAddress,
  //   funds: [
  //     {
  //       amount: '50',
  //       denom: OSMO_DENOM,
  //     },
  //   ],
  // })
  //   console.log('Bond result for bob:', JSON.stringify(bond_result_2, null, 2))

  await expect_balance_increase(vaultAddress, true, true, false);

  console.log("\n=== Start Simple Start Unbond Test ===");
  let start_unbond_result = await start_unbond({
    from: "alice",
    vaultAddress,
    amount: "50",
  });
  //   console.log(
  //     'Start unbond result for alice:',
  //     JSON.stringify(start_unbond_result, null, 2),
  //   )
  await expect_unlock_time_passed(vaultAddress, true, false, false);

  // let start_unbond_result_2 = await start_unbond({
  //   from: "bob",
  //   vaultAddress,
  //   amount: "50",
  // });
  //   console.log(
  //     'Start unbond result for bob:',
  //     JSON.stringify(start_unbond_result_2, null, 2),
  //   )

  await expect_unlock_time_passed(vaultAddress, false, true, false);

  console.log("\n=== Start Simple Claim Test ===");
  await Promise.all([
    claim({ from: "alice", vaultAddress }),
    claim({ from: "bob", vaultAddress }),

    expect_chain_balance_increase(true, true, false),
  ]);

  console.log("=== Simple Test Complete ===");
}

export async function extreme_test(vaultAddress: string) {
  console.log("=== Starting Extreme Test ===");
  const alice_start_balance = await getChainBalance("alice");
  const bob_start_balance = await getChainBalance("bob");
  const charlie_start_balance = await getChainBalance("charlie");

  console.log("\nAlice start balance:", alice_start_balance.amount);
  console.log("Bob start balance:", bob_start_balance.amount);
  console.log("Charlie start balance:", charlie_start_balance.amount);
  console.log("==============================\n");

  console.log("## Start epoch 1 ###########################");
  await Promise.all([
    await bond({
      from: "alice",
      vaultAddress,
      funds: [
        {
          amount: "500000",
          denom: OSMO_DENOM,
        },
      ],
    }),
    await bond({
      from: "bob",
      vaultAddress,
      funds: [
        {
          amount: "5000",
          denom: OSMO_DENOM,
        },
      ],
    }),
    await bond({
      from: "charlie",
      vaultAddress,
      funds: [
        {
          amount: "250000",
          denom: OSMO_DENOM,
        },
      ],
    }),
  ]);

  await expect_balance_increase(vaultAddress, true, true, true);

  console.log("## End epoch 1 ###########################");
  console.log("## Start epoch 2 ###########################");

  await Promise.all([
    await start_unbond({
      from: "alice",
      vaultAddress,
      amount: "100000", // 40 after this
    }),
    await bond({
      from: "bob",
      vaultAddress,
      funds: [
        {
          amount: "3000", //total 80 after this
          denom: OSMO_DENOM,
        },
      ],
    }),
    await start_unbond({
      from: "charlie",
      vaultAddress,
      amount: "2500", //2475 after this
    }),
  ]);

  await Promise.all([
    await expect_unlock_time_passed(vaultAddress, true, false, true),
    await expect_balance_increase(vaultAddress, false, true, false),
  ]);

  console.log("## End epoch 2 ###########################");
  console.log("## Start epoch 3 ###########################");

  await Promise.all([
    await bond({
      from: "alice",
      vaultAddress,
      funds: [
        {
          amount: "2000000", // total 60 after this
          denom: OSMO_DENOM,
        },
      ],
    }),
    await start_unbond({
      from: "bob",
      vaultAddress,
      amount: "3000", // total 50 after this
    }),
    await bond({
      from: "charlie",
      vaultAddress,
      funds: [
        {
          amount: "25000", // total 2500 after this
          denom: OSMO_DENOM,
        },
      ],
    }),
  ]);

  await Promise.all([
    expect_balance_increase(vaultAddress, true, false, true),
    expect_unlock_time_passed(vaultAddress, false, true, false),
  ]);

  console.log("## End epoch 3 ###########################");
  console.log(
    "## Start epoch 4 ########################### (this is the hard one)"
  );

  await Promise.all([
    start_unbond({
      from: "alice",
      vaultAddress,
      amount: "10000",
    }),
    // await claim({ from: 'alice', vaultAddress }),
    // await claim({ from: 'bob', vaultAddress }),
    bond({
      from: "bob",
      vaultAddress,
      funds: [
        {
          amount: "2000",
          denom: OSMO_DENOM,
        },
      ],
    }),
    bond({
      from: "charlie",
      vaultAddress,
      funds: [
        {
          amount: "2500000", //total 5000 after this
          denom: OSMO_DENOM,
        },
      ],
    }),
  ]);

  await Promise.all([
    expect_balance_increase(vaultAddress, false, true, true),
    expect_unlock_time_passed(vaultAddress, true, false, false),
    // expect_chain_balance_increase(true, true, false),
  ]);

  console.log("## End epoch 4 ###########################");
  console.log("## Start epoch 5 ###########################");

  await Promise.all([
    // await claim({ from: 'alice', vaultAddress }),
    await start_unbond({ from: "bob", vaultAddress, amount: "30" }), // total 20 after this
    // await claim({ from: 'charlie', vaultAddress }),
  ]);

  await Promise.all([
    // await expect_chain_balance_increase(true, false, true),
    await expect_unlock_time_passed(vaultAddress, false, true, false),
  ]);

  console.log("## End epoch 5 ###########################");
  console.log("## Start epoch 6 ###########################");

  //   await Promise.all([await claim({ from: 'bob', vaultAddress })])

  //   await Promise.all([await expect_chain_balance_increase(false, true, false)])

  console.log("## End epoch 6 ###########################");

  const alice_end_balance = await getChainBalance("alice");
  const bob_end_balance = await getChainBalance("bob");
  const charlie_end_balance = await getChainBalance("charlie");

  console.log("\n=====================");
  console.log(
    "Alice balance change:",
    `start: ${alice_start_balance.amount}, end: ${
      alice_end_balance.amount
    }, diff: ${
      Number(alice_end_balance.amount) - Number(alice_start_balance.amount)
    }`
  );
  console.log(
    "Bob balance change:",
    `start: ${bob_start_balance.amount}, end: ${
      bob_end_balance.amount
    }, diff: ${
      Number(bob_end_balance.amount) - Number(bob_start_balance.amount)
    }`
  );
  console.log(
    "Charlie balance change:",
    `start: ${charlie_start_balance.amount}, end: ${
      charlie_end_balance.amount
    }, diff: ${
      Number(charlie_end_balance.amount) - Number(charlie_start_balance.amount)
    }`
  );
  console.log("=======================\n");

  console.log("TEST PASSED WTF (verify end funds vs start funds)");
}

export async function mayhem(vaultAddress: string) {
  const alice_start_balance = await getChainBalance("alice");
  const bob_start_balance = await getChainBalance("bob");
  const charlie_start_balance = await getChainBalance("charlie");

  console.log("\nAlice start balance:", alice_start_balance.amount);
  console.log("Bob start balance:", bob_start_balance.amount);
  console.log("Charlie start balance:", charlie_start_balance.amount);
  console.log("==============================\n");

  console.log("## Start epoch 1 ###########################");
  await Promise.all([
    bond({
      from: "alice",
      vaultAddress,
      funds: [
        {
          amount: "20000", //70
          denom: OSMO_DENOM,
        },
      ],
    }),
    bond({
      from: "bob",
      vaultAddress,
      funds: [
        {
          amount: "500", //50
          denom: OSMO_DENOM,
        },
      ],
    }),
    bond({
      from: "charlie",
      vaultAddress,
      funds: [
        {
          amount: "25000", //2500
          denom: OSMO_DENOM,
        },
      ],
    }),
  ]);

  await expect_balance_increase(vaultAddress, true, true, true);

  console.log("## End epoch 1 ###########################");
  console.log("## Start epoch 2 ###########################");

  await Promise.all([
    bond({
      from: "alice",
      vaultAddress,
      funds: [
        {
          amount: "30000", //100
          denom: OSMO_DENOM,
        },
      ],
    }),
    bond({
      from: "bob",
      vaultAddress,
      funds: [
        {
          amount: "300", //80
          denom: OSMO_DENOM,
        },
      ],
    }),
    bond({
      from: "charlie",
      vaultAddress,
      funds: [
        {
          amount: "10000", //3500
          denom: OSMO_DENOM,
        },
      ], //2475 after this
    }),
  ]);

  await expect_balance_increase(vaultAddress, true, true, true);
  // then 2 do bond and one does try icq

  console.log("## End epoch 2 ###########################");
  console.log("## Start epoch 3 ###########################");

  await Promise.all([
    bond({
      from: "alice",
      vaultAddress,
      funds: [
        {
          amount: "100000", //200
          denom: OSMO_DENOM,
        },
      ], //2475 after this
    }),
    start_unbond({
      from: "bob",
      vaultAddress,
      amount: "300", //50
    }),
    bond({
      from: "charlie",
      vaultAddress,
      funds: [
        {
          amount: "10000", //4500
          denom: OSMO_DENOM,
        },
      ], //2475 after this
    }),
  ]);

  await Promise.all([
    expect_unlock_time_passed(vaultAddress, false, true, false),
    expect_balance_increase(vaultAddress, true, false, true),
  ]);

  console.log("## End epoch 3 ###########################");
  console.log("## Start epoch 4 ###########################");

  // then the other two do bond and one does try icq
  await Promise.all([
    start_unbond({
      from: "alice",
      vaultAddress,
      amount: "30000", // 170
    }),
    bond({
      from: "bob",
      vaultAddress,
      funds: [
        {
          amount: "2200", //270
          denom: OSMO_DENOM,
        },
      ],
    }),
    start_unbond({
      from: "charlie",
      vaultAddress,
      amount: "3000", //4470
    }),
  ]);

  await Promise.all([
    expect_unlock_time_passed(vaultAddress, true, false, true),
    expect_balance_increase(vaultAddress, false, true, false),
  ]);
  //   // then one final try icq to clear everything

  console.log("## End epoch 4 ###########################");
  console.log("## Start epoch 5 ###########################");

  await Promise.all([
    claim({ from: "alice", vaultAddress }),
    start_unbond({
      from: "bob",
      vaultAddress,
      amount: "900", //180
    }),
    claim({ from: "charlie", vaultAddress }),
  ]);
  // then one more for good measure

  await Promise.all([
    expect_chain_balance_increase(true, false, true),
    expect_unlock_time_passed(vaultAddress, false, true, false),
  ]);

  console.log("## End epoch 5 ###########################");
  console.log("## Start epoch 6 ###########################");

  await Promise.all([
    start_unbond({
      from: "alice",
      vaultAddress,
      amount: (await getBalance(vaultAddress, "alice")).balance,
    }),
    claim({ from: "bob", vaultAddress }),
    start_unbond({
      from: "charlie",
      vaultAddress,
      amount: (await getBalance(vaultAddress, "charlie")).balance,
    }),
  ]);

  await new Promise((r) => setTimeout(r, 5000));
  await Promise.all([
    expect_chain_balance_increase(false, true, false),
    expect_unlock_time_passed(vaultAddress, true, false, true),
  ]);

  console.log("## End epoch 6 ############################");
  console.log("## Start epoch 7 ###########################");

  await Promise.all([
    claim({ from: "alice", vaultAddress }),
    start_unbond({
      from: "bob",
      vaultAddress,
      amount: (await getBalance(vaultAddress, "bob")).balance,
    }),
    claim({ from: "charlie", vaultAddress }),
  ]);

  await new Promise((r) => setTimeout(r, 5000));
  await Promise.all([expect_chain_balance_increase(true, false, true)]);

  console.log("## End epoch 7 ###########################");
  console.log("## End epoch 8 ############################");

  await Promise.all([claim({ from: "bob", vaultAddress })]);

  await new Promise((r) => setTimeout(r, 5000));
  await Promise.all([expect_chain_balance_increase(false, true, false)]);

  console.log("## End epoch 8 ###########################");

  const alice_end_balance = await getBalance(vaultAddress, "alice");
  const bob_end_balance = await getBalance(vaultAddress, "bob");
  const charlie_end_balance = await getBalance(vaultAddress, "charlie");

  console.log("\n=====================");
  console.log(
    "Alice end:",
    `${alice_end_balance}, start: ${alice_start_balance}, diff: ${
      Number(alice_end_balance) - Number(alice_start_balance)
    }`
  );
  console.log(
    "Bob end:",
    `${bob_end_balance}, start: ${bob_start_balance}, diff: ${
      Number(bob_end_balance) - Number(bob_start_balance)
    }`
  );
  console.log(
    "Charlie end:",
    `${charlie_end_balance}, start: ${charlie_start_balance}, diff: ${
      Number(charlie_end_balance) - Number(charlie_start_balance)
    }`
  );
  console.log("=====================\n");
}
