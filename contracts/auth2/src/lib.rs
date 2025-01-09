#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, Address, Env};

#[contract]
pub struct PaymentContract;

#[derive(Debug)]
#[contracterror]
pub enum PaymentError {
    TimeNotReached = 1,
    AlreadyPaid = 2,
    TransferFailed = 3,
}

#[contractimpl]
impl PaymentContract {
    /// Schedule a payment with recipient, amount, and start time
    pub fn schedule_payment(env: Env, recipient: Address, amount: i128, start_time: u64) {
        let payment_data = (recipient, amount, start_time);
        env.storage().instance().set(&"payment", &payment_data);
    }

    /// Execute the payment after the delay
    pub fn execute_payment(env: Env, token_contract_id: Address) -> Result<(), PaymentError> {
        let _now = env.ledger().timestamp();
        let payment: Option<(Address, i128, u64)> = env.storage().instance().get(&"payment");
        Ok(())
    }
}
