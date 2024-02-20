import { MerkleFileProcessor } from './src/merkle/MerkleFileProcessor';
import { HashAlgorithm } from './src/merkle/MerkleHash';
let directoryPath = process.env.ASSET_DIRECTORY_PATH || './exampleAssets';

try {
  (async () => {
    directoryPath = directoryPath.trim();
    const start = new Date().getTime();
    console.log(`\n\n------------------- Directory Processing Starting -------------------`);
    console.log(`directoryPath = ${directoryPath}`);
    const directoryMerkleRoot: string = await MerkleFileProcessor.processDirectory(directoryPath, HashAlgorithm.sha256);
    console.log(`directory merkle root for ${directoryPath}:`, directoryMerkleRoot);
    const end = new Date().getTime();
    const runTimeSeconds = (end - start) / 1000;
    console.log(`Directory processing complete in ${runTimeSeconds} seconds.`);
  })();
  
  } catch (err) {
    console.error(err);
}