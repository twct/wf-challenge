use std::collections::HashMap;
use async_recursion::async_recursion;
use serde::{Deserialize, de::DeserializeOwned};
use dotenv::dotenv;
use crate::error::Error;

mod env;
mod error;

type NodeCountHashMap = HashMap<String, u64>;

#[derive(Deserialize, Debug)]
struct Node {
    id: String,
    child_node_ids: Vec<String>
}

async fn call_api<T: DeserializeOwned>(node_ids: &str) -> Result<T, Error> {
    let resp = reqwest::get(format!("{}/nodes/{}", env::API_ENDPOINT.as_str(), node_ids))
        .await?
        .json::<T>()
        .await?
    ;
    Ok(resp)
}

#[async_recursion]
async fn visit(node: &Node, node_count_hash_map: &mut NodeCountHashMap) -> Result<(), Error> {
    println!("Visiting node {:?}", node.id);

    match node_count_hash_map.get_mut(&node.id) {
        Some(count) => {
            *count = *count + 1
        },
        None => {
            node_count_hash_map.insert(node.id.to_owned(), 1);
        }
    }

    if node.child_node_ids.len() > 0 {
        let child_node_ids = &node.child_node_ids.join(",");
        let nodes = call_api::<Vec<Node>>(&child_node_ids).await?;

        for node in nodes {
            visit(&node, node_count_hash_map).await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let root_node_id = "089ef556-dfff-4ff2-9733-654645be56fe";
    
    let mut node_count_hash_map: NodeCountHashMap = HashMap::new();

    let nodes = call_api::<Vec<Node>>(root_node_id).await?;

    for node in nodes {
        visit(&node, &mut node_count_hash_map).await?;
    }

    match node_count_hash_map.iter().max_by(|a, b| a.1.cmp(&b.1)) {
        Some((most_common_id, most_common_count)) => {
            println!("Unique IDs: {}", node_count_hash_map.keys().len());
            println!("Most common: {:?}, {} time(s)", most_common_id, most_common_count);
        },
        None => {
            println!("No nodes found");
        }
    }

    Ok(())
}
