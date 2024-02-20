import { MerkleChain } from '@App/merkle/MerkleChain';
import { MerkleTree } from '@App/merkle/MerkleTree';

describe ('MerkleChain', () => {
    test ('#validate chain root array', () => {
        const genesisAt: number = 123456789;
        const merkleChain: MerkleChain = new MerkleChain(['first', 'block']);
        merkleChain.genesisAt = genesisAt;
        merkleChain.addNode(['second', 'block']);
        expect(merkleChain.merkleRoot)
            .toBe('a04f47b831a9bc115a1a099de15c53ced7a008b19616dd450b66588566247d12');

        expect(merkleChain.toRootArray())
            .toEqual(["9480e01b0af985489b3e74160db898214c03cf4ab66f6014074128460175c876", 
                "8031b55085a33ddbd10f6789c4099b05d5639af760b3c46772020ddf801e5715"]);

        expect(new MerkleTree(merkleChain.toRootArray()).root)
            .toBe('a04f47b831a9bc115a1a099de15c53ced7a008b19616dd450b66588566247d12');
        
    });
});