use std::collections::HashMap;
use crate::errors::{DatabaseErrors, TransactionErrors};
use std::sync::Arc;  
use tokio::sync::RwLock; 

#[derive(Clone)]
struct Data {
    item: String,
}

impl Data {
    fn new(item: String) -> Data {
        Data {
            item
        }
    }
}

enum DatabaseOperations {
    Insert(u32, Data), 
    Get(u32),
    Update(u32, Data),
    Delete(u32)
}

pub struct Database {
    primary_key: RwLock<u32>,
    storage: RwLock<HashMap<u32, Data>>
}

impl Database {
    pub fn new() -> Arc<Database> {
       Arc::new( Database { primary_key: RwLock::new(1), storage: RwLock::new(HashMap::new())})
    }

    async fn insert_data(&self, data: String) -> Result<(), DatabaseErrors> {
        let mut key = self.primary_key.write().await;  
    
        self.storage.write().await.insert(*key, Data::new(data));
        *key = key.checked_add(1).unwrap();  
       
        Ok(())
    }

    async fn get_data(&self, key: u32) -> Result<Data, DatabaseErrors> {
        if key >= *self.primary_key.read().await {
            return Err(DatabaseErrors::InvalidKeyError);
        }
        let result = self.storage.read().await.get(&key).cloned().ok_or(DatabaseErrors::InvalidKeyError)?;
        Ok(result)
    }

    async fn update_data(&self, key: u32, update: String ) -> Result<(), DatabaseErrors> {
        if key >= *self.primary_key.read().await {
            return Err(DatabaseErrors::InvalidKeyError)
        }
        self.storage.write().await.get_mut(&key).ok_or(DatabaseErrors::KeyNotFound)?
        .item = update;
        
        Ok(())
    }

    async fn delete_data(&self, key: u32) -> Result<(), DatabaseErrors> {
        if key >= *self.primary_key.read().await {
            return Err(DatabaseErrors::InvalidKeyError);
        }
        self.storage.write().await.remove(&key);
        Ok(())
    }

    async fn begin_transaction(&self) -> Transaction {
        Transaction { operations: Vec::new(), tx_state: TransactionState::New, database_state: Arc::clone(self) }
    }
}

enum TransactionState {
    New,
    RolledBack,
    Commited,
}

struct Transaction {
    operations: Vec<DatabaseOperations>,
    tx_state: TransactionState,
    database_state: Arc<Database>
}

impl Transaction {

    fn add_operation(&mut self, operation: DatabaseOperations) -> Result<(), TransactionErrors> {
        todo!();
        //push crud operartions on the vector which will be executed on the database
    }

    fn commit(&mut self,) -> Result<(), TransactionErrors> {
        todo!();
        //match statment which executes the operation from the transaction vector on the database_state 
    }

    fn roll_back(&mut self,) -> Result<(), DatabaseErrors> {
        todo!();
        //clear / delete the current transaction instnce if on one and is empty 
    }
}