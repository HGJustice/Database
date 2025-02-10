use std::collections::HashMap;
use crate::errors::{DatabaseErrors, TransactionErrors};
use std::sync::Arc;  
use tokio::sync::Mutex; 

#[derive(Clone)]
struct Data<T> {
    item: T,
}

impl<T> Data<T> {
    fn new(item: T) -> Data<T> {
        Data {
            item
        }
    }
}

enum DatabaseOperations<T> {
    Insert(u32, Data<T>), 
    Get(u32),
    Update(u32, Data<T>),
    Delete(u32)
}

struct Database<T> {
    primary_key: Mutex<u32>,
    storage: Mutex<HashMap<u32, Data<T>>>
}

impl<T: Clone> Database<T> {

    fn new() -> Arc<Database<T>> {
       Arc::new( Database { primary_key: Mutex::new(1), storage: Mutex::new(HashMap::new())})
    }

    async fn insert_data(&self, data: T) -> Result<(), DatabaseErrors> {
        let mut key = *self.primary_key.lock().await;

        self.storage.lock().await.insert(key, Data::new(data));
        key += 1;
        Ok(())
    }

    async fn get_data(&self, key: u32) -> Result<Data<T>, DatabaseErrors> {
        if key >= *self.primary_key.lock().await {
            return Err(DatabaseErrors::InvalidKeyError);
        }
        let result = self.storage.lock().await.get(&key).cloned().ok_or(DatabaseErrors::InvalidKeyError)?;
        Ok(result)
    }

    async fn update_data(self, key: u32, update: Data<T> ) -> Result<(), DatabaseErrors> {
        if key >= *self.primary_key.lock().await {
            return Err(DatabaseErrors::InvalidKeyError)
        }
        let result = self.storage.lock().await.get_mut(&key).unwrap();
        *result = update;
        Ok(())
    }

    async fn delete_data(self, key: u32) -> Result<(), DatabaseErrors> {
        if key >= *self.primary_key.lock().await {
            return Err(DatabaseErrors::InvalidKeyError);
        }
        Ok(())

    }
}

impl Database<String> {
    async fn get_string_data(self, key: u32) -> Result<Data<String>, DatabaseErrors> {
 
        todo!();
    }
}


enum TransactionState {
    Created,
    RolledBack,
    Commited,
}

struct Transaction<T> {
    operations: Vec<DatabaseOperations<T>>,
    state: TransactionState,
}

impl<T> Transaction<T> {
    fn new() -> Transaction<T> {
        Transaction { operations: Vec::new(), state: TransactionState::Created }
    }

    fn commit_changes() -> Result<(), TransactionErrors> {
        todo!();
    }

    fn rollback_changes() -> Result<(), TransactionErrors> {
        todo!();
    }

    fn migrate_changes() -> Result<(), TransactionErrors> {
        todo!();
    }
}

enum Actions {
    Read,
    Wrtie    //????
}