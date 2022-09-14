use std::env;

use coreemu::{core::Node, Client};

fn print_pos(node: Node) {
    let pos = node.position.unwrap();
    println!("{}={},{},{}", node.name, pos.x, pos.y, pos.z);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let remote = env::var("REMOTE_CORE").unwrap_or_else(|_| "http://127.0.0.1:50051".into());
    let mut client = Client::connect(remote).await?;

    let response = client.get_sessions().await?;

    if response.is_empty() {
        eprintln!("No sessions found");
        return Ok(());
    }
    let session_id = response[0].id;

    // getting initial positions
    let response = client.get_session(session_id).await?.unwrap();

    for n in response.nodes {
        print_pos(n);
    }

    while let Some(msg) = client
        .events(session_id, vec![coreemu::core::event_type::Enum::Node])
        .await?
        .message()
        .await?
    {
        if let Some(coreemu::core::event::EventType::NodeEvent(node_event)) = msg.event_type {
            if let Some(node) = node_event.node {
                print_pos(node);
                //println!("{:#?}", node);
            }
        }
    }
    Ok(())
}
