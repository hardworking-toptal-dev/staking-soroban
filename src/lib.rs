use soroban_sdk::{contractimpl, Env};

pub struct StakingContract;

#[contractimpl]
impl StakingContract {
    pub fn new() -> StakingContract {
        StakingContract{}
    }
}