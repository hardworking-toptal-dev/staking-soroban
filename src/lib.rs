#![no_std]
use soroban_sdk::{contracterror, contractimpl, contracttype, token, Address, Env, Symbol};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    StakeDetailNotExist = 1,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StakeDetail {
    owner: Address,
    total_staked: i128,
    last_staked: i128,
    reward_amount: i128,
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

    pub fn stake(env: Env, amount: i128, account: Address, token_id: Address) {
        account.require_auth();

        let stake_detail = StakeDetail {
            owner: account.clone(),
            total_staked: amount,
            last_staked: amount,
            reward_amount: 0,
        };

        let client = token::Client::new(&env.clone(), &token_id);

        client.transfer(&account, &env.current_contract_address(), &amount);

        env.storage().set(&account, &stake_detail);

        env.events().publish(
            (Symbol::short("stake"), Symbol::short("amount")),
            stake_detail,
        );
    }

    pub fn unstake(env: Env, account: Address, token_id: Address) {
        account.require_auth();

        let mut stake_detail = Self::get_stake_detail(env.clone(), account.clone()).unwrap();

        let client = token::Client::new(&env.clone(), &token_id);
        client.transfer(&env.current_contract_address(), &stake_detail.owner, &stake_detail.total_staked);

        stake_detail.total_staked = 0;
        
        env.storage().set(&account, &stake_detail);

        env.events().publish(
            (Symbol::short("unstake"), Symbol::short("amount")),
            stake_detail,
        );
    }

    pub fn claim_reward(env: Env) {}

    pub fn get_stake_detail(env: Env, account: Address) -> Result<StakeDetail, Error> {
        let stake_detail: StakeDetail = env
            .storage()
            .get(&account)
            .unwrap_or(Ok(StakeDetail {
                owner: env.current_contract_address(),
                total_staked: 0,
                last_staked: 0,
                reward_amount: 0,
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
