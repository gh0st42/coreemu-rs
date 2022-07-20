use coreemu::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = Client::connect("http://127.0.0.1:50051".into()).await?;

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

    for n in response.nodes {
        //dbg!(&n);
        let pos = n.position.unwrap();
        if !n.model.is_empty() {
            println!("{}={},{},{}", n.name, pos.x, pos.y, pos.z);
        }
        //if n.name.starts_with("n") {
        //println!("RESPONSE={:#?}", n.position.unwrap().x);
        //}
    }
    Ok(())
}
