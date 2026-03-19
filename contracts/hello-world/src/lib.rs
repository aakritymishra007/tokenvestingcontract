#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Address, Symbol};

#[contract]
pub struct TokenVestingContract;

#[contractimpl]
impl TokenVestingContract {

    // Initialize vesting details
    pub fn init(
        env: Env,
        beneficiary: Address,
        amount: i128,
        release_time: u64,
    ) {
        env.storage().instance().set(&Symbol::short("beneficiary"), &beneficiary);
        env.storage().instance().set(&Symbol::short("amount"), &amount);
        env.storage().instance().set(&Symbol::short("release"), &release_time);
        env.storage().instance().set(&Symbol::short("claimed"), &false);
    }

    // Claim vested tokens after release time
    pub fn claim(env: Env, caller: Address) -> i128 {
        let beneficiary: Address =
            env.storage().instance().get(&Symbol::short("beneficiary")).unwrap();

        let release_time: u64 =
            env.storage().instance().get(&Symbol::short("release")).unwrap();

        let amount: i128 =
            env.storage().instance().get(&Symbol::short("amount")).unwrap();

        let claimed: bool =
            env.storage().instance().get(&Symbol::short("claimed")).unwrap();

        if caller != beneficiary {
            panic!("Not authorized");
        }

        if env.ledger().timestamp() < release_time {
            panic!("Tokens are still locked");
        }

        if claimed {
            panic!("Tokens already claimed");
        }

        env.storage().instance().set(&Symbol::short("claimed"), &true);
        amount
    }
}