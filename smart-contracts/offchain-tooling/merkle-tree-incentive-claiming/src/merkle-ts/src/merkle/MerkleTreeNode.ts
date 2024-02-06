import { MerkleHash } from "./MerkleHash";
import { MerkleDataType } from "./MerkleTree";

export class MerkleTreeNode {

    left: MerkleTreeNode | null = null;
    right: MerkleTreeNode | null = null;
    parent: MerkleTreeNode | null = null;

    hash: string = '';

    // constructor(data: MerkleDataType | null = null) {
    //     this.update(data);
    // }

    // update(data: MerkleDataType | null = null) {
    //     if (this.left && this.right) {
    //         this.hash = MerkleHash.createHash(this.left.hash + this.right.hash);
    //     } else if (data !== null) {
    //         this.hash = MerkleHash.createHash(data);
    //     }
    //     return this.hash;
    // }
}