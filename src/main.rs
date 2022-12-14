use std::collections::HashMap;
use async_recursion::async_recursion;
use serde::{Deserialize, de::DeserializeOwned};
use dotenv::dotenv;

mod env;

#[derive(Deserialize, Debug)]
struct Node {
    id: String,
    child_node_ids: Vec<String>
}

async fn call_api<T: DeserializeOwned>(node_ids: &str) -> Result<T, Box<dyn std::error::Error>> {
    let resp = reqwest::get(format!("{}/nodes/{}", env::API_ENDPOINT.as_str(), node_ids))
        .await?
        .json::<T>()
        .await?
    ;
    Ok(resp)
}

#[async_recursion]
async fn visit(node: &Node, hash_map: &mut HashMap<String, Vec<String>>) -> Result<(), Box<dyn std::error::Error>> {
    match hash_map.get_mut(&node.id) {
        Some(vec) => vec.push(node.id.to_owned()),
        None => {
            hash_map.insert(node.id.to_owned(), vec![node.id.to_owned()]);
        }
    }

    for child_node_id in &node.child_node_ids {
        let nodes = call_api::<Vec<Node>>(child_node_id).await?;
        for node in nodes {
            println!("Visiting node {:?}", node.id);
            visit(&node, hash_map).await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let root_node_id = "089ef556-dfff-4ff2-9733-654645be56fe";
    
    let mut hash_map: HashMap<String, Vec<String>> = HashMap::new();

    let nodes = call_api::<Vec<Node>>(root_node_id).await?;

    for node in nodes {
        visit(&node, &mut hash_map).await?;
    }

    println!("Unique IDs: {}", hash_map.keys().len());

    match hash_map.iter().max_by_key(|(_k, v)| v.len()).map(|(k, _v)| k) {
        Some(id) => {
            println!("Most common: {}, {} time(s)", id, hash_map.get(id).unwrap().len());
        },
        None => {
            eprintln!("Failed to find most common node ID");
        }
    }

    Ok(())
}
