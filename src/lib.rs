#![no_std]
use soroban_sdk::{contractimpl, contracttype, Env, Address, Symbol};


const INIT: Symbol = Symbol::short("initialized");

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    RewardToken,
    TokenAdmin,
}

pub struct StakingContract;

#[contractimpl]
impl StakingContract {

    pub fn initialize(
        env: Env,
        reward_token: Address,
        token_admin: Address,
    ) {
        token_admin.require_auth();
        assert!(
            !env.storage().has(&DataKey::TokenAdmin),
            "already initialized"
        );

        env.storage().set(&DataKey::RewardToken, &reward_token);
        env.storage().set(&DataKey::TokenAdmin, &token_admin);

        env.events().publish((INIT, Symbol::short("staking")), token_admin);
    }

    pub fn stake(env: Env) {

    }

    pub fn unstake(env: Env) {

    }

    pub fn claim_reward(env: Env) {

    }

    pub fn withdraw(env: Env) {

    }
}