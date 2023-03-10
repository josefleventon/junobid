use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::msg::ExecuteMsg;
use cosmwasm_std::{to_binary, Addr, CosmosMsg, StdResult, WasmMsg};

/// JunoBidContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct JunoBidContract(pub Addr);

impl JunoBidContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    /// Wrapper for calling execute messages
    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }
}
