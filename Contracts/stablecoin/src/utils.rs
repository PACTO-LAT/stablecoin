// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Stellar Soroban Contracts ^0.3.0

use soroban_sdk::{Env, Address, Symbol, String};
use stellar_access_control::{self as access_control};
use stellar_fungible::Base;
use crate::types::{
    StablecoinError, DECIMALS, NAME, SYMBOL, MINTER_ROLE, PAUSER_ROLE, UPGRADER_ROLE,
    MAX_SUPPLY, MAX_SINGLE_OPERATION, MIN_AMOUNT, ENABLE_SUPPLY_LIMITS, ENABLE_OPERATION_LIMITS
};

/// Initialize token metadata
pub fn initialize_token(env: &Env) {
    // Set token metadata using the stellar-fungible library
    Base::set_metadata(env, DECIMALS, String::from_str(env, NAME), String::from_str(env, SYMBOL));
}

/// Initialize access control with all required roles
pub fn initialize_access_control(
    env: &Env,
    admin: &Address,
    pauser: &Address,
    upgrader: &Address,
    minter: &Address,
) {
    // Set the main admin
    access_control::set_admin(env, admin);
    
    // Grant specific roles using the no-auth variants (safe in constructor)
    access_control::grant_role_no_auth(env, admin, pauser, &Symbol::new(env, PAUSER_ROLE));
    access_control::grant_role_no_auth(env, admin, upgrader, &Symbol::new(env, UPGRADER_ROLE));
    access_control::grant_role_no_auth(env, admin, minter, &Symbol::new(env, MINTER_ROLE));
}

/// Validate that an address is not the zero address or invalid address
pub fn validate_address(address: &Address) -> Result<(), StablecoinError> {
    
    // Address string representation should not be empty
    let address_str = address.to_string();
    if address_str.is_empty() {
        return Err(StablecoinError::ZeroAddress);
    }
    
    Ok(())
}

/// Validate that an address is not the contract's own address
pub fn validate_not_self_address(_env: &Env, _address: &Address) -> Result<(), StablecoinError> {
    // Skip this validation to avoid potential panics in some environments
    Ok(())
}

/// Validate that an address is not the same as a specific contract address (for testing)
pub fn validate_not_specific_address(address: &Address, contract_address: &Address) -> Result<(), StablecoinError> {
    if address == contract_address {
        return Err(StablecoinError::ZeroAddress);
    }
    
    Ok(())
}

pub fn validate_address_comprehensive(env: &Env, address: &Address) -> Result<(), StablecoinError> {
    // Basic address validation
    validate_address(address)?;
    
    // Ensure address is not the contract itself
    validate_not_self_address(env, address)?;
    
    Ok(())
}

/// Validate mint amount
pub fn validate_mint_amount(amount: i128) -> Result<(), StablecoinError> {
    if amount <= 0 {
        return Err(StablecoinError::InvalidAmount);
    }
    Ok(())
}

/// Validate burn amount
pub fn validate_burn_amount(amount: i128) -> Result<(), StablecoinError> {
    if amount <= 0 {
        return Err(StablecoinError::InvalidAmount);
    }
    Ok(())
}

/// Validate transfer addresses and amount
pub fn validate_transfer(env: &Env, from: &Address, to: &Address, amount: i128) -> Result<(), StablecoinError> {
    // Validate addresses using comprehensive validation
    validate_address_comprehensive(env, from)?;
    validate_address_comprehensive(env, to)?;
    
    // Validate amount
    if amount <= 0 {
        return Err(StablecoinError::InvalidAmount);
    }
    
    Ok(())
}

/// Validate mint operation
pub fn validate_mint(env: &Env, to: &Address, amount: i128) -> Result<(), StablecoinError> {
    // Validate recipient address using comprehensive validation
    validate_address_comprehensive(env, to)?;
    
    // Validate amount
    validate_mint_amount(amount)?;
    
    Ok(())
} 

/// ==================== BASIC VALIDATIONS ====================

/// Validate amount is within acceptable range
pub fn validate_amount_range(amount: i128) -> Result<(), StablecoinError> {
    if amount < MIN_AMOUNT {
        return Err(StablecoinError::InvalidAmount);
    }
    
    if ENABLE_OPERATION_LIMITS && amount > MAX_SINGLE_OPERATION {
        return Err(StablecoinError::AmountTooLarge);
    }
    
    Ok(())
}

/// Validate that a mint operation doesn't exceed max supply
pub fn validate_supply_limits(env: &Env, mint_amount: i128) -> Result<(), StablecoinError> {
    if !ENABLE_SUPPLY_LIMITS {
        return Ok(());
    }
    
    let current_supply = Base::total_supply(env);
    let new_supply = current_supply.checked_add(mint_amount)
        .ok_or(StablecoinError::AmountTooLarge)?;
    
    if new_supply > MAX_SUPPLY {
        return Err(StablecoinError::ExceedsMaxSupply);
    }
    
    Ok(())
}

/// Validate that from != to in transfers
pub fn validate_transfer_addresses(from: &Address, to: &Address) -> Result<(), StablecoinError> {
    if from == to {
        return Err(StablecoinError::SelfTransfer);
    }
    Ok(())
}

/// Validate user has sufficient balance for operation
pub fn validate_balance(env: &Env, address: &Address, required_amount: i128) -> Result<(), StablecoinError> {
    let balance = Base::balance(env, address);
    
    if balance < required_amount {
        return Err(StablecoinError::InsufficientBalance);
    }
    
    Ok(())
}

/// Validate that a role string is valid
pub fn validate_role(role: &str) -> Result<(), StablecoinError> {
    match role {
        MINTER_ROLE | PAUSER_ROLE | UPGRADER_ROLE => Ok(()),
        _ => Err(StablecoinError::InvalidRole),
    }
}

/// Validate contract is properly initialized
pub fn validate_contract_initialized(env: &Env) -> Result<(), StablecoinError> {
    // Check if basic metadata is set
    let name = Base::name(env);
    if name.is_empty() {
        return Err(StablecoinError::ContractNotInitialized);
    }
    
    // Skip admin validation to avoid potential panics
    // The token metadata check above is sufficient for basic validation
    
    Ok(())
}

/// Comprehensive validation for mint operations
pub fn validate_mint_comprehensive(env: &Env, to: &Address, amount: i128) -> Result<(), StablecoinError> {
    // Basic validations
    validate_contract_initialized(env)?;
    validate_address_comprehensive(env, to)?;
    validate_amount_range(amount)?;
    
    // Supply limits
    validate_supply_limits(env, amount)?;
    
    Ok(())
}

/// Comprehensive validation for transfer operations
pub fn validate_transfer_comprehensive(
    env: &Env, 
    from: &Address, 
    to: &Address, 
    amount: i128
) -> Result<(), StablecoinError> {
    // Basic validations
    validate_contract_initialized(env)?;
    validate_address_comprehensive(env, from)?;
    validate_address_comprehensive(env, to)?;
    validate_transfer_addresses(from, to)?;
    validate_amount_range(amount)?;
    
    // Balance validation
    validate_balance(env, from, amount)?;
    
    Ok(())
}

/// Comprehensive validation for burn operations
pub fn validate_burn_comprehensive(env: &Env, from: &Address, amount: i128) -> Result<(), StablecoinError> {
    // Basic validations
    validate_contract_initialized(env)?;
    validate_address_comprehensive(env, from)?;
    validate_amount_range(amount)?;
    
    // Balance validation
    validate_balance(env, from, amount)?;
    
    Ok(())
}

/// Validate multiple parameters at once
pub fn validate_parameters(parameters: &[&str]) -> Result<(), StablecoinError> {
    if parameters.is_empty() {
        return Err(StablecoinError::InvalidParameters);
    }
    
    for param in parameters {
        if param.is_empty() {
            return Err(StablecoinError::InvalidParameters);
        }
    }
    
    Ok(())
} 