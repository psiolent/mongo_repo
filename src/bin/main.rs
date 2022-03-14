use mongo_test::{
    common::Id,
    items::{Item, MongoItemsRepo},
    storage::Repository,
};

#[tokio::main]
async fn main() {
    let id_orig = Id::new();
    println!("{:?}", id_orig);
    let id_json = serde_json::to_string(&id_orig).expect("oops");
    println!("{}", id_json);
    let id_parsed: Id = serde_json::from_str(&id_json).expect("oops again");
    println!("{:?}", id_parsed);
    assert_eq!(id_orig, id_parsed);

    let orig_item = Item::new(Id::new(), "item-name");
    let ser_item = serde_json::to_string(&orig_item).unwrap();
    println!("{}", ser_item);

    if let Err(e) = test_items_repo().await {
        println!("{:?}", e);
    }
}

async fn test_items_repo() -> Result<(), mongodb::error::Error> {
    // Parse a connection string into an options struct.
    let mut client_options =
        mongodb::options::ClientOptions::parse("mongodb://localhost:27017").await?;

    // Manually set an option.
    client_options.app_name = Some("Mongo Test".to_string());

    // Get a handle to the deployment.
    let client = mongodb::Client::with_options(client_options)?;

    let items_repo = MongoItemsRepo::new(client.clone());

    let item = Item::new(Id::new(), "An Item");

    items_repo.clone().insert(&item).await?;
    items_repo.clone().insert(&item).await?;

    Ok(())
}
