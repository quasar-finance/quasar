import { CONTRACTS, readFileAsJSON, writeFileFromJSON } from './util/osmosis.js';

async function main() {
    const version = "0.3.1";
    const last_update = "X/Y/2024";
    let contracts = readFileAsJSON(CONTRACTS);
    for (var contract of contracts) {
        console.log(contract.name);
        if (contract.version == version) {
            continue;
        }
        contract.version = version;
        contract.last_update = last_update;
    }
    writeFileFromJSON(CONTRACTS, contracts);
}

main();
