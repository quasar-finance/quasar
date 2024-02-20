/** 
 * @module MerkleHash - hash utlities for MerkleTree
 * 
 * @author Lucas Armstrong - Lucas@throneit.com - github.com/LucasArmstrong 
 */

/**
 * @import node:crypto - Used to calculate one way hashes
 * @import fs.createReadStream - For creating a hash from a file
 * @import MerkleDataType - all the data types used in MerkleTree
 */
import * as crypto from 'node:crypto';
import { createReadStream } from 'fs';
import { MerkleDataType } from "./MerkleTree";

/**
 * @enum HashAlgorithm
 */
export enum HashAlgorithm {
    'RSA-MD4' = 'RSA-MD4',
    'RSA-MD5' = 'RSA-MD5',
    'RSA-MDC2' = 'RSA-MDC2',
    'RSA-RIPEMD160' = 'RSA-RIPEMD160',
    'RSA-SHA1' = 'RSA-SHA1',
    'RSA-SHA1-2' = 'RSA-SHA1-2',
    'RSA-SHA224' = 'RSA-SHA224',
    'RSA-SHA256' = 'RSA-SHA256',
    'RSA-SHA3-224' = 'RSA-SHA3-224',
    'RSA-SHA3-256' = 'RSA-SHA3-256',
    'RSA-SHA3-384' = 'RSA-SHA3-384',
    'RSA-SHA3-512' = 'RSA-SHA3-512',
    'RSA-SHA384' = 'RSA-SHA384',
    'RSA-SHA512' = 'RSA-SHA512',
    'RSA-SHA512/224' = 'RSA-SHA512/224',
    'RSA-SHA512/256' = 'RSA-SHA512/256',
    'RSA-SM3' = 'RSA-SM3',
    'blake2b512' = 'blake2b512',
    'blake2s256' = 'blake2s256',
    'id-rsassa-pkcs1-v1_5-with-sha3-224' = 'id-rsassa-pkcs1-v1_5-with-sha3-224',
    'id-rsassa-pkcs1-v1_5-with-sha3-256' = 'id-rsassa-pkcs1-v1_5-with-sha3-256',
    'id-rsassa-pkcs1-v1_5-with-sha3-384' = 'id-rsassa-pkcs1-v1_5-with-sha3-384',
    'id-rsassa-pkcs1-v1_5-with-sha3-512' = 'id-rsassa-pkcs1-v1_5-with-sha3-512',
    'md4' = 'md4',
    'md4WithRSAEncryption' = 'md4WithRSAEncryption',
    'md5' = 'md5',
    'md5-sha1' = 'md5-sha1',
    'md5WithRSAEncryption' = 'md5WithRSAEncryption',
    'mdc2' = 'mdc2',
    'mdc2WithRSA' = 'mdc2WithRSA',
    'ripemd' = 'ripemd',
    'ripemd160' = 'ripemd160',
    'ripemd160WithRSA' = 'ripemd160WithRSA',
    'rmd160' = 'rmd160',
    'sha1' = 'sha1',
    'sha1WithRSAEncryption' = 'sha1WithRSAEncryption',
    'sha224' = 'sha224',
    'sha224WithRSAEncryption' = 'sha224WithRSAEncryption',
    'sha256' = 'sha256',
    'sha256WithRSAEncryption' = 'sha256WithRSAEncryption',
    'sha3-224' = 'sha3-224',
    'sha3-256' = 'sha3-256',
    'sha3-384' = 'sha3-384',
    'sha3-512' = 'sha3-512',
    'sha384' = 'sha384',
    'sha384WithRSAEncryption' = 'sha384WithRSAEncryption',
    'sha512' = 'sha512',
    'sha512-224' = 'sha512-224',
    'sha512-224WithRSAEncryption' = 'sha512-224WithRSAEncryption',
    'sha512-256' = 'sha512-256',
    'sha512-256WithRSAEncryption' = 'sha512-256WithRSAEncryption',
    'sha512WithRSAEncryption' = 'sha512WithRSAEncryption',
    'shake128' = 'shake128',
    'shake256' = 'shake256',
    'sm3' = 'sm3',
    'sm3WithRSAEncryption' = 'sm3WithRSAEncryption',
    'ssl3-md5' = 'ssl3-md5',
    'ssl3-sha1' = 'ssl3-sha1',
    'whirlpool' = 'whirlpool'
}

/**
 * @class MerkleHash
 */
export class MerkleHash {
    
    /**
     * @var ENABLE_CACHING
     */
    public static ENABLE_CACHING = false;

    public static BENCHMARK_ITERATIONS = 300000;

    /**
     * @var hashCache - Cache for all the hashes, keyed by algorithm + data string
     */
    private static hashCache: {[algo_dataString: string]: string} = {};

    /**
     * @method createHash - takes a MerkleDataType payload to generate a
     * 
     * @param data MerkleDataType - The value used to generate a hash
     * @param hashAlgorithm HashAlgorithm - hash algorithm to use
     * @returns {string}
     */
    static createHash(data: MerkleDataType, hashAlgorithm: HashAlgorithm = HashAlgorithm.sha256): string {
        const dataString: string = this.convertToString(data);
        let algo_dataString: string = '';

        if (this.ENABLE_CACHING) {
            algo_dataString = hashAlgorithm + dataString;
            if (typeof this.hashCache[algo_dataString] !== 'undefined') {
                return this.hashCache[algo_dataString];
            }
        }
        
        const hash = crypto.createHash(hashAlgorithm)
                            .update(dataString)
                            .digest('hex');

        if (this.ENABLE_CACHING) {
            this.hashCache[algo_dataString] = hash;
        }

        return hash;
    }

    /**
     * @method createHashFromFile - Static method to be used as utility in order to create a hash from a file of any size
     * 
     * @param filePath 
     * @returns {Promise<string>}
     */
    static async createHashFromFile(filePath: string, hashAlgorithm: HashAlgorithm = HashAlgorithm.sha256): Promise<string> {
        return new Promise((resolve, reject) => {
            const fileHash = crypto.createHash(hashAlgorithm.toString());
            const fileStream = createReadStream(filePath);
            fileStream.on('error', error => {
                reject(error);
            });
            fileStream.on('data', buffer => {
                fileHash.update(buffer.toString(), 'utf8');
            });
            fileStream.on('end', () => {
                resolve(fileHash.digest('hex'));
            });
        });
    }

    /**
     * @method convertToString - takes a MerkleDataType payload to generate a string for hashing
     * 
     * @param data MerkleDataType
     * @returns {string}
     */
     private static convertToString(data: MerkleDataType): string {
        const dataType: string = typeof data;
        switch(dataType) {
            case 'string':
            case 'boolean':
            case 'number':
                return data.toString();
            case 'object':
                return JSON.stringify(data);
            default:
                return dataType;
        }
    }

}