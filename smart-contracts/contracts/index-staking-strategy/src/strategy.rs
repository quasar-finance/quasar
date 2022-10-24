use cosmwasm_std::Coin;

pub trait strategy {
    fn on_deposit(funds: Vec<Coin>);
}


struct dummy {
}


impl strategy for dummy {
    fn on_deposit(funds: Vec<Coin>) {

    }
}