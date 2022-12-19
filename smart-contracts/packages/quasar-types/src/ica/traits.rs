use cosmos_sdk_proto::Any;

use crate::error::Error;

pub trait Pack {
    fn pack(self) -> Any;
}

pub trait Unpack 
where Self: Sized {
    fn unpack(msg: Any) -> Result<Self, Error>;
}
