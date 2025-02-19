use database::errors::*;
use database::types::*;

mod tests {
    use super::*;

    #[tokio::test]
    async fn test_intergration() {
        let database = Database::new();
   
        // Setup test data
        database.insert_data(String::from("test_data")).await.unwrap();
        
        // Spawn multiple read tasks
        let mut handles = vec![];
        for _ in 0..5 {
            let db = database.clone();
            handles.push(tokio::spawn(async move {
                for _ in 0..3 {
                    let result = db.get_data(1).await.unwrap();
                    assert_eq!(result.item, "test_data");
                }
            }));
        }
     
        // Wait for all tasks to complete
        for handle in handles {
            handle.await.unwrap();
        }
    }
}
