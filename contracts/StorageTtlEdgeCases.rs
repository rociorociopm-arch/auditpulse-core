//! Sample Soroban-style contract covering persistent and temporary TTL refreshes.
//!
//! The two storage tiers deliberately live behind different branches so the
//! scanner's storage/TTL heuristic is exercised with edge-case control flow.

use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct StorageTtlEdgeCases;

#[contractimpl]
impl StorageTtlEdgeCases {
    pub fn refresh(env: Env, key: Symbol, persistent: bool) {
        if persistent {
            let store = env.storage().persistent();
            store.set(&key, &1u32);

            // Persistent entries are refreshed on the persistent branch.
            env.storage().persistent().extend_ttl(&key, 100, 200);
        } else {
            let store = env.storage().temporary();
            store.set(&key, &1u32);

            // Temporary entries are refreshed on the temporary branch.
            env.storage().temporary().extend_ttl(&key, 100, 200);
        }
    }

    pub fn refresh_with_early_return(
        env: Env,
        key: Symbol,
        persistent: bool,
        skip_refresh: bool,
    ) {
        if skip_refresh {
            return;
        }

        if persistent {
            env.storage().persistent().set(&key, &2u32);
            env.storage().persistent().extend_ttl(&key, 50, 150);
        } else {
            env.storage().temporary().set(&key, &2u32);
            env.storage().temporary().extend_ttl(&key, 50, 150);
        }
    }
}
