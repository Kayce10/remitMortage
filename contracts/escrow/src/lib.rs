#![no_std]

mod errors;
mod types;

use soroban_sdk::{contract, contractimpl, Env};

/// Escrow Contract
///
/// Holds borrower contributions toward a 30% down-payment savings target.
/// Accepts USDC deposits, tracks individual balances, and releases funds
/// once the savings target is met — or refunds the borrower on early withdrawal.
#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    /// Placeholder — will be replaced with full initialization in the next commit.
    pub fn version(env: Env) -> u32 {
        env.storage().instance().extend_ttl(100, 100);
        1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_version() {
        let env = Env::default();
        let contract_id = env.register(EscrowContract, ());
        let client = EscrowContractClient::new(&env, &contract_id);
        assert_eq!(client.version(), 1);
    }
}
