// SPDX-License-Identifier: MIT
// Compatible with OpenZeppelin Stellar Soroban Contracts ^0.3.0

#[cfg(test)]
mod test {
    use soroban_sdk::{testutils::Address as _, Address, Env, Vec, String};
    use crate::contract::{MyStablecoin, MyStablecoinClient};

    #[test]
    fn test_basic_functionality() {
        let env = Env::default();
        env.mock_all_auths();
        
        let admin = Address::generate(&env);
        let minter = Address::generate(&env);
        let pauser = Address::generate(&env);
        let upgrader = Address::generate(&env);
        
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);
        
        let contract = MyStablecoinClient::new(&env, &env.register(MyStablecoin, ()));
        
        // Initialize contract
        contract.initialize(&admin, &pauser, &upgrader, &minter);
        
        // Test basic mint functionality
        contract.mint(&minter, &user1, &1000);
        assert_eq!(contract.balance(&user1), 1000);
        
        // Test contract metadata
        assert_eq!(contract.name(), String::from_str(&env, "Costa Rica Colon"));
        assert_eq!(contract.symbol(), String::from_str(&env, "CRCX"));
        assert_eq!(contract.decimals(), 2);
        assert_eq!(contract.total_supply(), 1000);
        assert_eq!(contract.is_paused(), false);
        
        // Test transfer functionality
        contract.transfer(&user1, &user2, &500);
        assert_eq!(contract.balance(&user1), 500);
        assert_eq!(contract.balance(&user2), 500);
        
        // Test burn functionality
        contract.burn(&user1, &100);
        assert_eq!(contract.balance(&user1), 400);
        assert_eq!(contract.total_supply(), 900);
        
        // Test role checks
        assert!(contract.has_role_minter(&minter));
        assert!(contract.has_role_pauser(&pauser));
        assert!(contract.has_role_upgrader(&upgrader));
        
        // Test that random user doesn't have roles
        let random_user = Address::generate(&env);
        assert!(!contract.has_role_minter(&random_user));
        assert!(!contract.has_role_pauser(&random_user));
        assert!(!contract.has_role_upgrader(&random_user));
    }

    #[test]
    fn test_batch_mint() {
        let env = Env::default();
        env.mock_all_auths();
        
        let admin = Address::generate(&env);
        let minter = Address::generate(&env);
        let pauser = Address::generate(&env);
        let upgrader = Address::generate(&env);
        
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);
        let user3 = Address::generate(&env);
        
        let contract = MyStablecoinClient::new(&env, &env.register(MyStablecoin, ()));
        
        // Initialize contract
        contract.initialize(&admin, &pauser, &upgrader, &minter);
        
        // Test batch mint
        let mut recipients = Vec::new(&env);
        recipients.push_back((user1.clone(), 100));
        recipients.push_back((user2.clone(), 200));
        recipients.push_back((user3.clone(), 300));
        
        // Execute batch mint
        contract.batch_mint(&minter, &recipients);
        
        // Verify balances
        assert_eq!(contract.balance(&user1), 100);
        assert_eq!(contract.balance(&user2), 200);
        assert_eq!(contract.balance(&user3), 300);
        assert_eq!(contract.total_supply(), 600);
    }

    #[test]
    fn test_burn_functionality() {
        let env = Env::default();
        env.mock_all_auths();
        
        let admin = Address::generate(&env);
        let minter = Address::generate(&env);
        let pauser = Address::generate(&env);
        let upgrader = Address::generate(&env);
        let owner = Address::generate(&env);
        
        let contract = MyStablecoinClient::new(&env, &env.register(MyStablecoin, ()));
        
        // Initialize contract
        contract.initialize(&admin, &pauser, &upgrader, &minter);
        
        // Mint tokens to owner
        contract.mint(&minter, &owner, &1000);
        assert_eq!(contract.balance(&owner), 1000);
        assert_eq!(contract.total_supply(), 1000);
        
        // Test burn functionality
        contract.burn(&owner, &300);
        assert_eq!(contract.balance(&owner), 700);
        assert_eq!(contract.total_supply(), 700);
        
        // Test burn all remaining tokens
        contract.burn(&owner, &700);
        assert_eq!(contract.balance(&owner), 0);
        assert_eq!(contract.total_supply(), 0);
    }

    #[test]
    fn test_pausable_functionality() {
        let env = Env::default();
        env.mock_all_auths();
        
        let admin = Address::generate(&env);
        let minter = Address::generate(&env);
        let pauser = Address::generate(&env);
        let upgrader = Address::generate(&env);
        let user = Address::generate(&env);
        
        let contract = MyStablecoinClient::new(&env, &env.register(MyStablecoin, ()));
        
        // Initialize contract
        contract.initialize(&admin, &pauser, &upgrader, &minter);
        
        // Test contract is not paused initially
        assert_eq!(contract.is_paused(), false);
        
        // Mint tokens when not paused
        contract.mint(&minter, &user, &1000);
        assert_eq!(contract.balance(&user), 1000);
        
        // Test pause functionality
        contract.pause(&pauser);
        assert_eq!(contract.is_paused(), true);
        
        // Test unpause functionality
        contract.unpause(&pauser);
        assert_eq!(contract.is_paused(), false);
        
        // Test operations work after unpause
        contract.mint(&minter, &user, &500);
        assert_eq!(contract.balance(&user), 1500);
    }

    #[test]
    fn test_allowance_and_transfer_from() {
        let env = Env::default();
        env.mock_all_auths();
        
        let admin = Address::generate(&env);
        let minter = Address::generate(&env);
        let pauser = Address::generate(&env);
        let upgrader = Address::generate(&env);
        
        let owner = Address::generate(&env);
        let spender = Address::generate(&env);
        let recipient = Address::generate(&env);
        
        let contract = MyStablecoinClient::new(&env, &env.register(MyStablecoin, ()));
        
        // Initialize contract
        contract.initialize(&admin, &pauser, &upgrader, &minter);
        
        // Mint tokens to owner
        contract.mint(&minter, &owner, &1000);
        assert_eq!(contract.balance(&owner), 1000);
        
        // Test approve
        contract.approve(&owner, &spender, &500, &1000);
        assert_eq!(contract.allowance(&owner, &spender), 500);
        
        // Test transfer_from
        contract.transfer_from(&spender, &owner, &recipient, &200);
        assert_eq!(contract.balance(&owner), 800);
        assert_eq!(contract.balance(&recipient), 200);
        assert_eq!(contract.allowance(&owner, &spender), 300);
    }

    #[test]
    fn test_comprehensive_validation() {
        let env = Env::default();
        env.mock_all_auths();
        
        let admin = Address::generate(&env);
        let minter = Address::generate(&env);
        let pauser = Address::generate(&env);
        let upgrader = Address::generate(&env);
        
        let contract = MyStablecoinClient::new(&env, &env.register(MyStablecoin, ()));
        
        // Initialize contract
        contract.initialize(&admin, &pauser, &upgrader, &minter);
        
        // Test validation works correctly
        let user = Address::generate(&env);
        let user2 = Address::generate(&env);
        
        // Mint tokens first
        contract.mint(&minter, &user, &1000);
        assert_eq!(contract.balance(&user), 1000);
        
        // Test transfer validation
        contract.transfer(&user, &user2, &500);
        assert_eq!(contract.balance(&user), 500);
        assert_eq!(contract.balance(&user2), 500);
        
        // Test burn validation
        contract.burn(&user, &200);
        assert_eq!(contract.balance(&user), 300);
        assert_eq!(contract.total_supply(), 800);
    }

    #[test]
    fn test_admin_functionality() {
        let env = Env::default();
        env.mock_all_auths();
        
        let admin = Address::generate(&env);
        let minter = Address::generate(&env);
        let pauser = Address::generate(&env);
        let upgrader = Address::generate(&env);
        
        let contract = MyStablecoinClient::new(&env, &env.register(MyStablecoin, ()));
        
        // Initialize contract
        contract.initialize(&admin, &pauser, &upgrader, &minter);
        
        // Test admin function
        assert!(contract.get_admin().is_some());
        
        // Test role assignments
        assert!(contract.has_role_minter(&minter));
        assert!(contract.has_role_pauser(&pauser));
        assert!(contract.has_role_upgrader(&upgrader));
        
        // Test that unauthorized users don't have roles
        let unauthorized_user = Address::generate(&env);
        assert!(!contract.has_role_minter(&unauthorized_user));
        assert!(!contract.has_role_pauser(&unauthorized_user));
        assert!(!contract.has_role_upgrader(&unauthorized_user));
    }

    #[test]
    fn test_address_validation() {
        let env = Env::default();
        env.mock_all_auths();
        
        let admin = Address::generate(&env);
        let minter = Address::generate(&env);
        let pauser = Address::generate(&env);
        let upgrader = Address::generate(&env);
        
        let contract = MyStablecoinClient::new(&env, &env.register(MyStablecoin, ()));
        
        // Initialize contract
        contract.initialize(&admin, &pauser, &upgrader, &minter);
        
        // Test that generated addresses are valid (they should pass validation)
        let valid_user = Address::generate(&env);
        
        // Test mint with valid address - should succeed
        contract.mint(&minter, &valid_user, &1000);
        assert_eq!(contract.balance(&valid_user), 1000);
        
        // Test transfer with valid addresses - should succeed
        let valid_recipient = Address::generate(&env);
        contract.transfer(&valid_user, &valid_recipient, &500);
        assert_eq!(contract.balance(&valid_user), 500);
        assert_eq!(contract.balance(&valid_recipient), 500);
        
        // Test that the validation is working by checking basic functionality
        // Note: In Soroban test environment, generated addresses are always valid
        let address_str = valid_user.to_string();
        
        // Verify the address string representation is not empty
        assert!(!address_str.is_empty());
    }

    #[test]
    fn test_error_conditions() {
        let env = Env::default();
        env.mock_all_auths();
        
        let admin = Address::generate(&env);
        let minter = Address::generate(&env);
        let pauser = Address::generate(&env);
        let upgrader = Address::generate(&env);
        
        let contract = MyStablecoinClient::new(&env, &env.register(MyStablecoin, ()));
        
        // Initialize contract
        contract.initialize(&admin, &pauser, &upgrader, &minter);
        
        let user = Address::generate(&env);
        
        // Test error conditions that should fail
        
        // Test mint with zero amount - should fail
        let result = contract.try_mint(&minter, &user, &0);
        assert!(result.is_err());
        
        // Test mint with negative amount - should fail
        let result = contract.try_mint(&minter, &user, &-100);
        assert!(result.is_err());
        
        // Test transfer with zero amount - should fail
        contract.mint(&minter, &user, &1000);
        let user2 = Address::generate(&env);
        let result = contract.try_transfer(&user, &user2, &0);
        assert!(result.is_err());
        
        // Test transfer with negative amount - should fail
        let result = contract.try_transfer(&user, &user2, &-100);
        assert!(result.is_err());
        
        // Test transfer with insufficient balance - should fail
        let result = contract.try_transfer(&user, &user2, &2000);
        assert!(result.is_err());
        
        // Test burn with zero amount - should fail
        let result = contract.try_burn(&user, &0);
        assert!(result.is_err());
        
        // Test burn with negative amount - should fail
        let result = contract.try_burn(&user, &-100);
        assert!(result.is_err());
        
        // Test burn with insufficient balance - should fail
        let result = contract.try_burn(&user, &2000);
        assert!(result.is_err());
    }

    #[test]
    fn test_self_transfer_prevention() {
        let env = Env::default();
        env.mock_all_auths();
        
        let admin = Address::generate(&env);
        let minter = Address::generate(&env);
        let pauser = Address::generate(&env);
        let upgrader = Address::generate(&env);
        
        let contract = MyStablecoinClient::new(&env, &env.register(MyStablecoin, ()));
        
        // Initialize contract
        contract.initialize(&admin, &pauser, &upgrader, &minter);
        
        let user = Address::generate(&env);
        
        // Mint some tokens
        contract.mint(&minter, &user, &1000);
        assert_eq!(contract.balance(&user), 1000);
        
        // Test self-transfer - should fail
        let result = contract.try_transfer(&user, &user, &100);
        assert!(result.is_err());
        
        // Balance should remain unchanged
        assert_eq!(contract.balance(&user), 1000);
    }

    #[test]
    fn test_integer_only_operations() {
        let env = Env::default();
        env.mock_all_auths();
        
        let admin = Address::generate(&env);
        let minter = Address::generate(&env);
        let pauser = Address::generate(&env);
        let upgrader = Address::generate(&env);
        
        let contract = MyStablecoinClient::new(&env, &env.register(MyStablecoin, ()));
        
        // Initialize contract
        contract.initialize(&admin, &pauser, &upgrader, &minter);
        
        let user = Address::generate(&env);
        
        // Test that integer operations work correctly
        contract.mint(&minter, &user, &1000);
        assert_eq!(contract.balance(&user), 1000);
        
        // Test that minimum amount of 1 works
        contract.mint(&minter, &user, &1);
        assert_eq!(contract.balance(&user), 1001);
        
        // Test transfer with integer amounts
        let user2 = Address::generate(&env);
        contract.transfer(&user, &user2, &500);
        assert_eq!(contract.balance(&user), 501);
        assert_eq!(contract.balance(&user2), 500);
        
        // Test burn with integer amounts
        contract.burn(&user, &1);
        assert_eq!(contract.balance(&user), 500);
        assert_eq!(contract.total_supply(), 1000);
        
        // Verify decimals is 2
        assert_eq!(contract.decimals(), 2);
    }
}


