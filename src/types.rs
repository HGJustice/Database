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

    pub async fn insert_data(&self, data: String) -> Result<(), DatabaseErrors> {
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

}

#[derive(Clone, Copy)]
enum TransactionState {
    New,
    RolledBack,
    Commited,
}

struct Transaction {
    operations: RwLock<Vec<DatabaseOperations>>,
    tx_state: RwLock<TransactionState>,
    database_state: Arc<Database>
}

impl Transaction {

    async fn begin_transaction(&self) -> Transaction {
        Transaction { operations: RwLock::new(Vec::new()), tx_state: RwLock::new(TransactionState::New), database_state: Arc::clone(&self.database_state) }
    }

    async fn add_operation(&self, operation: DatabaseOperations) -> Result<(), TransactionErrors> {
        //push crud operartions on the vector which will be executed on the database
        self.operations.write().await.push(operation);
        Ok(())
    }

    async fn commit_changes(&self,) -> Result<(), TransactionErrors> {
         //match statment which executes the operation from the transaction vector on the database_state 
         let operartions = &mut *self.operations.write().await;
        for op in operartions  {
            match op {
                DatabaseOperations::Insert(_,_ ) => {
                    Database::insert_data(&self.database_state, String::from("test"));
                    operartions.pop();
                }
                DatabaseOperations::Update(_,_ ) => {
                    Database::update_data(&self.database_state, 1, String::from("test"));
                    operartions.pop();
                }
                DatabaseOperations::Delete(_) => {
                    Database::delete_data(&self.database_state, 1);
                    operartions.pop();
                }
                _ => return Err(TransactionErrors::InvalidOperation)

                
            }
        }
        Ok(())
    }

    async fn roll_back(&mut self,) -> Result<(), DatabaseErrors> {
        //clear / delete the current transaction instnce if on one and is empty 
        self.operations.write().await.clear();
        let mut tx_state = self.tx_state.write().await;
        *tx_state = TransactionState::RolledBack; 
        Ok(())
       
    }
}