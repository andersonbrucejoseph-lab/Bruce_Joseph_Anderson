#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short,
    Address,
    Env,
    Symbol,
};

#[contract]
pub struct SkillLock;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Escrow(u64),
    Reputation(Address),
}

#[derive(Clone)]
#[contracttype]
pub struct Escrow {
    client: Address,
    freelancer: Address,
    amount: i128,
    released: bool,
}

#[contractimpl]
impl SkillLock {
    /// register escrow job
    pub fn create_escrow(
        env: Env,
        job_id: u64,
        client: Address,
        freelancer: Address,
        amount: i128,
    ) {
        client.require_auth();

        if env.storage()
            .persistent()
            .has(&DataKey::Escrow(job_id))
        {
            panic!("duplicate contract");
        }

        let deal = Escrow {
            client,
            freelancer,
            amount,
            released: false,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Escrow(job_id), &deal);
    }

    /// release milestone payment
    pub fn release_payment(
        env: Env,
        job_id: u64,
        client: Address,
    ) {
        client.require_auth();

        let key = DataKey::Escrow(job_id);
        let mut deal: Escrow = env.storage()
            .persistent()
            .get(&key)
            .unwrap();

        if deal.released {
            panic!("already paid");
        }

        deal.released = true;

        env.storage()
            .persistent()
            .set(&key, &deal);

        env.events().publish(
            (symbol_short!("paid"), job_id),
            deal.amount,
        );

        Self::mint_reputation(env, deal.freelancer);
    }

    /// mint freelancer credential
    pub fn mint_reputation(
        env: Env,
        freelancer: Address,
    ) {
        let rep: u32 = env.storage()
            .persistent()
            .get(&DataKey::Reputation(freelancer.clone()))
            .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&DataKey::Reputation(freelancer.clone()), &(rep + 1));

        env.events().publish(
            (Symbol::new(&env, "rep"), freelancer),
            rep + 1,
        );
    }

    /// verification check
    pub fn reputation_of(
        env: Env,
        freelancer: Address,
    ) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::Reputation(freelancer))
            .unwrap_or(0)
    }
}
