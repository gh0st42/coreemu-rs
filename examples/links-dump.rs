use std::env;

use coreemu::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let remote = env::var("REMOTE_CORE").unwrap_or_else(|_| "http://127.0.0.1:50051".into());
    let mut client = Client::connect(remote).await?;

    let response = client.get_sessions().await?;

    // println!("RESPONSE={:?}", response);

    if response.is_empty() {
        eprintln!("No sessions found");
        return Ok(());
    }
    let session_id = response[0].id;

    let response = client.get_session(session_id).await?.unwrap();

    //println!("RESPONSE={:#?}", response);
    //println!("RESPONSE={:#?}", response.nodes);
    //println!("RESPONSE={:#?}", response.location.unwrap());

    let mut links = Vec::new();
    for l in response.links {
        //dbg!(&l);
        if l.r#type() == coreemu::core::link_type::Enum::Wireless {
            // smaller of node1_id and node2_id
            let (n1, n2) = if l.node1_id < l.node2_id {
                (l.node1_id, l.node2_id)
            } else {
                (l.node2_id, l.node1_id)
            };
            links.push(format!("{}-{}", n1, n2));
        }
    }
    println!("{}", links.join(","));
    Ok(())
}
