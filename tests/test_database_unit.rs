use database::types::*;

mod tests {
    use super::*;

    #[tokio::test]
    async fn test_insert_get_data(){
        let database = Database::new();
        database.insert_data(String::from("hoo")).await.unwrap();
        let result = database.get_data(1).await.unwrap();
        assert_eq!(result.item, "hoo".to_string())
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
    async fn test_delete_data(){
        let database = Database::new();
        database.insert_data(String::from("value")).await.unwrap();
        database.delete_data(1).await.unwrap();
        let err = database.get_data(1).await;
        assert!(err.is_err());
    }

    #[tokio::test]
    async fn test_transaction_insert(){
        let database = Database::new();
        let tx1 = Transaction::new(&database).await;
        tx1.add_operation(DatabaseOperations::Insert(String::from("hi"))).await.unwrap();
        tx1.add_operation(DatabaseOperations::Insert(String::from("hello"))).await.unwrap();
        tx1.add_operation(DatabaseOperations::Insert(String::from("lol"))).await.unwrap();
     
    }

    #[tokio::test]
    async fn test_transaction_rollback(){
        let database = Database::new();
        let tx1 = Transaction::new(&database).await;
        tx1.add_operation(DatabaseOperations::Insert(String::from("loool"))).await.unwrap();
        tx1.add_operation(DatabaseOperations::Insert(String::from("hello"))).await.unwrap();
        tx1.add_operation(DatabaseOperations::Insert(String::from("lol"))).await.unwrap();
        tx1.roll_back().await.unwrap();
        assert_eq!(tx1.tx_state, TransactionState::RolledBack);
        assert!(tx1.operations.is_empty());
    }

    #[tokio::test]
    async fn test_transaction_commit(){
        
    }
}