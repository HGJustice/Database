use database::types::*;
use database::errors::*;

    #[tokio::test]
    async fn test_insert_get_data(){
        let database = Database::new();
        database.insert_data(String::from("hoo")).await.unwrap();
        let result = database.get_data(1).await.unwrap();
        assert_eq!(result.item, "hoo".to_string())
    }

    #[tokio::test]
    async fn test_get_invalid_key_revert(){
        let database = Database::new();
        assert!(matches!(database.get_data(1).await, Err(DatabaseErrors::InvalidKeyError)));
    }

    #[tokio::test]
    async fn test_update_date(){
        let database = Database::new();
        database.insert_data(String::from("hello")).await.unwrap();
        database.update_data(1, "hi".to_string()).await.unwrap();
        let result = database.get_data(1).await.unwrap();
        assert_eq!(result.item, "hi".to_string());
    }

    #[tokio::test]
    async fn test_update_invalid_key_revert(){
        let database = Database::new();
        assert!(matches!(database.update_data(1,String::from("hey")).await, Err(DatabaseErrors::InvalidKeyError)));
    }

    #[tokio::test]
    async fn test_delete_data(){
        let database = Database::new();
        database.insert_data(String::from("value")).await.unwrap();
        database.delete_data(1).await.unwrap();
        let err = database.get_data(1).await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn test_delete_invalid_key_revert(){
        let database = Database::new();
        assert!(matches!(database.delete_data(1).await, Err(DatabaseErrors::InvalidKeyError)));
    }

    #[tokio::test]
    async fn test_transaction_insert(){
        let database = Database::new();
        let tx1 = Transaction::new(&database).await;
        tx1.add_operation(DatabaseOperations::Insert(String::from("hi"))).await.unwrap();
        tx1.add_operation(DatabaseOperations::Insert(String::from("hello"))).await.unwrap();
        tx1.add_operation(DatabaseOperations::Insert(String::from("lol"))).await.unwrap();
        tx1.commit_changes().await.unwrap();
        let result = database.get_data(1).await.unwrap();
        assert_eq!(result.item, "hi".to_string());
    }

    #[tokio::test]
    async fn test_add_invalid_tx_state_revert(){
        let database = Database::new();
        let tx1 = Transaction::new(&database).await;
        tx1.roll_back().await.unwrap();
        let tx1_state = tx1.tx_state.read().await;
        assert_eq!(*tx1_state, TransactionState::RolledBack);
        assert!(matches!(tx1.add_operation(DatabaseOperations::Insert("error?".to_string())).await, Err(TransactionErrors::NotNewTransaction)))
    }

    #[tokio::test]
    async fn test_transaction_rollback(){
        let database = Database::new();
        let tx1 = Transaction::new(&database).await;
        tx1.add_operation(DatabaseOperations::Insert(String::from("loool"))).await.unwrap();
        tx1.add_operation(DatabaseOperations::Insert(String::from("hello"))).await.unwrap();
        tx1.add_operation(DatabaseOperations::Insert(String::from("lol"))).await.unwrap();
        tx1.roll_back().await.unwrap();
        let current_state = tx1.tx_state.read().await;
        let current_queue = tx1.operations.read().await;
        assert_eq!(*current_state, TransactionState::RolledBack);
        assert!(current_queue.is_empty());
    }

    #[tokio::test]
    async fn test_failled_commit_state_rollback() {
        let database = Database::new();
        let tx1 = Transaction::new(&database).await;
        tx1.add_operation(DatabaseOperations::Insert("hi".to_string())).await.unwrap();
        tx1.add_operation(DatabaseOperations::Insert("lool".to_string())).await.unwrap();
        tx1.add_operation(DatabaseOperations::Update(2, String::from("updated"))).await.unwrap();
        tx1.add_operation(DatabaseOperations::Update(5, String::from("error?"))).await.unwrap();
        assert!(tx1.commit_changes().await.is_err());
        let state = tx1.tx_state.read().await;
        let ops = tx1.operations.read().await;
        assert_eq!(*state, TransactionState::RolledBack);
        assert!(ops.is_empty());
    }

    #[tokio::test]
    async fn test_transaction_commit(){
        let database = Database::new();
        let tx1 = Transaction::new(&database).await;
        tx1.add_operation(DatabaseOperations::Insert("hi".to_string())).await.unwrap();
        tx1.add_operation(DatabaseOperations::Insert("lool".to_string())).await.unwrap();
        tx1.add_operation(DatabaseOperations::Insert("kek".to_string())).await.unwrap();
        tx1.commit_changes().await.unwrap();
        let result = database.get_data(3).await.unwrap();
        assert_eq!(result.item, "kek".to_string());
        let tx2 = Transaction::new(&database).await;
        tx2.add_operation(DatabaseOperations::Update(2, String::from("updated"))).await.unwrap();
        tx2.add_operation(DatabaseOperations::Delete(1)).await.unwrap();
        tx2.commit_changes().await.unwrap();
        let result = database.get_data(2).await.unwrap();
        let empty = database.get_data(1).await;
        assert_eq!(result.item, "updated".to_string());
        assert!(empty.is_err());
    }

    #[tokio::test]
    async fn test_commit_invalid_tx_state_revert(){
        let database = Database::new();
        let tx1 = Transaction::new(&database).await;
        tx1.add_operation(DatabaseOperations::Insert("hi".to_string())).await.unwrap();
        tx1.roll_back().await.unwrap();
        assert!(matches!(tx1.commit_changes().await, Err(TransactionErrors::NotNewTransaction)));   
    }

    #[tokio::test]
    async fn test_commit_rolledback_tx_revert(){
        let database = Database::new();
        let tx1 = Transaction::new(&database).await;
        tx1.roll_back().await.unwrap();
        let state = tx1.tx_state.read().await;
        assert_eq!(*state, TransactionState::RolledBack);
        assert!(matches!(tx1.commit_changes().await, Err(TransactionErrors::NotNewTransaction) ))

    }

    #[tokio::test]
    async fn test_already_commited_tx_revert(){
        let database = Database::new();
        let tx1 = Transaction::new(&database).await;
        tx1.add_operation(DatabaseOperations::Insert(String::from("hi"))).await.unwrap();
        tx1.commit_changes().await.unwrap();
        let state = tx1.tx_state.read().await;
        assert_eq!(*state, TransactionState::Commited);
        assert!(matches!(tx1.commit_changes().await, Err(TransactionErrors::NotNewTransaction)));
    }
