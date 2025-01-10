#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, Address, Env, IntoVal, Symbol, Vec};

#[contract]
pub struct PaymentContract;

#[contracterror]
#[derive(Debug)]
pub enum PaymentError {
    TimeNotReached=1,
    AlreadyPaid=2,
    TransferFailed=3,
}

#[contractimpl]
impl PaymentContract {
    /// Schedule a payment with recipient, amount, and start time
    pub fn schedule_payment(env: Env, recipient: Address, amount: i128, start_time: u64) {
        let payment_data = (recipient, amount, start_time);
        env.storage().instance().set(&"payment", &payment_data);
    }

    /// Execute the payment after the delay
    pub fn execute_payment(env: Env, token_contract_id: Address, sender: Address) -> Result<(), PaymentError> {
        let now = env.ledger().timestamp();
        let payment: Option<(Address, i128, u64)> = env.storage().instance().get(&"payment");

        if let Some((recipient, amount, start_time)) = payment {
            if now < start_time + 48 * 3600 {
                return Err(PaymentError::TimeNotReached);
            }

            // Prepare arguments as a `Vec<Val>`
            let args = Vec::from_slice(
                &env,
                &[
                    sender.into_val(&env),
                    recipient.into_val(&env),
                    amount.into_val(&env),
                ],
            );

            // Explicitly specify the return type for `invoke_contract`
            let transfer_result: Result<(), soroban_sdk::Error> = env.invoke_contract(
                &token_contract_id,
                &Symbol::new(&env,"transfer"),
                args,
            );

            // Handle potential transfer failure
            if transfer_result.is_err() {
                return Err(PaymentError::TransferFailed);
            }

            // Remove the payment data after successful transfer
            env.storage().instance().remove(&"payment");
            Ok(())
        } else {
            Err(PaymentError::AlreadyPaid)
        }
    }
}

