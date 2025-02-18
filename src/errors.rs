#[derive(Debug)]
pub enum DatabaseErrors {
    InvalidKeyError,
    KeyNotFound,
    PrimaryKeyIncreaseFailed,
}

#[derive(Debug)]
pub enum TransactionErrors {
    NotNewTransaction,
    InvalidOperation,
    ErrorInInsertingData,
    ErrorUpdatingTheDatabase,
    ErrorInDeletingData,
}
