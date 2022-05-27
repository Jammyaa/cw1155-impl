use cw1155::{Cw1155ExecuteMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub type ExecuteMsg = Cw1155ExecuteMsg;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub minter: String,
    pub num_tokens: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Balance {
        owner: String,
        token_id: String,
    },
    BatchBalance {
        owner: String,
        token_ids: Vec<String>,
    },
    ApprovedForAll {
        owner: String,
        include_expired: Option<bool>,
        start_after: Option<String>,
        limit: Option<u32>,
    },

    IsApprovedForAll {
        owner: String,
        operator: String,
    },

    TokenInfo {
        token_id: String,
    },

    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },

    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },

    ContractInfo {},
}

