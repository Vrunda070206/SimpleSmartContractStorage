#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String, Symbol, symbol_short};

// Storage key for mapping user addresses to their data
#[contracttype]
pub enum DataKey {
    UserData(Symbol),
}

// Structure to store user data
#[contracttype]
#[derive(Clone)]
pub struct StoredData {
    pub owner: Symbol,
    pub data: String,
    pub timestamp: u64,
}

// Counter key for total stored records
const TOTAL_RECORDS: Symbol = symbol_short!("TOTAL");

#[contract]
pub struct SimpleStorageContract;

#[contractimpl]
impl SimpleStorageContract {
    
    /// Store data on the blockchain for a specific user
    /// Parameters:
    /// - owner: unique identifier for the user
    /// - data: the actual data to be stored
    pub fn store_data(env: Env, owner: Symbol, data: String) {
        let timestamp = env.ledger().timestamp();
        
        let stored_data = StoredData {
            owner: owner.clone(),
            data,
            timestamp,
        };
        
        // Store the data with user's identifier as key
        env.storage()
            .instance()
            .set(&DataKey::UserData(owner), &stored_data);
        
        // Update total records counter
        let mut total: u64 = env.storage().instance().get(&TOTAL_RECORDS).unwrap_or(0);
        total += 1;
        env.storage().instance().set(&TOTAL_RECORDS, &total);
        
        // Extend storage lifetime
        env.storage().instance().extend_ttl(5000, 5000);
    }
    
    /// Retrieve stored data for a specific user
    /// Parameters:
    /// - owner: unique identifier for the user
    /// Returns: StoredData structure containing owner, data, and timestamp
    pub fn retrieve_data(env: Env, owner: Symbol) -> StoredData {
        let key = DataKey::UserData(owner.clone());
        
        env.storage().instance().get(&key).unwrap_or(StoredData {
            owner: symbol_short!("NONE"),
            data: String::from_str(&env, "No data found"),
            timestamp: 0,
        })
    }
    
    /// Update existing data for a specific user
    /// Parameters:
    /// - owner: unique identifier for the user
    /// - new_data: the updated data to replace existing data
    pub fn update_data(env: Env, owner: Symbol, new_data: String) {
        let key = DataKey::UserData(owner.clone());
        
        // Check if data exists for this user
        let existing_data: Option<StoredData> = env.storage().instance().get(&key);
        
        if existing_data.is_some() {
            let timestamp = env.ledger().timestamp();
            
            let updated_data = StoredData {
                owner: owner.clone(),
                data: new_data,
                timestamp,
            };
            
            env.storage().instance().set(&key, &updated_data);
            env.storage().instance().extend_ttl(5000, 5000);
        } else {
            panic!("No existing data found for this user");
        }
    }
    
    /// Get the total number of stored records
    /// Returns: Total count of stored records
    pub fn get_total_records(env: Env) -> u64 {
        env.storage().instance().get(&TOTAL_RECORDS).unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{symbol_short, Env, String};

    #[test]
    fn test_store_and_retrieve() {
        let env = Env::default();
        let contract_id = env.register_contract(None, SimpleStorageContract);
        let client = SimpleStorageContractClient::new(&env, &contract_id);

        let owner = symbol_short!("user1");
        let data = String::from_str(&env, "Hello Blockchain");

        // Store data
        client.store_data(&owner, &data);

        // Retrieve data
        let stored = client.retrieve_data(&owner);
        assert_eq!(stored.data, data);
    }
}