use cosmwasm_std::{IbcMsg};
use intergamm_bindings::msg::IntergammMsg;


pub enum Message {
    Intergamm{msg: IntergammMsg},
    Ibc{msg: IbcMsg}
}

// Next is an absolutely disgusting but sadly necessary enum that has to be extended by users of
pub enum Callback {
    One,
    Two,
}

pub struct IbcBuilder {
}

impl IbcBuilder {
    fn new() -> IbcBuilder {
        todo!()
    }

    fn handle(next: Callback) {
        match next {
            Callback::One => todo!(),
            Callback::Two => todo!(),
        }
    }

    fn execute(msg: Message, next: Callback) {
        
    }
}