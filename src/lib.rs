#![no_std]
use soroban_sdk::{contracterror, contractimpl, contracttype, token, Address, Env, Symbol};

extern crate std;
use std::time::{SystemTime, UNIX_EPOCH};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    StakeDetailNotExist = 1,
    PlanNotExist = 2,
    PlanNotFinished = 3,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StakeDetail {
    owner: Address,
    total_staked: i128,
    last_staked: i128,
    reward_amount: i128,
    plan: i128,
    start_time: u64,
    end_time: u64,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    RewardToken,
    TokenAdmin,
}

pub struct StakingContract;

#[contractimpl]
impl StakingContract {
    pub fn initialize(env: Env, reward_token: Address, token_admin: Address) {
        token_admin.require_auth();
        assert!(
            !env.storage().has(&DataKey::TokenAdmin),
            "already initialized"
        );

        env.storage().set(&DataKey::RewardToken, &reward_token);
        env.storage().set(&DataKey::TokenAdmin, &token_admin);

        env.events().publish(
            (Symbol::short("INIT"), Symbol::short("staking")),
            token_admin,
        );
    }

    pub fn stake(
        env: Env,
        amount: i128,
        account: Address,
        plan: i128,
        start_time: u64,
        end_time: u64,
        token_id: Address,
    ) -> Result<StakeDetail, Error> {
        account.require_auth();

        let plan1: i128 = 7;
        let plan2: i128 = 14;
        let plan3: i128 = 30;

        if plan != plan1 && plan != plan2 && plan != plan3 {
            return Err(Error::PlanNotExist);
        }

        let stake_detail = StakeDetail {
            owner: account.clone(),
            total_staked: amount,
            last_staked: amount,
            reward_amount: 0,
            plan: plan,
            start_time: start_time,
            end_time: end_time,
        };

        let client = token::Client::new(&env.clone(), &token_id);

        client.transfer(&account, &env.current_contract_address(), &amount);

        env.storage().set(&account, &stake_detail.clone());

        env.events().publish(
            (Symbol::short("stake"), Symbol::short("amount")),
            stake_detail.clone(),
        );

        return Ok(stake_detail);
    }

    pub fn unstake(env: Env, account: Address, token_id: Address) {
        account.require_auth();

        let mut stake_detail = Self::get_stake_detail(env.clone(), account.clone()).unwrap();

        let current_time = Self::get_current_time();

        if stake_detail.end_time < current_time {
            Error::PlanNotFinished;
        }

        let client = token::Client::new(&env.clone(), &token_id);
        client.transfer(
            &env.current_contract_address(),
            &stake_detail.owner,
            &stake_detail.total_staked,
        );

        stake_detail.total_staked = 0;

        env.storage().set(&account, &stake_detail);

        env.events().publish(
            (Symbol::short("unstake"), Symbol::short("amount")),
            stake_detail,
        );
    }

    pub fn claim_reward(env: Env) {}

    pub fn calculate_reward(env: Env, account: Address) -> i128 {
        let stake_detail = Self::get_stake_detail(env.clone(), account.clone()).unwrap();
        let plan = stake_detail.plan;

        let mut reward_amount = 0;

        if plan == 7 {
            reward_amount = 14;
        } else if plan == 14 {
            reward_amount = 28;
        } else if plan == 30 {
            reward_amount = 60;
        }

        return reward_amount;
    }

    fn get_current_time() -> u64 {
        let current_time = SystemTime::now();
        let current_timestamp = current_time
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get the current timestamp")
            .as_secs() as u64;

        return current_timestamp;
    }

    pub fn get_stake_detail(env: Env, account: Address) -> Result<StakeDetail, Error> {
        let stake_detail: StakeDetail = env
            .storage()
            .get(&account)
            .unwrap_or(Ok(StakeDetail {
                owner: env.current_contract_address(),
                total_staked: 0,
                last_staked: 0,
                reward_amount: 0,
                plan: 0,
                start_time: 0,
                end_time: 0,
            }))
            .unwrap();

        if stake_detail.owner == env.current_contract_address() {
            return Err(Error::StakeDetailNotExist);
        }

        return Ok(stake_detail);
    }

    pub fn get_reward_token(env: Env) -> Address {
        env.storage()
            .get(&DataKey::RewardToken)
            .expect("none")
            .unwrap()
    }
}

#[cfg(test)]
mod test;

mod testutils;
