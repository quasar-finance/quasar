/// a collection of pack implementations for foreign types to be used in our ica contracts
use cosmos_sdk_proto::Any;
use osmosis_std::types::osmosis::gamm::v1beta1::{MsgJoinSwapExternAmountIn, MsgJoinSwapExternAmountInResponse};
use prost::Message;
use crate::{ica::traits::Unpack, error::Error};


impl Unpack for MsgJoinSwapExternAmountIn {
    fn unpack(msg: Any) -> Result<Self, Error> {
        if msg.type_url != MsgJoinSwapExternAmountIn::TYPE_URL {
            return Err(Error::UnpackInvalidTypeUrl{ expected: MsgJoinSwapExternAmountIn::TYPE_URL.to_string(), actual: msg.type_url });
        }
        let val: Self = Message::decode(msg.value.as_ref())?; 
        Ok(val)
    }
}

impl Unpack for MsgJoinSwapExternAmountInResponse {
    // For some reason, MsgJoinSwapExternAmountInResponse's type url on Osmosis is the same as MsgJoinSwapExternAmountIn
    fn unpack(msg: Any) -> Result<Self, Error> {
        // the type url is intended to MsgJoinSwapExternAmountIn
        if msg.type_url != MsgJoinSwapExternAmountIn::TYPE_URL {
            return Err(Error::UnpackInvalidTypeUrl{ expected: MsgJoinSwapExternAmountIn::TYPE_URL.to_string(), actual: msg.type_url });
        }
        let val: Self = Message::decode(msg.value.as_ref())?; 
        Ok(val)
    }
}