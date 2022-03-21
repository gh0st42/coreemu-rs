use coreemu::Client;
use std::{env, fs::File, io::Read};

fn usage() {
    println!(
        "usage: {} <scenario.xml> [-s]\n\t-s start scenario\n\n\tDefault server: http://127.0.0.1:50051\n\talternative server via REMOTE_CORE environment variable",
        env::args().next().unwrap()
    );
    std::process::exit(1);
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let remote = env::var("REMOTE_CORE").unwrap_or_else(|_| "http://127.0.0.1:50051".into());
    let mut client = Client::connect(remote).await?;

    let args: Vec<String> = env::args().collect();

    let mut start = false;
    if args.len() < 2 {
        usage();
    } else if args.len() == 3 && args[2] == "-s" {
        start = true;
    }

    // load complete file into string
    let mut f = File::open(args[1].clone())?;
    let mut data = String::new();

    f.read_to_string(&mut data)?;

    let response = client.open_xml(data, start, args[1].clone()).await?;
    dbg!(&response);
    Ok(())
}
