// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Stellar Soroban Contracts ^0.3.0

use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, String, Vec};
use stellar_fungible::Base;
use stellar_access_control as access_control;
use stellar_pausable as pausable;

// Import our modular components
use crate::types::{StablecoinError, MINTER_ROLE, PAUSER_ROLE, UPGRADER_ROLE, MINT_EVENT, BURN_EVENT, TRANSFER_EVENT, PAUSE_EVENT, UNPAUSE_EVENT};
use crate::utils::{
    initialize_token, 
    initialize_access_control,
    validate_mint_comprehensive,
    validate_transfer_comprehensive,
    validate_burn_comprehensive,
};

/// Main stablecoin contract
#[contract]
pub struct MyStablecoin;

#[contractimpl]
impl MyStablecoin {
    /// Initialize the stablecoin contract
    pub fn initialize(
        env: Env,
        admin: Address,
        pauser: Address,
        upgrader: Address,
        minter: Address,
    ) -> Result<(), StablecoinError> {
        // Initialize token metadata
        initialize_token(&env);
        
        // Initialize access control with all roles
        initialize_access_control(&env, &admin, &pauser, &upgrader, &minter);
        
        Ok(())
    }

    /// Mint tokens to a specific address
    pub fn mint(env: Env, caller: Address, to: Address, amount: i128) -> Result<(), StablecoinError> {
        // Check if contract is paused
        if pausable::paused(&env) {
            return Err(StablecoinError::Paused);
        }
        
        // Authenticate the caller
        caller.require_auth();
        
        // Validate minter role
        access_control::ensure_role(&env, &caller, &Symbol::new(&env, MINTER_ROLE));
        
        // Comprehensive validation for mint operation
        validate_mint_comprehensive(&env, &to, amount)?;
        
        // Mint tokens
        Base::mint(&env, &to, amount);
        
        // Emit mint event
        env.events().publish(
            (Symbol::new(&env, MINT_EVENT), &to),
            amount
        );
        
        Ok(())
    }
    
    /// Transfer tokens between addresses
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) -> Result<(), StablecoinError> {
        // Check if contract is paused
        if pausable::paused(&env) {
            return Err(StablecoinError::Paused);
        }
        
        // Comprehensive validation for transfer operation
        validate_transfer_comprehensive(&env, &from, &to, amount)?;
        
        // Transfer tokens
        Base::transfer(&env, &from, &to, amount);
        
        // Emit transfer event
        env.events().publish(
            (Symbol::new(&env, TRANSFER_EVENT), &from, &to),
            amount
        );
        
        Ok(())
    }
    
    /// Transfer tokens from one address to another with allowance
    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) -> Result<(), StablecoinError> {
        // Check if contract is paused
        if pausable::paused(&env) {
            return Err(StablecoinError::Paused);
        }
        
        // Comprehensive validation for transfer operation
        validate_transfer_comprehensive(&env, &from, &to, amount)?;
        
        // Transfer tokens with allowance
        Base::transfer_from(&env, &spender, &from, &to, amount);
        
        // Emit transfer event
        env.events().publish(
            (Symbol::new(&env, TRANSFER_EVENT), &from, &to),
            amount
        );
        
        Ok(())
    }
    
    /// Burn tokens from a specific address
    pub fn burn(env: Env, from: Address, amount: i128) -> Result<(), StablecoinError> {
        // Check if contract is paused
        if pausable::paused(&env) {
            return Err(StablecoinError::Paused);
        }
        
        // Comprehensive validation for burn operation
        validate_burn_comprehensive(&env, &from, amount)?;
        
        // Burn tokens
        Base::burn(&env, &from, amount);
        
        // Emit burn event
        env.events().publish(
            (Symbol::new(&env, BURN_EVENT), &from),
            amount
        );
        
        Ok(())
    }
    
    /// Burn tokens from a specific address by a burner
    pub fn burn_from(env: Env, spender: Address, from: Address, amount: i128) -> Result<(), StablecoinError> {
        // Check if contract is paused
        if pausable::paused(&env) {
            return Err(StablecoinError::Paused);
        }
        
        // Comprehensive validation for burn operation
        validate_burn_comprehensive(&env, &from, amount)?;
        
        // Burn tokens with allowance
        Base::burn_from(&env, &spender, &from, amount);
        
        // Emit burn event
        env.events().publish(
            (Symbol::new(&env, BURN_EVENT), &from),
            amount
        );
        
        Ok(())
    }

    /// Get token information including metadata and current state
    pub fn get_token_info(env: Env) -> (String, String, u32, i128, bool) {
        (
            Base::name(&env),
            Base::symbol(&env),
            Base::decimals(&env),
            Base::total_supply(&env),
            pausable::paused(&env),
        )
    }

    /// Batch mint tokens to multiple addresses
    pub fn batch_mint(env: Env, caller: Address, recipients: Vec<(Address, i128)>) -> Result<(), StablecoinError> {
        // Check if contract is paused
        if pausable::paused(&env) {
            return Err(StablecoinError::Paused);
        }
        
        // Authenticate the caller
        caller.require_auth();
        
        // Validate minter role
        access_control::ensure_role(&env, &caller, &Symbol::new(&env, MINTER_ROLE));
        
        // Validate and mint to each recipient
        for (account, amount) in recipients.iter() {
            // Validate mint operation (address and amount)
            validate_mint_comprehensive(&env, &account, amount)?;
            
            // Perform the mint
            Base::mint(&env, &account, amount);
            
            // Emit mint event for each recipient
            env.events().publish(
                (Symbol::new(&env, MINT_EVENT), &account),
                amount
            );
        }
        
        Ok(())
    }
    
    /// Pause the contract (only pauser role)
    pub fn pause(env: Env, caller: Address) -> Result<(), StablecoinError> {
        // Authenticate the caller
        caller.require_auth();
        
        // Validate pauser role
        access_control::ensure_role(&env, &caller, &Symbol::new(&env, PAUSER_ROLE));
        
        // Pause the contract
        pausable::pause(&env);
        
        // Emit pause event
        env.events().publish(
            (Symbol::new(&env, PAUSE_EVENT),),
            ()
        );
        
        Ok(())
    }
    
    /// Unpause the contract (only pauser role)
    pub fn unpause(env: Env, caller: Address) -> Result<(), StablecoinError> {
        // Authenticate the caller
        caller.require_auth();
        
        // Validate pauser role
        access_control::ensure_role(&env, &caller, &Symbol::new(&env, PAUSER_ROLE));
        
        // Unpause the contract
        pausable::unpause(&env);
        
        // Emit unpause event
        env.events().publish(
            (Symbol::new(&env, UNPAUSE_EVENT),),
            ()
        );
        
        Ok(())
    }

    /// Get balance of an address
    pub fn balance(env: Env, address: Address) -> i128 {
        Base::balance(&env, &address)
    }

    /// Get allowance between two addresses
    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        Base::allowance(&env, &from, &spender)
    }

    /// Approve spending allowance
    pub fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) -> Result<(), StablecoinError> {
        // Check if contract is paused
        if pausable::paused(&env) {
            return Err(StablecoinError::Paused);
        }
        
        // Approve allowance
        Base::approve(&env, &from, &spender, amount, expiration_ledger);
        
        Ok(())
    }

    /// Get token name
    pub fn name(env: Env) -> String {
        Base::name(&env)
    }

    /// Get token symbol
    pub fn symbol(env: Env) -> String {
        Base::symbol(&env)
    }

    /// Get token decimals
    pub fn decimals(env: Env) -> u32 {
        Base::decimals(&env)
    }

    /// Get total supply
    pub fn total_supply(env: Env) -> i128 {
        Base::total_supply(&env)
    }

    /// Check if contract is paused
    pub fn is_paused(env: Env) -> bool {
        pausable::paused(&env)
    }

    /// Check if address has a specific role
    pub fn has_role_minter(env: Env, address: Address) -> bool {
        let role_symbol = Symbol::new(&env, MINTER_ROLE);
        access_control::has_role(&env, &address, &role_symbol).is_some()
    }

    /// Check if address has pauser role
    pub fn has_role_pauser(env: Env, address: Address) -> bool {
        let role_symbol = Symbol::new(&env, PAUSER_ROLE);
        access_control::has_role(&env, &address, &role_symbol).is_some()
    }

    /// Check if address has upgrader role
    pub fn has_role_upgrader(env: Env, address: Address) -> bool {
        let role_symbol = Symbol::new(&env, UPGRADER_ROLE);
        access_control::has_role(&env, &address, &role_symbol).is_some()
    }

    /// Get admin address
    pub fn get_admin(env: Env) -> Option<Address> {
        access_control::get_admin(&env)
    }
}
