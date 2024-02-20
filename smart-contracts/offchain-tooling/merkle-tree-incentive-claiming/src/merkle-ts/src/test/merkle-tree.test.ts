import { IMerkleTree, MerkleTree } from '../merkle/MerkleTree';
import { readFileSync } from 'fs';
import { HashAlgorithm, MerkleHash } from '../merkle/MerkleHash';


describe ('MerkleTree', () => {
    test ('#validate hashList.txt data', () => {
        const hashListArray: string[] = readFileSync('exampleAssets/hashList.txt').toString().split("\n");
        const merkleTree: MerkleTree = new MerkleTree(hashListArray);
        expect(merkleTree.root)
            .toBe('8b65097db5948da501a243395088d2177eb94da1289570a22dab46a6d05bcd1b');
    });

    test ('#enforce minimum data array length', () => {
        expect(() => {
            let tree: IMerkleTree = new MerkleTree([]);
        }).toThrowError('dataArray has a minimum length of 1');

        expect(() => {
            let tree: IMerkleTree = new MerkleTree(new Array());
        }).toThrowError('dataArray has a minimum length of 1');

        expect(() => {
            let tree: IMerkleTree = new MerkleTree([], HashAlgorithm.md5);
        }).toThrowError('dataArray has a minimum length of 1');
    });

    test ('#validate hash from single value', () => {
        let tree: IMerkleTree = new MerkleTree([1]);
        expect(tree.root)
            .toBe('6b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b');
        expect(tree.createHash('1'))
            .toBe('6b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b');

        tree = new MerkleTree(['Test string']);
        expect(tree.root)
            .toBe('a3e49d843df13c2e2a7786f6ecd7e0d184f45d718d1ac1a8a63e570466e489dd');
        expect(tree.createHash('Test string'))
            .toBe('a3e49d843df13c2e2a7786f6ecd7e0d184f45d718d1ac1a8a63e570466e489dd');

        tree = new MerkleTree([1], HashAlgorithm.md5);
        expect(tree.root).toBe('c4ca4238a0b923820dcc509a6f75849b');
        expect(tree.createHash('1')).toBe('c4ca4238a0b923820dcc509a6f75849b');

        tree = new MerkleTree(['Test string'], HashAlgorithm.md5);
        expect(tree.root).toBe('0fd3dbec9730101bff92acc820befc34');
        expect(tree.createHash('Test string')).toBe('0fd3dbec9730101bff92acc820befc34');
    });

    test ('#addNode & #addNodes', () => {
        let tree12: MerkleTree = new MerkleTree([1,2]);
        let tree123: MerkleTree = new MerkleTree([1,2,3]);
        let tree1234: MerkleTree = new MerkleTree([1,2,3,4]);
        let tree12345: MerkleTree = new MerkleTree([1,2,3,4,5]);
        let tree123456: MerkleTree = new MerkleTree([1,2,3,4,5,6]);
        let tree1234567: MerkleTree = new MerkleTree([1,2,3,4,5,6,7]);
        let tree12345678: MerkleTree = new MerkleTree([1,2,3,4,5,6,7,8]);
        let tree123456789: MerkleTree = new MerkleTree([1,2,3,4,5,6,7,8,9]);

        let tree: MerkleTree = new MerkleTree([1]);
        expect(tree.root)
            .toBe('6b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b');
        tree.addNode(2);
        expect(tree.root).toBe(tree12.root);
        expect(tree.root)
            .toBe('33b675636da5dcc86ec847b38c08fa49ff1cace9749931e0a5d4dfdbdedd808a');
        tree.addNode(3);
        expect(tree.root).toBe(tree123.root);
        expect(tree.root)
            .toBe('f3f1917304e3af565b827d1baa9fac18d5b287ae97adda22dc51a0aef900b787');
        tree.addNode(4);
        expect(tree.root).toBe(tree1234.root);
        expect(tree.root)
            .toBe('85df8945419d2b5038f7ac83ec1ec6b8267c40fdb3b1e56ff62f6676eb855e70');
        tree.addNode(5);
        expect(tree.root).toBe(tree12345.root);
        expect(tree.root)
            .toBe('c19ce1b23fc9057eb072011d793ce33a47bb6fc3fe4cf9bf5d8f737abd3be0cb');
        tree.addNode(6);
        expect(tree.root).toBe(tree123456.root);
        expect(tree.root)
            .toBe('058bd72c469db066d7b28c9e63e1b7b05c48df9ca23dd521afd0b6154ea47be6');
        tree.addNode(7);
        expect(tree.root).toBe(tree1234567.root);
        expect(tree.root)
            .toBe('99b80facafca5b81e018de3ea24c2bc6eec81ff21fbf358b512f3df8b862199b');
        tree.addNode(8);
        expect(tree.root).toBe(tree12345678.root);
        expect(tree.root)
            .toBe('c27450cd3fd4df029145f3437ae9c381e0ae55e8400de06cb973005b36d7b222');
        tree.addNode(9);
        expect(tree.root).toBe(tree123456789.root);
        expect(tree.root)
            .toBe('e6f639f0b32f5602f36bdeb8540b5bdc4e922f55d079cdc6d81b20601f5a7d87');

        tree = new MerkleTree(['Test string']);
        expect(tree.root)
            .toBe('a3e49d843df13c2e2a7786f6ecd7e0d184f45d718d1ac1a8a63e570466e489dd');
        tree.addNode('More');
        expect(tree.root)
            .toBe('a84e8547891590b0b7a2ec14f27f584859f96054255b1ecc134143ab8dec7c2f');

        tree = new MerkleTree([1]);
        expect(tree.root)
            .toBe('6b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b');
        tree.addNodes([2]);
        expect(tree12.root).toBe(tree.root);
        expect(tree.root)
            .toBe('33b675636da5dcc86ec847b38c08fa49ff1cace9749931e0a5d4dfdbdedd808a');
        
        tree = new MerkleTree([1]);
        expect(tree.root)
            .toBe('6b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b');
        tree.addNodes([2,3]);
        expect(tree123.root).toBe(tree.root);
        expect(tree.root)
            .toBe('f3f1917304e3af565b827d1baa9fac18d5b287ae97adda22dc51a0aef900b787');
        tree.addNodes([4]);
        expect(tree1234.root).toBe(tree.root);
        expect(tree.root)
            .toBe('85df8945419d2b5038f7ac83ec1ec6b8267c40fdb3b1e56ff62f6676eb855e70');
        
        tree.addNodes([5]);
        expect(tree12345.root).toBe(tree.root);
        expect(tree.root)
            .toBe('c19ce1b23fc9057eb072011d793ce33a47bb6fc3fe4cf9bf5d8f737abd3be0cb');

        tree = new MerkleTree(['Test string']);
        expect(tree.root)
            .toBe('a3e49d843df13c2e2a7786f6ecd7e0d184f45d718d1ac1a8a63e570466e489dd');
        tree.addNodes(['More','Stuff']);
        expect(tree.root)
            .toBe('dc4aab0853b6ad15862daf14e3f95708dc06e22d39dc341be2a5b65c856e0aa4');
        tree.addNodes([0]);
        expect(tree.root)
            .toBe('e4e797a805f0ef965afe5c680fd167a8ae50be52d729170d6e05f7886bef46ae');
        tree.addNodes([0,1]);
        expect(tree.root)
            .toBe('ea9dc6bf1def93f67f5750e97c9ae790511e85662e4a4523aaa443f709630b52');
        tree.addNodes([0,1,2,3,4,5]);
        expect(tree.root)
            .toBe('9e4f1581f75460552255ecf5fc817e254d2b69a80cebeaad15c1a85a7b248c7e');
        tree.addNodes([0,1,2,3,4,5,6]);
        expect(tree.root)
            .toBe('344c256b37f27230d3041fbe4153be97f0e5f7197cd6ae9e8be9081219563dda');
        tree = new MerkleTree(['Test string',0,1,2,3,4,5,true,false,{t:'t'}]);
        expect(tree.root)
            .toBe('0543d7c3eb93d174907216eda38ca06c71f4435f2533e3384fb2078b991bc27a');
        tree.addNodes(['More','Stuff',0x00,10,20,30,40,50,true,false,{ta:'ta',tb:'tb'}]);
        expect(tree.root)
            .toBe('0aee812417d66476a329b2f92da831672b2cbed63bc3c7a761c081a6ddd71990');

        let binTree = new MerkleTree(new Array(1000).fill('1'));
        let binTree2 = new MerkleTree(new Array(1000).fill('1'));
        expect(binTree.root).toBe(binTree2.root);
        for (let i = 0; i < 1000; i++) {
            let num = i % 2 === 0 ? '1' : '0';
            binTree.addNode(num);
            binTree2.addNode(num);
            expect(binTree.root).toBe(binTree2.root);
        }
    });
    
    test ('#validate hash from multiples', () => {
        let tree: IMerkleTree = new MerkleTree([1,2]);
        expect(tree.root)
            .toBe('33b675636da5dcc86ec847b38c08fa49ff1cace9749931e0a5d4dfdbdedd808a');

        tree = new MerkleTree(['Test string','More']);
        expect(tree.root)
            .toBe('a84e8547891590b0b7a2ec14f27f584859f96054255b1ecc134143ab8dec7c2f');

        tree = new MerkleTree([1,2,3]);
        expect(tree.root)
            .toBe('f3f1917304e3af565b827d1baa9fac18d5b287ae97adda22dc51a0aef900b787');

        tree = new MerkleTree(['Test string','More','Stuff']);
        expect(tree.root)
            .toBe('dc4aab0853b6ad15862daf14e3f95708dc06e22d39dc341be2a5b65c856e0aa4');

        tree = new MerkleTree([1,2,3,4,5,6,7]);
        expect(tree.root)
            .toBe('99b80facafca5b81e018de3ea24c2bc6eec81ff21fbf358b512f3df8b862199b');

        tree = new MerkleTree(['Test string','More','Stuff',44,55,66,77]);
        expect(tree.root)
            .toBe('d8fac01434262e90bcd620818e14574dc558e5073655bb722e41ccc88f4c1b88');

        tree = new MerkleTree(['Test string','More','Stuff',44,55,66,77,true,false,
            {test:'this'}]);
        expect(tree.root)
            .toBe('6dbd40775d68d665019668a59072c5a283847797778cc518ef97b18bbad09919');

        let numList: number[] = [];
        for (let i = 0; i < 9999; i++) {
            numList.push(i);
        }
        tree = new MerkleTree(numList);
        expect(tree.root)
            .toBe('8e5de0cb76eb9b7b420574765e9174d4fc209af474448edb733bb6cc8fc1096e');

        tree = new MerkleTree([1,2], HashAlgorithm.md5);
        expect(tree.root).toBe('302cbafc0dfbc97f30d576a6f394dad3');

        tree = new MerkleTree(['Test string','More'], HashAlgorithm.md5);
        expect(tree.root).toBe('e1bfa1951ca12b49e60324127951373a');

        tree = new MerkleTree([1,2,3], HashAlgorithm.md5);
        expect(tree.root).toBe('d37a60fb7556c542502509dfe4d93928');

        tree = new MerkleTree(['Test string','More','stuff'], HashAlgorithm.md5);
        expect(tree.root).toBe('8b8a56cc2e0c741c07712a76c7ccc553');

        tree = new MerkleTree([1,2,3,4,5,6,7], HashAlgorithm.md5);
        expect(tree.root).toBe('662d7787d650efad62a6eac2d9ce6dba');

        tree = new MerkleTree(['Test string','More','stuff',44,55,66,77], HashAlgorithm.md5);
        expect(tree.root).toBe('12a8ba3a5818a326661865d327edbb10');

        tree = new MerkleTree(['Test string','More','Stuff',44,55,66,77,true,false,
            {test:'this'}], HashAlgorithm.md5);
        expect(tree.root).toBe('a2cb7e58da10549ba35bbcecd7fe75f5');

        tree = new MerkleTree(numList, HashAlgorithm.md5);
        expect(tree.root).toBe('744556995f960fddfe4303ab4175c601');
    });

    test ('#validate root', () => {
        let tree1 = new MerkleTree([1]);
        expect(tree1.root)
            .toBe('6b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b');

        let tree2 = new MerkleTree([2]);
        expect(tree2.root)
            .toBe('d4735e3a265e16eee03f59718b9b5d03019c07d8b6c51f90da3a666eec13ab35');

        let tree1_2 = new MerkleTree([1,2]);
        let hash_1_2 = tree1_2.createHash(tree1.root + tree2.root);
        expect(hash_1_2).toBe(tree1_2.root);

        let tree3 = new MerkleTree([3]);
        expect(tree3.root)
            .toBe('4e07408562bedb8b60ce05c1decfe3ad16b72230967de01f640b7e4729b49fce');

        let tree4 = new MerkleTree([4]);
        expect(tree4.root)
            .toBe('4b227777d4dd1fc61c6f884f48641d02b4d121d3fd328cb08b5531fcacdabf8a');

        let tree3_4 = new MerkleTree([3,4]);
        let hash_3_4 = tree3_4.createHash(tree3.root + tree4.root);
        expect(hash_3_4).toBe(tree3_4.root);

        let tree5 = new MerkleTree([5]);
        expect(tree5.root)
            .toBe('ef2d127de37b942baad06145e54b0c619a1f22327b2ebbcfbec78f5564afe39d');
        
        let tree5_5 = new MerkleTree([5,5]);
        let hash_5_5 = tree5_5.createHash(tree5.root + tree5.root);
        expect(hash_5_5).toBe(tree5_5.root);

        //combine 1,2 3,4
        let tree1_2_3_4 = new MerkleTree([1,2,3,4]);
        let hash_1_2_3_4 = tree1_2_3_4.createHash(tree1_2.root + tree3_4.root);
        expect(hash_1_2_3_4).toBe(tree1_2_3_4.root);

        //combine 5,5 5,5
        let tree5_5_5_5 = new MerkleTree([5,5,5,5]);
        let hash_5_5_5_5 = tree5_5_5_5.createHash(tree5_5.root + tree5_5.root);
        expect(hash_5_5_5_5).toBe(tree5_5_5_5.root);

        // combine 1,2,3,4 and 5,5,5,5
        let tree1_2_3_4_5_5_5_5 = new MerkleTree([1,2,3,4,5,5,5,5]);
        let hash_1_2_3_4_5_5_5_5 = tree1_2_3_4_5_5_5_5.createHash(tree1_2_3_4.root + tree5_5_5_5.root);
        expect(hash_1_2_3_4_5_5_5_5).toBe(tree1_2_3_4_5_5_5_5.root);

        // proof for 1,2,3,4,5
        let tree1_2_3_4_5 = new MerkleTree([1,2,3,4,5]);
        expect(tree1_2_3_4_5.root).toBe(tree1_2_3_4_5_5_5_5.root);

        let treeA = new MerkleTree(['A']);
        expect(treeA.root)
            .toBe('559aead08264d5795d3909718cdd05abd49572e84fe55590eef31a88a08fdffd');

        let treeB = new MerkleTree(['B']);
        expect(treeB.root)
            .toBe('df7e70e5021544f4834bbee64a9e3789febc4be81470df629cad6ddb03320a5c');

        let treeA_B = new MerkleTree(['A', 'B']);
        expect(treeA_B.root)
            .toBe('b30ab174f7459cdd40a3acdf15d0c9444fec2adcfb9d579aa154c084885edd0a');

        let hash_A_B = treeA_B.createHash(treeA.root + treeB.root);
        expect(hash_A_B).toBe(treeA_B.root);

        let treeC = new MerkleTree(['C']);
        expect(treeC.root)
            .toBe('6b23c0d5f35d1b11f9b683f0b0a617355deb11277d91ae091d399c655b87940d');

        let treeD = new MerkleTree(['D']);
        expect(treeD.root)
            .toBe('3f39d5c348e5b79d06e842c114e6cc571583bbf44e4b0ebfda1a01ec05745d43');

        let treeC_D = new MerkleTree(['C', 'D']);
        let hash_C_D = treeC_D.createHash(treeC.root + treeD.root);
        expect(hash_C_D).toBe(treeC_D.root);

        let treeA_B_C_D = new MerkleTree(['A', 'B', 'C', 'D']);
        let hashA_B_C_D = treeA_B_C_D.createHash(treeA_B.root + treeC_D.root);
        expect(hashA_B_C_D).toBe(treeA_B_C_D.root);
    });

    test('#getDataFromHash - Retrieve data from associated hash key', () => {
        const dataArray = ['some', 1, 'data', {array:['ok']}];
        const dataMerkleTree = new MerkleTree(dataArray);
        for (let data of dataArray) {
            const dataHash = MerkleHash.createHash(data);
            expect(dataMerkleTree.getDataFromHash(dataHash))
                .toEqual(data);
        }
    });

    test('#maxDepthFromDataArray - validated max MerkleTree depth', () => {
        const dataArray100 = new Array(100);
        expect(MerkleTree.maxDepthFromDataArray(dataArray100)).toBe(8);

        const dataArray200 = new Array(200);
        expect(MerkleTree.maxDepthFromDataArray(dataArray200)).toBe(9);

        const dataArray1000 = new Array(1000);
        expect(MerkleTree.maxDepthFromDataArray(dataArray1000)).toBe(11);

        const dataArray = ['Test string','More','Stuff',44,55,66,77,true,false,{test:'this'}];
        expect(MerkleTree.maxDepthFromDataArray(dataArray)).toBe(5);

        const dataArray500filled = new Array(500).fill(Math.random());
        const maxDepth500 = MerkleTree.maxDepthFromDataArray(dataArray500filled);
        expect(maxDepth500).toBe(10);
        const tree = new MerkleTree(dataArray500filled);
        expect(tree.hashRecords.length).toBe(maxDepth500);

        const dataArrayOne = [1];
        const maxDepth1 = MerkleTree.maxDepthFromDataArray(dataArrayOne);
        expect(maxDepth1).toBe(1);
        const treeOneData = new MerkleTree(dataArrayOne);
        expect(treeOneData.hashRecords.length).toBe(maxDepth1);
    });

    test('#updateNodeAt', () => {
        let failTree = new MerkleTree([1]);
        expect(failTree.updateNodeAt(0, 'test')).toBeTruthy();
        expect(failTree.dataArray[0]).toBe('test');
        expect(failTree.updateNodeAt(5, 'test')).toBeFalsy();

        let tree = new MerkleTree([1,2,3,4,5,6,7,8]);
        expect(tree.root).toBe('c27450cd3fd4df029145f3437ae9c381e0ae55e8400de06cb973005b36d7b222');
        expect(tree.dataArray[3]).toBe(4);
        expect(tree.updateNodeAt(3, 10)).toBeTruthy();
        expect(tree.dataArray[3]).toBe(10);

        let tree2 = new MerkleTree([1,2,3,10,5,6,7,8]);
        expect(tree2.root).toBe(tree.root);

        expect(tree.updateNodeAt(0, 0)).toBeTruthy();
        expect(tree.updateNodeAt(7, 10000)).toBeTruthy();
        let tree3 = new MerkleTree([0,2,3,10,5,6,7,10000]);
        expect(tree3.root).toBe(tree.root);
        expect(tree2.updateNodeAt(0, 0)).toBeTruthy();
        expect(tree2.updateNodeAt(7, 10000)).toBeTruthy();
        expect(tree3.root).toBe(tree2.root);


        expect(tree3.updateNodeAt(4, 222)).toBeTruthy();
        expect(tree3.dataArray[4]).toBe(222);
        expect(tree3.updateNodeAt(tree3.dataArray.length-1, 123)).toBeTruthy();
        expect(tree3.dataArray[7]).toBe(123);
        let treeCopyData = new MerkleTree(tree3.dataArray.slice());
        expect(treeCopyData.root).toBe(tree3.root);

        expect(tree3.updateNodeAt(0, 1234)).toBeTruthy();
        expect(tree3.dataArray[0]).toBe(1234);
        treeCopyData = new MerkleTree(tree3.dataArray.slice());
        expect(treeCopyData.root).toBe(tree3.root);

        let sA = new Array(11).fill(Math.random());
        let sTree = new MerkleTree(sA.slice());
        let sTree2 = new MerkleTree(sTree.dataArray.slice());
        expect(sTree2.root).toBe(sTree.root);
        expect(sTree.updateNodeAt(4, 999)).toBeTruthy();
        expect(sTree.updateNodeAt(9, 123123)).toBeTruthy();
        expect(sTree.dataArray[4]).toBe(999);
        expect(sTree.dataArray[9]).toBe(123123);
        let sTree3 = new MerkleTree(sTree.dataArray.slice());
        expect(sTree3.dataArray).toEqual(sTree.dataArray);
        expect(sTree3.root).toBe(sTree.root);

        let medA = new Array(10000).fill(Math.random());
        let medTree = new MerkleTree(medA.slice());
        let medTree2 = new MerkleTree(medTree.dataArray.slice());
        expect(medTree2.root).toBe(medTree.root);
        expect(medTree.updateNodeAt(4000, 9999)).toBeTruthy();
        expect(medTree.updateNodeAt(9999, 9999)).toBeTruthy();
        expect(medTree.dataArray[4000]).toBe(9999);
        expect(medTree.dataArray[9999]).toBe(9999);
        let medTree3 = new MerkleTree(medTree.dataArray.slice());
        expect(medTree3.root).toBe(medTree.root);

        let largeArray1 = new Array(50000).fill(Math.random());
        let treeLarge1 = new MerkleTree(largeArray1.slice());
        let root1 = treeLarge1.root;
        expect(treeLarge1.updateNodeAt(40000, 9999)).toBeTruthy();
        expect(treeLarge1.updateNodeAt(49999, 9999)).toBeTruthy();
        expect(treeLarge1.dataArray[40000]).toBe(9999);
        expect(treeLarge1.dataArray[49999]).toBe(9999);
        expect(root1 === treeLarge1.root).toBeFalsy();
        let treeLarge2 = new MerkleTree(treeLarge1.dataArray.slice());
        expect(treeLarge2.root).toBe(treeLarge1.root);
        expect(treeLarge1.updateNodeAt(35999, 123)).toBeTruthy();
        expect(treeLarge2.updateNodeAt(35999, 123)).toBeTruthy();
        expect(treeLarge1.dataArray[35999]).toBe(123);
        expect(treeLarge2.dataArray[35999]).toBe(123);
        expect(treeLarge2.root).toBe(treeLarge1.root);
    });

    if (process.env.BENCHMARK) {
        test('#benchmark Int MerkleTree with caching', () => {
            MerkleHash.ENABLE_CACHING = true;
            const dataArray50k = new Array(50000).fill(Math.random());
            const maxDepth50k = MerkleTree.maxDepthFromDataArray(dataArray50k);
            expect(maxDepth50k).toBe(17);
            const treeData50k = new MerkleTree(dataArray50k); 
            expect(treeData50k.hashRecords.length).toBe(maxDepth50k);
    
            const dataArray100k = new Array(100000).fill(Math.random());
            const maxDepth100k = MerkleTree.maxDepthFromDataArray(dataArray100k);
            expect(maxDepth100k).toBe(18);
            const treeData100k = new MerkleTree(dataArray100k); 
            expect(treeData100k.hashRecords.length).toBe(maxDepth100k);
    
            const dataArray500k = new Array(500000).fill(Math.random());
            const maxDepth500k = MerkleTree.maxDepthFromDataArray(dataArray500k);
            expect(maxDepth500k).toBe(20);
            const treeData500k = new MerkleTree(dataArray500k); 
            expect(treeData500k.hashRecords.length).toBe(maxDepth500k);
    
            const dataArray1mill = new Array(1000000).fill(Math.random());
            const maxDepth1mill = MerkleTree.maxDepthFromDataArray(dataArray1mill);
            expect(maxDepth1mill).toBe(21);
            const treeData1mill = new MerkleTree(dataArray1mill); 
            expect(treeData1mill.hashRecords.length).toBe(maxDepth1mill);
        });
    
        test('#benchmark String MerkleTree with caching', () => {
            MerkleHash.ENABLE_CACHING = true;
            function randomBitString() {
                return ['0','1'][Math.floor(Math.random() * 1)];
            }
            const dataArray50k = new Array(50000).fill(randomBitString());
            const maxDepth50k = MerkleTree.maxDepthFromDataArray(dataArray50k);
            expect(maxDepth50k).toBe(17);
            const treeData50k = new MerkleTree(dataArray50k); 
            expect(treeData50k.hashRecords.length).toBe(maxDepth50k);
    
            const dataArray100k = new Array(100000).fill(randomBitString());
            const maxDepth100k = MerkleTree.maxDepthFromDataArray(dataArray100k);
            expect(maxDepth100k).toBe(18);
            const treeData100k = new MerkleTree(dataArray100k); 
            expect(treeData100k.hashRecords.length).toBe(maxDepth100k);
    
            const dataArray500k = new Array(500000).fill(randomBitString());
            const maxDepth500k = MerkleTree.maxDepthFromDataArray(dataArray500k);
            expect(maxDepth500k).toBe(20);
            const treeData500k = new MerkleTree(dataArray500k); 
            expect(treeData500k.hashRecords.length).toBe(maxDepth500k);
    
            const dataArray1mill = new Array(1000000).fill(randomBitString());
            const maxDepth1mill = MerkleTree.maxDepthFromDataArray(dataArray1mill);
            expect(maxDepth1mill).toBe(21);
            const treeData1mill = new MerkleTree(dataArray1mill); 
            expect(treeData1mill.hashRecords.length).toBe(maxDepth1mill);
        });
    
        test('#benchmark MerkleHash sha256', () => {
            MerkleHash.ENABLE_CACHING = false;
            for(let i = 0; i < MerkleHash.BENCHMARK_ITERATIONS; i++) {
                MerkleHash.createHash(i, HashAlgorithm.sha256);
            }
        });
    
        test('#benchmark MerkleHash md5', () => {
            MerkleHash.ENABLE_CACHING = false;
            for(let i = 0; i < MerkleHash.BENCHMARK_ITERATIONS; i++) {
                MerkleHash.createHash(i, HashAlgorithm.md5);
            }
        });
        
        test('#benchmark MerkleHash sha1', () => {
            MerkleHash.ENABLE_CACHING = false;
            for(let i = 0; i < MerkleHash.BENCHMARK_ITERATIONS; i++) {
                MerkleHash.createHash(i, HashAlgorithm.sha1);
            }
        });
    
        test('#benchmark addNode - Large Binary String sha256', () => {
            MerkleHash.ENABLE_CACHING = true;
            const tree = new MerkleTree(new Array(1000000).fill('1'), HashAlgorithm.sha256);
            for(let i = 0; i < MerkleHash.BENCHMARK_ITERATIONS; i++) {
                let num = i % 2 === 0 ? '1' : '0';
                tree.addNode(num);
            }
            expect(tree.root)
                .toBe('42086e7c88688f0fb2826ad14932577a4cb715a48027e21d8651216cd71da714');
        
            const testTree = new MerkleTree(tree.dataArray, HashAlgorithm.sha256);
            expect(tree.root).toBe(testTree.root);
        });
    
        test('#benchmark addNode - Large Binary String sha512', () => {
            MerkleHash.ENABLE_CACHING = true;
            const tree = new MerkleTree(new Array(1000000).fill('1'), HashAlgorithm.sha512);
            for(let i = 0; i < MerkleHash.BENCHMARK_ITERATIONS; i++) {
                let num = i % 2 === 0 ? '1' : '0';
                tree.addNode(num);
            }
            expect(tree.root)
                .toBe('5e0b6e2b26fdf307a1b69e8f9b1b9cd3538c19e983b1e6851ceb941166c19895c3db60719265a7fe6937ad613a51fc1385452f795b854eb73eb7a720582160a3');
            
            const testTree = new MerkleTree(tree.dataArray, HashAlgorithm.sha512);
            expect(tree.root).toBe(testTree.root);
        });

        test('#benchmark addNode - Super Large Binary String sha256', () => {
            MerkleHash.ENABLE_CACHING = true;
            const tree = new MerkleTree(new Array(10000000).fill('1'), HashAlgorithm.sha256);
            for(let i = 0; i < MerkleHash.BENCHMARK_ITERATIONS; i++) {
                let num = i % 2 === 0 ? '1' : '0';
                tree.addNode(num);
            }
            expect(tree.root)
                .toBe('ba69f6c65cb2e77d8195ff25cfdbfa49659f371b17b9a3ece055d6e017d973ad');
            
            const testTree = new MerkleTree(tree.dataArray, HashAlgorithm.sha256);
            expect(tree.root).toBe(testTree.root);
        });

        test('#benchmark addNode - Super Large Binary String sha512', () => {
            MerkleHash.ENABLE_CACHING = true;
            const tree = new MerkleTree(new Array(10000000).fill('1'), HashAlgorithm.sha512);
            for(let i = 0; i < MerkleHash.BENCHMARK_ITERATIONS; i++) {
                let num = i % 2 === 0 ? '1' : '0';
                tree.addNode(num);
            }
            expect(tree.root)
                .toBe('d71d5b05d8a62c67392add22c6800f194b8a335742823e2bd72cdd38711b5fcd3d2819edac21105970f6b17b7edcf5e0cfe6637866a2b06894512eebff118c4d');
            
            const testTree = new MerkleTree(tree.dataArray, HashAlgorithm.sha512);
            expect(tree.root).toBe(testTree.root);
        });
    }

});