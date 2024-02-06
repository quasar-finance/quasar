/** 
 * @module MerkleTree - generates a MerkleTree from a list of data
 * 
 * @author Lucas Armstrong - Lucas@throneit.com - github.com/LucasArmstrong 
 */

/**
 * @import HashAlgorithm - Enum of hash algorithms
 * @import MerkleHash - Hash utlities for MerkleTree
 */
import { HashAlgorithm, MerkleHash } from "./MerkleHash";

/**
 * @type MerkleDataType - Union type containing the primitives handled by MerkleTree
 */
export type MerkleDataType = string | number | boolean | object;

/**
 * @interface IMerkleTree 
 */
export interface IMerkleTree {
    root: string;
    createHash(data: MerkleDataType): string;
    addNode(data: MerkleDataType): void;
    addNodes(dataArray: MerkleDataType[]): void;
}

/**
 * @class MerkleTree
 */
export class MerkleTree implements IMerkleTree {

    /**
    * @var root - Merkle Root - contains the root hash calculated for the data values provided
    */
    private _root: string = '';
    public get root(): string {
        return this._root;
    }
    private set root(value: string) {
        this._root = value;
    }

    /**
    * @var type - Hash Type - the type of one way hash algorithm the MerkleTree uses to build hashes
    */
    private type: HashAlgorithm;
    
    /**
    * @var _hashRecords - Hash Log - a list of hashes generated while calculating the merkle root
    */
    private _hashRecords: string[][] = [];
    public get hashRecords(): string[][] {
        return this._hashRecords;
    }

    /**
     * @var dataArray - stores a copy of the MerkleDataType[] that the tree is populated by
     */
    private _dataArray: MerkleDataType[] = [];
    public get dataArray(): MerkleDataType[] {
        return this._dataArray;
    }

    /**
     * @var dataHashIndex - A hash key index that points to a data's correpsonding dataArray index
     */
    private dataHashIndex: {[hashKey: string]: number} = {};

    /**
     * 
     * @param dataArray MerkleDataType[] - A list of data used to calculate the Merkle Root of type MerkleDataType
     * @param type string - The type of one way hash algorithm used to generate hashes
     */
    constructor(dataArray: MerkleDataType[], type: HashAlgorithm = HashAlgorithm.sha256) {
        this.type = type;
        this._dataArray = dataArray;
        this.buildTree();
    }



    /**
     * @method addNode - adds a new data node and rebuilds the tree
     * 
     * @param data 
     */
    public addNode(data: MerkleDataType): string {
        this._dataArray.push(data);
        this._hashRecords[0].push(this.createHash(data));

        let parentIndex = this._dataArray.length-1;
        let leftHash = '';
        let rightHash = '';

        if (this._dataArray.length <= 2) {
            leftHash = this._hashRecords[0][parentIndex-1];
            rightHash = this._hashRecords[0][parentIndex];
            const newHash = this.createHash(leftHash + rightHash);
            this._hashRecords[1] = [newHash];
            this.root = this._hashRecords[1][0];
        } else {
            let recordIndex = 1;
            while (parentIndex > 0) {
                const previousIndex = parentIndex;
                const odd = parentIndex & 1; // parentIndex % 2 === 0;

                if (odd) {
                    // parentIndex -= 1;
                    parentIndex = (parentIndex << 1) + (~parentIndex);
                    // parentIndex /= 2;
                    parentIndex = parentIndex >> 1;

                    leftHash = this._hashRecords[recordIndex - 1][previousIndex - 1];
                    rightHash = this._hashRecords[recordIndex - 1][previousIndex];
                } else {
                    // parentIndex /= 2;
                    parentIndex = parentIndex >> 1;

                    leftHash = this._hashRecords[recordIndex - 1][previousIndex];
                    rightHash = leftHash;
                }

                if (!this._hashRecords[recordIndex]) {
                    this._hashRecords[recordIndex] = [];
                }
                this._hashRecords[recordIndex][parentIndex] = this.createHash(leftHash + rightHash);

                if (parentIndex === 0) {
                    this.root = this._hashRecords[recordIndex][parentIndex];
                }
                
                recordIndex = (-(~recordIndex));
            }
            
        }

        return this.root;
    }

    /**
     * @method addNodes - adds multiple new data nodes and rebuilds the tree
     * 
     * @param addDataArray 
     */
    public addNodes(addDataArray: MerkleDataType[]): string {
        addDataArray.forEach((data) => {
           this.addNode(data);
        });
        return this.root;
    }

    /**
     * @method updateNodeAt - update a data node at index and rebuild hash path to the root hash
     * 
     * @param index
     * @param data 
     * @returns boolean 
     */
    public updateNodeAt(index: number, data: MerkleDataType): boolean {
        if (index >= this._dataArray.length) {
            return false;
        }
        this._dataArray[index] = data;
        this._hashRecords[0][index] = this.createHash(data);
        for (let i = 1; i < this._hashRecords.length; i++) {
            const previousIndex = index;
            let leftHash = '';
            let rightHash = '';

            if (index & 1) { // index % 2 !== 0
                // index -= 1;
                index = (index << 1) + (~index);
                // index /= 2;
                index = index >> 1;

                leftHash = this._hashRecords[i - 1][previousIndex - 1];
                rightHash = this._hashRecords[i - 1][previousIndex];
            } else {
                // index /= 2;
                index = index >> 1;

                leftHash = this._hashRecords[i - 1][previousIndex];
                rightHash = this._hashRecords[i - 1][previousIndex + 1] ?? leftHash;
            }
            
            this._hashRecords[i][index] = this.createHash(leftHash + rightHash);

            if (index === 0) {
                this.root = this._hashRecords[i][index];
            }
        }

        return true;
    }

    /**
     * @method createHash - takes a MerkleDataType payload to generate a
     * 
     * @param data MerkleDataType - The value used to generate a hash
     * @returns {string}
     */
    public createHash(data: MerkleDataType): string {
        return MerkleHash.createHash(data, this.type);
    }

    /**
     * @method getDataFromHash - returns the MerkleDataType associated with the given hash key
     * 
     * @param hashKey
     * @returns MerkleDataType | null
     */
    public getDataFromHash(hashKey: string): MerkleDataType | null {
        if (typeof this.dataHashIndex[hashKey] !== undefined) {
            return this._dataArray[this.dataHashIndex[hashKey]] ?? null;
        }
        return null;
    }

    /**
     * @method buildTree - breaks the data down into hashes then processes to find root of tree
     * 
     * @param dataArray MerkleDataType[] - The values used to generate the Merkle Tree
     */
    private buildTree(): void {
        if (!this._dataArray.length) {
            throw new Error('dataArray has a minimum length of 1');
        }

        this._hashRecords = [];
        this.dataHashIndex = {};

        if (this._dataArray.length > 1) {
            const hashed: string[] = [];
            for (let i = 0; i < this._dataArray.length; i++) {
                const eleHash = this.createHash(this._dataArray[i]);
                hashed.push(eleHash);
                this.dataHashIndex[eleHash] = i;
            }
            this._hashRecords.push(hashed);
            this.root = this.process(hashed);
        } else if (this._dataArray.length === 1) {
            this.root = this.createHash(this._dataArray[0]);
            this._hashRecords.push([this.root]);
        }
    }

    /**
     * @method process - recursively breaks down a list of hashes into nodes until the root is found (a single hash)
     * 
     * @param hashArray string[] - Array of hashes to calculate the Merkle Root from
     * @returns {string}
     */
    private process(hashArray: string[]): string {
        if (!hashArray.length) {
            throw new Error('hashArray has a minimum length of 1');
        }

        // digest elements from the hash array to create the nodes
        const hashed: string[] = [];
        let hashIndex = 0;
        while (hashIndex <= hashArray.length - 1) {
            if (hashArray.length - hashIndex > 1) {
                hashed.push(this.createHash(hashArray[hashIndex++] + hashArray[hashIndex++]));
            } else if (hashArray.length - hashIndex === 1) {
                hashed.push(this.createHash(hashArray[hashIndex] + hashArray[hashIndex++]));
            }
        }
        
        // track the hashes processed for this step
        this._hashRecords.push(hashed);

        // more than one hash means we can process another step
        if (hashed.length > 1) {
            return this.process(hashed);
        }

        // one hash means the root has been found
        return hashed[0];
    }

    /**
     * @method maxDepthFromDataArray - Calculate the maximum depth of the MerkleTree from the length of the dataArray
     * 
     * @param dataArray
     */
    public static maxDepthFromDataArray(dataArray: MerkleDataType[]): number {
        let currentLength = dataArray.length;
        let depth = 1;

        // currently testing performance of bitwise operations
        while (currentLength > 1) {
            //if (currentLength % 2 !== 0) {
            if (currentLength & 1) {
                // currentLength++;
                currentLength = (-(~currentLength));
            }
            // currentLength = currentLength / 2;
            currentLength = currentLength >> 1;
            //depth++;
            depth = (-(~depth));
        }

        return depth;
    }

}