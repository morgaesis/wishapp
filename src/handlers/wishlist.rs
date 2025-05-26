use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Wishlist {
    pub id: String,
    pub name: String,
    pub owner: String,
    pub items: Vec<String>,
}

impl TryFrom<HashMap<String, AttributeValue>> for Wishlist {
    type Error = String;

    fn try_from(value: HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        let id = value
            .get("id")
            .and_then(|v| v.as_s().ok())
            .ok_or("ID not found or not a string")?
            .to_string();
        let name = value
            .get("name")
            .and_then(|v| v.as_s().ok())
            .ok_or("Name not found or not a string")?
            .to_string();
        let owner = value
            .get("owner")
            .and_then(|v| v.as_s().ok())
            .ok_or("Owner not found or not a string")?
            .to_string();
        let items = value
            .get("items")
            .and_then(|v| v.as_l().ok())
            .map(|v| {
                v.iter()
                    .filter_map(|attr| attr.as_s().ok().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        Ok(Wishlist {
            id,
            name,
            owner,
            items,
        })
    }
}
