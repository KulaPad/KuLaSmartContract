use crate::*;
use near_sdk::collections::UnorderedMap;

pub type TierId = u8;
pub type XTokenConfigs = UnorderedMap<TierId, u32>;
