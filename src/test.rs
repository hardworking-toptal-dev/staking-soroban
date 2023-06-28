#![cfg(test)]

use super::testutils::{register_test_contract as register_contract, StakingContract};
use super::StakingContractClient;
use soroban_sdk::token::Client;
use soroban_sdk::{testutils::Address as _, token::Client as Token, Address, Env};

fn create_staking_contract() -> (
    StakingContractClient<'static>,
    Env,
    Client<'static>,
    Address,
) {
    let env = Env::default();
    env.mock_all_auths();

    let id = register_contract(&env);
    let crowdfund = StakingContract::new(&env, id.clone());

    // ARTY token creation
    let token_admin = Address::random(&env);
    let contract_reward_token = env.register_stellar_asset_contract(token_admin.clone());
    let reward_token = Token::new(&env, &contract_reward_token);

    // Mint some Rewards tokens to work with
    reward_token.mint(&token_admin, &50000);

    let client = crowdfund.client();

    // initialize the accounts, Reward token and Admin Account
    client.initialize(&contract_reward_token, &token_admin);

    (client, env.clone(), reward_token, token_admin)
}

struct Setup {
    env: Env,
    client: StakingContractClient<'static>,
    reward_token_client: Client<'static>,
    reward_token_address: Address,
}

impl Setup {
    fn new() -> Self {
        let contract_client = create_staking_contract();
        let client = contract_client.0;
        let env = contract_client.1;
        let reward_token_client = contract_client.2;
        let reward_token_address = contract_client.3;

        Self {
            env: env,
            client,
            reward_token_client,
            reward_token_address,
        }
    }
}

#[test]
fn test_all_stakes() {
    let setup = Setup::new();
}

#[test]
fn test_all_claim_rewards() {
    let setup = Setup::new();
}


#[test]
fn test_all_unstake() {
    let setup = Setup::new();
}
