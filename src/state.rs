use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Data {
    pub sID: Addr,
    pub cID: Addr,
    pub username: String,
    pub service_name: String,
    pub customerAccepted: bool,
    pub ServiceProviderAccepted: bool,
}

pub const States: Map<(&str,&str),Data>= Map::new("people");

