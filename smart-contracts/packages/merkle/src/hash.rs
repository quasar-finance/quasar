use std::fmt;

use serde::{de, de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use sha3::{Digest, Sha3_256};

// The distinction in prefixes is needed
// to guard against second preimage attack
// with Merkle trees:
// https://flawed.net.nz/2018/02/21/attacking-merkle-trees-with-a-second-preimage-attack/
const LEAF_NODE_PREFIX: &[u8] = &[0];
const BRANCH_NODE_PREFIX: &[u8] = &[1];

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default)]
pub struct Hash(pub(crate) [u8; 32]);

// Serialize for Entry is the custom serialization implementation.
// Since plain-text SHA3 hash might not be exclusive to the ASCII set,
// we need to first base64 encoded it before serializing. This is what
// this implementation achieves.
impl Serialize for Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let hash = base64::encode(self);
        let bytes = hash.as_bytes();

        serializer.serialize_some(bytes)
    }
}

struct HashVisitor;

impl<'de> Visitor<'de> for HashVisitor {
    type Value = Hash;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence of bytes")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let mut new_obj = Vec::<u8>::new();
        while let Some(value) = seq.next_element()? {
            let value_b: u8 = value;
            new_obj.push(value_b);
        }
        Ok(Hash::from(base64::decode(&new_obj).unwrap()))
    }
}

impl<'de> Deserialize<'de> for Hash {
    fn deserialize<D>(deserializer: D) -> Result<Hash, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(HashVisitor)
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl From<Vec<u8>> for Hash {
    fn from(item: Vec<u8>) -> Self {
        Hash(<[u8; 32]>::try_from(item.as_slice()).unwrap())
    }
}

#[derive(Clone, Default)]
struct Hasher {
    hasher: Sha3_256,
}

impl Hasher {
    fn update(&mut self, val: &[u8]) {
        self.hasher.update(val);
    }

    fn result(self) -> Hash {
        // At the time of this writing, the sha2 library is stuck on an old version
        // of generic_array (0.9.0). Decouple ourselves with a clone to our version.
        Hash(<[u8; 32]>::try_from(self.hasher.finalize().as_slice()).unwrap())
    }
}

pub fn leaf(data: &[u8]) -> Hash {
    hash(&[LEAF_NODE_PREFIX, data])
}

pub fn branch(left_child: &Hash, right_child: &Hash) -> Hash {
    hash(&[
        BRANCH_NODE_PREFIX,
        left_child.as_ref(),
        right_child.as_ref(),
    ])
}

fn hash(values: &[&[u8]]) -> Hash {
    let mut hasher = Hasher::default();
    for value in values {
        hasher.update(value);
    }
    hasher.result()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util;

    #[test]
    fn custom_serialization_works() {
        let test_entry = leaf(test_util::OSMO);

        let serialized = serde_json_wasm::to_string(&test_entry).unwrap();
        let deserialized: Hash = serde_json_wasm::from_str(&serialized).unwrap();

        assert_eq!(test_entry, deserialized);
    }
}
