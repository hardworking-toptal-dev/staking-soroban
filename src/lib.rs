#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, token, Address, Env,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    StakeDetailNotExist = 1,
    PlanNotExist = 2,
    PlanNotFinished = 3,
    ZeroStake = 4,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StakeDetail {
    owner: Address,
    total_staked: i128,
    last_staked: i128,
    reward_amount: i128,
    plan: u64,
    end_time: u64,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    RewardToken,
    TokenAdmin,
}

const PLAN1: u64 = 7;
const PLAN2: u64 = 14;
const PLAN3: u64 = 30;

#[contract]
pub struct StakingContract;

#[contractimpl]
impl StakingContract {
    pub fn initialize(env: Env, reward_token: Address, token_admin: Address) {
        token_admin.require_auth();
        assert!(
            !env.storage().instance().has(&DataKey::TokenAdmin),
            "already initialized"
        );

        env.storage()
            .instance()
            .set(&DataKey::RewardToken, &reward_token);
        env.storage()
            .instance()
            .set(&DataKey::TokenAdmin, &token_admin);

        env.events().publish(
            (symbol_short!("INIT"), symbol_short!("staking")),
            token_admin,
        );
    }

    pub fn stake(
        env: Env,
        amount: i128,
        account: Address,
        plan: u64,
        token_id: Address,
    ) -> Result<(StakeDetail, Address), Error> {
        account.require_auth();

        if plan != PLAN1 && plan != PLAN2 && plan != PLAN3 {
            return Err(Error::PlanNotExist);
        }

        let end_time = Self::get_end_time(env.clone(), plan);

        let stake_detail = Self::get_stake_detail(env.clone(), account.clone());
        let mut total_staked = amount;

        if stake_detail.owner == account {
            total_staked = total_staked + stake_detail.total_staked;
        }

        let stake_detail = StakeDetail {
            owner: account.clone(),
            total_staked: total_staked,
            last_staked: amount,
            reward_amount: 0,
            plan: plan,
            end_time: end_time,
        };

        let real_amount = amount*10000000;

        let client = token::Client::new(&env.clone(), &token_id);

        client.transfer(&account, &env.current_contract_address(), &real_amount);

        env.storage()
            .instance()
            .set(&account, &stake_detail.clone());

        env.events().publish(
            (symbol_short!("stake"), symbol_short!("amount")),
            stake_detail.clone(),
        );

        return Ok((stake_detail, env.current_contract_address()));
    }

    pub fn unstake(env: Env, account: Address, token_id: Address) -> Result<StakeDetail, Error> {
        account.require_auth();

        let mut stake_detail = Self::get_stake_detail(env.clone(), account.clone());
        if stake_detail.owner == env.current_contract_address() {
            return Err(Error::StakeDetailNotExist);
        }

        let current_time = Self::get_current_time(env.clone());
        if stake_detail.end_time >= current_time {
            return Err(Error::PlanNotFinished);
        }

        let client = token::Client::new(&env.clone(), &token_id);
        client.transfer(
            &env.current_contract_address(),
            &stake_detail.owner,
            &stake_detail.total_staked,
        );

        stake_detail.total_staked = 0;

        env.storage()
            .instance()
            .set(&account, &stake_detail.clone());

        env.events().publish(
            (symbol_short!("unstake"), symbol_short!("amount")),
            stake_detail.clone(),
        );

        return Ok(stake_detail);
    }

    pub fn claim_reward(env: Env, account: Address) -> (StakeDetail, i128) {
        account.require_auth();

        let data = Self::calculate_reward(env.clone(), account.clone()).unwrap();

        let total_reward = data.1.clone();
        let mut stake_detail = data.0.clone();

        let reward_token_id = Self::get_reward_token(env.clone());
        let client_admin = token::Client::new(&env.clone(), &reward_token_id);

        client_admin.transfer(
            &env.current_contract_address(),
            &stake_detail.owner,
            &total_reward,
        );

        stake_detail.owner = env.current_contract_address();
        stake_detail.total_staked = 0;
        stake_detail.reward_amount = 0;
        stake_detail.plan = 0;
        stake_detail.end_time = 0;

        env.storage().instance().set(&account, &stake_detail);

        return data;
    }

    pub fn calculate_reward(env: Env, account: Address) -> Result<(StakeDetail, i128), Error> {
        let stake_detail = Self::get_stake_detail(env.clone(), account.clone());

        if stake_detail.total_staked == 0{
            return Err(Error::ZeroStake);
        }

        if stake_detail.owner == env.current_contract_address() {
            return Err(Error::StakeDetailNotExist);
        }

        let plan = stake_detail.plan;
        let mut reward_amount = 0;

        if plan == 7 {
            reward_amount = 14;
        } else if plan == 14 {
            reward_amount = 28;
        } else if plan == 30 {
            reward_amount = 60;
        }

        return Ok((stake_detail, reward_amount));
    }

    fn get_end_time(env: Env, plan: u64) -> u64 {
        let current_timestamp = Self::get_current_time(env);

        let seconds_in_a_day: u64 = 24 * 60 * 60;
        let plan_days_in_seconds = plan * seconds_in_a_day;

        let end_timestamp = current_timestamp + plan_days_in_seconds;

        return end_timestamp;
    }

    fn get_current_time(env: Env) -> u64 {
        let current_timestamp = env.ledger().timestamp();

        return current_timestamp;
    }

    pub fn get_stake_detail(env: Env, account: Address) -> StakeDetail {
        let stake_detail: StakeDetail =
            env.storage()
                .instance()
                .get(&account)
                .unwrap_or(StakeDetail {
                    owner: env.current_contract_address(),
                    total_staked: 0,
                    last_staked: 0,
                    reward_amount: 0,
                    plan: 0,
                    end_time: 0,
                });

        return stake_detail;
    }

    pub fn get_reward_token(env: Env) -> Address {
        env.storage()
            .instance()
            .get::<DataKey, Address>(&DataKey::RewardToken)
            .expect("none")
    }
}

#[cfg(test)]
mod test;

mod testutils;
