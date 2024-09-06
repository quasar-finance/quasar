import { createMigrateMsg, createProposalMsg, getClient, readFileAsJSON, CONTRACTS, DAODAO } from './util/osmosis.js';

async function main() {
    const mnemonic = process.env.MNEMONIC;
    const client = await getClient(mnemonic);

    const migrate_msg = {
        // MIGRATION MSG ARGS
    };
    let contracts = readFileAsJSON(CONTRACTS);
    const newCodeId = -1 // CHANGE CODE ID;
    var msgs = [];
    for (var contract of contracts) {
        console.log(contract.name);
        const msg = createMigrateMsg(contract.address, newCodeId, migrate_msg);
        msgs.push(msg);
    }
    const title = "<ADD_TITLE>";
    const description = "<ADD DESCRIPTION>";
    const proposalMsg = createProposalMsg(title, description, msgs);
    console.log(proposalMsg.propose);

    const response = await client.execute(sender, DAODAO, proposalMsg, "auto", "", []);
    console.log(response);
}

main();
