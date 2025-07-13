// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Stellar Soroban Contracts ^0.3.0

use soroban_sdk::{Address, Env};
use stellar_pausable::{self as pausable};
use stellar_access_control_macros::only_role;

/// Pausable extension for the stablecoin
pub struct StablecoinPausable;

impl StablecoinPausable {
    /// Check if the contract is paused
    pub fn paused(env: &Env) -> bool {
        pausable::paused(env)
    }

    /// Pause the contract (only pauser role)
    #[only_role(caller, "pauser")]
    pub fn pause(env: &Env, caller: &Address) {
        pausable::pause(env);
    }

    /// Unpause the contract (only pauser role)
    #[only_role(caller, "pauser")]
    pub fn unpause(env: &Env, caller: &Address) {
        pausable::unpause(env);
    }
}

/// Trait for implementing pausable functionality
pub trait StablecoinPausableImpl {
    /// Check if the contract is paused
    fn paused(env: &Env) -> bool;

    /// Pause the contract
    fn pause(env: &Env, caller: Address);

    /// Unpause the contract
    fn unpause(env: &Env, caller: Address);
}

/// Helper functions for pausable operations
pub mod pause_utils {
    use super::*;

    /// Check if an operation should be blocked due to pause
    pub fn require_not_paused(env: &Env) {
        if pausable::paused(env) {
            panic!("Contract is paused");
        }
    }

    /// Check if an operation should be blocked due to not being paused
    pub fn require_paused(env: &Env) {
        if !pausable::paused(env) {
            panic!("Contract is not paused");
        }
    }

    /// Get pause status with additional context
    pub fn get_pause_status(env: &Env) -> PauseStatus {
        if pausable::paused(env) {
            PauseStatus::Paused
        } else {
            PauseStatus::NotPaused
        }
    }
}

/// Enum representing pause status
#[derive(Debug, Clone, PartialEq)]
pub enum PauseStatus {
    Paused,
    NotPaused,
} 