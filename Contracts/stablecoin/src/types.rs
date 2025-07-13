// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Stellar Soroban Contracts ^0.3.0

use soroban_sdk::{Env, String, Symbol, contracterror, contracttype};

/// Stablecoin metadata constants
pub const DECIMALS: u32 = 0;
pub const NAME: &str = "Costa Rica Colon";
pub const SYMBOL: &str = "CRCX";

/// Role constants for access control
pub const PAUSER_ROLE: &str = "pauser";
pub const UPGRADER_ROLE: &str = "upgrader";
pub const MINTER_ROLE: &str = "minter";

/// Operational limits for validation
pub const MAX_SUPPLY: i128 = 1_000_000_000; // 1 billion tokens
pub const MAX_SINGLE_OPERATION: i128 = 100_000_000; // 100 million tokens max per operation
pub const MIN_AMOUNT: i128 = 5; // Minimum 1 whole token - smallest transferable amount

/// Validation configuration
pub const ENABLE_SUPPLY_LIMITS: bool = true;
pub const ENABLE_OPERATION_LIMITS: bool = true;
pub const ENABLE_STRICT_VALIDATION: bool = true;

/// Events
pub const MINT_EVENT: &str = "mint";
pub const BURN_EVENT: &str = "burn";
pub const TRANSFER_EVENT: &str = "transfer";
pub const PAUSE_EVENT: &str = "pause";
pub const UNPAUSE_EVENT: &str = "unpause";

/// Error types for the stablecoin contract
#[contracterror]
#[derive(Debug, Clone, PartialEq)]
pub enum StablecoinError {
    InvalidAmount = 1,
    InsufficientBalance = 2,
    InsufficientAllowance = 3,
    Paused = 4,
    NotPaused = 5,
    ZeroAddress = 6,
    Unauthorized = 7,
    AlreadyInitialized = 8,
    ExceedsMaxSupply = 9,
    AmountTooLarge = 10,
    InvalidParameters = 11,
    SelfTransfer = 12,
    InvalidRole = 13,
    ContractNotInitialized = 14,
}

/// Token statistics for monitoring
#[contracttype]
#[derive(Debug, Clone, PartialEq)]
pub struct TokenStats {
    pub total_supply: i128,
    pub total_minted: i128,
    pub total_burned: i128,
    pub holders_count: u32,
}

/// Helper function to create role symbols
pub fn create_role_symbol(env: &Env, role: &str) -> Symbol {
    Symbol::new(env, role)
}

/// Helper function to create event symbols
pub fn create_event_symbol(env: &Env, event: &str) -> Symbol {
    Symbol::new(env, event)
}

/// Helper function to create metadata strings
pub fn create_metadata_strings(env: &Env) -> (String, String) {
    (
        String::from_str(env, NAME),
        String::from_str(env, SYMBOL),
    )
}

/// Convert errors to human-readable messages
pub fn error_to_message(error: StablecoinError) -> &'static str {
    match error {
        StablecoinError::InvalidAmount => "Invalid amount: must be a positive integer",
        StablecoinError::InsufficientBalance => "Insufficient balance for operation",
        StablecoinError::InsufficientAllowance => "Insufficient allowance for operation",
        StablecoinError::Paused => "Contract is paused",
        StablecoinError::NotPaused => "Contract is not paused",
        StablecoinError::ZeroAddress => "Invalid address: zero address not allowed",
        StablecoinError::Unauthorized => "Unauthorized: insufficient permissions",
        StablecoinError::AlreadyInitialized => "Contract already initialized",
        StablecoinError::ExceedsMaxSupply => "Operation would exceed maximum supply",
        StablecoinError::AmountTooLarge => "Amount too large for this operation",
        StablecoinError::InvalidParameters => "Invalid parameters provided",
        StablecoinError::SelfTransfer => "Cannot transfer to same address",
        StablecoinError::InvalidRole => "Invalid or unrecognized role",
        StablecoinError::ContractNotInitialized => "Contract not properly initialized",
    }
} 