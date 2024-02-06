# merkle-ts
- [MerkleTree](https://github.com/LucasArmstrong/merkle-ts/blob/main/src/merkle/MerkleTree.ts), 
[MerkleFileProcessor](https://github.com/LucasArmstrong/merkle-ts/blob/main/src/merkle/MerkleFileProcessor.ts), 
[MerkleHash](https://github.com/LucasArmstrong/merkle-ts/blob/main/src/merkle/MerkleHash.ts), 
and [MerkleChain](https://github.com/LucasArmstrong/merkle-ts/blob/main/src/merkle/MerkleChain.ts) 
in TypeScript
- Created by: Lucas Armstrong - Lucas@throneit.com - [github.com/LucasArmstrong](https://github.com/LucasArmstrong)
- License: `MIT`

## Install in your project with NPM and use it
- In your project run:
- >npm i https://github.com/LucasArmstrong/merkle-ts
- For TypeScript projects you can import the module MerkleTree
- For JavaScript projects you can build with:
- >npm run build
- - Then require in your code like:
- - >const {MerkleTree} = require('./node_modules/merkle-ts/dist/src/merkle/MerkleTree.js');
- - Example of using a merkle root in a jest test:
- - >let tree = new MerkleTree([1]); expect(tree.root).toBe('6b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b');

## Calculate a Merkle Root from a directory of files recursively
- First set or export environment variable `ASSET_DIRECTORY_PATH` to the directory you want to process.
- Then run:
- >npm run process
- Digests and calculates a Merkle Root for the entire specified directory and contents
- If you don't set the env var above, then the default directory is `exampleAssets`, the output should look like this:
- >------------------- Directory Processing Starting -------------------  
directoryPath = ./exampleAssets  
directory merkle root for ./exampleAssets: `d52c8079ae7fdaf0c455451ec1b8b729fa8de64602012062679126c90586bd7d`  
Directory processing complete in 0.024 seconds.

## Test
- Execute tests with:
- >npm run test
- Runs a variety of tests proving that the MerkleTree class correctly calculates the merkle root for a set of data

## Merkle Tree Info.
In cryptography and computer science, a hash tree or Merkle tree is a tree in which every "leaf" (node) is labelled with the cryptographic hash of a data block, and every node that is not a leaf (called a branch, inner node, or inode) is labelled with the cryptographic hash of the labels of its child nodes. A hash tree allows efficient and secure verification of the contents of a large data structure. A hash tree is a generalization of a hash list and a hash chain.
- Read more at https://en.wikipedia.org/wiki/Merkle_tree

## Available Hash Algorithms
`'RSA-MD4',
'RSA-MD5',
'RSA-MDC2',
'RSA-RIPEMD160',
'RSA-SHA1',
'RSA-SHA1-2',
'RSA-SHA224',
'RSA-SHA256',
'RSA-SHA3-224',
'RSA-SHA3-256',
'RSA-SHA3-384',
'RSA-SHA3-512',
'RSA-SHA384',
'RSA-SHA512',
'RSA-SHA512/224',
'RSA-SHA512/256',
'RSA-SM3',
'blake2b512',
'blake2s256',
'id-rsassa-pkcs1-v1_5-with-sha3-224',
'id-rsassa-pkcs1-v1_5-with-sha3-256',
'id-rsassa-pkcs1-v1_5-with-sha3-384',
'id-rsassa-pkcs1-v1_5-with-sha3-512',
'md4',
'md4WithRSAEncryption',
'md5',
'md5-sha1',
'md5WithRSAEncryption',
'mdc2',
'mdc2WithRSA',
'ripemd',
'ripemd160',
'ripemd160WithRSA',
'rmd160',
'sha1',
'sha1WithRSAEncryption',
'sha224',
'sha224WithRSAEncryption',
'sha256',
'sha256WithRSAEncryption',
'sha3-224',
'sha3-256',
'sha3-384',
'sha3-512',
'sha384',
'sha384WithRSAEncryption',
'sha512',
'sha512-224',
'sha512-224WithRSAEncryption',
'sha512-256',
'sha512-256WithRSAEncryption',
'sha512WithRSAEncryption',
'shake128',
'shake256',
'sm3',
'sm3WithRSAEncryption',
'ssl3-md5',
'ssl3-sha1',
'whirlpool'`