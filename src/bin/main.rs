use mongo_repo::{
    common::Entity,
    items::{ItemSpec, MongoItemsRepo},
    storage::{MongoRepoError, Repo},
};
use std::ops::DerefMut;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    if let Err(e) = test_items_repo().await {
        println!("{:?}", e);
    }
}

async fn test_items_repo() -> Result<(), MongoRepoError> {
    // Parse a connection string into an options struct.
    let mut client_options =
        mongodb::options::ClientOptions::parse("mongodb://127.0.0.1:27017").await?;
    println!("client options parsed");

    // Manually set an option.
    client_options.app_name = Some("Mongo Repo Test".to_string());

    // Get a handle to the deployment.
    let client = mongodb::Client::with_options(client_options)?;

    let mut session = client.start_session(None).await.unwrap();
    println!("session started");
    session.start_transaction(None).await?;
    println!("transaction started");
    let session = Arc::new(tokio::sync::Mutex::new(session));

    let items_repo =
        MongoItemsRepo::new_with_session("test", "items", client.clone(), Arc::clone(&session));

    let item1_spec = ItemSpec {
        name: "An Item".into(),
    };

    let item2_spec = ItemSpec {
        name: "Another Item".into(),
    };

    let id1 = items_repo.create(&item1_spec).await?;
    let id2 = items_repo.create(&item2_spec).await?;

    println!("created 2 docs: {} and {}", id1, id2);

    let opt_doc1 = items_repo.retrieve(&id1).await?;

    println!("queried doc 1: {:?}", opt_doc1);

    let mut doc1 = opt_doc1.unwrap();

    *doc1.name_mut() = "An Updated Item".into();

    let update_success = items_repo.update(&doc1).await?;

    println!("updated doc 1 successfully? {}", update_success);

    let all_docs = items_repo.retrieve_all().await?;

    println!("all docs: {:?}", all_docs);

    let delete_success = items_repo.delete(doc1.id()).await?;

    println!("delete doc 1 successfully? {}", delete_success);

    let update_success = items_repo.update(&doc1).await?;

    println!("updated doc 1 successfully? {}", update_success);

    let opt_doc1 = items_repo.retrieve(&id1).await?;

    println!("queried doc 1: {:?}", opt_doc1);

    let all_docs = items_repo.retrieve_all().await?;

    println!("all docs: {:?}", all_docs);

    for doc in all_docs {
        println!(
            "delete {}: {}",
            doc.id(),
            items_repo.delete(doc.id()).await?
        );
    }

    let mut session_guard = session.lock().await;
    println!("session locked");
    let session = session_guard.deref_mut();
    session.commit_transaction().await?;
    println!("transaction committed");

    Ok(())
}
