// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Stellar Soroban Contracts ^0.3.0

use soroban_sdk::{Address, Env};
use stellar_fungible::Base;
use stellar_pausable_macros::when_not_paused;
use crate::utils::validate_burn_amount;

/// Burnable extension for the stablecoin
pub struct StablecoinBurnable;

impl StablecoinBurnable {
    /// Burn tokens from an account
    #[when_not_paused]
    pub fn burn(env: &Env, from: &Address, amount: i128) {
        validate_burn_amount(amount).unwrap();
        Base::burn(env, from, amount);
    }

    /// Burn tokens from an account on behalf of a spender
    #[when_not_paused]
    pub fn burn_from(env: &Env, spender: &Address, from: &Address, amount: i128) {
        validate_burn_amount(amount).unwrap();
        Base::burn_from(env, spender, from, amount);
    }
}

/// Trait for implementing burnable functionality
pub trait StablecoinBurnableImpl {
    /// Burn tokens from the caller's account
    fn burn(env: &Env, from: Address, amount: i128);

    /// Burn tokens from another account using allowance
    fn burn_from(env: &Env, spender: Address, from: Address, amount: i128);
}

/// Helper functions for burn operations
pub mod burns {
    use super::*;

    /// Check if an account has sufficient balance to burn
    pub fn can_burn(env: &Env, account: &Address, amount: i128) -> bool {
        let balance = Base::balance(env, account);
        balance >= amount && amount > 0
    }

    /// Get the maximum amount that can be burned from an account
    pub fn max_burnable_amount(env: &Env, account: &Address) -> i128 {
        Base::balance(env, account)
    }

    /// Check if a spender can burn from an account
    pub fn can_burn_from(env: &Env, spender: &Address, from: &Address, amount: i128) -> bool {
        let allowance = Base::allowance(env, from, spender);
        let balance = Base::balance(env, from);
        allowance >= amount && balance >= amount && amount > 0
    }
} 