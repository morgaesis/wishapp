use crate::error::AppError;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client as DynamoDbClient;

pub const TABLE_NAME: &str = "wishlist_table";

pub async fn get_db_client() -> DynamoDbClient {
    let endpoint = std::env::var("DYNAMODB_ENDPOINT");
    let config_builder = aws_config::from_env();
    let config = if let Ok(endpoint_url) = endpoint {
        config_builder
            .endpoint_url(endpoint_url)
            .behavior_version(aws_config::BehaviorVersion::latest())
            .credentials_provider(aws_credential_types::Credentials::for_tests())
            .region(aws_sdk_dynamodb::config::Region::new("us-east-1"))
            .load()
            .await
    } else {
        config_builder.load().await
    };
    DynamoDbClient::new(&config)
}
use crate::handlers::wishlist::Wishlist;
use log::error;

pub async fn get_item(client: &DynamoDbClient, id: String) -> Result<Option<Wishlist>, AppError> {
    let get_item_output = client
        .get_item()
        .table_name(TABLE_NAME)
        .key("id", AttributeValue::S(id))
        .send()
        .await?;

    if let Some(item) = get_item_output.item {
        match Wishlist::try_from(item) {
            Ok(wishlist) => Ok(Some(wishlist)),
            Err(e) => {
                error!("Error converting item to Wishlist: {:?}", e);
                Err(AppError::from(e.to_string()))
            }
        }
    } else {
        Ok(None)
    }
}

pub async fn put_item(client: &DynamoDbClient, wishlist: Wishlist) -> Result<(), AppError> {
    client
        .put_item()
        .table_name(TABLE_NAME)
        .item("id", AttributeValue::S(wishlist.id.clone()))
        .item("name", AttributeValue::S(wishlist.name.clone()))
        .item("owner", AttributeValue::S(wishlist.owner.clone()))
        .item(
            "items",
            AttributeValue::L(
                wishlist
                    .items
                    .iter()
                    .map(|item| AttributeValue::S(item.clone()))
                    .collect(),
            ),
        )
        .send()
        .await?;
    Ok(())
}

pub async fn delete_item(client: &DynamoDbClient, id: String) -> Result<(), AppError> {
    client
        .delete_item()
        .table_name(TABLE_NAME)
        .key("id", AttributeValue::S(id))
        .send()
        .await?;
    Ok(())
}

pub async fn scan_items(client: &DynamoDbClient) -> Result<Vec<Wishlist>, AppError> {
    let scan_output = client.scan().table_name(TABLE_NAME).send().await?;
    let wishlists: Vec<Wishlist> = scan_output
        .items
        .unwrap_or_default()
        .into_iter()
        .filter_map(|item| Wishlist::try_from(item).ok())
        .collect();
    Ok(wishlists)
}
