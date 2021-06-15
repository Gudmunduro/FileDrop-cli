use std::path::PathBuf;
use structopt::StructOpt;
use crate::filedrop_api_client::FiledropApiClient;

pub mod filedrop_api_client;
pub mod models;
pub mod simple_error;
// pub mod common;

#[derive(StructOpt)]
struct Options {
    action: String,

    #[structopt(parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    let args: Options = Options::from_args();
    
    for file in &args.files {
        println!("{}", file.as_path().to_str().expect("Failed to get path str"));
    }

    let client = FiledropApiClient::new();

    let drop = client.create_drop().unwrap();
    println!("{}", &drop.id);

    let test_file = args.files.first().unwrap();
    client.upload_file(&drop.id, test_file, &drop.access_token.unwrap()).unwrap();
}
