// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Stellar Soroban Contracts ^0.3.0

use soroban_sdk::{Address, Env};
use stellar_access_control::{self as access_control};
use crate::types::{create_role_symbol, UPGRADER_ROLE};

/// Upgradeable extension for the stablecoin
pub struct StablecoinUpgradeable;

impl StablecoinUpgradeable {
    /// Require authorization for upgrades
    pub fn require_auth(env: &Env, operator: &Address) {
        access_control::ensure_role(env, operator, &create_role_symbol(env, UPGRADER_ROLE));
    }

    /// Check if an address can perform upgrades
    pub fn can_upgrade(env: &Env, operator: &Address) -> bool {
        access_control::has_role(env, operator, &create_role_symbol(env, UPGRADER_ROLE)).is_some()
    }
}

/// Trait for implementing upgradeable functionality
pub trait StablecoinUpgradeableImpl {
    /// Require authorization for upgrade operations
    fn _require_auth(env: &Env, operator: &Address);
}

/// Helper functions for upgrade operations
pub mod upgrade_utils {
    use super::*;

    /// Validate upgrade permissions
    pub fn validate_upgrade_permissions(env: &Env, operator: &Address) -> Result<(), UpgradeError> {
        if !access_control::has_role(env, operator, &create_role_symbol(env, UPGRADER_ROLE)).is_some() {
            return Err(UpgradeError::Unauthorized);
        }
        Ok(())
    }

    /// Check if upgrade is allowed in current state
    pub fn can_upgrade_now(env: &Env, operator: &Address) -> bool {
        // Check if paused (upgrades might be restricted when paused)
        if crate::extensions::pausable::StablecoinPausable::paused(env) {
            return false;
        }
        
        // Check upgrader role
        access_control::has_role(env, operator, &create_role_symbol(env, UPGRADER_ROLE)).is_some()
    }

    /// Get upgrade status
    pub fn get_upgrade_status(env: &Env, operator: &Address) -> UpgradeStatus {
        if !access_control::has_role(env, operator, &create_role_symbol(env, UPGRADER_ROLE)).is_some() {
            return UpgradeStatus::Unauthorized;
        }

        if crate::extensions::pausable::StablecoinPausable::paused(env) {
            return UpgradeStatus::Restricted;
        }

        UpgradeStatus::Authorized
    }
}

/// Enum representing upgrade status
#[derive(Debug, Clone, PartialEq)]
pub enum UpgradeStatus {
    Authorized,
    Unauthorized,
    Restricted,
}

/// Enum representing upgrade errors
#[derive(Debug, Clone, PartialEq)]
pub enum UpgradeError {
    Unauthorized,
    ContractPaused,
    InvalidParameters,
} 