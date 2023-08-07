#![cfg(test)]

use crate::StakingContractClient;

use soroban_sdk::{testutils::Ledger, testutils::LedgerInfo, Address, Env};

extern crate std;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn register_test_contract(e: &Env) -> Address {
    let current_time = SystemTime::now();
    let current_timestamp = current_time
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get the current timestamp")
        .as_secs() as u64;

    e.ledger().set(LedgerInfo {
        timestamp: current_timestamp.clone(),
        protocol_version: 1,
        sequence_number: 10,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_expiration: 0,
        min_persistent_entry_expiration: 0,
        max_entry_expiration: 0,
    });
    e.register_contract(None, crate::StakingContract {})
}

pub struct StakingContract {
    env: Env,
    contract_id: Address,
}

impl StakingContract {
    #[must_use]
    pub fn client(&self) -> StakingContractClient<'static> {
        StakingContractClient::new(&self.env, &self.contract_id)
    }

    #[must_use]
    pub fn new(env: &Env, contract_id: Address) -> Self {
        Self {
            env: env.clone(),
            contract_id,
        }
    }
}
