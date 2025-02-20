use std::sync::Arc;

use database::errors::*;
use database::types::*;

    #[tokio::test]
    async fn test_intergration_concurrency(){
        let database = Database::new();
        let mut handles = vec![];

        let tx1 = Transaction::new(&database).await;
        tx1.add_operation(DatabaseOperations::Insert("1".to_string())).await.unwrap();
        tx1.add_operation(DatabaseOperations::Insert("2".to_string())).await.unwrap();
        tx1.add_operation(DatabaseOperations::Insert("3".to_string())).await.unwrap();
        tx1.add_operation(DatabaseOperations::Insert("4".to_string())).await.unwrap();
        tx1.add_operation(DatabaseOperations::Insert("5".to_string())).await.unwrap();
        tx1.add_operation(DatabaseOperations::Insert("6".to_string())).await.unwrap();
        tx1.add_operation(DatabaseOperations::Insert("7".to_string())).await.unwrap();
        tx1.add_operation(DatabaseOperations::Insert("8".to_string())).await.unwrap();
        tx1.add_operation(DatabaseOperations::Insert("9".to_string())).await.unwrap();
        tx1.add_operation(DatabaseOperations::Insert("19".to_string())).await.unwrap();
        tx1.add_operation(DatabaseOperations::Insert("11".to_string())).await.unwrap();
        tx1.add_operation(DatabaseOperations::Insert("12".to_string())).await.unwrap();
        tx1.commit_changes().await.unwrap();
        let result = database.get_data(1).await.unwrap();
        assert_eq!(result.item, "1".to_string());
        assert!(matches!(tx1.commit_changes().await, Err(TransactionErrors::NotNewTransaction)));

        let db2 = Arc::clone(&database);
        handles.push(tokio::spawn(async move {
            let tx2 = Transaction::new(&db2).await;
            tx2.add_operation(DatabaseOperations::Insert("13".to_string())).await.unwrap();
            tx2.add_operation(DatabaseOperations::Update(2, "updated".to_string())).await.unwrap();
            tx2.add_operation(DatabaseOperations::Delete(4)).await.unwrap();
            tx2.commit_changes().await.unwrap();
            let updated = db2.get_data(2).await.unwrap();
            let empty = db2.get_data(4).await;
            assert!(empty.is_err());
            assert_eq!(updated.item, "updated".to_string());
        }));
        
        let db3 = Arc::clone(&database);
        handles.push(tokio::spawn(async move {
            let tx3 = Transaction::new(&db3).await;
            tx3.add_operation(DatabaseOperations::Insert("rollback".to_string())).await.unwrap();
            tx3.add_operation(DatabaseOperations::Insert("rollback".to_string())).await.unwrap();
            tx3.roll_back().await.unwrap();
            let empty = db3.get_data(24).await;
            let current_tx3_state = tx3.tx_state.read().await;
            assert_eq!(*current_tx3_state, TransactionState::RolledBack);
            drop(current_tx3_state);
            assert!(empty.is_err()); 
            assert!(matches!(tx3.commit_changes().await, Err(TransactionErrors::NotNewTransaction)));
        }));

        let db4 = Arc::clone(&database);
        handles.push(tokio::spawn(async move {
            let tx4 = Transaction::new(&db4).await;
            tx4.add_operation(DatabaseOperations::Insert("commit rollback".to_string())).await.unwrap();
            tx4.add_operation(DatabaseOperations::Insert("commit rollback".to_string())).await.unwrap();
            tx4.add_operation(DatabaseOperations::Delete(22)).await.unwrap(); //failed
            let err = tx4.commit_changes().await;
            let current_tx4_state = tx4.tx_state.read().await;
            assert_eq!(*current_tx4_state, TransactionState::RolledBack);
            drop(current_tx4_state);
            assert!(err.is_err()); 
        }));

        let db5 = Arc::clone(&database);
        handles.push(tokio::spawn(async move {
            let result1 = db5.get_data(1).await.unwrap();
            let result2 = db5.get_data(3).await.unwrap();
            let result3 = db5.get_data(5).await.unwrap();
            let result4  = db5.get_data(2).await.unwrap();
            let result6  = db5.get_data(7).await.unwrap();
            let result7  = db5.get_data(8).await.unwrap();
            assert_eq!(result1.item, "1".to_string());
            assert_eq!(result2.item, "3".to_string());
            assert_eq!(result3.item, "5".to_string());
            assert_eq!(result4.item, "updated".to_string());
            assert_eq!(result6.item, "7".to_string());
            assert_eq!(result7.item, "8".to_string());
        }));

        let db6 = Arc::clone(&database);
        handles.push(tokio::spawn(async move {
            let tx6 = Transaction::new(&db6).await;
            tx6.add_operation(DatabaseOperations::Insert("14".to_string())).await.unwrap();
            tx6.add_operation(DatabaseOperations::Insert("15".to_string())).await.unwrap();
            tx6.add_operation(DatabaseOperations::Insert("16".to_string())).await.unwrap();
            tx6.commit_changes().await.unwrap();
            let tx7 = Transaction::new(&db6).await;
            tx7.add_operation(DatabaseOperations::Update(16, "updated".to_string())).await.unwrap();
            tx7.add_operation(DatabaseOperations::Update(15, "updated".to_string())).await.unwrap();
            tx7.add_operation(DatabaseOperations::Update(6, "updated".to_string())).await.unwrap();
            tx7.add_operation(DatabaseOperations::Update(14, "updated".to_string())).await.unwrap();
            tx7.add_operation(DatabaseOperations::Update(9, "updated".to_string())).await.unwrap();
            tx7.commit_changes().await.unwrap();
            let result1 = db6.get_data(16).await.unwrap();
            let result2 = db6.get_data(16).await.unwrap();
            let result3 = db6.get_data(16).await.unwrap();
            let result4 = db6.get_data(16).await.unwrap();
            let result5 = db6.get_data(16).await.unwrap();
            assert_eq!(result1.item, "updated".to_string());
            assert_eq!(result2.item, "updated".to_string());
            assert_eq!(result3.item, "updated".to_string());
            assert_eq!(result4.item, "updated".to_string());
            assert_eq!(result5.item, "updated".to_string());
        }));
    
        for handle in handles {
            handle.await.unwrap();
        }

        let final_result1 = database.get_data(16).await.unwrap();
        let final_result2 = database.get_data(2).await.unwrap();
        let final_result3 = database.get_data(8).await.unwrap();
        let final_result4 = database.get_data(13).await.unwrap();
        let final_result5 = database.get_data(4).await;
        assert_eq!(final_result1.item, "updated".to_string());
        assert_eq!(final_result2.item, "updated".to_string());
        assert_eq!(final_result3.item, "8".to_string());
        assert_eq!(final_result4.item, "13".to_string());
        assert!(final_result5.is_err());

    }