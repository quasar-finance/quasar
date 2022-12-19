/// a collection of pack implementations for foreign types to be used in our ica contracts
use cosmos_sdk_proto::Any;
use osmosis_std::types::osmosis::{gamm::v1beta1::MsgJoinSwapExternAmountIn, lockup::MsgLockTokens};
use prost::Message;
use crate::ica::traits::Pack;

impl Pack for MsgJoinSwapExternAmountIn {
    fn pack(self) -> Any {
        Any { type_url: MsgJoinSwapExternAmountIn::TYPE_URL.to_string(), value: self.encode_to_vec() }
    }
}

impl Pack for MsgLockTokens {
    fn pack(self) -> Any {
        Any { type_url: MsgLockTokens::TYPE_URL.to_string(), value: self.encode_to_vec() }
    }
}