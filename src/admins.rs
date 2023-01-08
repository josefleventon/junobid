use cosmwasm_schema::cw_serde;

use cosmwasm_std::{Addr, Deps, StdResult};
use cw_storage_plus::Item;

#[cw_serde]
pub struct AdminList {
    pub admins: Vec<Addr>,
}

impl AdminList {
    /// returns true if the address is a registered admin
    pub fn is_admin(&self, addr: impl AsRef<str>) -> bool {
        let addr = addr.as_ref();
        self.admins.iter().any(|a| a.as_ref() == addr)
    }
}

/// Verify that an address is authorized to execute a privileged operation
pub fn can_execute(deps: Deps, sender: &str) -> StdResult<bool> {
    let cfg = ADMINS.load(deps.storage)?;
    let can = cfg.is_admin(sender);
    Ok(can)
}

pub const ADMINS: Item<AdminList> = Item::new("admins");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_admin() {
        let admins: Vec<_> = vec!["bob", "paul", "john"]
            .into_iter()
            .map(Addr::unchecked)
            .collect();
        let config = AdminList {
            admins: admins.clone(),
        };

        assert!(config.is_admin(admins[0].as_ref()));
        assert!(config.is_admin(admins[2].as_ref()));
        assert!(!config.is_admin("other"));
    }
}
