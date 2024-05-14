use crate::{ibc::MsgTransfer, ica::traits::Pack};
/// a collection of pack implementations for foreign types to be used in our ica contracts
use cosmos_sdk_proto::Any;
use osmosis_std::types::cosmos::bank::v1beta1::MsgSend;
use osmosis_std::types::osmosis::{
    gamm::v1beta1::{MsgExitSwapShareAmountIn, MsgJoinSwapExternAmountIn},
    lockup::{MsgBeginUnlocking, MsgLockTokens},
};
use prost::Message;

impl Pack for MsgJoinSwapExternAmountIn {
    fn pack(self) -> Any {
        Any {
            type_url: MsgJoinSwapExternAmountIn::TYPE_URL.to_string(),
            value: self.encode_to_vec(),
        }
    }
}

impl Pack for MsgLockTokens {
    fn pack(self) -> Any {
        Any {
            type_url: MsgLockTokens::TYPE_URL.to_string(),
            value: self.encode_to_vec(),
        }
    }
}

impl Pack for MsgBeginUnlocking {
    fn pack(self) -> Any {
        Any {
            type_url: MsgBeginUnlocking::TYPE_URL.to_string(),
            value: self.encode_to_vec(),
        }
    }
}

impl Pack for MsgExitSwapShareAmountIn {
    fn pack(self) -> Any {
        Any {
            type_url: MsgExitSwapShareAmountIn::TYPE_URL.to_string(),
            value: self.encode_to_vec(),
        }
    }
}

impl Pack for MsgTransfer {
    fn pack(self) -> Any {
        Any {
            type_url: MsgTransfer::TYPE_URL.to_string(),
            value: self.encode_to_vec(),
        }
    }
}

impl Pack for MsgSend {
    fn pack(self) -> Any {
        Any {
            type_url: MsgSend::TYPE_URL.to_string(),
            value: self.encode_to_vec(),
        }
    }
}
