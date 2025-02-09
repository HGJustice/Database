use std::collections::HashMap;
use crate::errors::{DatabaseErrors, TransactionErrors};


struct Data<T> {
    item: T,
}k 

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
    primary_key: u32,
    storage: HashMap<u32, Data<T>>
}

impl<T> Database<T> {

    fn new() -> Database<T> {
        Database { primary_key: 1, storage: HashMap::new()}
    }

    fn insert_data() -> Result<(), DatabaseErrors> {
        todo!();
    }

    fn get_data() -> Result<(), DatabaseErrors> {
        todo!();
    }

    fn update_data() -> Result<(), DatabaseErrors> {
        todo!();
    }

    fn delete_data() -> Result<(), DatabaseErrors>{
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