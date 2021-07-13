use coreemu::core::core_api_client::CoreApiClient;
use coreemu::core::{GetSessionRequest, GetSessionsRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CoreApiClient::connect("http://127.0.0.1:50051").await?;

    let request = tonic::Request::new(GetSessionsRequest {});

    let response = client.get_sessions(request).await?.into_inner().sessions;

    println!("RESPONSE={:?}", response);

    let session_id = response[0].id;
    let request = tonic::Request::new(GetSessionRequest { session_id });
    let response = client
        .get_session(request)
        .await?
        .into_inner()
        .session
        .unwrap();

    //println!("RESPONSE={:#?}", response);
    //println!("RESPONSE={:#?}", response.nodes);
    //println!("RESPONSE={:#?}", response.location.unwrap());

    for n in response.nodes {
        if n.name.starts_with("n") {
            println!("RESPONSE={:#?}", n.position.unwrap());
        }
    }
    Ok(())
}
