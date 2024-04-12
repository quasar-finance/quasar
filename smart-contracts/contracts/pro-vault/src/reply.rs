use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(FromPrimitive, IntoPrimitive)]
#[repr(u64)]
pub enum Replies {
    #[default]
    Unknown,
}
