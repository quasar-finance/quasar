use merkle::hash;
use merkle::Tree;
use std::error::Error;

pub fn generate_root(data: &[Vec<u8>]) -> String {
    let tree = Tree::new(data);
    let hash = tree.get_root().unwrap();

    base64::encode(hash)
}

pub fn get_proof(data: &[Vec<u8>], proof_for: &Vec<u8>) -> Result<String, Box<dyn Error>> {
    let tree = Tree::new(data);

    let proof_opt = tree.find_proof(proof_for);

    if proof_opt.is_none() {
        return Err(format!(
            "failed to find proof for {:?}, the data hash is {:?}",
            proof_for,
            hash::leaf(proof_for)
        )
        .into());
    }

    let proof = proof_opt.unwrap();

    let serialized = serde_json_wasm::to_string(&proof)?;

    Ok(serialized)
}

pub fn verify_proof(
    root: &String,
    proof_bytes: &str,
    to_verify: String,
) -> Result<bool, Box<dyn Error>> {
    let proof: merkle::proof::Proof = serde_json_wasm::from_str(proof_bytes)?;
    let root_decoded = base64::decode(root)?;

    Ok(proof.verify(&to_verify, &merkle::hash::Hash::from(root_decoded)))
}

pub fn hash(data: &String) -> String {
    return merkle::hash::leaf(data.as_bytes()).to_string();
}
