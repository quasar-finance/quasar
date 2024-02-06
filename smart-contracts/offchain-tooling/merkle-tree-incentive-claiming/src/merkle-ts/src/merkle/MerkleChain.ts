/** 
 * @module MerkleChain - merkle tree list based data structure seeded with a genesis timestamp
 * 
 * @author Lucas Armstrong - Lucas@throneit.com - github.com/LucasArmstrong
 */

/**
 * @import MerkleDataType - all the data types used in MerkleTree
 * @import MerkleTree - generates a MerkleTree from a list of data
 */
import { MerkleDataType, MerkleTree } from "./MerkleTree";

/**
 * @type MerkleChainNodeNullable
 */
type MerkleChainNodeNullable = MerkleChainNode | null;

/**
 * @class MerkleChainNode
 * 
 * @param genesisAt number
 * @param dataArray MerkleDataType[]
 */
class MerkleChainNode {

    /**
     * @var next
     */
    private _next: MerkleChainNodeNullable = null;
    set next(nextNode: MerkleChainNodeNullable) {
        this._next = nextNode;
        if (nextNode) {
            this._nextRoot = nextNode.merkleRoot;
        } else {
            this._nextRoot = '';
        }
    }
    get next(): MerkleChainNodeNullable {
        return this._next;
    }

    /**
     * @var nextRoot
     */
    private _nextRoot: string = '';
    get nextRoot(): string {
        return this._nextRoot;
    }

    /**
     * @var prev
     */
    private _prev: MerkleChainNodeNullable = null;
    set prev(prevNode: MerkleChainNodeNullable) {
        this._prev = prevNode;
        if (prevNode) {
            this._prevRoot = prevNode.merkleRoot;
        } else {
            this._prevRoot = '';
        }
    }
    get prev(): MerkleChainNodeNullable {
        return this._prev;
    }

    /**
     * @var prevRoot
     */
    private _prevRoot: string = '';
    get prevRoot(): string {
        return this._prevRoot;
    }
    
    /**
     * @var merkleRoot
     */
    merkleRoot: string = '';

    /**
     * @var dataArray
     */
    dataArray: MerkleDataType[];

    constructor (genesisAt: number, dataArray: MerkleDataType[] = []) {
        const merkleTree: MerkleTree = new MerkleTree([genesisAt]);
        this.dataArray = dataArray;
        if (dataArray.length) {
            merkleTree.addNodes(dataArray);
        }
        this.merkleRoot = merkleTree.root;
    }
}

/**
 * @class MerkleChain
 * 
 * @param dataArray MerkleDataType[]
 */
export class MerkleChain {
    
    /**
     * @var head - pointer for the node at the start of the list
     */
    head: MerkleChainNodeNullable = null;

    /**
     * @var tail - pointer for the node at the end of the list
     */
    tail: MerkleChainNodeNullable = null;

    /**
     * @var genesisAt - timestamp used to seed the list
     */
    genesisAt: number = 0;

    /**
     * @var merkleRoot
     */
    private _merkleRoot: string = '';
    get merkleRoot(): string {
        return this._merkleRoot;
    }

    constructor(dataArray: MerkleDataType[]) {
        this.addNode(dataArray);
        this.genesisAt = new Date().getTime();
    }

    /**
     * @method addNode - creates a new node from a MerkleDataType array
     * 
     * @param dataArray MerkleDataType[] - array of MerkleDataType data that gets inserted into the list as a new node
     * @returns {void}
     */
    addNode(dataArray: MerkleDataType[] = []): void {
        this.newNode(new MerkleChainNode(this.genesisAt, dataArray));
        this._merkleRoot = new MerkleTree(this.toRootArray()).root;
    }

    /**
     * @method toRootArray - returns an array of the merkle roots from all nodes in the list
     * 
     * @returns {string[]}
     */
    toRootArray(): string[] {
        const values: string[] = [];
        let current: MerkleChainNodeNullable = this.head;
        while (current) {
            values.push(current.merkleRoot);
            current = current.next;
        }
        return values;
    }
    
    /**
     * @method newNode - manages inserting a new node to the list
     * 
     * @param node MerkleChainNode - node to be added to the list
     * @returns {MerkleChainNode}
     */
    private newNode(node: MerkleChainNode) : MerkleChainNode {
        if (!this.head) {
            this.head = node;
        } else if (!this.head.next) {
            this.head.next = node;
        }
        if (this.tail) {
            this.tail.next = node;
            node.prev = this.tail;
        } else {
            node.prev = this.head;
        }
        this.tail = node;

        return node;
    }

}