use std::collections::{ HashMap, VecDeque};
use crate::errors::{DatabaseErrors, TransactionErrors};
use std::sync::Arc;  
use tokio::sync::RwLock; 

#[derive(Clone, Debug)]
pub struct Data {
    item: String,
}

impl Data {
    fn new(item: String) -> Data {
        Data {
            item
        }
    }
}

pub enum DatabaseOperations {
    Insert(String), 
    Update(u32, String),
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

    pub async fn get_data(&self, key: u32) -> Result<Data, DatabaseErrors> {
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

#[derive(Clone, Copy, PartialEq, )]
enum TransactionState {
    New,
    RolledBack,
    Commited,
}

pub struct Transaction {
    operations: RwLock<VecDeque<DatabaseOperations>>,
    tx_state: RwLock<TransactionState>,
    database_state: Arc<Database>
}

impl Transaction {

    pub async fn new(database: &Arc<Database>) -> Transaction {
        Transaction { operations: RwLock::new(VecDeque::new()), tx_state: RwLock::new(TransactionState::New), database_state: Arc::clone(database) }
    }

    pub async fn add_operation(&self, operation: DatabaseOperations) -> Result<(), TransactionErrors> {
        //push crud operartions on the vector which will be executed on the database
        let tx_state = self.tx_state.read().await;
        if *tx_state != TransactionState::New {
            return Err(TransactionErrors::NotNewTransaction)
        }
        self.operations.write().await.push_back(operation);
        Ok(())
    }

    pub async fn commit_changes(&self,) -> Result<(), TransactionErrors> {
         let tx_state = self.tx_state.read().await;
        if *tx_state != TransactionState::New {
            return Err(TransactionErrors::NotNewTransaction);
        }
        let operartions = &mut *self.operations.write().await;
        while !operartions.is_empty() {
            let current_operation = &operartions[0];
            match current_operation {
                DatabaseOperations::Insert(data,) => {
                    let result = self.database_state.insert_data(data.to_string()).await;
                    if result.is_err() {
                        self.roll_back().await?;
                        return Err(TransactionErrors::ErrorInInsertingData);
                    }
                   
                }
                DatabaseOperations::Update(key,data) => {
                    let result = self.database_state.update_data(*key, data.to_string()).await;
                    if result.is_err() {
                        self.roll_back().await?;
                        return Err(TransactionErrors::ErrorUpdatingTheDatabase)
                    }
                }
                DatabaseOperations::Delete(key) => {
                    let result = self.database_state.delete_data(*key).await;
                    if result.is_err() {
                        self.roll_back().await?;
                        return Err(TransactionErrors::ErrorInDeletingData)
                    }
                }
            }
            operartions.pop_front();
        }
        drop(tx_state);
        let mut tx_state = self.tx_state.write().await;
        *tx_state = TransactionState::Commited;
        Ok(())
    }

    pub async fn roll_back(&self,) -> Result<(), TransactionErrors> {
        self.operations.write().await.clear();
        let mut tx_state = self.tx_state.write().await;
        *tx_state = TransactionState::RolledBack; 
        Ok(())
    }
}