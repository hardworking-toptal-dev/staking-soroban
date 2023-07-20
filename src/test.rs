#![cfg(test)]

use super::testutils::{register_test_contract as register_contract, StakingContract};
use super::StakingContractClient;
use soroban_sdk::token::Client;
use soroban_sdk::{testutils::Address as _, token::Client as Token, Address, Env};

use crate::StakeDetail;

fn create_staking_contract() -> (
    StakingContractClient<'static>,
    Env,
    Client<'static>,
    Address,
    Address,
) {
    let env = Env::default();
    env.mock_all_auths();

    let id = register_contract(&env);
    let stating_contract = StakingContract::new(&env, id.clone());

    // Reward token creation
    let token_admin = Address::random(&env);
    let contract_reward_token = env.register_stellar_asset_contract(token_admin.clone());
    let reward_token = Token::new(&env, &contract_reward_token);

    // Mint some Rewards tokens to work with
    reward_token.mint(&token_admin, &50000);

    let client = stating_contract.client();

    // initialize the accounts, Reward token and Admin Account
    client.initialize(&contract_reward_token, &token_admin);

    (
        client,
        env.clone(),
        reward_token,
        contract_reward_token,
        token_admin,
    )
}

struct Setup {
    env: Env,
    client: StakingContractClient<'static>,
    reward_token_client: Client<'static>,
    reward_token_address: Address,
    stake_token_client: Client<'static>,
    stake_token_address: Address,
    staker_acc1: Address,
    stake_amount: i128,
    plan: u64,
    end_time: u64,
    contract_address: Address,
}

impl Setup {
    fn new() -> Self {
        let contract_client = create_staking_contract();
        let client = contract_client.0;
        let env = contract_client.1;
        let reward_token_client = contract_client.2;
        let reward_token_address = contract_client.3;
        let token_admin = contract_client.4;

        let stake_token_address = env.register_stellar_asset_contract(token_admin.clone());
        let stake_token_client = Token::new(&env, &stake_token_address);

        let stake_amount: i128 = 100;
        let plan = 7;

        let staker_acc1 = Address::random(&env);
        stake_token_client.mint(&staker_acc1, &1000);

        let contract_address = client
            .stake(&stake_amount, &staker_acc1, &plan, &stake_token_address)
            .1;

        client
            .stake(&stake_amount, &staker_acc1, &plan, &stake_token_address)
            .1;

        let end_time = get_end_time(env.clone(), plan);

        Self {
            env: env,
            client,
            reward_token_client,
            reward_token_address,
            stake_token_client,
            stake_token_address,
            staker_acc1,
            stake_amount,
            plan,
            end_time,
            contract_address,
        }
    }
}

fn get_end_time(env: Env, plan: u64) -> u64 {
    let current_timestamp = get_current_time(env);

    let seconds_in_a_day: u64 = 24 * 60 * 60;
    let plan_days_in_seconds = plan * seconds_in_a_day;

    let end_timestamp = current_timestamp + plan_days_in_seconds;

    return end_timestamp;
}

fn get_current_time(env: Env) -> u64 {
    let current_timestamp = env.ledger().timestamp();

    return current_timestamp;
}

#[test]
fn test_all_stakes() {
    let setup = Setup::new();

    let stake_detail = StakeDetail {
        owner: setup.staker_acc1.clone(),
        total_staked: setup.stake_amount + setup.stake_amount,
        last_staked: setup.stake_amount,
        reward_amount: 0,
        plan: setup.plan,
        end_time: setup.end_time,
    };

    // check stake detail
    let detail = setup.client.get_stake_detail(&setup.staker_acc1);
    assert_eq!(stake_detail, detail);

    // check the contract address balance
    let contract_balance = setup.stake_token_client.balance(&setup.contract_address);
    assert_eq!(contract_balance, detail.total_staked);
}

// #[test]
// fn test_all_unstake() {
//     let setup = Setup::new();

//     let stake_detail = StakeDetail {
//         owner: setup.staker_acc1.clone(),
//         total_staked: 0,
//         last_staked: setup.stake_amount,
//         reward_amount: 0,
//         plan: setup.plan,
//         end_time: setup.end_time,
//     };

//     let detail = setup
//         .client
//         .unstake(&setup.staker_acc1, &setup.stake_token_address);
//     assert_eq!(detail, stake_detail);

//     // check the contract address balance
//     let contract_balance = setup.stake_token_client.balance(&setup.contract_address);
//     assert_eq!(contract_balance, detail.total_staked);
// }

// #[test]
// fn test_all_claim_rewards() {
//     let setup = Setup::new();

//     let data = setup.client.claim_reward(&setup.staker_acc1);
//     let total_reward = data.1;

//     assert_eq!(total_reward, setup.plan as i128 + setup.plan as i128)
// }
