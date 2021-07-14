use coreemu::{
    core::{MoveNodesRequest, Position},
    Client,
};
use std::env;

fn usage() {
    println!(
        "usage: {} <node_name|node_id> [x] [y] [z]\n\tupdating position requires node id!\n\n\tDefault server: http://127.0.0.1:50051\n\talternative server via REMOTE_CORE environment variable",
        env::args().next().unwrap()
    );
    std::process::exit(1);
}

async fn print_pos(node_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let remote = env::var("REMOTE_CORE").unwrap_or("http://127.0.0.1:50051".into());
    let mut client = Client::connect(remote).await?;

    let response = client.get_sessions().await?;

    //println!("RESPONSE={:?}", response);

    let session_id = response[0].id;

    let response = client.get_session(session_id).await?.unwrap();

    //println!("RESPONSE={:#?}", response);
    //println!("RESPONSE={:#?}", response.nodes);
    //println!("RESPONSE={:#?}", response.location.unwrap());

    for n in response.nodes {
        if n.name == node_name || node_name == n.id.to_string() {
            println!("{} : {:#?}", n.id, n.position.unwrap());
        }
    }
    Ok(())
}

async fn set_pos(node_id: i32, x: f32, y: f32, z: f32) -> Result<(), Box<dyn std::error::Error>> {
    let remote = env::var("REMOTE_CORE").unwrap_or("http://127.0.0.1:50051".into());
    let mut client = Client::connect(remote).await?;

    let response = client.get_sessions().await?;

    //println!("RESPONSE={:?}", response);

    let session_id = response[0].id;

    let pos = coreemu::core::move_nodes_request::MoveType::Position(Position { x, y, z });
    let move_pos: MoveNodesRequest = MoveNodesRequest {
        session_id,
        node_id,
        source: "".into(),
        move_type: Some(pos),
    };
    let _response = client.move_nodes(vec![move_pos]).await?;

    //println!("RESPONSE={:#?}", response);

    Ok(())
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        usage();
    } else if args.len() == 2 {
        print_pos(&args[1]).await?;
    } else if args.len() == 4 || args.len() == 5 {
        let node_id: i32 = args[1].parse()?;
        let x: f32 = args[2].parse()?;
        let y: f32 = args[3].parse()?;
        let z: f32 = if args.len() == 5 {
            args[4].parse()?
        } else {
            0.0
        };
        set_pos(node_id, x, y, z).await?;
    } else {
        usage();
    }
    Ok(())
}
