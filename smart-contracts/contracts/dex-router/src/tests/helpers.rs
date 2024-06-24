use cosmwasm_std::Coin;

pub fn sort_tokens(tokens: Vec<Coin>) -> Vec<Coin> {
    let mut sorted_tokens = tokens;
    sorted_tokens.sort_by(|a, b| a.denom.cmp(&b.denom));
    sorted_tokens
}
