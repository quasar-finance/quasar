/** 
 * @module MerkleFileProcessor - utilities to find the Merkle Root of a file or directory
 * 
 * @author Lucas Armstrong - Lucas@throneit.com - github.com/LucasArmstrong
 */

/**
 * @import fs.readdirSync
 * @import fs.statSync
 * @import HashAlgorithm - Enum of hash algorithms
 * @import MerkleHash - Hash utlities for MerkleTree
 * @import MerkleTree - generates a MerkleTree from a list of data
 */
import { readdirSync, statSync } from 'fs';
import { HashAlgorithm, MerkleHash } from './MerkleHash';
import { MerkleTree } from './MerkleTree';

/**
 * @class MerkleFileProcessor
 */
export class MerkleFileProcessor {

    /**
     * @method processFile - Generates a Merkle Root from a file buffer string
     * 
     * @param filePath 
     * @returns {string}
     */
    static async processFile(filePath: string, hashAlgorithm: HashAlgorithm = HashAlgorithm.sha256): Promise<string> {
        try {
            const fileHash: string = await MerkleHash.createHashFromFile(filePath, hashAlgorithm);
            const merkleTree: MerkleTree = new MerkleTree([fileHash]);
            // console.log(`Merkle root for ${filePath}: ${merkleTree.root}`);
            return merkleTree.root;
        } catch (error) {
            console.log(`processFile error for file path '${filePath}': `, error);
            return '';
        }
    }
    
    /**
     * @method processDirectory - Recursively searches directory for files to generate a Merkle Root
     * 
     * @param directoryPath 
     * @returns {string}
     */
    static async processDirectory(directoryPath: string, hashAlgorithm: HashAlgorithm = HashAlgorithm.sha256): Promise<string> {
        try {
            const fileHashRoots: string[] = [];
            const files: string[] = readdirSync(directoryPath);
            if (!files.length) {
                return '';
            }
            for (let i = 0; i < files.length; i++) {
                const file = files[i];
                const stat = statSync(`${directoryPath}/${file}`);
                if (stat?.isDirectory()) {
                    fileHashRoots.push(await MerkleFileProcessor.processDirectory(`${directoryPath}/${file}`, hashAlgorithm));
                } else {
                    fileHashRoots.push(await MerkleFileProcessor.processFile(`${directoryPath}/${file}`, hashAlgorithm));
                }
            }
            if (!fileHashRoots.length) {
                return '';
            }
            const merkleTree: MerkleTree = new MerkleTree(fileHashRoots);
            // console.log(`\nMerkle root for ${directoryPath}/: ${type} - ${merkleTree.root}`);
            return merkleTree.root;
        } catch (error) {
            console.log(`Unable to scan directory: ${directoryPath}`, error);
            throw new Error(`processDirectory Error: ${error}`);
        }
    }

}