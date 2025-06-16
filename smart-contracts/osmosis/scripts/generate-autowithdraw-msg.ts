import * as fs from 'fs/promises';

interface ActiveUsersData {
  total_users: number;
  active_users: number;
  filtered_users: number;
  users: Array<{
    address: string;
    shares: string;
  }>;
}

interface AutoWithdrawMsg {
  vault_extension: {
    admin: {
      auto_withdraw: {
        users: Array<[string, string]>;
      };
    };
  };
}

async function generateAutoWithdrawMsg() {
  const inputFile = process.env.INPUT_FILE || 'active_users.json';
  const outputFile = process.env.OUTPUT_FILE || 'autowithdraw_msg.json';

  try {
    // Read the active users data
    const data = await fs.readFile(inputFile, 'utf-8');
    const activeUsersData: ActiveUsersData = JSON.parse(data);

    // Convert users to the required format: Vec<(String, Uint128)>
    const users: Array<[string, string]> = activeUsersData.users.map(user => [
      user.address,
      user.shares
    ]);

    // Create the execute message
    const executeMsg: AutoWithdrawMsg = {
      vault_extension: {
        admin: {
          auto_withdraw: {
            users: users
          }
        }
      }
    };

    // Write the output
    await fs.writeFile(outputFile, JSON.stringify(executeMsg, null, 2));
    
    console.log(`AutoWithdraw message generated successfully!`);
    console.log(`Input file: ${inputFile}`);
    console.log(`Output file: ${outputFile}`);
    console.log(`Number of users to withdraw: ${users.length}`);
    console.log(`\nMessage saved to ${outputFile}`);
    console.log(`\nYou can now copy the contents of ${outputFile} to use in your transaction.`);

    // Also display the message for immediate use
    console.log(`\n--- Execute Message ---`);
    console.log(JSON.stringify(executeMsg, null, 2));

  } catch (error) {
    if (error instanceof Error && 'code' in error && error.code === 'ENOENT') {
      console.error(`Error: Input file '${inputFile}' not found.`);
      console.error(`Please run the query script first to generate the active users file.`);
    } else {
      console.error('Error:', error);
    }
    process.exit(1);
  }
}

// Run the script
generateAutoWithdrawMsg();