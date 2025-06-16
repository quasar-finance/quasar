import { CosmWasmClient } from "@cosmjs/cosmwasm-stargate";

interface ActiveUsersResponse {
  users: [string, string][]; // Array of [address, shares] tuples
  next_token?: string;
}

interface VaultExtensionQuery {
  vault_extension: {
    users: {
      start_bound_exclusive?: string;
      limit: number;
    };
  };
}

async function queryAllUsers(
  client: CosmWasmClient,
  contractAddress: string
): Promise<[string, string][]> {
  const allUsers: [string, string][] = [];
  let nextToken: string | undefined = undefined;
  const limit = 200;

  console.log("Querying all users from CL vault...");

  while (true) {
    const query: VaultExtensionQuery = {
      vault_extension: {
        users: {
          limit,
          ...(nextToken && { start_bound_exclusive: nextToken }),
        },
      },
    };

    try {
      const response = await client.queryContractSmart(
        contractAddress,
        query
      ) as ActiveUsersResponse;

      allUsers.push(...response.users);
      console.log(`Fetched ${response.users.length} users, total: ${allUsers.length}`);

      if (!response.next_token) {
        console.log("No more users to fetch");
        break;
      }

      nextToken = response.next_token;
    } catch (error) {
      console.error("Error querying users:", error);
      throw error;
    }
  }

  return allUsers;
}

function filterNonZeroShareUsers(users: [string, string][]): [string, string][] {
  return users.filter(([_, shares]) => shares !== "0");
}

async function main() {
  // Configuration
  const RPC_ENDPOINT = process.env.RPC_ENDPOINT || "https://osmosis-rpc.publicnode.com:443";
  const CONTRACT_ADDRESS = process.env.CONTRACT_ADDRESS;

  if (!CONTRACT_ADDRESS) {
    console.error("Please provide CONTRACT_ADDRESS environment variable");
    process.exit(1);
  }

  try {
    // Connect to the chain
    const client = await CosmWasmClient.connect(RPC_ENDPOINT);
    
    // Query all users
    const allUsers = await queryAllUsers(client, CONTRACT_ADDRESS);
    console.log(`\nTotal users found: ${allUsers.length}`);

    // Filter users with non-zero shares
    const activeUsers = filterNonZeroShareUsers(allUsers);
    console.log(`Users with non-zero shares: ${activeUsers.length}`);
    console.log(`Users with zero shares (filtered out): ${allUsers.length - activeUsers.length}`);

    // Output results
    console.log("\nActive users (address, shares):");
    for (const [address, shares] of activeUsers) {
      console.log(`${address}: ${shares}`);
    }

    // Optionally save to file
    if (process.env.OUTPUT_FILE) {
      const fs = await import("fs/promises");
      const output = {
        total_users: allUsers.length,
        active_users: activeUsers.length,
        filtered_users: allUsers.length - activeUsers.length,
        users: activeUsers.map(([address, shares]) => ({ address, shares })),
      };
      await fs.writeFile(
        process.env.OUTPUT_FILE,
        JSON.stringify(output, null, 2)
      );
      console.log(`\nResults saved to ${process.env.OUTPUT_FILE}`);
    }

  } catch (error) {
    console.error("Error:", error);
    process.exit(1);
  }
}

// Run the script
main();