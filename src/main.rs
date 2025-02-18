use database::types::*;

#[tokio::main]
async fn main() {
    let database = Database::new();
    println!("Database created");
    let tx1 = Transaction::new(&database).await;
    println!("Transaction created");

    tx1.add_operation(DatabaseOperations::Insert(String::from("1"))).await.unwrap();
    tx1.add_operation(DatabaseOperations::Insert(String::from("2"))).await.unwrap();
    tx1.add_operation(DatabaseOperations::Insert(String::from("3"))).await.unwrap();
    tx1.add_operation(DatabaseOperations::Update(1, String::from("updated"))).await.unwrap();
    tx1.add_operation(DatabaseOperations::Delete(2)).await.unwrap();
    println!("Transactions added");

  
    tx1.commit_changes().await.unwrap();
    println!("commited");
    let result = database.get_data(1).await.unwrap();
    println!("Key 1 holds: :{:?}", result);
}